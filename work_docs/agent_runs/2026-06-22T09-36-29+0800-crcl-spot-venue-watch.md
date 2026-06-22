结论：**观察（短样本）**。CRCLB 场内数据链路正常，完成日线 11 天也必须分析；本轮可分析 status、daily close、实际窗口成交额、盘口 spread/depth，但不能外推为 30D/90D 长线趋势。USDCUSDT 场内 peg/盘口信号正常，未触发交叉验证阈值。

data-quality：**不阻断**。Binance Spot 本地采集链路正常：本次按 `agent-context --profile spot-venue --no-collect --format text` 读取，`collector_run=skipped`，最近 `exchangeInfo / ticker / depth / klines` 均为 `ok`、HTTP 200，Binance 相关 `missing_items=0`。限制点是 CRCLB 完成日线 `<30`，因此只能做短样本场内分析，不能写完整 30D/90D 长线判断；这不是采集失败。

关键证据：

CRCLB：
- 数据日期：日线最新完成 K 线 `2026-06-21`；盘口/24h ticker `2026-06-22`；采集时间约 `2026-06-22T01:34:58Z`。
- 状态：`TRADING`，来源 `https://api.binance.com/api/v3/exchangeInfo?symbol=CRCLBUSDT`。
- daily close：`80.81 USDT`；24h last price `81.08 USDT`；24h change `+0.521%`。
- history_days：`11` 个完成日线，范围 `2026-06-11` 至 `2026-06-21`，来源 `https://api.binance.com/api/v3/klines?symbol=CRCLBUSDT&interval=1d&limit=120`。
- 近窗口 quote volume：`9,537,188.64 USDT`，窗口实际只有 `11` 天，折算日均约 `867,017 USDT/day`；最新完成日 quote volume `518,447.73 USDT`。
- 盘口：spread `7.4010 bps`，bid depth `323,711.91 USDT`，ask depth `191,970.69 USDT`，来源 `https://api.binance.com/api/v3/depth?symbol=CRCLBUSDT&limit=100`。
- 30D/90D price change：不可用；完成日线 `<30`，不得写 30D/90D 趋势，但短样本的价格、成交额和盘口分析仍然有效。

USDCUSDT：
- 数据日期：日线最新完成 K 线 `2026-06-21`；盘口/24h ticker `2026-06-22`；采集时间约 `2026-06-22T01:34:59Z`。
- 状态：`TRADING`，来源 `https://api.binance.com/api/v3/exchangeInfo?symbol=USDCUSDT`。
- daily close：`1.00096 USDT`，相对 1.0 偏离约 `+9.6 bps`，低于 `30 bps` 观察阈值。
- spread：`0.0999 bps`，低于 `5 bps` 交叉验证阈值；bid depth `70,442,572.08 USDT`，ask depth `27,102,089.29 USDT`。
- history_days：`119` 个完成日线；30D price change `+0.0120%`，90D price change `+0.0660%`。
- 30D quote volume：`47,316,392,079.36 USDT`；最新完成日 quote volume `507,278,491.82 USDT`；24h quote volume `590,441,450.51 USDT`。
- 来源：`https://api.binance.com/api/v3/klines?symbol=USDCUSDT&interval=1d&limit=120`、`https://api.binance.com/api/v3/depth?symbol=USDCUSDT&limit=100`、`https://api.binance.com/api/v3/ticker/24hr?symbol=USDCUSDT`。

confirmed_fact：
- 本次只读本地 `data/crcl_research.sqlite`，没有刷新外部数据。
- CRCLBUSDT 是 Binance Spot 的 tokenized bStock/现货交易对，不能替代 `NYSE:CRCL`。
- USDCUSDT 只代表 Binance 场内交易对，不能替代全市场 USDC peg、Circle 官方透明度数据或全交易所 USDC balance。
- Binance Spot 最近一批 CRCLBUSDT / USDCUSDT 四类端点均成功落库，Binance 相关缺口为 0。

inference：
- 总体结论落在“观察（短样本）”：CRCLB 只有 11 天完成日线，但仍可分析当下 Binance 场内交易质量。
- CRCLB 近 11 天日均 quote volume 高于 `25万 USDT/day` 的偏薄阈值，只能说明当前 Binance 场内有一定交易活跃度，不能外推 30D 趋势。
- USDCUSDT 的 close 偏离和 spread 均低于阈值，本轮没有从 Binance 场内看到需要立即交叉验证的 peg/盘口异常。

missing_info：
- CRCLB 长线窗口不足：缺至少 30 个完成日线，30D price change 和 30D 可靠趋势不可用；缺至少 90 个完成日线，90D 长线辅助趋势不可用。
- 本次未纳入 `NYSE:CRCL` 成交量、价格、筹码或基本面共振验证。
- USDCUSDT 未替代 CoinGecko USDC/USD、Circle Transparency、Curve 池或交易所 USDC balance 的全市场交叉验证。
- 盘口是单次 snapshot，不是 order book 时间序列。

反证 / 解除条件：
- 解除 CRCLB 长线窗口不足：CRCLB 完成日线达到 `>=30` 后再看 30D；达到 `>=90` 后才允许写 90D 长线辅助趋势。
- 触发交叉验证：USDCUSDT daily close 偏离 1.0 超 `30 bps`，或 spread 超 `5 bps`；届时必须查 CoinGecko / Circle / Curve / 交易所余额。
- 触发 CRCLB 场内观察：CRCLB 30D close change 绝对值超 `20%`，或近窗口日均 quote volume 跌破 `25万 USDT/day`；仍需 NYSE:CRCL、筹码或基本面共振。
- 反证当前“链路正常”：后续 `source_runs` 出现 Binance 端点失败、HTTP 非 200，或 `missing_items` 出现 Binance Spot 指标缺口。

下一步：
- 继续按父任务统一 evidence packet 读取，不单独刷新；等下一次父任务采集后复核 CRCLB history_days。
- CRCLB 满 30 天前，周报/估值仍要写“观察（短样本）”：分析 status、daily close、实际窗口日均成交额和 spread/depth，但不写 30D/90D 趋势。
- 若 USDCUSDT 超阈值，再拉 CoinGecko USDC/USD、Circle Transparency、Curve 和交易所余额做交叉验证。
- 本报告不是个性化投资建议；本报告已保存到 `work_docs/agent_runs/2026-06-22T09-36-29+0800-crcl-spot-venue-watch.md`。
