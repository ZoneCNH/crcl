use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::{Context, Result, anyhow};
use chrono::{NaiveDate, Utc};
use roxmltree::Document;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::models::{Event, Filing, Observation};

macro_rules! obs {
    (
        $metric_code:expr,
        $metric_name:expr,
        $priority:expr,
        $category:expr,
        $value_num:expr,
        $value_text:expr,
        $unit:expr,
        $observed_at:expr,
        $source:expr,
        $source_url:expr,
        $attributes:expr $(,)?
    ) => {
        build_observation(ObservationSpec {
            metric_code: $metric_code,
            metric_name: $metric_name,
            priority: $priority,
            category: $category,
            value_num: $value_num,
            value_text: $value_text,
            unit: $unit,
            observed_at: $observed_at,
            source: $source,
            source_url: $source_url,
            attributes: $attributes,
        })
    };
}

struct ObservationSpec<'a> {
    metric_code: &'a str,
    metric_name: &'a str,
    priority: &'a str,
    category: &'a str,
    value_num: Option<f64>,
    value_text: Option<String>,
    unit: &'a str,
    observed_at: &'a str,
    source: &'a str,
    source_url: &'a str,
    attributes: Value,
}

fn build_observation(spec: ObservationSpec<'_>) -> Observation {
    Observation {
        metric_code: spec.metric_code.to_string(),
        metric_name: spec.metric_name.to_string(),
        priority: spec.priority.to_string(),
        category: spec.category.to_string(),
        value_num: spec.value_num,
        value_text: spec.value_text,
        unit: spec.unit.to_string(),
        observed_at: spec.observed_at.to_string(),
        source: spec.source.to_string(),
        source_url: spec.source_url.to_string(),
        attributes: spec.attributes,
    }
}

#[derive(Debug, Deserialize)]
struct DefiLlamaResponse {
    #[serde(rename = "peggedAssets")]
    pegged_assets: Vec<PeggedAsset>,
}

#[derive(Debug, Deserialize)]
struct PeggedAsset {
    id: String,
    name: String,
    symbol: String,
    #[serde(rename = "gecko_id")]
    gecko_id: Option<String>,
    #[serde(rename = "pegType")]
    peg_type: Option<String>,
    price: Option<f64>,
    circulating: Option<PeggedAmount>,
    #[serde(rename = "circulatingPrevDay")]
    circulating_prev_day: Option<PeggedAmount>,
    #[serde(rename = "circulatingPrevWeek")]
    circulating_prev_week: Option<PeggedAmount>,
    #[serde(rename = "circulatingPrevMonth")]
    circulating_prev_month: Option<PeggedAmount>,
    #[serde(rename = "chainCirculating", default)]
    chain_circulating: BTreeMap<String, ChainCirculating>,
}

#[derive(Debug, Deserialize)]
struct PeggedAmount {
    #[serde(rename = "peggedUSD")]
    pegged_usd: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct ChainCirculating {
    current: Option<PeggedAmount>,
}

pub fn parse_defillama_stablecoins(
    text: &str,
    source_url: &str,
    observed_at: &str,
) -> Result<Vec<Observation>> {
    let response: DefiLlamaResponse = serde_json::from_str(text)?;
    let total = response
        .pegged_assets
        .iter()
        .filter(|asset| asset.peg_type.as_deref() == Some("peggedUSD"))
        .filter_map(|asset| asset.circulating.as_ref()?.pegged_usd)
        .sum::<f64>();

    let usdc = response
        .pegged_assets
        .iter()
        .find(|asset| asset.symbol == "USDC")
        .context("USDC not found in DefiLlama payload")?;
    let usdt = response
        .pegged_assets
        .iter()
        .find(|asset| asset.symbol == "USDT")
        .context("USDT not found in DefiLlama payload")?;

    let usdc_current = usdc
        .circulating
        .as_ref()
        .and_then(|amount| amount.pegged_usd)
        .context("USDC current circulating supply missing")?;
    let usdt_current = usdt
        .circulating
        .as_ref()
        .and_then(|amount| amount.pegged_usd)
        .context("USDT current circulating supply missing")?;

    let mut out = vec![
        obs!(
            "P0_USDC_CIRCULATING_SUPPLY",
            "USDC circulating supply",
            "P0",
            "stablecoin_supply",
            Some(usdc_current),
            None,
            "USD",
            observed_at,
            "DefiLlama",
            source_url,
            json!({"asset_id": usdc.id, "asset_name": usdc.name, "gecko_id": usdc.gecko_id, "price": usdc.price}),
        ),
        obs!(
            "P1_STABLECOIN_TOTAL_MARKET_CAP",
            "Stablecoin total market cap",
            "P1",
            "stablecoin_supply",
            Some(total),
            None,
            "USD",
            observed_at,
            "DefiLlama",
            source_url,
            json!({"peg_type": "peggedUSD", "method": "sum_of_current_circulating"}),
        ),
        obs!(
            "P1_USDT_MARKET_CAP",
            "USDT market cap",
            "P1",
            "competition",
            Some(usdt_current),
            None,
            "USD",
            observed_at,
            "DefiLlama",
            source_url,
            json!({"asset_id": usdt.id, "asset_name": usdt.name, "gecko_id": usdt.gecko_id, "price": usdt.price}),
        ),
        obs!(
            "P1_USDC_MARKET_CAP_SHARE",
            "USDC market cap share",
            "P1",
            "competition",
            Some(usdc_current / total * 100.0),
            None,
            "percent",
            observed_at,
            "DefiLlama",
            source_url,
            json!({"numerator": "USDC circulating.peggedUSD", "denominator": "sum peggedUSD circulating"}),
        ),
        obs!(
            "P1_USDC_USDT_RATIO",
            "USDC / USDT ratio",
            "P1",
            "competition",
            Some(usdc_current / usdt_current),
            None,
            "ratio",
            observed_at,
            "DefiLlama",
            source_url,
            json!({"numerator": "USDC market cap", "denominator": "USDT market cap"}),
        ),
    ];

    if let Some(prev_day) = usdc
        .circulating_prev_day
        .as_ref()
        .and_then(|x| x.pegged_usd)
    {
        out.push(obs!(
            "P0_USDC_1D_CHANGE_PCT",
            "USDC 1D change",
            "P0",
            "stablecoin_supply",
            Some(percent_change(usdc_current, prev_day)),
            None,
            "percent",
            observed_at,
            "DefiLlama",
            source_url,
            json!({"current": usdc_current, "previous": prev_day}),
        ));
    }
    if let Some(prev_week) = usdc
        .circulating_prev_week
        .as_ref()
        .and_then(|x| x.pegged_usd)
    {
        out.push(obs!(
            "P0_USDC_7D_CHANGE_PCT",
            "USDC 7D change",
            "P0",
            "stablecoin_supply",
            Some(percent_change(usdc_current, prev_week)),
            None,
            "percent",
            observed_at,
            "DefiLlama",
            source_url,
            json!({"current": usdc_current, "previous": prev_week}),
        ));
    }
    if let Some(prev_month) = usdc
        .circulating_prev_month
        .as_ref()
        .and_then(|x| x.pegged_usd)
    {
        out.push(obs!(
            "P0_USDC_30D_CHANGE_PCT",
            "USDC 30D change",
            "P0",
            "stablecoin_supply",
            Some(percent_change(usdc_current, prev_month)),
            None,
            "percent",
            observed_at,
            "DefiLlama",
            source_url,
            json!({"current": usdc_current, "previous": prev_month}),
        ));
    }

    for chain in [
        "Ethereum",
        "Solana",
        "Base",
        "Arbitrum",
        "Optimism",
        "Polygon",
        "Avalanche",
    ] {
        if let Some(value) = usdc
            .chain_circulating
            .get(chain)
            .and_then(|chain| chain.current.as_ref())
            .and_then(|amount| amount.pegged_usd)
        {
            out.push(obs!(
                &format!("P1_USDC_CHAIN_{}", metric_suffix(chain)),
                &format!("USDC by chain: {chain}"),
                "P1",
                "chain_distribution",
                Some(value),
                None,
                "USD",
                observed_at,
                "DefiLlama",
                source_url,
                json!({"chain": chain}),
            ));
        }
    }

    Ok(out)
}

pub fn parse_defillama_protocol_usdc_deposits(
    text: &str,
    source_url: &str,
    protocol_name: &str,
    metric_code: &str,
) -> Result<Vec<Observation>> {
    let payload: Value = serde_json::from_str(text)?;
    let chain_tvls = payload
        .get("chainTvls")
        .and_then(Value::as_object)
        .context("DefiLlama protocol chainTvls missing")?;
    let mut chain_values = Vec::new();
    let mut latest_timestamp = None;

    for (chain, value) in chain_tvls {
        if chain == "borrowed"
            || chain.ends_with("-borrowed")
            || chain.ends_with("-staking")
            || chain.ends_with("-pool2")
        {
            continue;
        }

        let latest = value
            .get("tokensInUsd")
            .and_then(Value::as_array)
            .and_then(|items| items.last());
        let Some(latest) = latest else {
            continue;
        };
        let timestamp = latest.get("date").and_then(Value::as_i64);
        latest_timestamp = latest_timestamp.max(timestamp);
        let Some(tokens) = latest.get("tokens").and_then(Value::as_object) else {
            continue;
        };
        let usdc_value = tokens
            .iter()
            .filter(|(symbol, _)| is_usdc_equivalent_symbol(symbol))
            .filter_map(|(_, value)| value.as_f64())
            .sum::<f64>();

        if usdc_value > 0.0 {
            chain_values.push((chain.clone(), usdc_value));
        }
    }

    let total = chain_values.iter().map(|(_, value)| value).sum::<f64>();
    if total == 0.0 {
        return Err(anyhow!(
            "DefiLlama protocol {protocol_name} USDC-equivalent token values missing"
        ));
    }

    let observed_at = latest_timestamp
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
        .map(|dt| dt.date_naive().to_string())
        .unwrap_or_else(|| Utc::now().date_naive().to_string());
    let mut out = vec![obs!(
        metric_code,
        &format!("{protocol_name} USDC-equivalent deposits"),
        "P1",
        "defi_adoption",
        Some(total),
        None,
        "USD",
        &observed_at,
        "DefiLlama protocol TVL",
        source_url,
        json!({
            "protocol": protocol_name,
            "method": "sum latest chainTvls[*].tokensInUsd tokens for USDC-equivalent symbols",
            "included_symbols": DEFI_USDC_EQUIVALENT_SYMBOLS,
            "chain_count": chain_values.len(),
        }),
    )];

    for (chain, value) in chain_values {
        out.push(obs!(
            &format!("{metric_code}_{}", metric_suffix(&chain)),
            &format!("{protocol_name} USDC-equivalent deposits on {chain}"),
            "P1",
            "defi_adoption",
            Some(value),
            None,
            "USD",
            &observed_at,
            "DefiLlama protocol TVL",
            source_url,
            json!({
                "protocol": protocol_name,
                "chain": chain,
                "included_symbols": DEFI_USDC_EQUIVALENT_SYMBOLS,
            }),
        ));
    }

    Ok(out)
}

const DEFI_USDC_EQUIVALENT_SYMBOLS: &[&str] = &["USDC", "USDC.E", "USDBC", "AVALANCHEUSDC"];

fn is_usdc_equivalent_symbol(symbol: &str) -> bool {
    DEFI_USDC_EQUIVALENT_SYMBOLS.contains(&symbol)
}

pub fn parse_rwa_treasuries(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let page_props = rwa_page_props(text)?;
    let results = page_props
        .get("listQueryResponse")
        .and_then(|value| value.get("results"))
        .and_then(Value::as_array)
        .context("RWA.xyz treasuries results missing")?;
    let mut seen_assets = HashSet::new();
    let mut total_value = 0.0;
    let mut asset_count = 0;
    let mut buidl_value = None;
    let mut latest_updated_at: Option<String> = None;
    let mut top_assets = Vec::new();

    for asset in results {
        if asset
            .get("hidden")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            continue;
        }
        if asset.get("asset_class_name").and_then(Value::as_str) != Some("US Treasury Debt") {
            continue;
        }

        let asset_id = asset_identity(asset).context("RWA.xyz treasury asset id missing")?;
        if !seen_assets.insert(asset_id) {
            continue;
        }
        let Some(value) = metric_value(asset, "total_asset_value_dollar") else {
            continue;
        };
        if value <= 0.0 {
            continue;
        }

        asset_count += 1;
        total_value += value;

        let ticker = asset
            .get("ticker")
            .and_then(Value::as_str)
            .unwrap_or("-")
            .to_string();
        if ticker == "BUIDL" {
            buidl_value = Some(value);
        }

        if let Some(updated_at) = asset.get("_updated_at").and_then(Value::as_str)
            && latest_updated_at
                .as_deref()
                .is_none_or(|current| updated_at > current)
        {
            latest_updated_at = Some(updated_at.to_string());
        }

        top_assets.push((
            asset
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("-")
                .to_string(),
            ticker,
            value,
            asset
                .get("manager")
                .and_then(|manager| manager.get("name"))
                .and_then(Value::as_str)
                .unwrap_or("-")
                .to_string(),
        ));
    }

    if total_value == 0.0 {
        return Err(anyhow!("RWA.xyz tokenized treasury AUM missing"));
    }

    top_assets.sort_by(|left, right| right.2.total_cmp(&left.2));
    let observed_at = latest_updated_at
        .as_deref()
        .map(date_part)
        .unwrap_or_else(|| Utc::now().date_naive().to_string());
    let top_assets_json = top_assets
        .iter()
        .take(10)
        .map(|(name, ticker, value, manager)| {
            json!({
                "name": name,
                "ticker": ticker,
                "manager": manager,
                "total_asset_value_dollar": value
            })
        })
        .collect::<Vec<_>>();

    let mut out = vec![obs!(
        "P1_TOKENIZED_TREASURY_AUM",
        "Tokenized U.S. Treasury debt AUM",
        "P1",
        "rwa_treasuries",
        Some(total_value),
        None,
        "USD",
        &observed_at,
        "RWA.xyz public treasuries page",
        source_url,
        json!({
            "method": "sum visible US Treasury Debt assets by total_asset_value_dollar.val",
            "asset_count": asset_count,
            "latest_updated_at": latest_updated_at,
            "top_assets": top_assets_json,
        }),
    )];

    if let Some(value) = buidl_value {
        out.push(obs!(
            "P1_TOKENIZED_TREASURY_BUIDL_AUM",
            "BlackRock BUIDL AUM",
            "P1",
            "rwa_treasuries",
            Some(value),
            None,
            "USD",
            &observed_at,
            "RWA.xyz public treasuries page",
            source_url,
            json!({"ticker": "BUIDL", "latest_updated_at": latest_updated_at}),
        ));
    }

    Ok(out)
}

fn rwa_page_props(text: &str) -> Result<Value> {
    let trimmed = text.trim_start();
    if trimmed.starts_with('{') {
        let value: Value = serde_json::from_str(trimmed)?;
        return page_props_from_value(value);
    }

    let marker = "<script id=\"__NEXT_DATA__\"";
    let start = text
        .find(marker)
        .context("RWA.xyz __NEXT_DATA__ script missing")?;
    let json_start = text[start..]
        .find('>')
        .map(|offset| start + offset + 1)
        .context("RWA.xyz __NEXT_DATA__ script open tag incomplete")?;
    let json_end = text[json_start..]
        .find("</script>")
        .map(|offset| json_start + offset)
        .context("RWA.xyz __NEXT_DATA__ script close tag missing")?;
    let value: Value = serde_json::from_str(&text[json_start..json_end])?;
    page_props_from_value(value)
}

fn page_props_from_value(value: Value) -> Result<Value> {
    if let Some(page_props) = value.get("pageProps") {
        return Ok(page_props.clone());
    }
    value
        .get("props")
        .and_then(|props| props.get("pageProps"))
        .cloned()
        .context("RWA.xyz pageProps missing")
}

