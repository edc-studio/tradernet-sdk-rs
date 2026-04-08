use crate::errors::TradernetError;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Typed response for the `getTradesHistory` endpoint.
///
/// Deserialization is intentionally lossy to handle inconsistent API payloads.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TradesHistoryResponse {
    #[serde(default)]
    pub trades: TradesSection,
}

/// Trade section in the `getTradesHistory` response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TradesSection {
    #[serde(default, deserialize_with = "deserialize_max_trade_id_rows_lossy")]
    pub max_trade_id: Vec<MaxTradeIdRow>,
    #[serde(default, deserialize_with = "deserialize_trade_rows_lossy")]
    pub trade: Vec<TradeRow>,
}

/// Row with latest trade id marker.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MaxTradeIdRow {
    #[serde(
        rename = "@text",
        default,
        deserialize_with = "deserialize_option_string_lossy"
    )]
    pub text: Option<String>,
}

/// A single trade row from `getTradesHistory`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TradeRow {
    #[serde(default, deserialize_with = "deserialize_option_i64_lossy")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_lossy")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_f64_lossy")]
    pub p: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64_lossy")]
    pub q: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64_lossy")]
    pub v: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub date: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub profit: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub instr_nm: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub curr_c: Option<String>,
    #[serde(
        rename = "type",
        default,
        deserialize_with = "deserialize_option_i64_lossy"
    )]
    pub trade_type: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_lossy")]
    pub reception: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub login: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64_lossy")]
    pub summ: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub curr_q: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_lossy")]
    pub instr_type_c: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_lossy")]
    pub mkt_id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_lossy")]
    pub instr_id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub comment: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub step_price: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub min_step: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64_lossy")]
    pub rate_offer: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64_lossy")]
    pub fv: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub acd: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub go_sum: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub curr_price: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub curr_price_money: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub curr_price_begin: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub curr_price_begin_money: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub pay_d: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub trade_d_exch: Option<String>,
    #[serde(
        rename = "T2_confirm",
        default,
        deserialize_with = "deserialize_option_string_lossy"
    )]
    pub t2_confirm: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_lossy")]
    pub trade_nb: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub repo_close: Option<String>,
    #[serde(
        rename = "StartCash",
        default,
        deserialize_with = "deserialize_option_string_lossy"
    )]
    pub start_cash: Option<String>,
    #[serde(
        rename = "EndCash",
        default,
        deserialize_with = "deserialize_option_string_lossy"
    )]
    pub end_cash: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub commiss_exchange: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub otc: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub details: Option<String>,
    #[serde(
        rename = "OrigClOrdID",
        default,
        deserialize_with = "deserialize_option_string_lossy"
    )]
    pub orig_cl_ord_id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Method-level API error returned by `getTradesHistory`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TradesHistoryApiError {
    pub code: i64,
    pub message: String,
}

impl TradesHistoryResponse {
    /// Returns max trade id from the response, if present.
    pub fn max_trade_id(&self) -> Option<i64> {
        self.trades
            .max_trade_id
            .first()
            .and_then(|row| row.text.as_deref())
            .and_then(|value| value.trim().parse::<i64>().ok())
    }
}

/// Parses raw `getTradesHistory` JSON into [`TradesHistoryResponse`] and surfaces
/// method-level API errors.
pub fn parse_trades_history_response(
    response: Value,
) -> Result<TradesHistoryResponse, TradernetError> {
    if let Some(api_error) = parse_trades_history_api_error(&response) {
        return Err(TradernetError::ApiMethodError {
            code: api_error.code,
            message: api_error.message,
        });
    }

    let response_text = response.to_string();
    let mut deserializer = serde_json::Deserializer::from_str(&response_text);
    serde_path_to_error::deserialize(&mut deserializer).map_err(|error| TradernetError::JsonPath {
        path: error.path().to_string(),
        source: Box::new(error.into_inner()),
    })
}

/// Extracts method-level API error from a raw `getTradesHistory` response, if present.
pub fn parse_trades_history_api_error(response: &Value) -> Option<TradesHistoryApiError> {
    let message = parse_string(response.get("error").or_else(|| response.get("errMsg")))?;
    let code = parse_i64(response.get("code")).unwrap_or(0);
    Some(TradesHistoryApiError { code, message })
}

fn deserialize_max_trade_id_rows_lossy<'de, D>(
    deserializer: D,
) -> Result<Vec<MaxTradeIdRow>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_max_trade_id_rows(value.unwrap_or(Value::Null)))
}

fn deserialize_trade_rows_lossy<'de, D>(deserializer: D) -> Result<Vec<TradeRow>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_trade_rows(value.unwrap_or(Value::Null)))
}

fn deserialize_option_i64_lossy<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_i64(value.as_ref()))
}

fn deserialize_option_f64_lossy<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_f64(value.as_ref()))
}

fn deserialize_option_string_lossy<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_string(value.as_ref()))
}

fn parse_max_trade_id_rows(value: Value) -> Vec<MaxTradeIdRow> {
    match value {
        Value::Array(entries) => entries
            .into_iter()
            .filter_map(parse_max_trade_id_row)
            .collect(),
        Value::Object(_) => parse_max_trade_id_row(value).into_iter().collect(),
        Value::Null => Vec::new(),
        other => {
            log::warn!("max_trade_id expected array/object, got: {other}");
            Vec::new()
        }
    }
}

fn parse_max_trade_id_row(value: Value) -> Option<MaxTradeIdRow> {
    match value {
        Value::Object(map) => serde_json::from_value::<MaxTradeIdRow>(Value::Object(map)).ok(),
        Value::Null => None,
        other => {
            log::warn!("max_trade_id row expected object, got: {other}");
            None
        }
    }
}

