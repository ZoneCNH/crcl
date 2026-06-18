use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::OutputFormat;
use crate::db::{Database, RecentObservation};

const WORKFLOW_CATEGORIES: &[(&str, &str)] = &[
    ("monitoring", "monitoring"),
    ("daily-monitor", "daily-monitor"),
    ("weekly-review", "weekly-review"),
    ("valuation-decision", "valuation-decision"),
    ("quarterly-earnings", "quarterly-earnings"),
    ("framework-review", "framework-review"),
];

#[derive(Debug)]
pub struct FullAnalysisOptions {
    pub format: OutputFormat,
    pub save: bool,
    pub out_dir: PathBuf,
    pub work_docs_dir: PathBuf,
}

#[derive(Debug, Serialize)]
struct FullAnalysisOutput {
    generated_at: String,
    database: String,
    conclusion: String,
    action: String,
    data_quality_blocked: bool,
    data_quality_summary: String,
    confidence: String,
    workflow_artifacts: Vec<WorkflowArtifact>,
    key_metrics: Vec<MetricLine>,
    source_warnings: Vec<String>,
    missing_info: Vec<String>,
    contradictions: Vec<String>,
    next_actions: Vec<String>,
    saved_paths: Vec<String>,
}

#[derive(Debug, Serialize)]
struct WorkflowArtifact {
    workflow: String,
    category: String,
    final_path: Option<String>,
    meta_path: Option<String>,
    started_at: Option<String>,
    completed_at: Option<String>,
    exit_status: Option<i32>,
    no_collect: Option<bool>,
    conclusion: Option<String>,
    data_quality: Option<String>,
    data_quality_blocked: bool,
}

#[derive(Debug, Serialize)]
struct MetricLine {
    item: String,
    value: String,
    observed_at: String,
    source: String,
    source_url: String,
    interpretation: String,
}

#[derive(Debug, Deserialize)]
struct CronMeta {
    started_at: Option<String>,
    completed_at: Option<String>,
    exit_status: Option<i32>,
    no_collect: Option<bool>,
}

pub fn run(db: &Database, database: &Path, options: FullAnalysisOptions) -> Result<()> {
    let generated_at = Utc::now().to_rfc3339();
    let artifacts = latest_workflow_artifacts(&options.work_docs_dir)?;
    let observations = db
        .latest_observations()?
        .into_iter()
        .map(|obs| (obs.metric_code.clone(), obs))
        .collect::<BTreeMap<_, _>>();
    let key_metrics = key_metrics(&observations);
    let source_warnings = source_warnings(db)?;
    let missing_info = missing_info(db)?;
    let data_quality_blocked = artifacts
        .iter()
        .any(|artifact| artifact.data_quality_blocked);
    let data_quality_summary = data_quality_summary(
        &artifacts,
        data_quality_blocked,
        &source_warnings,
        &missing_info,
    );
    let conclusion = synthesize_conclusion(&observations, data_quality_blocked);
    let action = synthesize_action(&conclusion, data_quality_blocked);
    let confidence = if data_quality_blocked {
        "low"
    } else if !source_warnings.is_empty() || !missing_info.is_empty() {
        "medium"
    } else {
        "high"
    }
    .to_string();

    let mut output = FullAnalysisOutput {
        generated_at,
        database: database.display().to_string(),
        conclusion,
        action,
        data_quality_blocked,
        data_quality_summary,
        confidence,
        workflow_artifacts: artifacts,
        key_metrics,
        source_warnings,
        missing_info,
        contradictions: contradictions(),
        next_actions: next_actions(),
        saved_paths: Vec::new(),
    };

    if options.save {
        let paths = save_output(&mut output, &options.out_dir)?;
        output.saved_paths = paths;
        rewrite_saved_output(&output, &options.out_dir)?;
    }

    match options.format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&output)?),
        OutputFormat::Text => print!("{}", render_markdown(&output)),
    }

    Ok(())
}

