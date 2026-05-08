# CRCL 研究框架入口

## 结论

CRCL 的研究主线不是“稳定币公司”四个字，而是五个判断：

1. USDC 规模能否持续增长。
2. 储备收益率下行时，规模增长能否抵消利率压力。
3. 分销成本率能否稳定或下降。
4. GENIUS Act 后的监管壁垒能否大于合规成本。
5. CPN、Arc、企业结算能否把 CRCL 从利差股推向支付基础设施股。

利差股是依赖资产收益率赚钱的公司，白话就是靠储备资产吃利息。
支付基础设施股是靠网络、API、结算和客户黏性赚钱的公司，白话就是靠金融管道收服务费。

## 阅读顺序

1. `framework/00-research-map.md`：先看总研究地图。
2. `metrics/00-metric-dictionary.md`：再看指标口径。
3. `metrics/04-competition-dashboard.md`：每周填竞争仪表盘。
4. `risk/00-risk-map.md`：确认哪些信号会破坏投资逻辑。
5. `valuation/00-valuation-framework.md`：把业务指标映射到估值。
6. `playbook/00-research-routine.md`：按日、周、季执行。
7. `playbook/02-competition-review.md`：按竞争维度做专项复盘。
8. `autoresearch/00-loop.md`：用固定循环继续迭代研究框架。
9. `sources.md`：最后核对官方来源。

## 目录职责

| 目录            | 职责                                       |
| --------------- | ------------------------------------------ |
| `framework/`    | 商业模式、监管、竞争、平台化的主逻辑       |
| `metrics/`      | 数据字典、监控表、财报检查清单             |
| `risk/`         | 风险地图、预警信号、失效条件               |
| `valuation/`    | 估值锚、情景模型、倍数切换条件             |
| `playbook/`     | 研究流程、决策模板、复盘格式               |
| `autoresearch/` | 1000 次微迭代日志、目标函数、保留/丢弃规则 |
| `archive/`      | 原始文档归档，不作为最新结论               |
| `sources.md`    | 官方来源索引和核验优先级                   |

## Autoresearch 矩阵入口

上一轮 1000 次 autoresearch 中，474 条已落到 `framework/`。

| 使用场景 | 入口文件                         | 执行文件                              |
| -------- | -------------------------------- | ------------------------------------- |
| 财报复核 | `framework/01-business-model.md` | `metrics/03-quarterly-earnings.md`    |
| 监管复核 | `framework/02-regulation.md`     | `playbook/00-research-routine.md`     |
| 竞争复核 | `framework/03-competition.md`    | `metrics/04-competition-dashboard.md` |
| 平台复核 | `framework/04-platform-option.md` | `playbook/02-competition-review.md`   |

矩阵不是新结论。
矩阵是复核顺序，白话就是遇到财报、监管、竞争、产品事件时先查哪张表。

## IPO 后股权结构与锁定期时间表

IPO 后筹码结构是短期波动放大器，不影响基本面结论，但影响仓位纪律执行。

### 核心筹码事实（截至 2026-05-08）

以下数据应在每次 Form 4、13F、SEC 归档后更新：

| 类别 | 已知状态 | 数据来源 | 下次更新时机 |
| ---- | -------- | -------- | ------------ |
| 锁定期到期时间 | 需从 S-1/Prospectus 确认具体日期 | SEC EDGAR S-1/424B4 | IPO 6 个月后到期，需核实具体日期 |
| 内部人持股比例 | 需从最新 DEF 14A 或 10-Q 确认 | SEC EDGAR DEF 14A | 季度更新 |
| 机构持仓（13F） | 每季度披露，有 45 天滞后 | SEC EDGAR 13F | Q1 2026 持仓约 2026-05-15 前披露 |
| 做空比例（short interest） | 双周更新 | FINRA/交易所 | 双周 |
| 员工期权和 RSU 兑现计划 | 需从 S-1 或 10-K 确认 | SEC EDGAR 10-K 注释 | 年报后更新 |

### 锁定期动作规则

| 锁定期状态 | 风险等级 | 动作 |
| -------- | -------- | ---- |
| 锁定期到期前 30 天 | P2 观察 | 提前核实解禁规模，不操作 |
| 锁定期到期当周 | P1 监控 | 减少额外加仓，观察大宗交易 |
| 锁定期到期后有大额 Form 4 减持 | P1 警报 | 触发仓位减仓因子（见 valuation/00 仓位加减表） |
| 短期内多位高管连续减持 | P0 预警 | 参考 risk/01 预警信号 12 号条目 |

### 13F 和 Form 4 追踪规则

