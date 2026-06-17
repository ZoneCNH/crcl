# CRCL framework-review subagent summary

workflow: framework-review
batch_id: decision-pack-1781685764001253000-52607
generated_at: 2026-06-17T08:44:10.452012+00:00
output_dir: work_docs/framework-review

## 结论

主处置：revise。

附属处置：
- keep: Base 主框架、RLDC 38% 主告警线、平台化弱验证框架。
- defer: 平台化/监管情景上调，等待收入拆分、Arc 主网收入化和监管细则逐条解析。
- reject: 用全口径 `RLDC / total revenue and reserve income` 替代主口径 `RLDC / reserve income` 触发 38% 告警线。

本轮最小框架修订建议：在 `docs/metrics/06-validation-matrix.md` 第二层新增 WE-01A，要求任何输出使用 RLDC margin 做 38% 告警、D3 分销渠道议价、情景切换或估值判断前，必须确认该值是主口径 `RLDC / reserve income`；全口径仅辅助披露。

## data-quality

不阻断，但降置信度进入判断。

- collector: ok_sources=22, warn_sources=1, missing_items=1.
- 无命令级失败，无 source unreachable，无官方源失败，无 P0 missing_info。
- 唯一明确缺口：`P1_EXCHANGE_USDC_BALANCES` 因 `COINGLASS_API_KEY` 缺失，CoinGlass API 返回 401。
- `P0_CIRCLE_RESERVE_FUND_7D_YIELD=3.57%` observed_at=2026-05-29，为 SEC N-MFP3 月度合理口径，不作为阻断项。

## 关键证据

- Circle Transparency 2026-06-15: USDC supply 74.86B USD, reserves 75.10B USD, reserve coverage 100.32%, 7D net issuance -1.05B USD, 30D net issuance -1.96B USD.
- CoinGecko 2026-06-17: USDC/USD 0.999737.
- SEC BlackRock N-MFP3 report_date 2026-05-31: Circle Reserve Fund 7D net yield 3.57%, WAM/WAL 12/12 days, net assets 65.17B USD.
- U.S. Treasury 2026-06-16: 3M T-Bill yield 3.79%; NY Fed 2026-06-15: SOFR 3.69%.
- DefiLlama 2026-06-17: USDC market share 23.876%, stablecoin total market cap 313.826B USD, USDC/USDT ratio 0.4020.
- Circle Q1 2026 SEC filing observed_at=2026-03-31: total revenue and reserve income 694.133M USD, reserve income 652.508M USD, costs 406.781M USD, RLDC 287.352M USD, RLDC margin main 44.04%, RLDC margin full 41.40%, Other revenue 41.625M USD, Other revenue share 5.997%, Adjusted EBITDA 151.401M USD.
- Circle Q1 press release 2026-03-31: CPN annualized TPV 8.30B USD/year.
- Arc 2026-06-17: public testnet, weekly transactions 21,938,428, weekly accounts 53,917.
- Congress/GovInfo fetched 2026-06-17: GENIUS Act became Public Law No. 119-27.
- Competition score 2026-06-17: 55.0, up from 48.3, not crossing 75 or 40 thresholds.

## missing_info

- `P1_EXCHANGE_USDC_BALANCES`: missing due to missing `COINGLASS_API_KEY`.
- D4/D5 competition dimensions: missing full 4-week/weekly trend inputs and borrow/utilization corroboration, excluded from weighted score.
- Regulatory details: OCC/Treasury/FinCEN/OFAC/FDIC/SEC/CFTC source checks are mostly reachability/title level; section-level official-text parsing is missing.
- Platform: CPN fee model, take rate, revenue contribution, Arc mainnet, Arc fee model, real customer revenue and Other revenue recurring/one-time split are missing.

## 反证 / 解除条件

-解除 `revise`: next quarterly filing T+24h output shows `P2_CIRCLE_RLDC_MARGIN` and hand-calculated `RLDC / reserve income` differ by <=0.5 ppt, and docs clearly mark full margin as auxiliary.
- Platform defer解除: Other revenue share >10% with CPN/Arc/platform fee or revenue source disclosure; >15% before considering stronger platform-infrastructure scenario.
- Regulatory defer解除: OCC/Treasury/FinCEN/OFAC/FDIC/SEC/CFTC official texts are parsed section by section and mapped to cost, access, reserve or distribution metrics.
- 降级触发: USDC/USD sustained depeg, reserve coverage breach, formal enforcement action, core banking channel disruption, or RLDC main margin <38% without one-off explanation.
- 增强触发: credit layer stable, RLDC main margin sustained >40%, competition score >75, Other revenue share >10%, and platform revenue verified.

## 下一步

1. Do not change valuation or platform scenario this run.
2. Add WE-01A to `docs/metrics/06-validation-matrix.md` in a future write-authorized pass.
3. Configure `COINGLASS_API_KEY` or use higher-priority alternatives before using exchange balances in competition/liquidity conclusions.
4. Parse GENIUS Act implementation rules and agency texts before converting regulatory source checks into rule conclusions.
5. Re-run framework-review after Q2 2026 10-Q / earnings release, after `P2_CIRCLE_RLDC_MARGIN` differs from the main hand calculation by >0.5 ppt, or after Other revenue share crosses 10%.

## Subagents

- crcl-data-collector: completed.
- crcl-data-quality-auditor: completed.
- crcl-source-verifier: completed.
- crcl-financial-valuation: completed.
- crcl-regulatory-watch: completed.
- crcl-competition-score: completed.
- crcl-platform-option: completed.
- crcl-autoresearch-curator: completed.
- crcl-risk-decision: completed; risk context command attempted but hit `Too many open files (os error 24)`, so it used upstream subagent evidence without refreshing or saving.

## Saved paths

- work_docs/framework-review/2026-06-17T08-40-05-133172+00-00-framework-review-subagent-summary.md
- work_docs/framework-review/2026-06-17T08-40-05-133172+00-00-framework-review-codex-final.md