fn latest_workflow_artifacts(work_docs_dir: &Path) -> Result<Vec<WorkflowArtifact>> {
    WORKFLOW_CATEGORIES
        .iter()
        .map(|(workflow, category)| latest_workflow_artifact(work_docs_dir, workflow, category))
        .collect()
}

fn latest_workflow_artifact(
    work_docs_dir: &Path,
    workflow: &str,
    category: &str,
) -> Result<WorkflowArtifact> {
    let dir = work_docs_dir.join(category);
    let suffix = format!("-{workflow}-codex-final.md");
    let final_path = latest_matching_file(&dir, &suffix)?;
    let mut artifact = WorkflowArtifact {
        workflow: workflow.to_string(),
        category: category.to_string(),
        final_path: final_path.as_ref().map(|path| relative(path)),
        meta_path: None,
        started_at: None,
        completed_at: None,
        exit_status: None,
        no_collect: None,
        conclusion: None,
        data_quality: None,
        data_quality_blocked: false,
    };

    let Some(final_path) = final_path else {
        return Ok(artifact);
    };

    let content = fs::read_to_string(&final_path)
        .with_context(|| format!("failed to read {}", final_path.display()))?;
    artifact.conclusion = extract_line_after(&content, "结论");
    artifact.data_quality = extract_data_quality_line(&content);
    artifact.data_quality_blocked = text_has_data_quality_block(&content);

    if let Some(meta_path) = meta_path_for_final(&final_path, workflow)
        && meta_path.exists()
    {
        artifact.meta_path = Some(relative(&meta_path));
        let meta_text = fs::read_to_string(&meta_path)
            .with_context(|| format!("failed to read {}", meta_path.display()))?;
        if let Ok(meta) = serde_json::from_str::<CronMeta>(&meta_text) {
            artifact.started_at = meta.started_at;
            artifact.completed_at = meta.completed_at;
            artifact.exit_status = meta.exit_status;
            artifact.no_collect = meta.no_collect;
        }
    }

    Ok(artifact)
}

fn latest_matching_file(dir: &Path, suffix: &str) -> Result<Option<PathBuf>> {
    if !dir.exists() {
        return Ok(None);
    }
    let mut matches = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(suffix))
        })
        .collect::<Vec<_>>();
    matches.sort();
    Ok(matches.pop())
}

fn meta_path_for_final(final_path: &Path, workflow: &str) -> Option<PathBuf> {
    let file_name = final_path.file_name()?.to_str()?;
    let suffix = format!("-{workflow}-codex-final.md");
    let prefix = file_name.strip_suffix(&suffix)?;
    Some(final_path.with_file_name(format!("{prefix}-{workflow}-cron-meta.json")))
}

fn extract_line_after(content: &str, needle: &str) -> Option<String> {
    let lines = content.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate().take(80) {
        if !line
            .to_ascii_lowercase()
            .contains(&needle.to_ascii_lowercase())
        {
            continue;
        }
        let cleaned = clean_markdown(line);
        if cleaned.chars().count() > needle.chars().count() + 6 {
            return Some(truncate(&cleaned, 220));
        }
        if let Some(next) = lines[index + 1..].iter().find_map(|next| {
            let cleaned = clean_markdown(next);
            (!cleaned.is_empty()).then_some(cleaned)
        }) {
            return Some(truncate(&next, 220));
        }
    }
    None
}

fn extract_data_quality_line(content: &str) -> Option<String> {
    let lines = content.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate().take(120) {
        let cleaned = clean_markdown(line);
        let lower = cleaned.to_ascii_lowercase();
        let is_header = matches!(lower.as_str(), "data-quality" | "data quality")
            || lower.starts_with("data-quality：")
            || lower.starts_with("data-quality:")
            || lower.starts_with("data-quality 是否阻断")
            || lower.starts_with("data quality：")
            || lower.starts_with("data quality:");
        if !is_header {
            continue;
        }
        if cleaned.chars().count() > "data-quality".chars().count() + 6 {
            return Some(truncate(&cleaned, 220));
        }
        if let Some(next) = lines[index + 1..].iter().find_map(|next| {
            let cleaned = clean_markdown(next);
            (!cleaned.is_empty()).then_some(cleaned)
        }) {
            return Some(truncate(&next, 220));
        }
    }
    None
}

