# CRCL framework-review subagent summary

generated_at=2026-06-18T05:22:45+00:00
workflow=framework-review
batch_id=decision-pack-1781760276520173000-97823

## dispatch

- crcl-data-collector: completed. `ok_sources=23`, `warn_sources=0`, `source_runs ok=39`, `missing_items=0`.
- crcl-data-quality-auditor: completed. data-quality 不阻断；stale 候选为官方披露周期或状态源业务状态。
- crcl-source-verifier: completed. 事实足以支持判断；RLDC margin 口径、Base status 详情、监管逐条原文为重点缺口。
- crcl-financial-valuation: completed. 财务桥结论为小修 `revise`，核心是 RLDC margin 主/辅口径固化。
- crcl-regulatory-watch: completed. GENIUS Act 为 Public Law No.119-27；监管维持 P1，细则逐条解析 `defer`。
- crcl-competition-score: completed. 竞争规则小修 `revise`；35% coverage 的归一分 `71.4` 不能作为 action-grade headline。
- crcl-platform-option: completed. 平台化弱验证；规则 `keep`，平台化切换 `defer`。
- crcl-autoresearch-curator: completed. 微迭代候选为 RLDC margin 主/辅口径治理；阈值不改。
- crcl-risk-decision: completed. 最终 `revise`，风险动作 `观察 / 不增强 / 不降级`。

## unified conclusion

结论：`revise`。框架不 reject，不升级 Bull；信用层未触发硬覆盖，风险动作保持观察。

data-quality 是否阻断：否。统一 evidence batch 为 `decision-pack-1781760276520173000-97823`，collector `ok_sources=23`、`warn_sources=0`、`source_runs ok=39`、`missing_items=0`。

## key evidence

- Circle Transparency：USDC circulation `74.860bn`、reserves `75.10bn`、7D net issuance `-1.05bn`、30D net issuance `-1.96bn`，observed_at=`2026-06-15`，source=`https://www.circle.com/transparency`。
- DefiLlama：USDC supply `74.812bn`、market share `23.793%`，observed_at=`2026-06-18`，source=`https://stablecoins.llama.fi/stablecoins?includePrices=true`。
- SEC Circle Q1 2026 10-Q：reserve income `652.508m`、distribution/transaction costs `405.402m`、RLDC `287.352m`、Other revenue `41.625m`，report_date=`2026-03-31`，filing_date=`2026-05-11`，source=`https://www.sec.gov/Archives/edgar/data/1876042/000187604226000150/crcl-20260331.htm`。
- RLDC margin：主口径 `RLDC / Reserve income = 44.04%`；当前库内 `P2_CIRCLE_RLDC_MARGIN=41.397%` 是 `RLDC / total revenue and reserve income` 全口径辅助值，不得触发 38% 主告警线。
- GENIUS Act：`Public Law No.119-27`，latest_action=`2025-07-18`，batch_fetched=`2026-06-18`，source=`https://www.govinfo.gov/bulkdata/BILLSTATUS/119/s/BILLSTATUS-119s1582.xml`。
- Competition：coverage-adjusted score `71.4`，但可得权重仅 `35%`，不得作为完整 action-grade headline。
- Platform：Other revenue share `5.997%`，只支持弱平台化；CPN/Arc 收费模型与收入归属缺失。

## missing_info

- Base `Partially Degraded Service` 的 incident 标题、component、duration、resolution，以及是否影响 USDC/CPN/Arc 路径。
- Agency final rule 对 third-party incentive、yield sharing、Coinbase rewards 的官方定义和经济影响。
- CPN fee/revenue、Arc mainnet、Arc fee model、真实客户、收入归属。
- Competition D1/D3/D4 覆盖不足，尤其 AUM 周环比、borrow utilization、可得权重不足。
- Reserve Fund 7D yield 最新日度交叉验证；本轮为 `2026-05-29` 的官方披露周期口径。

##反证 / 解除条件

- RLDC 主/辅口径在 valuation、validation、competition scoring 中完成固化，且后续输出不再用全口径套 38% 主告警线。
- Base incident 详情确认不影响 USDC 核心转账、赎回、结算，或状态恢复 Operational。
- OCC/Treasury/FinCEN/OFAC/FDIC 等最终规则逐条解析后，未限制第三方分销激励或 Coinbase/钱包/交易所奖励结构。
- Competition coverage 回到 `>=70%`，D1/D3/D4 关键输入补齐后，score 才恢复 action-grade。

## next actions

- 后续文档修改候选：在 `docs/valuation/00-valuation-framework.md` 固化 `RLDC / Reserve income` 为主口径；在 `docs/metrics/06-validation-matrix.md` 的 WE-01 后新增 WE-01a。
- Competition 规则候选：coverage `<70%` 或 D1/D3 缺失时标记 `coverage-adjusted / not action-grade`。
- Regulatory 候选：D6 文案区分 Public Law 与 agency final rule。
- 下次触发：下一次 Circle Transparency 更新、Base incident 更新、Q2 2026 10-Q / earnings release、agency final rule 或正式执法、competition coverage 补齐至 `>=70%`。

不构成个性化投资建议。