- 13F 有 45 天延迟，Q4 2025 机构持仓最晚 2026-02-15 披露；Q1 2026 持仓最晚 2026-05-15 披露。
- Form 4 为 2 个工作日内实时披露，内部人交易必须关注。
- 使用 SEC EDGAR Full-Text Search 或 `investor.circle.com` 跟踪。
- 13F 数据不用于基本面估值，只用于筹码压力判断（P2 信号）。

### 与基本面框架的隔离原则

筹码信号（解禁、做空、Form 4）与基本面情景判断完全隔离：

- 筹码 P2 信号 → 只调仓位，不触发情景切换，不修改估值倍数。
- 即使出现大额解禁，只要基本面 Base case 成立，情景判断不变。
- 基本面框架 → `framework/` + `valuation/` + `risk/`。
- 筹码框架 → `valuation/00-valuation-framework.md` 仓位加减因子表。

## 更新规则

P0 指标每日更新，P1 指标每周更新，P2 指标按季度或事件更新。
每次更新必须写清楚日期、数据源、结论变化和动作变化。

官方源优先级高于二级数据平台。
财报和监管文件优先级高于新闻稿和媒体解读。

## 事实源

| 来源                           | 用途                              |
| ------------------------------ | --------------------------------- |
| Circle Investor Relations      | 财报、指引、CPN、Arc、管理层表述  |
| Circle Transparency            | USDC 储备和流通量                 |
| SEC EDGAR                      | 10-K、10-Q、Form 4、S-1、13F      |
| Congress.gov                   | GENIUS Act 与后续修订             |
| OCC / Treasury / FinCEN / OFAC | 稳定币监管细则、BSA/AML、制裁合规 |
| DefiLlama / CoinGecko / Dune   | 稳定币市占率、链上分布、使用强度  |

## 当前研究基准

截至 2026-05-08，框架使用以下已核验事实：

- Circle 已公布 FY2025 与 Q4 2025 业绩。
- Circle Q1 2026 财报发布日期为 2026-05-11。
- GENIUS Act 已在 2025-07-18 成为 Public Law No. 119-27。
- OCC 已在 2026-02-25 发布 GENIUS Act 拟议规则。
- Treasury / FinCEN / OFAC 已在 2026-04-08 发布稳定币 BSA/AML 与制裁合规拟议规则。

## 输出纪律

任何结论都必须落到三类动作之一：

1. 增强：核心假设被数据强化。
2. 降级：估值或持仓逻辑需要下修。
3. 观察：数据不足，等待下一项 P0/P1/P2 证据。

禁止只写”利好””利空”。
每条判断都必须说明对应层级、指标、阈值和下一步。

## 框架自检 SOP

### 触发条件（以下任一出现时启动自检）

| 触发 | 说明 |
| ---- | ---- |
| 每季度财报后 | 强制自检，不可省略 |
| 情景切换 | Bull ↔ Base ↔ Bear 任何方向切换后 |
| 监管重大变化 | OCC / Treasury 最终规则发布 |
| USDC 市占率越档 | 超过 25% 或跌破 20% |
| competition score 越档 | 穿越 75 或 40 分边界 |
| 框架假设被质疑 | 使用者或数据表明框架某个参数明显错误 |

### 自检项目

| 自检项 | 检查文件 | 检查问题 |
| ------ | -------- | -------- |
| 业务模式假设 | `framework/01-business-model.md` | FY2025 基准数字是否仍是最新财报数据？ |
| RLDC margin 阈值 | `metrics/00-metric-dictionary.md` | 38% 告警线是否仍合理？（参考近 4 季实际值） |
| 竞争评分权重 | `framework/03-competition.md` | 各维度权重是否反映当前竞争格局？ |
| 情景参数 | `valuation/01-scenario-model.md` | Bull/Base/Bear 的 USDC 假设是否需要更新？ |
| 目标价倍数 | `valuation/00-valuation-framework.md` | Market cap / RLDC 倍数区间是否仍与可比公司匹配？ |
| 风险优先级 | `risk/00-risk-map.md` | P0/P1/P2 风险分类是否有需要重新排序的项？ |
| 数据源有效性 | `sources.md` | 有无过期 URL 或来源不再更新的情况？ |

### 自检输出

```text
自检日期：
触发原因：
检查项：（逐项填写）
  业务模式假设：有变化 / 无变化
  RLDC margin 阈值：有变化 / 无变化，说明：
  竞争评分权重：有变化 / 无变化，说明：
  情景参数：有变化 / 无变化，说明：
  目标价倍数：有变化 / 无变化，说明：
  风险优先级：有变化 / 无变化，说明：
  数据源有效性：有失效链接 / 全部有效
变更动作：
  需更新文件：
  需新建任务：
下次自检触发：
```