fn text_has_data_quality_block(content: &str) -> bool {
    [
        "data-quality：**阻断",
        "Data Quality**\n\ndata-quality：**阻断",
        "data-quality 是否阻断：是",
        "阻断完整 valuation",
        "阻断完整估值",
        "data-quality 阻断完整",
    ]
    .iter()
    .any(|needle| content.contains(needle))
}

fn key_metrics(observations: &BTreeMap<String, RecentObservation>) -> Vec<MetricLine> {
    let mut metrics = Vec::new();
    metrics.push(metric_from_obs(
        observations,
        "USDC 规模",
        "P0_USDC_CIRCULATING_SUPPLY",
        "核心规模锚；连续净赎回会压制增强。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "USDC 储备",
        "P0_CIRCLE_USDC_TOTAL_RESERVES",
        "信用层锚；储备低于流通量会触发降级复核。",
    ));
    metrics.push(coverage_metric(observations));
    metrics.push(metric_from_obs(
        observations,
        "USDC 7D 净发行",
        "P0_CIRCLE_USDC_7D_NET_ISSUANCE",
        "短期增长质量锚；负值进入赎回压力观察。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "USDC 30D 净发行",
        "P0_CIRCLE_USDC_30D_NET_ISSUANCE",
        "月度增长质量锚；持续为负不支持增强。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "USDC/USD",
        "P0_USDC_USD_PRICE",
        "信用层锚；持续折价超过 0.1% 优先触发风险复核。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "Reserve income",
        "P2_CIRCLE_RESERVE_INCOME",
        "储备收益底座；需结合 Average USDC 和利率判断质量。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "RLDC",
        "P2_CIRCLE_RLDC",
        "分销后留存收入，是估值和财务桥核心。",
    ));
    metrics.push(rldc_main_margin_metric(observations));
    metrics.push(metric_from_obs(
        observations,
        "Other revenue share",
        "P2_CIRCLE_OTHER_REVENUE_SHARE",
        "平台化财务验证；超过 10% 才进入更强验证。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "USDC 市占率",
        "P1_USDC_MARKET_CAP_SHARE",
        "竞争结构锚；需和 USDC/USDT ratio 同向验证。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "USDC/USDT ratio",
        "P1_USDC_USDT_RATIO",
        "相对 USDT 的份额强弱指标。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "交易所 USDC 余额",
        "P1_EXCHANGE_USDC_BALANCES",
        "交易所流动性锚；需和市占率、净发行同向验证。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "交易所 USDC 30D 变化",
        "P1_EXCHANGE_USDC_BALANCE_30D_CHANGE",
        "交易所 USDC 库存趋势；持续下降会降低竞争/流动性置信度。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "交易所 USDC top3 集中度",
        "P1_EXCHANGE_USDC_TOP3_CONCENTRATION",
        "交易所余额集中度；过高时需要单独看 Coinbase/Binance 结构。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "CPN annualized TPV",
        "P1_CPN_ANNUALIZED_TPV",
        "平台化线索；不能替代 take rate 和收入确认。",
    ));
    metrics.push(metric_from_obs(
        observations,
        "Arc 状态",
        "P1_ARC_PUBLIC_NETWORK_STATUS",
        "平台化线索；测试网不能当作收入验证。",
    ));
    metrics.push(valuation_multiple_metric(observations));
    metrics
}

