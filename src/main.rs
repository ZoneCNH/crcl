mod agent_runner;
mod collectors;
mod cron_runner;
mod db;
mod full_analysis;
mod models;
mod parsing;
mod workflows;

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use collectors::{CollectorContext, run_collectors, source_selector_label};
use db::{Database, RecentEvent, RecentFiling, RecentObservation, RecentSourceRun, Summary};
use serde::Serialize;
use workflows::{WorkflowCollectorRun, WorkflowKind, WorkflowPacketRequest};

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
    DecisionPack {
        #[arg(long, value_enum, default_value_t = WorkflowKind::DailyMonitor)]
        workflow: WorkflowKind,

        #[arg(long, default_value_t = 32)]
        limit: usize,

        #[arg(long, default_value_t = 6)]
        history_limit: usize,

        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,

        #[arg(long)]
        no_collect: bool,
    },
    AgentRun {
        #[arg(long, value_enum, default_value_t = WorkflowKind::DailyMonitor)]
        workflow: WorkflowKind,

        #[arg(long, default_value_t = 64)]
        limit: usize,

        #[arg(long, default_value_t = 6)]
        history_limit: usize,

        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,

        #[arg(long)]
        no_collect: bool,

        #[arg(long)]
        save: bool,

        #[arg(long, default_value = "work_docs/agent_runs")]
        out_dir: PathBuf,

        #[arg(long)]
        current_position_pct: Option<f64>,
    },
    FullAnalysis {
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,

        #[arg(long)]
        save: bool,

        #[arg(long, default_value = "work_docs/agent_runs")]
        out_dir: PathBuf,

        #[arg(long, default_value = "work_docs")]
        work_docs_dir: PathBuf,
    },
    Cron {
        #[command(subcommand)]
        command: cron_runner::CronCommand,
    },
    Summary,
    Missing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum SourceSelector {
    All,
    Market,
    BinanceSpot,
    Rates,
    Sec,
    Events,
    Status,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum AgentProfile {
    Collector,
    DataQuality,
    Source,
    Monitor,
    Financial,
    Regulatory,
    Competition,
    SpotVenue,
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

struct DecisionPackRequest<'a> {
    db: &'a Database,
    database: &'a Path,
    timeout_secs: u64,
    user_agent: String,
    workflow: WorkflowKind,
    limit: usize,
    history_limit: usize,
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
        Command::DecisionPack {
            workflow,
            limit,
            history_limit,
            format,
            no_collect,
        } => {
            run_decision_pack(DecisionPackRequest {
                db: &db,
                database: &cli.database,
                timeout_secs: cli.timeout_secs,
                user_agent: cli.user_agent,
                workflow,
                limit,
                history_limit,
                format,
                no_collect,
            })
            .await?;
        }
        Command::AgentRun {
            workflow,
            limit,
            history_limit,
            format,
            no_collect,
            save,
            out_dir,
            current_position_pct,
        } => {
            run_agent_run(
                DecisionPackRequest {
                    db: &db,
                    database: &cli.database,
                    timeout_secs: cli.timeout_secs,
                    user_agent: cli.user_agent,
                    workflow,
                    limit,
                    history_limit,
                    format,
                    no_collect,
                },
                save,
                out_dir,
                current_position_pct,
            )
            .await?;
        }
        Command::FullAnalysis {
            format,
            save,
            out_dir,
            work_docs_dir,
        } => {
            full_analysis::run(
                &db,
                &cli.database,
                full_analysis::FullAnalysisOptions {
                    format,
                    save,
                    out_dir,
                    work_docs_dir,
                },
            )?;
        }
        Command::Cron { command } => {
            cron_runner::handle(command)?;
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

async fn run_decision_pack(request: DecisionPackRequest<'_>) -> Result<()> {
    let format = request.format;
    let packet_request = workflow_packet_request(request).await?;

    workflows::print_workflow_packet(packet_request, format)
}

async fn run_agent_run(
    request: DecisionPackRequest<'_>,
    save: bool,
    out_dir: PathBuf,
    current_position_pct: Option<f64>,
) -> Result<()> {
    let format = request.format;
    let packet_request = workflow_packet_request(request).await?;
    let packet = workflows::build_workflow_packet(packet_request)?;
    agent_runner::run_agent_packet(
        packet,
        agent_runner::AgentRunOptions {
            format,
            save,
            out_dir,
            current_position_pct,
        },
    )
}

async fn workflow_packet_request<'a>(
    request: DecisionPackRequest<'a>,
) -> Result<WorkflowPacketRequest<'a>> {
    let collector_run = collect_for_workflow(&request).await?;
    Ok(WorkflowPacketRequest {
        db: request.db,
        database: request.database,
        workflow: request.workflow,
        limit: request.limit,
        history_limit: request.history_limit,
        collector_run,
    })
}

async fn collect_for_workflow(
    request: &DecisionPackRequest<'_>,
) -> Result<Option<WorkflowCollectorRun>> {
    if request.no_collect {
        return Ok(None);
    }

    let batch_id = new_batch_id("decision-pack");
    let selector_summary = workflows::selectors_for_workflow(request.workflow)
        .iter()
        .map(|selector| source_selector_label(*selector))
        .collect::<Vec<_>>()
        .join(",");
    request
        .db
        .start_collection_batch(&batch_id, "decision-pack", None, Some(&selector_summary))?;

    let client = reqwest::Client::builder()
        .use_native_tls()
        .timeout(Duration::from_secs(request.timeout_secs))
        .user_agent(request.user_agent.clone())
        .build()?;
    let ctx = CollectorContext {
        db: request.db,
        client,
        user_agent: request.user_agent.clone(),
        batch_id: Some(batch_id.clone()),
        profile: None,
        selector: None,
    };

    let mut ok_sources = 0;
    let mut warn_sources = 0;
    for selector in workflows::selectors_for_workflow(request.workflow) {
        let report = run_collectors(&ctx, *selector).await?;
        ok_sources += report.ok_sources;
        warn_sources += report.warn_sources;
    }
    request
        .db
        .finish_collection_batch(&batch_id, ok_sources, warn_sources)?;

    Ok(Some(WorkflowCollectorRun {
        batch_id,
        ok_sources,
        warn_sources,
    }))
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
        AgentProfile::Collector
        | AgentProfile::DataQuality
        | AgentProfile::Source
        | AgentProfile::Monitor
        | AgentProfile::Risk
        | AgentProfile::Autoresearch => {
            vec![SourceSelector::All]
        }
        AgentProfile::Financial => vec![
            SourceSelector::Sec,
            SourceSelector::Rates,
            SourceSelector::Market,
        ],
        AgentProfile::Regulatory => vec![SourceSelector::Events, SourceSelector::Status],
        AgentProfile::Competition => vec![SourceSelector::Market, SourceSelector::Events],
        AgentProfile::SpotVenue => vec![SourceSelector::BinanceSpot],
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
    let effective_limit = if profile == AgentProfile::SpotVenue {
        limit.max(32)
    } else {
        limit
    };
    let summary = db.summary()?;
    let recent_source_runs = db
        .recent_source_runs(effective_limit.saturating_mul(20).max(100))?
        .into_iter()
        .filter(|run| source_run_matches_profile(profile, run))
        .take(effective_limit)
        .collect::<Vec<_>>();
    let latest_observations = db
        .latest_observations()?
        .into_iter()
        .filter(|obs| observation_matches_profile(profile, obs))
        .take(effective_limit)
        .collect::<Vec<_>>();
    let recent_filings = if profile == AgentProfile::SpotVenue {
        Vec::new()
    } else {
        db.recent_filings(effective_limit)?
    };
    let recent_events = db
        .recent_events(effective_limit.saturating_mul(20).max(100))?
        .into_iter()
        .filter(|event| event_matches_profile(profile, event))
        .take(effective_limit)
        .collect::<Vec<_>>();
    let missing_items = db
        .missing_items()?
        .into_iter()
        .filter(|item| missing_item_matches_profile(profile, item))
        .take(effective_limit)
        .collect::<Vec<_>>();

    let output = AgentContextOutput {
        profile: profile_label(profile).to_string(),
        database: database.display().to_string(),
        limit: effective_limit,
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
    if profile == AgentProfile::SpotVenue {
        if run.selector.as_deref() == Some("binance-spot") {
            return true;
        }
        let text = format!("{} {}", run.source, run.url).to_ascii_lowercase();
        return text.contains("binance spot")
            || text.contains("api.binance.com/api/v3")
            || text.contains("crclbusdt")
            || text.contains("usdcusdt");
    }

    if matches!(
        profile,
        AgentProfile::Collector
            | AgentProfile::DataQuality
            | AgentProfile::Source
            | AgentProfile::Monitor
            | AgentProfile::Risk
            | AgentProfile::Autoresearch
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
            "coinglass",
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
            "coinglass",
            "circle pressroom",
        ]
        .iter()
        .any(|needle| source.contains(needle)),
        AgentProfile::SpotVenue => false,
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
        AgentProfile::Collector
        | AgentProfile::DataQuality
        | AgentProfile::Source
        | AgentProfile::Monitor
        | AgentProfile::Risk
        | AgentProfile::Autoresearch => true,
    }
}

fn observation_matches_profile(profile: AgentProfile, obs: &RecentObservation) -> bool {
    if profile == AgentProfile::SpotVenue {
        return obs.metric_code.starts_with("P1_BINANCE_SPOT_")
            || obs.category.starts_with("binance_spot");
    }

    if matches!(
        profile,
        AgentProfile::Collector
            | AgentProfile::DataQuality
            | AgentProfile::Source
            | AgentProfile::Monitor
            | AgentProfile::Risk
            | AgentProfile::Autoresearch
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
                | "platform_metrics"
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
                | "institutional_ownership"
                | "peg_check"
                | "peg_liquidity"
                | "equity_market"
        ),
        AgentProfile::SpotVenue => false,
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
                | "platform_metrics"
        ),
        AgentProfile::Collector
        | AgentProfile::DataQuality
        | AgentProfile::Source
        | AgentProfile::Monitor
        | AgentProfile::Risk
        | AgentProfile::Autoresearch => true,
    }
}