fn asset_identity(asset: &Value) -> Option<String> {
    asset
        .get("id")
        .and_then(|value| {
            value
                .as_i64()
                .map(|id| id.to_string())
                .or_else(|| value.as_str().map(ToString::to_string))
        })
        .or_else(|| {
            asset
                .get("slug")
                .and_then(Value::as_str)
                .map(ToString::to_string)
        })
}

fn metric_value(asset: &Value, metric_name: &str) -> Option<f64> {
    asset
        .get(metric_name)
        .and_then(|metric| metric.get("val"))
        .and_then(Value::as_f64)
}

fn date_part(value: &str) -> String {
    value.split('T').next().unwrap_or(value).to_string()
}

fn percent_change(current: f64, previous: f64) -> f64 {
    if previous == 0.0 {
        0.0
    } else {
        (current - previous) / previous * 100.0
    }
}

fn metric_suffix(input: &str) -> String {
    input
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>()
        .to_ascii_uppercase()
}

#[derive(Debug, Deserialize)]
struct CoinGeckoAsset {
    usd: Option<f64>,
    usd_market_cap: Option<f64>,
    usd_24h_vol: Option<f64>,
    usd_24h_change: Option<f64>,
}

pub fn parse_coingecko_simple_price(
    text: &str,
    source_url: &str,
    observed_at: &str,
) -> Result<Vec<Observation>> {
    let data: HashMap<String, CoinGeckoAsset> = serde_json::from_str(text)?;
    let assets = [
        (
            "usd-coin",
            "P0_USDC_USD_PRICE",
            "USDC/USD price",
            "P0",
            "peg_check",
        ),
        (
            "tether",
            "P0_USDT_USD_PRICE",
            "USDT/USD price",
            "P0",
            "peg_check",
        ),
        ("bitcoin", "P0_BTC_PRICE", "BTC price", "P0", "crypto_beta"),
        ("ethereum", "P0_ETH_PRICE", "ETH price", "P0", "crypto_beta"),
        (
            "paypal-usd",
            "P1_COMPETITOR_PYUSD_MARKET_CAP",
            "PYUSD market cap",
            "P1",
            "competition",
        ),
        (
            "first-digital-usd",
            "P1_COMPETITOR_FDUSD_MARKET_CAP",
            "FDUSD market cap",
            "P1",
            "competition",
        ),
        (
            "ethena-usde",
            "P1_COMPETITOR_USDE_MARKET_CAP",
            "USDe market cap",
            "P1",
            "competition",
        ),
        (
            "ondo-us-dollar-yield",
            "P1_COMPETITOR_USDY_MARKET_CAP",
            "Ondo USDY market cap",
            "P1",
            "competition",
        ),
    ];
    let mut out = Vec::new();

    for (id, metric_code, metric_name, priority, category) in assets {
        let Some(asset) = data.get(id) else {
            continue;
        };
        let value = if metric_code.ends_with("MARKET_CAP") {
            asset.usd_market_cap
        } else {
            asset.usd
        };
        out.push(obs!(
            metric_code,
            metric_name,
            priority,
            category,
            value,
            None,
            "USD",
            observed_at,
            "CoinGecko",
            source_url,
            json!({
                "coingecko_id": id,
                "usd_price": asset.usd,
                "usd_market_cap": asset.usd_market_cap,
                "usd_24h_vol": asset.usd_24h_vol,
                "usd_24h_change": asset.usd_24h_change
            }),
        ));
    }

    Ok(out)
}

pub fn parse_treasury_yield_curve(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let doc = Document::parse(text)?;
    let mut latest: Option<TreasuryRow> = None;

    for entry in doc.descendants().filter(|node| node.has_tag_name("entry")) {
        let row = TreasuryRow {
            date: child_text(entry, "NEW_DATE"),
            three_month: child_text(entry, "BC_3MONTH").and_then(|x| x.parse().ok()),
            one_year: child_text(entry, "BC_1YEAR").and_then(|x| x.parse().ok()),
            five_year: child_text(entry, "BC_5YEAR").and_then(|x| x.parse().ok()),
        };
        if row.date.is_some() {
            latest = match latest {
                Some(current) if current.date >= row.date => Some(current),
                _ => Some(row),
            };
        }
    }

    let latest = latest.context("no Treasury rows found")?;
    let observed_at = latest
        .date
        .clone()
        .unwrap_or_else(|| Utc::now().date_naive().to_string());
    let mut out = Vec::new();
    for (code, name, value, maturity) in [
        (
            "P0_TREASURY_3M_YIELD",
            "3M T-Bill yield",
            latest.three_month,
            "3M",
        ),
        (
            "P0_TREASURY_1Y_YIELD",
            "1Y Treasury yield",
            latest.one_year,
            "1Y",
        ),
        (
            "P0_TREASURY_5Y_YIELD",
            "5Y Treasury yield",
            latest.five_year,
            "5Y",
        ),
    ] {
        if let Some(value) = value {
            out.push(obs!(
                code,
                name,
                "P0",
                "rates",
                Some(value),
                None,
                "percent",
                &observed_at,
                "U.S. Treasury",
                source_url,
                json!({"maturity": maturity}),
            ));
        }
    }
    Ok(out)
}

#[derive(Clone, Debug)]
struct TreasuryRow {
    date: Option<String>,
    three_month: Option<f64>,
    one_year: Option<f64>,
    five_year: Option<f64>,
}

fn child_text(node: roxmltree::Node, tag: &str) -> Option<String> {
    node.descendants()
        .find(|child| child.tag_name().name() == tag)
        .and_then(|child| child.text())
        .map(ToString::to_string)
}

#[derive(Debug, Deserialize)]
struct NyFedResponse {
    #[serde(rename = "refRates")]
    ref_rates: Vec<NyFedRate>,
}

#[derive(Debug, Deserialize)]
struct NyFedRate {
    #[serde(rename = "effectiveDate")]
    effective_date: String,
    #[serde(rename = "percentRate")]
    percent_rate: f64,
    #[serde(rename = "volumeInBillions")]
    volume_in_billions: Option<f64>,
}

pub fn parse_nyfed_sofr(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let response: NyFedResponse = serde_json::from_str(text)?;
    let rate = response
        .ref_rates
        .first()
        .context("SOFR refRates is empty")?;
    Ok(vec![obs!(
        "P0_SOFR",
        "SOFR",
        "P0",
        "rates",
        Some(rate.percent_rate),
        None,
        "percent",
        &rate.effective_date,
        "NY Fed",
        source_url,
        json!({"volume_in_billions": rate.volume_in_billions}),
    )])
}

#[derive(Debug, Deserialize)]
struct CoinMetricsResponse {
    data: Vec<CoinMetricsRow>,
}

#[derive(Debug, Deserialize)]
struct CoinMetricsRow {
    asset: String,
    time: String,
    #[serde(rename = "AdrActCnt")]
    adr_act_cnt: Option<String>,
    #[serde(rename = "TxCnt")]
    tx_cnt: Option<String>,
    #[serde(rename = "TxTfrCnt")]
    tx_tfr_cnt: Option<String>,
}

pub fn parse_coinmetrics_usdc_activity(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let response: CoinMetricsResponse = serde_json::from_str(text)?;
    let row = response.data.first().context("CoinMetrics data is empty")?;
    let observed_at = row.time.split('T').next().unwrap_or(&row.time);
    let mut out = Vec::new();
    for (code, name, raw, unit) in [
        (
            "P1_USDC_ACTIVE_ADDRESSES",
            "USDC active addresses",
            &row.adr_act_cnt,
            "count",
        ),
        (
            "P1_USDC_TRANSACTION_COUNT",
            "USDC transaction count",
            &row.tx_cnt,
            "count",
        ),
        (
            "P1_USDC_TRANSFER_COUNT",
            "USDC transfer count",
            &row.tx_tfr_cnt,
            "count",
        ),
    ] {
        if let Some(value) = raw.as_ref().and_then(|raw| raw.parse::<f64>().ok()) {
            out.push(obs!(
                code,
                name,
                "P1",
                "chain_activity",
                Some(value),
                None,
                unit,
                observed_at,
                "CoinMetrics Community API",
                source_url,
                json!({"asset": row.asset}),
            ));
        }
    }
    Ok(out)
}

#[derive(Debug, Deserialize)]
struct AlliumAdjustedTransferRow {
    day: String,
    usd_amount: f64,
    txn_count: f64,
    usd_amount_30d: f64,
    txn_count_30d: f64,
}

#[derive(Debug, Deserialize)]
struct CoinGlassExchangeBalanceResponse {
    #[serde(default)]
    code: Option<Value>,
    #[serde(default)]
    msg: Option<String>,
    #[serde(default)]
    data: Option<Value>,
}

pub fn parse_visa_allium_usdc_adjusted_transfer_volume(
    text: &str,
    source_url: &str,
) -> Result<Vec<Observation>> {
    let rows: Vec<AlliumAdjustedTransferRow> = serde_json::from_str(text)?;
    let row = rows.first().context("Allium adjusted USDC row is empty")?;
    let observed_at = row.day.split('T').next().unwrap_or(&row.day);
    NaiveDate::parse_from_str(observed_at, "%Y-%m-%d")
        .with_context(|| format!("invalid Allium day: {}", row.day))?;

    Ok(vec![obs!(
        "P1_USDC_ADJUSTED_TRANSFER_VOLUME",
        "USDC adjusted transfer volume, latest complete UTC day",
        "P1",
        "chain_activity",
        Some(row.usd_amount),
        None,
        "USD",
        observed_at,
        "Visa Onchain Analytics / Allium",
        source_url,
        json!({
            "base_asset": "USDC",
            "tag": "Adjusted",
            "txn_count": row.txn_count,
            "usd_amount_30d": row.usd_amount_30d,
            "txn_count_30d": row.txn_count_30d,
            "window": "latest complete UTC day",
            "query_id": "Lmrs1Uvwa0RXc4J3a0vC",
            "share_id": "JKyWRaJi",
            "method": "sum Allium adjusted stablecoin transfer usd_amount for USDC across chains and adjusted categories",
        }),
    )])
}

pub fn parse_coinglass_exchange_balance(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let response: CoinGlassExchangeBalanceResponse = serde_json::from_str(text)?;
    if response.data.is_none()
        && let Some(code) = response.code.as_ref()
    {
        let code_text = code
            .as_str()
            .map(str::to_string)
            .unwrap_or_else(|| code.to_string());
        let message = response
            .msg
            .as_deref()
            .filter(|msg| !msg.trim().is_empty())
            .unwrap_or("no message");
        return Err(anyhow!(
            "CoinGlass API returned code {code_text}: {message}; set COINGLASS_API_KEY for the CG-API-KEY header"
        ));
    }
    let rows = response
        .data
        .as_ref()
        .context("CoinGlass exchange balance response missing data")?
        .as_array()
        .context("CoinGlass exchange balance data is not an array")?;

    let mut total_balance = 0.0;
    let mut components = Vec::new();
    for row in rows {
        let Some(balance) = numeric_field(row, &["balance", "totalBalance", "value", "amount"])
        else {
            continue;
        };
        total_balance += balance;
        components.push(json!({
            "exchange": string_field(row, &["exchange", "exchangeName", "name"]),
            "balance": balance,
            "change_percent_1d": numeric_field(row, &["changePercent24h", "changePercent1d", "change1d", "change24h"]),
            "change_percent_7d": numeric_field(row, &["changePercent7d", "change7d"]),
            "change_percent_30d": numeric_field(row, &["changePercent30d", "change30d"]),
        }));
    }

    if total_balance <= 0.0 {
        return Err(anyhow!(
            "CoinGlass USDC exchange balance rows have no numeric balance"
        ));
    }

    let observed_at = Utc::now().date_naive().to_string();
    Ok(vec![obs!(
        "P1_EXCHANGE_USDC_BALANCES",
        "USDC exchange balance, all exchanges",
        "P1",
        "exchange_balance",
        Some(total_balance),
        None,
        "USDC",
        &observed_at,
        "CoinGlass Exchange Balance List",
        source_url,
        json!({
            "asset": "USDC",
            "scope": "all exchanges",
            "method": "sum CoinGlass exchange-level USDC balances",
            "component_count": components.len(),
            "components": components,
        }),
    )])
}

pub fn parse_circle_platform_metrics(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let plain = html_plain_text(text);
    let mut observations = Vec::new();

    if let Some(cpn_idx) = plain.find("Continued CPN Expansion") {
        let section = plain[cpn_idx..].chars().take(900).collect::<String>();
        let observed_at = first_long_form_date_after(&section, "as of")
            .unwrap_or_else(|| Utc::now().date_naive().to_string());
        let annualized_tpv = money_amount_after(&section, "annualized transaction volume")
            .or_else(|| money_amount_after(&section, "CPN network"))
            .context("Circle CPN annualized TPV missing")?;
        observations.push(obs!(
            "P1_CPN_ANNUALIZED_TPV",
            "Circle Payments Network annualized TPV",
            "P1",
            "platform_metrics",
            Some(annualized_tpv),
            None,
            "USD/year",
            &observed_at,
            "Circle press release",
            source_url,
            json!({
                "business_line": "Circle Payments Network",
                "window": "annualized trailing 30 day activity",
                "method": "parse Circle press release CPN expansion paragraph",
            }),
        ));
    }

    if let Some(arc_idx) = arc_metrics_section_start(&plain) {
        let section = plain[arc_idx..].chars().take(1200).collect::<String>();
        let observed_at = first_long_form_date_after(&section, "as of")
            .or_else(|| first_long_form_date_after(&plain, "NEW YORK"))
            .unwrap_or_else(|| Utc::now().date_naive().to_string());
        if let Some(status) = arc_status_from_text(&section) {
            observations.push(obs!(
                "P1_ARC_MAINNET_STATUS",
                "Arc mainnet status",
                "P1",
                "platform_metrics",
                None,
                Some(status.clone()),
                "status",
                &observed_at,
                "Circle press release",
                source_url,
                json!({
                    "business_line": "Arc",
                    "method": "parse Circle press release Arc status language",
                    "status_text": status,
                }),
            ));
        }

        if let Some(testnet_daily_transactions) =
            number_after_marker(&section, "daily average transaction volumes")
                .or_else(|| number_before_marker(&section, "daily average transaction volumes"))
        {
            observations.push(obs!(
                "P1_ARC_TESTNET_DAILY_AVG_TRANSACTIONS",
                "Arc testnet daily average transactions",
                "P1",
                "platform_metrics",
                Some(testnet_daily_transactions),
                None,
                "transactions/day",
                &observed_at,
                "Circle press release",
                source_url,
                json!({
                    "business_line": "Arc",
                    "source_caveat": "testnet usage, not revenue",
                    "method": "parse Circle press release Arc testnet activity paragraph",
                }),
            ));
        }

        if let Some(total_transactions) =
            number_after_marker(&section, "Total transactions have exceeded")
        {
            observations.push(obs!(
                "P1_ARC_TESTNET_TOTAL_TRANSACTIONS",
                "Arc testnet total transactions",
                "P1",
                "platform_metrics",
                Some(total_transactions),
                None,
                "transactions",
                &observed_at,
                "Circle press release",
                source_url,
                json!({
                    "business_line": "Arc",
                    "source_caveat": "testnet usage, not revenue",
                    "method": "parse Circle press release Arc cumulative testnet transaction sentence",
                }),
            ));
        }
    }

    if observations.is_empty() {
        return Err(anyhow!("Circle platform metrics missing from source page"));
    }

    Ok(observations)
}

