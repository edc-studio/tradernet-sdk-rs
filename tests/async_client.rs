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
