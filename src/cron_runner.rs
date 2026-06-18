use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use chrono::Utc;
use clap::{Subcommand, ValueEnum};
use serde::Serialize;

const CRON_BEGIN: &str = "# BEGIN CRCL CODEX CRON";
const CRON_END: &str = "# END CRCL CODEX CRON";

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum CronTask {
    All,
    Monitor,
    Daily,
    Weekly,
    Decision,
    Quarterly,
    Framework,
    FullAnalysis,
}

#[derive(Debug, Subcommand)]
pub enum CronCommand {
    Print,
    Install,
    Remove,
    List,
    Run {
        #[arg(value_enum, default_value_t = CronTask::All)]
        task: CronTask,

        #[arg(long)]
        dry_run: bool,

        #[arg(long)]
        no_collect: bool,

        #[arg(long)]
        safe: bool,

        #[arg(long, env = "CRCL_CODEX_MODEL")]
        model: Option<String>,

        #[arg(long, env = "CRCL_CODEX_PROFILE")]
        profile: Option<String>,
    },
}

#[derive(Debug)]
struct TaskSpec {
    key: &'static str,
    workflow: &'static str,
    category: &'static str,
    schedule_env: &'static str,
    default_schedule: &'static str,
    title: &'static str,
    agents: &'static [&'static str],
    purpose: &'static str,
}

#[derive(Debug, Serialize)]
struct RunMeta {
    task: String,
    workflow: String,
    category: String,
    started_at: String,
    root_dir: String,
    output_dir: String,
    output_last_message: String,
    event_log: String,
    no_collect: bool,
    safe_mode: bool,
    completed_at: Option<String>,
    exit_status: Option<i32>,
    signal: Option<String>,
}

const TASKS: &[TaskSpec] = &[
    TaskSpec {
        key: "monitor",
        workflow: "monitoring",
        category: "monitoring",
        schedule_env: "CRCL_CRON_MONITOR",
        default_schedule: "20 8-23/2 * * *",
        title: "CRCL 自动化监控",
        agents: &[
            "crcl-data-collector",
            "crcl-data-quality-auditor",
            "crcl-source-verifier",
            "crcl-regulatory-watch",
            "crcl-monitor-guard",
        ],
        purpose: "监控数据源失败、stale 指标、P0 missing_info、状态页异常和紧急事件。",
    },
    TaskSpec {
        key: "daily",
        workflow: "daily-monitor",
        category: "daily-monitor",
        schedule_env: "CRCL_CRON_DAILY",
        default_schedule: "10 8 * * *",
        title: "CRCL 日监",
        agents: &[
            "crcl-data-collector",
            "crcl-data-quality-auditor",
            "crcl-source-verifier",
            "crcl-regulatory-watch",
            "crcl-risk-decision",
        ],
        purpose: "判断当天是否出现 P0 数据、监管、锚定或状态异常。",
    },
    TaskSpec {
        key: "weekly",
        workflow: "weekly-review",
        category: "weekly-review",
        schedule_env: "CRCL_CRON_WEEKLY",
        default_schedule: "30 9 * * 1",
        title: "CRCL 周度复盘",
        agents: &[
            "crcl-data-collector",
            "crcl-data-quality-auditor",
            "crcl-source-verifier",
            "crcl-competition-score",
            "crcl-financial-valuation",
            "crcl-regulatory-watch",
            "crcl-platform-option",
            "crcl-risk-decision",
        ],
        purpose: "计算 D1-D7、competition score，并输出周度增长质量结论。",
    },
    TaskSpec {
        key: "decision",
        workflow: "valuation-decision",
        category: "valuation-decision",
        schedule_env: "CRCL_CRON_DECISION",
        default_schedule: "0 16 * * 1-5",
        title: "CRCL 估值仓位复核",
        agents: &[
            "crcl-data-collector",
            "crcl-data-quality-auditor",
            "crcl-source-verifier",
            "crcl-financial-valuation",
            "crcl-competition-score",
            "crcl-regulatory-watch",
            "crcl-platform-option",
            "crcl-risk-decision",
        ],
        purpose: "复核 Bull/Base/Bear、仓位上限和交易动作触发条件。",
    },
    TaskSpec {
        key: "quarterly",
        workflow: "quarterly-earnings",
        category: "quarterly-earnings",
        schedule_env: "CRCL_CRON_QUARTERLY",
        default_schedule: "0 10 15 2,5,8,11 *",
        title: "CRCL 季度财报复核",
        agents: &[
            "crcl-data-collector",
            "crcl-data-quality-auditor",
            "crcl-source-verifier",
            "crcl-financial-valuation",
            "crcl-platform-option",
            "crcl-competition-score",
            "crcl-risk-decision",
        ],
        purpose: "复核 SEC/公司披露、RLDC、Other revenue、平台化和财报结论。",
    },
    TaskSpec {
        key: "framework",
        workflow: "framework-review",
        category: "framework-review",
        schedule_env: "CRCL_CRON_FRAMEWORK",
        default_schedule: "30 10 * * 6",
        title: "CRCL 框架自检",
        agents: &[
            "crcl-data-collector",
            "crcl-data-quality-auditor",
            "crcl-source-verifier",
            "crcl-financial-valuation",
            "crcl-regulatory-watch",
            "crcl-competition-score",
            "crcl-platform-option",
            "crcl-autoresearch-curator",
            "crcl-risk-decision",
        ],
        purpose: "执行 keep/revise/defer/reject 框架自检和 autoresearch 治理。",
    },
];