pub fn parse_arc_public_status(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let plain = html_plain_text(text);
    let observed_at = Utc::now().date_naive().to_string();
    let mut observations = Vec::new();

    if plain.contains("Live on public testnet") || plain.contains("public testnet") {
        observations.push(obs!(
            "P1_ARC_PUBLIC_NETWORK_STATUS",
            "Arc public network status",
            "P1",
            "platform_metrics",
            None,
            Some("public testnet".to_string()),
            "status",
            &observed_at,
            "Arc website",
            source_url,
            json!({"method": "parse Arc homepage public network status"}),
        ));
    }

    for (code, name, label, unit) in [
        (
            "P1_ARC_WEEKLY_TRANSACTION_COST",
            "Arc average weekly transaction cost",
            "Avg. Weekly transaction cost",
            "USD/transaction",
        ),
        (
            "P1_ARC_WEEKLY_CONTRACTS_DEPLOYED",
            "Arc weekly contracts deployed",
            "Weekly contracts deployed",
            "contracts/week",
        ),
        (
            "P1_ARC_WEEKLY_ACCOUNTS_CREATED",
            "Arc weekly accounts created",
            "Weekly accounts created",
            "accounts/week",
        ),
        (
            "P1_ARC_WEEKLY_TRANSACTIONS",
            "Arc weekly transactions",
            "Weekly transactions",
            "transactions/week",
        ),
    ] {
        if let Some(value) = number_after_marker(&plain, label) {
            observations.push(obs!(
                code,
                name,
                "P1",
                "platform_metrics",
                Some(value),
                None,
                unit,
                &observed_at,
                "Arc website",
                source_url,
                json!({
                    "source_caveat": "public testnet usage, not mainnet revenue",
                    "method": "parse Arc homepage live public testnet metric",
                    "source_label": label,
                }),
            ));
        }
    }

    if observations.is_empty() {
        return Err(anyhow!(
            "Arc public status metrics missing from source page"
        ));
    }

    Ok(observations)
}

pub fn parse_marketbeat_institutional_ownership(
    text: &str,
    source_url: &str,
) -> Result<Vec<Observation>> {
    let html = Html::parse_document(text);
    let observed_at =
        marketbeat_revised_date(&html).unwrap_or_else(|| Utc::now().date_naive().to_string());
    let mut observations = Vec::new();

    let stat_selector = Selector::parse(".stat-summary-wrapper").unwrap();
    for node in html.select(&stat_selector) {
        let text = normalize_text(&node.text().collect::<Vec<_>>().join(" "));
        if text.contains("Number of Institutional Buyers") {
            if let Some(value) = last_number_in_text(&text) {
                observations.push(institutional_obs(
                    "P2_CRCL_INSTITUTIONAL_BUYERS_12M",
                    "CRCL institutional buyers, last 12 months",
                    value,
                    "count",
                    &observed_at,
                    source_url,
                    "MarketBeat stat summary",
                ));
            }
        } else if text.contains("Total Institutional Inflows") {
            if let Some(value) = money_amount_after(&text, "Inflows") {
                observations.push(institutional_obs(
                    "P2_CRCL_INSTITUTIONAL_INFLOW_12M",
                    "CRCL institutional inflows, last 12 months",
                    value,
                    "USD",
                    &observed_at,
                    source_url,
                    "MarketBeat stat summary",
                ));
            }
        } else if text.contains("Number of Institutional Sellers") {
            if let Some(value) = last_number_in_text(&text) {
                observations.push(institutional_obs(
                    "P2_CRCL_INSTITUTIONAL_SELLERS_12M",
                    "CRCL institutional sellers, last 12 months",
                    value,
                    "count",
                    &observed_at,
                    source_url,
                    "MarketBeat stat summary",
                ));
            }
        } else if text.contains("Total Institutional Outflows")
            && let Some(value) = money_amount_after(&text, "Outflows")
        {
            observations.push(institutional_obs(
                "P2_CRCL_INSTITUTIONAL_OUTFLOW_12M",
                "CRCL institutional outflows, last 12 months",
                value,
                "USD",
                &observed_at,
                source_url,
                "MarketBeat stat summary",
            ));
        }
    }

    for (selector, code, name, unit, marker) in [
        (
            "#answer1",
            "P2_CRCL_INSTITUTIONAL_HOLDER_COUNT_24M",
            "CRCL institutional holder count, previous 24 months",
            "count",
            "previous two years",
        ),
        (
            "#answer3",
            "P2_CRCL_INSTITUTIONAL_SHARES_BOUGHT_24M",
            "CRCL institutional shares bought, previous 24 months",
            "shares",
            "bought a total of",
        ),
        (
            "#answer5",
            "P2_CRCL_INSTITUTIONAL_SHARES_SOLD_24M",
            "CRCL institutional shares sold, previous 24 months",
            "shares",
            "sold a total of",
        ),
    ] {
        let selector = Selector::parse(selector).unwrap();
        if let Some(node) = html.select(&selector).next() {
            let text = normalize_text(&node.text().collect::<Vec<_>>().join(" "));
            if let Some(value) = number_after_marker(&text, marker) {
                observations.push(institutional_obs(
                    code,
                    name,
                    value,
                    unit,
                    &observed_at,
                    source_url,
                    "MarketBeat FAQ summary",
                ));
            }
        }
    }

    if observations.is_empty() {
        return Err(anyhow!(
            "MarketBeat CRCL institutional ownership metrics missing"
        ));
    }

    Ok(observations)
}

#[derive(Debug, Deserialize)]
struct CurvePoolsResponse {
    success: bool,
    data: CurvePoolsData,
}

#[derive(Debug, Deserialize)]
struct CurvePoolsData {
    #[serde(rename = "poolData")]
    pool_data: Vec<CurvePool>,
}

