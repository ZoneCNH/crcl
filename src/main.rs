mod collectors;
mod db;
mod models;
mod parsing;

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use collectors::{CollectorContext, run_collectors, source_selector_label};
use db::{Database, RecentEvent, RecentFiling, RecentObservation, RecentSourceRun, Summary};
use serde::Serialize;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(long, default_value = "data/crcl_research.sqlite")]
    database: PathBuf,

    #[arg(long, default_value_t = 20)]
    timeout_secs: u64,

    #[arg(
        long,
        env = "CRCL_DATA_USER_AGENT",
        default_value = "crcl-data-collector/0.1 research@example.com"
    )]
    user_agent: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    InitDb,
    Collect {
        #[arg(long, value_enum, default_value_t = SourceSelector::All)]
        source: SourceSelector,
    },
    AgentContext {
        #[arg(long, value_enum, default_value_t = AgentProfile::Source)]
        profile: AgentProfile,

        #[arg(long, default_value_t = 12)]
        limit: usize,

        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,

        #[arg(long)]
        no_collect: bool,
    },
    Summary,
    Missing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum SourceSelector {
    All,
    Market,
    Rates,
    Sec,
    Events,
    Status,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum AgentProfile {
    Source,
    Financial,
    Regulatory,
    Competition,
    Platform,
    Risk,
    Autoresearch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Serialize)]
struct CollectorRunOutput {
    batch_id: String,
    ok_sources: usize,
    warn_sources: usize,
}

#[derive(Debug, Serialize)]
struct AgentContextOutput {
    profile: String,
    database: String,
    limit: usize,
    collector_run: Option<CollectorRunOutput>,
    summary: Summary,
    recent_source_runs: Vec<RecentSourceRun>,
    latest_observations: Vec<RecentObservation>,
    recent_filings: Vec<RecentFiling>,
    recent_events: Vec<RecentEvent>,
    missing_items: Vec<models::MissingItem>,
}

struct AgentContextRequest<'a> {
    db: &'a Database,
    database: &'a Path,
    timeout_secs: u64,
    user_agent: String,
    profile: AgentProfile,
    limit: usize,
    format: OutputFormat,
    no_collect: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let db = Database::open(&cli.database)?;
    db.init()?;

    match cli.command {
        Command::InitDb => {
            println!("SQLite schema ready: {}", cli.database.display());
        }
        Command::Collect { source } => {
            let batch_id = new_batch_id("collect");
            db.start_collection_batch(
                &batch_id,
                "collect",
                None,
                Some(source_selector_label(source)),
            )?;
            let client = reqwest::Client::builder()
                .use_native_tls()
                .timeout(Duration::from_secs(cli.timeout_secs))
                .user_agent(cli.user_agent.clone())
                .build()?;
            let ctx = CollectorContext {
                db: &db,
                client,
                user_agent: cli.user_agent,
                batch_id: Some(batch_id.clone()),
                profile: None,
                selector: None,
            };
            let report = run_collectors(&ctx, source).await?;
            db.finish_collection_batch(&batch_id, report.ok_sources, report.warn_sources)?;
            println!(
                "collection finished: {} sources ok, {} sources warned/failed, batch_id={}, database={}",
                report.ok_sources,
                report.warn_sources,
                batch_id,
                cli.database.display()
            );
        }
        Command::AgentContext {
            profile,
            limit,
            format,
            no_collect,
        } => {
            run_agent_context(AgentContextRequest {
                db: &db,
                database: &cli.database,
                timeout_secs: cli.timeout_secs,
                user_agent: cli.user_agent,
                profile,
                limit,
                format,
                no_collect,
            })
            .await?;
        }
        Command::Summary => {
            let summary = db.summary()?;
            println!("collection_batches={}", summary.collection_batches);
            println!("source_runs={}", summary.source_runs);
            println!("observations={}", summary.observations);
            println!("filings={}", summary.filings);
            println!("events={}", summary.events);
            println!("missing_items={}", summary.missing_items);
        }
        Command::Missing => {
            for item in db.missing_items()? {
                println!(
                    "{} | {} | {} | {}",
                    item.priority, item.metric_code, item.metric_name, item.reason
                );
            }
        }
    }

    Ok(())
}

