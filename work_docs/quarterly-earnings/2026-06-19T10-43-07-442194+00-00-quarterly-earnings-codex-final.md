结论：**观察；Q1 2026 财报为中性，偏基础盘留存强化**。不降级 CRCL 研究框架，但也不触发增强 / Bull / 平台基础设施股切换。本轮已按项目规则派发 `crcl-data-collector -> data-quality/source -> financial/platform/competition -> risk-decision`，后续 agent 均只读同一 evidence batch：`decision-pack-1781865890212451000-47845`。

data-quality：**不阻断，降置信度进入判断**。SEC / Circle / 利率 / 监管官方源可用；唯一采集告警是 CoinGlass 交易所 USDC balance 前端 payload 解密失败，影响 P1 交易所余额辅助判断，不影响 SEC 财报核心复核。

关键证据：
- SEC 10-Q：Q1 2026，数据日 `2026-03-31`，filing `2026-05-11`，来源 `https://www.sec.gov/Archives/edgar/data/1876042/000187604226000150/crcl-20260331.htm`
- Average USDC：`752 亿美元`，低于财报前 `760-800 亿美元`基准。
- Reserve income：`6.52508 亿美元`；Reserve return rate 约 `3.47%`，低于 `3.6-3.8%`基准但未跌破 `3.4%`降级线。
- RLDC：`2.87352 亿美元`；主口径 `RLDC / Reserve income = 44.04%`，高于 `38%`降级阈值；collector 存储的 `41.40%`是全口径辅助值。
- Other revenue：`4162.5 万美元`，share `5.997%`，年化约 `1.665 亿美元`，接近 FY2026 `1.5-1.7 亿美元`指引节奏，但仍只是 `5-10%`弱平台化。
- CPN annualized TPV：`83 亿美元/年`，来源 Circle Q1 release `https://www.circle.com/pressroom/circle-reports-first-quarter-2026-results`；缺收费模式和收入贡献。
- Arc：`public testnet`，来源 `https://www.arc.io/`，测试网 activity 不能替代主网收入。
- competition score：`67.5`，上周 `57.5`，未过 `75`增强阈值；USDC 7D net issuance `-10.5 亿美元`、30D `-19.6 亿美元`需观察。
- Reserve Fund：7D yield `3.57%`，WAM/WAL `12/12 天`，net assets `651.67 亿美元`，来源 BlackRock N-MFP3 `https://www.sec.gov/Archives/edgar/data/844779/000119312526258597/xslN-MFP3_X01/primary_doc.xml`

missing_info：
- `P1_EXCHANGE_USDC_BALANCES`、`P1_EXCHANGE_USDC_BALANCE_30D_CHANGE`、`P1_EXCHANGE_USDC_TOP3_CONCENTRATION` 缺失；CoinGlass HTTP 200 但 `unsupported CoinGlass frontend encryption v=1`。
- 竞争矩阵 D1/D2/D5 缺 4 周趋势口径。
- Other revenue 未拆 CPN / Arc / enterprise API / integration services。
- CPN fee model、take rate、收入确认方式未披露。
- Arc 主网时间、费用模型、真实客户和可重复收入未披露。
- GENIUS Act / OCC / Treasury 等规则对收益分享和第三方激励的细项影响仍需后续监管矩阵跟踪。

反证 / 解除条件：
- 若 RLDC 主口径跌破 `38%`，触发财报降级检查。
- 若 USDC 7D/30D net issuance 连续转负并扩大，尤其接近或超过 C-TRIGGER-C，需要优先进入信用层协议。
- 若 Other revenue share 连续两个季度超过 `10%`，且 CPN/Arc 披露可重复收入，可从弱平台化升到中验证。
- 若 CPN TPV/Arc activity 增长但 Other revenue 不增长，平台化维持未验证或下调。
- 观察解除需要 USDC 净发行转正或不再连续负值、CoinGlass/替代源补齐交易所余额、RLDC 继续高于 `40%`，并看到 CPN/Arc 收入口径改善。

下一步动作 / 下次复盘触发：
- 修复 CoinGlass v=1 解密或接入 Open API/替代源后，重跑 market 采集。
- Circle Transparency 下一次更新后，复核 USDC 7D/30D 净发行是否继续为负。
- 下一份 10-Q / earnings call 重点追问 Other revenue 拆分、CPN 收费模式、Arc 主网/费用模型、分销协议变化。
- competition score 上穿 `75`或跌破 `40`时，重新跑竞争和风险矩阵。
- 出现 USDC/USD 折价、储备异常、核心银行通道异常或正式执法行动时，立即转信用层复核。

已保存 / 将由 cron 保存的路径：
- `work_docs/quarterly-earnings/2026-06-19T10-43-07-442194+00-00-quarterly-earnings-codex-final.md`
- `work_docs/quarterly-earnings/2026-06-19T10-43-07-442194+00-00-quarterly-earnings-evidence.json`

以上为项目研究框架复核结论，不是个性化投资建议。
