# CRCL 估值仓位复核

运行时间：2026-06-19
workflow：valuation-decision
同批证据包：`decision-pack-1781864897917234000-25626`
SQLite：`data/crcl_research.sqlite`
输出目录：`work_docs/valuation-decision`

## 结论

结论：观察。

Bull/Base/Bear：Base 偏弱，落在 Base 下沿；不切 Bull，不进入 Bear。

data-quality：降置信度进入判断，不完全阻断本轮估值复核，但阻断“增强 / keep / 上调仓位”。主要阻断来自 `SOURCE_UNREACHABLE_TREASURY_NEWS`；CoinGlass exchange balance 三项 P1 缺口降低交易 beta 和集中度判断置信度。

仓位上限：Base 下沿 `50-60%`。未设置 `CRCL_CURRENT_POSITION_PCT`，本轮只输出仓位上限，不判断具体加减仓量；不输出个性化投资建议。

交易动作触发：只有在 Treasury P0 source gap 解除、CoinGlass P1 gaps 修复、USDC 30D 转正或至少净赎回停止、competition score 回到 `>=55`，且财务桥维持 RLDC 主口径 `>40%` 后，才允许回到标准 Base 上限 `60-80%` 的讨论。若 competition score `<40`、RLDC margin `<38%`、USDC 30D 继续恶化并触发连续净赎回条件，则转入 Bear / 降级复核。

## 关键证据

1. 采集批次：`decision-pack-1781864897917234000-25626`，采集范围 `sec,rates,market,events,status`，`ok_sources=22`、`warn_sources=1`；source_runs 明细 `ok=36`、`decode_error=1`、`network_error=1`。
2. 信用层：USDC/USD `0.999777`（CoinGecko，2026-06-19），未达到 `>0.1%` 折价触发；Circle Transparency 显示 USDC 流通 `74.86B`、总储备 `75.10B`（2026-06-15）；Circle service 为 `All Systems Operational`。
3. USDC 动能：DefiLlama supply `74.8938B`（2026-06-19），7D / 30D 为 `-0.0229% / -1.9426%`；Circle official supply `74.86B`（2026-06-15），7D / 30D 为 `-1.3832% / -2.5514%`，因此进入观察而不是增强。
4. 财务桥：Q1 2026 Reserve income `652.508M`、RLDC `287.352M`、Other revenue `41.625M`、Adjusted EBITDA `151.401M`、diluted shares `266.687M`，period ended 2026-03-31，来源 SEC EDGAR Circle 10-Q。
5. RLDC margin：主口径 `RLDC / Reserve income = 44.0381%`，未触发 `38%` 降级告警；全口径 `RLDC / Total revenue and reserve income = 41.3973%`，只作辅助，不套主告警阈值。
6. 估值锚：CRCL price `80.23`（Yahoo Finance，2026-06-18），隐含市值约 `21.396B`；年化 RLDC 约 `1.149B`；Market cap / annualized RLDC 约 `18.62x`，位于 Base 区间 `15-22x`。
7. 平台化：Other revenue share `5.9967%`，达到 `5%` 弱验证门槛，未达到 `10%` 中验证和 `15%` 强验证门槛；CPN annualized TPV `8.3B`（2026-03-31）缺收费模式和收入拆分；Arc 为 public testnet（2026-06-19），无主网收入和客户收入贡献。
8. 竞争：competition score `53.1`，中性偏弱 / 预警档；不支持 Bull 条件 `>75`，也未跌破 `<40` 的 Bear 竞争恶化阈值。
9. 监管：本批未确认监管 P0 业务风险；GENIUS Act latest action 为 2025-07-18 Became Public Law No. 119-27。但 Treasury news 当前为 `network_error`，不能声称 Treasury 完整无事件。

## 矩阵复核

财务桥：中性，Base 偏弱。RLDC 主口径 44.0381% 托住基本盘，但 average USDC 75.2B、当前 USDC 7D/30D 回落、reserve return rate 压力和 data-quality 缺口不支持上调估值或仓位上限。

