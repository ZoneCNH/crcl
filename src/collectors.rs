use crate::SourceSelector;
use crate::db::Database;
use crate::models::{Event, Filing, MissingItem, Observation, SourceRun};
use crate::parsing::{
    page_title, parse_arc_public_status, parse_circle_platform_metrics, parse_circle_pressroom,
    parse_circle_sec_filing, parse_circle_transparency, parse_coinbase_sec_filing,
    parse_coingecko_simple_price, parse_coinglass_exchange_balance_history_snapshot,
    parse_coinglass_exchange_balance_snapshot, parse_coinmetrics_usdc_activity, parse_curve_3pool,
    parse_defillama_protocol_usdc_deposits, parse_defillama_stablecoins,
    parse_ethereum_latest_block, parse_finra_short_interest,
    parse_marketbeat_institutional_ownership, parse_nyfed_sofr, parse_rwa_treasuries,
    parse_sec_blackrock_nmfp3_filing, parse_sec_submissions, parse_statuspage,
    parse_treasury_yield_curve, parse_visa_allium_usdc_adjusted_transfer_volume, parse_yahoo_chart,
};
use aes::Aes128;
use anyhow::{Context, Result, anyhow};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::{Datelike, Utc};
use cipher::{BlockDecryptMut, KeyInit, block_padding::Pkcs7};
use flate2::read::{DeflateDecoder, GzDecoder, ZlibDecoder};
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, REFERER, USER_AGENT};
use serde_json::json;
use std::env;
use std::io::Read;

type Aes128EcbDec = ecb::Decryptor<Aes128>;

const COINGLASS_OPEN_API_BALANCE_URL: &str =
    "https://open-api-v4.coinglass.com/api/exchange/balance/list?symbol=USDC";
const COINGLASS_FRONTEND_BALANCE_BASE_URL: &str =
    "https://capi.coinglass.com/api/exchange/chain/v3/balance/list?symbol=USDC";
const COINGLASS_FRONTEND_BALANCE_HISTORY_BASE_URL: &str = "https://capi.coinglass.com/api/exchange/chain/v3/balance?symbol=USDC&exName=all&size=300&resolution=7";
const COINGLASS_FRONTEND_REFERER: &str = "https://www.coinglass.com/zh/Balance";

pub struct CollectorContext<'a> {
    pub db: &'a Database,
    pub client: reqwest::Client,
    pub user_agent: String,
    pub batch_id: Option<String>,
    pub profile: Option<String>,
    pub selector: Option<String>,
}

#[derive(Debug, Default)]
pub struct CollectorReport {
    pub ok_sources: usize,
    pub warn_sources: usize,
}

pub async fn run_collectors(
    ctx: &CollectorContext<'_>,
    selector: SourceSelector,
) -> Result<CollectorReport> {
    let scoped_ctx = CollectorContext {
        db: ctx.db,
        client: ctx.client.clone(),
        user_agent: ctx.user_agent.clone(),
        batch_id: ctx.batch_id.clone(),
        profile: ctx.profile.clone(),
        selector: None,
    };

    record_known_gaps(scoped_ctx.db, selector)?;

    let mut report = CollectorReport::default();
    let mut jobs: Vec<CollectorJob> = Vec::new();

    if matches!(selector, SourceSelector::All | SourceSelector::Market) {
        jobs.push(CollectorJob::CircleTransparency);
        jobs.push(CollectorJob::DefiLlama);
        jobs.push(CollectorJob::DefiLlamaProtocolUsdc);
        jobs.push(CollectorJob::CoinGecko);
        jobs.push(CollectorJob::CoinMetrics);
        jobs.push(CollectorJob::VisaAlliumAdjustedTransfers);
        jobs.push(CollectorJob::Curve3Pool);
        jobs.push(CollectorJob::RwaTreasuries);
        jobs.push(CollectorJob::Yahoo);
        jobs.push(CollectorJob::FinraShortInterest);
        jobs.push(CollectorJob::CoinGlassExchangeBalance);
        jobs.push(CollectorJob::CirclePlatformMetrics);
        jobs.push(CollectorJob::ArcPublicStatus);
        jobs.push(CollectorJob::MarketBeatInstitutionalOwnership);
    }
    if matches!(selector, SourceSelector::All | SourceSelector::Rates) {
        jobs.push(CollectorJob::Treasury);
        jobs.push(CollectorJob::NyFed);
        jobs.push(CollectorJob::BlackRockNmfp);
    }
    if matches!(selector, SourceSelector::All | SourceSelector::Sec) {
        jobs.push(CollectorJob::SecCircle);
        jobs.push(CollectorJob::SecCoinbase);
    }
    if matches!(selector, SourceSelector::All | SourceSelector::Events) {
        jobs.push(CollectorJob::CirclePressroom);
        jobs.push(CollectorJob::RegulatoryPages);
    }
    if matches!(selector, SourceSelector::All | SourceSelector::Status) {
        jobs.push(CollectorJob::StatusPages);
        jobs.push(CollectorJob::EthereumStatus);
    }

    for job in jobs {
        let job_ctx = CollectorContext {
            db: scoped_ctx.db,
            client: scoped_ctx.client.clone(),
            user_agent: scoped_ctx.user_agent.clone(),
            batch_id: scoped_ctx.batch_id.clone(),
            profile: scoped_ctx.profile.clone(),
            selector: Some(source_selector_label(source_selector_for_job(job)).to_string()),
        };
        match run_job(&job_ctx, job).await {
            Ok(()) => {
                clear_job_failures(&job_ctx, job)?;
                report.ok_sources += 1;
            }
            Err(error) => {
                report.warn_sources += 1;
                record_job_failure(&job_ctx, job, &error)?;
                eprintln!("{job:?}: {error:#}");
            }
        }
    }

    Ok(report)
}

fn source_selector_for_job(job: CollectorJob) -> SourceSelector {
    match job {
        CollectorJob::CircleTransparency
        | CollectorJob::DefiLlama
        | CollectorJob::DefiLlamaProtocolUsdc
        | CollectorJob::CoinGecko
        | CollectorJob::CoinMetrics
        | CollectorJob::VisaAlliumAdjustedTransfers
        | CollectorJob::Curve3Pool
        | CollectorJob::RwaTreasuries
        | CollectorJob::Yahoo
        | CollectorJob::FinraShortInterest
        | CollectorJob::CoinGlassExchangeBalance
        | CollectorJob::CirclePlatformMetrics
        | CollectorJob::ArcPublicStatus
        | CollectorJob::MarketBeatInstitutionalOwnership => SourceSelector::Market,
        CollectorJob::Treasury | CollectorJob::NyFed | CollectorJob::BlackRockNmfp => {
            SourceSelector::Rates
        }
        CollectorJob::SecCircle | CollectorJob::SecCoinbase => SourceSelector::Sec,
        CollectorJob::CirclePressroom | CollectorJob::RegulatoryPages => SourceSelector::Events,
        CollectorJob::StatusPages | CollectorJob::EthereumStatus => SourceSelector::Status,
    }
}

pub fn source_selector_label(selector: SourceSelector) -> &'static str {
    match selector {
        SourceSelector::All => "all",
        SourceSelector::Market => "market",
        SourceSelector::Rates => "rates",
        SourceSelector::Sec => "sec",
        SourceSelector::Events => "events",
        SourceSelector::Status => "status",
    }
}

#[derive(Clone, Copy, Debug)]
enum CollectorJob {
    CircleTransparency,
    DefiLlama,
    DefiLlamaProtocolUsdc,
    CoinGecko,
    CoinMetrics,
    VisaAlliumAdjustedTransfers,
    Curve3Pool,
    RwaTreasuries,
    Yahoo,
    FinraShortInterest,
    CoinGlassExchangeBalance,
    CirclePlatformMetrics,
    ArcPublicStatus,
    MarketBeatInstitutionalOwnership,
    Treasury,
    NyFed,
    BlackRockNmfp,
    SecCircle,
    SecCoinbase,
    CirclePressroom,
    RegulatoryPages,
    StatusPages,
    EthereumStatus,
}