fn metric_from_obs(
    observations: &BTreeMap<String, RecentObservation>,
    item: &str,
    code: &str,
    interpretation: &str,
) -> MetricLine {
    observations
        .get(code)
        .map(|obs| MetricLine {
            item: item.to_string(),
            value: display_value(obs.value_num, obs.value_text.as_deref(), &obs.unit),
            observed_at: obs.observed_at.clone(),
            source: obs.source.clone(),
            source_url: obs.source_url.clone(),
            interpretation: interpretation.to_string(),
        })
        .unwrap_or_else(|| missing_metric(item, code, interpretation))
}

fn coverage_metric(observations: &BTreeMap<String, RecentObservation>) -> MetricLine {
    let supply = num(observations, "P0_USDC_CIRCULATING_SUPPLY");
    let reserves = num(observations, "P0_CIRCLE_USDC_TOTAL_RESERVES");
    match (supply, reserves) {
        (Some(supply), Some(reserves)) if supply > 0.0 => MetricLine {
            item: "储备覆盖率".to_string(),
            value: format!("{:.2}%", reserves / supply * 100.0),
            observed_at: observed_pair(
                observations,
                "P0_USDC_CIRCULATING_SUPPLY",
                "P0_CIRCLE_USDC_TOTAL_RESERVES",
            ),
            source: "derived from Circle Transparency".to_string(),
            source_url: source_pair(
                observations,
                "P0_USDC_CIRCULATING_SUPPLY",
                "P0_CIRCLE_USDC_TOTAL_RESERVES",
            ),
            interpretation: "高于 100% 支持信用层未坏；低于 100% 触发储备异常复核。".to_string(),
        },
        _ => missing_metric(
            "储备覆盖率",
            "derived",
            "缺 USDC supply 或 reserves，无法计算。",
        ),
    }
}

fn rldc_main_margin_metric(observations: &BTreeMap<String, RecentObservation>) -> MetricLine {
    let rldc = num(observations, "P2_CIRCLE_RLDC");
    let reserve_income = num(observations, "P2_CIRCLE_RESERVE_INCOME");
    match (rldc, reserve_income) {
        (Some(rldc), Some(reserve_income)) if reserve_income > 0.0 => MetricLine {
            item: "RLDC 主口径 margin".to_string(),
            value: format!("{:.2}%", rldc / reserve_income * 100.0),
            observed_at: observed_pair(observations, "P2_CIRCLE_RLDC", "P2_CIRCLE_RESERVE_INCOME"),
            source: "derived from SEC filing".to_string(),
            source_url: source_pair(observations, "P2_CIRCLE_RLDC", "P2_CIRCLE_RESERVE_INCOME"),
            interpretation:
                "主告警口径是 RLDC / reserve income；低于 38% 触发降级，高于 40% 支持财务桥。"
                    .to_string(),
        },
        _ => missing_metric(
            "RLDC 主口径 margin",
            "derived",
            "缺 RLDC 或 reserve income，不能套用 38% 告警线。",
        ),
    }
}

fn valuation_multiple_metric(observations: &BTreeMap<String, RecentObservation>) -> MetricLine {
    let price = num(observations, "P0_CRCL_PRICE");
    let shares = num(observations, "P2_CRCL_DILUTED_SHARES_OUTSTANDING")
        .or_else(|| num(observations, "P2_CRCL_SHARES_OUTSTANDING"));
    let rldc = num(observations, "P2_CIRCLE_RLDC");
    match (price, shares, rldc) {
        (Some(price), Some(shares), Some(rldc)) if rldc > 0.0 => {
            let multiple = price * shares / (rldc * 4.0);
            MetricLine {
                item: "Market cap / annualized RLDC".to_string(),
                value: format!("{multiple:.1}x"),
                observed_at: observed_pair(observations, "P0_CRCL_PRICE", "P2_CIRCLE_RLDC"),
                source: "derived from market price, diluted shares and RLDC".to_string(),
                source_url: source_pair(observations, "P0_CRCL_PRICE", "P2_CIRCLE_RLDC"),
                interpretation:
                    "用于 Base/Bull/Bear 估值区间复核；data-quality 阻断时不得直接给强仓位动作。"
                        .to_string(),
            }
        }
        _ => missing_metric(
            "Market cap / annualized RLDC",
            "derived",
            "缺价格、稀释股数或 RLDC，无法计算估值倍数。",
        ),
    }
}

