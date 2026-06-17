# CRCL weekly review - real subagents

workflow=weekly-review
generated_at=2026-06-17T01:01:08Z
evidence_packet=/tmp/crcl-agent-rerun/weekly-pack.json
base_runner_artifact=work_docs/agent_runs/2026-06-17T00-53-39-842Z-weekly-review.md
execution_model=先生成同一批 decision-pack evidence packet，再派发真实 Codex subagents 只读复核，最后由 crcl-risk-decision 收口。
confidence=low-to-medium

## final conclusion

结论：观察 / Base 框架暂不降级。

增强或 Bull 被信用层缺口、USDC 近端转弱和竞争分数中性偏弱阻断。本轮不触发财务桥硬降级，原因是 RLDC margin 仍高于 38% 降级阈值；但也不支持上调估值倍数或提高仓位上限。

动作：维持 Base 研究框架，暂停增强，暂停上调估值倍数，框架层仓位/风险动作为 0 ppts 调整。此处不是个性化投资建议。

## data-quality

结论：不硬阻断，但降置信度。

关键证据：
- collector_run：ok_sources=19，warn_sources=2，batch=decision-pack-1781657502940570000-75633，observed_at=2026-06-17T00:53:25Z。
- P1_EXCHANGE_USDC_BALANCES 缺失：CoinGlass API 返回 401，缺 COINGLASS_API_KEY。
- P2_FINRA_SHORT_INTEREST 本轮请求超时；现有 short interest 只能作为滞后筹码参考，且不能直接转为流通股占比。
- P0_CIRCLE_RESERVE_FUND_7D_YIELD = 3.57%，observed_at=2026-05-29，距本轮约 19 天，超过 P0 7 天阈值。

阻断边界：无 P0 硬阻断确认，但 P0 关键数据缺失/过期时不得输出增强或 keep。

## source verification

结论：观察。

关键证据：
- P0_USDC_CIRCULATING_SUPPLY = 75.017B USD，observed_at=2026-06-17，source=DefiLlama stablecoins；未过期，但不是 Circle Transparency 主源。
- P0_USDC_7D_CHANGE_PCT = -1.3685%，P0_USDC_30D_CHANGE_PCT = -2.63%，observed_at=2026-06-17，source=DefiLlama stablecoins。
- P0_CIRCLE_USDC_7D_NET_ISSUANCE = -1.05B USD，observed_at=2026-06-15，source=Circle Transparency。
- P0_CIRCLE_RESERVE_FUND_7D_YIELD = 3.57%，observed_at=2026-05-29，source=SEC N-MFP3 BlackRock filing，已过期。
- 3M T-Bill yield = 3.79%，observed_at=2026-06-16，source=U.S. Treasury。
- SOFR = 3.69%，observed_at=2026-06-15，source=NY Fed。

source missing_info：
- P0_CIRCLE_USDC_TOTAL_RESERVES 缺失。
- P0_USDC_USD_PRICE 缺失。
- P0_CURVE_3POOL_USDC_RATIO 缺失。
- P1_EXCHANGE_USDC_BALANCES 缺失。
- P1_ARC_MAINNET_STATUS 缺失。

## competition score

结论：观察 / 中性偏弱。

competition score：40。

D1 USDC 市占率趋势：25
- USDC market share = 23.8978%，USDC/USDT ratio = 0.4024，observed_at=2026-06-17，source=DefiLlama。

D2 USDT 相对威胁：50
- Stablecoin total market cap = 313.907B USD；USDT market cap = 186.408B USD；USDT dominance 约 59.38%，observed_at=2026-06-17，source=DefiLlama。

D3 分销渠道议价：75
- RLDC margin = 41.3973%，observed_at=2026-03-31，source=Circle SEC filing。
- Base USDC = 4.270B USD，USDC supply = 75.017B USD，Base 占比约 5.69%，observed_at=2026-06-17，source=DefiLlama。

