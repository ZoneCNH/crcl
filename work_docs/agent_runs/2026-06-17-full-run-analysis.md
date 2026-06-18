# CRCL 2026-06-17 完整跑批总分析报告

生成时间：2026-06-18  
数据口径：基于 2026-06-17 已保存的 monitoring、daily-monitor、weekly-review、valuation-decision、quarterly-earnings、framework-review 六组 cron / Codex final，不重新刷新外部数据。  
报告用途：把 USDC 规模、储备收益、分销成本、监管规则、平台化进展，以及财务、链上、竞争、估值、风险信号收口为一份可复盘结论。  
说明：以下内容不构成个性化投资建议。

## 1. 总结论

本轮 CRCL 完整跑批结论为：**观察 / Base**。

不增强：USDC 规模和竞争侧没有共振。USDC 7D / 30D 净发行仍为负，USDC/USDT ratio 约 0.402，competition score 最高只到 55.0，未穿越 75 的增强阈值。平台化仍处于弱验证，Other revenue share 约 6%，CPN 和 Arc 缺可验证收入拆分。

不降级：信用层未坏。USDC/USD 接近 1，Circle Transparency 显示储备覆盖仍高于流通量，未见正式执法、储备异常、核心银行通道中断或 USDC 脱锚。Q1 2026 RLDC 主口径约 44.04%，仍高于 38% 降级线。

执行层需要 **revise**：框架自检确认应保留 Base 主框架，但需要新增 RLDC margin 输出前口径校验，防止把全口径 `RLDC / total revenue and reserve income` 误用于主告警线。主告警线只能用于 `RLDC / reserve income`。

## 2. Data Quality 与 Source 结论

collector 已完成采集。六个 workflow 的 cron meta 均显示 `exit_status=0`，且均为 `no_collect=false`，即昨天这轮不是只读旧库，而是允许刷新后保存的完整跑批。

data-quality 整体判断：

| Workflow | 结论 | Data-quality |
| --- | --- | --- |
| monitoring | 紧急 | 不阻断输出，但 Treasury news P0 官方源不可达，不能收成正常 |
| daily-monitor | 观察 | 不阻断，降置信度 |
| weekly-review | 观察 | 不阻断，主要缺 CoinGlass |
| valuation-decision | 观察 / 低置信度 Base | 阻断完整估值仓位判断 |
| quarterly-earnings | 观察 / Base | 不阻断 |
| framework-review | revise | 不阻断，但降置信度 |

主要缺口：

- `P1_EXCHANGE_USDC_BALANCES`：缺 `COINGLASS_API_KEY`，交易所 USDC 余额不可用。
- monitoring 批次出现 `Treasury news network_error`，不能确认财政部当日新闻/监管更新为空。
- valuation-decision 批次出现 `FinCEN news room network_error` 和 USDXX yield stale，因此只能输出低置信度 Base，不能完成强仓位动作判断。
- Base status 显示 `Partially Degraded Service / minor`，需要人工确认 incident 组件、时长和是否 resolved。

## 3. 核心跟踪项

### 3.1 USDC 规模与储备覆盖

结论：**观察，规模端偏弱但信用层未坏**。

关键证据：

- USDC in circulation：`74.86B USD`，数据日 `2026-06-15`，来源 Circle Transparency。
- Total reserves：`75.10B USD`，数据日 `2026-06-15`，来源 Circle Transparency。
- Reserve coverage：约 `100.32%`，储备高于流通量。
- USDC 7D net issuance：`-1.05B USD`，数据日 `2026-06-15`，来源 Circle Transparency。
- USDC 30D net issuance：`-1.96B USD`，数据日 `2026-06-15`，来源 Circle Transparency。
- USDC/USD：约 `0.9996-0.9997`，数据日 `2026-06-17`，来源 CoinGecko。
- Curve 3pool USDC ratio：`17.4768%`，数据日 `2026-06-17`，来源 Curve API。

解释：净发行转负是观察项，但 USDC/USD、储备覆盖和链上流动性未给出信用层降级证据。

