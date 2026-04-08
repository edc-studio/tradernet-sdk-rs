use crate::errors::TradernetError;
use chrono::{NaiveDate, NaiveDateTime};
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

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_trades_history_params(
    start: NaiveDate,
    end: NaiveDate,
    trade_id: Option<i64>,
    limit: Option<i64>,
    symbol: Option<&str>,
    currency: Option<&str>,
    reception: Option<i64>,
) -> Result<Map<String, Value>, TradernetError> {
    if end < start {
        return Err(TradernetError::InvalidInput(
            "end date cannot be earlier than begin date".to_string(),
        ));
    }
    if let Some(trade_id) = trade_id
        && trade_id <= 0
    {
        return Err(TradernetError::InvalidInput(
            "trade_id must be positive".to_string(),
        ));
    }
    if let Some(limit) = limit
        && limit < 0
    {
        return Err(TradernetError::InvalidInput(
            "limit cannot be negative".to_string(),
        ));
    }
    if let Some(symbol) = symbol
        && symbol.trim().is_empty()
    {
        return Err(TradernetError::InvalidInput(
            "symbol cannot be empty".to_string(),
        ));
    }
    if let Some(currency) = currency
        && currency.trim().is_empty()
    {
        return Err(TradernetError::InvalidInput(
            "currency cannot be empty".to_string(),
        ));
    }
    if let Some(reception) = reception
        && reception < 0
    {
        return Err(TradernetError::InvalidInput(
            "reception cannot be negative".to_string(),
        ));
    }

    let mut params = Map::new();
    params.insert("beginDate".to_string(), Value::String(start.to_string()));
    params.insert("endDate".to_string(), Value::String(end.to_string()));
    if let Some(trade_id) = trade_id {
        params.insert("tradeId".to_string(), Value::Number(trade_id.into()));
    }
    if let Some(limit) = limit {
        params.insert("max".to_string(), Value::Number(limit.into()));
    }
    if let Some(symbol) = symbol {
        params.insert("nt_ticker".to_string(), Value::String(symbol.to_string()));
    }
    if let Some(currency) = currency {
        params.insert("curr".to_string(), Value::String(currency.to_string()));
    }
    if let Some(reception) = reception {
        params.insert("reception".to_string(), Value::Number(reception.into()));
    }

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
    use super::{build_candles_params, build_trades_history_params};
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

    #[test]
    fn build_trades_history_params_includes_reception() {
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date");
        let end = NaiveDate::from_ymd_opt(2026, 1, 31).expect("valid date");

        let params = build_trades_history_params(
            start,
            end,
            Some(123),
            Some(100),
            Some("AAPL.US"),
            Some("USD"),
            Some(1),
        )
        .expect("valid payload");

        assert_eq!(
            params.get("beginDate").and_then(|v| v.as_str()),
            Some("2026-01-01")
        );
        assert_eq!(params.get("tradeId").and_then(|v| v.as_i64()), Some(123));
        assert_eq!(params.get("max").and_then(|v| v.as_i64()), Some(100));
        assert_eq!(
            params.get("nt_ticker").and_then(|v| v.as_str()),
            Some("AAPL.US")
        );
        assert_eq!(params.get("curr").and_then(|v| v.as_str()), Some("USD"));
        assert_eq!(params.get("reception").and_then(|v| v.as_i64()), Some(1));
    }

    #[test]
    fn build_trades_history_params_rejects_negative_limit() {
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date");
        let end = NaiveDate::from_ymd_opt(2026, 1, 31).expect("valid date");

        let error = build_trades_history_params(start, end, None, Some(-1), None, None, None)
            .expect_err("negative limit is invalid");
        assert!(error.to_string().contains("limit cannot be negative"));
    }
}
