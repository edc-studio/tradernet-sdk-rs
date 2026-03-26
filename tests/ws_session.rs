use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::net::TcpListener as StdTcpListener;
use tokio::net::TcpListener;
use tokio::runtime::Builder;
use tokio::time::{Duration, timeout};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tradernet_sdk_rs::{
    Core, QuoteEvent, SubscribeRequest, TradernetWebsocket, UnsubscribeRequest, WsEvent,
    WsReconnectConfig,
};

fn reconnect_config() -> WsReconnectConfig {
    WsReconnectConfig {
        initial_delay: Duration::from_millis(50),
        max_delay: Duration::from_millis(200),
        multiplier: 2.0,
    }
}

fn runtime() -> tokio::runtime::Runtime {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime")
}

async fn next_text<S>(socket: &mut S) -> String
where
    S: futures_util::Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    loop {
        let message = timeout(Duration::from_secs(3), socket.next())
            .await
            .expect("timed out waiting ws message")
            .expect("ws stream ended")
            .expect("ws read failed");
        if let Message::Text(text) = message {
            return text.to_string();
        }
    }
}

#[test]
fn ws_session_handles_all_subscriptions_on_single_connection() {
    let std_listener = StdTcpListener::bind("127.0.0.1:0").expect("bind mock ws listener");
    std_listener
        .set_nonblocking(true)
        .expect("set nonblocking listener");
    let addr = std_listener.local_addr().expect("local addr");

    let core = Core::new(None, None).expect("core");
    let ws = TradernetWebsocket::from_core(&core).with_websocket_url(format!("ws://{}", addr));

    runtime().block_on(async move {
        let listener = TcpListener::from_std(std_listener).expect("tokio listener");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept client");
            let mut socket = accept_async(stream).await.expect("ws handshake");

            let mut commands = Vec::<Value>::new();
            for _ in 0..5 {
                let text = next_text(&mut socket).await;
                commands.push(serde_json::from_str(&text).expect("valid command json"));
            }

            let events = [
                serde_json::json!(["q", {"c": "AAPL.US", "ltp": 191.2}]),
                serde_json::json!(["b", {"i":"AAPL.US", "cnt":1, "ins":[], "del":[], "upd":[]}]),
                serde_json::json!(["b", {"i":"AAPL.US", "cnt":2, "ins":[], "del":[], "upd":[]}]),
                serde_json::json!(["portfolio", {"key":"x", "acc":[], "pos":[]}]),
                serde_json::json!(["orders", []]),
                serde_json::json!(["markets", {"t":"2026-03-26 10:10:10", "m":[]}]),
                serde_json::json!(["error", {"code": 500}]),
            ];

            for event in events {
                socket
                    .send(Message::Text(event.to_string()))
                    .await
                    .expect("send event");
            }

            socket.close(None).await.expect("close socket");
            commands
        });

        let session = ws
            .connect_with_config(reconnect_config())
            .await
            .expect("connect");

        session
            .subscribe(SubscribeRequest::Quotes {
                symbols: vec!["AAPL.US".to_string(), "AAPL.US".to_string()],
            })
            .await
            .expect("subscribe quotes");
        session
            .subscribe(SubscribeRequest::OrderBook {
                symbols: vec!["AAPL.US".to_string()],
            })
            .await
            .expect("subscribe depth");
        session
            .subscribe(SubscribeRequest::Portfolio)
            .await
            .expect("subscribe portfolio");
        session
            .subscribe(SubscribeRequest::Orders)
            .await
            .expect("subscribe orders");
        session
            .subscribe(SubscribeRequest::Markets)
            .await
            .expect("subscribe markets");

        let mut events = session.events();
        let mut seen_quote = false;
        let mut seen_depth = false;
        let mut seen_portfolio = false;
        let mut seen_orders = false;
        let mut seen_markets = false;
        let mut seen_error = false;

        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        while tokio::time::Instant::now() < deadline {
            let event = timeout(Duration::from_millis(300), events.next()).await;
            let Ok(Some(event)) = event else {
                continue;
            };
            let event = event.expect("ws event must not fail");

            match event {
                WsEvent::Quote(_) => seen_quote = true,
                WsEvent::MarketDepth(_) => seen_depth = true,
                WsEvent::Portfolio(_) => seen_portfolio = true,
                WsEvent::Orders(_) => seen_orders = true,
                WsEvent::Markets(_) => seen_markets = true,
                WsEvent::Error(_) => seen_error = true,
                _ => {}
            }

            if seen_quote
                && seen_depth
                && seen_portfolio
                && seen_orders
                && seen_markets
                && seen_error
            {
                break;
            }
        }

        assert!(seen_quote);
        assert!(seen_depth);
        assert!(seen_portfolio);
        assert!(seen_orders);
        assert!(seen_markets);
        assert!(seen_error);

        session.close().await.expect("close session");

        let commands = server.await.expect("server join");
        assert_eq!(commands.len(), 5);

        let mut command_names = commands
            .into_iter()
            .map(|value| value[0].as_str().unwrap_or("?").to_string())
            .collect::<Vec<_>>();
        command_names.sort();

        assert_eq!(
            command_names,
            vec!["markets", "orderBook", "orders", "portfolio", "quotes"]
        );
    });
}