async fn run_job(ctx: &CollectorContext<'_>, job: CollectorJob) -> Result<()> {
    match job {
        CollectorJob::CircleTransparency => collect_circle_transparency(ctx).await,
        CollectorJob::DefiLlama => collect_defillama(ctx).await,
        CollectorJob::DefiLlamaProtocolUsdc => collect_defillama_protocol_usdc(ctx).await,
        CollectorJob::CoinGecko => collect_coingecko(ctx).await,
        CollectorJob::CoinMetrics => collect_coinmetrics(ctx).await,
        CollectorJob::VisaAlliumAdjustedTransfers => {
            collect_visa_allium_adjusted_transfers(ctx).await
        }
        CollectorJob::Curve3Pool => collect_curve_3pool(ctx).await,
        CollectorJob::RwaTreasuries => collect_rwa_treasuries(ctx).await,
        CollectorJob::Yahoo => collect_yahoo(ctx).await,
        CollectorJob::FinraShortInterest => collect_finra_short_interest(ctx).await,
        CollectorJob::CoinGlassExchangeBalance => collect_coinglass_exchange_balance(ctx).await,
        CollectorJob::CirclePlatformMetrics => collect_circle_platform_metrics(ctx).await,
        CollectorJob::ArcPublicStatus => collect_arc_public_status(ctx).await,
        CollectorJob::MarketBeatInstitutionalOwnership => {
            collect_marketbeat_institutional_ownership(ctx).await
        }
        CollectorJob::Treasury => collect_treasury(ctx).await,
        CollectorJob::NyFed => collect_nyfed(ctx).await,
        CollectorJob::BlackRockNmfp => collect_blackrock_nmfp(ctx).await,
        CollectorJob::SecCircle => {
            collect_sec_company(ctx, "Circle Internet Group", "0001876042").await
        }
        CollectorJob::SecCoinbase => {
            collect_sec_company(ctx, "Coinbase Global", "0001679788").await
        }
        CollectorJob::CirclePressroom => collect_circle_pressroom(ctx).await,
        CollectorJob::RegulatoryPages => collect_regulatory_pages(ctx).await,
        CollectorJob::StatusPages => collect_status_pages(ctx).await,
        CollectorJob::EthereumStatus => collect_ethereum_status(ctx).await,
    }
}

fn clear_job_failures(ctx: &CollectorContext<'_>, job: CollectorJob) -> Result<()> {
    for (metric_code, _, _, _) in job_failure_gaps(job) {
        ctx.db.delete_missing_metric(metric_code)?;
    }
    Ok(())
}

fn record_job_failure(
    ctx: &CollectorContext<'_>,
    job: CollectorJob,
    error: &anyhow::Error,
) -> Result<()> {
    for (metric_code, metric_name, priority, source_hint) in job_failure_gaps(job) {
        ctx.db.delete_missing_metric(metric_code)?;
        ctx.db.upsert_missing_item(&MissingItem {
            collector: format!("{job:?}"),
            metric_code: metric_code.to_string(),
            metric_name: metric_name.to_string(),
            priority: priority.to_string(),
            reason: format!("Latest collector run failed: {error:#}"),
            source_hint: source_hint.to_string(),
        })?;
    }

    Ok(())
}

fn job_failure_gaps(
    job: CollectorJob,
) -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    match job {
        CollectorJob::CircleTransparency => vec![(
            "P0_CIRCLE_MINTED_REDEEMED",
            "Minted / Redeemed / Net mint",
            "P0",
            "https://www.circle.com/transparency",
        )],
        CollectorJob::DefiLlama => vec![
            (
                "P0_USDC_CIRCULATING_SUPPLY",
                "USDC circulating supply",
                "P0",
                "https://stablecoins.llama.fi/stablecoins?includePrices=true",
            ),
            (
                "P1_USDC_MARKET_CAP_SHARE",
                "USDC market cap share",
                "P1",
                "https://stablecoins.llama.fi/stablecoins?includePrices=true",
            ),
        ],
        CollectorJob::DefiLlamaProtocolUsdc => vec![(
            "P1_DEFI_PROTOCOL_USDC_DEPOSITS",
            "Aave / Compound USDC deposits",
            "P1",
            "https://api.llama.fi/protocol/aave-v3 and https://api.llama.fi/protocol/compound-v3",
        )],
        CollectorJob::CoinGecko => vec![
            (
                "P0_USDC_USD_PRICE",
                "USDC/USD secondary-market price",
                "P0",
                "https://api.coingecko.com/api/v3/simple/price",
            ),
            (
                "P0_BTC_PRICE",
                "BTC price",
                "P0",
                "https://api.coingecko.com/api/v3/simple/price",
            ),
            (
                "P0_ETH_PRICE",
                "ETH price",
                "P0",
                "https://api.coingecko.com/api/v3/simple/price",
            ),
        ],
        CollectorJob::CoinMetrics => vec![(
            "P1_USDC_ACTIVE_ADDRESSES",
            "USDC active addresses",
            "P1",
            "https://community-api.coinmetrics.io",
        )],
        CollectorJob::VisaAlliumAdjustedTransfers => vec![(
            "P1_USDC_ADJUSTED_TRANSFER_VOLUME",
            "USDC adjusted transfer volume",
            "P1",
            "https://visaonchainanalytics.com/transactions",
        )],
        CollectorJob::Curve3Pool => vec![(
            "P0_CURVE_3POOL_USDC_RATIO",
            "Curve 3pool / USDC pool ratio",
            "P0",
            "https://api.curve.fi/api/getPools/ethereum/main",
        )],
        CollectorJob::RwaTreasuries => vec![(
            "P1_TOKENIZED_TREASURY_AUM",
            "Tokenized Treasury AUM",
            "P1",
            "https://app.rwa.xyz/treasuries",
        )],
        CollectorJob::Yahoo => vec![
            (
                "P0_CRCL_PRICE",
                "CRCL price",
                "P0",
                "https://query1.finance.yahoo.com/v8/finance/chart/CRCL",
            ),
            (
                "P0_CRCL_VOLUME",
                "CRCL volume",
                "P0",
                "https://query1.finance.yahoo.com/v8/finance/chart/CRCL",
            ),
        ],
        CollectorJob::FinraShortInterest => vec![(
            "P2_FINRA_SHORT_INTEREST",
            "FINRA short interest",
            "P2",
            "https://api.finra.org/data/group/otcMarket/name/consolidatedShortInterest",
        )],
        CollectorJob::CoinGlassExchangeBalance => vec![
            (
                "P1_EXCHANGE_USDC_BALANCES",
                "Exchange USDC balances",
                "P1",
                "https://capi.coinglass.com/api/exchange/chain/v3/balance/list?symbol=USDC",
            ),
            (
                "P1_EXCHANGE_USDC_BALANCE_30D_CHANGE",
                "Exchange USDC balance 30D change",
                "P1",
                "https://capi.coinglass.com/api/exchange/chain/v3/balance/list?symbol=USDC",
            ),
            (
                "P1_EXCHANGE_USDC_TOP3_CONCENTRATION",
                "Exchange USDC top 3 concentration",
                "P1",
                "https://capi.coinglass.com/api/exchange/chain/v3/balance/list?symbol=USDC",
            ),
        ],
        CollectorJob::CirclePlatformMetrics => vec![(
            "P1_CPN_ANNUALIZED_TPV",
            "Circle Payments Network annualized TPV",
            "P1",
            "https://www.circle.com/pressroom/circle-reports-first-quarter-2026-results",
        )],
        CollectorJob::ArcPublicStatus => vec![
            (
                "P1_ARC_PUBLIC_NETWORK_STATUS",
                "Arc public network status",
                "P1",
                "https://www.arc.io/",
            ),
            (
                "P1_ARC_MAINNET_STATUS",
                "Arc mainnet status",
                "P1",
                "https://www.circle.com/pressroom/circle-reports-fourth-quarter-and-full-fiscal-year-2025-financial-results",
            ),
        ],
        CollectorJob::MarketBeatInstitutionalOwnership => vec![(
            "P2_CRCL_INSTITUTIONAL_OWNERSHIP",
            "CRCL institutional ownership",
            "P2",
            "https://www.marketbeat.com/stocks/NYSE/CRCL/institutional-ownership/",
        )],
        CollectorJob::Treasury => vec![
            (
                "P0_TREASURY_3M_YIELD",
                "3M T-Bill yield",
                "P0",
                "https://home.treasury.gov/resource-center/data-chart-center/interest-rates",
            ),
            (
                "P0_TREASURY_1Y_YIELD",
                "1Y Treasury yield",
                "P0",
                "https://home.treasury.gov/resource-center/data-chart-center/interest-rates",
            ),
            (
                "P0_TREASURY_5Y_YIELD",
                "5Y Treasury yield",
                "P0",
                "https://home.treasury.gov/resource-center/data-chart-center/interest-rates",
            ),
        ],
        CollectorJob::NyFed => vec![(
            "P0_SOFR",
            "SOFR",
            "P0",
            "https://markets.newyorkfed.org/api/rates/secured/sofr/last/1.json",
        )],
        CollectorJob::BlackRockNmfp => vec![
            (
                "P0_CIRCLE_RESERVE_FUND_7D_YIELD",
                "Circle Reserve Fund 7-day yield",
                "P0",
                "https://data.sec.gov/submissions/CIK0000844779.json",
            ),
            (
                "P2_RESERVE_FUND_WAM_WAL",
                "Reserve Fund WAM / WAL",
                "P2",
                "https://data.sec.gov/submissions/CIK0000844779.json",
            ),
        ],
        CollectorJob::SecCircle => vec![
            (
                "P2_CIRCLE_FINANCIAL_LINE_ITEMS",
                "Reserve income / distribution costs / RLDC / Other revenue / Adjusted EBITDA",
                "P2",
                "https://data.sec.gov/submissions/CIK0001876042.json",
            ),
            (
                "P1_USDC_VELOCITY",
                "USDC velocity",
                "P1",
                "Circle SEC 10-Q / 10-K",
            ),
        ],
        CollectorJob::SecCoinbase => vec![(
            "P1_COINBASE_USDC_ON_PLATFORM",
            "Coinbase platform USDC balance / channel concentration",
            "P1",
            "https://data.sec.gov/submissions/CIK0001679788.json",
        )],
        CollectorJob::StatusPages => vec![
            (
                "P0_CHAIN_STATUS_BASE",
                "Base chain status",
                "P0",
                "https://status.base.org/api/v2/status.json",
            ),
            (
                "P0_CHAIN_STATUS_SOLANA",
                "Solana chain status",
                "P0",
                "https://status.solana.com/api/v2/status.json",
            ),
            (
                "P0_CIRCLE_STATUS",
                "Circle service status",
                "P0",
                "https://status.circle.com/api/v2/status.json",
            ),
        ],
        CollectorJob::EthereumStatus => vec![(
            "P0_CHAIN_STATUS_ETHEREUM",
            "Ethereum chain status",
            "P0",
            "https://ethereum.publicnode.com",
        )],
        CollectorJob::CirclePressroom | CollectorJob::RegulatoryPages => Vec::new(),
    }
}

