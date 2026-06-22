# CRCL monitoring guard final

生成时间：2026-06-22T02:02:05.514152+00:00
evidence：`work_docs/monitoring/2026-06-22T01-57-38-073801+00-00-monitoring-evidence.json`
batch_id：`decision-pack-1782093610609665000-65713`

## 结论：紧急

本轮自动化 monitoring 健康状态为紧急，不输出投资动作。

触发原因：

- `SOURCE_UNREACHABLE_FINCEN_NEWS_ROOM` 是 P0 `missing_info`，且 FinCEN 属于官方监管源；data-quality-auditor 已判定 `先补采`，阻断完整 monitoring 判断。
- Base 状态页本批成功访问，但最新状态为 `Partially Degraded Service`，`observed_at=2026-06-19T01:24:49.530Z`，需要补 incident 详情才能解除核心链状态异常。
- Circle status、Solana status、Ethereum freshness 当前未见异常；USDC/USD 未见折价级别异常，但这些不足以覆盖 P0 官方源缺口。

## data-quality 是否阻断

阻断。阻断项为：

- `SOURCE_UNREACHABLE_FINCEN_NEWS_ROOM`
- priority：P0
- reason：`error sending request for url (https://www.fincen.gov/news-room)`
- source_hint：`https://www.fincen.gov/news-room`

## collector_run

- batch_id：`decision-pack-1782093610609665000-65713`
- generated_at：`2026-06-22T02:02:05.514152+00:00`
- collector_run：`ok_sources=24`，`warn_sources=0`
- 本批 `source_runs.status`：`ok=46`，`network_error=1`
- 紧急性判断：不是 collector 全局瘫痪，因 `ok_sources` 不为 0；但唯一失败源是 P0 官方监管源 FinCEN，且已进入 missing_info，所以升级紧急。

## 失败源

| source | status | fetched_at | source_url / hint | error |
| --- | --- | --- | --- | --- |
| FinCEN news room | `network_error` | `2026-06-22T02:01:46.534856+00:00` | `https://www.fincen.gov/news-room` | `error sending request for url (https://www.fincen.gov/news-room)` |

## 过期指标

- Circle Transparency 官方锚：`P0_CIRCLE_USDC_TOTAL_RESERVES=75.100B USD`、`P0_CIRCLE_USDC_7D_NET_ISSUANCE=-1.050B USD`，`observed_at=2026-06-15`，source_url：`https://www.circle.com/transparency`。当前性中等，需要下一轮官方刷新确认。
- Circle Reserve Fund 7-day net yield：`3.57%`，`observed_at=2026-05-29`，source_url：`https://www.sec.gov/Archives/edgar/data/844779/000119312526258597/xslN-MFP3_X01/primary_doc.xml`。这是 SEC N-MFP3 月度口径，非日度异常，但不应用作当前日度锚。
- CRCL Yahoo：`80.23 USD`、`14.863M shares`，`observed_at=2026-06-18`，source_url：`https://query1.finance.yahoo.com/v8/finance/chart/CRCL?range=5d&interval=1d`。当前性中等，不能单独支撑最新市场状态。

## P0 / 紧急事件

- P0 missing_info：`SOURCE_UNREACHABLE_FINCEN_NEWS_ROOM`，官方监管源不可达，阻断完整 monitoring。
- 核心链状态异常：Base `Partially Degraded Service`，`observed_at=2026-06-19T01:24:49.530Z`，fetched `2026-06-22T02:02:03.567428+00:00`，source_url：`https://status.base.org/api/v2/status.json`；需补 incident 详情和恢复时间。
- 未触发的 P0 锚定异常：USDC/USD CoinGecko `0.999813`，`observed_at=2026-06-22`，source_url：`https://api.coingecko.com/api/v3/simple/price?ids=usd-coin,tether,bitcoin,ethereum,paypal-usd,first-digital-usd,ethena-usde,ondo-us-dollar-yield&vs_currencies=usd&include_market_cap=true&include_24hr_vol=true&include_24hr_change=true`；Curve 3pool USDC ratio `14.713%`，`observed_at=2026-06-22`，source_url：`https://api.curve.fi/api/getPools/ethereum/main`。
- 其他状态源：Circle `All Systems Operational`，`observed_at=2026-06-22T00:24:33.178Z`，source_url：`https://status.circle.com/api/v2/status.json`；Solana `All Systems Operational`，`observed_at=2026-06-22T00:24:37.383Z`，source_url：`https://status.solana.com/api/v2/status.json`；Ethereum `Operational`，`observed_at=2026-06-22T02:02:05.508758+00:00`，source_url：`https://ethereum.publicnode.com`。

## missing_info

```json
{
  "collector": "FinCEN news room",
  "metric_code": "SOURCE_UNREACHABLE_FINCEN_NEWS_ROOM",
  "metric_name": "Source page unreachable: FinCEN news room",
  "priority": "P0",
  "reason": "error sending request for url (https://www.fincen.gov/news-room)",
  "source_hint": "https://www.fincen.gov/news-room"
}
```

## 补采 / 人工核验动作

本轮禁止刷新外部数据，因此不执行新的采集。

本轮只读复核命令，如需由主线程读取本地库确认上下文：

```bash
cargo run --release -- --database data/crcl_research.sqlite agent-context --profile monitor --format json --no-collect
```

解除阻断前的人工核验：

1. 人工打开 `https://www.fincen.gov/news-room`，确认是否为临时网络错误、站点阻断、证书/DNS 问题或页面迁移。
2. 人工打开 `https://status.base.org/api/v2/status.json` 及 Base incident 页面，记录 `Partially Degraded Service` 的 incident 标题、影响范围、开始/恢复时间。
3. 核对 Circle Transparency `https://www.circle.com/transparency` 是否已有 2026-06-22 或更新日期的官方数据；若仍停留 2026-06-15，保留当前性中等标记。

允许刷新后的补采命令：

```bash
cargo run --release -- decision-pack --workflow monitoring --format json
```

若只允许本地库复读，则继续使用：

```bash
cargo run --release -- --database data/crcl_research.sqlite agent-context --profile monitor --format json --no-collect
```

## 解除条件

- FinCEN news room 下一轮 `source_runs.status=ok`，且 `SOURCE_UNREACHABLE_FINCEN_NEWS_ROOM` 从 P0 `missing_info` 消失。
- Base status 回到 `Operational`，或已补齐 incident 详情并确认不影响 USDC 相关核心路径。
- data-quality-auditor 不再输出 `先补采` / 阻断完整 monitoring 判断。
- Circle Transparency 官方锚刷新到当前或能解释其低频日期；锚定监控继续保持 USDC/USD 无持续折价、Curve 池比例无信用挤兑信号。

## 下次自动检查触发

- 下一轮 monitoring 定时任务立即重查 FinCEN、Base status、Circle Transparency。
- 若 FinCEN 连续两轮仍 `SOURCE_UNREACHABLE` 或 Base 仍 `Partially Degraded Service` 且无 incident 详情，继续保持紧急。
- 若 FinCEN 恢复但 Base 未恢复，降级与否取决于 Base incident 详情是否影响 USDC 链上发行、赎回、转账或 Coinbase/Base 生态可用性。

## 已保存路径

`work_docs/monitoring/2026-06-22T02-02-05-monitoring-guard-final.md`
