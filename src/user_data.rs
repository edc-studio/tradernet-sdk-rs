use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataResponse {
    #[serde(rename = "OPQ")]
    pub opq: Opq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Opq {
    #[serde(deserialize_with = "deserialize_i64_rev")]
    pub rev: i64,
    #[serde(deserialize_with = "deserialize_i64_init_margin")]
    pub init_margin: i64,
    pub brief_nm: String,
    #[serde(deserialize_with = "deserialize_i64_reception")]
    pub reception: i64,
    #[serde(deserialize_with = "deserialize_i64_active")]
    pub active: i64,
    pub quotes: Quotes,
    pub ps: PortfolioSummary,
    pub orders: Orders,
    pub sess: Vec<Value>,
    pub markets: MarketsWrapper,
    pub source: String,
    pub offbalance: Offbalance,
    #[serde(rename = "homeCurrency")]
    pub home_currency: String,
    #[serde(rename = "userLists")]
    pub user_lists: UserLists,
    #[serde(rename = "NO_ORDER_GROWLS")]
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub no_order_growls: Option<String>,
    #[serde(rename = "userInfo")]
    pub user_info: UserInfo,
    #[serde(rename = "userOptions")]
    pub user_options: UserOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quotes {
    pub q: Vec<Quote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub acd: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_baf")]
    pub baf: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub bap: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub bas: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub base_contract_code: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub base_currency: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub base_ltr: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_bbf")]
    pub bbf: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub bbp: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub bbs: Option<f64>,
    pub c: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub chg: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub chg110: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub chg22: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub chg220: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub chg5: Option<f64>,
    #[serde(rename = "ClosePrice")]
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub close_price: Option<f64>,
    pub codesub_nm: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub cpn: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_cpp")]
    pub cpp: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_dpb")]
    pub dpb: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_dps")]
    pub dps: Option<i64>,
    pub emitent_type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub fv: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_init")]
    pub init: Option<i64>,
    pub ipo: Option<Value>,
    pub issue_nb: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_kind")]
    pub kind: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub ltp: Option<f64>,
    pub ltr: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub lts: Option<f64>,
    pub ltt: Option<String>,
    #[serde(rename = "marketStatus")]
    pub market_status: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub maxtp: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub min_step: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub mintp: Option<f64>,
    pub mrg: Option<String>,
    pub mtd: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_n")]
    pub n: Option<i64>,
    pub name: Option<String>,
    pub name2: Option<String>,
    pub ncd: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_ncp")]
    pub ncp: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub op: Option<f64>,
    pub option_type: Option<String>,
    pub otc_instr: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub p110: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub p22: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub p220: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub p5: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub pcp: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub pp: Option<f64>,
    pub quote_basis: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_rev")]
    pub rev: Option<i64>,
    pub scheme_calc: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub step_price: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_strike_price")]
    pub strike_price: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_trades")]
    pub trades: Option<i64>,
    #[serde(rename = "TradingReferencePrice")]
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub trading_reference_price: Option<f64>,
    #[serde(rename = "TradingSessionSubID")]
    pub trading_session_sub_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, deserialize_with = "deserialize_option_i64_quote_type")]
    pub quote_type: Option<i64>,
    #[serde(rename = "UTCOffset")]
    #[serde(default, deserialize_with = "deserialize_option_i64_utc_offset")]
    pub utc_offset: Option<i64>,
    pub virt_base_instr: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub vlt: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub vol: Option<f64>,
    pub x_agg_futures: Option<String>,
    pub x_curr: Option<String>,
    #[serde(rename = "x_currVal")]
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub x_curr_val: Option<f64>,
    pub x_descr: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_x_dsc1")]
    pub x_dsc1: Option<i64>,
    pub x_dsc1_reception: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_x_dsc2")]
    pub x_dsc2: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_x_dsc3")]
    pub x_dsc3: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_x_istrade")]
    pub x_istrade: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub x_lot: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub x_max: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub x_min: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub x_min_lot_q: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_x_short")]
    pub x_short: Option<i64>,
    pub x_short_reception: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_f64")]
    pub yld: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_yld_ytm_ask")]
    pub yld_ytm_ask: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_yld_ytm_bid")]
    pub yld_ytm_bid: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioSummary {
    pub loaded: bool,
    pub acc: Vec<PortfolioAccount>,
    pub pos: Vec<PortfolioPosition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioAccount {
    pub curr: String,
    #[serde(deserialize_with = "deserialize_f64")]
    pub currval: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub forecast_in: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub forecast_out: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub t2_in: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub t2_out: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub s: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioPosition {
    #[serde(deserialize_with = "deserialize_f64")]
    pub open_bal: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub mkt_price: f64,
    pub name: String,
    pub i: String,
    #[serde(deserialize_with = "deserialize_i64_t")]
    pub t: i64,
    pub scheme_calc: String,
    #[serde(default, deserialize_with = "deserialize_option_i64_instr_id")]
    pub instr_id: Option<i64>,
    #[serde(rename = "Yield")]
    #[serde(deserialize_with = "deserialize_i64_yield_value")]
    pub yield_value: i64,
    pub issue_nb: String,
    #[serde(deserialize_with = "deserialize_f64")]
    pub profit_price: f64,
    #[serde(deserialize_with = "deserialize_i64_acc_pos_id")]
    pub acc_pos_id: i64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub accruedint_a: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub acd: f64,
    #[serde(deserialize_with = "deserialize_i64_k")]
    pub k: i64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub bal_price_a: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub price_a: f64,
    pub base_currency: String,
    #[serde(deserialize_with = "deserialize_f64")]
    pub face_val_a: f64,
    pub curr: String,
    #[serde(deserialize_with = "deserialize_i64_go")]
    pub go: i64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub profit_close: f64,
    #[serde(deserialize_with = "deserialize_i64_fv")]
    pub fv: i64,
    #[serde(deserialize_with = "deserialize_i64_vm")]
    pub vm: i64,
    #[serde(deserialize_with = "deserialize_i64_q")]
    pub q: i64,
    pub name2: String,
    #[serde(deserialize_with = "deserialize_f64")]
    pub market_value: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub close_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub currval: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub s: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orders {
    pub loaded: bool,
    pub order: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketsWrapper {
    pub markets: Markets,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Markets {
    pub t: String,
    pub m: Vec<Market>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Market {
    pub n: String,
    pub n2: String,
    pub s: String,
    pub o: String,
    pub c: String,
    #[serde(deserialize_with = "deserialize_i64_dt")]
    pub dt: i64,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub p: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub post: Option<String>,
    pub date: Option<Vec<MarketDate>>,
    pub ev: Option<Vec<MarketEvent>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketDate {
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub from: String,
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub to: String,
    #[serde(deserialize_with = "deserialize_i64_dayoff")]
    pub dayoff: i64,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketEvent {
    pub id: String,
    pub t: String,
    pub next: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offbalance {
    #[serde(deserialize_with = "deserialize_i64_net_assets")]
    pub net_assets: i64,
    pub pos: Vec<Value>,
    pub acc: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLists {
    #[serde(
        default,
        rename = "userStockLists",
        deserialize_with = "deserialize_user_stock_lists"
    )]
    pub user_stock_lists: UserStockLists,
    #[serde(rename = "userStockListSelected")]
    pub user_stock_list_selected: String,
    #[serde(rename = "stocksArray")]
    pub stocks_array: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserStockLists {
    #[serde(default)]
    pub default: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(default, deserialize_with = "deserialize_option_i64_id")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_group_id")]
    pub group_id: Option<i64>,
    pub login: Option<String>,
    pub lastname: Option<String>,
    pub firstname: Option<String>,
    pub middlename: Option<String>,
    pub last_first_middle_name: Option<String>,
    pub first_last_name: Option<String>,
    pub email: Option<String>,
    pub mod_tmstmp: Option<String>,
    pub rec_tmstmp: Option<String>,
    pub last_visit_tmstmp: Option<String>,
    pub umod_tmstmp: Option<String>,
    pub date_tsmod: Option<String>,
    pub date_last_request: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_f_active")]
    pub f_active: Option<i64>,
    pub trader_systems_id: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_f_demo")]
    pub f_demo: Option<i64>,
    pub birthday: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub sex: Option<String>,
    pub citizenship: Option<String>,
    pub citizenship_code: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "type")]
    pub user_type: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_status_id")]
    pub status_id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub utm_campaign: Option<String>,
    pub auth_login: Option<String>,
    pub settlement_pair: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub description: Option<String>,
    pub tel: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub fb_uid: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_robot")]
    pub robot: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub minimum_investment: Option<String>,
    pub language: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_additional_status")]
    pub additional_status: Option<i64>,
    pub profilename: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_reception")]
    pub reception: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_reception_service")]
    pub reception_service: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub briefnm_additional: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_manager_user_id")]
    pub manager_user_id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub google_id: Option<String>,
    pub details: Option<UserInfoDetails>,
    pub inn: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub country: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_original_client_user_id")]
    pub original_client_user_id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub contact_id: Option<String>,
    pub role_name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_role")]
    pub role: Option<i64>,
    pub date_open_real: Option<String>,
    pub numdoc: Option<String>,
    pub docseries: Option<String>,
    pub regname: Option<String>,
    pub regcode: Option<String>,
    pub datedoc: Option<String>,
    pub documents: Option<String>,
    pub bornplace: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_f_kval")]
    pub f_kval: Option<i64>,
    pub account_block_date: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub client_date_close: Option<String>,
    pub date_client_doc_received: Option<String>,
    pub iis: Option<String>,
    pub isleadaccount: Option<String>,
    pub mkt_codes: Option<String>,
    pub object_type: Option<String>,
    pub registered_at_domain: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_email_confirm")]
    pub email_confirm: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_blocks_count")]
    pub blocks_count: Option<i64>,
    #[serde(rename = "isIpoAvailable")]
    pub is_ipo_available: Option<bool>,
    #[serde(rename = "currentlyAvailableIpos")]
    #[serde(default, deserialize_with = "deserialize_option_i64_currently_available_ipos")]
    pub currently_available_ipos: Option<i64>,
    #[serde(rename = "isSubscribedToNewIpos")]
    #[serde(default, deserialize_with = "deserialize_option_i64_is_subscribed_to_new_ipos")]
    pub is_subscribed_to_new_ipos: Option<i64>,
    #[serde(rename = "isStockBonusAvailable")]
    pub is_stock_bonus_available: Option<bool>,
    #[serde(rename = "stockBonusIdKey")]
    pub stock_bonus_id_key: Option<bool>,
    #[serde(rename = "kassaNovaInvestCardAvailable")]
    pub kassa_nova_invest_card_available: Option<bool>,
    pub messages_counts: Option<MessageCounts>,
    #[serde(rename = "tariffDetails")]
    pub tariff_details: Option<TariffDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoDetails {
    pub iis: Option<String>,
    pub push: Option<PushSettings>,
    pub comment: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub smev_sms: Option<String>,
    pub statuses: Option<HashMap<String, String>>,
    pub mkt_codes: Option<HashMap<String, String>>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub telegram_id: Option<String>,
    pub telegram_bot: Option<bool>,
    #[serde(rename = "Date register")]
    pub date_register: Option<String>,
    #[serde(rename = "isLeadAccount")]
    pub is_lead_account: Option<bool>,
    #[serde(rename = "Date open real")]
    pub date_open_real: Option<String>,
    pub passport_check: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_mail_subscription")]
    pub mail_subscription: Option<i64>,
    pub ffinbank_requisites: Option<FfinbankRequisites>,
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub initial_telegram_id: Option<String>,
    pub passport_check_date: Option<String>,
    #[serde(rename = "lastShownDateMessage")]
    pub last_shown_date_message: Option<HashMap<String, i64>>,
    #[serde(rename = "utm_campaign - to Real")]
    #[serde(default, deserialize_with = "deserialize_option_string_or_number")]
    pub utm_campaign_to_real: Option<String>,
    #[serde(rename = "utm_campaign - Register")]
    pub utm_campaign_register: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_telegram_last_updated_at")]
    pub telegram_last_updated_at: Option<i64>,
    pub personal_anketa_last_date: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_detected_reception_service")]
    pub detected_reception_service: Option<i64>,
}

fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(value) => Ok(value),
        Value::Number(value) => Ok(value.to_string()),
        Value::Bool(value) => Ok(value.to_string()),
        Value::Null => Ok(String::new()),
        _ => Err(serde::de::Error::custom("expected string or number")),
    }
}

fn deserialize_option_string_or_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value)),
        Some(Value::Number(value)) => Ok(Some(value.to_string())),
        Some(Value::Bool(value)) => Ok(Some(value.to_string())),
        _ => Err(serde::de::Error::custom("expected string or number")),
    }
}

fn deserialize_option_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(value)) => value
            .as_f64()
            .map(Some)
            .ok_or_else(|| serde::de::Error::custom("expected number")),
        Some(Value::String(value)) => {
            let value = value.trim();
            if value.is_empty() {
                Ok(None)
            } else {
                value
                    .parse::<f64>()
                    .map(Some)
                    .map_err(|_| serde::de::Error::custom("expected float"))
            }
        }
        Some(Value::Bool(value)) => Ok(Some(if value { 1.0 } else { 0.0 })),
        _ => Err(serde::de::Error::custom("expected number")),
    }
}