async fn collect_circle_transparency(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://www.circle.com/transparency";
    let fetched = fetch_text(ctx, "Circle Transparency reserves", url).await?;
    let observations = parse_circle_transparency(&fetched.text, url)?;
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    ctx.db.delete_missing_metric("P0_CIRCLE_MINTED_REDEEMED")?;
    Ok(())
}

async fn collect_defillama(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://stablecoins.llama.fi/stablecoins?includePrices=true";
    let fetched = fetch_text(ctx, "DefiLlama stablecoins", url).await?;
    let observed_at = Utc::now().date_naive().to_string();
    let observations = parse_defillama_stablecoins(&fetched.text, url, &observed_at)?;
    let has_chain_distribution = observations
        .iter()
        .any(|obs| obs.metric_code.starts_with("P1_USDC_CHAIN_"));
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    if has_chain_distribution {
        ctx.db.delete_missing_metric("P1_USDC_BY_CHAIN_DUNE")?;
    }
    Ok(())
}

async fn collect_defillama_protocol_usdc(ctx: &CollectorContext<'_>) -> Result<()> {
    let protocols = [
        (
            "Aave V3",
            "P1_DEFI_AAVE_V3_USDC_DEPOSITS",
            "https://api.llama.fi/protocol/aave-v3",
        ),
        (
            "Compound V3",
            "P1_DEFI_COMPOUND_V3_USDC_DEPOSITS",
            "https://api.llama.fi/protocol/compound-v3",
        ),
    ];
    let mut aggregate_run_id = None;
    let mut aggregate_value = 0.0;
    let mut aggregate_observed_at = Utc::now().date_naive().to_string();
    let mut source_urls = Vec::new();

    for (protocol_name, metric_code, url) in protocols {
        let fetched = fetch_text(
            ctx,
            &format!("DefiLlama protocol TVL: {protocol_name}"),
            url,
        )
        .await?;
        let observations =
            parse_defillama_protocol_usdc_deposits(&fetched.text, url, protocol_name, metric_code)?;
        if let Some(total) = observations
            .iter()
            .find(|obs| obs.metric_code == metric_code)
        {
            if let Some(value) = total.value_num {
                aggregate_value += value;
            }
            aggregate_observed_at = total.observed_at.clone();
        }
        insert_observations(ctx.db, fetched.run_id, &observations)?;
        aggregate_run_id = Some(fetched.run_id);
        source_urls.push(url);
    }

    let run_id = aggregate_run_id.context("DefiLlama protocol USDC source run missing")?;
    let aggregate = Observation {
        metric_code: "P1_DEFI_PROTOCOL_USDC_DEPOSITS".to_string(),
        metric_name: "Aave V3 + Compound V3 USDC-equivalent deposits".to_string(),
        priority: "P1".to_string(),
        category: "defi_adoption".to_string(),
        value_num: Some(aggregate_value),
        value_text: None,
        unit: "USD".to_string(),
        observed_at: aggregate_observed_at,
        source: "DefiLlama protocol TVL".to_string(),
        source_url: source_urls.join("; "),
        attributes: json!({
            "method": "sum Aave V3 and Compound V3 USDC-equivalent deposits",
            "component_metrics": [
                "P1_DEFI_AAVE_V3_USDC_DEPOSITS",
                "P1_DEFI_COMPOUND_V3_USDC_DEPOSITS"
            ],
        }),
    };
    ctx.db.insert_observation(run_id, &aggregate)?;
    ctx.db
        .delete_missing_metric("P1_DEFI_PROTOCOL_USDC_DEPOSITS")?;
    Ok(())
}

async fn collect_coingecko(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=usd-coin,tether,bitcoin,ethereum,paypal-usd,first-digital-usd,ethena-usde,ondo-us-dollar-yield&vs_currencies=usd&include_market_cap=true&include_24hr_vol=true&include_24hr_change=true";
    let fetched = fetch_text(ctx, "CoinGecko simple price", url).await?;
    let observed_at = Utc::now().date_naive().to_string();
    let observations = parse_coingecko_simple_price(&fetched.text, url, &observed_at)?;
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    Ok(())
}

async fn collect_coinmetrics(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://community-api.coinmetrics.io/v4/timeseries/asset-metrics?assets=usdc&metrics=AdrActCnt,TxCnt,TxTfrCnt&frequency=1d&page_size=1";
    let fetched = fetch_text(ctx, "CoinMetrics USDC activity", url).await?;
    let observations = parse_coinmetrics_usdc_activity(&fetched.text, url)?;
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    ctx.db.upsert_missing_item(&MissingItem {
        collector: "CoinMetrics".to_string(),
        metric_code: "P1_USDC_ADJUSTED_TRANSFER_VOLUME".to_string(),
        metric_name: "USDC adjusted transfer volume".to_string(),
        priority: "P1".to_string(),
        reason: "Community API rejects TxTfrValAdjUSD for asset usdc without paid credentials; Dune query/API key is the fallback.".to_string(),
        source_hint: "CoinMetrics Pro or Dune Analytics".to_string(),
    })?;
    Ok(())
}

