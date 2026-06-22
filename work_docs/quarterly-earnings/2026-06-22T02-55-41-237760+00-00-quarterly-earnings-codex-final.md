**结论**
本轮季度财报复核结论：**观察**。更细拆是：Q1 2026 财报数字层**强化 Base 内财务桥**，但竞争与平台化没有共振，不触发整体“增强”，也不触发“降级”或 Bull / 支付基础设施股框架切换。

已按要求使用项目级 subagents：`crcl-data-collector -> crcl-data-quality-auditor / crcl-source-verifier -> crcl-financial-valuation / crcl-platform-option / crcl-competition-score -> crcl-risk-decision`。统一 evidence batch：`decision-pack-1782097042621279000-50434`，采集范围 `sec,rates,market,events`，`ok_sources=21 / warn_sources=1`。

**Data Quality**
data-quality：**不阻断，降置信度进入判断**。

阻断未触发：无 P0 missing；SEC / Circle / 监管官方源未失败；未见 `SOURCE_BLOCKED` 或 `SOURCE_UNREACHABLE`。

降置信度项：
- `RWA.xyz treasuries` HTTP 200 但 body 解码失败，缺 `P1_TOKENIZED_TREASURY_AUM`。
- Circle Transparency 官方 flows / reserves 数据日期停在 `2026-06-15`；DefiLlama `2026-06-22` 只能交叉验证，不能替代官方口径。

**关键证据**
- Circle Q1 2026 10-Q：report date `2026-03-31`，filing `2026-05-11`，SEC URL `https://www.sec.gov/Archives/edgar/data/1876042/000187604226000150/crcl-20260331.htm`
- Average USDC：`75.2B USD`；期末 USDC：`77.049B USD`，SEC 10-Q。
- Reserve income：`652.508m USD`；reserve return rate 年化约 `3.47%`。
- Distribution and transaction costs：`405.402m USD`；total distribution / transaction / other costs：`406.781m USD`。
- RLDC：`287.352m USD`。主口径 `RLDC / Reserve income = 44.04%`，高于 40% 强化线和 38% 降级线；DB 中 `41.397%` 是全口径 `RLDC / total revenue and reserve income`，只作辅助。
- Other revenue：`41.625m USD`；share `5.997%`，年化约 `166.5m USD`，落在 FY2026 `150-170m USD` 指引区间偏上，但仍只是 5-10% 弱平台化。
- CPN annualized TPV：`8.3B USD/year`，Circle Q1 release `https://www.circle.com/pressroom/circle-reports-first-quarter-2026-results`
- Arc：`public testnet`，weekly tx `22,708,343`，avg transaction cost `0.004 USD/tx`，来源 `https://www.arc.io/`；测试网不能替代收入。
- Competition score：`51.7`，中性偏弱；未跌破 `<40` Bear 阈值，但低于 `55` 预警线，暂停上调估值倍数。
- USDC peg / 信用层：USDCUSDT completed daily close `1.00096`（`2026-06-21`），spread `0.0999 bps`（`2026-06-22`），未触发 C-TRIGGER。

**Missing Info**
- `P1_TOKENIZED_TREASURY_AUM`：RWA.xyz body 解码失败，影响 RWA / 收益型稳定币竞争维度。
- D4 / D5 competition score 缺完整 action-grade 数据，不能把当前竞争分数当增强依据。
- Circle Transparency `2026-06-22` 当周官方 flows 未更新。
- Other revenue 未拆 CPN / Arc / enterprise API / integration services。
- CPN 缺收费模式、take rate、收入确认方式。
- Arc 缺主网时间、费用模型、真实客户和收入归属。
- Coinbase / Binance 等渠道分销协议变化、成本一次性因素、FY 指引上修/下修口径仍需财报电话会或后续 filing 复核。

**反证 / 解除条件**
- 若 `RLDC / Reserve income <38%`，触发财报降级检查。
- 若 competition score 跌破 `<40`，触发 Bear / 降级复核；回到 `>=55` 才解除竞争预警，`>75` 才允许讨论增强。
- 若 Other revenue share 连续进入 `10-15%`，且披露 CPN / Arc 可重复收入，可升级到中平台化验证。
- 若 CPN TPV / Arc activity 增长但 Other revenue 不增长，平台化期权应降权。
- 若 USDC/USD 折价、储备异常、核心银行通道异常或正式执法行动出现，信用层覆盖全部矩阵。

**下一步**
- 保持本轮结论为观察；估值动作是暂停上调倍数，不切换平台基础设施股框架。
- 修复或人工补核 RWA.xyz / `P1_TOKENIZED_TREASURY_AUM`。
- Circle Transparency 下一次官方更新后，复核 USDC 7D / 30D flows 是否继续净赎回。
- Q2 2026 10-Q / earnings release 后重算 Average USDC、reserve return rate、RLDC 主/辅口径、Other revenue share。
- 财报电话会优先追问 Other revenue 拆分、CPN 收费模式、Arc 主网 / 费用模型、分销协议变化。

**保存路径**
本轮最终回复由 cron / `codex exec` 保存到：

`work_docs/quarterly-earnings/2026-06-22T02-55-41-237760+00-00-quarterly-earnings-codex-final.md`

为避免 collector 后二次刷新造成口径漂移，本轮没有再调用本地 `agent-run --save` 生成额外 Rust 报告；后续 agent 全部使用同一 SQLite evidence batch 的 `--no-collect` 上下文。

以上为 CRCL 项目研究框架复核结论，不是个性化投资建议。
