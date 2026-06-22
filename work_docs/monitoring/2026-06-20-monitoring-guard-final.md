# CRCL Monitoring Guard Final

生成时间：2026-06-20

## 结论

警戒。

理由：本轮数据采集和官方源可达性健康，未出现 P0 missing_info、SOURCE_BLOCKED、SOURCE_UNREACHABLE、USDC/USD 折价、储备覆盖异常或 Circle 状态异常；但 Base 核心链状态为 `Partially Degraded Service`，按监控纪律不能判为正常。当前异常尚未阻断 P0 判断，因此不升级为紧急。

## Evidence Packet

- workflow：monitoring
- batch_id：`decision-pack-1781936280037602000-70210`
- evidence：`work_docs/monitoring/decision-pack-monitoring-2026-06-20.json`
- generated_at：`2026-06-20T06:19:23Z`
- 只读核验命令：`cargo run --release -- --database data/crcl_research.sqlite agent-context --profile monitor --format json --no-collect`

## data-quality 是否阻断

不阻断。

data-quality 结论为降置信度进入判断，不阻断。降置信度项主要来自低频或滞后指标，而不是源失败或 P0 缺口。

## collector_run

- collector_run：`ok_sources=23`，`warn_sources=0`
- source_runs：本批 SQLite 核验为 `39 ok / 0 non-ok`
- missing_items：`[]`

## 失败源

无。

本批官方与关键源均为 `ok`，包括 SEC、Circle、OCC、Treasury、FinCEN、OFAC、FDIC、CFTC、Congress/GovInfo、Circle status、Base、Solana、Ethereum。

关键 source_url：

- Base status：`https://status.base.org/api/v2/status.json`
- Circle status：`https://status.circle.com/api/v2/status.json`
- Solana status：`https://status.solana.com/api/v2/status.json`
- Ethereum public JSON-RPC：`https://ethereum.publicnode.com`
- Circle Transparency：`https://www.circle.com/transparency`
- SEC EDGAR Circle：`https://data.sec.gov/submissions/CIK0001876042.json`
- SEC EDGAR BlackRock N-MFP3：`https://data.sec.gov/submissions/CIK0000844779.json`
- Treasury：`https://home.treasury.gov/news`
- FinCEN：`https://www.fincen.gov/news-room`
- OFAC：`https://ofac.treasury.gov/recent-actions`
- FDIC：`https://www.fdic.gov/news/press-releases/`
- CFTC：`https://www.cftc.gov/rss.xml`
- Congress/GovInfo GENIUS Act：`https://www.govinfo.gov/bulkdata/BILLSTATUS/119/s/BILLSTATUS-119s1582.xml`

## 过期指标

- Circle Transparency 三项 P0 官方口径：observed_at=`2026-06-15`，source_url=`https://www.circle.com/transparency`
- `P0_CIRCLE_RESERVE_FUND_7D_YIELD`：`3.57%`，observed_at=`2026-05-29`，source_url=`https://www.sec.gov/Archives/edgar/data/844779/000119312526258597/xslN-MFP3_X01/primary_doc.xml`
- `P0_CRCL_PRICE`：`80.2300033569336 USD`，observed_at=`2026-06-18`，source_url=`https://query1.finance.yahoo.com/v8/finance/chart/CRCL?range=5d&interval=1d`
- `P0_CRCL_VOLUME`：`14,863,000 shares`，observed_at=`2026-06-18`，source_url=`https://query1.finance.yahoo.com/v8/finance/chart/CRCL?range=5d&interval=1d`

## P0 / 紧急事件

- Base：`Partially Degraded Service`，observed_at=`2026-06-19T01:24:49.530Z`，source_url=`https://status.base.org/api/v2/status.json`。这是本轮警戒主因。
- Circle status：`All Systems Operational`，observed_at=`2026-06-20T06:04:13.612Z`，source_url=`https://status.circle.com/api/v2/status.json`
- Solana：`All Systems Operational`，observed_at=`2026-06-20T06:04:16.139Z`，source_url=`https://status.solana.com/api/v2/status.json`
- Ethereum：`Operational`，observed_at=`2026-06-20T06:19:23.906848+00:00`，source_url=`https://ethereum.publicnode.com`
- USDC/USD：`0.999855`，observed_at=`2026-06-20`，source_url=`https://api.coingecko.com/api/v3/simple/price?ids=usd-coin,tether,bitcoin,ethereum,paypal-usd,first-digital-usd,ethena-usde,ondo-us-dollar-yield&vs_currencies=usd&include_market_cap=true&include_24hr_vol=true&include_24hr_change=true`
- Circle total reserves：`75.10B USD`；USDC circulation：`74.86B USD`，observed_at=`2026-06-15`，source_url=`https://www.circle.com/transparency`
- Circle 7D / 30D net issuance：`-1.05B / -1.96B USD`，observed_at=`2026-06-15`，source_url=`https://www.circle.com/transparency`
- DefiLlama current USDC supply：`74.9389B USD`，observed_at=`2026-06-20`，source_url=`https://stablecoins.llama.fi/stablecoins?includePrices=true`
- Curve 3pool USDC ratio：`16.7744%`，observed_at=`2026-06-20`，source_url=`https://api.curve.fi/api/getPools/ethereum/main`

未发现需要升级为紧急的 P0 事件：无 P0 missing_info、无官方源阻断、无 USDC/USD 明显折价、无储备覆盖异常、无 Circle 状态异常。

## missing_info

`[]`

## 补采 / 人工核验动作

当前不得在本轮 guard 中刷新外部数据。建议下轮或人工核验执行：

- Base 状态复核：`cargo run --release -- collect --source status`
- 同批 evidence 再读：`cargo run --release -- --database data/crcl_research.sqlite agent-context --profile monitor --format json --no-collect`
- 如需完整 monitoring evidence packet 刷新，下一轮调度执行：`cargo run --release -- decision-pack --workflow monitoring --format json`
- 人工打开 Base 状态页确认 incident 是否仍在持续：`https://status.base.org/`
- 人工交叉检查 Circle Transparency 页面是否已发布 2026-06-20 或更新日期的官方储备/流通量口径：`https://www.circle.com/transparency`
- 人工交叉检查 Yahoo/交易所行情，确认 CRCL 2026-06-19 之后价格与成交量是否因非交易日或源延迟缺失。

## 解除条件

- Base status 恢复为 `Operational` 或 `All Systems Operational`，且下一轮 `source_runs` 仍为全 ok。
- `missing_items=[]`，无 P0 missing_info。
- Circle status、Solana、Ethereum 继续正常。
- USDC/USD 未出现持续折价，储备覆盖继续满足 total reserves >= circulation。
- Circle Transparency / Reserve Fund / CRCL 行情的低频或滞后项在下一可用数据日更新，或被人工确认属于官方发布频率/非交易日造成的正常滞后。

## 下次自动检查触发

- cron 下一轮 monitoring 自动触发。
- 提前触发条件：Base 状态仍为 degraded、Circle status 异常、任一官方/关键源出现 non-ok、出现 P0 missing_info、USDC/USD 折价、储备覆盖异常、Circle Transparency 更新延迟扩大。

## 已保存路径

`work_docs/monitoring/2026-06-20-monitoring-guard-final.md`
