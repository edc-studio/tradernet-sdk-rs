use crate::errors::TradernetError;
use chrono::NaiveDateTime;
use regex::Regex;
use serde_json::{Map, Value};
use std::io::Read;

pub(crate) fn duration_code(duration: &str) -> Option<i64> {
    match duration.to_ascii_lowercase().as_str() {
        "day" => Some(1),
        "ext" => Some(2),
        "gtc" => Some(3),
        _ => None,
    }
}

pub(crate) fn build_trade_params(
    symbol: &str,
    quantity: i64,
    price: f64,
    duration: &str,
    use_margin: bool,
    custom_order_id: Option<i64>,
) -> Result<Map<String, Value>, TradernetError> {
    if quantity == 0 {
        return Err(TradernetError::InvalidInput("Zero quantity".to_string()));
    }

    let duration_code = duration_code(duration)
        .ok_or_else(|| TradernetError::InvalidInput(format!("Unknown duration {duration}")))?;

    let action_id = if quantity > 0 {
        if use_margin { 2 } else { 1 }
    } else if use_margin {
        4
    } else {
        3
    };

    let mut params = Map::new();
    params.insert("instr_name".to_string(), Value::String(symbol.to_string()));
    params.insert("action_id".to_string(), Value::Number(action_id.into()));
    params.insert(
        "order_type_id".to_string(),
        Value::Number((if price != 0.0 { 2 } else { 1 }).into()),
    );
    params.insert(
        "qty".to_string(),
        Value::Number(quantity.unsigned_abs().into()),
    );
    let limit_price = serde_json::Number::from_f64(price)
        .ok_or_else(|| TradernetError::InvalidInput("Invalid price".to_string()))?;
    params.insert("limit_price".to_string(), Value::Number(limit_price));
    params.insert(
        "expiration_id".to_string(),
        Value::Number(duration_code.into()),
    );
    if let Some(custom_order_id) = custom_order_id {
        params.insert(
            "user_order_id".to_string(),
            Value::Number(custom_order_id.into()),
        );
    }

    Ok(params)
}

pub(crate) fn build_stop_params(
    symbol: &str,
    price: f64,
) -> Result<Map<String, Value>, TradernetError> {
    let mut params = Map::new();
    params.insert("instr_name".to_string(), Value::String(symbol.to_string()));
    let price = serde_json::Number::from_f64(price)
        .ok_or_else(|| TradernetError::InvalidInput("Invalid price".to_string()))?;
    params.insert("stop_loss".to_string(), Value::Number(price));
    Ok(params)
}

pub(crate) fn build_trailing_stop_params(symbol: &str, percent: i64) -> Map<String, Value> {
    let mut params = Map::new();
    params.insert("instr_name".to_string(), Value::String(symbol.to_string()));
    params.insert(
        "stop_loss_percent".to_string(),
        Value::Number(percent.into()),
    );
    params.insert(
        "stoploss_trailing_percent".to_string(),
        Value::Number(percent.into()),
    );
    params
}

pub(crate) fn build_candles_params(
    symbol: &str,
    start: NaiveDateTime,
    end: NaiveDateTime,
    timeframe_seconds: i64,
    count: i64,
) -> Result<Map<String, Value>, TradernetError> {
    if symbol.trim().is_empty() {
        return Err(TradernetError::InvalidInput(
            "Symbol cannot be empty".to_string(),
        ));
    }
    if end < start {
        return Err(TradernetError::InvalidInput(
            "date_to cannot be earlier than date_from".to_string(),
        ));
    }
    if count >= 0 {
        return Err(TradernetError::InvalidInput(
            "count must be negative (-1 means no extra candles)".to_string(),
        ));
    }
    if timeframe_seconds <= 0 || timeframe_seconds % 60 != 0 {
        return Err(TradernetError::InvalidInput(
            "timeframe must be a positive number of seconds divisible by 60".to_string(),
        ));
    }

    let timeframe_minutes = timeframe_seconds / 60;
    if !matches!(timeframe_minutes, 1 | 5 | 15 | 60 | 1440) {
        return Err(TradernetError::InvalidInput(
            "unsupported timeframe, allowed values: 60, 300, 900, 3600, 86400 seconds".to_string(),
        ));
    }

    let mut params = Map::new();
    params.insert("id".to_string(), Value::String(symbol.to_string()));
    params.insert("count".to_string(), Value::Number(count.into()));
    params.insert(
        "timeframe".to_string(),
        Value::Number(timeframe_minutes.into()),
    );
    params.insert(
        "date_from".to_string(),
        Value::String(start.format("%d.%m.%Y %H:%M").to_string()),
    );
    params.insert(
        "date_to".to_string(),
        Value::String(end.format("%d.%m.%Y %H:%M").to_string()),
    );
    params.insert(
        "intervalMode".to_string(),
        Value::String("ClosedRay".to_string()),
    );
    Ok(params)
}