pub fn handle(command: CronCommand) -> Result<()> {
    match command {
        CronCommand::Print => {
            println!("{}", cron_block()?);
        }
        CronCommand::Install => {
            install_cron()?;
        }
        CronCommand::Remove => {
            remove_cron()?;
        }
        CronCommand::List => {
            print_tasks();
        }
        CronCommand::Run {
            task,
            dry_run,
            no_collect,
            safe,
            model,
            profile,
        } => {
            run_task_selection(task, dry_run, no_collect, safe, model, profile)?;
        }
    }
    Ok(())
}

fn run_task_selection(
    task: CronTask,
    dry_run: bool,
    no_collect: bool,
    safe: bool,
    model: Option<String>,
    profile: Option<String>,
) -> Result<()> {
    if task == CronTask::All {
        for spec in TASKS {
            run_one(
                spec,
                dry_run,
                no_collect,
                safe,
                model.as_deref(),
                profile.as_deref(),
            )?;
        }
        run_full_analysis(dry_run)?;
        return Ok(());
    }

    if task == CronTask::FullAnalysis {
        run_full_analysis(dry_run)?;
        return Ok(());
    }

    let selected = match task {
        CronTask::Monitor => vec![task_by_key("monitor")?],
        CronTask::Daily => vec![task_by_key("daily")?],
        CronTask::Weekly => vec![task_by_key("weekly")?],
        CronTask::Decision => vec![task_by_key("decision")?],
        CronTask::Quarterly => vec![task_by_key("quarterly")?],
        CronTask::Framework => vec![task_by_key("framework")?],
        CronTask::All | CronTask::FullAnalysis => unreachable!(),
    };

    for spec in selected {
        run_one(
            spec,
            dry_run,
            no_collect,
            safe,
            model.as_deref(),
            profile.as_deref(),
        )?;
    }
    Ok(())
}

fn run_full_analysis(dry_run: bool) -> Result<()> {
    let root_dir = env::current_dir()?;
    let log_dir = root_dir.join("work_docs/cron_logs");
    let command = vec![
        "cargo".to_string(),
        "run".to_string(),
        "--release".to_string(),
        "--".to_string(),
        "full-analysis".to_string(),
        "--save".to_string(),
    ];

    if dry_run {
        println!("# dry-run full-analysis");
        println!("cd {} && {}", shell_quote(&root_dir), shell_join(&command));
        println!();
        return Ok(());
    }

    fs::create_dir_all(&log_dir)?;
    let stamp = Utc::now().to_rfc3339().replace([':', '.'], "-");
    eprintln!("[crcl-cron] start full-analysis -> work_docs/agent_runs");
    let output = Command::new("cargo")
        .args(["run", "--release", "--", "full-analysis", "--save"])
        .current_dir(&root_dir)
        .output()
        .context("failed to start full-analysis")?;

    fs::write(
        log_dir.join(format!("{stamp}-full-analysis-stdout.log")),
        &output.stdout,
    )?;
    if !output.stderr.is_empty() {
        fs::write(
            log_dir.join(format!("{stamp}-full-analysis-stderr.log")),
            &output.stderr,
        )?;
    }

    if !output.status.success() {
        bail!(
            "full-analysis failed with status {:?}; see work_docs/cron_logs",
            output.status.code()
        );
    }
    eprintln!("[crcl-cron] done full-analysis");
    Ok(())
}