监管：P1 观察，降置信度。本批未确认第三方激励、Coinbase 分成、钱包/交易所奖励或 yield workaround 被最终规则限制；Treasury news P0 source gap 未解除，阻断增强 / keep / 上调仓位。

竞争：中性偏弱 / 预警档。D3 分销渠道议价可评分，D4 被净赎回压低，D1/D2/D5/D6 因趋势或来源缺口 missing；score 53.1 不支持 Bull，也不触发 Bear。

平台化：弱验证。Other revenue 达到早期期权门槛，但 CPN/Arc 仍未披露可重复收入、收费模型和收入归属，不满足支付基础设施切换条件。

信用层检查：未触发 C-TRIGGER。USDC/USD 未出现 >0.1% 折价，储备规模高于流通规模，Circle 官方状态正常，未看到正式执法行动覆盖信号。观察项为 USDC 30D 回落、Base status `Partially Degraded Service`、Treasury news P0 source gap。

## missing_info

1. `SOURCE_UNREACHABLE_TREASURY_NEWS`，P0，source_hint `https://home.treasury.gov/news`，本批 collector 为 `network_error`。
2. `P1_EXCHANGE_USDC_BALANCES`，CoinGlass decode_error，不能写成当前事实。
3. `P1_EXCHANGE_USDC_BALANCE_30D_CHANGE`，CoinGlass decode_error。
4. `P1_EXCHANGE_USDC_TOP3_CONCENTRATION`，CoinGlass decode_error。
5. 缺 Treasury 当日新闻标题与正文核验，不能确认 Treasury 侧无新增监管事件。
6. 缺 Other revenue 子项拆分：CPN、Arc、企业 API、integration services。
7. 缺 CPN fee model、take rate、收入确认方式、Arc 主网费用模型、主网客户和收入归属。
8. 缺 CoinGlass 或替代源对交易所 USDC balances、30D change、top3 concentration 的当前值。

## 反证 / 解除条件

升级解除条件：Treasury news 最新 source_run 恢复 `ok`，且监管矩阵确认无 P0 / 正式执法行动；CoinGlass 或替代源恢复交易所 USDC balances、30D change、top3 concentration；USDC 30D change 转正或至少净赎回停止；competition score 回到 `>=55`；Q2 或后续财报确认 RLDC 主口径继续 `>40%`，Other revenue 占比升至 `>10%`，且 CPN/Arc 披露收费模式或收入拆分。

降级 / Bear 触发：RLDC 主口径跌破 `38%`；Reserve return rate 低于 `3.4%` 且 USDC 规模未补偿；USDC 连续 2-4 周净赎回；competition score 跌破 `<40`；监管最终规则或正式执法明确限制第三方激励、收益分享或 Coinbase / 钱包奖励资金路径；USDC/USD 折价 `>0.1%`、储备报告异常、核心银行或赎回通道中断时直接进入信用层协议。

## 下一步动作 / 下次复盘触发

1. Circle Transparency 下一次更新，重点看 7D / 30D net issuance、USDC supply、total reserves。
2. Treasury news / OCC / FinCEN / OFAC / SEC 任一监管源出现 P0 或正式执法相关更新。
3. CoinGlass P1 collector 修复，或替代源可用。
4. USDC 30D change 继续为负并接近或超过连续 2 周净赎回观察窗口。
5. Q2 2026 财报或 Circle 披露 CPN/Arc 收入拆分、Other revenue 指引变化。
6. competition score 穿越 `55`、`75` 或跌破 `40`。

## 已保存路径

本汇总文件：`work_docs/valuation-decision/2026-06-19T10-26-38-465104+00-00-valuation-decision-subagent-summary.md`

codex exec 最终回复目标路径：`work_docs/valuation-decision/2026-06-19T10-26-38-465104+00-00-valuation-decision-codex-final.md`
