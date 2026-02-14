use std::fs;

use tradernet_sdk_rs::UserDataResponse;

#[test]
fn deserializes_user_data_fixture() {
    let payload = fs::read_to_string("tests/fixtures/get_user_data.json").unwrap();
    let data: UserDataResponse = serde_json::from_str(&payload).unwrap();

    assert_eq!(data.opq.brief_nm, "000000");
    assert_eq!(data.opq.home_currency, "USD");
    assert_eq!(data.opq.quotes.q.len(), 1);
    assert_eq!(data.opq.user_info.id, 100000000);
    assert_eq!(data.opq.user_options.grid_portfolio.len(), 3);
}
