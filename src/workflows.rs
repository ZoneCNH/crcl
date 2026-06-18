use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use clap::ValueEnum;
use serde::Serialize;

use crate::collectors::source_selector_label;
use crate::db::{Database, RecentEvent, RecentFiling, RecentObservation, RecentSourceRun, Summary};
use crate::models::MissingItem;
use crate::{AgentProfile, OutputFormat, SourceSelector};

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum WorkflowKind {
    DailyMonitor,
    Monitoring,
    WeeklyReview,
    QuarterlyEarnings,
    ValuationDecision,
    FrameworkReview,
}

#[derive(Debug, Serialize)]
pub struct WorkflowCollectorRun {
    pub batch_id: String,
    pub ok_sources: usize,
    pub warn_sources: usize,
}

#[derive(Debug, Serialize)]
pub struct WorkflowPacket {
    pub workflow: String,
    pub generated_at: String,
    pub database: String,
    pub execution_model: &'static str,
    pub collector_run: Option<WorkflowCollectorRun>,
    pub collect_scope: Vec<&'static str>,
    pub docs_to_read: Vec<&'static str>,
    pub orchestrator_steps: Vec<&'static str>,
    pub hard_gates: Vec<&'static str>,
    pub quality_gates: Vec<&'static str>,
    pub subagent_dispatch: Vec<SubagentTask>,
    pub final_output_template: &'static str,
    pub evidence: WorkflowEvidence,
}

#[derive(Debug, Serialize)]
pub struct SubagentTask {
    pub profile: &'static str,
    pub title: &'static str,
    pub objective: &'static str,
    pub context_command: String,
    pub docs_to_read: Vec<&'static str>,
    pub output_contract: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
pub struct WorkflowEvidence {
    pub summary: Summary,
    pub latest_observations: Vec<RecentObservation>,
    pub observation_history: Vec<RecentObservation>,
    pub recent_source_runs: Vec<RecentSourceRun>,
    pub recent_filings: Vec<RecentFiling>,
    pub recent_events: Vec<RecentEvent>,
    pub missing_items: Vec<MissingItem>,
}

pub struct WorkflowPacketRequest<'a> {
    pub db: &'a Database,
    pub database: &'a Path,
    pub workflow: WorkflowKind,
    pub limit: usize,
    pub history_limit: usize,
    pub collector_run: Option<WorkflowCollectorRun>,
}

struct WorkflowSpec {
    label: &'static str,
    collect_selectors: &'static [SourceSelector],
    docs_to_read: &'static [&'static str],
    metric_codes: &'static [&'static str],
    subagents: &'static [SubagentSpec],
    orchestrator_steps: &'static [&'static str],
    hard_gates: &'static [&'static str],
    quality_gates: &'static [&'static str],
    final_output_template: &'static str,
}

struct SubagentSpec {
    profile: AgentProfile,
    title: &'static str,
    objective: &'static str,
    docs_to_read: &'static [&'static str],
    output_contract: &'static [&'static str],
}

const DAILY_SELECTORS: &[SourceSelector] = &[
    SourceSelector::Market,
    SourceSelector::Rates,
    SourceSelector::Events,
    SourceSelector::Status,
];
const MONITOR_SELECTORS: &[SourceSelector] = &[SourceSelector::All];
const WEEKLY_SELECTORS: &[SourceSelector] = &[
    SourceSelector::Market,
    SourceSelector::Rates,
    SourceSelector::Sec,
    SourceSelector::Events,
];
const QUARTERLY_SELECTORS: &[SourceSelector] = &[
    SourceSelector::Sec,
    SourceSelector::Rates,
    SourceSelector::Market,
    SourceSelector::Events,
];
const VALUATION_SELECTORS: &[SourceSelector] = &[
    SourceSelector::Sec,
    SourceSelector::Rates,
    SourceSelector::Market,
    SourceSelector::Events,
    SourceSelector::Status,
];
const FRAMEWORK_SELECTORS: &[SourceSelector] = &[SourceSelector::All];

const DAILY_METRICS: &[&str] = &[
    "P0_USDC_CIRCULATING_SUPPLY",
    "P0_USDC_1D_CHANGE_PCT",
    "P0_USDC_7D_CHANGE_PCT",
    "P0_USDC_30D_CHANGE_PCT",
    "P0_CIRCLE_USDC_7D_NET_ISSUANCE",
    "P0_CIRCLE_USDC_30D_NET_ISSUANCE",
    "P0_CIRCLE_USDC_TOTAL_RESERVES",
    "P0_CIRCLE_RESERVE_FUND_7D_YIELD",
    "P0_TREASURY_3M_YIELD",
    "P0_SOFR",
    "P0_USDC_USD_PRICE",
    "P0_USDT_USD_PRICE",
    "P0_CURVE_3POOL_USDC_RATIO",
    "P0_BTC_PRICE",
    "P0_ETH_PRICE",
    "P0_CRCL_PRICE",
    "P0_CRCL_VOLUME",
    "P0_CHAIN_STATUS_BASE",
    "P0_CHAIN_STATUS_SOLANA",
    "P0_CHAIN_STATUS_ETHEREUM",
    "P0_CIRCLE_STATUS",
];
const MONITOR_METRICS: &[&str] = &[
    "P0_USDC_CIRCULATING_SUPPLY",
    "P0_USDC_1D_CHANGE_PCT",
    "P0_USDC_7D_CHANGE_PCT",
    "P0_USDC_30D_CHANGE_PCT",
    "P0_CIRCLE_USDC_7D_NET_ISSUANCE",
    "P0_CIRCLE_USDC_30D_NET_ISSUANCE",
    "P0_CIRCLE_USDC_TOTAL_RESERVES",
    "P0_CIRCLE_RESERVE_FUND_7D_YIELD",
    "P0_USDC_USD_PRICE",
    "P0_USDT_USD_PRICE",
    "P0_CURVE_3POOL_USDC_RATIO",
    "P0_CRCL_PRICE",
    "P0_CRCL_VOLUME",
    "P0_CHAIN_STATUS_BASE",
    "P0_CHAIN_STATUS_SOLANA",
    "P0_CHAIN_STATUS_ETHEREUM",
    "P0_CIRCLE_STATUS",
    "P1_STABLECOIN_TOTAL_MARKET_CAP",
    "P1_USDC_MARKET_CAP_SHARE",
    "P1_USDC_USDT_RATIO",
    "P1_USDT_MARKET_CAP",
    "P1_EXCHANGE_USDC_BALANCES",
    "P1_EXCHANGE_USDC_BALANCE_24H_CHANGE",
    "P1_EXCHANGE_USDC_BALANCE_24H_CHANGE_PCT",
    "P2_CIRCLE_RLDC_MARGIN",
    "P2_CIRCLE_OTHER_REVENUE_SHARE",
];

