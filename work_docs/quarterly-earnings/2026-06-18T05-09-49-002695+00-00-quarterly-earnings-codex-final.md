结论：**观察 / Base**。Q1 2026 财报对 CRCL 是“中性偏稳”，不增强、不降级。RLDC 主口径很强，但 Average USDC 和储备收益率低于预期锚；Other revenue 只到弱平台化验证，CPN / Arc 还不能支撑平台基础设施重定价。本结论不是个性化投资建议。

data-quality：**不阻断**。本轮统一 evidence packet 为 `batch_id=decision-pack-1781759473339815000-80707`，collector `ok_sources=21`、`warn_sources=0`。SQLite 中仍有 `SOURCE_UNREACHABLE_FINCEN_NEWS_ROOM` 的机器采集记录，但已在本轮人工核验 FinCEN 官方新闻页可达；页面显示 2026-06-05、2026-05-11、2026-04-08 等官方新闻/公告，其中 2026-04-08 包含 GENIUS Act / PPSI AML-CFT 相关材料。因此解除 data-quality active 阻断，但监管结论不能写成“无缺口”。

**关键证据**
- SEC 10-Q：Q1 2026，period `2026-03-31`，filing `2026-05-11`，来源：`https://www.sec.gov/Archives/edgar/data/1876042/000187604226000150/crcl-20260331.htm`。
- Average USDC：`75.2B USD`，低于财报前 `76-80B` 预期锚。
- Reserve income：`652.508M USD`；按 `Reserve income × 4 / Average USDC`，reserve return rate 约 `3.47%`。
- Total revenue and reserve income：`694.133M USD`；Distribution and transaction costs：`405.402M USD`；total distribution / transaction / other costs：`406.781M USD`。
- RLDC：`287.352M USD`；主口径 `RLDC / Reserve income = 44.04%`，高于 `38%` 降级线；全口径 `RLDC / total revenue and reserve income = 41.40%` 只作辅助。
- Other revenue：`41.625M USD`；Other revenue share：`5.997%`，处于 `5-10%` 弱平台化区间。
- Adjusted EBITDA：`151.401M USD`；`Adjusted EBITDA / RLDC = 52.69%`。
- Circle Q1 press release：`https://www.circle.com/pressroom/circle-reports-first-quarter-2026-results`；CPN annualized TPV 为 `8.3B USD/year`。
- Arc：`public testnet`，`2026-06-18`；测试网活动不能作为收入或主网收入证据。
- Competition score：`67.5`，上周 `48.3`，未穿越 `75` 增强阈值，也未跌破 `40` 降级阈值。
- BlackRock / Circle Reserve Fund N-MFP3：7-day yield `3.57%`，yield date `2026-05-29`，来源：`https://www.sec.gov/Archives/edgar/data/844779/000119312526258597/xslN-MFP3_X01/primary_doc.xml`。

**矩阵复核**
财务桥：中性 / 观察。RLDC margin 强，守住基础盘；但 Average USDC 低于预期锚，reserve return rate 环比下行，不能增强。

平台化：弱验证。Other revenue share 约 `6%`，CPN 有 TPV，但缺收费模式、收入确认、客户数和收入拆分；Arc 仍是 public testnet，不满足平台化切换。

竞争：中性偏观察。D3 分销渠道议价因 RLDC 主口径强而上修，但 D1/D2/D5 缺 4 周趋势，D4 收益型稳定币蚕食仍压制评分。

信用层：未触发覆盖性降级。当前未见 USDC/USD 折价、储备异常、核心银行通道中断或 Circle 正式执法行动；但仍需持续监控 C-TRIGGER。

**missing_info**
CPN take rate、收费模式、收入确认、客户数；Other revenue 中 CPN / Arc / API / integration services 拆分；Arc 主网、费用模型、企业客户和真实收入；D1/D2/D5 的 4 周固定趋势；FinCEN 机器采集记录仍需下批恢复为 `ok`；监管 final rule 对 third-party incentive / yield sharing 的经济影响仍需逐条映射。

**反证 / 解除条件**
增强条件：Average USDC 回到或高于 `76-80B` 预期锚，RLDC 主口径继续 `>40%`，competition score `>75` 且 D1/D2/D5 补齐后不削弱，Other revenue share 进入 `10-15%` 并披露 CPN / Arc 可重复收入。

降级条件：RLDC 主口径 `<38%`；reserve return rate 环比下行超过 `50 bps` 且 USDC 规模无法抵消；Other revenue share 回落 `<5%` 且连续两个季度低增长；competition score `<40`；或 USDC 净赎回 / 市占率下降触发 C-TRIGGER。

**下一步**
下次复盘触发：Q2 2026 10-Q / earnings release；每周 USDC net mint/redeem、市占率和 competition score 穿越 `75/40`；FinCEN / Treasury / OCC / SEC 出现正式规则或执法；CPN 披露收费模型或收入贡献；Arc 从 public testnet 进入主网并披露费用/客户/收入。

**已保存路径**
- `work_docs/quarterly-earnings/2026-06-18T05-09-49-002695+00-00-quarterly-earnings-codex-final.md`

本轮已按项目规则使用 subagents：`crcl-data-collector` → `crcl-data-quality-auditor` / `crcl-source-verifier` → `crcl-financial-valuation` / `crcl-platform-option` / `crcl-competition-score` → `crcl-risk-decision`。没有调用本地 `agent-run --save` 生成额外报告，以免在 subagent 统一 evidence packet 后再次刷新造成口径漂移。