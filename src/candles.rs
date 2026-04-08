use crate::errors::TradernetError;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Typed response for the `getHloc` endpoint.
///
/// Deserialization is intentionally lossy to handle inconsistent API payloads.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CandlesResponse {
    #[serde(default, deserialize_with = "deserialize_hloc_map_lossy")]
    pub hloc: HashMap<String, Vec<CandleOhlc>>,
    #[serde(default, deserialize_with = "deserialize_i64_series_map_lossy")]
    pub vl: HashMap<String, Vec<i64>>,
    #[serde(
        rename = "xSeries",
        default,
        deserialize_with = "deserialize_i64_series_map_lossy"
    )]
    pub x_series: HashMap<String, Vec<i64>>,
    #[serde(
        rename = "maxSeries",
        default,
        deserialize_with = "deserialize_i64_lossy"
    )]
    pub max_series: i64,
    #[serde(default, deserialize_with = "deserialize_info_map_lossy")]
    pub info: HashMap<String, Vec<CandleInfo>>,
    #[serde(default, deserialize_with = "deserialize_f64_lossy")]
    pub took: f64,
}

/// Normalized candle row.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CandlePoint {
    /// Candle timestamp in Unix seconds.
    pub ts: i64,
    /// Open price.
    pub open: f64,
    /// High price.
    pub high: f64,
    /// Low price.
    pub low: f64,
    /// Close price.
    pub close: f64,
    /// Candle volume.
    pub volume: i64,
}

/// Normalized candle series for one symbol.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SymbolCandles {
    pub symbol: String,
    pub items: Vec<CandlePoint>,
}

/// A single candle in `[high, low, open, close]` form.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CandleOhlc {
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64,
}

/// Security metadata attached to candle responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CandleInfo {
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub id: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub nt_ticker: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub short_name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub default_ticker: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub code_nm: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub currency: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub min_step: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_lossy")]
    pub lot: Option<String>,
}

/// Method-level API error returned by `getHloc`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CandlesApiError {
    pub code: i64,
    pub message: String,
}

impl CandlesResponse {
    /// Builds a normalized candle series for a symbol.
    ///
    /// Alignment policy:
    /// - base series length is `min(xSeries.len(), hloc.len())`;
    /// - if volume row is missing at index `i`, `volume` is set to `0`;
    /// - extra tail elements in any source array are ignored.
    pub fn series_for_symbol(&self, symbol: &str) -> SymbolCandles {
        let timestamps = self.x_series.get(symbol).cloned().unwrap_or_default();
        let candles = self.hloc.get(symbol).cloned().unwrap_or_default();
        let volumes = self.vl.get(symbol).cloned().unwrap_or_default();

        let len = timestamps.len().min(candles.len());
        let mut items = Vec::with_capacity(len);

        for index in 0..len {
            let row = &candles[index];
            items.push(CandlePoint {
                ts: timestamps[index],
                open: row.open,
                high: row.high,
                low: row.low,
                close: row.close,
                volume: volumes.get(index).copied().unwrap_or(0),
            });
        }

        SymbolCandles {
            symbol: symbol.to_string(),
            items,
        }
    }
}

/// Parses raw `getHloc` JSON into [`CandlesResponse`] and surfaces method-level API errors.
pub fn parse_candles_response(response: Value) -> Result<CandlesResponse, TradernetError> {
    if let Some(api_error) = parse_candles_api_error(&response) {
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

/// Extracts method-level API error from a raw `getHloc` response, if present.
pub fn parse_candles_api_error(response: &Value) -> Option<CandlesApiError> {
    let message = parse_string(response.get("error").or_else(|| response.get("errMsg")))?;
    let code = parse_i64(response.get("code")).unwrap_or(0);
    Some(CandlesApiError { code, message })
}

fn deserialize_hloc_map_lossy<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, Vec<CandleOhlc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_hloc_map(value.unwrap_or(Value::Null)))
}

fn deserialize_i64_series_map_lossy<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, Vec<i64>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_i64_series_map(value.unwrap_or(Value::Null)))
}

fn deserialize_info_map_lossy<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, Vec<CandleInfo>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_info_map(value.unwrap_or(Value::Null)))
}

fn deserialize_i64_lossy<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_i64(value.as_ref()).unwrap_or(0))
}

fn deserialize_f64_lossy<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_f64(value.as_ref()).unwrap_or(0.0))
}

fn deserialize_option_string_lossy<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(parse_string(value.as_ref()))
}

fn parse_hloc_map(value: Value) -> HashMap<String, Vec<CandleOhlc>> {
    match value {
        Value::Object(map) => map
            .into_iter()
            .map(|(symbol, entry)| (symbol, parse_ohlc_series(entry)))
            .collect(),
        Value::Null => HashMap::new(),
        other => {
            log::warn!("hloc expected object, got: {other}");
            HashMap::new()
        }
    }
}

fn parse_ohlc_series(value: Value) -> Vec<CandleOhlc> {
    match value {
        Value::Array(entries) => entries.into_iter().map(parse_ohlc_entry).collect(),
        Value::Object(_) => vec![parse_ohlc_entry(value)],
        Value::Null => Vec::new(),
        other => {
            log::warn!("hloc symbol entry expected array/object, got: {other}");
            Vec::new()
        }
    }
}

