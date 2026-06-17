结论：**revise（主处置）**。本轮框架不降级，也不增强；保留 Base 主框架和平台化弱验证框架，但需要修正执行层：新增 **RLDC margin 输出前口径校验**，防止把全口径 `RLDC / total revenue and reserve income` 误当成主口径 `RLDC / reserve income` 去套 38% 告警线。附属处置：**keep** 现有主框架，**defer** 平台化/监管上调，**reject** 全口径替代主口径。

data-quality：**不阻断，但降置信度进入判断**。collector 已先生成统一 evidence packet：`batch_id=decision-pack-1781685764001253000-52607`，`ok_sources=22`、`warn_sources=1`、`missing_items=1`。唯一明确缺口是 `P1_EXCHANGE_USDC_BALANCES`，原因是缺 `COINGLASS_API_KEY`；无官方源失败、无 P0 missing_info、无 source unreachable。

关键证据：
- Circle Transparency，2026-06-15：USDC supply `74.86B`，reserves `75.10B`，reserve coverage `100.32%`；来源 `https://www.circle.com/transparency`。
- CoinGecko，2026-06-17：USDC/USD `0.999737`，未触发信用层覆盖。
- SEC / Circle Q1 2026 filing，2026-03-31：reserve income `652.508M`，RLDC `287.352M`，RLDC 主口径 `44.04%`，全口径 `41.40%`，Other revenue `41.625M`，Other revenue share `5.997%`。
- DefiLlama，2026-06-17：USDC market share `23.876%`，stablecoin total market cap `313.826B`，USDC/USDT ratio `0.4020`。
- Competition score：`55.0`，上周 `48.3`，未穿越 `75` 增强阈值，也未跌破 `40` 降级阈值。
- GENIUS Act：已确认为 Public Law No.119-27，但 OCC/Treasury/FinCEN/OFAC/FDIC/SEC/CFTC 细则仍缺逐条原文解析。

missing_info：
- `P1_EXCHANGE_USDC_BALANCES` 缺失，需要配置 `COINGLASS_API_KEY` 或替代高优先级来源。
- 竞争 D4/D5 缺完整周度趋势和利用率口径，已从加权中剔除。
- CPN fee/revenue、Arc mainnet/费用模型/真实收入、Other revenue 拆分缺失。
- GENIUS Act 下游实施机构规则仍缺 section-level 官方原文解析。

反证 / 解除条件：
- 解除 revise：下次财报 T+24h 后，系统输出的 `P2_CIRCLE_RLDC_MARGIN` 与手算主口径差异 `<=0.5ppt`，且文档明确全口径只作辅助。
- 降级触发：USDC/USD 持续折价、储备覆盖异常、正式执法行动、核心银行通道中断，或 RLDC 主口径跌破 `38%` 且无一次性解释。
- 增强触发：信用层稳定、RLDC 主口径持续高于 `40%`、competition score > `75`、Other revenue share > `10%` 且平台收入可验证。

下一步：
1. 后续写入授权时，只做一个最小改动：在 `docs/metrics/06-validation-matrix.md` 新增 WE-01A，规定 38% 告警线只能用于 `RLDC / reserve income` 主口径。
2. 补 `COINGLASS_API_KEY` 后重跑 market/competition 相关采集。
3. 监管侧补 Congress/Public Law、OCC、Treasury/FinCEN/OFAC、FDIC 的逐条原文解析。
4. 下次触发：Q2 2026 10-Q / earnings release、Other revenue share > `10%`、RLDC 主/全口径差异 > `0.5ppt`、或监管正式规则/执法出现。

已保存路径：
- `work_docs/framework-review/2026-06-17T08-40-05-133172+00-00-framework-review-subagent-summary.md`
- `work_docs/framework-review/2026-06-17T08-40-05-133172+00-00-framework-review-codex-final.md`

说明：已按要求使用项目级 subagents 完成 collector、data-quality、source、financial、regulatory、competition、platform、autoresearch、risk-decision 流程。risk-decision 的本地 `agent-context --profile risk --no-collect` 尝试遇到 `Too many open files (os error 24)`，但未刷新数据，最终收口基于同一批上游 evidence packet 和 subagent 结果。不构成个性化投资建议。