async fn collect_visa_allium_adjusted_transfers(ctx: &CollectorContext<'_>) -> Result<()> {
    let url =
        "https://app-server-dp-xjpv5b26pq-uw.a.run.app/api/v1/explorer/results/data?format=json";
    let source_url = "https://visaonchainanalytics.com/transactions";
    let payload = json!({
        "sql": r#"with daily as (
  select
    day::date as day,
    sum(usd_amount) as usd_amount,
    sum(txn_count) as txn_count
  from "share"."JKyWRaJi"."Lmrs1Uvwa0RXc4J3a0vC"
  where base_asset = 'USDC'
    and tag = 'Adjusted'
  group by 1
),
latest as (
  select max(day) as day
  from daily
  where day < current_date
)
select
  latest.day,
  daily.usd_amount,
  daily.txn_count,
  (
    select sum(usd_amount)
    from daily
    where day > latest.day - interval '30 days'
      and day <= latest.day
  ) as usd_amount_30d,
  (
    select sum(txn_count)
    from daily
    where day > latest.day - interval '30 days'
      and day <= latest.day
  ) as txn_count_30d
from latest
join daily on daily.day = latest.day"#,
    });
    let fetched = post_json_text(
        ctx,
        "Visa Onchain Analytics / Allium stablecoin transfers",
        url,
        payload,
    )
    .await?;
    let observations = parse_visa_allium_usdc_adjusted_transfer_volume(&fetched.text, source_url)?;
    let has_adjusted_volume = observations
        .iter()
        .any(|obs| obs.metric_code == "P1_USDC_ADJUSTED_TRANSFER_VOLUME");
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    if has_adjusted_volume {
        ctx.db
            .delete_missing_metric("P1_USDC_ADJUSTED_TRANSFER_VOLUME")?;
    }
    Ok(())
}

async fn collect_curve_3pool(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://api.curve.fi/api/getPools/ethereum/main";
    let fetched = fetch_text(ctx, "Curve 3pool", url).await?;
    let observations = parse_curve_3pool(&fetched.text, url)?;
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    ctx.db.delete_missing_metric("P0_CURVE_3POOL_USDC_RATIO")?;
    Ok(())
}

async fn collect_rwa_treasuries(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://app.rwa.xyz/treasuries";
    let fetched = fetch_text(ctx, "RWA.xyz treasuries", url).await?;
    let observations = parse_rwa_treasuries(&fetched.text, url)?;
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    ctx.db.delete_missing_metric("P1_TOKENIZED_TREASURY_AUM")?;
    Ok(())
}

async fn collect_yahoo(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://query1.finance.yahoo.com/v8/finance/chart/CRCL?range=5d&interval=1d";
    let fetched = fetch_text(ctx, "Yahoo Finance CRCL chart", url).await?;
    let observations = parse_yahoo_chart(&fetched.text, url)?;
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    Ok(())
}

async fn collect_finra_short_interest(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://api.finra.org/data/group/otcMarket/name/consolidatedShortInterest";
    let payload = json!({
        "limit": 200,
        "compareFilters": [
            {
                "compareType": "equal",
                "fieldName": "symbolCode",
                "fieldValue": "CRCL"
            }
        ]
    });
    let fetched = post_json_text(ctx, "FINRA consolidated short interest", url, payload).await?;
    let observations = parse_finra_short_interest(&fetched.text, url)?;
    let has_short_interest = observations
        .iter()
        .any(|obs| obs.metric_code == "P2_FINRA_SHORT_INTEREST");
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    if has_short_interest {
        ctx.db.delete_missing_metric("P2_FINRA_SHORT_INTEREST")?;
    }
    Ok(())
}

async fn collect_coinglass_exchange_balance(ctx: &CollectorContext<'_>) -> Result<()> {
    let (run_id, source_url, snapshot) = match collect_coinglass_open_api_balance(ctx).await {
        Ok(Some(snapshot)) => snapshot,
        Ok(None) => collect_coinglass_frontend_balance(ctx).await?,
        Err(open_api_error) => {
            collect_coinglass_frontend_balance(ctx)
                .await
                .with_context(|| {
                    format!(
                        "CoinGlass Open API failed before frontend fallback: {open_api_error:#}"
                    )
                })?
        }
    };

    let has_exchange_balance = snapshot
        .observations
        .iter()
        .any(|obs| obs.metric_code == "P1_EXCHANGE_USDC_BALANCES");
    insert_observations(ctx.db, run_id, &snapshot.observations)?;
    for row in &snapshot.balances {
        ctx.db.insert_exchange_usdc_balance(run_id, row)?;
    }
    if has_exchange_balance {
        ctx.db.delete_missing_metric("P1_EXCHANGE_USDC_BALANCES")?;
    }
    if snapshot.balances.is_empty() {
        return Err(anyhow!(
            "CoinGlass exchange balance snapshot parsed no exchange-level rows from {source_url}"
        ));
    }
    match collect_coinglass_frontend_balance_history(ctx).await {
        Ok((history_run_id, _, history)) => {
            insert_observations(ctx.db, history_run_id, &history.observations)?;
            for point in &history.points {
                ctx.db
                    .insert_exchange_usdc_balance_history_point(history_run_id, point)?;
            }
        }
        Err(error) => {
            eprintln!("CoinGlass frontend Exchange Balance History: {error:#}");
        }
    }
    Ok(())
}

async fn collect_coinglass_open_api_balance(
    ctx: &CollectorContext<'_>,
) -> Result<
    Option<(
        i64,
        String,
        crate::parsing::CoinGlassExchangeBalanceSnapshot,
    )>,
> {
    if coinglass_api_key().is_none() {
        return Ok(None);
    }

    let source = "CoinGlass Open API Exchange Balance List: USDC";
    let fetched = fetch_coinglass_text(ctx, source, COINGLASS_OPEN_API_BALANCE_URL).await?;
    let snapshot =
        parse_coinglass_exchange_balance_snapshot(&fetched.text, COINGLASS_OPEN_API_BALANCE_URL)
            .with_context(|| "CoinGlass Open API response could not be parsed")?;
    Ok(Some((
        fetched.run_id,
        COINGLASS_OPEN_API_BALANCE_URL.to_string(),
        snapshot,
    )))
}

async fn collect_coinglass_frontend_balance(
    ctx: &CollectorContext<'_>,
) -> Result<(
    i64,
    String,
    crate::parsing::CoinGlassExchangeBalanceSnapshot,
)> {
    let url = format!(
        "{COINGLASS_FRONTEND_BALANCE_BASE_URL}&t={}",
        Utc::now().timestamp_millis()
    );
    let source = "CoinGlass frontend Exchange Balance List: USDC";
    let fetched = fetch_coinglass_frontend_text(ctx, source, &url).await?;
    let snapshot = parse_coinglass_exchange_balance_snapshot(&fetched.text, &url)
        .with_context(|| "CoinGlass frontend balance payload could not be parsed")?;
    Ok((fetched.run_id, url, snapshot))
}

async fn collect_coinglass_frontend_balance_history(
    ctx: &CollectorContext<'_>,
) -> Result<(
    i64,
    String,
    crate::parsing::CoinGlassExchangeBalanceHistorySnapshot,
)> {
    let url = format!(
        "{COINGLASS_FRONTEND_BALANCE_HISTORY_BASE_URL}&t={}",
        Utc::now().timestamp_millis()
    );
    let source = "CoinGlass frontend Exchange Balance History: USDC";
    let fetched = fetch_coinglass_frontend_text(ctx, source, &url).await?;
    let snapshot = parse_coinglass_exchange_balance_history_snapshot(&fetched.text, &url)
        .with_context(|| "CoinGlass frontend balance history payload could not be parsed")?;
    Ok((fetched.run_id, url, snapshot))
}

