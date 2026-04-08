use chrono::{NaiveDate, NaiveTime};
use tradernet_sdk_rs::{AsyncTradernet, TradernetError};

#[tokio::test]
async fn async_client_reports_missing_keys() {
    let client = AsyncTradernet::new(None, None).expect("client should be created");
    let error = client
        .user_info()
        .await
        .expect_err("missing keys should error");

    assert!(matches!(error, TradernetError::MissingKeypair));
}

#[tokio::test]
async fn async_client_get_candles_series_reports_missing_keys() {
    let client = AsyncTradernet::new(None, None).expect("client should be created");
    let start = NaiveDate::from_ymd_opt(2026, 1, 1)
        .expect("valid date")
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"));
    let end = NaiveDate::from_ymd_opt(2026, 1, 2)
        .expect("valid date")
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"));

    let error = client
        .get_candles_series("AAPL.US", start, end, 86_400)
        .await
        .expect_err("missing keys should error");

    assert!(matches!(error, TradernetError::MissingKeypair));
}

#[tokio::test]
async fn async_client_get_trades_history_with_reception_validates_input() {
    let client = AsyncTradernet::new(None, None).expect("client should be created");
    let start = NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date");
    let end = NaiveDate::from_ymd_opt(2026, 1, 31).expect("valid date");

    let error = client
        .get_trades_history_with_reception(start, end, None, Some(-1), None, None, Some(1))
        .await
        .expect_err("negative limit must be rejected");

    assert!(matches!(error, TradernetError::InvalidInput(_)));
}

#[tokio::test]
async fn async_client_get_trades_history_typed_validates_input() {
    let client = AsyncTradernet::new(None, None).expect("client should be created");
    let start = NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date");
    let end = NaiveDate::from_ymd_opt(2026, 1, 31).expect("valid date");

    let error = client
        .get_trades_history_typed(start, end, None, Some(-1), None, None)
        .await
        .expect_err("negative limit must be rejected");

    assert!(matches!(error, TradernetError::InvalidInput(_)));
}