fn event_matches_profile(profile: AgentProfile, event: &RecentEvent) -> bool {
    if matches!(
        profile,
        AgentProfile::Collector
            | AgentProfile::DataQuality
            | AgentProfile::Source
            | AgentProfile::Monitor
            | AgentProfile::Risk
            | AgentProfile::Autoresearch
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
        AgentProfile::SpotVenue => false,
        AgentProfile::Collector
        | AgentProfile::DataQuality
        | AgentProfile::Source
        | AgentProfile::Monitor
        | AgentProfile::Risk
        | AgentProfile::Autoresearch => true,
    }
}

fn missing_item_matches_profile(profile: AgentProfile, item: &models::MissingItem) -> bool {
    if matches!(
        profile,
        AgentProfile::Collector
            | AgentProfile::DataQuality
            | AgentProfile::Source
            | AgentProfile::Risk
            | AgentProfile::Autoresearch
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
                || code.starts_with("P1_CPN")
                || code.starts_with("P1_ARC")
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
                || code.starts_with("P2_CRCL_INSTITUTIONAL")
                || code.starts_with("P0_CURVE")
                || code == "P0_CIRCLE_MINTED_REDEEMED"
        }
        AgentProfile::SpotVenue => {
            code.starts_with("P1_BINANCE_SPOT_") || collector.contains("binance")
        }
        AgentProfile::Platform => {
            code.starts_with("P2_CIRCLE")
                || code.starts_with("P1_COINBASE")
                || code == "P1_USDC_ADJUSTED_TRANSFER_VOLUME"
                || code.starts_with("P1_DEFI")
                || code.starts_with("P1_TOKENIZED")
                || code.starts_with("P1_CPN")
                || code.starts_with("P1_ARC")
                || collector.contains("circle")
        }
        AgentProfile::Collector
        | AgentProfile::DataQuality
        | AgentProfile::Source
        | AgentProfile::Monitor
        | AgentProfile::Risk
        | AgentProfile::Autoresearch => true,
    }
}

fn selector_labels_for_profile(profile: AgentProfile) -> &'static [&'static str] {
    match profile {
        AgentProfile::Collector
        | AgentProfile::DataQuality
        | AgentProfile::Source
        | AgentProfile::Monitor
        | AgentProfile::Risk
        | AgentProfile::Autoresearch => &["all"],
        AgentProfile::Financial => &["sec", "rates", "market"],
        AgentProfile::Regulatory => &["events", "status"],
        AgentProfile::Competition => &["market", "events"],
        AgentProfile::SpotVenue => &["binance-spot"],
        AgentProfile::Platform => &["events", "sec", "market"],
    }
}

fn profile_label(profile: AgentProfile) -> &'static str {
    match profile {
        AgentProfile::Collector => "collector",
        AgentProfile::DataQuality => "data-quality",
        AgentProfile::Source => "source",
        AgentProfile::Monitor => "monitor",
        AgentProfile::Financial => "financial",
        AgentProfile::Regulatory => "regulatory",
        AgentProfile::Competition => "competition",
        AgentProfile::SpotVenue => "spot-venue",
        AgentProfile::Platform => "platform",
        AgentProfile::Risk => "risk",
        AgentProfile::Autoresearch => "autoresearch",
    }
}