async fn collect_circle_platform_metrics(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://www.circle.com/pressroom/circle-reports-first-quarter-2026-results";
    let fetched = fetch_text(ctx, "Circle Q1 2026 platform metrics", url).await?;
    let observations = parse_circle_platform_metrics(&fetched.text, url)?;
    let has_cpn_tpv = observations
        .iter()
        .any(|obs| obs.metric_code == "P1_CPN_ANNUALIZED_TPV");
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    if has_cpn_tpv {
        ctx.db.delete_missing_metric("P1_CPN_ANNUALIZED_TPV")?;
    }
    Ok(())
}

async fn collect_arc_public_status(ctx: &CollectorContext<'_>) -> Result<()> {
    let primary_url = "https://www.arc.io/";
    let (run_id, observations) = match fetch_text(ctx, "Arc public website", primary_url).await {
        Ok(fetched) => (
            fetched.run_id,
            parse_arc_public_status(&fetched.text, primary_url)?,
        ),
        Err(primary_error) => {
            let primary_error_text = primary_error.to_string();
            let fallback_url = "https://www.circle.com/pressroom/circle-reports-fourth-quarter-and-full-fiscal-year-2025-financial-results";
            let fetched = fetch_text(ctx, "Circle Q4 FY2025 Arc metrics", fallback_url)
                .await
                .with_context(|| {
                    format!(
                        "Arc public website failed ({primary_error_text}); Circle press release fallback also failed"
                    )
                })?;
            let observations = parse_circle_platform_metrics(&fetched.text, fallback_url)
                .with_context(|| {
                    format!(
                        "Arc public website failed ({primary_error_text}); Circle press release fallback had no parsable Arc metrics"
                    )
                })?
                .into_iter()
                .filter(|obs| obs.metric_code.starts_with("P1_ARC_"))
                .collect::<Vec<_>>();
            if observations.is_empty() {
                return Err(anyhow!(
                    "Arc public website failed ({primary_error_text}); Circle press release fallback had no Arc metrics"
                ));
            }
            (fetched.run_id, observations)
        }
    };
    let has_arc_status = observations
        .iter()
        .any(|obs| obs.metric_code.starts_with("P1_ARC_"));
    insert_observations(ctx.db, run_id, &observations)?;
    if has_arc_status {
        ctx.db
            .delete_missing_metric("P1_ARC_PUBLIC_NETWORK_STATUS")?;
        ctx.db.delete_missing_metric("P1_ARC_MAINNET_STATUS")?;
    }
    Ok(())
}

async fn collect_marketbeat_institutional_ownership(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://www.marketbeat.com/stocks/NYSE/CRCL/institutional-ownership/";
    let fetched = fetch_text(ctx, "MarketBeat CRCL institutional ownership", url).await?;
    let observations = parse_marketbeat_institutional_ownership(&fetched.text, url)?;
    if !observations.is_empty() {
        ctx.db
            .delete_missing_metric("P2_CRCL_INSTITUTIONAL_OWNERSHIP")?;
    }
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    Ok(())
}

async fn collect_treasury(ctx: &CollectorContext<'_>) -> Result<()> {
    let now = Utc::now();
    let url = format!(
        "https://home.treasury.gov/resource-center/data-chart-center/interest-rates/pages/xml?data=daily_treasury_yield_curve&field_tdr_date_value_month={}{:02}",
        now.year(),
        now.month()
    );
    let fetched = fetch_text(ctx, "U.S. Treasury yield curve", &url).await?;
    let observations = parse_treasury_yield_curve(&fetched.text, &url)?;
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    Ok(())
}

async fn collect_nyfed(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://markets.newyorkfed.org/api/rates/secured/sofr/last/1.json";
    let fetched = fetch_text(ctx, "NY Fed SOFR", url).await?;
    let observations = parse_nyfed_sofr(&fetched.text, url)?;
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    Ok(())
}

async fn collect_sec_company(
    ctx: &CollectorContext<'_>,
    company_name: &str,
    cik: &str,
) -> Result<()> {
    let url = format!("https://data.sec.gov/submissions/CIK{cik}.json");
    let fetched = fetch_text(ctx, &format!("SEC EDGAR submissions: {company_name}"), &url).await?;
    let (filings, observations) = parse_sec_submissions(&fetched.text, &url)?;
    for filing in filings.iter().take(80) {
        ctx.db.upsert_filing(filing)?;
    }
    insert_observations(ctx.db, fetched.run_id, &observations)?;

    if company_name == "Circle Internet Group" {
        collect_circle_sec_filing_metrics(ctx, &filings).await?;
    } else if company_name == "Coinbase Global" {
        collect_coinbase_sec_filing_metrics(ctx, &filings).await?;
    }

    Ok(())
}

async fn collect_circle_sec_filing_metrics(
    ctx: &CollectorContext<'_>,
    filings: &[Filing],
) -> Result<()> {
    let filing = latest_periodic_filing(filings).context("Circle 10-Q/10-K filing missing")?;
    let url = sec_filing_document_url(filing).context("Circle SEC primary document URL missing")?;
    let fetched = fetch_text(ctx, "SEC EDGAR filing: Circle Internet Group", &url).await?;
    let observed_at = filing.report_date.as_deref().unwrap_or(&filing.filing_date);
    let observations = parse_circle_sec_filing(&fetched.text, &url, observed_at)?;
    let has_velocity = observations
        .iter()
        .any(|obs| obs.metric_code == "P1_USDC_VELOCITY");
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    ctx.db
        .delete_missing_metric("P2_CIRCLE_FINANCIAL_LINE_ITEMS")?;
    if has_velocity {
        ctx.db.delete_missing_metric("P1_USDC_VELOCITY")?;
    }
    Ok(())
}

async fn collect_coinbase_sec_filing_metrics(
    ctx: &CollectorContext<'_>,
    filings: &[Filing],
) -> Result<()> {
    let filing = latest_periodic_filing(filings).context("Coinbase 10-Q/10-K filing missing")?;
    let url =
        sec_filing_document_url(filing).context("Coinbase SEC primary document URL missing")?;
    let fetched = fetch_text(ctx, "SEC EDGAR filing: Coinbase Global", &url).await?;
    let observed_at = filing.report_date.as_deref().unwrap_or(&filing.filing_date);
    let observations = parse_coinbase_sec_filing(&fetched.text, &url, observed_at)?;
    let has_usdc_on_platform = observations
        .iter()
        .any(|obs| obs.metric_code == "P1_COINBASE_USDC_ON_PLATFORM");
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    if has_usdc_on_platform {
        ctx.db
            .delete_missing_metric("P1_COINBASE_USDC_ON_PLATFORM")?;
    }
    Ok(())
}

async fn collect_blackrock_nmfp(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://data.sec.gov/submissions/CIK0000844779.json";
    let fetched = fetch_text(ctx, "SEC EDGAR submissions: BlackRock Funds", url).await?;
    let (filings, observations) = parse_sec_submissions(&fetched.text, url)?;
    for filing in filings
        .iter()
        .filter(|filing| filing.form == "N-MFP3")
        .take(24)
    {
        ctx.db.upsert_filing(filing)?;
    }
    insert_observations(ctx.db, fetched.run_id, &observations)?;

    let filing = filings
        .iter()
        .find(|filing| filing.form == "N-MFP3" && filing.primary_doc.is_some())
        .context("BlackRock Circle Reserve Fund N-MFP3 filing missing")?;
    let filing_url =
        sec_filing_document_url(filing).context("BlackRock N-MFP3 primary document URL missing")?;
    let fetched = fetch_text(ctx, "SEC EDGAR filing: BlackRock Funds N-MFP3", &filing_url).await?;
    let observations = parse_sec_blackrock_nmfp3_filing(&fetched.text, &filing_url)?;
    let has_yield = observations
        .iter()
        .any(|obs| obs.metric_code == "P0_CIRCLE_RESERVE_FUND_7D_YIELD");
    let has_wam = observations
        .iter()
        .any(|obs| obs.metric_code == "P2_RESERVE_FUND_WAM");
    let has_wal = observations
        .iter()
        .any(|obs| obs.metric_code == "P2_RESERVE_FUND_WAL");

    insert_observations(ctx.db, fetched.run_id, &observations)?;
    if has_yield {
        ctx.db
            .delete_missing_metric("P0_CIRCLE_RESERVE_FUND_7D_YIELD")?;
    }
    if has_wam && has_wal {
        ctx.db.delete_missing_metric("P2_RESERVE_FUND_WAM_WAL")?;
    }
    Ok(())
}

