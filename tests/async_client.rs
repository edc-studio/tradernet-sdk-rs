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