fn run_one(
    spec: &TaskSpec,
    dry_run: bool,
    no_collect: bool,
    safe: bool,
    model: Option<&str>,
    profile: Option<&str>,
) -> Result<()> {
    let root_dir = env::current_dir()?;
    let work_docs = root_dir.join("work_docs");
    let output_dir = work_docs.join(spec.category);
    let log_dir = work_docs.join("cron_logs");
    let started_at = Utc::now().to_rfc3339();
    let stamp = started_at.replace([':', '.'], "-");
    let output_last_message = output_dir.join(format!("{stamp}-{}-codex-final.md", spec.workflow));
    let event_log = output_dir.join(format!("{stamp}-{}-codex-events.jsonl", spec.workflow));
    let meta_path = output_dir.join(format!("{stamp}-{}-cron-meta.json", spec.workflow));
    let prompt = build_prompt(spec, &output_dir, &output_last_message, no_collect);
    let args = codex_args(
        &root_dir,
        &output_last_message,
        &prompt,
        safe,
        model,
        profile,
    );

    if dry_run {
        println!("# dry-run {}", spec.key);
        println!(
            "cd {} && codex {}",
            shell_quote(&root_dir),
            shell_join(&args)
        );
        println!();
        println!("{prompt}");
        println!();
        return Ok(());
    }

    fs::create_dir_all(&output_dir)?;
    fs::create_dir_all(&log_dir)?;
    let mut meta = RunMeta {
        task: spec.key.to_string(),
        workflow: spec.workflow.to_string(),
        category: spec.category.to_string(),
        started_at,
        root_dir: root_dir.display().to_string(),
        output_dir: output_dir.display().to_string(),
        output_last_message: output_last_message.display().to_string(),
        event_log: event_log.display().to_string(),
        no_collect,
        safe_mode: safe,
        completed_at: None,
        exit_status: None,
        signal: None,
    };
    fs::write(
        &meta_path,
        format!("{}\n", serde_json::to_string_pretty(&meta)?),
    )?;

    eprintln!("[crcl-cron] start {} -> {}", spec.key, output_dir.display());
    let output = Command::new("codex")
        .args(&args)
        .current_dir(&root_dir)
        .env("CRCL_SCHEDULED_TASK", spec.key)
        .env("CRCL_SCHEDULED_WORKFLOW", spec.workflow)
        .env("CRCL_SCHEDULED_OUTPUT_DIR", &output_dir)
        .output()
        .context("failed to start codex exec")?;

    fs::write(&event_log, &output.stdout)?;
    if !output.stderr.is_empty() {
        fs::write(
            log_dir.join(format!("{stamp}-{}-stderr.log", spec.key)),
            &output.stderr,
        )?;
    }

    meta.completed_at = Some(Utc::now().to_rfc3339());
    meta.exit_status = output.status.code();
    meta.signal = status_signal(&output.status);
    fs::write(
        &meta_path,
        format!("{}\n", serde_json::to_string_pretty(&meta)?),
    )?;

    if !output.status.success() {
        bail!(
            "codex exec failed for {} with status {:?}; see {}",
            spec.key,
            output.status.code(),
            event_log.display()
        );
    }
    eprintln!(
        "[crcl-cron] done {}; final={}",
        spec.key,
        output_last_message.display()
    );
    Ok(())
}

fn codex_args(
    root_dir: &Path,
    output_last_message: &Path,
    prompt: &str,
    safe: bool,
    model: Option<&str>,
    profile: Option<&str>,
) -> Vec<String> {
    let mut args = vec![
        "exec".to_string(),
        "--cd".to_string(),
        root_dir.display().to_string(),
        "--output-last-message".to_string(),
        output_last_message.display().to_string(),
        "--json".to_string(),
    ];
    if let Some(model) = model {
        args.extend(["--model".to_string(), model.to_string()]);
    }
    if let Some(profile) = profile {
        args.extend(["--profile".to_string(), profile.to_string()]);
    }
    if safe {
        args.extend(["--sandbox".to_string(), "danger-full-access".to_string()]);
    } else {
        args.push("--dangerously-bypass-approvals-and-sandbox".to_string());
    }
    args.push(prompt.to_string());
    args
}

