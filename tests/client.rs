use chrono::{NaiveDate, NaiveTime};
use tradernet_sdk_rs::{Tradernet, TradernetError};

#[test]
fn client_get_candles_series_reports_missing_keys() {
    let client = Tradernet::new(None, None).expect("client should be created");
    let start = NaiveDate::from_ymd_opt(2026, 1, 1)
        .expect("valid date")
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"));
    let end = NaiveDate::from_ymd_opt(2026, 1, 2)
        .expect("valid date")
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"));

    let error = client
        .get_candles_series("AAPL.US", start, end, 86_400)
        .expect_err("missing keys should error");

    assert!(matches!(error, TradernetError::MissingKeypair));
}