#[test]
fn ws_session_reconnects_restores_subscriptions_and_applies_local_unsubscribe_filter() {
    let std_listener = StdTcpListener::bind("127.0.0.1:0").expect("bind mock ws listener");
    std_listener
        .set_nonblocking(true)
        .expect("set nonblocking listener");
    let addr = std_listener.local_addr().expect("local addr");

    let core = Core::new(None, None).expect("core");
    let ws = TradernetWebsocket::from_core(&core).with_websocket_url(format!("ws://{}", addr));

    runtime().block_on(async move {
        let listener = TcpListener::from_std(std_listener).expect("tokio listener");

        let server = tokio::spawn(async move {
            let (stream1, _) = listener.accept().await.expect("accept client #1");
            let mut socket1 = accept_async(stream1).await.expect("ws handshake #1");

            let cmd1 =
                serde_json::from_str::<Value>(&next_text(&mut socket1).await).expect("cmd #1");
            let cmd2 =
                serde_json::from_str::<Value>(&next_text(&mut socket1).await).expect("cmd #2");

            socket1
                .send(Message::Text(
                    serde_json::json!(["q", {"c":"AAPL.US", "ltp": 100.0}]).to_string(),
                ))
                .await
                .expect("send aapl #1");
            socket1
                .send(Message::Text(
                    serde_json::json!(["q", {"c":"TSLA.US", "ltp": 200.0}]).to_string(),
                ))
                .await
                .expect("send tsla #1");

            tokio::time::sleep(Duration::from_millis(120)).await;

            socket1
                .send(Message::Text(
                    serde_json::json!(["q", {"c":"AAPL.US", "ltp": 101.0}]).to_string(),
                ))
                .await
                .expect("send aapl #2");
            socket1
                .send(Message::Text(
                    serde_json::json!(["q", {"c":"TSLA.US", "ltp": 201.0}]).to_string(),
                ))
                .await
                .expect("send tsla #2");

            socket1.close(None).await.expect("close #1");

            let (stream2, _) = listener.accept().await.expect("accept client #2");
            let mut socket2 = accept_async(stream2).await.expect("ws handshake #2");

            let replay1 =
                serde_json::from_str::<Value>(&next_text(&mut socket2).await).expect("replay #1");
            let replay2 =
                serde_json::from_str::<Value>(&next_text(&mut socket2).await).expect("replay #2");

            socket2
                .send(Message::Text(
                    serde_json::json!(["q", {"c":"AAPL.US", "ltp": 102.0}]).to_string(),
                ))
                .await
                .expect("send aapl #3");
            socket2
                .send(Message::Text(
                    serde_json::json!(["q", {"c":"TSLA.US", "ltp": 202.0}]).to_string(),
                ))
                .await
                .expect("send tsla #3");

            socket2.close(None).await.expect("close #2");

            (cmd1, cmd2, replay1, replay2)
        });

        let session = ws
            .connect_with_config(reconnect_config())
            .await
            .expect("connect");

        session
            .subscribe(SubscribeRequest::Quotes {
                symbols: vec!["AAPL.US".to_string(), "TSLA.US".to_string()],
            })
            .await
            .expect("subscribe quotes");
        session
            .subscribe(SubscribeRequest::OrderBook {
                symbols: vec!["AAPL.US".to_string()],
            })
            .await
            .expect("subscribe depth");

        let mut events = session.events();

        let mut received_quotes = Vec::<String>::new();
        let mut unsubscribed = false;
        let mut saw_reconnecting = false;

        let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
        while tokio::time::Instant::now() < deadline {
            let event = timeout(Duration::from_millis(300), events.next()).await;
            let Ok(Some(Ok(event))) = event else {
                continue;
            };

            match event {
                WsEvent::Quote(quote) => {
                    if let Some(symbol) = quote.c {
                        received_quotes.push(symbol.clone());
                    }

                    if !unsubscribed
                        && received_quotes.contains(&"AAPL.US".to_string())
                        && received_quotes.contains(&"TSLA.US".to_string())
                    {
                        session
                            .unsubscribe(UnsubscribeRequest::Quotes {
                                symbols: vec!["AAPL.US".to_string()],
                            })
                            .await
                            .expect("unsubscribe aapl");
                        unsubscribed = true;
                    }
                }
                WsEvent::Reconnecting => saw_reconnecting = true,
                WsEvent::Closed => break,
                _ => {}
            }

            if saw_reconnecting
                && received_quotes
                    .iter()
                    .filter(|s| s.as_str() == "TSLA.US")
                    .count()
                    >= 3
            {
                break;
            }
        }

        session.close().await.expect("close session");

        assert!(saw_reconnecting, "must observe reconnecting state");

        let aapl_count = received_quotes
            .iter()
            .filter(|s| s.as_str() == "AAPL.US")
            .count();
        let tsla_count = received_quotes
            .iter()
            .filter(|s| s.as_str() == "TSLA.US")
            .count();

        assert!(tsla_count >= 2, "must keep TSLA updates after unsubscribe");
        assert_eq!(
            aapl_count, 1,
            "AAPL should be filtered after local unsubscribe"
        );

        let (cmd1, cmd2, replay1, replay2) = server.await.expect("server join");
        let first_commands = [
            cmd1[0].as_str().unwrap_or(""),
            cmd2[0].as_str().unwrap_or(""),
        ];
        assert!(first_commands.contains(&"quotes"));
        assert!(first_commands.contains(&"orderBook"));

        let (replay_quotes, replay_depth) = if replay1[0] == "quotes" {
            (&replay1, &replay2)
        } else {
            (&replay2, &replay1)
        };

        let replay_symbols = replay_quotes[1]
            .as_array()
            .expect("quotes symbols array")
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>();

        assert_eq!(replay_symbols, vec!["TSLA.US"]);
        assert_eq!(replay_depth[0], "orderBook");
    });
}

