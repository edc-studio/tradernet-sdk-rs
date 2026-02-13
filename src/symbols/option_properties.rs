use chrono::NaiveDate;
use rust_decimal::Decimal;

/// Parsed option contract attributes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OptionProperties {
    /// Underlying ticker symbol.
    pub ticker: String,
    /// Optional trading venue suffix (e.g. `US`).
    pub location: Option<String>,
    /// Option right (call/put marker as used by Tradernet).
    pub right: i32,
    /// Strike price of the option.
    pub strike: Decimal,
    /// Maturity date of the contract.
    pub maturity_date: NaiveDate,
    /// Symbolic expiration code used by Tradernet.
    pub symbolic_expiration: String,
}