D4 收益型稳定币蚕食：25
- USDC 7D net issuance = -1.05B USD，observed_at=2026-06-15，source=Circle Transparency。
- USDe market cap = 4.493B USD，observed_at=2026-06-17，source=CoinGecko。
- Tokenized Treasury AUM = 15.661B USD，observed_at=2026-06-16，source=RWA.xyz。

D5 链生态健康：0
- Adjusted transfer volume = 5.473B USD，observed_at=2026-06-16，source=Visa Onchain Analytics / Allium。
- Runner 口径 worst activity change = -82.58%；active addresses = 2,774,677，observed_at=2026-06-15，source=CoinMetrics。

D6 监管壁垒：50
- GENIUS Act source check 显示 Public Law No. 119-27；无新增明确限制第三方激励 / yield sharing 的 P0 事件。

D7 平台化验证：50
- Other revenue share = 5.9967%，observed_at=2026-03-31，source=Circle SEC filing。
- CPN annualized TPV = 8.3B USD/year，observed_at=2026-03-31，source=Circle Q1 2026 results press release。
- Arc = public testnet，observed_at=2026-06-17，source=Arc website。

计算：25%*25 + 15%*50 + 20%*75 + 15%*25 + 10%*0 + 10%*50 + 5%*50 = 40。

## financial valuation

结论：中性偏观察。

关键证据：
- RLDC margin = 41.3973%，observed_at=2026-03-31，source=Circle 10-Q / SEC EDGAR。高于 38% 降级阈值，也高于 40% Base 支撑线。
- Other revenue share = 5.9967%，observed_at=2026-03-31，source=Circle 10-Q / SEC EDGAR。落在 5%-10% 弱平台化区间，低于 10% 中验证门槛。
- USDC circulating supply 约 75.017B USD，observed_at=2026-06-17，source=DefiLlama；但 7D=-1.3685%、30D=-2.63%，7D net issuance=-1.05B USD，说明下一季 Average USDC 有压力。
- Reserve Fund 7-day net yield = 3.57%，observed_at=2026-05-29，source=SEC / BlackRock N-MFP3，已过期；辅助锚 3M T-Bill=3.79%、SOFR=3.69%。

估值影响：基础盘不硬降级，但不提高 Market cap/RLDC 倍数，不上调平台化 multiple，维持 Base 下沿到中枢观察。

## regulatory watch

结论：P1 观察，无 P0/P1 硬阻断。

关键证据：
- Congress / GovInfo GENIUS Act page：包内 2026-06-17 source_check，latest action 为 2025-07-18: Became Public Law No: 119-27。
- OCC news releases：包内 2026-06-17T00:52:30Z 可达；框架记录 OCC 于 2026-02-25 发布 GENIUS Act 拟议规则，包内未证明已有 final rule。
- Treasury / FinCEN / OFAC：包内 2026-06-17 页面可达；框架记录 2026-04-08 发布 BSA/AML 与制裁合规拟议规则。
- 包内没有官方文本显示第三方激励、交易所奖励、钱包返利或 Coinbase 分成已被最终规则认定为变相 yield。

监管动作：D6 维持中性或沿用处理；不因本包证据下调或上调 RLDC margin。

## platform option

结论：弱验证，不满足主框架切换。

关键证据：
- Other revenue share = 5.9967%，observed_at=2026-03-31，source=CRCL SEC filing。只对应弱平台化。
- CPN annualized TPV = 8.3B USD/year，observed_at=2026-03-31，source=Circle Q1 2026 results press release。TPV 是使用量，不是收入验证。
- Arc public network status = public testnet，observed_at=2026-06-17，source=Arc website。不是 mainnet，也不是收入验证。

平台化动作：维持“利差股 + 平台期权”，不切换为支付基础设施股；Bull 的支付基础设施倍数不释放。

## risk-decision synthesis

结论：观察。