async fn run_agent_context(request: AgentContextRequest<'_>) -> Result<()> {
    let mut ok_sources = 0;
    let mut warn_sources = 0;
    let mut batch_id = None;

    if !request.no_collect {
        let new_batch_id = new_batch_id("agent-context");
        let selector_summary = selectors_for_profile(request.profile)
            .into_iter()
            .map(source_selector_label)
            .collect::<Vec<_>>()
            .join(",");
        request.db.start_collection_batch(
            &new_batch_id,
            "agent-context",
            Some(profile_label(request.profile)),
            Some(&selector_summary),
        )?;
        batch_id = Some(new_batch_id);

        let client = reqwest::Client::builder()
            .use_native_tls()
            .timeout(Duration::from_secs(request.timeout_secs))
            .user_agent(request.user_agent.clone())
            .build()?;
        let ctx = CollectorContext {
            db: request.db,
            client,
            user_agent: request.user_agent,
            batch_id: batch_id.clone(),
            profile: Some(profile_label(request.profile).to_string()),
            selector: None,
        };

        for selector in selectors_for_profile(request.profile) {
            let report = run_collectors(&ctx, selector).await?;
            ok_sources += report.ok_sources;
            warn_sources += report.warn_sources;
        }

        if let Some(batch_id) = &batch_id {
            request
                .db
                .finish_collection_batch(batch_id, ok_sources, warn_sources)?;
        }
    }

    print_agent_context(
        request.db,
        request.database,
        request.profile,
        request.limit,
        request.format,
        if request.no_collect {
            None
        } else {
            Some(CollectorRunOutput {
                batch_id: batch_id.unwrap_or_default(),
                ok_sources,
                warn_sources,
            })
        },
    )
}

fn selectors_for_profile(profile: AgentProfile) -> Vec<SourceSelector> {
    match profile {
        AgentProfile::Source | AgentProfile::Risk | AgentProfile::Autoresearch => {
            vec![SourceSelector::All]
        }
        AgentProfile::Financial => vec![
            SourceSelector::Sec,
            SourceSelector::Rates,
            SourceSelector::Market,
        ],
        AgentProfile::Regulatory => vec![SourceSelector::Events, SourceSelector::Status],
        AgentProfile::Competition => vec![SourceSelector::Market, SourceSelector::Events],
        AgentProfile::Platform => vec![
            SourceSelector::Events,
            SourceSelector::Sec,
            SourceSelector::Market,
        ],
    }
}

fn print_agent_context(
    db: &Database,
    database: &Path,
    profile: AgentProfile,
    limit: usize,
    format: OutputFormat,
    collector_run: Option<CollectorRunOutput>,
) -> Result<()> {
    let summary = db.summary()?;
    let recent_source_runs = db
        .recent_source_runs(limit.saturating_mul(20).max(100))?
        .into_iter()
        .filter(|run| source_run_matches_profile(profile, run))
        .take(limit)
        .collect::<Vec<_>>();
    let latest_observations = db
        .latest_observations()?
        .into_iter()
        .filter(|obs| observation_matches_profile(profile, obs))
        .take(limit)
        .collect::<Vec<_>>();
    let recent_filings = db.recent_filings(limit)?;
    let recent_events = db
        .recent_events(limit.saturating_mul(20).max(100))?
        .into_iter()
        .filter(|event| event_matches_profile(profile, event))
        .take(limit)
        .collect::<Vec<_>>();
    let missing_items = db
        .missing_items()?
        .into_iter()
        .filter(|item| missing_item_matches_profile(profile, item))
        .take(limit)
        .collect::<Vec<_>>();

    let output = AgentContextOutput {
        profile: profile_label(profile).to_string(),
        database: database.display().to_string(),
        limit,
        collector_run,
        summary,
        recent_source_runs,
        latest_observations,
        recent_filings,
        recent_events,
        missing_items,
    };

    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        OutputFormat::Text => print_agent_context_text(&output),
    }
}