fn deserialize_user_stock_lists<'de, D>(deserializer: D) -> Result<UserStockLists, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    match value {
        None | Some(Value::Null) => Ok(UserStockLists::default()),
        Some(Value::Object(map)) => serde_json::from_value(Value::Object(map))
            .map_err(serde::de::Error::custom),
        Some(Value::Array(items)) => {
            if items.is_empty() {
                return Ok(UserStockLists::default());
            }

            if items.iter().all(|item| item.as_str().is_some()) {
                let default = items
                    .into_iter()
                    .filter_map(|item| item.as_str().map(|value| value.to_string()))
                    .collect();
                return Ok(UserStockLists { default });
            }

            let mut merged = serde_json::Map::new();
            for item in items {
                match item {
                    Value::Object(map) => merged.extend(map),
                    _ => {
                        return Err(serde::de::Error::custom(
                            "expected userStockLists as object or array",
                        ))
                    }
                }
            }

            serde_json::from_value(Value::Object(merged)).map_err(serde::de::Error::custom)
        }
        Some(_) => Err(serde::de::Error::custom(
            "expected userStockLists as object or array",
        )),
    }
}

macro_rules! make_deserialize_i64 {
    ($name:ident, $field:expr) => {
        fn $name<'de, D>(deserializer: D) -> Result<i64, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = Value::deserialize(deserializer)?;
            coerce_i64(value, $field)
        }
    };
}