信用层未确认触发 C-TRIGGER 或信用层硬失效；本轮没有确认 USDC/USD 折价、储备异常、核心银行通道中断或正式执法行动。但 P0 关键数据缺失/过期，信用层只能判“未清空风险”，覆盖财务桥中 RLDC margin 单项较强的增强信号。

主因：
1. Reserve Fund 7D yield 过期，且 P0 Circle total reserves、USDC/USD、Curve 3pool 在周度包中缺失，阻断增强。
2. USDC 7D=-1.3685%、30D=-2.63%、7D net issuance=-1.05B USD，进入赎回压力观察。
3. Competition score=40，落在 40-54 中性偏弱档，暂停上调估值倍数。
4. RLDC margin=41.3973% 支撑 Base，但 Other revenue share=5.9967%、Arc=testnet、CPN 只有 TPV，平台化不支持上修。
5. 监管无 P0 硬阻断，但 final rule 和第三方激励边界仍是观察项。

## missing_info

- P0_CIRCLE_USDC_TOTAL_RESERVES 缺失。
- P0_USDC_USD_PRICE 缺失。
- P0_CURVE_3POOL_USDC_RATIO 缺失。
- P0_CIRCLE_RESERVE_FUND_7D_YIELD 过期 19 天。
- P1_EXCHANGE_USDC_BALANCES 缺失，CoinGlass API key missing。
- P2_FINRA_SHORT_INTEREST 超时，且当前 short interest 不能转为流通股占比。
- P1_ARC_MAINNET_STATUS 缺失。
- CPN/Arc 可重复收入、费率模型、收入归属缺失。
- 最新季度 Average USDC、distribution and transaction costs 绝对额及分销成本率缺失。
- OCC / Treasury / FinCEN / OFAC / FDIC final rule 正文与正式执法行动仍需持续复核。

## rebuttals and release conditions

反证：
- RLDC margin=41.3973%，未触发 <38% 财务桥降级。
- 当前包内未确认监管 P0、正式执法、第三方激励禁令或信用层硬失效。
- Competition score=40 未跌破 40，暂不自动进入 Bear 重算。

解除条件：
- P0 数据补齐：Circle total reserves、USDC/USD、Curve 3pool、Reserve Fund 7D yield 均恢复最新且无异常。
- USDC 7D/30D change 转正，或至少 net issuance 连续转正并确认不是单周噪音。
- Competition score 回到 55+；若要增强，需要 75+ 或财务桥、竞争、平台化多矩阵共振。
- Other revenue share >10% 且连续两个季度增长，并披露 CPN/Arc 可重复收入；主框架切换需 Other revenue >15% 并满足估值框架切换条件。
- OCC / Treasury / FinCEN / OFAC / FDIC final rule 明确不限制服务费、渠道分成或用户激励，且 Circle / Coinbase 披露未调整 USDC rewards 或分成结构。

## next review trigger

- USDC 市占率连续变化。
- Net issuance 连续第二周为负，或连续 4 周且累计流通量下降 >2%。
- Competition score 跌破 40 或回到 55。
- RLDC margin / Other revenue 更新。
- GENIUS / OCC / Treasury / FinCEN / OFAC final rule 或正式执法。
- CPN / Arc 收入披露，或 Arc mainnet 官方上线并有 SEC/IR/Circle 官方源确认。

## subagents

- crcl-data-quality-auditor: 019ed311-f99a-7df0-8a49-4da2a2388943
- crcl-source-verifier: 019ed312-2e94-7861-8b5f-bf8a4d243419
- crcl-competition-score: 019ed313-f9e7-7ed3-832c-b3b6b164b784
- crcl-financial-valuation: 019ed314-159c-7d71-b075-67f7c534fa1e
- crcl-regulatory-watch: 019ed314-2bcc-7041-83a6-2b6c5b089fee
- crcl-platform-option: 019ed314-44ca-7151-ad6e-a45419c9fc79
- crcl-risk-decision: 019ed316-bbb1-72f0-9c42-7cfc1611834d