fn print_agent_context_text(output: &AgentContextOutput) -> Result<()> {
    println!("# CRCL agent context");
    println!("profile={}", output.profile);
    println!("database={}", output.database);
    println!("limit={}", output.limit);
    match &output.collector_run {
        Some(run) => {
            println!(
                "collector_run=ok:{} warn:{} batch_id={}",
                run.ok_sources, run.warn_sources, run.batch_id
            );
        }
        None => println!("collector_run=skipped"),
    }

    println!("\n## database summary");
    println!("collection_batches={}", output.summary.collection_batches);
    println!("source_runs={}", output.summary.source_runs);
    println!("observations={}", output.summary.observations);
    println!("filings={}", output.summary.filings);
    println!("events={}", output.summary.events);
    println!("missing_items={}", output.summary.missing_items);

    println!("\n## recent source runs");
    for run in &output.recent_source_runs {
        let http_status = run
            .http_status
            .map(|status| status.to_string())
            .unwrap_or_else(|| "-".to_string());
        let error = run.error.as_deref().unwrap_or("-");
        println!(
            "- {} | {} | selector={} | batch={} | http={} | fetched_at={} | error={} | {}",
            run.source,
            run.status,
            run.selector.as_deref().unwrap_or("-"),
            run.batch_id.as_deref().unwrap_or("-"),
            http_status,
            run.fetched_at,
            error,
            run.url
        );
    }

    println!("\n## latest observations");
    for obs in &output.latest_observations {
        println!(
            "- {} | {} | {} | {} | observed_at={} | created_at={} | selector={} | {} | {} | {}",
            obs.priority,
            obs.metric_code,
            obs.category,
            display_value(obs.value_num, obs.value_text.as_deref(), &obs.unit),
            obs.observed_at,
            obs.created_at,
            obs.selector.as_deref().unwrap_or("-"),
            obs.source,
            obs.metric_name,
            obs.source_url
        );
    }

    println!("\n## recent filings");
    for filing in &output.recent_filings {
        println!(
            "- {} | {} | filing_date={} | report_date={} | doc={} | {} | {}",
            filing.company,
            filing.form,
            filing.filing_date,
            filing.report_date.as_deref().unwrap_or("-"),
            filing.primary_doc.as_deref().unwrap_or("-"),
            filing.description.as_deref().unwrap_or("-"),
            filing.source_url
        );
    }

    println!("\n## recent events");
    for event in &output.recent_events {
        println!(
            "- {} | {} | date={} | {} | {} | {}",
            event.source,
            event.event_type,
            event.event_date.as_deref().unwrap_or("-"),
            event.title,
            event.summary.as_deref().unwrap_or("-"),
            event.url
        );
    }

    println!("\n## missing items");
    for item in &output.missing_items {
        println!(
            "- {} | {} | {} | {} | {} | {}",
            item.priority,
            item.metric_code,
            item.collector,
            item.metric_name,
            item.reason,
            item.source_hint
        );
    }

    Ok(())
}

fn display_value(value_num: Option<f64>, value_text: Option<&str>, unit: &str) -> String {
    match (value_num, value_text) {
        (Some(value), _) if unit == "status" => format!("{value:.0} {unit}"),
        (Some(value), _) => format!("{value:.4} {unit}"),
        (None, Some(value)) => format!("{value} {unit}"),
        (None, None) => format!("- {unit}"),
    }
}

fn new_batch_id(command: &str) -> String {
    let timestamp = chrono::Utc::now()
        .timestamp_nanos_opt()
        .unwrap_or_else(|| chrono::Utc::now().timestamp_micros() * 1000);
    format!("{command}-{timestamp}-{}", std::process::id())
}

fn source_run_matches_profile(profile: AgentProfile, run: &RecentSourceRun) -> bool {
    if matches!(
        profile,
        AgentProfile::Source | AgentProfile::Risk | AgentProfile::Autoresearch
    ) {
        return true;
    }

    if let Some(selector) = run.selector.as_deref() {
        if selector == "all" {
            return true;
        }
        return selector_labels_for_profile(profile).contains(&selector);
    }

    let source = run.source.to_ascii_lowercase();
    match profile {
        AgentProfile::Financial => [
            "sec edgar",
            "treasury",
            "ny fed",
            "blackrock",
            "defillama",
            "coingecko",
            "coinmetrics",
            "visa",
            "curve",
            "rwa",
            "yahoo",
            "finra",
            "glassnode",
        ]
        .iter()
        .any(|needle| source.contains(needle)),
        AgentProfile::Regulatory => [
            "occ",
            "treasury",
            "fincen",
            "ofac",
            "federal reserve",
            "sec newsroom",
            "cftc",
            "fdic",
            "congress",
            "status",
            "ethereum",
            "base",
            "solana",
            "circle status",
        ]
        .iter()
        .any(|needle| source.contains(needle)),
        AgentProfile::Competition => [
            "defillama",
            "coingecko",
            "coinmetrics",
            "visa",
            "curve",
            "rwa",
            "yahoo",
            "finra",
            "glassnode",
            "circle pressroom",
        ]
        .iter()
        .any(|needle| source.contains(needle)),
        AgentProfile::Platform => [
            "circle pressroom",
            "circle investor",
            "sec edgar",
            "defillama",
            "coingecko",
            "coinmetrics",
            "visa",
            "rwa",
            "yahoo",
        ]
        .iter()
        .any(|needle| source.contains(needle)),
        AgentProfile::Source | AgentProfile::Risk | AgentProfile::Autoresearch => true,
    }
}

