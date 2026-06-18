# CRCL Codex 运行规则

本仓库默认使用中文回答。

## 默认目标

当用户在本仓库提出 CRCL 日监、周报、财报复核、估值/仓位、框架自检、监管事件、竞争复盘或平台化复核时，Codex 应按项目研究框架完成最终判断，而不是只告诉用户该运行什么命令。

## Prompt 驱动模式

用户说“自动调用 agent”“让 agent 自己跑”“用项目 agent 复盘”“直接给最后结果”时，视为明确授权使用项目级 Codex subagents。此时按以下流程执行：

1. 先识别任务类型：
   - 日监：`daily-monitor`
   - 监控 / 数据健康巡检 / 紧急事件巡检：`monitoring`
   - 周报 / 周度复盘：`weekly-review`
   - 财报 / 季报复核：`quarterly-earnings`
   - 估值 / 仓位 / 交易决策：`valuation-decision`
   - 框架自检 / autoresearch / keep-revise-defer-reject：`framework-review`
   - 完整跑批后总分析 / 跨 workflow 收口：`full-analysis`
2. 判断是否刷新数据：
   - 用户说“当前”“最新”“今天”“本周”“运行一次”“允许刷新”时，默认允许刷新外部数据。
   - 用户说“只读”“不联网”“用已有数据库”“不刷新”时，必须加 `--no-collect`。
3. 需要同一批证据时，优先先运行对应 `decision-pack` 或 `agent-run` 入口生成 evidence packet，避免各 agent 自己刷新出不同口径。
4. 派发 agent 时，必须先做 `crcl-data-collector` 数据收集编排；它可以调用现有 `decision-pack` / `collect` / `agent-context --profile collector` 命令，也可以按 `docs/sources.md` 和 `data-collection-sop.md` 做补充采集。采集完成后，后续 agent 必须读取同一批 evidence packet，避免各自刷新出不同口径。
5. 数据收集完成后，必须先做 `crcl-data-quality-auditor` 和 `crcl-source-verifier` 前置检查，再按任务类型派发财务、监管、竞争、平台化、风险或 autoresearch agent。
6. 每个 agent 只输出证据、判断、missing_info、反证和下一步；最终由主线程或 `crcl-risk-decision` 收口。
7. Prompt 驱动运行默认保存最终产物到 `work_docs/agent_runs/`；除非用户明确说“不保存”，运行本地 `agent-run` 入口时应追加 `--save`。

## Agent 选择

- 日监：`crcl-data-collector`、`crcl-data-quality-auditor`、`crcl-source-verifier`、`crcl-regulatory-watch`、`crcl-risk-decision`
- 监控：`crcl-data-collector`、`crcl-data-quality-auditor`、`crcl-source-verifier`、`crcl-regulatory-watch`、`crcl-monitor-guard`
- 周报：`crcl-data-collector`、`crcl-data-quality-auditor`、`crcl-source-verifier`、`crcl-competition-score`、`crcl-financial-valuation`、`crcl-regulatory-watch`、`crcl-platform-option`、`crcl-risk-decision`
- 财报复核：`crcl-data-collector`、`crcl-data-quality-auditor`、`crcl-source-verifier`、`crcl-financial-valuation`、`crcl-platform-option`、`crcl-competition-score`、`crcl-risk-decision`
- 估值/仓位：`crcl-data-collector`、`crcl-data-quality-auditor`、`crcl-source-verifier`、`crcl-financial-valuation`、`crcl-competition-score`、`crcl-regulatory-watch`、`crcl-platform-option`、`crcl-risk-decision`
- 框架自检：`crcl-data-collector`、`crcl-data-quality-auditor`、`crcl-source-verifier`、`crcl-financial-valuation`、`crcl-regulatory-watch`、`crcl-competition-score`、`crcl-platform-option`、`crcl-autoresearch-curator`、`crcl-risk-decision`
- 完整跑批后总分析：`crcl-total-analysis`；它读取最新 monitoring、daily-monitor、weekly-review、valuation-decision、quarterly-earnings、framework-review 产物和本地 SQLite，不重新刷新外部数据，统一收口成总报告

## 命令判断

Codex 可以根据任务需要自行选择是否运行命令。

常规入口：

```bash
cargo run --release -- agent-run --workflow daily-monitor
cargo run --release -- agent-run --workflow monitoring
cargo run --release -- agent-run --workflow weekly-review
cargo run --release -- agent-run --workflow quarterly-earnings
cargo run --release -- agent-run --workflow valuation-decision --current-position-pct 50
cargo run --release -- agent-run --workflow framework-review
cargo run --release -- full-analysis --save
```

只读已有数据库时，在命令后追加：

```bash
--no-collect
```

需要保存复盘产物时追加：

```bash
--save
```

`--save` 默认输出到 `work_docs/agent_runs/`。如果需要别的目录，追加 `--out-dir <path>`。

完整跑批后总分析入口：

```bash
cargo run --release -- full-analysis --save
```

该入口默认读取 `work_docs/<workflow>/` 最新 final 和本地 SQLite，不重新刷新外部数据，保存到 `work_docs/agent_runs/`。

cron / `codex exec` 定时任务默认按类别保存到：

```bash
work_docs/monitoring/
work_docs/daily-monitor/
work_docs/weekly-review/
work_docs/quarterly-earnings/
work_docs/valuation-decision/
work_docs/framework-review/
work_docs/agent_runs/
```

如果用户明确要求真实 Codex subagents 逐个复核，先派 `crcl-data-collector` 生成同一批证据包，再让后续 subagents 读取对应 `agent-context --profile ... --no-collect` 上下文，避免多次刷新造成口径不一致。

## 输出格式

最终回答必须包含：

1. 结论：增强 / 降级 / 观察，或 Bull / Base / Bear，或 keep / revise / defer / reject。
2. data-quality 是否阻断。
3. 关键证据：指标值、数据日期、来源。
4. missing_info。
5. 反证 / 解除条件。
6. 下一步动作或下次复盘触发。
7. 已保存的 `work_docs/agent_runs/*.md` 路径；如果用户明确不保存，则说明未保存。

不要输出个性化投资建议。不要只写“利好”“利空”。