const WEEKLY_METRICS: &[&str] = &[
    "P0_USDC_CIRCULATING_SUPPLY",
    "P0_USDC_7D_CHANGE_PCT",
    "P0_USDC_30D_CHANGE_PCT",
    "P0_CIRCLE_USDC_7D_NET_ISSUANCE",
    "P1_STABLECOIN_TOTAL_MARKET_CAP",
    "P1_USDC_MARKET_CAP_SHARE",
    "P1_USDC_USDT_RATIO",
    "P1_USDT_MARKET_CAP",
    "P1_USDC_CHAIN_ETHEREUM",
    "P1_USDC_CHAIN_BASE",
    "P1_USDC_CHAIN_SOLANA",
    "P1_USDC_CHAIN_ARBITRUM",
    "P1_USDC_ADJUSTED_TRANSFER_VOLUME",
    "P1_USDC_ACTIVE_ADDRESSES",
    "P1_USDC_TRANSACTION_COUNT",
    "P1_USDC_TRANSFER_COUNT",
    "P1_DEFI_PROTOCOL_USDC_DEPOSITS",
    "P1_EXCHANGE_USDC_BALANCES",
    "P1_EXCHANGE_USDC_BALANCE_30D_CHANGE",
    "P1_EXCHANGE_USDC_BALANCE_30D_CHANGE_PCT",
    "P1_EXCHANGE_USDC_TOP3_CONCENTRATION",
    "P1_COMPETITOR_PYUSD_MARKET_CAP",
    "P1_COMPETITOR_FDUSD_MARKET_CAP",
    "P1_COMPETITOR_USDE_MARKET_CAP",
    "P1_COMPETITOR_USDY_MARKET_CAP",
    "P1_TOKENIZED_TREASURY_AUM",
    "P1_TOKENIZED_TREASURY_BUIDL_AUM",
    "P1_COINBASE_USDC_ON_PLATFORM",
    "P1_CPN_ANNUALIZED_TPV",
    "P1_ARC_PUBLIC_NETWORK_STATUS",
    "P0_CIRCLE_RESERVE_FUND_7D_YIELD",
    "P0_TREASURY_3M_YIELD",
    "P0_SOFR",
    "P2_CIRCLE_RLDC_MARGIN",
    "P2_CIRCLE_OTHER_REVENUE_SHARE",
    "P2_FINRA_SHORT_INTEREST",
];

const QUARTERLY_METRICS: &[&str] = &[
    "P2_CIRCLE_TOTAL_REVENUE_AND_RESERVE_INCOME",
    "P2_CIRCLE_RESERVE_INCOME",
    "P2_CIRCLE_OTHER_REVENUE",
    "P2_CIRCLE_OTHER_REVENUE_SHARE",
    "P2_CIRCLE_DISTRIBUTION_TRANSACTION_COSTS",
    "P2_CIRCLE_TOTAL_DISTRIBUTION_TRANSACTION_OTHER_COSTS",
    "P2_CIRCLE_RLDC",
    "P2_CIRCLE_RLDC_MARGIN",
    "P2_CIRCLE_ADJUSTED_EBITDA",
    "P2_CIRCLE_ADJUSTED_EBITDA_MARGIN",
    "P2_CRCL_BASIC_SHARES_OUTSTANDING",
    "P2_CRCL_DILUTED_SHARES_OUTSTANDING",
    "P2_CRCL_SHARES_OUTSTANDING",
    "P2_CIRCLE_USDC_IN_CIRCULATION_AVG_PERIOD",
    "P2_CIRCLE_USDC_IN_CIRCULATION_END_PERIOD",
    "P1_USDC_ONCHAIN_TRANSACTION_VOLUME",
    "P1_USDC_VELOCITY",
    "P1_COINBASE_STABLECOIN_REVENUE",
    "P1_COINBASE_USDC_ON_PLATFORM",
    "P1_CPN_ANNUALIZED_TPV",
    "P1_ARC_MAINNET_STATUS",
    "P1_ARC_PUBLIC_NETWORK_STATUS",
    "P0_CIRCLE_RESERVE_FUND_7D_YIELD",
    "P2_RESERVE_FUND_WAM",
    "P2_RESERVE_FUND_WAL",
    "P2_RESERVE_FUND_NET_ASSETS",
];

const VALUATION_METRICS: &[&str] = &[
    "P0_USDC_CIRCULATING_SUPPLY",
    "P0_USDC_7D_CHANGE_PCT",
    "P0_USDC_30D_CHANGE_PCT",
    "P0_CIRCLE_USDC_7D_NET_ISSUANCE",
    "P0_CRCL_PRICE",
    "P0_CRCL_VOLUME",
    "P2_CIRCLE_RLDC",
    "P2_CIRCLE_RLDC_MARGIN",
    "P2_CIRCLE_RESERVE_INCOME",
    "P2_CIRCLE_OTHER_REVENUE",
    "P2_CIRCLE_OTHER_REVENUE_SHARE",
    "P2_CIRCLE_ADJUSTED_EBITDA",
    "P2_CIRCLE_USDC_IN_CIRCULATION_AVG_PERIOD",
    "P2_CRCL_DILUTED_SHARES_OUTSTANDING",
    "P2_CRCL_SHARES_OUTSTANDING",
    "P0_CIRCLE_RESERVE_FUND_7D_YIELD",
    "P0_TREASURY_3M_YIELD",
    "P1_STABLECOIN_TOTAL_MARKET_CAP",
    "P1_USDC_MARKET_CAP_SHARE",
    "P1_USDC_USDT_RATIO",
    "P1_USDT_MARKET_CAP",
    "P1_USDC_CHAIN_BASE",
    "P1_USDC_ADJUSTED_TRANSFER_VOLUME",
    "P1_USDC_ACTIVE_ADDRESSES",
    "P1_USDC_TRANSACTION_COUNT",
    "P1_EXCHANGE_USDC_BALANCES",
    "P1_EXCHANGE_USDC_BALANCE_30D_CHANGE",
    "P1_EXCHANGE_USDC_BALANCE_30D_CHANGE_PCT",
    "P1_EXCHANGE_USDC_TOP3_CONCENTRATION",
    "P1_COMPETITOR_USDE_MARKET_CAP",
    "P1_TOKENIZED_TREASURY_AUM",
    "P1_TOKENIZED_TREASURY_BUIDL_AUM",
    "P1_CPN_ANNUALIZED_TPV",
    "P1_ARC_MAINNET_STATUS",
    "P1_ARC_PUBLIC_NETWORK_STATUS",
    "P2_FINRA_SHORT_INTEREST",
    "P2_CRCL_DAYS_TO_COVER",
    "P2_CRCL_SHORT_INTEREST_CHANGE_PCT",
    "P2_CRCL_INSTITUTIONAL_INFLOW_12M",
    "P2_CRCL_INSTITUTIONAL_OUTFLOW_12M",
];
const FRAMEWORK_METRICS: &[&str] = &[
    "P0_USDC_CIRCULATING_SUPPLY",
    "P0_USDC_7D_CHANGE_PCT",
    "P0_USDC_30D_CHANGE_PCT",
    "P0_CIRCLE_USDC_7D_NET_ISSUANCE",
    "P0_CIRCLE_USDC_TOTAL_RESERVES",
    "P0_CIRCLE_RESERVE_FUND_7D_YIELD",
    "P0_USDC_USD_PRICE",
    "P0_CURVE_3POOL_USDC_RATIO",
    "P0_CHAIN_STATUS_BASE",
    "P0_CIRCLE_STATUS",
    "P1_USDC_MARKET_CAP_SHARE",
    "P1_USDC_USDT_RATIO",
    "P1_USDT_MARKET_CAP",
    "P1_USDC_CHAIN_BASE",
    "P1_USDC_ADJUSTED_TRANSFER_VOLUME",
    "P1_USDC_ACTIVE_ADDRESSES",
    "P1_DEFI_PROTOCOL_USDC_DEPOSITS",
    "P1_EXCHANGE_USDC_BALANCES",
    "P1_EXCHANGE_USDC_BALANCE_30D_CHANGE",
    "P1_EXCHANGE_USDC_BALANCE_90D_CHANGE_PCT",
    "P1_EXCHANGE_USDC_TOP3_CONCENTRATION",
    "P1_CPN_ANNUALIZED_TPV",
    "P1_ARC_MAINNET_STATUS",
    "P1_ARC_PUBLIC_NETWORK_STATUS",
    "P2_CIRCLE_RLDC_MARGIN",
    "P2_CIRCLE_OTHER_REVENUE_SHARE",
    "P2_CIRCLE_RLDC",
    "P2_CRCL_DILUTED_SHARES_OUTSTANDING",
    "P2_FINRA_SHORT_INTEREST",
];