fn latest_periodic_filing(filings: &[Filing]) -> Option<&Filing> {
    filings
        .iter()
        .find(|filing| filing.form == "10-Q")
        .or_else(|| filings.iter().find(|filing| filing.form == "10-K"))
}

fn sec_filing_document_url(filing: &Filing) -> Option<String> {
    let primary_doc = filing.primary_doc.as_deref()?;
    if primary_doc.starts_with("http://") || primary_doc.starts_with("https://") {
        return Some(primary_doc.to_string());
    }
    let cik = filing.cik.trim_start_matches('0');
    let accession = filing.accession_no.replace('-', "");
    Some(format!(
        "https://www.sec.gov/Archives/edgar/data/{cik}/{accession}/{primary_doc}"
    ))
}

async fn collect_circle_pressroom(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://www.circle.com/pressroom";
    let fetched = fetch_text(ctx, "Circle Pressroom", url).await?;
    let events = parse_circle_pressroom(&fetched.text, url)?;
    for event in events {
        ctx.db.upsert_event(&event)?;
    }
    Ok(())
}

async fn collect_regulatory_pages(ctx: &CollectorContext<'_>) -> Result<()> {
    let pages = [
        (
            "OCC news releases",
            "https://www.occ.gov/news-events/newsroom/news-issuances-by-year/news-releases/index-news-releases.html",
        ),
        ("Treasury news", "https://home.treasury.gov/news"),
        ("FinCEN news room", "https://www.fincen.gov/news-room"),
        (
            "OFAC recent actions",
            "https://ofac.treasury.gov/recent-actions",
        ),
        (
            "Federal Reserve news",
            "https://www.federalreserve.gov/newsevents.htm",
        ),
        ("SEC newsroom", "https://www.sec.gov/newsroom"),
        ("CFTC press releases", "https://www.cftc.gov/rss.xml"),
        (
            "FDIC press releases",
            "https://www.fdic.gov/news/press-releases/",
        ),
        (
            "Congress GENIUS Act page",
            "https://www.govinfo.gov/bulkdata/BILLSTATUS/119/s/BILLSTATUS-119s1582.xml",
        ),
        (
            "Circle investor events",
            "https://investor.circle.com/events-and-presentations/default.aspx",
        ),
    ];

    for (name, url) in pages {
        match fetch_text(ctx, name, url).await {
            Ok(fetched) => {
                let title = source_page_title(name, &fetched.text);
                if looks_like_bot_challenge(&fetched.text) {
                    ctx.db.upsert_missing_item(&MissingItem {
                        collector: name.to_string(),
                        metric_code: format!("SOURCE_BLOCKED_{}", name_to_code(name)),
                        metric_name: format!("Source page blocked: {name}"),
                        priority: "P0".to_string(),
                        reason: "Page returned a bot or JavaScript challenge in command-line fetch; browser/API-key workflow required.".to_string(),
                        source_hint: url.to_string(),
                    })?;
                    continue;
                }

                ctx.db
                    .delete_missing_metric(&format!("SOURCE_BLOCKED_{}", name_to_code(name)))?;

                if looks_like_unavailable_page(&title, &fetched.text) {
                    ctx.db.upsert_missing_item(&MissingItem {
                        collector: name.to_string(),
                        metric_code: format!("SOURCE_UNREACHABLE_{}", name_to_code(name)),
                        metric_name: format!("Source page unavailable: {name}"),
                        priority: "P0".to_string(),
                        reason: "Page returned a soft 404 or unavailable page despite a successful HTTP status.".to_string(),
                        source_hint: url.to_string(),
                    })?;
                    continue;
                }

                ctx.db
                    .delete_missing_metric(&format!("SOURCE_UNREACHABLE_{}", name_to_code(name)))?;
                ctx.db.upsert_event(&Event {
                    source: name.to_string(),
                    event_type: "source_check".to_string(),
                    title,
                    event_date: Some(Utc::now().date_naive().to_string()),
                    url: url.to_string(),
                    summary: Some(source_check_summary(name, &fetched.text)),
                })?;
            }
            Err(error) => {
                ctx.db.upsert_missing_item(&MissingItem {
                    collector: name.to_string(),
                    metric_code: format!("SOURCE_UNREACHABLE_{}", name_to_code(name)),
                    metric_name: format!("Source page unreachable: {name}"),
                    priority: "P0".to_string(),
                    reason: error.to_string(),
                    source_hint: url.to_string(),
                })?;
            }
        }
    }

    Ok(())
}

async fn collect_status_pages(ctx: &CollectorContext<'_>) -> Result<()> {
    let pages = [
        (
            "Base status",
            "https://status.base.org/api/v2/status.json",
            "P0_CHAIN_STATUS_BASE",
            "Base chain status",
        ),
        (
            "Solana status",
            "https://status.solana.com/api/v2/status.json",
            "P0_CHAIN_STATUS_SOLANA",
            "Solana chain status",
        ),
        (
            "Circle status",
            "https://status.circle.com/api/v2/status.json",
            "P0_CIRCLE_STATUS",
            "Circle service status",
        ),
    ];

    for (name, url, metric_code, metric_name) in pages {
        let fetched = fetch_text(ctx, name, url).await?;
        let observations = parse_statuspage(&fetched.text, url, metric_code, metric_name)?;
        insert_observations(ctx.db, fetched.run_id, &observations)?;
    }

    Ok(())
}

async fn collect_ethereum_status(ctx: &CollectorContext<'_>) -> Result<()> {
    let url = "https://ethereum.publicnode.com";
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": ["latest", false],
        "id": 1,
    });
    let fetched = post_json_text(ctx, "Ethereum public JSON-RPC", url, payload).await?;
    let observations = parse_ethereum_latest_block(&fetched.text, url, Utc::now())?;
    insert_observations(ctx.db, fetched.run_id, &observations)?;
    ctx.db.delete_missing_metric("P0_CHAIN_STATUS_ETHEREUM")?;
    Ok(())
}

struct FetchedText {
    run_id: i64,
    text: String,
}

fn source_run(
    ctx: &CollectorContext<'_>,
    source: String,
    url: String,
    status: &str,
    http_status: Option<u16>,
    error: Option<String>,
    raw_excerpt: Option<String>,
) -> SourceRun {
    SourceRun {
        batch_id: ctx.batch_id.clone(),
        profile: ctx.profile.clone(),
        selector: ctx.selector.clone(),
        source,
        url,
        status: status.to_string(),
        http_status,
        error,
        raw_excerpt,
    }
}

async fn post_json_text(
    ctx: &CollectorContext<'_>,
    source: &str,
    url: &str,
    payload: serde_json::Value,
) -> Result<FetchedText> {
    let response = ctx
        .client
        .post(url)
        .header(ACCEPT, "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(response) => response_to_fetched_text(ctx, source, url, response).await,
        Err(error) => {
            let run = source_run(
                ctx,
                source.to_string(),
                url.to_string(),
                "network_error",
                None,
                Some(error.to_string()),
                None,
            );
            ctx.db.insert_source_run(&run)?;
            Err(error.into())
        }
    }
}

async fn fetch_text(ctx: &CollectorContext<'_>, source: &str, url: &str) -> Result<FetchedText> {
    let mut request = ctx
        .client
        .get(url)
        .header(ACCEPT, "*/*")
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.9");
    if source.starts_with("SEC EDGAR") {
        request = request.header("User-Agent", &ctx.user_agent);
    }
    let response = request.send().await;

    match response {
        Ok(response) => response_to_fetched_text(ctx, source, url, response).await,
        Err(error) => {
            let run = source_run(
                ctx,
                source.to_string(),
                url.to_string(),
                "network_error",
                None,
                Some(error.to_string()),
                None,
            );
            ctx.db.insert_source_run(&run)?;
            Err(error.into())
        }
    }
}

