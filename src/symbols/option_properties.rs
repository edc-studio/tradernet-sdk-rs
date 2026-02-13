use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OptionProperties {
    pub ticker: String,
    pub location: Option<String>,
    pub right: i32,
    pub strike: Decimal,
    pub maturity_date: NaiveDate,
    pub symbolic_expiration: String,
}