fn missing_metric(item: &str, source: &str, interpretation: &str) -> MetricLine {
    MetricLine {
        item: item.to_string(),
        value: "missing".to_string(),
        observed_at: "-".to_string(),
        source: source.to_string(),
        source_url: "-".to_string(),
        interpretation: interpretation.to_string(),
    }
}

fn source_warnings(db: &Database) -> Result<Vec<String>> {
    let mut seen = BTreeSet::new();
    let mut warnings = Vec::new();
    for run in db
        .recent_source_runs(400)?
        .into_iter()
        .filter(|run| run.status != "ok")
    {
        let key = format!("{}|{}|{}", run.source, run.status, run.url);
        if !seen.insert(key) {
            continue;
        }
        warnings.push(format!(
            "{} | status={} | http={} | fetched_at={} | batch={} | {} | {}",
            run.source,
            run.status,
            run.http_status
                .map(|status| status.to_string())
                .unwrap_or_else(|| "-".to_string()),
            run.fetched_at,
            run.batch_id.as_deref().unwrap_or("-"),
            run.error.as_deref().unwrap_or("-"),
            run.url
        ));
        if warnings.len() >= 16 {
            break;
        }
    }
    Ok(warnings)
}

fn missing_info(db: &Database) -> Result<Vec<String>> {
    Ok(db
        .missing_items()?
        .into_iter()
        .take(24)
        .map(|item| {
            format!(
                "{} {} | {} | {} | {}",
                item.priority, item.metric_code, item.metric_name, item.reason, item.source_hint
            )
        })
        .collect())
}

fn data_quality_summary(
    artifacts: &[WorkflowArtifact],
    blocked: bool,
    source_warnings: &[String],
    missing_info: &[String],
) -> String {
    let blocked_workflows = artifacts
        .iter()
        .filter(|artifact| artifact.data_quality_blocked)
        .map(|artifact| artifact.workflow.as_str())
        .collect::<Vec<_>>();
    if blocked {
        format!(
            "部分阻断：{}。完整总分析仍可输出，但不得给增强、Bull 或强仓位动作。",
            blocked_workflows.join(", ")
        )
    } else if !source_warnings.is_empty() || !missing_info.is_empty() {
        "不阻断主框架，但必须降置信度并保留 source_warnings / missing_info。".to_string()
    } else {
        "未发现阻断项，可按固定流程进入主判断。".to_string()
    }
}

fn synthesize_conclusion(
    observations: &BTreeMap<String, RecentObservation>,
    data_quality_blocked: bool,
) -> String {
    let usdc_price = num(observations, "P0_USDC_USD_PRICE");
    let supply = num(observations, "P0_USDC_CIRCULATING_SUPPLY");
    let reserves = num(observations, "P0_CIRCLE_USDC_TOTAL_RESERVES");
    let coverage_ok = match (supply, reserves) {
        (Some(supply), Some(reserves)) if supply > 0.0 => reserves / supply >= 1.0,
        _ => true,
    };
    let rldc_margin = match (
        num(observations, "P2_CIRCLE_RLDC"),
        num(observations, "P2_CIRCLE_RESERVE_INCOME"),
    ) {
        (Some(rldc), Some(reserve_income)) if reserve_income > 0.0 => {
            Some(rldc / reserve_income * 100.0)
        }
        _ => num(observations, "P2_CIRCLE_RLDC_MARGIN"),
    };
    let net_issuance_negative = num(observations, "P0_CIRCLE_USDC_7D_NET_ISSUANCE")
        .is_some_and(|value| value < 0.0)
        || num(observations, "P0_CIRCLE_USDC_30D_NET_ISSUANCE").is_some_and(|value| value < 0.0);
    let other_revenue_share =
        num(observations, "P2_CIRCLE_OTHER_REVENUE_SHARE").unwrap_or_default();

    if usdc_price.is_some_and(|price| price < 0.999) || !coverage_ok {
        "降级 / 信用层复核".to_string()
    } else if rldc_margin.is_some_and(|margin| margin < 38.0) {
        "降级 / Bear 复核".to_string()
    } else if !data_quality_blocked
        && rldc_margin.is_some_and(|margin| margin >= 40.0)
        && !net_issuance_negative
        && other_revenue_share >= 10.0
    {
        "增强 / Bull 候选".to_string()
    } else {
        "观察 / Base".to_string()
    }
}