#[derive(Debug, Deserialize)]
struct CurvePool {
    address: String,
    name: Option<String>,
    symbol: Option<String>,
    #[serde(default)]
    coins: Vec<CurveCoin>,
    #[serde(rename = "usdTotal")]
    usd_total: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct CurveCoin {
    symbol: Option<String>,
    #[serde(rename = "poolBalance")]
    pool_balance: Option<String>,
    decimals: Value,
    #[serde(rename = "usdPrice")]
    usd_price: Option<f64>,
}

pub fn parse_curve_3pool(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let response: CurvePoolsResponse = serde_json::from_str(text)?;
    if !response.success {
        return Err(anyhow!("Curve pools response success=false"));
    }
    let pool = response
        .data
        .pool_data
        .iter()
        .find(|pool| {
            pool.address
                .eq_ignore_ascii_case("0xbEbc44782C7dB0a1A60Cb6fe97d0b483032FF1C7")
        })
        .context("Curve 3pool not found")?;
    let observed_at = Utc::now().date_naive().to_string();
    let mut coin_values = Vec::new();

    for coin in &pool.coins {
        let symbol = coin.symbol.as_deref().unwrap_or("-");
        let decimals = value_to_u32(&coin.decimals)
            .with_context(|| format!("Curve 3pool {symbol} decimals missing"))?;
        let raw_balance = coin
            .pool_balance
            .as_deref()
            .with_context(|| format!("Curve 3pool {symbol} balance missing"))?;
        let price = coin
            .usd_price
            .with_context(|| format!("Curve 3pool {symbol} USD price missing"))?;
        let balance = scaled_token_balance(raw_balance, decimals)?;
        coin_values.push((symbol.to_string(), balance, balance * price, price));
    }

    let total_usd = coin_values
        .iter()
        .map(|(_, _, value_usd, _)| value_usd)
        .sum::<f64>();
    let usdc = coin_values
        .iter()
        .find(|(symbol, _, _, _)| symbol == "USDC")
        .context("Curve 3pool USDC coin missing")?;
    let usdc_ratio = if total_usd == 0.0 {
        0.0
    } else {
        usdc.2 / total_usd * 100.0
    };

    let mut out = vec![
        obs!(
            "P0_CURVE_3POOL_USDC_RATIO",
            "Curve 3pool USDC ratio",
            "P0",
            "peg_liquidity",
            Some(usdc_ratio),
            None,
            "percent",
            &observed_at,
            "Curve API",
            source_url,
            json!({"pool": pool.name, "symbol": pool.symbol, "pool_address": pool.address, "method": "USDC USD value / sum coin USD values"}),
        ),
        obs!(
            "P0_CURVE_3POOL_USDC_BALANCE",
            "Curve 3pool USDC balance",
            "P0",
            "peg_liquidity",
            Some(usdc.2),
            None,
            "USD",
            &observed_at,
            "Curve API",
            source_url,
            json!({"token_balance": usdc.1, "usd_price": usdc.3, "pool_address": pool.address}),
        ),
        obs!(
            "P0_CURVE_3POOL_TOTAL_LIQUIDITY",
            "Curve 3pool total liquidity",
            "P0",
            "peg_liquidity",
            Some(pool.usd_total.unwrap_or(total_usd)),
            None,
            "USD",
            &observed_at,
            "Curve API",
            source_url,
            json!({"computed_coin_usd_total": total_usd, "pool_usd_total": pool.usd_total, "pool_address": pool.address}),
        ),
    ];

    for (symbol, balance, value_usd, price) in coin_values {
        if symbol == "USDC" {
            continue;
        }
        out.push(obs!(
            &format!("P0_CURVE_3POOL_{}_BALANCE", metric_suffix(&symbol)),
            &format!("Curve 3pool {symbol} balance"),
            "P0",
            "peg_liquidity",
            Some(value_usd),
            None,
            "USD",
            &observed_at,
            "Curve API",
            source_url,
            json!({"token_balance": balance, "usd_price": price, "pool_address": pool.address}),
        ));
    }

    Ok(out)
}

fn value_to_u32(value: &Value) -> Option<u32> {
    value
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .or_else(|| value.as_str()?.parse::<u32>().ok())
}

fn scaled_token_balance(raw_balance: &str, decimals: u32) -> Result<f64> {
    let raw = raw_balance.parse::<f64>()?;
    Ok(raw / 10_f64.powi(i32::try_from(decimals)?))
}

#[derive(Debug, Deserialize)]
struct StatusPageResponse {
    page: StatusPagePage,
    status: StatusPageStatus,
}

#[derive(Debug, Deserialize)]
struct StatusPagePage {
    name: String,
    url: String,
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StatusPageStatus {
    indicator: String,
    description: String,
}

pub fn parse_statuspage(
    text: &str,
    source_url: &str,
    metric_code: &str,
    metric_name: &str,
) -> Result<Vec<Observation>> {
    let response: StatusPageResponse = serde_json::from_str(text)?;
    let observed_at = response.page.updated_at.as_deref().unwrap_or(source_url);
    Ok(vec![obs!(
        metric_code,
        metric_name,
        "P0",
        "technical_status",
        None,
        Some(response.status.description.clone()),
        "status",
        observed_at,
        &response.page.name,
        source_url,
        json!({
            "indicator": response.status.indicator,
            "page_url": response.page.url
        }),
    )])
}

pub fn parse_ethereum_latest_block(
    text: &str,
    source_url: &str,
    now: chrono::DateTime<Utc>,
) -> Result<Vec<Observation>> {
    let payload: Value = serde_json::from_str(text)?;
    let result = payload
        .get("result")
        .and_then(Value::as_object)
        .context("Ethereum JSON-RPC result missing")?;
    let block_number = result
        .get("number")
        .and_then(Value::as_str)
        .and_then(hex_to_u64)
        .context("Ethereum latest block number missing")?;
    let block_timestamp = result
        .get("timestamp")
        .and_then(Value::as_str)
        .and_then(hex_to_u64)
        .context("Ethereum latest block timestamp missing")?;
    let block_time = chrono::DateTime::from_timestamp(i64::try_from(block_timestamp)?, 0)
        .context("Ethereum latest block timestamp invalid")?;
    let block_age_seconds = now.signed_duration_since(block_time).num_seconds().max(0);
    let status = if block_age_seconds <= 300 {
        "Operational"
    } else {
        "Degraded: latest block is stale"
    };

    Ok(vec![obs!(
        "P0_CHAIN_STATUS_ETHEREUM",
        "Ethereum chain status",
        "P0",
        "technical_status",
        None,
        Some(status.to_string()),
        "status",
        &now.to_rfc3339(),
        "Ethereum public JSON-RPC",
        source_url,
        json!({
            "method": "eth_getBlockByNumber latest block freshness",
            "source_note": "Not an official Ethereum Foundation Statuspage; public RPC latest-block health proxy.",
            "block_number": block_number,
            "block_timestamp": block_time.to_rfc3339(),
            "block_age_seconds": block_age_seconds,
            "freshness_threshold_seconds": 300,
            "block_hash": result.get("hash").and_then(Value::as_str),
        }),
    )])
}

fn hex_to_u64(value: &str) -> Option<u64> {
    u64::from_str_radix(value.trim_start_matches("0x"), 16).ok()
}

#[derive(Debug, Deserialize)]
struct YahooChartResponse {
    chart: YahooChart,
}

#[derive(Debug, Deserialize)]
struct YahooChart {
    result: Option<Vec<YahooResult>>,
    error: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct YahooResult {
    timestamp: Option<Vec<i64>>,
    meta: YahooMeta,
    indicators: YahooIndicators,
}

#[derive(Debug, Deserialize)]
struct YahooMeta {
    symbol: String,
    regular_market_time: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct YahooIndicators {
    quote: Vec<YahooQuote>,
}

#[derive(Debug, Deserialize)]
struct YahooQuote {
    close: Option<Vec<Option<f64>>>,
    volume: Option<Vec<Option<f64>>>,
}

pub fn parse_yahoo_chart(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let response: YahooChartResponse = serde_json::from_str(text)?;
    if let Some(error) = response.chart.error {
        return Err(anyhow!("Yahoo chart error: {error}"));
    }
    let result = response
        .chart
        .result
        .and_then(|mut results| results.pop())
        .context("Yahoo chart result is empty")?;
    let quote = result
        .indicators
        .quote
        .first()
        .context("Yahoo chart quote is empty")?;
    let closes = quote.close.as_ref().context("Yahoo close array missing")?;
    let volumes = quote
        .volume
        .as_ref()
        .context("Yahoo volume array missing")?;

    let mut latest_idx = None;
    for (idx, close) in closes.iter().enumerate().rev() {
        if close.is_some() {
            latest_idx = Some(idx);
            break;
        }
    }
    let idx = latest_idx.context("Yahoo close contains no values")?;
    let observed_at = result
        .timestamp
        .as_ref()
        .and_then(|timestamps| timestamps.get(idx).copied())
        .or(result.meta.regular_market_time)
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
        .map(|dt| dt.date_naive().to_string())
        .unwrap_or_else(|| Utc::now().date_naive().to_string());

    Ok(vec![
        obs!(
            "P0_CRCL_PRICE",
            "CRCL price",
            "P0",
            "equity_market",
            closes[idx],
            None,
            "USD",
            &observed_at,
            "Yahoo Finance",
            source_url,
            json!({"symbol": result.meta.symbol}),
        ),
        obs!(
            "P0_CRCL_VOLUME",
            "CRCL volume",
            "P0",
            "equity_market",
            volumes.get(idx).copied().flatten(),
            None,
            "shares",
            &observed_at,
            "Yahoo Finance",
            source_url,
            json!({"symbol": result.meta.symbol}),
        ),
    ])
}

#[derive(Debug, Deserialize)]
struct FinraShortInterestRow {
    #[serde(rename = "symbolCode")]
    symbol_code: String,
    #[serde(rename = "issueName")]
    issue_name: String,
    #[serde(rename = "settlementDate")]
    settlement_date: String,
    #[serde(rename = "marketClassCode")]
    market_class_code: Option<String>,
    #[serde(rename = "issuerServicesGroupExchangeCode")]
    issuer_services_group_exchange_code: Option<String>,
    #[serde(rename = "currentShortPositionQuantity")]
    current_short_position_quantity: f64,
    #[serde(rename = "previousShortPositionQuantity")]
    previous_short_position_quantity: Option<f64>,
    #[serde(rename = "averageDailyVolumeQuantity")]
    average_daily_volume_quantity: Option<f64>,
    #[serde(rename = "daysToCoverQuantity")]
    days_to_cover_quantity: Option<f64>,
    #[serde(rename = "changePercent")]
    change_percent: Option<f64>,
    #[serde(rename = "changePreviousNumber")]
    change_previous_number: Option<f64>,
    #[serde(rename = "revisionFlag")]
    revision_flag: Option<String>,
}

pub fn parse_finra_short_interest(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let rows: Vec<FinraShortInterestRow> = serde_json::from_str(text)?;
    let row = rows
        .iter()
        .filter(|row| row.symbol_code == "CRCL")
        .max_by(|left, right| left.settlement_date.cmp(&right.settlement_date))
        .context("FINRA CRCL short interest row missing")?;

    let mut observations = vec![obs!(
        "P2_FINRA_SHORT_INTEREST",
        "CRCL short interest",
        "P2",
        "equity_market",
        Some(row.current_short_position_quantity),
        None,
        "shares",
        &row.settlement_date,
        "FINRA consolidated short interest",
        source_url,
        json!({
            "symbol": row.symbol_code,
            "issue_name": row.issue_name,
            "market_class_code": row.market_class_code,
            "issuer_services_group_exchange_code": row.issuer_services_group_exchange_code,
            "previous_short_position_quantity": row.previous_short_position_quantity,
            "average_daily_volume_quantity": row.average_daily_volume_quantity,
            "change_previous_number": row.change_previous_number,
            "revision_flag": row.revision_flag,
            "source_dataset": "otcMarket/consolidatedShortInterest",
            "source_note": "FINRA consolidated short interest row reports CRCL marketClassCode as NYSE"
        }),
    )];

    if let Some(days_to_cover) = row.days_to_cover_quantity {
        observations.push(obs!(
            "P2_CRCL_DAYS_TO_COVER",
            "CRCL days to cover",
            "P2",
            "equity_market",
            Some(days_to_cover),
            None,
            "days",
            &row.settlement_date,
            "FINRA consolidated short interest",
            source_url,
            json!({"symbol": row.symbol_code}),
        ));
    }

    if let Some(change_percent) = row.change_percent {
        observations.push(obs!(
            "P2_CRCL_SHORT_INTEREST_CHANGE_PCT",
            "CRCL short interest change",
            "P2",
            "equity_market",
            Some(change_percent),
            None,
            "percent",
            &row.settlement_date,
            "FINRA consolidated short interest",
            source_url,
            json!({"symbol": row.symbol_code}),
        ));
    }

    Ok(observations)
}

#[derive(Debug, Deserialize)]
struct SecSubmission {
    name: String,
    cik: String,
    filings: SecFilings,
}

#[derive(Debug, Deserialize)]
struct SecFilings {
    recent: SecRecent,
}

#[derive(Debug, Deserialize)]
struct SecRecent {
    #[serde(rename = "accessionNumber")]
    accession_number: Vec<String>,
    #[serde(rename = "filingDate")]
    filing_date: Vec<String>,
    #[serde(rename = "reportDate")]
    report_date: Vec<String>,
    form: Vec<String>,
    #[serde(rename = "primaryDocument")]
    primary_document: Vec<String>,
    #[serde(rename = "primaryDocDescription")]
    primary_doc_description: Vec<String>,
}

pub fn parse_sec_submissions(
    text: &str,
    source_url: &str,
) -> Result<(Vec<Filing>, Vec<Observation>)> {
    let response: SecSubmission = serde_json::from_str(text)?;
    let forms = ["10-Q", "10-K", "8-K", "4", "13F-HR", "N-MFP3"];
    let mut filings = Vec::new();
    let mut observations = Vec::new();
    let mut latest_by_form: HashMap<String, &Filing> = HashMap::new();

    for idx in 0..response.filings.recent.form.len() {
        let Some(form) = response.filings.recent.form.get(idx) else {
            continue;
        };
        if !forms.contains(&form.as_str()) {
            continue;
        }
        let accession_no = response
            .filings
            .recent
            .accession_number
            .get(idx)
            .cloned()
            .unwrap_or_default();
        let filing_date = response
            .filings
            .recent
            .filing_date
            .get(idx)
            .cloned()
            .unwrap_or_default();
        let filing = Filing {
            company: response.name.clone(),
            cik: response.cik.clone(),
            accession_no,
            form: form.clone(),
            filing_date,
            report_date: opt_string(response.filings.recent.report_date.get(idx)),
            primary_doc: opt_string(response.filings.recent.primary_document.get(idx)),
            description: opt_string(response.filings.recent.primary_doc_description.get(idx)),
            source_url: source_url.to_string(),
        };
        filings.push(filing);
    }

    for filing in &filings {
        latest_by_form.entry(filing.form.clone()).or_insert(filing);
    }

    for (form, filing) in latest_by_form {
        observations.push(obs!(
            &format!(
                "P2_SEC_{}_LATEST_{}",
                metric_suffix(&response.name),
                metric_suffix(&form)
            ),
            &format!("Latest SEC {form}: {}", response.name),
            if form == "8-K" || form == "4" {
                "P0"
            } else {
                "P2"
            },
            "sec_filing",
            None,
            Some(filing.accession_no.clone()),
            "filing",
            &filing.filing_date,
            "SEC EDGAR submissions",
            source_url,
            json!({
                "company": filing.company,
                "cik": filing.cik,
                "form": filing.form,
                "report_date": filing.report_date,
                "primary_doc": filing.primary_doc,
                "description": filing.description
            }),
        ));
    }

    Ok((filings, observations))
}

#[derive(Debug)]
struct IxContext {
    start_date: Option<String>,
    end_date: Option<String>,
    instant: Option<String>,
    segments: Vec<String>,
}

#[derive(Debug)]
struct IxFact {
    name: String,
    context_ref: String,
    value_num: Option<f64>,
}

pub fn parse_circle_sec_filing(
    text: &str,
    source_url: &str,
    observed_at: &str,
) -> Result<Vec<Observation>> {
    let ix = parse_inline_xbrl(text)?;
    let reserve_income = duration_fact(
        &ix,
        "us-gaap:InterestAndDividendIncomeOperating",
        observed_at,
        None,
    )
    .context("Circle filing reserve income fact missing")?;
    let other_revenue = duration_fact(
        &ix,
        "us-gaap:RevenueFromContractWithCustomerExcludingAssessedTax",
        observed_at,
        None,
    )
    .context("Circle filing other revenue fact missing")?;
    let total_revenue = duration_fact(&ix, "us-gaap:Revenues", observed_at, None)
        .context("Circle filing total revenue and reserve income fact missing")?;
    let distribution_costs = duration_fact(
        &ix,
        "crcl:DistributionAndTransactionCosts",
        observed_at,
        None,
    )
    .context("Circle filing distribution costs fact missing")?;
    let total_distribution_costs = duration_fact(
        &ix,
        "crcl:DistributionTransactionAndOtherCosts",
        observed_at,
        None,
    )
    .context("Circle filing total distribution transaction and other costs fact missing")?;
    let net_income = duration_fact(&ix, "us-gaap:NetIncomeLoss", observed_at, None)
        .context("Circle filing net income fact missing")?;
    let adjusted_ebitda = adjusted_ebitda_from_text(text)
        .context("Circle filing Adjusted EBITDA reconciliation value missing")?;
    let basic_shares = duration_fact(
        &ix,
        "us-gaap:WeightedAverageNumberOfSharesOutstandingBasic",
        observed_at,
        None,
    );
    let diluted_shares = duration_fact(
        &ix,
        "us-gaap:WeightedAverageNumberOfDilutedSharesOutstanding",
        observed_at,
        None,
    );
    let shares_outstanding = instant_fact(
        &ix,
        "dei:EntityCommonStockSharesOutstanding",
        observed_at,
        None,
    );
    let usdc_end = filing_table_value_after_label(text, "USDC in circulation, end of period")
        .map(|value| value * 1_000_000.0);
    let usdc_average =
        filing_table_value_after_label(text, "USDC in circulation, average of period")
            .map(|value| value * 1_000_000.0);
    let onchain_volume = filing_trillion_value_after_label(text, "USDC onchain transaction volume")
        .map(|value| value * 1_000_000_000_000.0);
    let holder_cash = instant_fact(
        &ix,
        "crcl:CashAndCashEquivalentsSegregatedForTheBenefitOfStablecoinHolders",
        observed_at,
        None,
    );
    let corporate_stablecoin_cash = instant_fact(
        &ix,
        "crcl:CashAndCashEquivalentsSegregatedForCorporateHeldStablecoins",
        observed_at,
        None,
    );

    let rldc = total_revenue - total_distribution_costs;
    let rldc_margin = percent_ratio(rldc, total_revenue);
    let other_revenue_share = percent_ratio(other_revenue, total_revenue);
    let adjusted_ebitda_margin = percent_ratio(adjusted_ebitda, rldc);

    let mut out = vec![
        sec_obs(
            "P2_CIRCLE_RESERVE_INCOME",
            "Circle reserve income",
            "income_statement",
            reserve_income,
            observed_at,
            source_url,
            "us-gaap:InterestAndDividendIncomeOperating",
        ),
        sec_obs(
            "P2_CIRCLE_OTHER_REVENUE",
            "Circle other revenue",
            "income_statement",
            other_revenue,
            observed_at,
            source_url,
            "us-gaap:RevenueFromContractWithCustomerExcludingAssessedTax",
        ),
        obs!(
            "P2_CIRCLE_OTHER_REVENUE_SHARE",
            "Circle other revenue share",
            "P2",
            "income_statement",
            Some(other_revenue_share),
            None,
            "percent",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({
                "method": "P2_CIRCLE_OTHER_REVENUE / P2_CIRCLE_TOTAL_REVENUE_AND_RESERVE_INCOME",
                "other_revenue": other_revenue,
                "total_revenue_and_reserve_income": total_revenue,
            }),
        ),
        sec_obs(
            "P2_CIRCLE_TOTAL_REVENUE_AND_RESERVE_INCOME",
            "Circle total revenue and reserve income",
            "income_statement",
            total_revenue,
            observed_at,
            source_url,
            "us-gaap:Revenues",
        ),
        sec_obs(
            "P2_CIRCLE_DISTRIBUTION_TRANSACTION_COSTS",
            "Circle distribution and transaction costs",
            "income_statement",
            distribution_costs,
            observed_at,
            source_url,
            "crcl:DistributionAndTransactionCosts",
        ),
        sec_obs(
            "P2_CIRCLE_TOTAL_DISTRIBUTION_TRANSACTION_OTHER_COSTS",
            "Circle total distribution transaction and other costs",
            "income_statement",
            total_distribution_costs,
            observed_at,
            source_url,
            "crcl:DistributionTransactionAndOtherCosts",
        ),
        sec_obs(
            "P2_CIRCLE_NET_INCOME",
            "Circle net income",
            "income_statement",
            net_income,
            observed_at,
            source_url,
            "us-gaap:NetIncomeLoss",
        ),
        obs!(
            "P2_CIRCLE_RLDC",
            "Circle revenue less distribution costs",
            "P2",
            "income_statement",
            Some(rldc),
            None,
            "USD",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({"method": "P2_CIRCLE_TOTAL_REVENUE_AND_RESERVE_INCOME - P2_CIRCLE_TOTAL_DISTRIBUTION_TRANSACTION_OTHER_COSTS"}),
        ),
        obs!(
            "P2_CIRCLE_RLDC_MARGIN",
            "Circle RLDC margin",
            "P2",
            "income_statement",
            Some(rldc_margin),
            None,
            "percent",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({"method": "RLDC / total revenue and reserve income"}),
        ),
        obs!(
            "P2_CIRCLE_ADJUSTED_EBITDA",
            "Circle Adjusted EBITDA",
            "P2",
            "income_statement",
            Some(adjusted_ebitda),
            None,
            "USD",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({"source_section": "Non-GAAP Financial Measures reconciliation"}),
        ),
        obs!(
            "P2_CIRCLE_ADJUSTED_EBITDA_MARGIN",
            "Circle Adjusted EBITDA margin",
            "P2",
            "income_statement",
            Some(adjusted_ebitda_margin),
            None,
            "percent",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({"method": "Adjusted EBITDA / RLDC"}),
        ),
    ];

    if let Some(value) = usdc_end {
        out.push(obs!(
            "P2_CIRCLE_USDC_IN_CIRCULATION_END_PERIOD",
            "Circle USDC in circulation, end of period",
            "P2",
            "operating_indicator",
            Some(value),
            None,
            "USD",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({"source_label": "USDC in circulation, end of period", "source_unit": "millions"}),
        ));
    }
    if let Some(value) = usdc_average {
        out.push(obs!(
            "P2_CIRCLE_USDC_IN_CIRCULATION_AVG_PERIOD",
            "Circle USDC in circulation, average of period",
            "P2",
            "operating_indicator",
            Some(value),
            None,
            "USD",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({"source_label": "USDC in circulation, average of period", "source_unit": "millions"}),
        ));
    }
    if let Some(value) = onchain_volume {
        out.push(obs!(
            "P1_USDC_ONCHAIN_TRANSACTION_VOLUME",
            "USDC onchain transaction volume",
            "P1",
            "chain_activity",
            Some(value),
            None,
            "USD",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({"source_label": "USDC onchain transaction volume", "source_unit": "trillions"}),
        ));
    }
    if let Some(value) = basic_shares {
        out.push(obs!(
            "P2_CRCL_BASIC_SHARES_OUTSTANDING",
            "CRCL weighted-average basic shares outstanding",
            "P2",
            "equity_valuation",
            Some(value),
            None,
            "shares",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({"xbrl_name": "us-gaap:WeightedAverageNumberOfSharesOutstandingBasic"}),
        ));
    }
    if let Some(value) = diluted_shares {
        out.push(obs!(
            "P2_CRCL_DILUTED_SHARES_OUTSTANDING",
            "CRCL weighted-average diluted shares outstanding",
            "P2",
            "equity_valuation",
            Some(value),
            None,
            "shares",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({"xbrl_name": "us-gaap:WeightedAverageNumberOfDilutedSharesOutstanding"}),
        ));
    }
    if let Some(value) = shares_outstanding {
        out.push(obs!(
            "P2_CRCL_SHARES_OUTSTANDING",
            "CRCL common shares outstanding",
            "P2",
            "equity_valuation",
            Some(value),
            None,
            "shares",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({"xbrl_name": "dei:EntityCommonStockSharesOutstanding"}),
        ));
    }
    if let (Some(volume), Some(average_circulation)) = (onchain_volume, usdc_average)
        && average_circulation > 0.0
    {
        out.push(obs!(
            "P1_USDC_VELOCITY",
            "USDC onchain velocity, filing period",
            "P1",
            "chain_activity",
            Some(volume / average_circulation),
            None,
            "turns",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({
                "method": "USDC onchain transaction volume / average USDC in circulation",
                "volume_metric": "P1_USDC_ONCHAIN_TRANSACTION_VOLUME",
                "average_circulation_metric": "P2_CIRCLE_USDC_IN_CIRCULATION_AVG_PERIOD",
                "source_note": "Circle filing period metric; not CoinMetrics adjusted transfer volume"
            }),
        ));
    }
    if let Some(value) = holder_cash {
        out.push(sec_obs(
            "P2_CIRCLE_SEGREGATED_CASH_FOR_STABLECOIN_HOLDERS",
            "Circle cash segregated for stablecoin holders",
            "reserve_composition",
            value,
            observed_at,
            source_url,
            "crcl:CashAndCashEquivalentsSegregatedForTheBenefitOfStablecoinHolders",
        ));
    }
    if let Some(value) = corporate_stablecoin_cash {
        out.push(sec_obs(
            "P2_CIRCLE_SEGREGATED_CASH_FOR_CORPORATE_HELD_STABLECOINS",
            "Circle cash segregated for corporate-held stablecoins",
            "reserve_composition",
            value,
            observed_at,
            source_url,
            "crcl:CashAndCashEquivalentsSegregatedForCorporateHeldStablecoins",
        ));
    }

    Ok(out)
}

pub fn parse_coinbase_sec_filing(
    text: &str,
    source_url: &str,
    observed_at: &str,
) -> Result<Vec<Observation>> {
    let ix = parse_inline_xbrl(text)?;
    let stablecoin_revenue = duration_fact(
        &ix,
        "us-gaap:RevenueFromContractWithCustomerExcludingAssessedTax",
        observed_at,
        Some("coin:SubscriptionAndCirculationStablecoinMember"),
    )
    .context("Coinbase filing stablecoin revenue fact missing")?;
    let stablecoin_accounts_receivable = instant_fact(
        &ix,
        "coin:AccountsReceivableBeforeAllowanceForCreditLossStablecoinRevenue",
        observed_at,
        None,
    )
    .context("Coinbase filing stablecoin revenue receivable fact missing")?;
    let stablecoin_financing_receivable = instant_fact(
        &ix,
        "coin:FiatAndPaymentStablecoinFinancingReceivableExcludingAccruedInterestBeforeAllowanceForCreditLoss",
        observed_at,
        None,
    )
    .context("Coinbase filing fiat and payment stablecoin financing receivable fact missing")?;
    let client_custodial_funds =
        instant_fact(&ix, "coin:ClientCustodialFundsCurrent", observed_at, None)
            .context("Coinbase filing client custodial funds fact missing")?;

    let mut out = vec![
        sec_obs(
            "P1_COINBASE_STABLECOIN_REVENUE",
            "Coinbase subscription and circulation stablecoin revenue",
            "channel_dependence",
            stablecoin_revenue,
            observed_at,
            source_url,
            "us-gaap:RevenueFromContractWithCustomerExcludingAssessedTax / coin:SubscriptionAndCirculationStablecoinMember",
        ),
        sec_obs(
            "P1_COINBASE_STABLECOIN_REVENUE_RECEIVABLE",
            "Coinbase stablecoin revenue accounts receivable",
            "channel_dependence",
            stablecoin_accounts_receivable,
            observed_at,
            source_url,
            "coin:AccountsReceivableBeforeAllowanceForCreditLossStablecoinRevenue",
        ),
        sec_obs(
            "P1_COINBASE_FIAT_PAYMENT_STABLECOIN_FINANCING_RECEIVABLE",
            "Coinbase fiat and payment stablecoin financing receivable",
            "channel_dependence",
            stablecoin_financing_receivable,
            observed_at,
            source_url,
            "coin:FiatAndPaymentStablecoinFinancingReceivableExcludingAccruedInterestBeforeAllowanceForCreditLoss",
        ),
        sec_obs(
            "P1_COINBASE_CLIENT_CUSTODIAL_FUNDS",
            "Coinbase client custodial funds",
            "channel_dependence",
            client_custodial_funds,
            observed_at,
            source_url,
            "coin:ClientCustodialFundsCurrent",
        ),
    ];

    if let Some((usdc_on_platform, total_aop)) = coinbase_usdc_on_platform_from_text(text) {
        out.push(obs!(
            "P1_COINBASE_USDC_ON_PLATFORM",
            "Coinbase customer USDC on platform",
            "P1",
            "channel_dependence",
            Some(usdc_on_platform),
            None,
            "USD",
            observed_at,
            "SEC EDGAR filing",
            source_url,
            json!({
                "source_field": "Assets on Platform by asset table / USDC row",
                "source_note": "Quarter-end customer USDC AOP, table is reported in millions; not average USDC balances held in eligible Coinbase products",
                "total_assets_on_platform": total_aop,
                "share_of_aop_percent": percent_ratio(usdc_on_platform, total_aop)
            }),
        ));
    }

    Ok(out)
}

pub fn parse_sec_blackrock_nmfp3_filing(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let html = Html::parse_document(text);
    let rows = html_table_rows(&html);
    let report_date = row_value_by_label(&rows, "Item 1.")
        .and_then(|value| first_iso_date(&value))
        .context("BlackRock N-MFP3 report date missing")?;
    let series_name = row_value_by_label(&rows, "Item 5.").context("N-MFP3 series name missing")?;
    let series_id =
        row_value_by_label(&rows, "Item 7.").context("N-MFP3 series identifier missing")?;
    if series_name != "Circle Reserve Fund" && series_id != "S000077205" {
        return Err(anyhow!(
            "BlackRock N-MFP3 filing is not Circle Reserve Fund: {series_name} / {series_id}"
        ));
    }

    let class_name = row_value_by_label(&rows, "Item B.1.");
    let class_id = row_value_by_label(&rows, "Item B.2.");
    let wam = row_value_by_label(&rows, "Item A.11.")
        .and_then(|value| first_number_after(&value))
        .context("Circle Reserve Fund WAM missing in N-MFP3 filing")?;
    let wal = row_value_by_label(&rows, "Item A.12.")
        .and_then(|value| first_number_after(&value))
        .context("Circle Reserve Fund WAL missing in N-MFP3 filing")?;
    let (net_yield_date, net_yield) = latest_dated_metric(&rows, "7-day net yield")
        .context("Circle Reserve Fund 7-day net yield missing in N-MFP3 filing")?;

    let mut observations = vec![
        nmfp_obs(NmfpObservationSpec {
            code: "P0_CIRCLE_RESERVE_FUND_7D_YIELD",
            name: "Circle Reserve Fund 7-day net yield",
            priority: "P0",
            value_num: Some(net_yield),
            unit: "percent",
            observed_at: &net_yield_date,
            source_url,
            attributes: json!({
                "fund": series_name,
                "series_id": series_id,
                "class_name": class_name,
                "class_id": class_id,
                "report_date": report_date,
                "source_field": "Form N-MFP3 Item B.8 7-day net yield",
                "source_note": "SEC monthly N-MFP3 latest reported business day; not BlackRock daily product page"
            }),
        }),
        nmfp_obs(NmfpObservationSpec {
            code: "P2_RESERVE_FUND_WAM",
            name: "Circle Reserve Fund WAM",
            priority: "P2",
            value_num: Some(wam),
            unit: "days",
            observed_at: &report_date,
            source_url,
            attributes: json!({
                "fund": series_name,
                "series_id": series_id,
                "report_date": report_date,
                "source_field": "Form N-MFP3 Item A.11 WAM"
            }),
        }),
        nmfp_obs(NmfpObservationSpec {
            code: "P2_RESERVE_FUND_WAL",
            name: "Circle Reserve Fund WAL",
            priority: "P2",
            value_num: Some(wal),
            unit: "days",
            observed_at: &report_date,
            source_url,
            attributes: json!({
                "fund": series_name,
                "series_id": series_id,
                "report_date": report_date,
                "source_field": "Form N-MFP3 Item A.12 WAL"
            }),
        }),
    ];

    if let Some((gross_yield_date, gross_yield)) = latest_dated_metric(&rows, "7-day gross yield") {
        observations.push(nmfp_obs(NmfpObservationSpec {
            code: "P2_CIRCLE_RESERVE_FUND_7D_GROSS_YIELD",
            name: "Circle Reserve Fund 7-day gross yield",
            priority: "P2",
            value_num: Some(gross_yield),
            unit: "percent",
            observed_at: &gross_yield_date,
            source_url,
            attributes: json!({
                "fund": series_name,
                "series_id": series_id,
                "report_date": report_date,
                "source_field": "Form N-MFP3 Item A.19 7-day gross yield",
                "source_note": "SEC monthly N-MFP3 latest reported business day"
            }),
        }));
    }

    if let Some(net_assets) =
        row_value_by_label(&rows, "Item B.4.").and_then(|value| first_number_after(&value))
    {
        observations.push(nmfp_obs(NmfpObservationSpec {
            code: "P2_RESERVE_FUND_NET_ASSETS",
            name: "Circle Reserve Fund net assets",
            priority: "P2",
            value_num: Some(net_assets),
            unit: "USD",
            observed_at: &report_date,
            source_url,
            attributes: json!({
                "fund": series_name,
                "series_id": series_id,
                "class_name": class_name,
                "class_id": class_id,
                "report_date": report_date,
                "source_field": "Form N-MFP3 Item B.4 class net assets"
            }),
        }));
    }

    Ok(observations)
}

fn opt_string(value: Option<&String>) -> Option<String> {
    value.cloned().filter(|value| !value.trim().is_empty())
}

fn parse_inline_xbrl(text: &str) -> Result<(HashMap<String, IxContext>, Vec<IxFact>)> {
    let doc = Document::parse(text).context("SEC inline XBRL document is not valid XML")?;
    let mut contexts = HashMap::new();
    let mut facts = Vec::new();

    for node in doc
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "context")
    {
        let Some(id) = node.attribute("id") else {
            continue;
        };
        let start_date = child_text(node, "startDate");
        let end_date = child_text(node, "endDate");
        let instant = child_text(node, "instant");
        let segments = node
            .descendants()
            .filter(|child| child.is_element() && child.tag_name().name() == "explicitMember")
            .map(|child| normalize_text(&node_text(&child)))
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        contexts.insert(
            id.to_string(),
            IxContext {
                start_date,
                end_date,
                instant,
                segments,
            },
        );
    }

