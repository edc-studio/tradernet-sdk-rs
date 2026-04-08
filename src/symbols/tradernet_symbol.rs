use crate::client::Tradernet;
use crate::errors::TradernetError;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

/// Helper for downloading and parsing candle data for a symbol.
pub struct TradernetSymbol {
    /// Symbol identifier (e.g. `AAPL.US`).
    pub symbol: String,
    /// Optional REST client instance to reuse.
    pub api: Option<Tradernet>,
    /// Start date/time for candles.
    pub start: NaiveDateTime,
    /// End date/time for candles.
    pub end: NaiveDateTime,
    /// Parsed candle timestamps.
    pub timestamps: Vec<NaiveDateTime>,
    /// Parsed candle OHLC values.
    pub candles: Vec<[f64; 4]>,
    /// Parsed candle volumes.
    pub volumes: Vec<i64>,
    /// Candle timeframe in seconds.
    pub timeframe: i64,
}

impl TradernetSymbol {
    /// Creates a new symbol helper with an optional API client.
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

    /// Downloads candles and populates `timestamps`, `candles`, and `volumes`.
    pub fn get_data(&mut self) -> Result<&mut Self, TradernetError> {
        if self.api.is_none() {
            self.api = Some(Tradernet::new(None, None)?);
        }

        let response = self
            .api
            .as_ref()
            .expect("Tradernet instance should be available")
            .get_candles_typed(&self.symbol, self.start, self.end, self.timeframe)?;

        self.timestamps = response
            .x_series
            .get(&self.symbol)
            .map(|series| {
                series
                    .iter()
                    .filter_map(|seconds| DateTime::<Utc>::from_timestamp(*seconds, 0))
                    .map(|timestamp| timestamp.naive_utc() + Duration::hours(3))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        self.candles = response
            .hloc
            .get(&self.symbol)
            .map(|entries| {
                entries
                    .iter()
                    .map(|entry| [entry.high, entry.low, entry.open, entry.close])
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        self.volumes = response.vl.get(&self.symbol).cloned().unwrap_or_default();

        Ok(self)
    }
}