pub(crate) fn build_take_profit_params(
    symbol: &str,
    price: f64,
) -> Result<Map<String, Value>, TradernetError> {
    let mut params = Map::new();
    params.insert("instr_name".to_string(), Value::String(symbol.to_string()));
    let price = serde_json::Number::from_f64(price)
        .ok_or_else(|| TradernetError::InvalidInput("Invalid price".to_string()))?;
    params.insert("take_profit".to_string(), Value::Number(price));
    Ok(params)
}

pub(crate) fn parse_refbook_archive(
    content: &[u8],
) -> Result<Vec<Map<String, Value>>, TradernetError> {
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(content))?;
    if archive.len() != 1 {
        return Err(TradernetError::InvalidInput(
            "More than one file in the archive".to_string(),
        ));
    }

    let mut file = archive.by_index(0)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let parsed: Vec<Map<String, Value>> = serde_json::from_str(&json)?;
    Ok(parsed)
}

pub(crate) fn parse_latest_refbook_dates(content: &str) -> Result<String, TradernetError> {
    let regex = Regex::new(r"\d{4}-\d{2}-\d{2}/")
        .map_err(|err| TradernetError::InvalidInput(err.to_string()))?;
    let mut dates = regex
        .find_iter(content)
        .map(|mat| mat.as_str().trim_end_matches('/').to_string())
        .collect::<Vec<_>>();
    dates.sort();
    dates
        .pop()
        .ok_or_else(|| TradernetError::InvalidInput("No refbook dates found".to_string()))
}

pub(crate) fn parse_refbooks(content: &str) -> Result<Vec<String>, TradernetError> {
    let regex = Regex::new(r"([A-Za-z0-9_]+)\.json\.zip")
        .map_err(|err| TradernetError::InvalidInput(err.to_string()))?;
    let mut result = regex
        .captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|value| value.as_str().to_string()))
        .collect::<Vec<_>>();
    result.sort();
    result.dedup();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::build_candles_params;
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn build_candles_params_builds_closed_ray_payload() {
        let start = NaiveDate::from_ymd_opt(2026, 1, 1)
            .expect("valid date")
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"));
        let end = NaiveDate::from_ymd_opt(2026, 1, 2)
            .expect("valid date")
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"));

        let params =
            build_candles_params("FB.US", start, end, 86_400, -1).expect("valid candle payload");

        assert_eq!(params.get("id").and_then(|v| v.as_str()), Some("FB.US"));
        assert_eq!(params.get("count").and_then(|v| v.as_i64()), Some(-1));
        assert_eq!(params.get("timeframe").and_then(|v| v.as_i64()), Some(1440));
        assert_eq!(
            params.get("intervalMode").and_then(|v| v.as_str()),
            Some("ClosedRay")
        );
        assert_eq!(
            params.get("date_from").and_then(|v| v.as_str()),
            Some("01.01.2026 00:00")
        );
        assert_eq!(
            params.get("date_to").and_then(|v| v.as_str()),
            Some("02.01.2026 00:00")
        );
    }

    #[test]
    fn build_candles_params_rejects_invalid_timeframe() {
        let start = NaiveDate::from_ymd_opt(2026, 1, 1)
            .expect("valid date")
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"));
        let end = NaiveDate::from_ymd_opt(2026, 1, 2)
            .expect("valid date")
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"));

        let error =
            build_candles_params("FB.US", start, end, 120, -1).expect_err("2 minutes unsupported");
        assert!(error.to_string().contains("unsupported timeframe"));
    }

    #[test]
    fn build_candles_params_rejects_non_negative_count() {
        let start = NaiveDate::from_ymd_opt(2026, 1, 1)
            .expect("valid date")
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"));
        let end = NaiveDate::from_ymd_opt(2026, 1, 2)
            .expect("valid date")
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"));

        let error =
            build_candles_params("FB.US", start, end, 60, 0).expect_err("count must be negative");
        assert!(error.to_string().contains("count must be negative"));
    }
}