macro_rules! make_deserialize_option_i64 {
    ($name:ident, $field:expr) => {
        fn $name<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = Option::<Value>::deserialize(deserializer)?;
            match value {
                None | Some(Value::Null) => Ok(None),
                Some(value) => coerce_i64(value, $field).map(Some),
            }
        }
    };
}

make_deserialize_i64!(deserialize_i64_rev, "rev");
make_deserialize_i64!(deserialize_i64_init_margin, "init_margin");
make_deserialize_i64!(deserialize_i64_reception, "reception");
make_deserialize_i64!(deserialize_i64_active, "active");
make_deserialize_option_i64!(deserialize_option_i64_bbf, "bbf");
make_deserialize_option_i64!(deserialize_option_i64_rev, "rev");
make_deserialize_i64!(deserialize_i64_t, "t");
make_deserialize_option_i64!(deserialize_option_i64_baf, "baf");
make_deserialize_option_i64!(deserialize_option_i64_instr_id, "instr_id");
make_deserialize_option_i64!(deserialize_option_i64_cpp, "cpp");
make_deserialize_option_i64!(deserialize_option_i64_dpb, "dpb");
make_deserialize_option_i64!(deserialize_option_i64_dps, "dps");
make_deserialize_option_i64!(deserialize_option_i64_init, "init");
make_deserialize_option_i64!(deserialize_option_i64_kind, "kind");
make_deserialize_option_i64!(deserialize_option_i64_n, "n");
make_deserialize_option_i64!(deserialize_option_i64_ncp, "ncp");
make_deserialize_option_i64!(deserialize_option_i64_strike_price, "strike_price");
make_deserialize_option_i64!(deserialize_option_i64_trades, "trades");
make_deserialize_option_i64!(deserialize_option_i64_quote_type, "type");
make_deserialize_option_i64!(deserialize_option_i64_utc_offset, "UTCOffset");
make_deserialize_option_i64!(deserialize_option_i64_x_dsc1, "x_dsc1");
make_deserialize_option_i64!(deserialize_option_i64_x_dsc2, "x_dsc2");
make_deserialize_option_i64!(deserialize_option_i64_x_dsc3, "x_dsc3");
make_deserialize_option_i64!(deserialize_option_i64_x_istrade, "x_istrade");
make_deserialize_option_i64!(deserialize_option_i64_x_short, "x_short");
make_deserialize_option_i64!(deserialize_option_i64_yld_ytm_ask, "yld_ytm_ask");
make_deserialize_option_i64!(deserialize_option_i64_yld_ytm_bid, "yld_ytm_bid");
make_deserialize_i64!(deserialize_i64_yield_value, "Yield");
make_deserialize_i64!(deserialize_i64_acc_pos_id, "acc_pos_id");
make_deserialize_i64!(deserialize_i64_k, "k");
make_deserialize_i64!(deserialize_i64_go, "go");
make_deserialize_i64!(deserialize_i64_fv, "fv");
make_deserialize_i64!(deserialize_i64_vm, "vm");
make_deserialize_i64!(deserialize_i64_q, "q");
make_deserialize_i64!(deserialize_i64_dt, "dt");
make_deserialize_i64!(deserialize_i64_dayoff, "dayoff");
make_deserialize_i64!(deserialize_i64_net_assets, "net_assets");
make_deserialize_i64!(deserialize_i64_id, "id");
make_deserialize_option_i64!(deserialize_option_i64_id, "id");
make_deserialize_option_i64!(deserialize_option_i64_group_id, "group_id");
make_deserialize_option_i64!(deserialize_option_i64_f_active, "f_active");
make_deserialize_option_i64!(deserialize_option_i64_f_demo, "f_demo");
make_deserialize_option_i64!(deserialize_option_i64_status_id, "status_id");
make_deserialize_option_i64!(deserialize_option_i64_robot, "robot");
make_deserialize_option_i64!(deserialize_option_i64_additional_status, "additional_status");
make_deserialize_option_i64!(deserialize_option_i64_reception, "reception");
make_deserialize_option_i64!(deserialize_option_i64_reception_service, "reception_service");
make_deserialize_option_i64!(deserialize_option_i64_manager_user_id, "manager_user_id");
make_deserialize_option_i64!(
    deserialize_option_i64_original_client_user_id,
    "original_client_user_id"
);
make_deserialize_option_i64!(deserialize_option_i64_role, "role");
make_deserialize_option_i64!(deserialize_option_i64_f_kval, "f_kval");
make_deserialize_option_i64!(deserialize_option_i64_email_confirm, "email_confirm");
make_deserialize_option_i64!(deserialize_option_i64_blocks_count, "blocks_count");
make_deserialize_option_i64!(
    deserialize_option_i64_currently_available_ipos,
    "currentlyAvailableIpos"
);
make_deserialize_option_i64!(
    deserialize_option_i64_is_subscribed_to_new_ipos,
    "isSubscribedToNewIpos"
);
make_deserialize_option_i64!(
    deserialize_option_i64_mail_subscription,
    "mail_subscription"
);
make_deserialize_option_i64!(
    deserialize_option_i64_telegram_last_updated_at,
    "telegram_last_updated_at"
);
make_deserialize_option_i64!(
    deserialize_option_i64_detected_reception_service,
    "detected_reception_service"
);
make_deserialize_i64!(deserialize_i64_no_read, "no_read");
make_deserialize_i64!(deserialize_i64_all, "all");
make_deserialize_option_i64!(deserialize_option_i64_cost_open, "cost_open");
make_deserialize_option_i64!(deserialize_option_i64_cost_last, "cost_last");
make_deserialize_option_i64!(deserialize_option_i64_cost_low, "cost_low");
make_deserialize_option_i64!(deserialize_option_i64_cost_high, "cost_high");
make_deserialize_option_i64!(deserialize_option_i64_bid_last, "bid_last");
make_deserialize_option_i64!(deserialize_option_i64_offer_last, "offer_last");
make_deserialize_option_i64!(deserialize_option_i64_volume, "volume");
make_deserialize_option_i64!(deserialize_option_i64_graphic_type, "graphic_type");
make_deserialize_option_i64!(deserialize_option_i64_f_transaction, "f_transaction");
make_deserialize_option_i64!(deserialize_option_i64_f_compare_index, "f_compare_index");
make_deserialize_option_i64!(deserialize_option_i64_profile_type, "profile_type");
make_deserialize_option_i64!(deserialize_option_i64_show_portfolio_block, "showPortfolioBlock");
make_deserialize_option_i64!(deserialize_option_i64_page_first_tab_open, "pageFirstTabOpen");
make_deserialize_option_i64!(deserialize_option_i64_access_cost, "access_cost");

