# CRCL sub agent 决策工作流

生成日期：2026-06-16

## 定位

本项目的 Rust 程序是工具层，不直接替代研究判断。

Rust 负责：

1. 按 workflow 刷新对应数据源。
2. 从 SQLite 取出最新 observation、历史 observation、filing、event 和 missing item。
3. 生成 sub agent dispatch、必读文档、硬性门槛、输出模板。

Sub agent / 总控 agent 负责：

1. 按 `docs/playbook/`、`docs/metrics/`、`docs/valuation/` 执行评分和判断。
2. 输出日常监控结论、周度 competition score、季度财报结论、估值和仓位动作。
3. 对 missing_info、反证和下次复盘触发负责。

当前本地实现是 Rust CLI `agent-run`：

1. 先由 collector sub agent 调用或确认 Rust `decision-pack --format json`，生成同一批 evidence packet。
2. 拆分 data-quality、source、financial、regulatory、competition、platform、risk/orchestrator 等 sub agent。
3. 用文档阈值执行规则化评分、判断和仓位映射。
4. 由总控按信用层/风险 > 财务 > 监管 > 竞争 > 平台化 > 筹码合成最终结论。

## 新命令

日常使用推荐直接在 Codex 里输入提示词，不需要先记命令：

```text
做一次 CRCL 日监，自动调用项目 agent，自己判断要不要跑命令，最后给我结论、P0、missing_info 和下一步。
```

```text
做一次 CRCL 周度复盘，自动调用项目 agent，允许刷新最新数据，最后给我 competition score、阻断项和动作。
```

```text
做一次 CRCL 框架自检，自动调用项目 agent，判断 keep / revise / defer / reject，不要直接改文档。
```

```text
基于最近一次完整跑批结果，调用总分析 agent，生成 CRCL 总分析报告并保存到 work_docs/agent_runs/。
```

Codex 看到这类提示词后，应按根目录 `AGENTS.md` 选择 workflow、数据刷新策略和 agent 组合。下面的命令是 Codex 可选择使用的本地自动执行层。

Prompt 驱动运行默认保存最终产物到 `work_docs/agent_runs/`；用户明确说“不保存”时才跳过 `--save`。

自动执行层：

```bash
cargo run --release -- agent-run --workflow daily-monitor
cargo run --release -- agent-run --workflow monitoring
cargo run --release -- agent-run --workflow weekly-review
cargo run --release -- agent-run --workflow quarterly-earnings
cargo run --release -- agent-run --workflow valuation-decision
cargo run --release -- agent-run --workflow framework-review
cargo run --release -- full-analysis --save
```

JSON 输出：

```bash
cargo run --release -- agent-run --workflow weekly-review --format json
```

只读已有数据库，不刷新外部数据：

```bash
cargo run --release -- agent-run --workflow weekly-review --no-collect
```

保存可复盘产物：

```bash
cargo run --release -- agent-run --workflow daily-monitor --save
```

`--save` 默认写到 `work_docs/agent_runs/`，会同时保存 Markdown 报告和 JSON 结构化结果。

估值/仓位流程可传当前仓位，用于判断具体加减仓动作：

```bash
cargo run --release -- agent-run --workflow valuation-decision --current-position-pct 50
```

Rust 工具层证据包：

```bash
cargo run --release -- decision-pack --workflow daily-monitor
cargo run --release -- decision-pack --workflow monitoring
cargo run --release -- decision-pack --workflow weekly-review
cargo run --release -- decision-pack --workflow quarterly-earnings
cargo run --release -- decision-pack --workflow valuation-decision
cargo run --release -- decision-pack --workflow framework-review
```

JSON 输出：

```bash
cargo run --release -- decision-pack --workflow weekly-review --format json
```

只读已有数据库，不刷新外部数据：

```bash
cargo run --release -- decision-pack --workflow weekly-review --no-collect
```

## Workflow 覆盖

| Workflow | 目标 | 默认刷新范围 | 主要输出 |
| --- | --- | --- | --- |
| `daily-monitor` | 判断当天是否出现 P0 信号 | market、rates、events、status | 今日增强 / 降级 / 观察，触发指标，动作 |
| `monitoring` | 巡检数据源失败、stale 指标、P0 missing_info 和紧急事件 | all | 正常 / 警戒 / 紧急，阻断项，补采动作 |
| `weekly-review` | 计算周度竞争评分和增长质量 | market、rates、sec、events | D1-D7、competition score、周度动作 |
| `quarterly-earnings` | 财报数字层和叙事层复核 | sec、rates、market、events | 财报强化 / 中性 / 降级，T+0/T+24h/T+48h 动作 |
| `valuation-decision` | 估值、情景、仓位和交易动作 | sec、rates、market、events、status | Bull/Base/Bear、仓位上限、加减仓动作 |
| `framework-review` | 数据健康和 autoresearch 框架自检 | all | keep / revise / defer / reject，写入建议和补采动作 |
| `full-analysis` | 完整跑批后的跨 workflow 总分析 | 已保存 final + SQLite | 观察/Base/Bear、核心指标、missing_info、下一步和保存路径 |

## Dispatch 规则

`decision-pack` 输出中的 `subagent_dispatch` 是总控 agent 的执行合同。
Rust `agent-run` 会消费这个合同和 evidence packet，直接产出结论；外部 LLM / Codex sub agent 也可以复用同一 JSON。

每个子任务包含：

- `profile`：调用哪个 `agent-context --profile`；`collector` profile 是采集编排。
- `context_command`：collector 使用 workflow 对应 `decision-pack` 命令生成 evidence packet；后续 agent 使用同一 SQLite 证据批次的只读上下文命令。
- `docs_to_read`：该子 agent 必读规则文件。
- `output_contract`：必须返回的字段和判断边界。

总控执行顺序：

1. 执行 collector sub agent：按 workflow 运行 `decision-pack` 或在只读模式确认已有 evidence packet。
2. 执行前置 sub agent：data-quality、source。
3. 执行专业 sub agent：financial、regulatory、competition、platform、autoresearch、risk/orchestrator。
4. 各 sub agent 按文档阈值输出证据、判断、missing_info、反证。
5. 总控按优先级合成最终结论：信用层/风险 > 财务 > 监管 > 竞争 > 平台化 > 筹码。
6. 最终输出必须套 `final_output_template` 的语义字段。

## 强制边界

1. Rust 输出不是投资结论，只是证据包和调度合同。
2. collector 必须先完成或确认本轮 evidence packet；后续 agent 不得自行刷新造成口径不一致。
3. data-quality 和 source 是所有自动闭环的前置检查；P0 missing_info 或来源阻断未解除时不能输出增强或 keep。
4. 周度 competition score 必须由 D1-D7 逐项得出。
5. 财报 T+0 只填事实数字，不输出仓位动作。
6. 估值和仓位动作必须来自矩阵输出，不从单条新闻直接改变倍数。
7. 任何关键数据缺失时写 `missing_info`，不能补乐观推断。

## 验证命令

```bash
cargo run --release -- agent-run --workflow daily-monitor --no-collect --limit 8
cargo run --release -- agent-run --workflow monitoring --no-collect --format json --limit 12
cargo run --release -- agent-run --workflow weekly-review --no-collect --format json --limit 16
cargo run --release -- agent-run --workflow quarterly-earnings --no-collect --limit 16
cargo run --release -- agent-run --workflow valuation-decision --no-collect --limit 16 --current-position-pct 50
cargo run --release -- agent-run --workflow framework-review --no-collect --limit 16
cargo run --release -- full-analysis --save
```