fn parse_ohlc_entry(value: Value) -> CandleOhlc {
    match value {
        Value::Array(values) => {
            if values.len() < 4 {
                log::warn!("hloc row expected at least 4 values, got {}", values.len());
                return CandleOhlc::default();
            }
            CandleOhlc {
                high: parse_f64(values.first()).unwrap_or(0.0),
                low: parse_f64(values.get(1)).unwrap_or(0.0),
                open: parse_f64(values.get(2)).unwrap_or(0.0),
                close: parse_f64(values.get(3)).unwrap_or(0.0),
            }
        }
        Value::Object(map) => CandleOhlc {
            high: parse_f64(map.get("high").or_else(|| map.get("h"))).unwrap_or(0.0),
            low: parse_f64(map.get("low").or_else(|| map.get("l"))).unwrap_or(0.0),
            open: parse_f64(map.get("open").or_else(|| map.get("o"))).unwrap_or(0.0),
            close: parse_f64(map.get("close").or_else(|| map.get("c"))).unwrap_or(0.0),
        },
        Value::Null => CandleOhlc::default(),
        other => {
            log::warn!("hloc row expected array/object, got: {other}");
            CandleOhlc::default()
        }
    }
}

fn parse_i64_series_map(value: Value) -> HashMap<String, Vec<i64>> {
    match value {
        Value::Object(map) => map
            .into_iter()
            .map(|(symbol, entry)| (symbol, parse_i64_series(entry)))
            .collect(),
        Value::Null => HashMap::new(),
        other => {
            log::warn!("series map expected object, got: {other}");
            HashMap::new()
        }
    }
}

fn parse_i64_series(value: Value) -> Vec<i64> {
    match value {
        Value::Array(entries) => entries
            .iter()
            .map(|entry| parse_i64(Some(entry)).unwrap_or(0))
            .collect(),
        Value::Null => Vec::new(),
        other => {
            log::warn!("series entry expected array, got: {other}");
            Vec::new()
        }
    }
}

fn parse_info_map(value: Value) -> HashMap<String, Vec<CandleInfo>> {
    match value {
        Value::Object(map) => map
            .into_iter()
            .map(|(symbol, entry)| (symbol, parse_info_series(entry)))
            .collect(),
        Value::Null => HashMap::new(),
        other => {
            log::warn!("info expected object, got: {other}");
            HashMap::new()
        }
    }
}

fn parse_info_series(value: Value) -> Vec<CandleInfo> {
    match value {
        Value::Array(entries) => entries
            .into_iter()
            .filter_map(|entry| parse_info_entry(Some(entry)))
            .collect(),
        Value::Object(_) => parse_info_entry(Some(value)).into_iter().collect(),
        Value::Null => Vec::new(),
        other => {
            log::warn!("info symbol entry expected array/object, got: {other}");
            Vec::new()
        }
    }
}

