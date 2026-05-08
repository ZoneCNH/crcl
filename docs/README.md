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

禁止只写“利好”“利空”。
每条判断都必须说明对应层级、指标、阈值和下一步。