const DAILY_DOCS: &[&str] = &[
    "docs/metrics/01-daily-watchlist.md",
    "docs/risk/01-warning-signals.md",
    "docs/playbook/01-decision-template.md",
    "docs/framework/02-regulation.md",
];
const MONITOR_DOCS: &[&str] = &[
    "docs/sources.md",
    "docs/superpowers/specs/data-collection-sop.md",
    "docs/metrics/01-daily-watchlist.md",
    "docs/risk/01-warning-signals.md",
    "work_docs/data-collector-coverage.md",
    "work_docs/data-source-inventory.md",
];
const WEEKLY_DOCS: &[&str] = &[
    "docs/metrics/02-weekly-review.md",
    "docs/metrics/04-competition-dashboard.md",
    "docs/metrics/05-competition-scoring-rubric.md",
    "docs/framework/03-competition.md",
    "docs/metrics/06-validation-matrix.md",
];
const QUARTERLY_DOCS: &[&str] = &[
    "docs/metrics/03-quarterly-earnings.md",
    "docs/framework/01-business-model.md",
    "docs/framework/04-platform-option.md",
    "docs/valuation/01-scenario-model.md",
    "docs/risk/00-risk-map.md",
];
const VALUATION_DOCS: &[&str] = &[
    "docs/valuation/00-valuation-framework.md",
    "docs/valuation/01-scenario-model.md",
    "docs/playbook/01-decision-template.md",
    "docs/risk/02-failure-conditions.md",
    "docs/metrics/06-validation-matrix.md",
];
const FRAMEWORK_DOCS: &[&str] = &[
    "docs/README.md",
    "docs/autoresearch/00-loop.md",
    "docs/autoresearch/01-iteration-log.md",
    "docs/playbook/00-research-routine.md",
    "docs/playbook/01-decision-template.md",
    "docs/metrics/06-validation-matrix.md",
    "docs/sources.md",
    "work_docs/data-collector-coverage.md",
    "work_docs/data-source-inventory.md",
];

const COMMON_ORCHESTRATOR_STEPS: &[&str] = &[
    "先执行 collector 生成或确认本轮 decision-pack evidence packet；后续 agent 必须读取同一批证据。",
    "再执行 data-quality 和 source 前置检查；采集失败、关键缺口或来源日期不清时不得直接给增强。",
    "并行派发 subagent_dispatch 中的任务；每个子 agent 只输出证据、判断、missing_info 和反证。",
    "总控 agent 按信用层/风险 > 财务 > 监管 > 竞争 > 平台化 > 筹码的优先级合成最终结论。",
    "最终输出必须使用 final_output_template，且每条依据都回连 metric_code、来源日期或事件 URL。",
];

const DAILY_HARD_GATES: &[&str] = &[
    "USDC/USD 持续折价或 Curve USDC 比例异常时，先走信用层风险，不讨论平台化估值。",
    "Circle 储备报告异常、延迟或储备基金收益率连续缺失时，结论不得写增强。",
    "监管最终规则限制收益分享、第三方激励或交易所分销奖励时，直接触发监管 P0 复核。",
    "单日股价波动不改变研究框架；只有基本面、监管或赎回信号共振才可降级。",
];
const MONITOR_HARD_GATES: &[&str] = &[
    "collector_run 全失败、官方源连续失败、P0 missing_info 或 SOURCE_BLOCKED 必须升级为紧急监控。",
    "USDC/USD 折价、储备覆盖异常、Circle/核心链状态异常或监管限制性事件必须覆盖普通数据缺口。",
    "监控 agent 不输出投资动作；只输出正常、警戒、紧急、阻断项和补采/复核动作。",
    "监控报告必须写入对应 work_docs/monitoring/ 目录，保留 batch_id、失败源和下次检查触发。",
];
const WEEKLY_HARD_GATES: &[&str] = &[
    "competition score 必须由 D1-D7 逐项打分得出，缺失维度写 missing_info 并按可得权重归一。",
    "Base 链增长不能单独上调结论；必须同时检查总 USDC 份额和 RLDC margin。",
    "USDC 份额连续 4 周下降、净赎回扩大或 competition score 跌破 40 时，进入降级复核。",
    "D3 和 D7 必须与最新 RLDC margin、Other revenue share 保持一致。",
];
const QUARTERLY_HARD_GATES: &[&str] = &[
    "T+0 只填事实数字，不输出仓位动作；T+24h 才完成结论层。",
    "RLDC margin 低于 38% 时触发降级检查，不被 CPN/Arc 叙事抵消。",
    "Other revenue share 单项超过 10% 只满足平台化切换条件之一，不能独立切换主框架。",
    "管理层口径不能替代 SEC filing；未披露项必须写 missing_info。",
];
const VALUATION_HARD_GATES: &[&str] = &[
    "估值动作必须来自矩阵输出，不从单条新闻直接改倍数。",
    "信用层 C-TRIGGER 任一触发时，仓位上限直接切到 0-10%，优先于 Bull/Base/Bear。",
    "筹码 P2 信号只调整仓位，不切换基本面情景，也不修改估值倍数。",
    "加仓必须在情景未降级且反证未触发的前提下执行。",
];
const FRAMEWORK_HARD_GATES: &[&str] = &[
    "collector warning、P0 missing_info 或来源阻断未解除时，autoresearch 只能输出 defer/revise，不能写 keep。",
    "新增规则必须能映射到增强、降级、观察之一；不能只增加解释性叙事。",
    "同一轮只建议一个最小框架改动，且必须说明目标文件和触发条件。",
    "历史快照不能当当前事实；所有保留项必须写数据日期或 source_url。",
];
const COMMON_QUALITY_GATES: &[&str] = &[
    "结论只能是增强、降级、观察或对应的 Bull/Base/Bear/仓位动作，不写笼统利好利空。",
    "每个判断必须列出指标值、阈值、数据日期和来源。",
    "存在缺失关键数据时，输出 missing_info 和下次复盘触发，不补乐观推断。",
    "最终输出必须包含反证或解除条件，便于下次复盘。",
];

