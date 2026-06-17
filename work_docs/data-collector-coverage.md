# CRCL 数据采集项目覆盖说明

生成日期：2026-06-15

## 项目定位

本 Rust 项目把 `docs/` 研究框架里的 P0/P1/P2 数据需求落到本地 SQLite。

默认数据库位置：

```bash
data/crcl_research.sqlite
```

初始化：

```bash
cargo run --release -- init-db
```

全量采集：

```bash
cargo run --release -- collect --source all
```

查看库内统计：

```bash
cargo run --release -- summary
```

查看暂时无法自动采集的数据：

```bash
cargo run --release -- missing
```

Subagent 统一调用入口：

```bash
cargo run --release -- agent-context --profile source
```

按角色可切换 `source`、`financial`、`regulatory`、`competition`、`platform`、`risk`、`autoresearch`。只读取已有库内数据时使用：

```bash
cargo run --release -- agent-context --profile source --no-collect
```

给 agent 或脚本消费时使用 JSON 输出：

```bash
cargo run --release -- agent-context --profile source --format json
```

`agent-context` 会写入 `collection_batches`，每轮采集带 `batch_id`、`profile` 和 `selector`。输出中的 `latest_observations` 按 `metric_code` 去重，只保留当前最新事实；`recent_source_runs`、`recent_events` 和 `missing_items` 会按 profile 做基础过滤。

数据收集 agent 入口：

```bash
cargo run --release -- agent-context --profile collector --format json
```

`collector` profile 覆盖 `all` 采集范围，用来执行或确认本轮数据收集状态。真实 subagent 工作流中，collector 应先生成或确认同一批 `decision-pack` evidence packet；后续 data-quality、source、financial、regulatory、competition、platform 和 risk agent 必须使用 `--no-collect` 读取同一批数据。

决策工作流统一入口：

```bash
cargo run --release -- decision-pack --workflow daily-monitor
cargo run --release -- decision-pack --workflow weekly-review
cargo run --release -- decision-pack --workflow quarterly-earnings
cargo run --release -- decision-pack --workflow valuation-decision
```

`decision-pack` 是工具层输出，不直接替代 sub agent 判断。它会生成 collect scope、subagent dispatch、必读文档、硬性门槛、证据快照、历史 observation 和最终输出模板。只读已有数据库时加 `--no-collect`，给调度层消费时加 `--format json`。

自动执行层入口：

```bash
cargo run --release -- agent-run --workflow daily-monitor
cargo run --release -- agent-run --workflow weekly-review
cargo run --release -- agent-run --workflow quarterly-earnings
cargo run --release -- agent-run --workflow valuation-decision
```

`agent-run` 会调用 Rust `decision-pack` 生成同一批 evidence packet，再由 Rust 规则化总控执行 collector、data-quality、source、financial、regulatory、competition、platform、risk/orchestrator 等合同，直接输出日监结论、周度复盘、季度财报判断和估值/仓位动作。

如果只需要 Rust 证据包，使用：

```bash
cargo run --release -- decision-pack --workflow daily-monitor
cargo run --release -- decision-pack --workflow weekly-review --format json
```

SEC 请求建议设置更明确的 User-Agent：

```bash
export CRCL_DATA_USER_AGENT="crcl-data-collector/0.1 your-email@example.com"
```

## 已自动化的数据

