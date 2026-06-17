use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::OutputFormat;
use crate::db::{RecentObservation, RecentSourceRun};
use crate::workflows::WorkflowPacket;

#[derive(Debug, Serialize)]
struct AgentRunOutput {
    workflow: String,
    generated_at: String,
    packet_generated_at: String,
    database: String,
    conclusion: String,
    action: String,
    data_quality_blocked: bool,
    confidence: String,
    key_evidence: Vec<String>,
    failed_sources: Vec<String>,
    stale_items: Vec<String>,
    missing_info: Vec<String>,
    contradictions: Vec<String>,
    next_review_trigger: String,
    saved_paths: Vec<String>,
}

#[derive(Debug)]
pub struct AgentRunOptions {
    pub format: OutputFormat,
    pub save: bool,
    pub out_dir: PathBuf,
    pub current_position_pct: Option<f64>,
}

#[derive(Debug)]
struct HealthSignals {
    failed_sources: Vec<String>,
    official_failed_sources: Vec<String>,
    stale_items: Vec<String>,
    missing_info: Vec<String>,
    critical_missing: Vec<String>,
    key_evidence: Vec<String>,
}

pub fn run_agent_packet(packet: WorkflowPacket, options: AgentRunOptions) -> Result<()> {
    let signals = health_signals(&packet);
    let blocked = data_quality_blocked(&packet, &signals);
    let warnings = has_warnings(&packet, &signals);
    let (conclusion, action, next_review_trigger, contradictions) = synthesize(
        &packet,
        &signals,
        blocked,
        warnings,
        options.current_position_pct,
    );

    let mut output = AgentRunOutput {
        workflow: packet.workflow.clone(),
        generated_at: Utc::now().to_rfc3339(),
        packet_generated_at: packet.generated_at.clone(),
        database: packet.database.clone(),
        conclusion,
        action,
        data_quality_blocked: blocked,
        confidence: confidence(blocked, warnings, &signals),
        key_evidence: signals.key_evidence,
        failed_sources: signals.failed_sources,
        stale_items: signals.stale_items,
        missing_info: unique(signals.missing_info),
        contradictions,
        next_review_trigger,
        saved_paths: Vec::new(),
    };

    let text = render_markdown(&output, &packet);
    if options.save {
        let paths = save_output(&output, &text, &options.out_dir)?;
        output.saved_paths = paths;
    }

    match options.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputFormat::Text => {
            let text = render_markdown(&output, &packet);
            print!("{text}");
        }
    }

    Ok(())
}

fn health_signals(packet: &WorkflowPacket) -> HealthSignals {
    let batch_id = packet
        .collector_run
        .as_ref()
        .map(|run| run.batch_id.as_str());
    let mut failed_sources = Vec::new();
    let mut official_failed_sources = Vec::new();

    for run in packet.evidence.recent_source_runs.iter().filter(|run| {
        run.status != "ok" && batch_id.is_none_or(|batch| run.batch_id.as_deref() == Some(batch))
    }) {
        let item = source_run_line(run);
        if is_official_or_critical_source(run) {
            official_failed_sources.push(item.clone());
        }
        failed_sources.push(item);
    }

    let mut stale_items = Vec::new();
    for &(code, max_days) in stale_thresholds() {
        if let Some(obs) = packet
            .evidence
            .latest_observations
            .iter()
            .find(|obs| obs.metric_code == code)
        {
            let age = days_old(&obs.observed_at, &packet.generated_at);
            if age > max_days {
                stale_items.push(format!("{code}: {age} 天未刷新，超过 {max_days} 天阈值"));
            }
        }
    }

    let missing_info = packet
        .evidence
        .missing_items
        .iter()
        .map(|item| {
            format!(
                "{}: {} ({})",
                item.metric_code, item.reason, item.source_hint
            )
        })
        .collect::<Vec<_>>();
    let critical_missing = missing_info
        .iter()
        .filter(|item| {
            item.starts_with("P0_")
                || item.contains("SOURCE_BLOCKED")
                || item.contains("SOURCE_UNREACHABLE")
                || item.contains("USDC")
                || item.contains("RLDC")
        })
        .cloned()
        .collect::<Vec<_>>();

    let key_evidence = key_evidence(packet);

    HealthSignals {
        failed_sources,
        official_failed_sources,
        stale_items,
        missing_info,
        critical_missing,
        key_evidence,
    }
}