### 3.2 储备收益

结论：**中性，不能作为增强依据**。

关键证据：

- Q1 2026 reserve income：`652.508M USD`，数据期 `2026-03-31`，来源 SEC 10-Q。
- Average USDC：`75.2B USD`，低于财报前 `76-80B` 基准。
- 3M Treasury：`3.79%`，数据日 `2026-06-16`，来源 U.S. Treasury。
- SOFR：`3.69%`，数据日 `2026-06-15`，来源 NY Fed。
- Circle Reserve Fund 7D net yield：`3.57%`，报告期 `2026-05-31` / 观测口径 `2026-05-29`，来源 SEC EDGAR BlackRock N-MFP3；valuation 主线程补核 BlackRock 页面显示 `2026-06-16` 为 `3.56%`，但未写回同批 packet。

解释：储备收益仍支撑财务桥，但 Average USDC 未强化，收益率也没有提供上修估值的独立证据。

### 3.3 分销成本与 RLDC

结论：**财务桥中性偏强化，是 Base 的核心托底**。

关键证据：

- Total revenue and reserve income：`694.133M USD`，数据期 `2026-03-31`，来源 SEC 10-Q。
- Reserve income：`652.508M USD`，数据期 `2026-03-31`，来源 SEC 10-Q。
- Other revenue：`41.625M USD`，数据期 `2026-03-31`，来源 SEC 10-Q。
- Distribution and transaction costs：`405.402M USD`，数据期 `2026-03-31`，来源 SEC 10-Q。
- Total distribution, transaction and other costs：`406.781M USD`，数据期 `2026-03-31`，来源 SEC 10-Q。
- RLDC：`287.352M USD`，数据期 `2026-03-31`。
- RLDC / reserve income 主口径：约 `44.04%`，高于 38% 降级线和 40% 强化观察线。
- RLDC / total revenue and reserve income 全口径：约 `41.40%`，只能辅助披露，不应用作 38% 主告警线。

解释：RLDC 主口径较强，但 Average USDC 和竞争侧没有共同强化，所以本轮不升级为增强。

### 3.4 监管规则

结论：**P1 观察，未触发 P0 降级**。

关键证据：

- GENIUS Act 已确认为 Public Law No. `119-27`。
- 本轮未确认新增 final rule、正式执法、制裁或明确改变 Circle / Coinbase 分销经济性的监管文本。
- monitoring 批次 Treasury news 不可达；valuation 批次 FinCEN news room 不可达。
- OCC / Treasury / FinCEN / OFAC / FDIC / SEC / CFTC 多数仍停留在 source check 或标题级别，缺 section-level 官方原文解析。

解释：监管已经是关键变量，但本轮没有从“规则方向”升级成“财务模型已被重写”的证据。由于官方源存在不可达批次，监管结论必须降置信度。

### 3.5 平台化进展

结论：**弱验证，不能切换到支付基础设施股框架**。

关键证据：

- Other revenue：`41.625M USD`，数据期 `2026-03-31`，来源 SEC 10-Q。
- Other revenue share：约 `5.997%`，处于 5-10% 弱平台化区间。
- CPN annualized TPV：`8.3B USD/year`，数据期 `2026-03-31`，来源 Circle Q1 press release。
- Arc：public testnet；framework-review 记录 weekly transactions `21,938,428`、weekly accounts `53,917`，但缺 mainnet、费用模型、客户收入归属。

解释：平台化有早期信号，但还不是收入模型切换证据。需要 CPN take rate、收入确认、客户留存，以及 Arc 主网和收费模型。

## 4. 财务、链上、竞争、估值、风险统一判断

### 4.1 财务

财务桥支持 Base：RLDC 主口径高于 40%，Other revenue 进入弱平台化区间，Adjusted EBITDA 和 Q1 指标没有给出降级证据。

限制是 Average USDC 低于财报前基准，reserve return rate 未强化，Other revenue 仍缺拆分。财务不能单独推动增强。

### 4.2 链上与市场