#[test]
fn legacy_quotes_method_works_via_session_wrapper() {
    let std_listener = StdTcpListener::bind("127.0.0.1:0").expect("bind mock ws listener");
    std_listener
        .set_nonblocking(true)
        .expect("set nonblocking listener");
    let addr = std_listener.local_addr().expect("local addr");

    let core = Core::new(None, None).expect("core");
    let ws = TradernetWebsocket::from_core(&core).with_websocket_url(format!("ws://{}", addr));

    runtime().block_on(async move {
        let listener = TcpListener::from_std(std_listener).expect("tokio listener");

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept client");
            let mut socket = accept_async(stream).await.expect("ws handshake");

            let text = next_text(&mut socket).await;
            let command: Value = serde_json::from_str(&text).expect("valid command json");
            assert_eq!(command[0], "quotes");

            socket
                .send(Message::Text(
                    serde_json::json!(["q", {"c": "AAPL.US", "ltp": 198.5}]).to_string(),
                ))
                .await
                .expect("send quote");
            socket.close(None).await.expect("close socket");
        });

        let mut stream = ws.quotes(["AAPL.US"]).await.expect("legacy quotes stream");
        let first = timeout(Duration::from_secs(3), stream.next())
            .await
            .expect("timeout waiting legacy quote")
            .expect("stream ended")
            .expect("quote must parse");

        let QuoteEvent::Quote(quote) = first else {
            panic!("expected quote event")
        };
        assert_eq!(quote.c.as_deref(), Some("AAPL.US"));

        drop(stream);
        server.await.expect("server join");
    });
}