fn synthesize_action(conclusion: &str, data_quality_blocked: bool) -> String {
    if conclusion.contains("降级") {
        "先按信用层或 Bear 触发条件复核，冻结乐观情景和平台化上修。".to_string()
    } else if conclusion.contains("增强") {
        "进入增强复核，但仍需确认监管、竞争和平台化收入证据没有反证。".to_string()
    } else if data_quality_blocked {
        "维持观察 / 低置信度 Base；补齐阻断源后重跑 valuation-decision 和 full-analysis。"
            .to_string()
    } else {
        "维持观察 / Base；等待 USDC 净发行、competition score、RLDC 和平台化收入共振。".to_string()
    }
}

fn contradictions() -> Vec<String> {
    vec![
        "USDC/USD 持续折价超过 0.1%、储备覆盖异常、核心银行通道中断或正式执法行动，覆盖财务桥结论。".to_string(),
        "RLDC 主口径低于 38% 且无一次性解释，触发财务桥降级。".to_string(),
        "competition score 跌破 40、USDC 连续净赎回扩大或 USDC/USDT ratio 继续走弱，触发竞争降级复核。".to_string(),
        "Other revenue share 未超过 10% 或 CPN/Arc 缺收入归属时，不能切换到支付基础设施股框架。".to_string(),
        "官方监管源不可达或缺 section-level 原文解析时，不能把监管结论写成已确认无风险。".to_string(),
    ]
}

fn next_actions() -> Vec<String> {
    vec![
        "持续用 CoinGlass 前台 fallback 监控交易所 USDC 余额；如配置 `COINGLASS_API_KEY`，作为官方 Open API 交叉验证。".to_string(),
        "复核 Treasury / FinCEN / OCC / OFAC / FDIC / SEC / CFTC 官方源，补 section-level 监管解析。".to_string(),
        "人工确认 Base status incident 的 component、duration、resolution 和是否影响 USDC 转账/赎回。".to_string(),
        "在下一次财报后核对 `RLDC / reserve income` 主口径，避免用全口径 margin 套 38% 告警线。".to_string(),
        "下一轮 Circle Transparency 更新后复核 USDC circulation、reserves、7D / 30D net issuance。".to_string(),
        "周度复盘补 D4/D5 的 4 周趋势、borrow utilization、活跃地址和收益型稳定币 AUM。".to_string(),
    ]
}