fn parse_info_entry(value: Option<Value>) -> Option<CandleInfo> {
    match value {
        Some(Value::Object(map)) => serde_json::from_value::<CandleInfo>(Value::Object(map)).ok(),
        Some(Value::Null) | None => None,
        Some(other) => {
            log::warn!("info row expected object, got: {other}");
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
        CandlePoint, CandlesResponse, SymbolCandles, parse_candles_api_error,
        parse_candles_response,
    };
    use serde_json::json;

    #[test]
    fn candles_response_parses_nominal_payload() {
        let payload = json!({
            "hloc": {"FB.US": [[107.25, 106.1603, 106.96, 106.26]]},
            "vl": {"FB.US": [7588957]},
            "xSeries": {"FB.US": [1451422800]},
            "maxSeries": 1452027600,
            "info": {"FB.US": [{
                "id": "FB.US",
                "nt_ticker": "FB.US",
                "short_name": "Facebook, Inc."
            }]},
            "took": 26.685
        });

        let parsed: CandlesResponse =
            serde_json::from_value(payload).expect("nominal response should parse");
        assert_eq!(parsed.hloc["FB.US"][0].high, 107.25);
        assert_eq!(parsed.vl["FB.US"][0], 7_588_957);
        assert_eq!(parsed.x_series["FB.US"][0], 1_451_422_800);
        assert_eq!(parsed.max_series, 1_452_027_600);
        assert_eq!(parsed.info["FB.US"][0].id.as_deref(), Some("FB.US"));
        assert_eq!(parsed.took, 26.685);
    }

    #[test]
    fn candles_response_is_lossy_and_does_not_fail_on_bad_types() {
        let payload = json!({
            "hloc": {"FB.US": [
                [107.25, "106.1603", "106.96", false],
                ["bad", 104.62, 106.45, 104.75],
                {"high": "103.33", "low": 100.25, "open": true, "close": "102.32"},
                "invalid"
            ]},
            "vl": {"FB.US": ["7588957", true, null, "bad"]},
            "xSeries": {"FB.US": ["1451422800", 1451509200.9, false, null]},
            "maxSeries": "1452027600",
            "info": {"FB.US": {
                "id": 10,
                "nt_ticker": true,
                "short_name": " Facebook "
            }},
            "took": "26.685"
        });

        let parsed: CandlesResponse =
            serde_json::from_value(payload).expect("lossy response should parse");

        assert_eq!(parsed.hloc["FB.US"].len(), 4);
        assert_eq!(parsed.hloc["FB.US"][0].close, 0.0);
        assert_eq!(parsed.hloc["FB.US"][1].high, 0.0);
        assert_eq!(parsed.hloc["FB.US"][2].open, 1.0);
        assert_eq!(parsed.hloc["FB.US"][3].high, 0.0);

        assert_eq!(parsed.vl["FB.US"], vec![7_588_957, 1, 0, 0]);
        assert_eq!(
            parsed.x_series["FB.US"],
            vec![1_451_422_800, 1_451_509_200, 0, 0]
        );
        assert_eq!(parsed.max_series, 1_452_027_600);
        assert_eq!(parsed.info["FB.US"][0].id.as_deref(), Some("10"));
        assert_eq!(parsed.info["FB.US"][0].nt_ticker.as_deref(), Some("true"));
        assert_eq!(
            parsed.info["FB.US"][0].short_name.as_deref(),
            Some("Facebook")
        );
        assert_eq!(parsed.took, 26.685);
    }

    #[test]
    fn candles_api_error_is_extracted_from_error_shapes() {
        let payload_a = json!({"errMsg": "Bad json", "code": 2});
        let payload_b = json!({"error": "Пользователь не найден", "code": "7"});

        let error_a = parse_candles_api_error(&payload_a).expect("must parse errMsg error");
        let error_b = parse_candles_api_error(&payload_b).expect("must parse error field");

        assert_eq!(error_a.code, 2);
        assert_eq!(error_a.message, "Bad json");
        assert_eq!(error_b.code, 7);
        assert_eq!(error_b.message, "Пользователь не найден");
    }

    #[test]
    fn parse_candles_response_returns_method_error() {
        let payload = json!({"error": "Пользователь не найден", "code": 7});
        let error = parse_candles_response(payload).expect_err("must return method error");
        assert!(
            error
                .to_string()
                .contains("api method error (7): Пользователь не найден")
        );
    }

    #[test]
    fn series_for_symbol_builds_normalized_points() {
        let payload = json!({
            "hloc": {"AAPL.US": [[10.5, 9.8, 10.0, 10.2], [10.9, 10.1, 10.2, 10.7]]},
            "vl": {"AAPL.US": [1000, 1200]},
            "xSeries": {"AAPL.US": [1700000000, 1700003600]}
        });
        let response = parse_candles_response(payload).expect("must parse candles");

        let series = response.series_for_symbol("AAPL.US");
        assert_eq!(
            series,
            SymbolCandles {
                symbol: "AAPL.US".to_string(),
                items: vec![
                    CandlePoint {
                        ts: 1_700_000_000,
                        open: 10.0,
                        high: 10.5,
                        low: 9.8,
                        close: 10.2,
                        volume: 1000,
                    },
                    CandlePoint {
                        ts: 1_700_003_600,
                        open: 10.2,
                        high: 10.9,
                        low: 10.1,
                        close: 10.7,
                        volume: 1200,
                    },
                ],
            }
        );
    }

    #[test]
    fn series_for_symbol_is_lossy_on_mixed_types() {
        let payload = json!({
            "hloc": {"AAPL.US": [[10.5, "9.8", "10.0", false], ["bad", 10.1, 10.2, 10.7]]},
            "vl": {"AAPL.US": ["1000", true]},
            "xSeries": {"AAPL.US": ["1700000000", 1700003600.9]}
        });
        let response = parse_candles_response(payload).expect("must parse candles");

        let series = response.series_for_symbol("AAPL.US");
        assert_eq!(series.items.len(), 2);
        assert_eq!(series.items[0].ts, 1_700_000_000);
        assert_eq!(series.items[0].open, 10.0);
        assert_eq!(series.items[0].close, 0.0);
        assert_eq!(series.items[0].volume, 1000);
        assert_eq!(series.items[1].ts, 1_700_003_600);
        assert_eq!(series.items[1].high, 0.0);
        assert_eq!(series.items[1].volume, 1);
    }

    #[test]
    fn series_for_symbol_handles_length_mismatch_predictably() {
        let payload = json!({
            "hloc": {"AAPL.US": [[11.0, 9.0, 10.0, 10.5], [12.0, 10.0, 10.5, 11.1], [13.0, 11.0, 11.1, 12.2]]},
            "vl": {"AAPL.US": [100]},
            "xSeries": {"AAPL.US": [1700000000, 1700003600]}
        });
        let response = parse_candles_response(payload).expect("must parse candles");

        let series = response.series_for_symbol("AAPL.US");
        assert_eq!(series.items.len(), 2);
        assert_eq!(series.items[0].volume, 100);
        assert_eq!(series.items[1].volume, 0);
        assert_eq!(series.items[1].ts, 1_700_003_600);
        assert_eq!(series.items[1].close, 11.1);
    }
}