    for node in doc
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "nonFraction")
    {
        let Some(name) = node.attribute("name") else {
            continue;
        };
        let Some(context_ref) = node.attribute("contextRef") else {
            continue;
        };
        let scale = node
            .attribute("scale")
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or(0);
        let value_raw = normalize_text(&node_text(&node));
        let value_num = parse_sec_number(&value_raw).map(|value| value * 10_f64.powi(scale));
        facts.push(IxFact {
            name: name.to_string(),
            context_ref: context_ref.to_string(),
            value_num,
        });
    }

    Ok((contexts, facts))
}

fn duration_fact(
    ix: &(HashMap<String, IxContext>, Vec<IxFact>),
    name: &str,
    observed_at: &str,
    segment: Option<&str>,
) -> Option<f64> {
    let (contexts, facts) = ix;
    facts
        .iter()
        .find(|fact| {
            fact.name == name
                && fact_matches_context(contexts, fact, observed_at, segment, ContextKind::Duration)
        })
        .and_then(|fact| fact.value_num)
}

fn instant_fact(
    ix: &(HashMap<String, IxContext>, Vec<IxFact>),
    name: &str,
    observed_at: &str,
    segment: Option<&str>,
) -> Option<f64> {
    let (contexts, facts) = ix;
    facts
        .iter()
        .find(|fact| {
            fact.name == name
                && fact_matches_context(contexts, fact, observed_at, segment, ContextKind::Instant)
        })
        .and_then(|fact| fact.value_num)
}

#[derive(Clone, Copy)]
enum ContextKind {
    Duration,
    Instant,
}

fn fact_matches_context(
    contexts: &HashMap<String, IxContext>,
    fact: &IxFact,
    observed_at: &str,
    segment: Option<&str>,
    kind: ContextKind,
) -> bool {
    let Some(context) = contexts.get(&fact.context_ref) else {
        return false;
    };
    let period_matches = match kind {
        ContextKind::Duration => {
            context.start_date.is_some() && context.end_date.as_deref() == Some(observed_at)
        }
        ContextKind::Instant => context.instant.as_deref() == Some(observed_at),
    };
    if !period_matches {
        return false;
    }
    match segment {
        Some(segment) => context.segments.iter().any(|value| value == segment),
        None => context.segments.is_empty(),
    }
}

fn node_text(node: &roxmltree::Node<'_, '_>) -> String {
    node.children()
        .flat_map(|child| child.descendants())
        .filter_map(|child| child.text())
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_sec_number(input: &str) -> Option<f64> {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("no") {
        return None;
    }
    let is_parenthesized = trimmed.contains('(') && trimmed.contains(')');
    let cleaned = trimmed
        .chars()
        .filter(|ch| ch.is_ascii_digit() || *ch == '.' || *ch == '-')
        .collect::<String>();
    if cleaned.is_empty() || cleaned == "-" || cleaned == "." {
        return None;
    }
    let parsed = cleaned.parse::<f64>().ok()?;
    Some(if is_parenthesized {
        -parsed.abs()
    } else {
        parsed
    })
}

fn sec_obs(
    code: &str,
    name: &str,
    category: &str,
    value: f64,
    observed_at: &str,
    source_url: &str,
    source_field: &str,
) -> Observation {
    obs!(
        code,
        name,
        if code.starts_with("P1_") { "P1" } else { "P2" },
        category,
        Some(value),
        None,
        "USD",
        observed_at,
        "SEC EDGAR filing",
        source_url,
        json!({"source_field": source_field}),
    )
}

struct NmfpObservationSpec<'a> {
    code: &'a str,
    name: &'a str,
    priority: &'a str,
    value_num: Option<f64>,
    unit: &'a str,
    observed_at: &'a str,
    source_url: &'a str,
    attributes: Value,
}

fn nmfp_obs(spec: NmfpObservationSpec<'_>) -> Observation {
    obs!(
        spec.code,
        spec.name,
        spec.priority,
        "reserve_fund",
        spec.value_num,
        None,
        spec.unit,
        spec.observed_at,
        "SEC EDGAR filing: BlackRock Funds N-MFP3",
        spec.source_url,
        spec.attributes,
    )
}

fn institutional_obs(
    code: &str,
    name: &str,
    value: f64,
    unit: &str,
    observed_at: &str,
    source_url: &str,
    source_section: &str,
) -> Observation {
    obs!(
        code,
        name,
        "P2",
        "institutional_ownership",
        Some(value),
        None,
        unit,
        observed_at,
        "MarketBeat institutional ownership",
        source_url,
        json!({
            "symbol": "CRCL",
            "method": "parse public MarketBeat CRCL institutional ownership page",
            "source_section": source_section,
        }),
    )
}

fn percent_ratio(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator * 100.0
    }
}

fn adjusted_ebitda_from_text(text: &str) -> Option<f64> {
    let plain = html_plain_text(text);
    let section = section_after(
        &plain,
        "The following table reconciles Adjusted EBITDA to net income",
    )?;
    let before_footnotes = section
        .split("Reflects interest income")
        .next()
        .unwrap_or(section);
    let label_idx = before_footnotes.rfind("Adjusted EBITDA")?;
    first_number_after(&before_footnotes[label_idx + "Adjusted EBITDA".len()..])
        .map(|value| value * 1_000.0)
}

fn filing_table_value_after_label(text: &str, label: &str) -> Option<f64> {
    let plain = html_plain_text(text);
    let idx = plain.find(label)?;
    first_number_after(&plain[idx + label.len()..])
}

fn filing_trillion_value_after_label(text: &str, label: &str) -> Option<f64> {
    let plain = html_plain_text(text);
    let growth_marker = format!("{label} grew");
    let idx = plain.find(&growth_marker).or_else(|| plain.find(label))?;
    let after = &plain[idx..];
    let window = after.chars().take(400).collect::<String>();
    let dollar_idx = window.find('$')?;
    first_number_after(&window[dollar_idx + 1..])
}

fn coinbase_usdc_on_platform_from_text(text: &str) -> Option<(f64, f64)> {
    let plain = html_plain_text(text);
    let section = section_after(
        &plain,
        "The following table sets forth the value of AOP by asset",
    )?;
    let table = section.split("Trading Volume").next().unwrap_or(section);
    let usdc_idx = table.find("USDC N/A")?;
    let total_idx = table.find("Total $").or_else(|| table.find("Total"))?;
    let usdc_millions = first_number_after(&table[usdc_idx + "USDC N/A".len()..])?;
    let total_millions = first_number_after(&table[total_idx + "Total".len()..])?;
    Some((usdc_millions * 1_000_000.0, total_millions * 1_000_000.0))
}

fn section_after<'a>(text: &'a str, marker: &str) -> Option<&'a str> {
    let idx = text.find(marker)?;
    Some(&text[idx + marker.len()..])
}