fn data_quality_blocked(packet: &WorkflowPacket, signals: &HealthSignals) -> bool {
    packet
        .collector_run
        .as_ref()
        .is_some_and(|run| run.ok_sources == 0 && run.warn_sources > 0)
        || !signals.official_failed_sources.is_empty()
        || !signals.critical_missing.is_empty()
}

fn has_warnings(packet: &WorkflowPacket, signals: &HealthSignals) -> bool {
    packet
        .collector_run
        .as_ref()
        .is_none_or(|run| run.warn_sources > 0)
        || !signals.failed_sources.is_empty()
        || !signals.stale_items.is_empty()
        || !signals.missing_info.is_empty()
}

fn synthesize(
    packet: &WorkflowPacket,
    _signals: &HealthSignals,
    blocked: bool,
    warnings: bool,
    current_position_pct: Option<f64>,
) -> (String, String, String, Vec<String>) {
    let mut contradictions = Vec::new();
    if blocked {
        contradictions.push("data-quality 阻断时不得输出增强、Bull 或 keep。".to_string());
    } else if warnings {
        contradictions.push("存在警戒项时必须降置信度并保留 missing_info。".to_string());
    }

    let monitor_action = if blocked {
        "先补采或人工核验阻断来源，再允许后续 workflow 输出乐观结论。"
    } else if warnings {
        "允许后续 workflow 降置信度运行，但必须保留 missing_info。"
    } else {
        "自动化健康正常，按既定定时任务继续。"
    };

    match packet.workflow.as_str() {
        "monitoring" => (
            if blocked {
                "紧急".to_string()
            } else if warnings {
                "警戒".to_string()
            } else {
                "正常".to_string()
            },
            monitor_action.to_string(),
            "collector warning、官方源失败、P0 missing_info、USDC/USD 偏离、储备异常、Circle/核心链状态异常或监管 P0。".to_string(),
            contradictions,
        ),
        "daily-monitor" => (
            if blocked {
                "降级".to_string()
            } else {
                "观察".to_string()
            },
            if blocked {
                "停止输出增强；先完成信用层、来源或监管 P0 复核。".to_string()
            } else {
                "维持观察，不因单日行情改框架；等待周度或事件复核确认。".to_string()
            },
            "Circle Transparency 更新、监管官方公告、USDC/USD 偏离、储备收益率刷新或 Base/Circle 状态异常。".to_string(),
            contradictions,
        ),
        "weekly-review" => (
            if blocked {
                "降级".to_string()
            } else {
                "观察".to_string()
            },
            if blocked {
                "进入降级复核；competition score 不得乐观归一。".to_string()
            } else {
                "维持框架，等待 D1-D7、RLDC margin 和平台化证据共振。".to_string()
            },
            "USDC 市占率连续变化、competition score 跨档、RLDC margin 更新、监管 P0 或 CPN/Arc 收入披露。".to_string(),
            contradictions,
        ),
        "quarterly-earnings" => (
            if blocked {
                "降级".to_string()
            } else {
                "中性".to_string()
            },
            if blocked {
                "SEC/公司披露或关键财务输入阻断，先补事实锚。".to_string()
            } else {
                "完成事实层复核；电话会和管理层 Q&A 后再更新叙事层。".to_string()
            },
            "最新 10-Q/10-K、shareholder letter、earnings call transcript、FY 指引更新或 RLDC margin 低于 38%。".to_string(),
            contradictions,
        ),
        "valuation-decision" => {
            let position = current_position_pct
                .map(|pct| format!("当前仓位 {pct:.1}%；"))
                .unwrap_or_else(|| "未传当前仓位；".to_string());
            (
                if blocked {
                    "信用层触发 / Bear".to_string()
                } else {
                    "Base".to_string()
                },
                if blocked {
                    format!("{position}仓位上限切到防守区间，先补数据或核验 P0。")
                } else {
                    format!("{position}维持仓位上限复核，不从单条新闻调整倍数。")
                },
                "competition score 跨档、RLDC margin 低于 38%、Other revenue share 跨 10%、监管 P0、信用层 C-TRIGGER 或筹码 P2。".to_string(),
                contradictions,
            )
        }
        "framework-review" => (
            if blocked {
                "defer".to_string()
            } else if warnings {
                "revise".to_string()
            } else {
                "keep".to_string()
            },
            if blocked {
                "数据健康或事实锚阻断，先补采/核源，不写入框架。".to_string()
            } else if warnings {
                "记录最小 revise 候选，等缺口缩小后再写入正式规则。".to_string()
            } else {
                "保留当前主流程，不新增规则；下轮继续固定 workflow 复核。".to_string()
            },
            "collector warning 清零、P0/P1 缺口变化、competition score 跨档、财报后自检或新假设需要 keep/revise/defer/reject。".to_string(),
            contradictions,
        ),
        _ => (
            "观察".to_string(),
            monitor_action.to_string(),
            "下次定时任务或关键事件触发。".to_string(),
            contradictions,
        ),
    }
}

