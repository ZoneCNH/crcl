# 启动

执行一次
```bash
 cargo run --release -- cron run all

  这个不会安装 crontab，只是立刻运行一次全任务：

  - monitor
  - daily
  - weekly
  - decision
  - quarterly
  - framework
  - full-analysis
```

定时执行
```bash
cargo run --release -- cron install

 会在系统 crontab 安装定时命令
  monitor    08:20 起，每 2 小时到 23 点区间运行
  daily      每天 08:10 运行
  weekly     每周一 09:30 运行
  decision   周一到周五 16:00 运行
  quarterly  2/5/8/11 月 15 日 10:00 运行
  framework  每周六 10:30 运行
  full-analysis 每周六 11:15 运行，并且 cron run all 会在六个任务后自动生成

# 取消
• 在仓库根目录执行：

  cargo run --release -- cron remove

  这会从系统 crontab 里移除 CRCL 自动任务块，也就是关闭已启动的定时任务。

  移除后可以确认一下：

  crontab -l
```


# crcl

本仓库用于维护 CRCL（Circle Internet Group）研究框架。

入口文档：`docs/README.md`

核心目标：

- 跟踪 USDC 规模、储备收益、分销成本、监管规则和平台化进展。
- 把财务、链上、监管、竞争、估值、风险信号统一成可复盘框架。
- 用固定流程生成日常监控、周度复盘、季度财报检查和交易决策结论。
- 用 data-quality / source 前置检查保证自动结论不绕过采集失败、过期数据和 missing_info。

说明：本文档体系是研究框架，不是个性化投资建议。

## 运行时入口

推荐在 Codex 里直接输入自然语言任务，让 Codex 根据 `AGENTS.md` 自动判断 workflow、是否刷新数据、是否调用项目级 subagents。例如：

```text
做一次 CRCL 周度复盘，自动调用项目 agent，自己判断要不要跑命令，最后给我结论、missing_info 和下一步。
```

```text
做一次 CRCL 估值和仓位决策，当前仓位 50%，自动调用 agent，允许刷新最新数据，最后直接给结果。
```

```text
做一次框架自检，自动调用 agent，判断本轮是 keep / revise / defer / reject，不要直接改文档。
```

自然语言触发的自动运行默认会把最终 Markdown/JSON 产物保存到 `work_docs/agent_runs/`；如果只想看终端结论，可以明确说“不保存”。

Rust 程序只做工具层：采集数据、落库、整理证据包和生成 sub agent 调度合同。
评分、财报判断、估值和仓位动作由 sub agent / 总控 agent 按 `docs/playbook/`、`docs/metrics/` 与 `docs/valuation/` 执行。

只采集币安现货长线辅助数据：

```bash
cargo run --release -- collect --source binance-spot
```

该入口只拉 Binance Spot 的 `CRCLBUSDT`（CRCLB tokenized bStock）和 `USDCUSDT`，保存 `exchangeInfo`、24h ticker、top100 order book snapshot、完整 1d K 线。日线写入 `binance_spot_klines`，汇总指标写入 `observations`。这些指标只作为币安场内辅助观察，不替代 NYSE:CRCL 正股行情、Circle 官方披露或全交易所 USDC balance。

`agent-run` 是 Codex 可自动选择的本地执行层，会调用 Rust 证据包，再由 Rust 规则化总控合成可保存的结论：

```bash
cargo run --release -- agent-run --workflow daily-monitor      # 日常监控结论
cargo run --release -- agent-run --workflow monitoring         # 数据源和紧急事件监控
cargo run --release -- agent-run --workflow weekly-review      # 周度复盘评分和结论
cargo run --release -- agent-run --workflow quarterly-earnings # 季度财报判断
cargo run --release -- agent-run --workflow valuation-decision # 估值 / 仓位 / 交易决策
cargo run --release -- agent-run --workflow framework-review   # 数据健康 + autoresearch 框架自检闭环
cargo run --release -- full-analysis --save                    # 完整跑批后的总分析报告
```

只读已有数据库时：

```bash
cargo run --release -- agent-run --workflow weekly-review --no-collect
cargo run --release -- agent-run --workflow valuation-decision --no-collect --current-position-pct 50
```

需要保存可复盘产物时加 `--save`，默认写到 `work_docs/agent_runs/`：

```bash
cargo run --release -- agent-run --workflow daily-monitor --save
```

## codex exec + cron 定时任务

定时自动化走真实 `codex exec`，由 Codex 按 `AGENTS.md` 和 `.codex/agents/` 调用项目级 subagents。每条 cron 分别执行一个 workflow，并把结果写到对应分类目录：

| 任务 | 默认频率 | 保存目录 |
| --- | --- | --- |
| `monitor` | 08:20-23:20 每 2 小时 | `work_docs/monitoring/` |
| `daily` | 每天 08:10 | `work_docs/daily-monitor/` |
| `weekly` | 每周一 09:30 | `work_docs/weekly-review/` |
| `decision` | 周一至周五 16:00 | `work_docs/valuation-decision/` |
| `quarterly` | 2/5/8/11 月 15 日 10:00 | `work_docs/quarterly-earnings/` |
| `framework` | 每周六 10:30 | `work_docs/framework-review/` |
| `full-analysis` | 每周六 11:15 | `work_docs/agent_runs/` |

一键安装或更新 cron：

```bash
cargo run --release -- cron install
```

查看将安装的 crontab：

```bash
cargo run --release -- cron print
```

立即干跑全套任务，不实际调用 Codex：

```bash
cargo run --release -- cron run all --dry-run
```

立即顺序运行全套任务：

```bash
cargo run --release -- cron run all
```

`cron run all` 会按顺序运行 monitor、daily、weekly、decision、quarterly、framework，并在最后自动执行：

```bash
cargo run --release -- full-analysis --save
```

总分析报告会读取最新分类 workflow final 和本地 SQLite，不重新刷新外部数据，默认保存到 `work_docs/agent_runs/`。

只运行某个任务：

```bash
cargo run --release -- cron run monitor
cargo run --release -- cron run weekly
cargo run --release -- cron run full-analysis
```

移除自动任务：

```bash
cargo run --release -- cron remove
```

默认 cron 为了无人值守会给 `codex exec` 使用 `--dangerously-bypass-approvals-and-sandbox`。如果想让 Codex 保持沙箱模式，可以安装前设置：

```bash
CRCL_CODEX_SAFE_MODE=1 cargo run --release -- cron install
```

估值仓位任务如果要带当前仓位，安装或运行前设置：

```bash
CRCL_CURRENT_POSITION_PCT=50 cargo run --release -- cron install
```

`decision-pack` 是 Rust 工具层入口，只输出 evidence packet 和调度合同：

```bash
cargo run --release -- decision-pack --workflow daily-monitor
cargo run --release -- decision-pack --workflow monitoring
cargo run --release -- decision-pack --workflow weekly-review --format json
```

机器消费 JSON 时直接使用 `--format json`。

底层命令仍是 `cargo run --release -- decision-pack --workflow ...`。
`decision-pack` 默认先刷新该 workflow 需要的数据源；只读取现有数据库时加 `--no-collect`；给外部 agent 消费时加 `--format json`。

自动闭环的固定顺序是：

1. `crcl-data-quality-auditor` 检查 collector_run、source_runs、stale 指标和 missing_info。
2. `crcl-source-verifier` 核验事实、来源优先级和数据日期。
3. 财务、监管、竞争、平台化和风险 agent 按 workflow 分工输出矩阵判断。
4. `agent-run --workflow framework-review` 额外调用 autoresearch curator 规则，给出 keep / revise / defer / reject 的框架迭代建议。