fn first_number_after(text: &str) -> Option<f64> {
    for raw_token in text.split_whitespace().take(80) {
        if raw_token.starts_with('(')
            && raw_token.ends_with(')')
            && raw_token.chars().filter(|ch| ch.is_ascii_digit()).count() <= 2
        {
            continue;
        }
        let token = raw_token
            .trim_matches(|ch: char| matches!(ch, '$' | '%' | ',' | '.' | ';' | ':' | ')' | '('));
        let normalized = token.replace(',', "");
        if normalized.chars().any(|ch| ch.is_ascii_digit())
            && let Ok(value) = normalized.parse::<f64>()
        {
            return Some(value);
        }
    }
    None
}

fn number_after_marker(text: &str, marker: &str) -> Option<f64> {
    let idx = text.find(marker)? + marker.len();
    number_with_unit(&text[idx..])
}

fn number_before_marker(text: &str, marker: &str) -> Option<f64> {
    let idx = text.find(marker)?;
    let before = &text[..idx];
    let tokens = before.split_whitespace().collect::<Vec<_>>();
    for start in (0..tokens.len()).rev().take(16) {
        let candidate = tokens[start..].join(" ");
        if let Some(value) = number_with_unit(&candidate) {
            return Some(value);
        }
    }
    None
}

fn number_with_unit(text: &str) -> Option<f64> {
    let mut previous_number = None;
    for (tokens_seen, raw_token) in text.split_whitespace().enumerate() {
        if tokens_seen >= 80 {
            break;
        }
        let token = raw_token
            .trim_matches(|ch: char| matches!(ch, '$' | '%' | ',' | '.' | ';' | ':' | ')' | '('));
        let lower = token.to_ascii_lowercase();
        if let Some(value) = previous_number.take() {
            if lower.starts_with("trillion") {
                return Some(value * 1_000_000_000_000.0);
            }
            if lower.starts_with("billion") {
                return Some(value * 1_000_000_000.0);
            }
            if lower.starts_with("million") {
                return Some(value * 1_000_000.0);
            }
            if lower.starts_with("thousand") {
                return Some(value * 1_000.0);
            }
            return Some(value);
        }

        if let Some(value) = parse_number_token(token) {
            if lower.ends_with('t') {
                return Some(value * 1_000_000_000_000.0);
            }
            if lower.ends_with('b') {
                return Some(value * 1_000_000_000.0);
            }
            if lower.ends_with('m') {
                return Some(value * 1_000_000.0);
            }
            if lower.ends_with('k') {
                return Some(value * 1_000.0);
            }
            previous_number = Some(value);
        }
    }
    previous_number
}

fn parse_number_token(token: &str) -> Option<f64> {
    let cleaned = token
        .chars()
        .filter(|ch| ch.is_ascii_digit() || *ch == '.' || *ch == '-' || *ch == '+')
        .collect::<String>();
    if cleaned.is_empty() || cleaned == "." || cleaned == "-" || cleaned == "+" {
        return None;
    }
    cleaned.parse::<f64>().ok()
}

fn last_number_in_text(text: &str) -> Option<f64> {
    text.split_whitespace()
        .filter_map(|token| {
            parse_number_token(token.trim_matches(|ch: char| {
                matches!(ch, '$' | '%' | ',' | '.' | ';' | ':' | ')' | '(')
            }))
        })
        .next_back()
}

fn money_amount_after(text: &str, marker: &str) -> Option<f64> {
    let idx = text.find(marker).map(|idx| idx + marker.len()).unwrap_or(0);
    let tail = &text[idx..];
    let dollar_idx = tail.find('$')?;
    number_with_unit(&tail[dollar_idx..])
}

fn numeric_field(value: &Value, names: &[&str]) -> Option<f64> {
    names.iter().find_map(|name| {
        let value = value.get(*name)?;
        value.as_f64().or_else(|| {
            value
                .as_str()
                .map(|text| text.replace(',', ""))
                .and_then(|text| text.parse::<f64>().ok())
        })
    })
}

fn string_field(value: &Value, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        value
            .get(*name)?
            .as_str()
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .map(str::to_string)
    })
}

fn first_long_form_date_after(text: &str, marker: &str) -> Option<String> {
    let idx = text.find(marker).map(|idx| idx + marker.len()).unwrap_or(0);
    let tail = &text[idx..];
    for month in [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ] {
        let Some(month_idx) = tail.find(month) else {
            continue;
        };
        let candidate = tail[month_idx..]
            .chars()
            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == ',' || *ch == ' ')
            .collect::<String>();
        let candidate = candidate
            .split_whitespace()
            .take(3)
            .collect::<Vec<_>>()
            .join(" ");
        if let Ok(date) = NaiveDate::parse_from_str(&candidate, "%B %d, %Y")
            .or_else(|_| NaiveDate::parse_from_str(&candidate, "%B %e, %Y"))
        {
            return Some(date.to_string());
        }
    }
    None
}

fn first_iso_date(text: &str) -> Option<String> {
    text.split_whitespace().find_map(|token| {
        let cleaned = token.trim_matches(|ch: char| !(ch.is_ascii_digit() || ch == '-'));
        if cleaned.len() == 10
            && cleaned.as_bytes().get(4) == Some(&b'-')
            && cleaned.as_bytes().get(7) == Some(&b'-')
            && NaiveDate::parse_from_str(cleaned, "%Y-%m-%d").is_ok()
        {
            Some(cleaned.to_string())
        } else {
            None
        }
    })
}

fn html_table_rows(html: &Html) -> Vec<(String, String)> {
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();
    html.select(&row_selector)
        .filter_map(|row| {
            let cells = row
                .select(&cell_selector)
                .map(|cell| normalize_text(&cell.text().collect::<Vec<_>>().join(" ")))
                .collect::<Vec<_>>();
            let label = cells.first()?.to_string();
            let value = cells.get(1).cloned().unwrap_or_default();
            if label.is_empty() && value.is_empty() {
                None
            } else {
                Some((label, value))
            }
        })
        .collect()
}

fn row_value_by_label(rows: &[(String, String)], label: &str) -> Option<String> {
    rows.iter()
        .find(|(row_label, value)| row_label.contains(label) && !value.trim().is_empty())
        .map(|(_, value)| value.to_string())
}

fn latest_dated_metric(rows: &[(String, String)], label: &str) -> Option<(String, f64)> {
    let target = label.to_ascii_lowercase();
    let mut pending_value = None;
    let mut values = Vec::new();

    for (row_label, value) in rows {
        let row_label = row_label.to_ascii_lowercase();
        if row_label.contains(&target) {
            pending_value = first_number_after(value);
        } else if row_label.contains("date")
            && let Some(value_num) = pending_value.take()
            && let Some(date) = first_iso_date(value)
        {
            values.push((date, value_num));
        }
    }

    values
        .into_iter()
        .max_by(|left, right| left.0.cmp(&right.0))
}

fn html_plain_text(text: &str) -> String {
    let html = Html::parse_document(text);
    let body_selector = Selector::parse("body").unwrap();
    html.select(&body_selector)
        .next()
        .map(|body| normalize_text(&body.text().collect::<Vec<_>>().join(" ")))
        .unwrap_or_else(|| normalize_text(text))
}

fn arc_status_from_text(text: &str) -> Option<String> {
    for marker in [
        "Arc remains on track for mainnet launch this year",
        "meaningful progress toward launching Arc mainnet",
        "soon launch on mainnet",
    ] {
        if text.contains(marker) {
            return Some(marker.to_string());
        }
    }
    None
}

fn arc_metrics_section_start(text: &str) -> Option<usize> {
    [
        "Arc: Public testnet",
        "Arc : Public testnet",
        "Public testnet launched",
        "Arc remains on track for mainnet launch this year",
        "meaningful progress toward launching Arc mainnet",
        "soon launch on mainnet",
        "Arc",
    ]
    .iter()
    .find_map(|marker| text.find(marker))
}

fn marketbeat_revised_date(html: &Html) -> Option<String> {
    let selector = Selector::parse("meta[name='revised'], meta[name='last-modified']").ok()?;
    html.select(&selector).find_map(|node| {
        let date = node.value().attr("content")?;
        if NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok() {
            Some(date.to_string())
        } else {
            None
        }
    })
}

pub fn parse_circle_pressroom(text: &str, _source_url: &str) -> Result<Vec<Event>> {
    let html = Html::parse_document(text);
    let link_selector = Selector::parse("a.press-link").unwrap();
    let date_selector = Selector::parse(".caption-disclosure").unwrap();
    let title_selector = Selector::parse("[fs-list-field='title'], .h6").unwrap();
    let desc_selector = Selector::parse("[fs-list-field='description'], .paragraph-body").unwrap();
    let mut events = Vec::new();

    for element in html.select(&link_selector).take(12) {
        let href = element.value().attr("href").unwrap_or_default();
        let url = if href.starts_with("http") {
            href.to_string()
        } else {
            format!("https://www.circle.com{href}")
        };
        let event_date = element
            .select(&date_selector)
            .next()
            .map(|node| normalize_text(&node.text().collect::<Vec<_>>().join(" ")));
        let title = element
            .select(&title_selector)
            .next()
            .map(|node| normalize_text(&node.text().collect::<Vec<_>>().join(" ")))
            .unwrap_or_else(|| url.clone());
        let summary = element
            .select(&desc_selector)
            .nth(1)
            .or_else(|| element.select(&desc_selector).next())
            .map(|node| normalize_text(&node.text().collect::<Vec<_>>().join(" ")));
        events.push(Event {
            source: "Circle Pressroom".to_string(),
            event_type: "company_press_release".to_string(),
            title,
            event_date,
            url,
            summary,
        });
    }

    Ok(events)
}

pub fn parse_circle_transparency(text: &str, source_url: &str) -> Result<Vec<Observation>> {
    let html = Html::parse_document(text);
    let observed_at =
        parse_circle_as_of_date(text).unwrap_or_else(|| Utc::now().date_naive().to_string());
    let circulation = data_point_by_id(&html, "usdc-in-circulation")?;
    let issued_7d = data_point_by_id(&html, "usdc-issued-7")?;
    let redeemed_7d = data_point_by_id(&html, "usdc-redeemed-7")?;
    let issued_30d = data_point_by_id(&html, "usdc-issued-30")?;
    let redeemed_30d = data_point_by_id(&html, "usdc-redeemed-30")?;
    let issued_365d = data_point_by_id(&html, "usdc-issued-365")?;
    let redeemed_365d = data_point_by_id(&html, "usdc-redeemed-365")?;

    let canvas_selector = Selector::parse("#usdc_chartjs_canvas").unwrap();
    let canvas = html
        .select(&canvas_selector)
        .next()
        .context("Circle Transparency USDC reserves chart missing")?;
    let other_bank_deposits = data_attr_billions(canvas, "data-usdc-in-circulation")?;
    let sii_deposits = data_attr_billions(canvas, "data-usdc-cash")?;
    let overnight_reverse_repo = data_attr_billions(canvas, "data-usdc-us-treasuries")?;
    let short_treasuries = data_attr_billions(canvas, "data-usdc-months")?;
    let total_reserves =
        other_bank_deposits + sii_deposits + overnight_reverse_repo + short_treasuries;

    let mut out = vec![
        obs!(
            "P0_USDC_CIRCULATING_SUPPLY",
            "USDC in circulation",
            "P0",
            "stablecoin_supply",
            Some(usd_from_billions(circulation)),
            None,
            "USD",
            &observed_at,
            "Circle Transparency",
            source_url,
            json!({"raw_value_billions": circulation, "source_field": "#usdc-in-circulation[data-point]"}),
        ),
        obs!(
            "P0_CIRCLE_USDC_TOTAL_RESERVES",
            "Circle USDC total reserves",
            "P0",
            "reserve_composition",
            Some(usd_from_billions(total_reserves)),
            None,
            "USD",
            &observed_at,
            "Circle Transparency",
            source_url,
            json!({"raw_value_billions": total_reserves, "components": "other_bank_deposits + sii_deposits + overnight_reverse_repo + short_treasuries"}),
        ),
        obs!(
            "P0_CIRCLE_USDC_RESERVE_OTHER_BANK_DEPOSITS",
            "USDC reserve other bank deposits",
            "P0",
            "reserve_composition",
            Some(usd_from_billions(other_bank_deposits)),
            None,
            "USD",
            &observed_at,
            "Circle Transparency",
            source_url,
            json!({"raw_value_billions": other_bank_deposits, "legend": "Other Bank Deposits"}),
        ),
        obs!(
            "P0_CIRCLE_USDC_RESERVE_SII_DEPOSITS",
            "USDC reserve deposits at systemically important institutions",
            "P0",
            "reserve_composition",
            Some(usd_from_billions(sii_deposits)),
            None,
            "USD",
            &observed_at,
            "Circle Transparency",
            source_url,
            json!({"raw_value_billions": sii_deposits, "legend": "Deposits at Systemically Important Institutions"}),
        ),
        obs!(
            "P0_CIRCLE_USDC_RESERVE_OVERNIGHT_REPO",
            "USDC reserve overnight reverse Treasury repo",
            "P0",
            "reserve_composition",
            Some(usd_from_billions(overnight_reverse_repo)),
            None,
            "USD",
            &observed_at,
            "Circle Transparency",
            source_url,
            json!({"raw_value_billions": overnight_reverse_repo, "legend": "Overnight Reverse Treasury Repo"}),
        ),
        obs!(
            "P0_CIRCLE_USDC_RESERVE_SHORT_TREASURIES",
            "USDC reserve <3-month Treasuries",
            "P0",
            "reserve_composition",
            Some(usd_from_billions(short_treasuries)),
            None,
            "USD",
            &observed_at,
            "Circle Transparency",
            source_url,
            json!({"raw_value_billions": short_treasuries, "legend": "<3-Month Treasuries"}),
        ),
    ];

    push_usdc_flow_observations(
        &mut out,
        source_url,
        &observed_at,
        "7D",
        issued_7d,
        redeemed_7d,
    );
    push_usdc_flow_observations(
        &mut out,
        source_url,
        &observed_at,
        "30D",
        issued_30d,
        redeemed_30d,
    );
    push_usdc_flow_observations(
        &mut out,
        source_url,
        &observed_at,
        "365D",
        issued_365d,
        redeemed_365d,
    );
    push_usdc_change_pct(
        &mut out,
        source_url,
        &observed_at,
        "7D",
        circulation,
        issued_7d - redeemed_7d,
    );
    push_usdc_change_pct(
        &mut out,
        source_url,
        &observed_at,
        "30D",
        circulation,
        issued_30d - redeemed_30d,
    );

    Ok(out)
}

fn data_point_by_id(html: &Html, id: &str) -> Result<f64> {
    let selector = Selector::parse(&format!("#{id}")).unwrap();
    let node = html
        .select(&selector)
        .next()
        .with_context(|| format!("Circle Transparency #{id} missing"))?;
    data_attr_billions(node, "data-point")
}

fn data_attr_billions(node: scraper::ElementRef<'_>, attr: &str) -> Result<f64> {
    node.value()
        .attr(attr)
        .with_context(|| format!("Circle Transparency {attr} missing"))?
        .parse::<f64>()
        .with_context(|| format!("Circle Transparency {attr} is not numeric"))
}

fn usd_from_billions(value: f64) -> f64 {
    value * 1_000_000_000.0
}

fn parse_circle_as_of_date(text: &str) -> Option<String> {
    let marker = "As of ";
    let start = text.find(marker)? + marker.len();
    let raw = text[start..]
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == ',' || *ch == ' ')
        .collect::<String>();
    NaiveDate::parse_from_str(raw.trim(), "%b %d, %Y")
        .or_else(|_| NaiveDate::parse_from_str(raw.trim(), "%b %e, %Y"))
        .ok()
        .map(|date| date.to_string())
}

