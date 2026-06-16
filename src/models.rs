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