fn render_markdown(output: &FullAnalysisOutput) -> String {
    let mut lines = vec![
        "# CRCL 完整跑批总分析报告".to_string(),
        String::new(),
        format!("generated_at={}", output.generated_at),
        format!("database={}", output.database),
        format!("confidence={}", output.confidence),
        "说明：本报告由 `full-analysis` 总分析 agent 根据最新分类 workflow 产物和本地数据库生成，不构成个性化投资建议。".to_string(),
        String::new(),
        "## 1. 结论".to_string(),
        String::new(),
        format!("结论：**{}**。", output.conclusion),
        output.action.clone(),
        String::new(),
        "## 2. Data Quality".to_string(),
        String::new(),
        format!(
            "data-quality 是否阻断：{}。",
            if output.data_quality_blocked {
                "是，阻断完整乐观/仓位判断"
            } else {
                "否"
            }
        ),
        output.data_quality_summary.clone(),
        String::new(),
        "## 3. Workflow 产物".to_string(),
        String::new(),
        "| Workflow | exit | no_collect | 结论摘录 | data-quality 摘录 | final |".to_string(),
        "| --- | ---: | --- | --- | --- | --- |".to_string(),
    ];

    for artifact in &output.workflow_artifacts {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            artifact.workflow,
            artifact
                .exit_status
                .map(|status| status.to_string())
                .unwrap_or_else(|| "-".to_string()),
            artifact
                .no_collect
                .map(|value| value.to_string())
                .unwrap_or_else(|| "-".to_string()),
            artifact
                .conclusion
                .as_deref()
                .map(escape_table)
                .unwrap_or_else(|| "-".to_string()),
            artifact
                .data_quality
                .as_deref()
                .map(escape_table)
                .unwrap_or_else(|| "-".to_string()),
            artifact.final_path.as_deref().unwrap_or("missing")
        ));
    }

    lines.extend([
        String::new(),
        "## 4. 核心指标".to_string(),
        String::new(),
        "| 跟踪项 | 数值 | 数据日期 | 来源 | 判断 |".to_string(),
        "| --- | ---: | --- | --- | --- |".to_string(),
    ]);
    for metric in &output.key_metrics {
        lines.push(format!(
            "| {} | {} | {} | {} | {} |",
            escape_table(&metric.item),
            escape_table(&metric.value),
            escape_table(&metric.observed_at),
            escape_table(&metric.source),
            escape_table(&metric.interpretation)
        ));
    }

    lines.extend([
        String::new(),
        "## 5. 统一判断".to_string(),
        String::new(),
        "财务：RLDC 主口径高于 40% 时支持 Base 财务桥；但 Average USDC、净发行和平台化收入未共振时，不升级为增强。".to_string(),
        String::new(),
        "链上与市场：USDC 规模、净发行、市占率和 USDC/USDT ratio 是增长质量主线；USDC/USD 和储备覆盖是信用层硬门槛。".to_string(),
        String::new(),
        "监管：GENIUS Act、OCC、Treasury、FinCEN、OFAC、FDIC、SEC/CFTC 的正式规则或执法优先级高于平台化叙事。".to_string(),
        String::new(),
        "竞争：competition score 未穿越 75 不写增强，跌破 40 触发降级复核；缺失维度必须保留 missing_info。".to_string(),
        String::new(),
        "平台化：Other revenue、CPN 收费模式和 Arc 主网收入归属是切换框架的必要证据；测试网和 TPV 只能作线索。".to_string(),
        String::new(),
        "估值/风险：data-quality 阻断时只允许低置信度 Base 或观察，不给强仓位动作。".to_string(),
        String::new(),
        "## 6. Source Warnings".to_string(),
    ]);
    push_list(
        &mut lines,
        &output.source_warnings,
        "本轮未发现近期失败源。",
    );

    lines.extend([String::new(), "## 7. Missing Info".to_string()]);
    push_list(&mut lines, &output.missing_info, "暂无 missing_info。");

    lines.extend([String::new(), "## 8. 反证 / 解除条件".to_string()]);
    push_list(&mut lines, &output.contradictions, "暂无覆盖性反证。");

    lines.extend([String::new(), "## 9. 下一步".to_string()]);
    push_list(&mut lines, &output.next_actions, "按既定定时任务继续。");

    lines.extend([String::new(), "## 10. 已保存路径".to_string()]);
    push_list(&mut lines, &output.saved_paths, "本次未保存。");

    lines.push(String::new());
    lines.join("\n")
}

fn save_output(output: &mut FullAnalysisOutput, out_dir: &Path) -> Result<Vec<String>> {
    fs::create_dir_all(out_dir)?;
    let stamp = output.generated_at.replace([':', '.'], "-");
    let base = out_dir.join(format!("{stamp}-full-analysis"));
    let markdown_path = base.with_extension("md");
    let json_path = base.with_extension("json");
    output.saved_paths = vec![relative(&markdown_path), relative(&json_path)];
    fs::write(&markdown_path, render_markdown(output))?;
    fs::write(
        &json_path,
        format!("{}\n", serde_json::to_string_pretty(output)?),
    )?;
    eprintln!(
        "saved {} and {}",
        markdown_path.display(),
        json_path.display()
    );
    Ok(output.saved_paths.clone())
}

