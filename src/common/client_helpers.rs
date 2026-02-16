use crate::errors::TradernetError;
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