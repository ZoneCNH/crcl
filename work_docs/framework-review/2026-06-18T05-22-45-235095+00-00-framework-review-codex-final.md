已按 cron 身份完成 `framework-review`，并按项目级 subagents 顺序跑完：collector -> data-quality/source -> financial/regulatory/competition/platform/autoresearch -> risk-decision。全程后续 agent 都使用同一批 evidence / `--no-collect` 上下文。

**结论**
`revise`，附风险动作：`观察 / 不增强 / 不降级`。本轮不是框架失效，也不触发 Bull/增强；主要是执行层需要小修：

- 固定 `RLDC margin` 主口径为 `RLDC / Reserve income`
- `RLDC / total revenue and reserve income` 只能作为全口径辅助值
- competition score 增加 coverage gate，避免低覆盖归一分被当作 action-grade headline

**data-quality**
不阻断。统一 batch 为 `decision-pack-1781760276520173000-97823`，collector 结果：

- `ok_sources=23`
- `warn_sources=0`
- `source_runs ok=39`
- `missing_items=0`

**关键证据**
- Circle Transparency：USDC circulation `74.860bn`、reserves `75.10bn`、7D net issuance `-1.05bn`、30D net issuance `-1.96bn`，日期 `2026-06-15`，来源 `https://www.circle.com/transparency`
- SEC Circle Q1 2026 10-Q：reserve income `652.508m`、distribution/transaction costs `405.402m`、RLDC `287.352m`、Other revenue `41.625m`，report date `2026-03-31`，filing date `2026-05-11`
- RLDC margin：主口径 `44.04%`，高于 `38%` 降级线；库内 `P2_CIRCLE_RLDC_MARGIN=41.397%` 是全口径辅助值，不能套主告警线
- GENIUS Act：`Public Law No.119-27`，latest action `2025-07-18`，本批抓取 `2026-06-18`
- Competition：coverage-adjusted score `71.4`，但可得权重仅 `35%`，不得作为完整 action-grade headline
- Platform：Other revenue share `5.997%`，只支持弱平台化；CPN/Arc 收费模型与收入归属缺失

**missing_info**
- Base `Partially Degraded Service` 的 incident 详情，以及是否影响 USDC/CPN/Arc 路径
- Agency final rule 对 third-party incentive、yield sharing、Coinbase rewards 的官方定义和经济影响
- CPN fee/revenue、Arc mainnet、Arc fee model、真实客户、收入归属
- Competition D1/D3/D4 覆盖不足，尤其 AUM 周环比、borrow utilization、可得权重不足
- Reserve Fund 7D yield 最新日度交叉验证，本轮为 `2026-05-29` 的官方披露周期口径

**反证 / 解除条件**
- RLDC 主/辅口径在 valuation、validation、competition scoring 中完成固化，且后续输出不再用全口径套 38% 主告警线
- Base incident 详情确认不影响 USDC 核心转账、赎回、结算，或状态恢复 Operational
- OCC/Treasury/FinCEN/OFAC/FDIC 等最终规则逐条解析后，未限制第三方分销激励或 Coinbase/钱包/交易所奖励结构
- Competition coverage 回到 `>=70%`，D1/D3/D4 关键输入补齐后，score 才恢复 action-grade

**下一步**
- 后续文档修改候选：`docs/valuation/00-valuation-framework.md` 固化 `RLDC / Reserve income` 为主口径
- 后续验证矩阵候选：`docs/metrics/06-validation-matrix.md` 的 WE-01 后新增 WE-01a
- Competition 规则候选：coverage `<70%` 或 D1/D3 缺失时标记 `coverage-adjusted / not action-grade`
- 下次触发：下一次 Circle Transparency 更新、Base incident 更新、Q2 2026 10-Q / earnings release、agency final rule 或正式执法、competition coverage 补齐至 `>=70%`

已保存：

- [work_docs/framework-review/2026-06-18T05-22-45-235095+00-00-framework-review-codex-final.md](/Users/zj/sync_disk/zjSync/量化/crcl/work_docs/framework-review/2026-06-18T05-22-45-235095+00-00-framework-review-codex-final.md)
- [work_docs/framework-review/2026-06-18T05-22-45-235095+00-00-framework-review-subagent-summary.md](/Users/zj/sync_disk/zjSync/量化/crcl/work_docs/framework-review/2026-06-18T05-22-45-235095+00-00-framework-review-subagent-summary.md)

不构成个性化投资建议。