fn coerce_i64<E>(value: Value, field: &str) -> Result<i64, E>
where
    E: serde::de::Error,
{
    match value {
        Value::Number(value) => value
            .as_i64()
            .or_else(|| {
                value.as_f64().map(|value| {
                    warn_if_fractional_i64(value, "number", field);
                    value.trunc() as i64
                })
            })
            .ok_or_else(|| serde::de::Error::custom("expected number")),
        Value::String(value) => {
            let value = value.trim();
            if value.is_empty() {
                Ok(0)
            } else if let Ok(parsed) = value.parse::<i64>() {
                Ok(parsed)
            } else {
                value
                    .parse::<f64>()
                    .map(|value| {
                        warn_if_fractional_i64(value, "string", field);
                        value.trunc() as i64
                    })
                    .map_err(|_| serde::de::Error::custom("expected integer"))
            }
        }
        Value::Bool(value) => Ok(i64::from(value)),
        Value::Null => Ok(0),
        _ => Err(serde::de::Error::custom("expected number")),
    }
}

fn warn_if_fractional_i64(value: f64, source: &str, field: &str) {
    if (value - value.trunc()).abs() > f64::EPSILON {
        log::warn!(
            "i64 field `{field}` received fractional {source} value {value}, truncating"
        );
    }
}

fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Number(value) => value
            .as_f64()
            .ok_or_else(|| serde::de::Error::custom("expected number")),
        Value::String(value) => {
            let value = value.trim();
            if value.is_empty() {
                Ok(0.0)
            } else {
                value
                    .parse::<f64>()
                    .map_err(|_| serde::de::Error::custom("expected float"))
            }
        }
        Value::Bool(value) => Ok(if value { 1.0 } else { 0.0 }),
        Value::Null => Ok(0.0),
        _ => Err(serde::de::Error::custom("expected number")),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushSettings {
    #[serde(rename = "android_tn")]
    pub android_tn: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FfinbankRequisites {
    pub date_mod: String,
    pub response: FfinbankResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FfinbankResponse {
    #[serde(rename = "Accounts")]
    pub accounts: Vec<FfinbankAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FfinbankAccount {
    #[serde(rename = "Number")]
    pub number: String,
    #[serde(rename = "Passport")]
    pub passport: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCounts {
    #[serde(deserialize_with = "deserialize_i64_no_read")]
    pub no_read: i64,
    #[serde(deserialize_with = "deserialize_i64_all")]
    pub all: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TariffDetails {
    #[serde(deserialize_with = "deserialize_i64_id")]
    pub id: i64,
    pub name: String,
    pub curr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserOptions {
    #[serde(default, deserialize_with = "deserialize_option_i64_cost_open")]
    pub cost_open: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_cost_last")]
    pub cost_last: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_cost_low")]
    pub cost_low: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_cost_high")]
    pub cost_high: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_bid_last")]
    pub bid_last: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_offer_last")]
    pub offer_last: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_volume")]
    pub volume: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_graphic_type")]
    pub graphic_type: Option<i64>,
    pub graphic_format: Option<String>,
    pub period: Option<String>,
    pub time_period: Option<String>,
    pub interval: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub api_secret: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_f_transaction")]
    pub f_transaction: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_option_i64_f_compare_index")]
    pub f_compare_index: Option<i64>,
    pub graphic_indicators: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_profile_type")]
    pub profile_type: Option<i64>,
    #[serde(rename = "showPortfolioBlock")]
    #[serde(default, deserialize_with = "deserialize_option_i64_show_portfolio_block")]
    pub show_portfolio_block: Option<i64>,
    #[serde(rename = "pageFirstTabOpen")]
    #[serde(default, deserialize_with = "deserialize_option_i64_page_first_tab_open")]
    pub page_first_tab_open: Option<i64>,
    pub cover: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_i64_access_cost")]
    pub access_cost: Option<i64>,
    pub theme: Option<String>,
    #[serde(rename = "showTransactionsMode")]
    pub show_transactions_mode: Option<String>,
    #[serde(rename = "gridPortfolio")]
    pub grid_portfolio: Option<Vec<String>>,
}
