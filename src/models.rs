use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize)]
pub struct Observation {
    pub metric_code: String,
    pub metric_name: String,
    pub priority: String,
    pub category: String,
    pub value_num: Option<f64>,
    pub value_text: Option<String>,
    pub unit: String,
    pub observed_at: String,
    pub source: String,
    pub source_url: String,
    pub attributes: Value,
}

#[derive(Clone, Debug, Serialize)]
pub struct MissingItem {
    pub collector: String,
    pub metric_code: String,
    pub metric_name: String,
    pub priority: String,
    pub reason: String,
    pub source_hint: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ExchangeUsdcBalance {
    pub exchange_name: String,
    pub symbol: String,
    pub balance: f64,
    pub balance_change_24h: Option<f64>,
    pub balance_change_percent_24h: Option<f64>,
    pub balance_change_7d: Option<f64>,
    pub balance_change_percent_7d: Option<f64>,
    pub balance_change_30d: Option<f64>,
    pub balance_change_percent_30d: Option<f64>,
    pub observed_at: String,
    pub source: String,
    pub source_url: String,
    pub attributes: Value,
}

#[derive(Clone, Debug, Serialize)]
pub struct ExchangeUsdcBalanceHistoryPoint {
    pub exchange_name: String,
    pub symbol: String,
    pub balance: f64,
    pub price_usd: Option<f64>,
    pub observed_at: String,
    pub source: String,
    pub source_url: String,
    pub attributes: Value,
}

#[derive(Clone, Debug, Serialize)]
pub struct BinanceSpotKline {
    pub symbol: String,
    pub interval: String,
    pub open_time_ms: i64,
    pub close_time_ms: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub quote_volume: f64,
    pub trade_count: i64,
    pub taker_buy_base_volume: f64,
    pub taker_buy_quote_volume: f64,
    pub observed_at: String,
    pub source: String,
    pub source_url: String,
    pub attributes: Value,
}

#[derive(Clone, Debug, Serialize)]
pub struct Filing {
    pub company: String,
    pub cik: String,
    pub accession_no: String,
    pub form: String,
    pub filing_date: String,
    pub report_date: Option<String>,
    pub primary_doc: Option<String>,
    pub description: Option<String>,
    pub source_url: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Event {
    pub source: String,
    pub event_type: String,
    pub title: String,
    pub event_date: Option<String>,
    pub url: String,
    pub summary: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SourceRun {
    pub batch_id: Option<String>,
    pub profile: Option<String>,
    pub selector: Option<String>,
    pub source: String,
    pub url: String,
    pub status: String,
    pub http_status: Option<u16>,
    pub error: Option<String>,
    pub raw_excerpt: Option<String>,
}
