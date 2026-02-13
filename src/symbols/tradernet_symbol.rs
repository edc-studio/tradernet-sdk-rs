use crate::client::Tradernet;
use crate::errors::TradernetError;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use serde_json::Value;

pub struct TradernetSymbol {
    pub symbol: String,
    pub api: Option<Tradernet>,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub timestamps: Vec<NaiveDateTime>,
    pub candles: Vec<[f64; 4]>,
    pub volumes: Vec<i64>,
    pub timeframe: i64,
}

impl TradernetSymbol {
    pub fn new(
        symbol: &str,
        api: Option<Tradernet>,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Self {
        Self {
            symbol: symbol.to_string(),
            api,
            start,
            end,
            timestamps: Vec::new(),
            candles: Vec::new(),
            volumes: Vec::new(),
            timeframe: 86_400,
        }
    }

    pub fn get_data(&mut self) -> Result<&mut Self, TradernetError> {
        if self.api.is_none() {
            self.api = Some(Tradernet::new(None, None)?);
        }

        let response = self
            .api
            .as_ref()
            .expect("Tradernet instance should be available")
            .get_candles(&self.symbol, self.start, self.end, self.timeframe)?;

        self.timestamps = parse_timestamps(&response, &self.symbol);
        self.candles = parse_candles(&response, &self.symbol);
        self.volumes = parse_volumes(&response, &self.symbol);

        Ok(self)
    }
}

fn parse_timestamps(response: &Value, symbol: &str) -> Vec<NaiveDateTime> {
    response
        .get("xSeries")
        .and_then(|value| value.get(symbol))
        .and_then(|value| value.as_array())
        .map(|series| {
            series
                .iter()
                .filter_map(|value| value.as_i64())
                .filter_map(|seconds| DateTime::<Utc>::from_timestamp(seconds, 0))
                .map(|timestamp| timestamp.naive_utc() + Duration::hours(3))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn parse_candles(response: &Value, symbol: &str) -> Vec<[f64; 4]> {
    response
        .get("hloc")
        .and_then(|value| value.get(symbol))
        .and_then(|value| value.as_array())
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| entry.as_array())
                .filter_map(|values| {
                    if values.len() < 4 {
                        return None;
                    }
                    Some([
                        values.get(0).and_then(|v| v.as_f64())?,
                        values.get(1).and_then(|v| v.as_f64())?,
                        values.get(2).and_then(|v| v.as_f64())?,
                        values.get(3).and_then(|v| v.as_f64())?,
                    ])
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn parse_volumes(response: &Value, symbol: &str) -> Vec<i64> {
    response
        .get("vl")
        .and_then(|value| value.get(symbol))
        .and_then(|value| value.as_array())
        .map(|entries| {
            entries
                .iter()
                .filter_map(|value| value.as_i64())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}