fn build_prompt(
    spec: &TaskSpec,
    output_dir: &Path,
    output_last_message: &Path,
    no_collect: bool,
) -> String {
    let data_policy = if no_collect {
        "本轮是只读复核：所有 Rust 命令必须追加 --no-collect，不刷新外部数据。"
    } else {
        "本轮允许刷新外部数据：先由 collector 生成同一批 evidence packet。"
    };
    let position_pct = env::var("CRCL_CURRENT_POSITION_PCT").ok();
    let position_text = if spec.workflow == "valuation-decision" {
        position_pct
            .as_deref()
            .map(|pct| {
                format!("当前仓位参数：{pct}%。运行本地入口时追加 --current-position-pct {pct}。")
            })
            .unwrap_or_else(|| {
                "未设置 CRCL_CURRENT_POSITION_PCT；估值仓位任务只输出仓位上限，不判断具体加减仓量。"
                    .to_string()
            })
    } else {
        String::new()
    };
    let local_command = local_agent_command(spec, output_dir, no_collect, position_pct.as_deref());
    let agents = spec
        .agents
        .iter()
        .map(|agent| format!("- {agent}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        r#"你现在以 cron 定时任务身份在 CRCL 仓库运行。

任务：{title}
workflow：{workflow}
输出目录：{output_dir}
最终回复会由 codex exec 自动保存到：{output_last_message}
目标：{purpose}

必须遵守：
1. 使用中文。
2. 遵守 AGENTS.md 和 .codex/agents/ 下的项目级 subagent 规则。
3. 必须正确使用项目级 Codex subagents，而不是只告诉我该运行什么命令。
4. 必须先派 crcl-data-collector 生成或确认同一批 evidence packet。
5. collector 后必须先做 crcl-data-quality-auditor 和 crcl-source-verifier；如果是 monitoring，还必须派 crcl-monitor-guard。
6. 后续 agent 必须读取同一批 evidence packet 或使用 --no-collect 的同一 SQLite 上下文，避免各自刷新造成口径不一致。
7. 最终产物保存到 {output_dir}；如果调用本地 Rust agent-run 入口，使用：{local_command}
8. 最终回答必须写明已保存路径，至少包含 {output_last_message}。
9. 不输出个性化投资建议。

本轮数据策略：
{data_policy}
{position_text}

本轮需要的 subagents：
{agents}

最终回答必须包含：
1. 结论：日监/周报/财报/估值/框架使用项目规定结论；monitoring 使用 正常 / 警戒 / 紧急。
2. data-quality 是否阻断。
3. 关键证据：指标值、数据日期、来源 URL 或 batch_id。
4. missing_info。
5. 反证 / 解除条件。
6. 下一步动作或下次复盘触发。
7. 已保存的 {output_dir}/*.md 路径。
"#,
        title = spec.title,
        workflow = spec.workflow,
        output_dir = relative(output_dir),
        output_last_message = relative(output_last_message),
        purpose = spec.purpose,
        local_command = local_command,
        data_policy = data_policy,
        position_text = position_text,
        agents = agents
    )
}

fn local_agent_command(
    spec: &TaskSpec,
    output_dir: &Path,
    no_collect: bool,
    position_pct: Option<&str>,
) -> String {
    let mut parts = vec![
        "cargo".to_string(),
        "run".to_string(),
        "--release".to_string(),
        "--".to_string(),
        "agent-run".to_string(),
        "--workflow".to_string(),
        spec.workflow.to_string(),
        "--save".to_string(),
        "--out-dir".to_string(),
        relative(output_dir),
    ];
    if no_collect {
        parts.push("--no-collect".to_string());
    }
    if spec.workflow == "valuation-decision"
        && let Some(position_pct) = position_pct
    {
        parts.extend([
            "--current-position-pct".to_string(),
            position_pct.to_string(),
        ]);
    }
    shell_join(&parts)
}

fn print_tasks() {
    for spec in TASKS {
        println!(
            "{}\t{}\t{}\twork_docs/{}",
            spec.key,
            schedule(spec),
            spec.workflow,
            spec.category
        );
    }
    println!(
        "full-analysis\t{}\tfull-analysis\twork_docs/agent_runs",
        env::var("CRCL_CRON_FULL_ANALYSIS").unwrap_or_else(|_| "15 11 * * 6".to_string())
    );
}

fn install_cron() -> Result<()> {
    fs::create_dir_all(env::current_dir()?.join("work_docs/cron_logs"))?;
    let existing = read_crontab()?;
    let next = format!(
        "{}\n\n{}\n",
        strip_existing_block(&existing).trim_end(),
        cron_block()?
    );
    write_crontab(next.trim_start())?;
    println!("installed CRCL Codex cron block");
    println!("{}", cron_block()?);
    Ok(())
}

fn remove_cron() -> Result<()> {
    let existing = read_crontab()?;
    let next = format!("{}\n", strip_existing_block(&existing).trim_end());
    write_crontab(next.trim_start())?;
    println!("removed CRCL Codex cron block");
    Ok(())
}

fn cron_block() -> Result<String> {
    let root = env::current_dir()?;
    let log_dir = root.join("work_docs/cron_logs");
    let mut lines = vec![
        CRON_BEGIN.to_string(),
        "SHELL=/bin/zsh".to_string(),
        format!("PATH={}", cron_path()),
        format!("CRCL_ROOT={}", root.display()),
        format!(
            "CRCL_CODEX_SAFE_MODE={}",
            env::var("CRCL_CODEX_SAFE_MODE").unwrap_or_else(|_| "0".to_string())
        ),
    ];
    for key in [
        "CRCL_CODEX_MODEL",
        "CRCL_CODEX_PROFILE",
        "CRCL_CURRENT_POSITION_PCT",
    ] {
        if let Ok(value) = env::var(key) {
            lines.push(format!("{key}={value}"));
        }
    }
    for spec in TASKS {
        lines.push(format!(
            "{} cd {} && cargo run --release -- cron run {} >> {} 2>&1",
            schedule(spec),
            shell_quote(&root),
            spec.key,
            shell_quote(log_dir.join(format!("{}.log", spec.key)))
        ));
    }
    lines.push(format!(
        "{} cd {} && cargo run --release -- cron run full-analysis >> {} 2>&1",
        env::var("CRCL_CRON_FULL_ANALYSIS").unwrap_or_else(|_| "15 11 * * 6".to_string()),
        shell_quote(&root),
        shell_quote(log_dir.join("full-analysis.log"))
    ));
    lines.push(CRON_END.to_string());
    Ok(lines.join("\n"))
}

fn read_crontab() -> Result<String> {
    let output = Command::new("crontab").arg("-l").output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Ok(String::new())
    }
}

fn write_crontab(content: &str) -> Result<()> {
    let mut child = Command::new("crontab")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .context("failed to start crontab -")?;
    child
        .stdin
        .as_mut()
        .context("failed to open crontab stdin")?
        .write_all(content.as_bytes())?;
    let status = child.wait()?;
    if !status.success() {
        bail!("failed to write crontab");
    }
    Ok(())
}

fn strip_existing_block(input: &str) -> String {
    let Some(begin) = input.find(CRON_BEGIN) else {
        return input.to_string();
    };
    let Some(end) = input[begin..].find(CRON_END) else {
        return input[..begin].to_string();
    };
    let end_index = begin + end + CRON_END.len();
    format!("{}{}", &input[..begin], &input[end_index..])
}

fn task_by_key(key: &str) -> Result<&'static TaskSpec> {
    TASKS
        .iter()
        .find(|spec| spec.key == key)
        .with_context(|| format!("unknown task: {key}"))
}

fn schedule(spec: &TaskSpec) -> String {
    env::var(spec.schedule_env).unwrap_or_else(|_| spec.default_schedule.to_string())
}

fn cron_path() -> String {
    if let Ok(value) = env::var("CRCL_CRON_PATH") {
        return value;
    }
    let mut paths = Vec::new();
    if let Ok(exe) = env::current_exe()
        && let Some(parent) = exe.parent()
    {
        paths.push(parent.display().to_string());
    }
    paths.extend([
        "/opt/homebrew/bin".to_string(),
        "/opt/homebrew/sbin".to_string(),
        "/usr/local/bin".to_string(),
        "/usr/bin".to_string(),
        "/bin".to_string(),
        "/usr/sbin".to_string(),
        "/sbin".to_string(),
    ]);
    if let Ok(home) = env::var("HOME") {
        paths.push(format!("{home}/.cargo/bin"));
    }
    paths.join(":")
}

fn relative(path: &Path) -> String {
    env::current_dir()
        .ok()
        .and_then(|root| path.strip_prefix(root).ok().map(PathBuf::from))
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| path.display().to_string())
}

fn shell_join(parts: &[String]) -> String {
    parts.iter().map(shell_quote).collect::<Vec<_>>().join(" ")
}

fn shell_quote(value: impl AsRef<Path>) -> String {
    let text = value.as_ref().display().to_string();
    format!("'{}'", text.replace('\'', "'\\''"))
}

#[cfg(unix)]
fn status_signal(status: &std::process::ExitStatus) -> Option<String> {
    use std::os::unix::process::ExitStatusExt;
    status.signal().map(|signal| signal.to_string())
}

#[cfg(not(unix))]
fn status_signal(_status: &std::process::ExitStatus) -> Option<String> {
    None
}