| 层级 | 数据 | 来源 | 落库位置 |
| --- | --- | --- | --- |
| P0 | USDC in circulation、Total reserves、储备构成、7D/30D/365D Issued / Redeemed / Net issuance、7D/30D change | Circle Transparency 页面内官方 data-point 字段 | `observations` |
| P0/P1 | USDC circulating supply、1D/7D/30D change、USDC share、USDC/USDT ratio、USDC by chain | DefiLlama stablecoins API | `observations` |
| P1 | Aave V3 / Compound V3 USDC-equivalent deposits，含协议总额和链级拆分 | DefiLlama protocol `tokensInUsd` breakdown | `observations` |
| P0/P1 | USDC/USDT/BTC/ETH 价格、PYUSD/FDUSD/USDe/USDY 市值 | CoinGecko simple price API | `observations` |
| P0 | Curve 3pool USDC ratio、USDC balance、pool liquidity | Curve getPools API | `observations` |
| P0 | 3M / 1Y / 5Y Treasury yield | U.S. Treasury XML | `observations` |
| P0 | SOFR | NY Fed API | `observations` |
| P0/P2 | Circle Reserve Fund latest SEC N-MFP3 7-day net yield、gross yield、WAM、WAL、net assets | SEC EDGAR BlackRock Funds N-MFP3 rendered filing | `filings` + `observations` |
| P0/P2 | Circle / Coinbase 最新 10-Q、10-K、8-K、Form 4、13F-HR 元数据 | SEC EDGAR submissions API | `filings` + `observations` |
| P1/P2 | Circle 10-Q/10-K reserve income、distribution costs、RLDC、Adjusted EBITDA、USDC 期末/平均流通量、onchain volume、filing-period velocity；Coinbase stablecoin revenue、receivables、client custodial funds、customer USDC on platform | SEC EDGAR inline XBRL + filing 正文表格 | `observations` |
| P1 | USDC active addresses、transaction count、transfer count | CoinMetrics Community API | `observations` |
| P1 | USDC adjusted transfer volume（最近完整 UTC 日，附 30D 合计属性） | Visa Onchain Analytics / Allium JSON API | `observations` |
| P1 | Exchange USDC balances（collector 已接入，需 key 才会落库） | CoinGlass Exchange Balance List v4 API；需 `COINGLASS_API_KEY` | `observations` / `missing_items` |
| P1 | CPN annualized TPV | Circle Q1 2026 pressroom / earnings disclosure | `observations` |
| P1 | Arc public network / mainnet status、testnet usage | Arc 官网；Circle Q4/FY2025 pressroom 兜底 | `observations` |
| P1 | Tokenized U.S. Treasury debt AUM、BlackRock BUIDL AUM | RWA.xyz public treasuries page `__NEXT_DATA__` | `observations` |
| P0 | Base / Solana / Circle status | Statuspage JSON | `observations` |
| P0 | Ethereum chain status | Public Ethereum JSON-RPC latest block freshness check | `observations` |
| P0 | Circle pressroom 近期公告 | Circle Pressroom 页面 | `events` |
| P0 | 监管与公司核心页面可达性检查 | OCC / Treasury / FinCEN / OFAC / Fed / SEC / CFTC RSS / FDIC / GovInfo GENIUS Act / Circle IR | `source_runs` + `events` |
| P0 | CRCL price / volume | Yahoo Finance chart endpoint | `observations`；若 429 会写 `source_runs` 错误 |
| P2 | CRCL short interest、days to cover、short-interest change | FINRA consolidated short interest Query API | `observations` |
| P2 | CRCL institutional ownership 聚合指标 | MarketBeat public institutional ownership page | `observations` |

## 当前缺口

自动源失败或需要人工补证时，程序会在 `missing_items` 表中记录缺口。当前不再阻塞
`agent-context --profile source` 的人工增强项：

| 层级 | 数据 | 原因 | 后续处理 |
| --- | --- | --- | --- |
| P1 | Dune 仪表盘截图、周度 adjusted velocity 交叉验证 | 日度 adjusted transfer volume 已用 Visa / Allium 自动落库；Dune 仍适合周报截图和口径核对 | 增加 `DUNE_API_KEY` 配置和固定 query id |
| P1 | CoinMetrics adjusted transfer value 交叉验证 | Community API 对 `TxTfrValAdjUSD` 返回凭证限制；主自动源改用 Visa / Allium | 如需严谨双源，接入 CoinMetrics Pro |
| P1 | Exchange USDC balances 凭证 | 已改用 CoinGlass Exchange Balance List；无 key 时接口返回 `API key missing` | 配置 `COINGLASS_API_KEY`；Nansen / TokenTerminal / Glassnode API 可作为付费确认源 |

## 数据库表

| 表 | 用途 |
| --- | --- |
| `collection_batches` | 每轮 `collect` / `agent-context` 的 batch id、profile、selector、开始结束时间和成功/警告数量 |
| `source_runs` | 每次访问 URL 的状态、HTTP code、错误和原始摘录 |
| `observations` | 标准化指标事实，带 `metric_code`、P0/P1/P2、数值、单位、来源 |
| `filings` | SEC filing 元数据 |
| `events` | 公司公告、监管页面检查等事件入口 |
| `missing_items` | 暂时无法自动获取的数据和原因 |

## 测试

当前测试覆盖解析逻辑：

```bash
cargo test --release
```

重点覆盖：

- DefiLlama USDC supply / market share / chain distribution 解析
- RWA.xyz tokenized Treasury AUM / BUIDL AUM 解析
- Circle Transparency 官方 data-point 字段、储备构成和 Issued / Redeemed 解析
- Curve 3pool USDC ratio 和余额解析
- Circle / Coinbase SEC inline XBRL 财务指标、Coinbase AOP 表格 USDC 行解析
- BlackRock Funds N-MFP3 Circle Reserve Fund 7-day yield、WAM/WAL 解析
- FINRA consolidated short interest 最新 CRCL 行解析
- Treasury XML 最新收益率行解析
- NY Fed SOFR JSON 解析
- Visa Onchain Analytics / Allium USDC adjusted transfer volume 解析
- CoinGlass USDC exchange balance list 解析和无 key 错误提示
- Circle pressroom CPN TPV、Arc mainnet / testnet metrics 解析
- MarketBeat CRCL institutional ownership 聚合页解析
