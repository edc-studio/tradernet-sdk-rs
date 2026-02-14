use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataResponse {
    #[serde(rename = "OPQ")]
    pub opq: Opq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Opq {
    pub rev: i64,
    pub init_margin: i64,
    pub brief_nm: String,
    pub reception: i64,
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
    pub acd: i64,
    pub baf: i64,
    pub bap: f64,
    pub bas: i64,
    pub base_contract_code: String,
    pub base_currency: String,
    pub base_ltr: String,
    pub bbf: i64,
    pub bbp: f64,
    pub bbs: f64,
    pub c: String,
    pub chg: f64,
    pub chg110: f64,
    pub chg22: f64,
    pub chg220: f64,
    pub chg5: f64,
    #[serde(rename = "ClosePrice")]
    pub close_price: f64,
    pub codesub_nm: String,
    pub cpn: i64,
    pub cpp: i64,
    pub dpb: i64,
    pub dps: i64,
    pub emitent_type: String,
    pub fv: i64,
    pub init: i64,
    pub ipo: Option<Value>,
    pub issue_nb: String,
    pub kind: i64,
    pub ltp: f64,
    pub ltr: String,
    pub lts: i64,
    pub ltt: String,
    #[serde(rename = "marketStatus")]
    pub market_status: String,
    pub maxtp: f64,
    pub min_step: f64,
    pub mintp: f64,
    pub mrg: String,
    pub mtd: String,
    pub n: i64,
    pub name: String,
    pub name2: String,
    pub ncd: String,
    pub ncp: i64,
    pub op: f64,
    pub option_type: String,
    pub otc_instr: String,
    pub p110: f64,
    pub p22: f64,
    pub p220: f64,
    pub p5: f64,
    pub pcp: f64,
    pub pp: f64,
    pub quote_basis: String,
    pub rev: i64,
    pub scheme_calc: String,
    pub step_price: f64,
    pub strike_price: i64,
    pub trades: i64,
    #[serde(rename = "TradingReferencePrice")]
    pub trading_reference_price: i64,
    #[serde(rename = "TradingSessionSubID")]
    pub trading_session_sub_id: String,
    #[serde(rename = "type")]
    pub quote_type: i64,
    #[serde(rename = "UTCOffset")]
    pub utc_offset: i64,
    pub virt_base_instr: String,
    pub vlt: f64,
    pub vol: i64,
    pub x_agg_futures: String,
    pub x_curr: String,
    #[serde(rename = "x_currVal")]
    pub x_curr_val: f64,
    pub x_descr: String,
    pub x_dsc1: i64,
    pub x_dsc1_reception: String,
    pub x_dsc2: i64,
    pub x_dsc3: i64,
    pub x_istrade: i64,
    pub x_lot: i64,
    pub x_max: f64,
    pub x_min: f64,
    pub x_min_lot_q: String,
    pub x_short: i64,
    pub x_short_reception: String,
    pub yld: i64,
    pub yld_ytm_ask: i64,
    pub yld_ytm_bid: i64,
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
    pub currval: f64,
    pub forecast_in: f64,
    pub forecast_out: f64,
    pub t2_in: f64,
    pub t2_out: f64,
    pub s: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioPosition {
    pub open_bal: f64,
    pub mkt_price: f64,
    pub name: String,
    pub i: String,
    pub t: i64,
    pub scheme_calc: String,
    pub instr_id: Option<i64>,
    #[serde(rename = "Yield")]
    pub yield_value: i64,
    pub issue_nb: String,
    pub profit_price: f64,
    pub acc_pos_id: i64,
    pub accruedint_a: i64,
    pub acd: i64,
    pub k: i64,
    pub bal_price_a: f64,
    pub price_a: f64,
    pub base_currency: String,
    pub face_val_a: f64,
    pub curr: String,
    pub go: i64,
    pub profit_close: f64,
    pub fv: i64,
    pub vm: i64,
    pub q: i64,
    pub name2: String,
    pub market_value: f64,
    pub close_price: f64,
    pub currval: i64,
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
    pub dt: i64,
    pub p: String,
    pub post: String,
    pub date: Vec<MarketDate>,
    pub ev: Vec<MarketEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketDate {
    pub from: String,
    pub to: String,
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
    pub net_assets: i64,
    pub pos: Vec<Value>,
    pub acc: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLists {
    #[serde(rename = "userStockLists")]
    pub user_stock_lists: UserStockLists,
    #[serde(rename = "userStockListSelected")]
    pub user_stock_list_selected: String,
    #[serde(rename = "stocksArray")]
    pub stocks_array: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStockLists {
    pub default: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub group_id: i64,
    pub login: String,
    pub lastname: String,
    pub firstname: String,
    pub middlename: String,
    pub last_first_middle_name: String,
    pub first_last_name: String,
    pub email: String,
    pub mod_tmstmp: String,
    pub rec_tmstmp: String,
    pub last_visit_tmstmp: String,
    pub umod_tmstmp: String,
    pub date_tsmod: String,
    pub date_last_request: String,
    pub f_active: i64,
    pub trader_systems_id: String,
    pub f_demo: i64,
    pub birthday: String,
    pub sex: Option<String>,
    pub citizenship: String,
    pub citizenship_code: String,
    pub status: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub status_id: i64,
    pub utm_campaign: Option<String>,
    pub auth_login: String,
    pub settlement_pair: String,
    pub description: Option<String>,
    pub tel: String,
    pub fb_uid: Option<String>,
    pub robot: i64,
    pub minimum_investment: Option<String>,
    pub language: String,
    pub additional_status: i64,
    pub profilename: String,
    pub reception: i64,
    pub reception_service: i64,
    pub briefnm_additional: Option<String>,
    pub manager_user_id: i64,
    pub google_id: Option<String>,
    pub details: UserInfoDetails,
    pub inn: String,
    pub country: Option<String>,
    pub original_client_user_id: i64,
    pub contact_id: Option<String>,
    pub role_name: String,
    pub role: i64,
    pub date_open_real: String,
    pub numdoc: String,
    pub docseries: String,
    pub regname: String,
    pub regcode: String,
    pub datedoc: String,
    pub documents: String,
    pub bornplace: String,
    pub f_kval: i64,
    pub account_block_date: String,
    pub client_date_close: Option<String>,
    pub date_client_doc_received: String,
    pub iis: String,
    pub isleadaccount: String,
    pub mkt_codes: String,
    pub object_type: String,
    pub registered_at_domain: String,
    pub email_confirm: i64,
    pub blocks_count: i64,
    #[serde(rename = "isIpoAvailable")]
    pub is_ipo_available: bool,
    #[serde(rename = "currentlyAvailableIpos")]
    pub currently_available_ipos: i64,
    #[serde(rename = "isSubscribedToNewIpos")]
    pub is_subscribed_to_new_ipos: i64,
    #[serde(rename = "isStockBonusAvailable")]
    pub is_stock_bonus_available: bool,
    #[serde(rename = "stockBonusIdKey")]
    pub stock_bonus_id_key: bool,
    #[serde(rename = "kassaNovaInvestCardAvailable")]
    pub kassa_nova_invest_card_available: bool,
    pub messages_counts: MessageCounts,
    #[serde(rename = "tariffDetails")]
    pub tariff_details: TariffDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoDetails {
    pub iis: String,
    pub push: PushSettings,
    pub comment: String,
    pub smev_sms: String,
    pub statuses: HashMap<String, String>,
    pub mkt_codes: HashMap<String, String>,
    pub telegram_id: String,
    pub telegram_bot: bool,
    #[serde(rename = "Date register")]
    pub date_register: String,
    #[serde(rename = "isLeadAccount")]
    pub is_lead_account: bool,
    #[serde(rename = "Date open real")]
    pub date_open_real: String,
    pub passport_check: String,
    pub mail_subscription: i64,
    pub ffinbank_requisites: FfinbankRequisites,
    pub initial_telegram_id: String,
    pub passport_check_date: String,
    #[serde(rename = "lastShownDateMessage")]
    pub last_shown_date_message: HashMap<String, i64>,
    #[serde(rename = "utm_campaign - to Real")]
    pub utm_campaign_to_real: Option<String>,
    #[serde(rename = "utm_campaign - Register")]
    pub utm_campaign_register: String,
    pub telegram_last_updated_at: i64,
    pub personal_anketa_last_date: String,
    pub detected_reception_service: i64,
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
    pub no_read: i64,
    pub all: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TariffDetails {
    pub id: i64,
    pub name: String,
    pub curr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserOptions {
    pub cost_open: i64,
    pub cost_last: i64,
    pub cost_low: i64,
    pub cost_high: i64,
    pub bid_last: i64,
    pub offer_last: i64,
    pub volume: i64,
    pub graphic_type: i64,
    pub graphic_format: String,
    pub period: String,
    pub time_period: String,
    pub interval: String,
    pub date_from: String,
    pub date_to: String,
    pub api_secret: String,
    pub f_transaction: i64,
    pub f_compare_index: i64,
    pub graphic_indicators: String,
    pub profile_type: i64,
    #[serde(rename = "showPortfolioBlock")]
    pub show_portfolio_block: i64,
    #[serde(rename = "pageFirstTabOpen")]
    pub page_first_tab_open: i64,
    pub cover: String,
    pub access_cost: i64,
    pub theme: String,
    #[serde(rename = "showTransactionsMode")]
    pub show_transactions_mode: String,
    #[serde(rename = "gridPortfolio")]
    pub grid_portfolio: Vec<String>,
}