fn parse_trade_rows(value: Value) -> Vec<TradeRow> {
    match value {
        Value::Array(entries) => entries.into_iter().filter_map(parse_trade_row).collect(),
        Value::Object(_) => parse_trade_row(value).into_iter().collect(),
        Value::Null => Vec::new(),
        other => {
            log::warn!("trade expected array/object, got: {other}");
            Vec::new()
        }
    }
}

fn parse_trade_row(value: Value) -> Option<TradeRow> {
    match value {
        Value::Object(map) => serde_json::from_value::<TradeRow>(Value::Object(map)).ok(),
        Value::Null => None,
        other => {
            log::warn!("trade row expected object, got: {other}");
            None
        }
    }
}

fn parse_f64(value: Option<&Value>) -> Option<f64> {
    match value? {
        Value::Number(value) => value.as_f64(),
        Value::String(value) => value.trim().parse::<f64>().ok(),
        Value::Bool(value) => Some(if *value { 1.0 } else { 0.0 }),
        Value::Null => None,
        _ => None,
    }
}

fn parse_i64(value: Option<&Value>) -> Option<i64> {
    match value? {
        Value::Number(value) => value
            .as_i64()
            .or_else(|| value.as_f64().map(|value| value.trunc() as i64)),
        Value::String(value) => {
            let value = value.trim();
            if value.is_empty() {
                None
            } else if let Ok(parsed) = value.parse::<i64>() {
                Some(parsed)
            } else {
                value.parse::<f64>().ok().map(|value| value.trunc() as i64)
            }
        }
        Value::Bool(value) => Some(i64::from(*value)),
        Value::Null => None,
        _ => None,
    }
}

fn parse_string(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }
        Value::Number(value) => Some(value.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        Value::Null => None,
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        TradesHistoryResponse, parse_trades_history_api_error, parse_trades_history_response,
    };
    use serde_json::json;

    #[test]
    fn trades_history_response_parses_nominal_payload() {
        let payload = json!({
            "trades": {
                "max_trade_id": [{"@text": "40975888"}],
                "trade": [{
                    "id": 2229992229292_i64,
                    "order_id": 299998887727_i64,
                    "p": 141.4,
                    "q": 20,
                    "v": 2828,
                    "date": "2019-08-15T10:10:22",
                    "type": 1,
                    "reception": 1,
                    "instr_nm": "AAPL.US",
                    "curr_c": "USD",
                    "OrigClOrdID": null
                }]
            }
        });

        let parsed: TradesHistoryResponse =
            serde_json::from_value(payload).expect("nominal response should parse");

        assert_eq!(parsed.max_trade_id(), Some(40_975_888));
        assert_eq!(parsed.trades.trade.len(), 1);
        assert_eq!(parsed.trades.trade[0].id, Some(2_229_992_229_292));
        assert_eq!(parsed.trades.trade[0].order_id, Some(299_998_887_727));
        assert_eq!(parsed.trades.trade[0].p, Some(141.4));
        assert_eq!(parsed.trades.trade[0].q, Some(20.0));
        assert_eq!(parsed.trades.trade[0].trade_type, Some(1));
        assert_eq!(parsed.trades.trade[0].orig_cl_ord_id, None);
    }

    #[test]
    fn trades_history_response_is_lossy_and_does_not_fail_on_bad_types() {
        let payload = json!({
            "trades": {
                "max_trade_id": {"@text": 40975888},
                "trade": [{
                    "id": "2229992229292",
                    "order_id": true,
                    "p": "141.4",
                    "q": false,
                    "v": "bad",
                    "type": "2",
                    "reception": "1",
                    "instr_nm": " AAPL.US ",
                    "curr_c": "",
                    "extra_field": "keep"
                }, "invalid"]
            }
        });

        let parsed: TradesHistoryResponse =
            serde_json::from_value(payload).expect("lossy response should parse");

        assert_eq!(parsed.max_trade_id(), Some(40_975_888));
        assert_eq!(parsed.trades.trade.len(), 1);
        assert_eq!(parsed.trades.trade[0].id, Some(2_229_992_229_292));
        assert_eq!(parsed.trades.trade[0].order_id, Some(1));
        assert_eq!(parsed.trades.trade[0].p, Some(141.4));
        assert_eq!(parsed.trades.trade[0].q, Some(0.0));
        assert_eq!(parsed.trades.trade[0].v, None);
        assert_eq!(parsed.trades.trade[0].trade_type, Some(2));
        assert_eq!(parsed.trades.trade[0].reception, Some(1));
        assert_eq!(parsed.trades.trade[0].instr_nm.as_deref(), Some("AAPL.US"));
        assert_eq!(parsed.trades.trade[0].curr_c, None);
        assert_eq!(
            parsed.trades.trade[0].extra.get("extra_field"),
            Some(&json!("keep"))
        );
    }

    #[test]
    fn trades_history_api_error_is_extracted_from_error_shapes() {
        let payload_a = json!({"errMsg": "Bad json", "code": 2});
        let payload_b = json!({"error": "Пользователь не найден", "code": "7"});

        let error_a = parse_trades_history_api_error(&payload_a).expect("must parse errMsg error");
        let error_b = parse_trades_history_api_error(&payload_b).expect("must parse error field");

        assert_eq!(error_a.code, 2);
        assert_eq!(error_a.message, "Bad json");
        assert_eq!(error_b.code, 7);
        assert_eq!(error_b.message, "Пользователь не найден");
    }

    #[test]
    fn parse_trades_history_response_returns_method_error() {
        let payload = json!({"error": "Пользователь не найден", "code": 7});
        let error = parse_trades_history_response(payload).expect_err("must return method error");
        assert!(
            error
                .to_string()
                .contains("api method error (7): Пользователь не найден")
        );
    }
}