fn observation_matches_profile(profile: AgentProfile, obs: &RecentObservation) -> bool {
    if matches!(
        profile,
        AgentProfile::Source | AgentProfile::Risk | AgentProfile::Autoresearch
    ) {
        return true;
    }

    let category = obs.category.as_str();
    let metric = obs.metric_code.as_str();
    match profile {
        AgentProfile::Financial => matches!(
            category,
            "rates"
                | "equity_market"
                | "sec_filing"
                | "income_statement"
                | "operating_indicator"
                | "reserve_composition"
                | "stablecoin_supply"
                | "chain_activity"
                | "channel_dependence"
                | "peg_check"
        ),
        AgentProfile::Regulatory => category == "technical_status" || metric.starts_with("SOURCE_"),
        AgentProfile::Competition => matches!(
            category,
            "stablecoin_supply"
                | "competition"
                | "chain_distribution"
                | "chain_activity"
                | "defi_adoption"
                | "rwa_treasuries"
                | "exchange_balance"
                | "peg_check"
                | "peg_liquidity"
                | "equity_market"
        ),
        AgentProfile::Platform => matches!(
            category,
            "income_statement"
                | "operating_indicator"
                | "chain_activity"
                | "stablecoin_supply"
                | "competition"
                | "defi_adoption"
                | "rwa_treasuries"
                | "sec_filing"
                | "channel_dependence"
        ),
        AgentProfile::Source | AgentProfile::Risk | AgentProfile::Autoresearch => true,
    }
}

fn event_matches_profile(profile: AgentProfile, event: &RecentEvent) -> bool {
    if matches!(
        profile,
        AgentProfile::Source | AgentProfile::Risk | AgentProfile::Autoresearch
    ) {
        return true;
    }

    let source = event.source.to_ascii_lowercase();
    let event_type = event.event_type.to_ascii_lowercase();
    match profile {
        AgentProfile::Financial => {
            source.contains("circle investor")
                || source.contains("circle pressroom")
                || event.title.contains("Results")
        }
        AgentProfile::Regulatory => {
            event_type == "source_check"
                || [
                    "occ",
                    "treasury",
                    "fincen",
                    "ofac",
                    "federal reserve",
                    "sec",
                    "cftc",
                    "fdic",
                    "congress",
                ]
                .iter()
                .any(|needle| source.contains(needle))
        }
        AgentProfile::Competition | AgentProfile::Platform => {
            source.contains("circle pressroom") || source.contains("circle investor")
        }
        AgentProfile::Source | AgentProfile::Risk | AgentProfile::Autoresearch => true,
    }
}

fn missing_item_matches_profile(profile: AgentProfile, item: &models::MissingItem) -> bool {
    if matches!(
        profile,
        AgentProfile::Source | AgentProfile::Risk | AgentProfile::Autoresearch
    ) {
        return true;
    }

    let code = item.metric_code.as_str();
    let collector = item.collector.to_ascii_lowercase();
    match profile {
        AgentProfile::Financial => {
            code.starts_with("P0_TREASURY")
                || code == "P0_SOFR"
                || code.starts_with("P0_CIRCLE_RESERVE")
                || code.starts_with("P2_RESERVE")
                || code.starts_with("P2_CIRCLE")
                || code.starts_with("P1_COINBASE")
                || code == "P1_USDC_VELOCITY"
                || code == "P1_USDC_ADJUSTED_TRANSFER_VOLUME"
        }
        AgentProfile::Regulatory => {
            code.starts_with("SOURCE_")
                || code.contains("STATUS")
                || [
                    "occ", "treasury", "fincen", "ofac", "federal", "sec", "cftc", "fdic",
                    "congress",
                ]
                .iter()
                .any(|needle| collector.contains(needle))
        }
        AgentProfile::Competition => {
            code.starts_with("P1_USDC")
                || code.starts_with("P1_DEFI")
                || code.starts_with("P1_TOKENIZED")
                || code.starts_with("P1_EXCHANGE")
                || code.starts_with("P2_FINRA")
                || code.starts_with("P0_CURVE")
                || code == "P0_CIRCLE_MINTED_REDEEMED"
        }
        AgentProfile::Platform => {
            code.starts_with("P2_CIRCLE")
                || code.starts_with("P1_COINBASE")
                || code == "P1_USDC_ADJUSTED_TRANSFER_VOLUME"
                || code.starts_with("P1_DEFI")
                || code.starts_with("P1_TOKENIZED")
                || collector.contains("circle")
        }
        AgentProfile::Source | AgentProfile::Risk | AgentProfile::Autoresearch => true,
    }
}

fn selector_labels_for_profile(profile: AgentProfile) -> &'static [&'static str] {
    match profile {
        AgentProfile::Source | AgentProfile::Risk | AgentProfile::Autoresearch => &["all"],
        AgentProfile::Financial => &["sec", "rates", "market"],
        AgentProfile::Regulatory => &["events", "status"],
        AgentProfile::Competition => &["market", "events"],
        AgentProfile::Platform => &["events", "sec", "market"],
    }
}

fn profile_label(profile: AgentProfile) -> &'static str {
    match profile {
        AgentProfile::Source => "source",
        AgentProfile::Financial => "financial",
        AgentProfile::Regulatory => "regulatory",
        AgentProfile::Competition => "competition",
        AgentProfile::Platform => "platform",
        AgentProfile::Risk => "risk",
        AgentProfile::Autoresearch => "autoresearch",
    }
}