链上规模偏弱：USDC 7D / 30D net issuance 为负，USDC market share 约 `23.87%`，USDC/USDT ratio 约 `0.402`。

链上信用未坏：USDC/USD 接近 1，储备覆盖正常，Curve 3pool 未显示异常，Circle status 为 `All Systems Operational`。

Base status 是本轮状态页风险点：`Partially Degraded Service / minor`，需要跟踪是否影响 USDC 转账、赎回或 Base 生态使用。

### 4.3 竞争

结论：**中性偏弱到观察改善**。

关键证据：

- weekly-review competition score：`48.3`，中性偏弱。
- quarterly / framework-review competition score：`55.0`，较上周 `48.3` 上升 `+6.7`，但未越过 75 增强阈值。
- USDT dominance 约 `59.40%`，USDC/USDT ratio 未改善到趋势反转。
- D3 分销渠道议价较强，主要来自 RLDC margin。
- D4 / D5 缺完整 4 周趋势、AUM 周环比、borrow utilization 和链生态健康口径，已从部分加权中剔除或列为 missing_info。

解释：竞争端没有失控，但也没有给出增强。后续需要用同口径周度序列复核，而不是只看单日点位。

### 4.4 估值与仓位边界

结论：**低置信度 Base，data-quality 阻断完整 valuation-decision 判断**。

关键证据：

- CRCL 价格：`79.72 USD`。
- 稀释股数：`266.687M`。
- Market cap / annualized RLDC：约 `18.5x`。
- Base 估值区间：`15-22x`。
- 当前仓位未设置 `CRCL_CURRENT_POSITION_PCT`，因此只输出研究框架仓位上限，不判断具体加减仓量。
- data-quality 阻断解除前，研究框架仓位上限不超过 Base 下沿，valuation final 给出 `<=60%`。

解释：估值不便宜到无视数据缺口，也没有进入 Bear 失效后的 Bull 证据链。仓位动作需要等 FinCEN、USDXX、exchange balances 补采后重跑。

### 4.5 风险

本轮风险信号分层如下：

- P0 信用层：未触发。USDC/USD、储备覆盖、Circle status 未显示覆盖性降级。
- P0 source / monitoring：有批次级警报。Treasury news 与 FinCEN news room 在不同 workflow 中出现不可达，不能强行宣称监管无更新。
- P1 流动性：CoinGlass 缺 key，交易所 USDC balances 缺失。
- P1 状态页：Base status degraded，需要人工复核。
- 框架风险：RLDC margin 口径易混，需要文档化校验。

## 5. Workflow 产物摘要

| Workflow | 保存路径 | 主结论 | 关键含义 |
| --- | --- | --- | --- |
| monitoring | `work_docs/monitoring/2026-06-17T07-14-31-249868+00-00-monitoring-codex-final.md` | 紧急 | Treasury news 不可达 + Base degraded，不能收成正常 |
| daily-monitor | `work_docs/daily-monitor/2026-06-17T07-29-34-695268+00-00-daily-monitor-codex-final.md` | 观察 | Base 状态异常需跟踪，但未触发信用层降级 |
| weekly-review | `work_docs/weekly-review/2026-06-17T07-40-58-207654+00-00-weekly-review-codex-final.md` | 观察 | 增长质量中性偏弱，competition score 48.3 |
| valuation-decision | `work_docs/valuation-decision/2026-06-17T07-56-57-495789+00-00-valuation-decision-codex-final.md` | 观察 / 低置信度 Base | data-quality 阻断完整估值仓位判断 |
| quarterly-earnings | `work_docs/quarterly-earnings/2026-06-17T08-15-43-737989+00-00-quarterly-earnings-codex-final.md` | 观察 / Base | Q1 财务桥中性偏强化，但不升级 |
| framework-review | `work_docs/framework-review/2026-06-17T08-40-05-133172+00-00-framework-review-codex-final.md` | revise | 保留主框架，新增 RLDC 主/全口径校验 |

## 6. Missing Info

必须补齐：