fn confidence(blocked: bool, warnings: bool, signals: &HealthSignals) -> String {
    if blocked || signals.critical_missing.len() >= 3 {
        "low".to_string()
    } else if warnings {
        "medium".to_string()
    } else {
        "high".to_string()
    }
}

fn key_evidence(packet: &WorkflowPacket) -> Vec<String> {
    let preferred = [
        "P0_USDC_CIRCULATING_SUPPLY",
        "P0_CIRCLE_USDC_TOTAL_RESERVES",
        "P0_CIRCLE_RESERVE_FUND_7D_YIELD",
        "P0_USDC_USD_PRICE",
        "P1_USDC_MARKET_CAP_SHARE",
        "P2_CIRCLE_RLDC_MARGIN",
        "P2_CIRCLE_OTHER_REVENUE_SHARE",
        "P0_CRCL_PRICE",
    ];
    let by_code = packet
        .evidence
        .latest_observations
        .iter()
        .map(|obs| (obs.metric_code.as_str(), obs))
        .collect::<BTreeMap<_, _>>();
    let mut output = Vec::new();
    for code in preferred {
        if let Some(obs) = by_code.get(code) {
            output.push(observation_line(obs));
        }
    }
    if output.is_empty() {
        output.extend(
            packet
                .evidence
                .latest_observations
                .iter()
                .take(8)
                .map(observation_line),
        );
    }
    output
}

fn observation_line(obs: &RecentObservation) -> String {
    format!(
        "{} | {} | observed_at={} | source={} | {}",
        obs.metric_code,
        display_value(obs.value_num, obs.value_text.as_deref(), &obs.unit),
        obs.observed_at,
        obs.source,
        obs.source_url
    )
}

fn source_run_line(run: &RecentSourceRun) -> String {
    format!(
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
    )
}

fn is_official_or_critical_source(run: &RecentSourceRun) -> bool {
    let text = format!("{} {}", run.source, run.url).to_ascii_lowercase();
    [
        "sec", "edgar", "circle", "occ", "treasury", "fincen", "ofac", "federal", "fdic", "cftc",
        "congress", "status", "base", "solana", "ethereum",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

fn stale_thresholds() -> &'static [(&'static str, i64)] {
    &[
        ("P0_USDC_CIRCULATING_SUPPLY", 7),
        ("P0_CIRCLE_USDC_TOTAL_RESERVES", 7),
        ("P0_CIRCLE_RESERVE_FUND_7D_YIELD", 7),
        ("P0_USDC_USD_PRICE", 3),
        ("P1_USDC_MARKET_CAP_SHARE", 14),
        ("P1_USDC_ADJUSTED_TRANSFER_VOLUME", 14),
        ("P2_CIRCLE_RLDC_MARGIN", 120),
        ("P2_CIRCLE_OTHER_REVENUE_SHARE", 120),
    ]
}