const COLLECTOR_DOCS: &[&str] = &[
    "docs/sources.md",
    "docs/superpowers/specs/data-collection-sop.md",
    "work_docs/data-collector-coverage.md",
];
const COLLECTOR_OUTPUT_CONTRACT: &[&str] = &[
    "执行或确认本轮 workflow 的 decision-pack 采集命令，输出 batch_id、collect_scope、ok_sources 和 warn_sources。",
    "列出 collector failure、凭证缺口、source unreachable、stale P0/P1 指标和人工补采建议。",
    "保存或传递同一批 evidence packet；后续 agent 必须使用 --no-collect 读取同一批数据。",
    "如用户要求只读/不联网，必须明确写 collector skipped，并只报告已有数据库状态。",
];

const DAILY_SUBAGENTS: &[SubagentSpec] = &[
    SubagentSpec {
        profile: AgentProfile::Collector,
        title: "日监数据收集编排",
        objective: "按 daily-monitor 的 market/rates/events/status 范围执行或确认本轮数据刷新，产出同一批 evidence packet。",
        docs_to_read: COLLECTOR_DOCS,
        output_contract: COLLECTOR_OUTPUT_CONTRACT,
    },
    SubagentSpec {
        profile: AgentProfile::DataQuality,
        title: "采集健康检查",
        objective: "检查本轮 collector_run、source_runs、关键 missing_info 和来源新鲜度是否足以支持判断。",
        docs_to_read: &[
            "docs/sources.md",
            "docs/superpowers/specs/data-collection-sop.md",
            "work_docs/data-collector-coverage.md",
        ],
        output_contract: &[
            "列出失败源、P0 missing_info 和过期关键指标。",
            "说明本轮是否允许进入研究判断。",
            "给出需要补采或人工核验的来源。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Source,
        title: "P0 数据巡检",
        objective: "核对 USDC 供应、净发行、储备、利率、锚定、CRCL 行情和链状态是否有异常。",
        docs_to_read: &["docs/metrics/01-daily-watchlist.md"],
        output_contract: &[
            "列出触发或接近触发的 P0 指标。",
            "标注每个关键值的数据日期和 source_url。",
            "未刷新或缺失的数据写 missing_info。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Regulatory,
        title: "监管与状态事件巡检",
        objective: "检查监管、Circle 公告、核心链和 Circle 服务状态是否出现 P0 事件。",
        docs_to_read: &[
            "docs/framework/02-regulation.md",
            "docs/risk/01-warning-signals.md",
        ],
        output_contract: &[
            "只处理官方源或已落库 source_check。",
            "判断事件层级 P0/P1/P2。",
            "给出是否阻断增强结论的理由。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Risk,
        title: "日监结论合成",
        objective: "根据 P0 数据和事件，输出今日增强/降级/观察和下次触发条件。",
        docs_to_read: &["docs/playbook/01-decision-template.md"],
        output_contract: &[
            "先结论，后依据，再动作。",
            "单日价格波动不能作为唯一依据。",
            "列出解除条件或下次复盘触发。",
        ],
    },
];

const MONITOR_SUBAGENTS: &[SubagentSpec] = &[
    SubagentSpec {
        profile: AgentProfile::Collector,
        title: "监控数据收集编排",
        objective: "按 monitoring 的全量范围执行或确认本轮数据刷新，产出同一批 evidence packet。",
        docs_to_read: COLLECTOR_DOCS,
        output_contract: COLLECTOR_OUTPUT_CONTRACT,
    },
    SubagentSpec {
        profile: AgentProfile::DataQuality,
        title: "数据源健康审计",
        objective: "检查 collector_run、source_runs、stale 指标、missing_info 和官方源可达性，判断是否阻断后续复盘。",
        docs_to_read: &[
            "docs/sources.md",
            "docs/superpowers/specs/data-collection-sop.md",
            "work_docs/data-collector-coverage.md",
            "work_docs/data-source-inventory.md",
        ],
        output_contract: &[
            "列出失败源、失败类型、batch_id、source_url 和补采优先级。",
            "区分 collector_failure、stale_data、missing_info 和 official_source_blocked。",
            "输出 data-quality 是否阻断其他 workflow。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Source,
        title: "关键事实与官方源巡检",
        objective: "核对 P0 指标、Circle/SEC/监管/状态页来源是否存在当前性或口径问题。",
        docs_to_read: &["docs/sources.md", "docs/metrics/01-daily-watchlist.md"],
        output_contract: &[
            "列出 P0 指标当前值、数据日期和 source_url。",
            "说明哪些事实只能沿用历史值。",
            "对来源冲突和缺失项给出人工核验入口。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Regulatory,
        title: "紧急事件巡检",
        objective: "检查监管、Circle 公告、核心链、Circle 状态页和市场锚定是否出现 P0/P1 事件。",
        docs_to_read: &[
            "docs/framework/02-regulation.md",
            "docs/risk/01-warning-signals.md",
        ],
        output_contract: &[
            "列出官方事件、状态页异常和 P0/P1 层级。",
            "没有新事件时明确写无新增。",
            "紧急事件必须给出解除条件和下一次复核时间。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Monitor,
        title: "监控收口",
        objective: "把数据源失败、P0 指标、状态事件和 missing_info 合成正常/警戒/紧急监控结论。",
        docs_to_read: MONITOR_DOCS,
        output_contract: &[
            "结论只能是正常、警戒或紧急，并映射到观察/降级阻断。",
            "列出阻断项、失败源、missing_info、补采动作和下次自动检查触发。",
            "保存监控报告到 work_docs/monitoring/，并记录本轮 evidence packet 或 batch_id。",
        ],
    },
];

const WEEKLY_SUBAGENTS: &[SubagentSpec] = &[
    SubagentSpec {
        profile: AgentProfile::Collector,
        title: "周度数据收集编排",
        objective: "按 weekly-review 的 market/rates/sec/events 范围执行或确认本轮数据刷新，产出同一批 evidence packet。",
        docs_to_read: COLLECTOR_DOCS,
        output_contract: COLLECTOR_OUTPUT_CONTRACT,
    },
    SubagentSpec {
        profile: AgentProfile::DataQuality,
        title: "周度采集健康检查",
        objective: "先确认本周评分所需 market/rates/sec/events 数据是否刷新、失败源是否阻断评分。",
        docs_to_read: &[
            "docs/sources.md",
            "docs/superpowers/specs/data-collection-sop.md",
            "work_docs/data-collector-coverage.md",
        ],
        output_contract: &[
            "列出影响 D1-D7 的失败源和 missing_info。",
            "标注只能沿用历史值的指标。",
            "给出是否需要先补采再评分。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Source,
        title: "周度事实锚核验",
        objective: "核验 USDC、利率、SEC、监管事件和 source_url 日期，作为 competition score 的事实底座。",
        docs_to_read: &["docs/README.md", "docs/sources.md"],
        output_contract: &[
            "输出关键事实表和数据日期。",
            "区分 confirmed_fact、inference 和 missing_info。",
            "来源冲突时给出取用规则。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Competition,
        title: "D1-D7 竞争评分",
        objective: "按评分细则计算 competition score，并解释增长质量和竞争结构变化。",
        docs_to_read: &[
            "docs/metrics/04-competition-dashboard.md",
            "docs/metrics/05-competition-scoring-rubric.md",
        ],
        output_contract: &[
            "逐项输出 D1-D7 分数、权重、证据和 missing_info。",
            "给出总分、档位和动作。",
            "说明本周相对上周的关键变化。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Financial,
        title: "分销经济性复核",
        objective: "用最新 SEC / 利率数据复核 RLDC margin、储备收益和渠道成本是否支持周度结论。",
        docs_to_read: &["docs/framework/01-business-model.md"],
        output_contract: &[
            "确认 D3 是否受 RLDC margin 硬约束。",
            "标注季度数据是否为沿用值。",
            "列出影响评分的财务反证。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Regulatory,
        title: "周度监管壁垒复核",
        objective: "复核 GENIUS Act、OCC、Treasury、FDIC 等事件是否改变 D6 或触发 P0。",
        docs_to_read: &["docs/framework/02-regulation.md"],
        output_contract: &[
            "输出 D6 所需事件证据。",
            "没有新事件时明确写沿用上周/无新增。",
            "监管 P0 需覆盖 competition score 结论。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Platform,
        title: "平台化证据复核",
        objective: "核对 CPN、Arc、Other revenue share 是否支持 D7 和估值期权权重。",
        docs_to_read: &["docs/framework/04-platform-option.md"],
        output_contract: &[
            "区分使用量、TPV、收入和费用模型。",
            "测试网数据不得当收入。",
            "给出 D7 的正反证。",
        ],
    },
];

const QUARTERLY_SUBAGENTS: &[SubagentSpec] = &[
    SubagentSpec {
        profile: AgentProfile::Collector,
        title: "财报数据收集编排",
        objective: "按 quarterly-earnings 的 sec/rates/market/events 范围执行或确认本轮数据刷新，产出同一批 evidence packet。",
        docs_to_read: COLLECTOR_DOCS,
        output_contract: COLLECTOR_OUTPUT_CONTRACT,
    },
    SubagentSpec {
        profile: AgentProfile::DataQuality,
        title: "财报采集健康检查",
        objective: "确认 SEC filing、Circle 官方披露、利率和市场数据是否已刷新到本轮财报判断所需状态。",
        docs_to_read: &[
            "docs/sources.md",
            "docs/superpowers/specs/data-collection-sop.md",
            "work_docs/data-source-inventory.md",
        ],
        output_contract: &[
            "列出财报判断的阻断性缺口。",
            "标注 SEC filing / Circle 披露是否为最新可用数据。",
            "给出 T+0/T+24h/T+48h 哪一层可执行。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Source,
        title: "财报事实锚核验",
        objective: "核验最新 10-Q/10-K、Circle IR、pressroom 和关键数据日期，防止用历史快照做季度结论。",
        docs_to_read: &["docs/README.md", "docs/sources.md"],
        output_contract: &[
            "输出财报事实表、来源和日期。",
            "缺少 filing 正文或官方披露时写 missing_info。",
            "说明哪些数字只能沿用上一季。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Financial,
        title: "财报数字层",
        objective: "提取并复核 Reserve income、Distribution costs、RLDC、Other revenue、Adjusted EBITDA、平均 USDC。",
        docs_to_read: &[
            "docs/metrics/03-quarterly-earnings.md",
            "docs/framework/01-business-model.md",
        ],
        output_contract: &[
            "只用 SEC filing / 公司披露作为季度结论事实源。",
            "填五项第一优先级和十项财务复核。",
            "判断 RLDC margin 是否触发 38% 告警。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Platform,
        title: "CPN/Arc 叙事层",
        objective: "复核 CPN TPV、Arc 主网、客户、费用模型和 Other revenue 可重复性。",
        docs_to_read: &[
            "docs/framework/04-platform-option.md",
            "docs/valuation/01-scenario-model.md",
        ],
        output_contract: &[
            "把使用量、收入贡献、费用模型分开写。",
            "无法验证收入贡献时写 missing_info。",
            "给出是否满足平台化切换条件。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Competition,
        title: "财报后竞争联动",
        objective: "把季度 RLDC margin、Coinbase 暴露和 USDC 份额带回 competition score。",
        docs_to_read: &[
            "docs/metrics/04-competition-dashboard.md",
            "docs/metrics/06-validation-matrix.md",
        ],
        output_contract: &[
            "更新 D3/D7 是否与财务数据一致。",
            "说明是否需要重新计算周度 competition score。",
            "列出影响估值动作的竞争反证。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Risk,
        title: "财报结论与风险动作",
        objective: "合成 Qx 财报强化/中性/降级结论，并给出估值和风控后续动作。",
        docs_to_read: &[
            "docs/playbook/01-decision-template.md",
            "docs/risk/00-risk-map.md",
        ],
        output_contract: &[
            "先结论、依据、动作。",
            "写明 T+0/T+24h/T+48h 阶段。",
            "列出下次复盘触发和反证。",
        ],
    },
];

const VALUATION_SUBAGENTS: &[SubagentSpec] = &[
    SubagentSpec {
        profile: AgentProfile::Collector,
        title: "估值数据收集编排",
        objective: "按 valuation-decision 的 sec/rates/market/events/status 范围执行或确认本轮数据刷新，产出同一批 evidence packet。",
        docs_to_read: COLLECTOR_DOCS,
        output_contract: COLLECTOR_OUTPUT_CONTRACT,
    },
    SubagentSpec {
        profile: AgentProfile::DataQuality,
        title: "估值采集健康检查",
        objective: "确认估值和仓位动作所需财务、市场、监管、竞争和状态数据是否足够新鲜。",
        docs_to_read: &[
            "docs/sources.md",
            "docs/superpowers/specs/data-collection-sop.md",
            "work_docs/data-collector-coverage.md",
        ],
        output_contract: &[
            "列出阻断估值动作的缺口。",
            "标注可以计算但置信度降低的输入。",
            "给出补采或人工核验来源。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Source,
        title: "估值事实锚核验",
        objective: "核验估值所需价格、股本、RLDC、Other revenue、竞争和监管事实的 source_url 与日期。",
        docs_to_read: &["docs/README.md", "docs/sources.md"],
        output_contract: &[
            "输出估值输入事实表。",
            "区分最新事实和历史沿用值。",
            "无法确认当前事实时阻断加仓动作。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Financial,
        title: "基础盘估值输入",
        objective: "给出 average USDC、reserve return rate、RLDC margin、Other revenue 的最新估值输入。",
        docs_to_read: &["docs/valuation/00-valuation-framework.md"],
        output_contract: &[
            "列出基础盘公式输入和数据日期。",
            "判断利率压力是否需要重算收入。",
            "标注任何缺失的季度输入。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Competition,
        title: "倍数和竞争约束",
        objective: "用 competition score、USDC 市占率和渠道议价判断倍数区间是否需要调整。",
        docs_to_read: &[
            "docs/metrics/05-competition-scoring-rubric.md",
            "docs/valuation/01-scenario-model.md",
        ],
        output_contract: &[
            "输出 competition score 对倍数的约束。",
            "说明 D3/D7 是否压制 Bull case。",
            "给出升级/降级反证。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Regulatory,
        title: "监管折价复核",
        objective: "判断监管和银行稳定币事件是否需要加入监管折价或 C-TRIGGER。",
        docs_to_read: &[
            "docs/framework/02-regulation.md",
            "docs/risk/02-failure-conditions.md",
        ],
        output_contract: &[
            "确认是否有监管 P0。",
            "输出折价项和解除条件。",
            "禁止用媒体解读替代官方源。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Platform,
        title: "平台化估值期权复核",
        objective: "用 Other revenue、CPN、Arc 和收费模型判断平台化期权是否影响 Bull/Base 倍数。",
        docs_to_read: &[
            "docs/framework/04-platform-option.md",
            "docs/valuation/01-scenario-model.md",
        ],
        output_contract: &[
            "区分使用量、收入、收费模型和可重复收入。",
            "给出平台化倍数是否可上修。",
            "未披露收入归属时写 missing_info。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Risk,
        title: "仓位与交易动作",
        objective: "把情景、估值和风险信号映射到仓位上限与加减仓动作。",
        docs_to_read: &[
            "docs/valuation/00-valuation-framework.md",
            "docs/playbook/01-decision-template.md",
        ],
        output_contract: &[
            "输出 Bull/Base/Bear 或信用层状态。",
            "给出仓位上限、调整方向和幅度。",
            "每个动作必须绑定触发指标、阈值和反证。",
        ],
    },
];

const FRAMEWORK_SUBAGENTS: &[SubagentSpec] = &[
    SubagentSpec {
        profile: AgentProfile::Collector,
        title: "框架自检数据收集编排",
        objective: "按 framework-review 的 all 范围执行或确认本轮数据刷新，产出同一批 evidence packet。",
        docs_to_read: COLLECTOR_DOCS,
        output_contract: COLLECTOR_OUTPUT_CONTRACT,
    },
    SubagentSpec {
        profile: AgentProfile::DataQuality,
        title: "框架数据健康检查",
        objective: "审计采集覆盖、失败源、stale 指标和 missing_info，决定本轮是否允许写入框架。",
        docs_to_read: &[
            "docs/sources.md",
            "docs/superpowers/specs/data-collection-sop.md",
            "work_docs/data-collector-coverage.md",
            "work_docs/data-source-inventory.md",
        ],
        output_contract: &[
            "输出 keep/revise/defer/reject 前的数据健康结论。",
            "列出阻断写入的失败源和缺失指标。",
            "给出下一次补采或人工核验动作。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Source,
        title: "框架事实锚核验",
        objective: "确认本轮 autoresearch 使用的事实、来源优先级和日期，避免历史快照污染框架。",
        docs_to_read: &["docs/README.md", "docs/sources.md"],
        output_contract: &[
            "输出关键事实表和 source_url。",
            "区分 confirmed_fact、inference 和 missing_info。",
            "说明是否足以支持框架修改。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Financial,
        title: "框架财务桥复核",
        objective: "复核 RLDC margin、reserve return rate 和 Other revenue 是否要求修改业务模式或估值规则。",
        docs_to_read: &[
            "docs/framework/01-business-model.md",
            "docs/valuation/00-valuation-framework.md",
        ],
        output_contract: &[
            "列出财务桥对框架规则的 keep/revise/defer 影响。",
            "说明是否需要更新 RLDC 或 Other revenue 阈值。",
            "缺少季度官方披露时写 missing_info。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Regulatory,
        title: "框架监管复核",
        objective: "复核最新监管状态是否要求修改监管路径、风险层级或分销激励规则。",
        docs_to_read: &[
            "docs/framework/02-regulation.md",
            "docs/risk/01-warning-signals.md",
        ],
        output_contract: &[
            "列出监管事件对框架规则的 keep/revise/defer 影响。",
            "说明是否有 P0/P1/P2 重分类需要。",
            "禁止用媒体解读替代官方源。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Competition,
        title: "框架竞争评分复核",
        objective: "复核 D1-D7 评分和 competition score 是否要求修改权重、阈值或 missing_info 规则。",
        docs_to_read: &[
            "docs/metrics/05-competition-scoring-rubric.md",
            "docs/framework/03-competition.md",
        ],
        output_contract: &[
            "列出低分或缺失维度。",
            "说明是否需要 revise 评分规则。",
            "竞争数据不足时输出 defer。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Platform,
        title: "框架平台化复核",
        objective: "复核 CPN、Arc、Other revenue 和收费模型是否要求修改平台化验证规则。",
        docs_to_read: &[
            "docs/framework/04-platform-option.md",
            "docs/valuation/01-scenario-model.md",
        ],
        output_contract: &[
            "列出平台化规则的 keep/revise/defer 影响。",
            "区分使用量、收入和收费模型。",
            "未披露收入归属时写 missing_info。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Autoresearch,
        title: "autoresearch 微迭代治理",
        objective: "基于数据健康和矩阵结果，提出一个最小 keep/revise/defer/reject 框架迭代建议。",
        docs_to_read: &[
            "docs/autoresearch/00-loop.md",
            "docs/autoresearch/01-iteration-log.md",
            "docs/playbook/01-decision-template.md",
        ],
        output_contract: &[
            "只输出一个本轮最值得处理的迭代项。",
            "说明处置为 keep/revise/defer/reject。",
            "给出目标文件、插入位置和下次复审触发。",
        ],
    },
    SubagentSpec {
        profile: AgentProfile::Risk,
        title: "框架闭环收口",
        objective: "判断本轮框架迭代是否可写入、只观察，或需要先补数据。",
        docs_to_read: &[
            "docs/playbook/01-decision-template.md",
            "docs/risk/00-risk-map.md",
            "docs/metrics/06-validation-matrix.md",
        ],
        output_contract: &[
            "输出增强/降级/观察或 keep/revise/defer/reject 的最终动作。",
            "列出反证和解除条件。",
            "说明下一次自动复盘触发。",
        ],
    },
];

const DAILY_TEMPLATE: &str = r#"结论：今日增强 / 降级 / 观察。

依据：
1. USDC / 储备 / 净发行：
2. 利率与储备收益：
3. 锚定、链状态、监管或公司事件：

动作：

风险：

下次复盘触发：
"#;

const MONITOR_TEMPLATE: &str = r#"结论：监控状态 = 正常 / 警戒 / 紧急。

data-quality 是否阻断：

失败源：
1.

关键 P0 / 紧急事件：
1.

missing_info：

补采 / 人工核验动作：

解除条件：

下次自动检查触发：
"#;

const WEEKLY_TEMPLATE: &str = r#"结论：本周增长质量强化 / 中性 / 降级。

competition score：
D1：
D2：
D3：
D4：
D5：
D6：
D7：

依据：
1. USDC 供应、市占率和 USDC/USDT ratio：
2. 链上使用强度和增长来源：
3. DeFi / 交易所 / 收益型稳定币资金流：
4. 渠道议价和平台化证据：

动作：

风险：

下周重点：
"#;

const QUARTERLY_TEMPLATE: &str = r#"结论：Qx 财报强化 / 中性 / 降级 CRCL 研究框架。

依据：
1. Average USDC：
2. Reserve return rate：
3. RLDC margin：
4. Other revenue share：
5. 十项财务复核：
6. CPN / Arc 进展：

动作：

风险：

下次复盘触发：
"#;

const VALUATION_TEMPLATE: &str = r#"结论：当前估值/仓位动作 = 加仓 / 减仓 / 维持 / 等待。

当前情景：Bull / Base / Bear / 信用层触发
估值锚：
1. Market cap / RLDC：
2. 基础盘：
3. 期权盘：
4. 折价项：

仓位动作：
当前仓位上限：
调整幅度：
触发指标：

反证：

下次复盘触发：
"#;

const FRAMEWORK_TEMPLATE: &str = r#"结论：本轮框架迭代 = keep / revise / defer / reject。

数据健康：
1. collector_run：
2. failed source / missing_info：
3. 过期关键指标：

候选迭代：
假设：
指标：
实验：
评估：
处置：
写入文件：
修改建议：

反证：

下次复审触发：
"#;

pub fn selectors_for_workflow(workflow: WorkflowKind) -> &'static [SourceSelector] {
    workflow_spec(workflow).collect_selectors
}

pub fn print_workflow_packet(
    request: WorkflowPacketRequest<'_>,
    format: OutputFormat,
) -> Result<()> {
    let packet = build_workflow_packet(request)?;
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&packet)?);
            Ok(())
        }
        OutputFormat::Text => print_workflow_packet_text(&packet),
    }
}

pub fn build_workflow_packet(request: WorkflowPacketRequest<'_>) -> Result<WorkflowPacket> {
    let spec = workflow_spec(request.workflow);
    let collection_enabled = request.collector_run.is_some();
    let latest_observations = selected_latest_observations(
        request.db.latest_observations()?,
        spec.metric_codes,
        request.limit,
    );
    let observation_history = selected_observation_history(
        request
            .db
            .recent_observations(request.limit.saturating_mul(80).max(500))?,
        spec.metric_codes,
        request.history_limit,
    );

    Ok(WorkflowPacket {
        workflow: spec.label.to_string(),
        generated_at: Utc::now().to_rfc3339(),
        database: request.database.display().to_string(),
        execution_model: "Rust 只生成证据包和调度合同；sub agent 负责评分、判断、估值和最终决策合成。",
        collector_run: request.collector_run,
        collect_scope: spec
            .collect_selectors
            .iter()
            .map(|selector| source_selector_label(*selector))
            .collect(),
        docs_to_read: spec.docs_to_read.to_vec(),
        orchestrator_steps: spec.orchestrator_steps.to_vec(),
        hard_gates: spec.hard_gates.to_vec(),
        quality_gates: spec.quality_gates.to_vec(),
        subagent_dispatch: build_subagent_tasks(&spec, request.database, collection_enabled),
        final_output_template: spec.final_output_template,
        evidence: WorkflowEvidence {
            summary: request.db.summary()?,
            latest_observations,
            observation_history,
            recent_source_runs: request
                .db
                .recent_source_runs(request.limit.saturating_mul(20).max(100))?,
            recent_filings: request.db.recent_filings(request.limit)?,
            recent_events: request.db.recent_events(request.limit)?,
            missing_items: request.db.missing_items()?,
        },
    })
}

fn workflow_spec(workflow: WorkflowKind) -> WorkflowSpec {
    match workflow {
        WorkflowKind::DailyMonitor => WorkflowSpec {
            label: "daily-monitor",
            collect_selectors: DAILY_SELECTORS,
            docs_to_read: DAILY_DOCS,
            metric_codes: DAILY_METRICS,
            subagents: DAILY_SUBAGENTS,
            orchestrator_steps: COMMON_ORCHESTRATOR_STEPS,
            hard_gates: DAILY_HARD_GATES,
            quality_gates: COMMON_QUALITY_GATES,
            final_output_template: DAILY_TEMPLATE,
        },
        WorkflowKind::Monitoring => WorkflowSpec {
            label: "monitoring",
            collect_selectors: MONITOR_SELECTORS,
            docs_to_read: MONITOR_DOCS,
            metric_codes: MONITOR_METRICS,
            subagents: MONITOR_SUBAGENTS,
            orchestrator_steps: COMMON_ORCHESTRATOR_STEPS,
            hard_gates: MONITOR_HARD_GATES,
            quality_gates: COMMON_QUALITY_GATES,
            final_output_template: MONITOR_TEMPLATE,
        },
        WorkflowKind::WeeklyReview => WorkflowSpec {
            label: "weekly-review",
            collect_selectors: WEEKLY_SELECTORS,
            docs_to_read: WEEKLY_DOCS,
            metric_codes: WEEKLY_METRICS,
            subagents: WEEKLY_SUBAGENTS,
            orchestrator_steps: COMMON_ORCHESTRATOR_STEPS,
            hard_gates: WEEKLY_HARD_GATES,
            quality_gates: COMMON_QUALITY_GATES,
            final_output_template: WEEKLY_TEMPLATE,
        },
        WorkflowKind::QuarterlyEarnings => WorkflowSpec {
            label: "quarterly-earnings",
            collect_selectors: QUARTERLY_SELECTORS,
            docs_to_read: QUARTERLY_DOCS,
            metric_codes: QUARTERLY_METRICS,
            subagents: QUARTERLY_SUBAGENTS,
            orchestrator_steps: COMMON_ORCHESTRATOR_STEPS,
            hard_gates: QUARTERLY_HARD_GATES,
            quality_gates: COMMON_QUALITY_GATES,
            final_output_template: QUARTERLY_TEMPLATE,
        },
        WorkflowKind::ValuationDecision => WorkflowSpec {
            label: "valuation-decision",
            collect_selectors: VALUATION_SELECTORS,
            docs_to_read: VALUATION_DOCS,
            metric_codes: VALUATION_METRICS,
            subagents: VALUATION_SUBAGENTS,
            orchestrator_steps: COMMON_ORCHESTRATOR_STEPS,
            hard_gates: VALUATION_HARD_GATES,
            quality_gates: COMMON_QUALITY_GATES,
            final_output_template: VALUATION_TEMPLATE,
        },
        WorkflowKind::FrameworkReview => WorkflowSpec {
            label: "framework-review",
            collect_selectors: FRAMEWORK_SELECTORS,
            docs_to_read: FRAMEWORK_DOCS,
            metric_codes: FRAMEWORK_METRICS,
            subagents: FRAMEWORK_SUBAGENTS,
            orchestrator_steps: COMMON_ORCHESTRATOR_STEPS,
            hard_gates: FRAMEWORK_HARD_GATES,
            quality_gates: COMMON_QUALITY_GATES,
            final_output_template: FRAMEWORK_TEMPLATE,
        },
    }
}

fn build_subagent_tasks(
    spec: &WorkflowSpec,
    database: &Path,
    collection_enabled: bool,
) -> Vec<SubagentTask> {
    spec.subagents
        .iter()
        .map(|subagent| {
            let profile = profile_label(subagent.profile);
            let context_command = if subagent.profile == AgentProfile::Collector {
                let mut command = format!(
                    "cargo run --release -- --database {} decision-pack --workflow {} --format json",
                    database.display(),
                    spec.label
                );
                if !collection_enabled {
                    command.push_str(" --no-collect");
                }
                command
            } else {
                format!(
                    "cargo run --release -- --database {} agent-context --profile {} --format json --no-collect",
                    database.display(),
                    profile
                )
            };
            SubagentTask {
                profile,
                title: subagent.title,
                objective: subagent.objective,
                context_command,
                docs_to_read: subagent.docs_to_read.to_vec(),
                output_contract: subagent.output_contract.to_vec(),
            }
        })
        .collect()
}

fn selected_latest_observations(
    observations: Vec<RecentObservation>,
    metric_codes: &[&str],
    limit: usize,
) -> Vec<RecentObservation> {
    let mut selected = observations
        .into_iter()
        .filter(|obs| metric_codes.contains(&obs.metric_code.as_str()))
        .collect::<Vec<_>>();
    selected.sort_by_key(|obs| metric_rank(&obs.metric_code, metric_codes));
    selected.truncate(limit);
    selected
}

fn selected_observation_history(
    observations: Vec<RecentObservation>,
    metric_codes: &[&str],
    history_limit: usize,
) -> Vec<RecentObservation> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut selected = Vec::new();

    for obs in observations {
        if !metric_codes.contains(&obs.metric_code.as_str()) {
            continue;
        }
        let count = counts.entry(obs.metric_code.clone()).or_default();
        if *count >= history_limit {
            continue;
        }
        *count += 1;
        selected.push(obs);
    }

    selected.sort_by(|left, right| {
        metric_rank(&left.metric_code, metric_codes)
            .cmp(&metric_rank(&right.metric_code, metric_codes))
            .then_with(|| right.created_at.cmp(&left.created_at))
    });
    selected
}

fn metric_rank(metric_code: &str, metric_codes: &[&str]) -> usize {
    metric_codes
        .iter()
        .position(|code| *code == metric_code)
        .unwrap_or(usize::MAX)
}

fn print_workflow_packet_text(packet: &WorkflowPacket) -> Result<()> {
    println!("# CRCL decision pack");
    println!("workflow={}", packet.workflow);
    println!("generated_at={}", packet.generated_at);
    println!("database={}", packet.database);
    println!("execution_model={}", packet.execution_model);
    match &packet.collector_run {
        Some(run) => println!(
            "collector_run=ok:{} warn:{} batch_id={}",
            run.ok_sources, run.warn_sources, run.batch_id
        ),
        None => println!("collector_run=skipped"),
    }
    println!("collect_scope={}", packet.collect_scope.join(","));

    println!("\n## docs to read");
    for doc in &packet.docs_to_read {
        println!("- {doc}");
    }

    println!("\n## orchestrator steps");
    for step in &packet.orchestrator_steps {
        println!("- {step}");
    }

    println!("\n## subagent dispatch");
    for task in &packet.subagent_dispatch {
        println!("- {} | {} | {}", task.profile, task.title, task.objective);
        println!("  command: {}", task.context_command);
        println!("  docs: {}", task.docs_to_read.join(", "));
        println!("  contract: {}", task.output_contract.join(" / "));
    }

    println!("\n## hard gates");
    for gate in &packet.hard_gates {
        println!("- {gate}");
    }

    println!("\n## quality gates");
    for gate in &packet.quality_gates {
        println!("- {gate}");
    }

    println!("\n## database summary");
    println!(
        "collection_batches={} source_runs={} observations={} filings={} events={} missing_items={}",
        packet.evidence.summary.collection_batches,
        packet.evidence.summary.source_runs,
        packet.evidence.summary.observations,
        packet.evidence.summary.filings,
        packet.evidence.summary.events,
        packet.evidence.summary.missing_items
    );

    println!("\n## latest observations");
    for obs in &packet.evidence.latest_observations {
        println!(
            "- {} | {} | {} | {} | observed_at={} | created_at={} | {} | {}",
            obs.priority,
            obs.metric_code,
            obs.category,
            display_value(obs.value_num, obs.value_text.as_deref(), &obs.unit),
            obs.observed_at,
            obs.created_at,
            obs.metric_name,
            obs.source_url
        );
    }

    println!("\n## observation history");
    for obs in &packet.evidence.observation_history {
        println!(
            "- {} | {} | {} | observed_at={} | created_at={} | {}",
            obs.metric_code,
            obs.category,
            display_value(obs.value_num, obs.value_text.as_deref(), &obs.unit),
            obs.observed_at,
            obs.created_at,
            obs.source
        );
    }

    println!("\n## recent filings");
    for filing in &packet.evidence.recent_filings {
        println!(
            "- {} | {} | filing_date={} | report_date={} | doc={} | {}",
            filing.company,
            filing.form,
            filing.filing_date,
            filing.report_date.as_deref().unwrap_or("-"),
            filing.primary_doc.as_deref().unwrap_or("-"),
            filing.source_url
        );
    }

    println!("\n## recent events");
    for event in &packet.evidence.recent_events {
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
    for item in &packet.evidence.missing_items {
        println!(
            "- {} | {} | {} | {} | {}",
            item.priority, item.metric_code, item.collector, item.reason, item.source_hint
        );
    }

    println!("\n## final output template");
    println!("{}", packet.final_output_template.trim_end());

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

fn profile_label(profile: AgentProfile) -> &'static str {
    match profile {
        AgentProfile::Collector => "collector",
        AgentProfile::DataQuality => "data-quality",
        AgentProfile::Source => "source",
        AgentProfile::Monitor => "monitor",
        AgentProfile::Financial => "financial",
        AgentProfile::Regulatory => "regulatory",
        AgentProfile::Competition => "competition",
        AgentProfile::Platform => "platform",
        AgentProfile::Risk => "risk",
        AgentProfile::Autoresearch => "autoresearch",
    }
}