fn push_usdc_flow_observations(
    out: &mut Vec<Observation>,
    source_url: &str,
    observed_at: &str,
    window: &str,
    issued: f64,
    redeemed: f64,
) {
    let window_lower = window.to_ascii_lowercase();
    for (suffix, name, value) in [
        ("ISSUED", "issued", issued),
        ("REDEEMED", "redeemed", redeemed),
        ("NET_ISSUANCE", "net issuance", issued - redeemed),
    ] {
        out.push(obs!(
            &format!("P0_CIRCLE_USDC_{window}_{suffix}"),
            &format!("USDC {window} {name}"),
            "P0",
            "mint_redeem_flow",
            Some(usd_from_billions(value)),
            None,
            "USD",
            observed_at,
            "Circle Transparency",
            source_url,
            json!({"raw_value_billions": value, "window": window_lower}),
        ));
    }
}

fn push_usdc_change_pct(
    out: &mut Vec<Observation>,
    source_url: &str,
    observed_at: &str,
    window: &str,
    current_circulation_billions: f64,
    net_change_billions: f64,
) {
    let previous = current_circulation_billions - net_change_billions;
    if previous == 0.0 {
        return;
    }
    out.push(obs!(
        &format!("P0_USDC_{window}_CHANGE_PCT"),
        &format!("USDC {window} change"),
        "P0",
        "stablecoin_supply",
        Some(net_change_billions / previous * 100.0),
        None,
        "percent",
        observed_at,
        "Circle Transparency",
        source_url,
        json!({"current_circulation_billions": current_circulation_billions, "net_change_billions": net_change_billions}),
    ));
}

pub fn page_title(text: &str) -> Option<String> {
    let html = Html::parse_document(text);
    let selector = Selector::parse("title").ok()?;
    html.select(&selector)
        .next()
        .map(|node| normalize_text(&node.text().collect::<Vec<_>>().join(" ")))
}