fn rewrite_saved_output(output: &FullAnalysisOutput, out_dir: &Path) -> Result<()> {
    let stamp = output.generated_at.replace([':', '.'], "-");
    let base = out_dir.join(format!("{stamp}-full-analysis"));
    fs::write(base.with_extension("md"), render_markdown(output))?;
    fs::write(
        base.with_extension("json"),
        format!("{}\n", serde_json::to_string_pretty(output)?),
    )?;
    Ok(())
}

fn push_list(lines: &mut Vec<String>, items: &[String], empty: &str) {
    if items.is_empty() {
        lines.push(format!("- {empty}"));
        return;
    }
    for item in items {
        lines.push(format!("- {item}"));
    }
}

fn num(observations: &BTreeMap<String, RecentObservation>, code: &str) -> Option<f64> {
    observations.get(code).and_then(|obs| obs.value_num)
}

fn observed_pair(
    observations: &BTreeMap<String, RecentObservation>,
    left: &str,
    right: &str,
) -> String {
    let left = observations
        .get(left)
        .map(|obs| obs.observed_at.as_str())
        .unwrap_or("-");
    let right = observations
        .get(right)
        .map(|obs| obs.observed_at.as_str())
        .unwrap_or("-");
    if left == right {
        left.to_string()
    } else {
        format!("{left} / {right}")
    }
}

fn source_pair(
    observations: &BTreeMap<String, RecentObservation>,
    left: &str,
    right: &str,
) -> String {
    let left = observations
        .get(left)
        .map(|obs| obs.source_url.as_str())
        .unwrap_or("-");
    let right = observations
        .get(right)
        .map(|obs| obs.source_url.as_str())
        .unwrap_or("-");
    if left == right {
        left.to_string()
    } else {
        format!("{left} ; {right}")
    }
}

fn display_value(value_num: Option<f64>, value_text: Option<&str>, unit: &str) -> String {
    let unit_lower = unit.to_ascii_lowercase();
    match (value_num, value_text) {
        (Some(value), _) if unit_lower.starts_with("usd") && value.abs() >= 1_000_000_000.0 => {
            format!("{:.2}B {unit}", value / 1_000_000_000.0)
        }
        (Some(value), _) if unit_lower.starts_with("usd") && value.abs() >= 1_000_000.0 => {
            format!("{:.2}M {unit}", value / 1_000_000.0)
        }
        (Some(value), _) if unit.eq_ignore_ascii_case("percent") => format!("{value:.3}%"),
        (Some(value), _) if unit.eq_ignore_ascii_case("ratio") => format!("{value:.4}"),
        (Some(value), _) if unit.eq_ignore_ascii_case("shares") && value.abs() >= 1_000_000.0 => {
            format!("{:.3}M shares", value / 1_000_000.0)
        }
        (Some(value), _) => format!("{value:.4} {unit}"),
        (None, Some(value)) => {
            if unit.is_empty() {
                value.to_string()
            } else {
                format!("{value} {unit}")
            }
        }
        (None, None) => format!("- {unit}"),
    }
}

fn clean_markdown(line: &str) -> String {
    line.trim()
        .trim_start_matches('#')
        .trim()
        .replace(['*', '`'], "")
        .replace("<br>", " ")
        .trim()
        .to_string()
}

fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut output = text.chars().take(max_chars).collect::<String>();
    output.push_str("...");
    output
}

fn escape_table(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', " ")
}

fn relative(path: &Path) -> String {
    std::env::current_dir()
        .ok()
        .and_then(|root| path.strip_prefix(root).ok().map(PathBuf::from))
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| path.display().to_string())
}
