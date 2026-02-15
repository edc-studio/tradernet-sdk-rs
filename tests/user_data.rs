use std::fs;
use std::sync::{Mutex, Once, OnceLock};

use log::{Level, LevelFilter, Log, Metadata, Record};
use tradernet_sdk_rs::UserDataResponse;

static LOGGER: TestLogger = TestLogger;
static LOGS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
static INIT_LOGGER: Once = Once::new();

struct TestLogger;

impl Log for TestLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Warn
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(logs) = LOGS.get() {
                let mut logs = logs.lock().unwrap();
                logs.push(format!("{}", record.args()));
            }
        }
    }

    fn flush(&self) {}
}

fn init_logger() {
    INIT_LOGGER.call_once(|| {
        let _ = LOGS.set(Mutex::new(Vec::new()));
        let _ = log::set_logger(&LOGGER);
        log::set_max_level(LevelFilter::Warn);
    });
}

fn take_logs() -> Vec<String> {
    let logs = LOGS.get().expect("logger not initialized");
    let mut logs = logs.lock().unwrap();
    let output = logs.clone();
    logs.clear();
    output
}

#[test]
fn deserializes_user_data_fixture() {
    let payload = fs::read_to_string("tests/fixtures/get_user_data.json").unwrap();
    let data: UserDataResponse = serde_json::from_str(&payload).unwrap();

    assert_eq!(data.opq.brief_nm, "000000");
    assert_eq!(data.opq.home_currency, "USD");
    assert_eq!(data.opq.quotes.q.len(), 1);
    assert_eq!(data.opq.user_info.id, Some(100000000));
    assert_eq!(
        data.opq
            .user_options
            .grid_portfolio
            .as_deref()
            .map(|items| items.len()),
        Some(3)
    );
}

#[test]
fn deserializes_user_data_with_empty_string_ints() {
    let payload = fs::read_to_string("tests/fixtures/get_user_data.json").unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    value["OPQ"]["rev"] = serde_json::json!("");

    let data: UserDataResponse = serde_json::from_value(value).unwrap();

    assert_eq!(data.opq.rev, 0);
}

#[test]
fn deserializes_user_data_with_float_ints() {
    let payload = fs::read_to_string("tests/fixtures/get_user_data.json").unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    value["OPQ"]["active"] = serde_json::json!(1.0);

    let data: UserDataResponse = serde_json::from_value(value).unwrap();

    assert_eq!(data.opq.active, 1);
}

#[test]
fn deserializes_user_data_with_numeric_optional_string() {
    let payload = fs::read_to_string("tests/fixtures/get_user_data.json").unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    value["OPQ"]["userInfo"]["minimum_investment"] = serde_json::json!(5000);

    let data: UserDataResponse = serde_json::from_value(value).unwrap();

    assert_eq!(
        data.opq.user_info.minimum_investment.as_deref(),
        Some("5000")
    );
}

#[test]
fn deserializes_user_data_with_numeric_market_date() {
    let payload = fs::read_to_string("tests/fixtures/get_user_data.json").unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    value["OPQ"]["markets"]["markets"]["m"][0]["date"][0]["from"] = serde_json::json!(1026);
    value["OPQ"]["markets"]["markets"]["m"][0]["date"][0]["to"] = serde_json::json!(2026);

    let data: UserDataResponse = serde_json::from_value(value).unwrap();

    let market_date = &data.opq.markets.markets.m[0].date.as_ref().unwrap()[0];
    assert_eq!(market_date.from, "1026");
    assert_eq!(market_date.to, "2026");
}

#[test]
fn deserializes_user_data_with_user_stock_lists_array() {
    let payload = fs::read_to_string("tests/fixtures/get_user_data.json").unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    value["OPQ"]["userLists"]["userStockLists"] = serde_json::json!([
        {"default": ["BA.US", "MCD.US"]}
    ]);

    let data: UserDataResponse = serde_json::from_value(value).unwrap();

    assert_eq!(data.opq.user_lists.user_stock_lists.default.len(), 2);
    assert_eq!(
        data.opq.user_lists.user_stock_lists.default,
        vec!["BA.US", "MCD.US"]
    );
}

#[test]
fn warns_on_fractional_i64_with_field_name() {
    init_logger();
    take_logs();

    let payload = fs::read_to_string("tests/fixtures/get_user_data.json").unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    value["OPQ"]["rev"] = serde_json::json!(1.5);

    let _data: UserDataResponse = serde_json::from_value(value).unwrap();

    let logs = take_logs();
    assert!(
        logs.iter()
            .any(|message| message.contains("rev") && message.contains("fractional"))
    );
}