- `COINGLASS_API_KEY` 或替代高优先级来源，用于交易所 USDC balances。
- Treasury news / FinCEN news room 的同批次恢复，或用官方替代入口人工确认监管更新为空。
- Base incident 的 component、duration、resolution、影响范围。
- CPN take rate、收费模式、收入确认、活跃机构数和客户留存。
- Arc mainnet、费用模型、客户收入、真实收入归属。
- Other revenue 拆分：CPN、Arc、API、其他一次性或经常性来源。
- D4 / D5 的 4 周趋势、收益型产品 AUM 周环比、Aave / Compound USDC borrow utilization、链上活跃地址周环比。
- GENIUS Act 及 OCC / Treasury / FinCEN / OFAC / FDIC / SEC / CFTC 实施细则的 section-level 官方原文解析。

## 7. 反证 / 解除条件

转强条件：

- USDC 30D change 转正，7D net issuance 连续 2 周转正。
- USDC market share 与 USDC/USDT ratio 稳定回升。
- competition score 稳定回到 `55+` 并进一步突破 `75`。
- RLDC 主口径持续 `>40%`，且 Average USDC 回到并站稳 `76-80B+`。
- Other revenue share `>10%`，且披露 CPN / Arc / 平台收费收入贡献。
- 监管细则没有破坏第三方激励、yield sharing 或 Coinbase rewards 等分销经济性。

降级条件：

- USDC/USD 持续折价 `>0.1%`。
- 储备覆盖异常、储备披露延迟或核心银行通道中断。
- RLDC 主口径 `<38%` 且无一次性解释。
- competition score 跌破 `40`。
- USDC 连续净赎回扩大，或单周净赎回超过季末余额 `1.5%` 且连续 2 周。
- 正式规则或执法明确限制第三方激励、收益分享或分销经济性。
- Base / Circle / Solana / Ethereum 状态异常联动到 USDC 转账、赎回或链上流动性。

解除本轮 data-quality 限制：

- CoinGlass 或替代来源补齐 exchange USDC balances。
- Treasury / FinCEN 官方新闻源在同一 workflow 中恢复 ok，或通过官方替代入口完成当日人工复核。
- USDXX / Circle Reserve Fund yield 使用可解释且已写回 batch 的最新口径。

## 8. 下一步动作

1. 配置 `COINGLASS_API_KEY`，或接入替代来源，补齐交易所 USDC balances。
2. 人工复核 Treasury news、FinCEN news room 和 Base status incident；确认是否已有 resolved 或官方替代入口。
3. 下次 valuation-decision 在补齐 FinCEN / USDXX / exchange balances 后重跑，否则只允许输出低置信度 Base。
4. 后续获得写入授权时，在 `docs/metrics/06-validation-matrix.md` 增加 WE-01A：38% 告警线只能用于 `RLDC / reserve income` 主口径，全口径只作辅助披露。
5. 下一次 Circle Transparency 更新后复核 USDC circulation、reserves、7D / 30D net issuance。
6. 下个周末快照复核 DefiLlama USDC share、USDC/USDT ratio 和 competition score。
7. Q2 2026 10-Q / earnings release 后复核 RLDC、Average USDC、Other revenue share、CPN / Arc 收入化证据。

## 9. 已保存路径

本总报告：

- `work_docs/agent_runs/2026-06-17-full-run-analysis.md`

源 workflow final：

- `work_docs/monitoring/2026-06-17T07-14-31-249868+00-00-monitoring-codex-final.md`
- `work_docs/daily-monitor/2026-06-17T07-29-34-695268+00-00-daily-monitor-codex-final.md`
- `work_docs/weekly-review/2026-06-17T07-40-58-207654+00-00-weekly-review-codex-final.md`
- `work_docs/valuation-decision/2026-06-17T07-56-57-495789+00-00-valuation-decision-codex-final.md`
- `work_docs/quarterly-earnings/2026-06-17T08-15-43-737989+00-00-quarterly-earnings-codex-final.md`
- `work_docs/framework-review/2026-06-17T08-40-05-133172+00-00-framework-review-codex-final.md`
