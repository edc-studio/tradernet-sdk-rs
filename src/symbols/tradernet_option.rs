use crate::errors::TradernetError;
use crate::symbols::option_properties::OptionProperties;
use chrono::NaiveDate;
use regex::Regex;
use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Parser and formatter for Tradernet option notation.
#[derive(Clone, Debug)]
pub struct TradernetOption {
    symbol: String,
    properties: OptionProperties,
}

impl TradernetOption {
    /// Parses a Tradernet option symbol into a structured representation.
    pub fn new(symbol: &str) -> Result<Self, TradernetError> {
        let properties = Self::decode_notation(symbol)?;
        Ok(Self {
            symbol: symbol.to_string(),
            properties,
        })
    }

    /// Returns the original option symbol string.
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Returns the underlying ticker.
    pub fn ticker(&self) -> &str {
        &self.properties.ticker
    }

    /// Returns the optional market/location suffix.
    pub fn location(&self) -> Option<&str> {
        self.properties.location.as_deref()
    }

    /// Returns the numeric option right (call = `1`, put = `-1`).
    pub fn right(&self) -> i32 {
        self.properties.right
    }

    /// Returns the option strike price.
    pub fn strike(&self) -> Decimal {
        self.properties.strike
    }

    /// Returns the option maturity date.
    pub fn maturity_date(&self) -> NaiveDate {
        self.properties.maturity_date
    }

    /// Returns the symbolic expiration string used by Tradernet.
    pub fn symbolic_expiration(&self) -> &str {
        &self.properties.symbolic_expiration
    }

    /// Returns the combined underlying symbol (ticker + optional location).
    pub fn underlying(&self) -> String {
        match &self.properties.location {
            Some(location) if !location.is_empty() => {
                format!("{}.{}", self.properties.ticker, location)
            }
            _ => self.properties.ticker.clone(),
        }
    }

    /// Returns the symbolic right (`C` for call, `P` for put).
    pub fn symbolic_right(&self) -> &str {
        if self.right() == 1 { "C" } else { "P" }
    }

    /// Converts a boolean right to numeric form (`1` for call, `-1` for put).
    pub fn numeric_right(is_call: bool) -> i32 {
        if is_call { 1 } else { -1 }
    }

    /// Returns the OSI symbol representation for the option.
    pub fn osi(&self) -> String {
        let expiration = self.maturity_date().format("%y%m%d").to_string();
        let strike = self.strike().to_string();
        let mut parts = strike.split('.');
        let dollars = parts.next().unwrap_or("0");
        let cents = parts.next().unwrap_or("0");
        let dollar = format!("{:0>5}", dollars);
        let cent = format!("{:0>3}", cents);
        format!(
            "{}{}{}{}{}",
            self.ticker(),
            expiration,
            self.symbolic_right(),
            dollar,
            cent
        )
    }

    /// Encodes a date into Tradernet symbolic expiration format.
    pub fn encode_date(date: NaiveDate) -> String {
        date.format("%d%b%Y").to_string().to_uppercase()
    }

    /// Decodes a Tradernet symbolic expiration date.
    pub fn decode_date(symbolic_date: &str) -> Result<NaiveDate, TradernetError> {
        NaiveDate::parse_from_str(symbolic_date, "%d%b%Y")
            .map_err(|err| TradernetError::InvalidInput(err.to_string()))
    }

    /// Parses Tradernet option notation into [`OptionProperties`].
    pub fn decode_notation(symbol: &str) -> Result<OptionProperties, TradernetError> {
        let regex = Regex::new(r"^\+(\D+(\d+)?)\.(\d{2}\D{3}\d{4})\.([CP])(\d+(\.\d*)?)$")
            .map_err(|err| TradernetError::InvalidInput(err.to_string()))?;
        let captures = regex.captures(symbol).ok_or_else(|| {
            TradernetError::InvalidInput(format!("Invalid Tradernet option symbol: {symbol}"))
        })?;

        let ticker = captures
            .get(1)
            .map(|m| m.as_str())
            .unwrap_or_default()
            .to_string();
        let symbolic_expiration = captures.get(3).map(|m| m.as_str()).unwrap_or_default();
        let right = captures.get(4).map(|m| m.as_str()).unwrap_or("C");
        let strike = captures.get(5).map(|m| m.as_str()).unwrap_or("0");

        Ok(OptionProperties {
            ticker,
            location: None,
            right: if right == "P" { -1 } else { 1 },
            strike: Decimal::from_str(strike)
                .map_err(|err| TradernetError::InvalidInput(err.to_string()))?,
            maturity_date: Self::decode_date(symbolic_expiration)?,
            symbolic_expiration: symbolic_expiration.to_string(),
        })
    }
}

impl Display for TradernetOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let right = if self.right() == -1 { "Put" } else { "Call" };
        write!(
            f,
            "{} @ {} {} {}",
            self.underlying(),
            self.strike(),
            right,
            self.maturity_date()
        )
    }
}

impl PartialEq for TradernetOption {
    fn eq(&self, other: &Self) -> bool {
        self.underlying() == other.underlying()
            && self.maturity_date() == other.maturity_date()
            && self.strike() == other.strike()
            && self.right() == other.right()
    }
}

impl Eq for TradernetOption {}

impl PartialOrd for TradernetOption {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TradernetOption {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.underlying(),
            self.maturity_date(),
            self.strike(),
            self.right(),
        )
            .cmp(&(
                other.underlying(),
                other.maturity_date(),
                other.strike(),
                other.right(),
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn parses_tradernet_option_notation() {
        let option = TradernetOption::new("+FRHC.16SEP2022.C55").unwrap();
        assert_eq!(option.ticker(), "FRHC");
        assert_eq!(option.right(), 1);
        assert_eq!(option.strike(), Decimal::from_str("55").unwrap());
        assert_eq!(
            option.maturity_date(),
            NaiveDate::from_ymd_opt(2022, 9, 16).unwrap()
        );
        assert_eq!(option.osi(), "FRHC220916C00055000");
        assert_eq!(option.to_string(), "FRHC @ 55 Call 2022-09-16");
    }
}