fn days_old(observed_at: &str, generated_at: &str) -> i64 {
    let observed = DateTime::parse_from_rfc3339(observed_at)
        .map(|value| value.with_timezone(&Utc))
        .or_else(|_| {
            chrono::NaiveDate::parse_from_str(observed_at, "%Y-%m-%d")
                .map(|date| date.and_hms_opt(0, 0, 0).unwrap().and_utc())
        });
    let generated = DateTime::parse_from_rfc3339(generated_at)
        .map(|value| value.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());
    observed
        .map(|value| (generated - value).num_days().max(0))
        .unwrap_or(0)
}

fn render_markdown(output: &AgentRunOutput, packet: &WorkflowPacket) -> String {
    let mut lines = vec![
        "# CRCL Rust agent run".to_string(),
        format!("workflow={}", output.workflow),
        format!("generated_at={}", output.generated_at),
        format!("packet_generated_at={}", output.packet_generated_at),
        format!("database={}", output.database),
        format!("confidence={}", output.confidence),
        String::new(),
        "## final conclusion".to_string(),
        format!("结论：{}", output.conclusion),
        format!(
            "data-quality 是否阻断：{}",
            if output.data_quality_blocked {
                "是"
            } else {
                "否"
            }
        ),
        format!("动作：{}", output.action),
        String::new(),
        "## key evidence".to_string(),
    ];

    push_list(&mut lines, &output.key_evidence, "暂无关键指标证据。");
    lines.push(String::new());
    lines.push("## failed sources".to_string());
    push_list(&mut lines, &output.failed_sources, "本轮未发现失败源。");
    lines.push(String::new());
    lines.push("## stale items".to_string());
    push_list(&mut lines, &output.stale_items, "本轮未发现过期关键指标。");
    lines.push(String::new());
    lines.push("## missing_info".to_string());
    push_list(&mut lines, &output.missing_info, "暂无 missing_info。");
    lines.push(String::new());
    lines.push("## 反证 / 解除条件".to_string());
    push_list(
        &mut lines,
        &output.contradictions,
        "无覆盖性反证；仍需等待下一轮自动复核。",
    );
    lines.push(String::new());
    lines.push("## next review trigger".to_string());
    lines.push(output.next_review_trigger.clone());
    lines.push(String::new());
    lines.push("## subagent dispatch contract".to_string());
    for task in &packet.subagent_dispatch {
        lines.push(format!(
            "- {} | {} | {}",
            task.profile, task.title, task.context_command
        ));
    }
    if !output.saved_paths.is_empty() {
        lines.push(String::new());
        lines.push("## saved paths".to_string());
        push_list(&mut lines, &output.saved_paths, "");
    }
    lines.push(String::new());
    lines.join("\n")
}

fn push_list(lines: &mut Vec<String>, items: &[String], empty: &str) {
    if items.is_empty() {
        if !empty.is_empty() {
            lines.push(format!("- {empty}"));
        }
        return;
    }
    for item in items {
        lines.push(format!("- {item}"));
    }
}

fn save_output(output: &AgentRunOutput, text: &str, out_dir: &Path) -> Result<Vec<String>> {
    fs::create_dir_all(out_dir)?;
    let stamp = output.generated_at.replace([':', '.'], "-");
    let base = out_dir.join(format!("{}-{}", stamp, output.workflow));
    let markdown_path = base.with_extension("md");
    let json_path = base.with_extension("json");
    fs::write(&markdown_path, text)?;
    fs::write(
        &json_path,
        format!("{}\n", serde_json::to_string_pretty(output)?),
    )?;
    eprintln!(
        "saved {} and {}",
        markdown_path.display(),
        json_path.display()
    );
    Ok(vec![
        markdown_path.display().to_string(),
        json_path.display().to_string(),
    ])
}

fn display_value(value_num: Option<f64>, value_text: Option<&str>, unit: &str) -> String {
    match (value_num, value_text) {
        (Some(value), _) if unit == "status" => format!("{value:.0} {unit}"),
        (Some(value), _) => format!("{value:.4} {unit}"),
        (None, Some(value)) => format!("{value} {unit}"),
        (None, None) => format!("- {unit}"),
    }
}

fn unique(items: Vec<String>) -> Vec<String> {
    let mut output = Vec::new();
    for item in items {
        if !output.contains(&item) {
            output.push(item);
        }
    }
    output
}