async fn fetch_coinglass_text(
    ctx: &CollectorContext<'_>,
    source: &str,
    url: &str,
) -> Result<FetchedText> {
    let mut request = ctx.client.get(url).header(ACCEPT, "application/json");
    if let Some(api_key) = coinglass_api_key() {
        request = request.header("CG-API-KEY", api_key);
    }
    let response = request.send().await;

    match response {
        Ok(response) => response_to_fetched_text(ctx, source, url, response).await,
        Err(error) => {
            let run = source_run(
                ctx,
                source.to_string(),
                url.to_string(),
                "network_error",
                None,
                Some(error.to_string()),
                None,
            );
            ctx.db.insert_source_run(&run)?;
            Err(error.into())
        }
    }
}

async fn fetch_coinglass_frontend_text(
    ctx: &CollectorContext<'_>,
    source: &str,
    url: &str,
) -> Result<FetchedText> {
    let response = ctx
        .client
        .get(url)
        .header(ACCEPT, "application/json")
        .header("language", "zh")
        .header("encryption", "true")
        .header("cache-ts-v2", "1")
        .header(REFERER, COINGLASS_FRONTEND_REFERER)
        .header(USER_AGENT, "Mozilla/5.0")
        .send()
        .await;

    match response {
        Ok(response) => {
            response_to_coinglass_frontend_fetched_text(ctx, source, url, response).await
        }
        Err(error) => {
            let run = source_run(
                ctx,
                source.to_string(),
                url.to_string(),
                "network_error",
                None,
                Some(error.to_string()),
                None,
            );
            ctx.db.insert_source_run(&run)?;
            Err(error.into())
        }
    }
}

async fn response_to_coinglass_frontend_fetched_text(
    ctx: &CollectorContext<'_>,
    source: &str,
    url: &str,
    response: reqwest::Response,
) -> Result<FetchedText> {
    let status = response.status();
    let http_status = status.as_u16();
    let headers = response.headers().clone();
    match response.text().await {
        Ok(text) => {
            if !status.is_success() {
                let run = source_run(
                    ctx,
                    source.to_string(),
                    url.to_string(),
                    "http_error",
                    Some(http_status),
                    Some(format!("HTTP {http_status}")),
                    Some(excerpt(&text)),
                );
                ctx.db.insert_source_run(&run)?;
                return Err(anyhow!("{source} returned HTTP {http_status}"));
            }

            match decrypt_coinglass_frontend_payload(&text, &headers) {
                Ok(decrypted_text) => {
                    let run = source_run(
                        ctx,
                        source.to_string(),
                        url.to_string(),
                        "ok",
                        Some(http_status),
                        None,
                        Some(excerpt(&decrypted_text)),
                    );
                    let run_id = ctx.db.insert_source_run(&run)?;
                    Ok(FetchedText {
                        run_id,
                        text: decrypted_text,
                    })
                }
                Err(error) => {
                    let run = source_run(
                        ctx,
                        source.to_string(),
                        url.to_string(),
                        "decode_error",
                        Some(http_status),
                        Some(error.to_string()),
                        Some(excerpt(&text)),
                    );
                    ctx.db.insert_source_run(&run)?;
                    Err(error).with_context(|| "failed to decrypt CoinGlass frontend payload")
                }
            }
        }
        Err(error) => {
            let error_text = format!("failed to read response body for {source}: {error}");
            let run = source_run(
                ctx,
                source.to_string(),
                url.to_string(),
                "body_error",
                Some(http_status),
                Some(error_text.clone()),
                None,
            );
            ctx.db.insert_source_run(&run)?;
            Err(anyhow!(error_text))
        }
    }
}

async fn response_to_fetched_text(
    ctx: &CollectorContext<'_>,
    source: &str,
    url: &str,
    response: reqwest::Response,
) -> Result<FetchedText> {
    let status = response.status();
    let http_status = status.as_u16();
    match response.text().await {
        Ok(text) => {
            let run = source_run(
                ctx,
                source.to_string(),
                url.to_string(),
                if status.is_success() {
                    "ok"
                } else {
                    "http_error"
                },
                Some(http_status),
                if status.is_success() {
                    None
                } else {
                    Some(format!("HTTP {http_status}"))
                },
                Some(excerpt(&text)),
            );
            let run_id = ctx.db.insert_source_run(&run)?;
            if !status.is_success() {
                return Err(anyhow!("{source} returned HTTP {http_status}"));
            }
            Ok(FetchedText { run_id, text })
        }
        Err(error) => {
            let error_text = format!("failed to read response body for {source}: {error}");
            let run = source_run(
                ctx,
                source.to_string(),
                url.to_string(),
                "body_error",
                Some(http_status),
                Some(error_text.clone()),
                None,
            );
            ctx.db.insert_source_run(&run)?;
            Err(anyhow!(error_text))
        }
    }
}

fn decrypt_coinglass_frontend_payload(text: &str, headers: &HeaderMap) -> Result<String> {
    let encrypted =
        header_text(headers, "encryption").is_some_and(|value| value.eq_ignore_ascii_case("true"));
    if !encrypted {
        return Ok(text.to_string());
    }

    let v = header_text(headers, "v").context("CoinGlass frontend response missing v header")?;
    let user = header_text(headers, "user")
        .context("CoinGlass frontend response missing encrypted user header")?;
    let seed = coinglass_frontend_seed(v)
        .with_context(|| format!("unsupported CoinGlass frontend encryption v={v}"))?;
    let key0 = BASE64
        .encode(seed.as_bytes())
        .chars()
        .take(16)
        .collect::<String>();
    let key1 = decrypt_coinglass_ciphertext(user, key0.as_bytes())
        .context("failed to decrypt CoinGlass frontend session key")?;

    let payload: serde_json::Value =
        serde_json::from_str(text).context("CoinGlass frontend response is not valid JSON")?;
    let data = payload
        .get("data")
        .and_then(serde_json::Value::as_str)
        .context("CoinGlass frontend response missing encrypted data field")?;
    decrypt_coinglass_ciphertext(data, key1.as_bytes())
        .context("failed to decrypt CoinGlass frontend data field")
}

fn header_text<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name)?.to_str().ok()
}

fn coinglass_frontend_seed(v: &str) -> Option<&'static str> {
    match v.trim() {
        "55" => Some("170b070da9654622"),
        "66" => Some("d6537d845a964081"),
        "77" => Some("863f08689c97435b"),
        _ => None,
    }
}

fn decrypt_coinglass_ciphertext(ciphertext: &str, key: &[u8]) -> Result<String> {
    if key.len() != 16 {
        return Err(anyhow!(
            "CoinGlass AES key must be 16 bytes, got {}",
            key.len()
        ));
    }

    let mut bytes = BASE64
        .decode(ciphertext.trim())
        .context("CoinGlass ciphertext is not valid base64")?;
    let decrypted = Aes128EcbDec::new_from_slice(key)
        .map_err(|_| anyhow!("invalid CoinGlass AES-128 key"))?
        .decrypt_padded_mut::<Pkcs7>(&mut bytes)
        .map_err(|_| anyhow!("CoinGlass AES-ECB decrypt or PKCS7 unpad failed"))?
        .to_vec();
    let inflated = inflate_coinglass_bytes(&decrypted)?;
    String::from_utf8(inflated).context("CoinGlass decrypted payload is not UTF-8")
}

fn inflate_coinglass_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
    for decoder in [InflateKind::Gzip, InflateKind::Zlib, InflateKind::Deflate] {
        if let Ok(inflated) = inflate_with(bytes, decoder) {
            return Ok(inflated);
        }
    }
    Err(anyhow!(
        "CoinGlass decrypted payload is not gzip, zlib, or deflate compressed"
    ))
}

fn inflate_with(bytes: &[u8], kind: InflateKind) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    match kind {
        InflateKind::Gzip => GzDecoder::new(bytes).read_to_end(&mut out)?,
        InflateKind::Zlib => ZlibDecoder::new(bytes).read_to_end(&mut out)?,
        InflateKind::Deflate => DeflateDecoder::new(bytes).read_to_end(&mut out)?,
    };
    Ok(out)
}

