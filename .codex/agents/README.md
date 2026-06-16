# CRCL Codex Subagents

这些是当前项目的项目级 Codex custom agents，放在 `.codex/agents/` 下。

Codex 会按 `name` 字段识别 agent。使用时可以直接要求主线程按角色并行派发，例如：

```text
根据本周数据做一次 CRCL 周度复盘。请并行启动 crcl-source-verifier、crcl-competition-score 和 crcl-risk-decision，等结果都回来后汇总。
```

## Agent 分工

| Agent | 适合任务 | 主要输出 |
| --- | --- | --- |
| `crcl-source-verifier` | 官方来源、日期、数据口径和 missing_info 核验 | 事实表、来源优先级、需要复核的数据 |
| `crcl-financial-valuation` | 财报、RLDC、Other revenue、估值锚和仓位因子 | 财务桥、情景影响、估值动作 |
| `crcl-regulatory-watch` | GENIUS Act、OCC、Treasury、FDIC、SEC/CFTC 监管事件 | P0/P1/P2 监管判断、影响层级、动作 |
| `crcl-competition-score` | 稳定币竞争、Coinbase/Base、USDT、RWA、银行稳定币 | D1-D7 打分、competition score、升级规则 |
| `crcl-platform-option` | CPN、Arc、Other revenue 和平台化验证 | 弱/中/强验证、missing_info、估值切换条件 |
| `crcl-risk-decision` | 多矩阵冲突、风险优先级、最终动作收口 | 增强/降级/观察、风险、下次触发 |
| `crcl-autoresearch-curator` | 新假设、阈值、模板或来源的迭代治理 | keep/revise/defer/reject 与写入建议 |

## 使用原则

- 并行 agent 适合读文档、核来源、打分、复核矩阵。
- 多个 agent 不应同时改同一个文件；需要写入时由主线程统一落盘。
- 所有结论都必须回到项目文档的三类动作：增强、降级、观察。
- 财务和监管事实优先用 SEC filing、Circle 官方披露和美国监管机构文件；二级数据平台只能作为监控线索。

## Rust 采集器调用

项目内 Rust 数据采集器是 subagent 刷新事实的统一入口。所有 Rust 执行都必须使用 release 模式：

```bash
cargo run --release -- agent-context --profile source
```

`agent-context` 会按角色先刷新对应来源，再输出数据库统计、最近 source runs、observations、filings、events 和 missing items。只需要读取已有库内数据时，加 `--no-collect`：

```bash
cargo run --release -- agent-context --profile source --no-collect
```

默认输出是给人阅读的文本。给 agent 串联、自动汇总或需要稳定字段时，使用 JSON：

```bash
cargo run --release -- agent-context --profile source --format json
```

JSON 输出包含本轮 `collector_run.batch_id`、按角色过滤后的 `recent_source_runs`、按指标去重的 `latest_observations`、`recent_filings`、`recent_events` 和 `missing_items`。`latest_observations` 是每个 `metric_code` 的最新记录，避免 agent 从重复历史采集中误取旧值。

| Agent | 推荐 profile | 覆盖的 Rust source |
| --- | --- | --- |
| `crcl-source-verifier` | `source` | `all` |
| `crcl-financial-valuation` | `financial` | `sec`、`rates`、`market` |
| `crcl-regulatory-watch` | `regulatory` | `events`、`status` |
| `crcl-competition-score` | `competition` | `market`、`events` |
| `crcl-platform-option` | `platform` | `events`、`sec`、`market` |
| `crcl-risk-decision` | `risk` | `all` |
| `crcl-autoresearch-curator` | `autoresearch` | `all` |

保留的底层命令也必须带 `--release`：

```bash
cargo run --release -- collect --source all
cargo run --release -- summary
cargo run --release -- missing
cargo test --release
```