fn normalize_text(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_defillama_supply_and_share() {
        let json = r#"{
          "peggedAssets": [
            {"id":"1","name":"Tether","symbol":"USDT","pegType":"peggedUSD","circulating":{"peggedUSD":200.0}},
            {"id":"2","name":"USD Coin","symbol":"USDC","gecko_id":"usd-coin","pegType":"peggedUSD","price":1.0,
             "circulating":{"peggedUSD":100.0},"circulatingPrevWeek":{"peggedUSD":80.0},
             "circulatingPrevMonth":{"peggedUSD":50.0},
             "chainCirculating":{"Ethereum":{"current":{"peggedUSD":60.0}}}}
          ]
        }"#;
        let observations =
            parse_defillama_stablecoins(json, "https://example.test", "2026-06-15").unwrap();
        let share = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_USDC_MARKET_CAP_SHARE")
            .unwrap();
        assert_eq!(share.value_num.unwrap(), 100.0 / 300.0 * 100.0);
        assert!(
            observations
                .iter()
                .any(|obs| obs.metric_code == "P1_USDC_CHAIN_ETHEREUM")
        );
    }

    #[test]
    fn parses_defillama_protocol_usdc_deposits() {
        let json = r#"{
          "chainTvls": {
            "Ethereum": {
              "tokensInUsd": [
                {"date": 1781395200, "tokens": {"USDC": 90.0}},
                {"date": 1781481600, "tokens": {"USDC": 100.0, "USDC.E": 5.0, "SYRUPUSDC": 999.0}}
              ]
            },
            "Base": {
              "tokensInUsd": [
                {"date": 1781481600, "tokens": {"USDBC": 10.0, "AVALANCHEUSDC": 2.0, "USDT": 50.0}}
              ]
            },
            "Ethereum-borrowed": {
              "tokensInUsd": [
                {"date": 1781481600, "tokens": {"USDC": 1000.0}}
              ]
            }
          }
        }"#;

        let observations = parse_defillama_protocol_usdc_deposits(
            json,
            "https://example.test/protocol",
            "Aave V3",
            "P1_DEFI_AAVE_V3_USDC_DEPOSITS",
        )
        .unwrap();
        let total = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_DEFI_AAVE_V3_USDC_DEPOSITS")
            .unwrap();
        assert_eq!(total.value_num.unwrap(), 117.0);
        assert_eq!(total.observed_at, "2026-06-15");
        assert!(observations.iter().any(|obs| {
            obs.metric_code == "P1_DEFI_AAVE_V3_USDC_DEPOSITS_ETHEREUM"
                && obs.value_num == Some(105.0)
        }));
    }

    #[test]
    fn parses_rwa_treasuries_aum() {
        let json = r#"{
          "pageProps": {
            "listQueryResponse": {
              "results": [
                {
                  "id": 1,
                  "name": "Visible Treasury Fund",
                  "ticker": "VTBILL",
                  "asset_class_name": "US Treasury Debt",
                  "hidden": false,
                  "_updated_at": "2026-06-13T12:00:00",
                  "manager": {"name": "Visible Manager"},
                  "total_asset_value_dollar": {"val": 100.0}
                },
                {
                  "id": 2,
                  "name": "BlackRock USD Institutional Digital Liquidity Fund",
                  "ticker": "BUIDL",
                  "asset_class_name": "US Treasury Debt",
                  "hidden": false,
                  "_updated_at": "2026-06-14T22:04:51",
                  "manager": {"name": "BlackRock"},
                  "total_asset_value_dollar": {"val": 200.0}
                },
                {
                  "id": 2,
                  "name": "Duplicate BUIDL",
                  "ticker": "BUIDL",
                  "asset_class_name": "US Treasury Debt",
                  "hidden": false,
                  "_updated_at": "2026-06-14T22:04:51",
                  "total_asset_value_dollar": {"val": 200.0}
                },
                {
                  "id": 3,
                  "name": "Hidden Treasury Fund",
                  "ticker": "HIDDEN",
                  "asset_class_name": "US Treasury Debt",
                  "hidden": true,
                  "total_asset_value_dollar": {"val": 999.0}
                },
                {
                  "id": 4,
                  "name": "Private Credit Fund",
                  "ticker": "CREDIT",
                  "asset_class_name": "Private Credit",
                  "hidden": false,
                  "total_asset_value_dollar": {"val": 999.0}
                }
              ]
            }
          }
        }"#;

        let observations = parse_rwa_treasuries(json, "https://example.test/treasuries").unwrap();
        let total = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_TOKENIZED_TREASURY_AUM")
            .unwrap();
        assert_eq!(total.value_num.unwrap(), 300.0);
        assert_eq!(total.observed_at, "2026-06-14");
        assert_eq!(total.attributes["asset_count"], 2);

        let buidl = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_TOKENIZED_TREASURY_BUIDL_AUM")
            .unwrap();
        assert_eq!(buidl.value_num.unwrap(), 200.0);
    }

    #[test]
    fn parses_treasury_latest_row() {
        let xml = r#"
        <feed xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices">
          <entry><content><m:properties xmlns:m="x"><d:NEW_DATE>2026-06-13T00:00:00</d:NEW_DATE><d:BC_3MONTH>3.7</d:BC_3MONTH><d:BC_1YEAR>3.8</d:BC_1YEAR><d:BC_5YEAR>4.1</d:BC_5YEAR></m:properties></content></entry>
          <entry><content><m:properties xmlns:m="x"><d:NEW_DATE>2026-06-14T00:00:00</d:NEW_DATE><d:BC_3MONTH>3.6</d:BC_3MONTH><d:BC_1YEAR>3.9</d:BC_1YEAR><d:BC_5YEAR>4.2</d:BC_5YEAR></m:properties></content></entry>
        </feed>"#;
        let observations = parse_treasury_yield_curve(xml, "https://example.test").unwrap();
        let three_month = observations
            .iter()
            .find(|obs| obs.metric_code == "P0_TREASURY_3M_YIELD")
            .unwrap();
        assert_eq!(three_month.value_num.unwrap(), 3.6);
        assert_eq!(three_month.observed_at, "2026-06-14T00:00:00");
    }

    #[test]
    fn parses_nyfed_sofr() {
        let json = r#"{ "refRates": [ { "effectiveDate": "2026-06-11", "type": "SOFR", "percentRate": 3.60, "volumeInBillions": 3061 } ] }"#;
        let observations = parse_nyfed_sofr(json, "https://example.test").unwrap();
        assert_eq!(observations[0].metric_code, "P0_SOFR");
        assert_eq!(observations[0].value_num.unwrap(), 3.6);
    }

    #[test]
    fn parses_ethereum_latest_block_status() {
        let json = r#"{
          "jsonrpc": "2.0",
          "result": {
            "number": "0x1825ff6",
            "timestamp": "0x6a2fac2b",
            "hash": "0x30e0f2ff4d6e90f76fa9f45b91dba73deea652966b8f696e0f449ca8c423c96f"
          },
          "id": 1
        }"#;
        let now = chrono::DateTime::from_timestamp(1781509180, 0).unwrap();
        let observations =
            parse_ethereum_latest_block(json, "https://example.test/rpc", now).unwrap();
        let status = observations
            .iter()
            .find(|obs| obs.metric_code == "P0_CHAIN_STATUS_ETHEREUM")
            .unwrap();
        assert_eq!(status.value_text.as_deref(), Some("Operational"));
        assert_eq!(status.attributes["block_number"], 25321462);
        assert_eq!(status.attributes["block_age_seconds"], 17);
    }

    #[test]
    fn parses_finra_short_interest_latest_row() {
        let json = r#"[
          {
            "stockSplitFlag": null,
            "previousShortPositionQuantity": 3723912,
            "averageDailyVolumeQuantity": 57576824,
            "issueName": "Circle Internet Group, Inc.",
            "currentShortPositionQuantity": 5446011,
            "changePreviousNumber": 1722099,
            "accountingYearMonthNumber": 20250630,
            "settlementDate": "2025-06-30",
            "marketClassCode": "NYSE",
            "symbolCode": "CRCL",
            "daysToCoverQuantity": 1.0,
            "issuerServicesGroupExchangeCode": "A",
            "revisionFlag": null,
            "changePercent": 46.24
          },
          {
            "stockSplitFlag": null,
            "previousShortPositionQuantity": 18087364,
            "averageDailyVolumeQuantity": 12408357,
            "issueName": "Circle Internet Group, Inc.",
            "currentShortPositionQuantity": 15348729,
            "changePreviousNumber": -2738635,
            "accountingYearMonthNumber": 20260529,
            "settlementDate": "2026-05-29",
            "marketClassCode": "NYSE",
            "symbolCode": "CRCL",
            "daysToCoverQuantity": 1.24,
            "issuerServicesGroupExchangeCode": "A",
            "revisionFlag": null,
            "changePercent": -15.14
          }
        ]"#;

        let observations = parse_finra_short_interest(json, "https://example.test/finra").unwrap();
        let short_interest = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_FINRA_SHORT_INTEREST")
            .unwrap();
        assert_eq!(short_interest.observed_at, "2026-05-29");
        assert_eq!(short_interest.value_num.unwrap(), 15_348_729.0);

        let days_to_cover = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_CRCL_DAYS_TO_COVER")
            .unwrap();
        assert_eq!(days_to_cover.value_num.unwrap(), 1.24);
    }

    #[test]
    fn parses_visa_allium_usdc_adjusted_transfer_volume() {
        let json = r#"[
          {
            "day": "2026-06-14",
            "usd_amount": 35279409934.484604,
            "txn_count": 1724955,
            "usd_amount_30d": 996640778630.8209,
            "txn_count_30d": 60647255
          }
        ]"#;

        let observations =
            parse_visa_allium_usdc_adjusted_transfer_volume(json, "https://example.test/allium")
                .unwrap();
        let volume = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_USDC_ADJUSTED_TRANSFER_VOLUME")
            .unwrap();
        assert_eq!(volume.observed_at, "2026-06-14");
        assert!((volume.value_num.unwrap() - 35_279_409_934.484604).abs() < 0.001);
        assert_eq!(
            volume.attributes["txn_count"].as_f64().unwrap(),
            1_724_955.0
        );
        assert!(
            (volume.attributes["usd_amount_30d"].as_f64().unwrap() - 996_640_778_630.8209).abs()
                < 0.001
        );
    }

    #[test]
    fn parses_coinglass_exchange_balance_list() {
        let json = r#"{
          "code": "0",
          "data": [
            {"exchange": "Binance", "balance": 1000.5, "changePercent24h": 1.2},
            {"exchangeName": "Coinbase", "totalBalance": "2000.25", "changePercent7d": -0.5}
          ]
        }"#;

        let observations =
            parse_coinglass_exchange_balance(json, "https://example.test/coinglass").unwrap();
        let balance = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_EXCHANGE_USDC_BALANCES")
            .unwrap();
        assert_eq!(balance.value_num.unwrap(), 3000.75);
        assert_eq!(balance.source, "CoinGlass Exchange Balance List");
        assert_eq!(balance.attributes["component_count"], 2);
    }

    #[test]
    fn coinglass_exchange_balance_error_mentions_api_key() {
        let json = r#"{"code":"401","msg":"API key missing."}"#;
        let error =
            parse_coinglass_exchange_balance(json, "https://example.test/coinglass").unwrap_err();
        assert!(error.to_string().contains("COINGLASS_API_KEY"));
    }

    #[test]
    fn parses_circle_platform_metrics_cpn_tpv() {
        let html = r#"
        <html><body>
          <p>NEW YORK – MAY 11, 2026 — Circle announced results.</p>
          <li><strong>Continued CPN Expansion: </strong>Circle continues to grow the CPN network,
          with $8.3 billion in annualized transaction volume based on the trailing 30 day activity
          as of March 31, 2026.</li>
        </body></html>
        "#;

        let observations =
            parse_circle_platform_metrics(html, "https://example.test/circle-q1").unwrap();
        let tpv = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_CPN_ANNUALIZED_TPV")
            .unwrap();
        assert_eq!(tpv.observed_at, "2026-03-31");
        assert!((tpv.value_num.unwrap() - 8_300_000_000.0).abs() < 0.01);
    }

    #[test]
    fn parses_circle_platform_metrics_arc_fallback_metrics() {
        let html = r#"
        <html><body>
          <nav>Platform Arc The Economic OS for the internet Payments Circle Payments Network</nav>
          <p>NEW YORK - February 25, 2026 - Circle announced results.</p>
          <p>Arc: Public testnet launched with 100+ participants spanning banking, capital markets,
          digital assets, payments, and technology. Testnet is performing strongly with near 100%
          uptime, half second transaction finality, and daily average transaction volumes of 2.3 million
          based on the trailing 30 days as of February 20, 2026. Total transactions have exceeded
          166 million since testnet launch. Arc remains on track for mainnet launch this year.</p>
        </body></html>
        "#;

        let observations =
            parse_circle_platform_metrics(html, "https://example.test/circle-q4").unwrap();
        let status = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_ARC_MAINNET_STATUS")
            .unwrap();
        assert_eq!(status.observed_at, "2026-02-20");
        let daily_txs = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_ARC_TESTNET_DAILY_AVG_TRANSACTIONS")
            .unwrap();
        assert_eq!(daily_txs.value_num.unwrap(), 2_300_000.0);
        let total_txs = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_ARC_TESTNET_TOTAL_TRANSACTIONS")
            .unwrap();
        assert_eq!(total_txs.value_num.unwrap(), 166_000_000.0);
    }

    #[test]
    fn parses_arc_public_status_metrics() {
        let html = r#"
        <html><body>
          <h2>Live on public testnet</h2>
          <div>Avg. Weekly transaction cost $ 0.005</div>
          <div>Weekly contracts deployed 1385031</div>
          <div>Weekly accounts created 53917</div>
          <div>Weekly transactions 21938428</div>
        </body></html>
        "#;

        let observations = parse_arc_public_status(html, "https://example.test/arc").unwrap();
        let status = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_ARC_PUBLIC_NETWORK_STATUS")
            .unwrap();
        assert_eq!(status.value_text.as_deref(), Some("public testnet"));
        let weekly_txs = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_ARC_WEEKLY_TRANSACTIONS")
            .unwrap();
        assert_eq!(weekly_txs.value_num.unwrap(), 21_938_428.0);
    }

    #[test]
    fn parses_marketbeat_institutional_ownership_summary() {
        let html = r#"
        <html><head><meta name="revised" content="2026-06-16"></head><body>
          <div class="stat-summary-wrapper">
            <dt>Number of Institutional Buyers (last 12 months)</dt><dd>403</dd>
          </div>
          <div class="stat-summary-wrapper">
            <dt>Total Institutional Inflows (last 12 months)</dt><dd>$11.27B</dd>
          </div>
          <div class="stat-summary-wrapper">
            <dt>Number of Institutional Sellers (last 12 months)</dt><dd>70</dd>
          </div>
          <div class="stat-summary-wrapper">
            <dt>Total Institutional Outflows (last 12 months)</dt><dd>$798.74M</dd>
          </div>
          <p id="answer1">During the previous two years, 424 institutional investors and hedge funds held shares.</p>
          <p id="answer3">Institutional investors have bought a total of 89,355,801 shares in the last 24 months.</p>
          <p id="answer5">Institutional investors have sold a total of 7,523,174 shares in the last 24 months.</p>
        </body></html>
        "#;

        let observations =
            parse_marketbeat_institutional_ownership(html, "https://example.test/marketbeat")
                .unwrap();
        let inflow = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_CRCL_INSTITUTIONAL_INFLOW_12M")
            .unwrap();
        assert_eq!(inflow.observed_at, "2026-06-16");
        assert_eq!(inflow.value_num.unwrap(), 11_270_000_000.0);
        let shares_sold = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_CRCL_INSTITUTIONAL_SHARES_SOLD_24M")
            .unwrap();
        assert_eq!(shares_sold.value_num.unwrap(), 7_523_174.0);
    }

    #[test]
    fn parses_circle_transparency_balances_and_flows() {
        let html = r#"
        <p>As of Jun 11, 2026</p>
        <span data-coin="usdc" data-point="74.79" id="usdc-in-circulation"></span>
        <canvas id="usdc_chartjs_canvas"
          data-usdc-in-circulation="0.73"
          data-usdc-cash="10.97"
          data-usdc-us-treasuries="44.69"
          data-usdc-months="18.6"></canvas>
        <span id="usdc-issued-7" data-point="8.49"></span>
        <span id="usdc-redeemed-7" data-point="9.23"></span>
        <span id="usdc-issued-30" data-point="33.48"></span>
        <span id="usdc-redeemed-30" data-point="35.69"></span>
        <span id="usdc-issued-365" data-point="309.1"></span>
        <span id="usdc-redeemed-365" data-point="295.39"></span>
        "#;

        let observations = parse_circle_transparency(html, "https://example.test").unwrap();
        let supply = observations
            .iter()
            .find(|obs| obs.metric_code == "P0_USDC_CIRCULATING_SUPPLY")
            .unwrap();
        assert_eq!(supply.observed_at, "2026-06-11");
        assert_eq!(supply.value_num.unwrap(), 74_790_000_000.0);

        let reserves = observations
            .iter()
            .find(|obs| obs.metric_code == "P0_CIRCLE_USDC_TOTAL_RESERVES")
            .unwrap();
        assert!((reserves.value_num.unwrap() - 74_990_000_000.0).abs() < 0.01);

        let net = observations
            .iter()
            .find(|obs| obs.metric_code == "P0_CIRCLE_USDC_7D_NET_ISSUANCE")
            .unwrap();
        assert!((net.value_num.unwrap() + 740_000_000.0).abs() < 0.01);
    }

    #[test]
    fn parses_curve_3pool_usdc_ratio() {
        let json = r#"{
          "success": true,
          "data": {
            "poolData": [{
              "address": "0xbEbc44782C7dB0a1A60Cb6fe97d0b483032FF1C7",
              "name": "Curve.fi DAI/USDC/USDT",
              "symbol": "3Crv",
              "usdTotal": 60.0,
              "coins": [
                {"symbol": "DAI", "poolBalance": "10000000000000000000", "decimals": "18", "usdPrice": 1.0},
                {"symbol": "USDC", "poolBalance": "20000000", "decimals": "6", "usdPrice": 1.0},
                {"symbol": "USDT", "poolBalance": "30000000", "decimals": "6", "usdPrice": 1.0}
              ]
            }]
          }
        }"#;

        let observations = parse_curve_3pool(json, "https://example.test").unwrap();
        let ratio = observations
            .iter()
            .find(|obs| obs.metric_code == "P0_CURVE_3POOL_USDC_RATIO")
            .unwrap();
        assert!((ratio.value_num.unwrap() - 100.0 / 3.0).abs() < 0.0001);

        let balance = observations
            .iter()
            .find(|obs| obs.metric_code == "P0_CURVE_3POOL_USDC_BALANCE")
            .unwrap();
        assert_eq!(balance.value_num.unwrap(), 20.0);
    }

    #[test]
    fn parses_circle_sec_filing_financial_metrics() {
        let html = r#"
        <html xmlns:ix="http://www.xbrl.org/2013/inlineXBRL"
              xmlns:xbrli="http://www.xbrl.org/2003/instance"
              xmlns:xbrldi="http://xbrl.org/2006/xbrldi">
          <body>
            <xbrli:context id="c-1"><xbrli:entity/><xbrli:period><xbrli:startDate>2026-01-01</xbrli:startDate><xbrli:endDate>2026-03-31</xbrli:endDate></xbrli:period></xbrli:context>
            <xbrli:context id="c-2"><xbrli:entity/><xbrli:period><xbrli:instant>2026-03-31</xbrli:instant></xbrli:period></xbrli:context>
            <ix:nonFraction unitRef="usd" contextRef="c-1" name="us-gaap:InterestAndDividendIncomeOperating" scale="3">652,508</ix:nonFraction>
            <ix:nonFraction unitRef="usd" contextRef="c-1" name="us-gaap:RevenueFromContractWithCustomerExcludingAssessedTax" scale="3">41,625</ix:nonFraction>
            <ix:nonFraction unitRef="usd" contextRef="c-1" name="us-gaap:Revenues" scale="3">694,133</ix:nonFraction>
            <ix:nonFraction unitRef="usd" contextRef="c-1" name="crcl:DistributionAndTransactionCosts" scale="3">405,402</ix:nonFraction>
            <ix:nonFraction unitRef="usd" contextRef="c-1" name="crcl:DistributionTransactionAndOtherCosts" scale="3">406,781</ix:nonFraction>
            <ix:nonFraction unitRef="usd" contextRef="c-1" name="us-gaap:NetIncomeLoss" scale="3">55,253</ix:nonFraction>
            <ix:nonFraction unitRef="shares" contextRef="c-1" name="us-gaap:WeightedAverageNumberOfSharesOutstandingBasic">202,000,000</ix:nonFraction>
            <ix:nonFraction unitRef="shares" contextRef="c-1" name="us-gaap:WeightedAverageNumberOfDilutedSharesOutstanding">221,000,000</ix:nonFraction>
            <ix:nonFraction unitRef="usd" contextRef="c-2" name="crcl:CashAndCashEquivalentsSegregatedForTheBenefitOfStablecoinHolders" scale="3">76,893,681</ix:nonFraction>
            <ix:nonFraction unitRef="shares" contextRef="c-2" name="dei:EntityCommonStockSharesOutstanding">210,000,000</ix:nonFraction>
            <p>USDC in circulation, end of period (1) $ 77,049 $ 59,976</p>
            <p>USDC in circulation, average of period (1) $ 75,200 $ 54,075</p>
            <p>USDC onchain transaction volume grew 263% to $21.5 trillion.</p>
            <p>The following table reconciles Adjusted EBITDA to net income from continuing operations.</p>
            <p>Foreign currency exchange (gains) losses (5,121) 539 Adjusted EBITDA $ 151,401 $ 122,439 Reflects interest income</p>
          </body>
        </html>
        "#;

        let observations =
            parse_circle_sec_filing(html, "https://example.test/crcl.htm", "2026-03-31").unwrap();
        let reserve_income = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_CIRCLE_RESERVE_INCOME")
            .unwrap();
        assert_eq!(reserve_income.value_num.unwrap(), 652_508_000.0);

        let rldc = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_CIRCLE_RLDC")
            .unwrap();
        assert_eq!(rldc.value_num.unwrap(), 287_352_000.0);

        let other_revenue_share = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_CIRCLE_OTHER_REVENUE_SHARE")
            .unwrap();
        assert!(
            (other_revenue_share.value_num.unwrap() - (41_625_000.0 / 694_133_000.0 * 100.0)).abs()
                < 0.0001
        );

        let adjusted_ebitda = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_CIRCLE_ADJUSTED_EBITDA")
            .unwrap();
        assert_eq!(adjusted_ebitda.value_num.unwrap(), 151_401_000.0);

        let diluted_shares = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_CRCL_DILUTED_SHARES_OUTSTANDING")
            .unwrap();
        assert_eq!(diluted_shares.value_num.unwrap(), 221_000_000.0);

        let shares_outstanding = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_CRCL_SHARES_OUTSTANDING")
            .unwrap();
        assert_eq!(shares_outstanding.value_num.unwrap(), 210_000_000.0);

        let onchain_volume = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_USDC_ONCHAIN_TRANSACTION_VOLUME")
            .unwrap();
        assert_eq!(onchain_volume.value_num.unwrap(), 21_500_000_000_000.0);

        let velocity = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_USDC_VELOCITY")
            .unwrap();
        assert!(
            (velocity.value_num.unwrap() - (21_500_000_000_000.0 / 75_200_000_000.0)).abs()
                < 0.0001
        );
    }

    #[test]
    fn parses_blackrock_nmfp3_reserve_fund_metrics() {
        let html = r#"
        <html><body>
          <table>
            <tr><td class="label"><b>Item 1.</b> Report for [YYYY-MM-DD]</td><td><div>2026-05-31</div></td></tr>
            <tr><td class="label"><b>Item 5.</b> Name of Series</td><td><div>Circle Reserve Fund</div></td></tr>
            <tr><td class="label"><b>Item 7.</b> EDGAR Series Identifier</td><td><div>S000077205</div></td></tr>
            <tr><td class="label"><b>Item A.11.</b> Dollar-weighted average portfolio maturity</td><td><div>12</div></td></tr>
            <tr><td class="label"><b>Item A.12.</b> Dollar-weighted average life maturity</td><td><div>13</div></td></tr>
            <tr><td class="label"><b>Item A.19.</b> 7-day gross yield.</td><td></td></tr>
            <tr><td class="label">a. 7-day gross yield</td><td><div>3.60%</div></td></tr>
            <tr><td class="label">b. Date</td><td><div>2026-05-28</div></td></tr>
            <tr><td class="label">a. 7-day gross yield</td><td><div>3.61%</div></td></tr>
            <tr><td class="label">b. Date</td><td><div>2026-05-29</div></td></tr>
            <tr><td class="label"><b>Item B.1.</b> Full name of the Class</td><td><div>Institutional Shares</div></td></tr>
            <tr><td class="label"><b>Item B.2.</b> EDGAR Class identifier</td><td><div>C000237396</div></td></tr>
            <tr><td class="label"><b>Item B.4.</b> Net assets of the Class, to the nearest cent</td><td><div>$65,167,217,581.55</div></td></tr>
            <tr><td class="label"><b>Item B.8.</b> 7-day net yield</td><td></td></tr>
            <tr><td class="label">a. 7-day net yield</td><td><div>3.51%</div></td></tr>
            <tr><td class="label">b. Date</td><td><div>2026-05-28</div></td></tr>
            <tr><td class="label">a. 7-day net yield</td><td><div>3.52%</div></td></tr>
            <tr><td class="label">b. Date</td><td><div>2026-05-29</div></td></tr>
          </table>
        </body></html>
        "#;

        let observations =
            parse_sec_blackrock_nmfp3_filing(html, "https://example.test/nmfp3.htm").unwrap();
        let net_yield = observations
            .iter()
            .find(|obs| obs.metric_code == "P0_CIRCLE_RESERVE_FUND_7D_YIELD")
            .unwrap();
        assert_eq!(net_yield.observed_at, "2026-05-29");
        assert_eq!(net_yield.value_num.unwrap(), 3.52);

        let wam = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_RESERVE_FUND_WAM")
            .unwrap();
        assert_eq!(wam.value_num.unwrap(), 12.0);

        let wal = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_RESERVE_FUND_WAL")
            .unwrap();
        assert_eq!(wal.value_num.unwrap(), 13.0);

        let net_assets = observations
            .iter()
            .find(|obs| obs.metric_code == "P2_RESERVE_FUND_NET_ASSETS")
            .unwrap();
        assert_eq!(net_assets.value_num.unwrap(), 65_167_217_581.55);
    }

    #[test]
    fn parses_coinbase_sec_filing_stablecoin_metrics() {
        let html = r#"
        <html xmlns:ix="http://www.xbrl.org/2013/inlineXBRL"
              xmlns:xbrli="http://www.xbrl.org/2003/instance"
              xmlns:xbrldi="http://xbrl.org/2006/xbrldi">
          <body>
            <xbrli:context id="c-1"><xbrli:entity><xbrli:segment><xbrldi:explicitMember dimension="srt:ProductOrServiceAxis">coin:SubscriptionAndCirculationStablecoinMember</xbrldi:explicitMember></xbrli:segment></xbrli:entity><xbrli:period><xbrli:startDate>2026-01-01</xbrli:startDate><xbrli:endDate>2026-03-31</xbrli:endDate></xbrli:period></xbrli:context>
            <xbrli:context id="c-2"><xbrli:entity/><xbrli:period><xbrli:instant>2026-03-31</xbrli:instant></xbrli:period></xbrli:context>
            <ix:nonFraction unitRef="usd" contextRef="c-1" name="us-gaap:RevenueFromContractWithCustomerExcludingAssessedTax" scale="3">305,435</ix:nonFraction>
            <ix:nonFraction unitRef="usd" contextRef="c-2" name="coin:AccountsReceivableBeforeAllowanceForCreditLossStablecoinRevenue" scale="3">125,223</ix:nonFraction>
            <ix:nonFraction unitRef="usd" contextRef="c-2" name="coin:FiatAndPaymentStablecoinFinancingReceivableExcludingAccruedInterestBeforeAllowanceForCreditLoss" scale="3">1,392,286</ix:nonFraction>
            <ix:nonFraction unitRef="usd" contextRef="c-2" name="coin:ClientCustodialFundsCurrent" scale="3">5,476,614</ix:nonFraction>
            <p>The following table sets forth the value of AOP by asset (in millions, except percentages):</p>
            <p>March 31, 2026 March 31, 2025 Value Change Units Value Units Value %</p>
            <p>Bitcoin 3.0 $ 202,335 2.7 $ 219,076 (8)</p>
            <p>USDC N/A 9,289 N/A 7,839 18</p>
            <p>Total $ 294,432 $ 327,510 (10)</p>
            <p>Trading Volume We define Trading Volume as the total U.S. dollar equivalent value</p>
          </body>
        </html>
        "#;

        let observations =
            parse_coinbase_sec_filing(html, "https://example.test/coin.htm", "2026-03-31").unwrap();
        let stablecoin_revenue = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_COINBASE_STABLECOIN_REVENUE")
            .unwrap();
        assert_eq!(stablecoin_revenue.value_num.unwrap(), 305_435_000.0);

        let custodial_funds = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_COINBASE_CLIENT_CUSTODIAL_FUNDS")
            .unwrap();
        assert_eq!(custodial_funds.value_num.unwrap(), 5_476_614_000.0);

        let usdc_on_platform = observations
            .iter()
            .find(|obs| obs.metric_code == "P1_COINBASE_USDC_ON_PLATFORM")
            .unwrap();
        assert_eq!(usdc_on_platform.value_num.unwrap(), 9_289_000_000.0);
        assert_eq!(
            usdc_on_platform.attributes["total_assets_on_platform"],
            294_432_000_000.0
        );
    }
}