#[derive(Clone, Copy)]
enum InflateKind {
    Gzip,
    Zlib,
    Deflate,
}

fn coinglass_api_key() -> Option<String> {
    env::var("COINGLASS_API_KEY")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn insert_observations(db: &Database, run_id: i64, observations: &[Observation]) -> Result<()> {
    for obs in observations {
        db.insert_observation(run_id, obs)?;
    }
    Ok(())
}

fn excerpt(text: &str) -> String {
    text.chars().take(1200).collect()
}

fn looks_like_bot_challenge(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("just a moment") || lower.contains("enable javascript and cookies")
}

fn looks_like_unavailable_page(title: &str, text: &str) -> bool {
    let lower_title = title.to_ascii_lowercase();
    if lower_title.contains("404") || lower_title.contains("not found") {
        return true;
    }

    let excerpt = text
        .chars()
        .take(4096)
        .collect::<String>()
        .to_ascii_lowercase();
    excerpt.contains("rx-contentitemname\" content=\"404")
        || excerpt.contains("page not found")
        || excerpt.contains("the requested page could not be found")
}

fn source_page_title(name: &str, text: &str) -> String {
    page_title(text)
        .filter(|title| !title.trim().is_empty() && title.chars().count() <= 180)
        .unwrap_or_else(|| name.to_string())
}

fn source_check_summary(name: &str, text: &str) -> String {
    if name == "Congress GENIUS Act page"
        && let Some(summary) = govinfo_bill_status_summary(text)
    {
        return summary;
    }

    "Source reachability check; detailed regulatory interpretation remains manual.".to_string()
}

fn govinfo_bill_status_summary(text: &str) -> Option<String> {
    let doc = roxmltree::Document::parse(text).ok()?;
    let bill = doc.descendants().find(|node| node.has_tag_name("bill"))?;
    let title = direct_child_text(bill, "title").unwrap_or_else(|| "GENIUS Act".to_string());
    let action = bill
        .children()
        .find(|node| node.has_tag_name("actions"))?
        .children()
        .find(|node| node.has_tag_name("item"))?;
    let action_date = direct_child_text(action, "actionDate")?;
    let action_text = direct_child_text(action, "text")?;

    Some(format!(
        "{title}; latest action {action_date}: {action_text}"
    ))
}

fn direct_child_text(node: roxmltree::Node<'_, '_>, tag_name: &str) -> Option<String> {
    node.children()
        .find(|child| child.has_tag_name(tag_name))?
        .text()
        .map(|text| text.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|text| !text.is_empty())
}

fn name_to_code(name: &str) -> String {
    name.chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>()
        .to_ascii_uppercase()
}

fn record_known_gaps(db: &Database, selector: SourceSelector) -> Result<()> {
    let gaps = [
        (
            "BlackRock USDXX",
            "P0_CIRCLE_RESERVE_FUND_7D_YIELD",
            "Circle Reserve Fund 7-day yield",
            "P0",
            "SEC N-MFP3 latest business-day net yield extraction is attempted; this remains missing if the latest Circle Reserve Fund filing is unavailable or its rendered table schema changes.",
            "https://data.sec.gov/submissions/CIK0000844779.json",
            &[SourceSelector::Rates][..],
        ),
        (
            "BlackRock USDXX",
            "P2_RESERVE_FUND_WAM_WAL",
            "Reserve Fund WAM / WAL",
            "P2",
            "SEC N-MFP3 WAM/WAL extraction is attempted; this remains missing if the latest Circle Reserve Fund filing is unavailable or its rendered table schema changes.",
            "https://data.sec.gov/submissions/CIK0000844779.json",
            &[SourceSelector::Rates][..],
        ),
        (
            "Dune Analytics",
            "P1_USDC_BY_CHAIN_DUNE",
            "Dune USDC by chain dashboards",
            "P1",
            "Dune requires query IDs/API key for reliable automation; DefiLlama chain distribution is collected as a fallback.",
            "https://dune.com",
            &[SourceSelector::Market][..],
        ),
        (
            "Dune Analytics",
            "P1_USDC_VELOCITY",
            "USDC velocity",
            "P1",
            "SEC filing-period velocity is attempted from Circle onchain volume and average USDC in circulation; adjusted weekly velocity still needs Dune or CoinMetrics Pro.",
            "Circle SEC filings; Dune or CoinMetrics Pro for adjusted weekly velocity",
            &[SourceSelector::Sec][..],
        ),
        (
            "CoinGlass",
            "P1_EXCHANGE_USDC_BALANCES",
            "Exchange USDC balances",
            "P1",
            "CoinGlass Exchange Balance List is collected through Open API when COINGLASS_API_KEY is set, otherwise through the public frontend capi.coinglass.com balance endpoints; history is used for 90D/365D trend observations.",
            "https://capi.coinglass.com/api/exchange/chain/v3/balance/list?symbol=USDC",
            &[SourceSelector::Market][..],
        ),
        (
            "DefiLlama protocol TVL",
            "P1_DEFI_PROTOCOL_USDC_DEPOSITS",
            "Aave / Compound USDC deposits",
            "P1",
            "DefiLlama protocol token breakdown extraction is attempted for Aave V3 and Compound V3; remains missing only if endpoint schema or USDC symbol mapping changes.",
            "https://api.llama.fi/protocol/aave-v3 and https://api.llama.fi/protocol/compound-v3",
            &[SourceSelector::Market][..],
        ),
        (
            "RWA.xyz",
            "P1_TOKENIZED_TREASURY_AUM",
            "Tokenized Treasury AUM",
            "P1",
            "RWA.xyz public treasuries page extraction is attempted; this remains missing only if the page schema changes or the source becomes unavailable. Official API still requires Authorization: Bearer $RWA_API_KEY.",
            "https://app.rwa.xyz/treasuries or https://api.rwa.xyz/v4/assets with RWA_API_KEY",
            &[SourceSelector::Market][..],
        ),
        (
            "SEC XBRL extraction",
            "P2_CIRCLE_FINANCIAL_LINE_ITEMS",
            "Reserve income / distribution costs / RLDC / Other revenue / Adjusted EBITDA",
            "P2",
            "Circle filing line-item extraction is attempted from the latest 10-Q/10-K; this remains missing only if the filing document or taxonomy mapping fails.",
            "SEC companyfacts/submissions and Circle filings",
            &[SourceSelector::Sec][..],
        ),
        (
            "SEC / Coinbase filings",
            "P1_COINBASE_USDC_ON_PLATFORM",
            "Coinbase platform USDC balance / channel concentration",
            "P1",
            "Coinbase stablecoin revenue, receivables, financing receivables, and client custodial funds are collected from SEC filings, but an exact USDC-on-platform balance is not disclosed as a stable public XBRL field.",
            "Coinbase 10-Q / 10-K",
            &[SourceSelector::Sec][..],
        ),
        (
            "FINRA",
            "P2_FINRA_SHORT_INTEREST",
            "FINRA short interest",
            "P2",
            "FINRA public OTC short-interest API is accessible, but it is not the exchange-listed NYSE CRCL dataset; a current exchange-listed source/API still needs to be configured.",
            "FINRA / NYSE / Nasdaq short interest for NYSE:CRCL",
            &[SourceSelector::Market][..],
        ),
        (
            "Ethereum status",
            "P0_CHAIN_STATUS_ETHEREUM",
            "Ethereum chain status",
            "P0",
            "Public Ethereum JSON-RPC latest-block freshness check is attempted; this remains missing only if the RPC endpoint or schema is unavailable. This is not an official Ethereum Foundation Statuspage.",
            "https://ethereum.publicnode.com",
            &[SourceSelector::Status][..],
        ),
    ];

    for (collector, code, name, priority, reason, source_hint, selectors) in gaps {
        if selector != SourceSelector::All && !selectors.contains(&selector) {
            continue;
        }
        db.upsert_missing_item(&MissingItem {
            collector: collector.to_string(),
            metric_code: code.to_string(),
            metric_name: name.to_string(),
            priority: priority.to_string(),
            reason: reason.to_string(),
            source_hint: source_hint.to_string(),
        })?;
    }

    Ok(())
}
