# CRCL 研究地图

## 结论

CRCL 的完整研究框架分为四条主线和十二个支撑层。

四条主线：

1. 现金流主线：USDC 平均流通量 × 储备收益率 × RLDC margin。
2. 监管主线：合规发行人资格、储备要求、收益限制、BSA/AML。
3. 竞争主线：USDT、Coinbase、银行稳定币、收益型稳定币、RWA。
4. 平台化主线：CPN、Arc、企业结算、API、非储备收入。

RLDC 是 Revenue Less Distribution Costs，白话就是扣掉渠道分成后的收入。

## 十二层支撑结构

| 层级 | 名称         | 核心问题                          | 主要文件                              |
| ---- | ------------ | --------------------------------- | ------------------------------------- |
| 1    | 储备信用层   | USDC 能否维持 1:1 赎回信任        | `risk/02-failure-conditions.md`       |
| 2    | 利差现金流层 | USDC 规模能否抵消利率下行         | `framework/01-business-model.md`      |
| 3    | 网络分发层   | 增长来自自有网络还是渠道分发      | `framework/03-competition.md`         |
| 4    | 成本议价层   | 分销成本率是否吞噬规模收益        | `metrics/03-quarterly-earnings.md`    |
| 5    | 监管牌照层   | 合规壁垒是否转化为护城河          | `framework/02-regulation.md`          |
| 6    | 平台化期权层 | Other revenue 能否突破利差股定价  | `framework/04-platform-option.md`     |
| 7    | 链生态结构层 | USDC 增长质量来自哪里             | `metrics/02-weekly-review.md`         |
| 8    | 利率久期层   | 短端利率下行如何传导到收入        | `valuation/00-valuation-framework.md` |
| 9    | 赎回流动性层 | 储备和银行通道能否承压            | `risk/00-risk-map.md`                 |
| 10   | 竞争结构层   | 哪类对手压缩 CRCL 倍数            | `framework/03-competition.md`         |
| 11   | 宏观 beta 层 | Crypto 风险偏好是否支撑稳定币扩张 | `metrics/01-daily-watchlist.md`       |
| 12   | 股票筹码层   | 解禁、做空、期权是否放大波动      | `risk/01-warning-signals.md`          |

beta 是市场联动性，白话就是 BTC、crypto 行情会不会带着 CRCL 一起涨跌。

## 研究闭环

```text
官方数据
  -> 指标更新
  -> 层级映射
  -> 风险判断
  -> 估值调整
  -> 决策输出
  -> 下次复盘
```

每次复盘必须回答四个问题：

1. 核心利润驱动有没有变强。
2. 平台化证据有没有变硬。
3. 监管或竞争有没有改变分销经济性。
4. 股价是否已经透支基本面改善。

## 权重框架

| 模块                | 权重 | 解释                       |
| ------------------- | ---: | -------------------------- |
| 储备与赎回信用      |  25% | 信用破坏后所有估值讨论失效 |
| USDC 规模与储备收益 |  25% | 当前利润主引擎             |
| 分销成本与合作结构  |  15% | 决定规模增长留给谁         |
| 监管路径            |  15% | 决定合规壁垒和收益限制     |
| 平台化收入          |  10% | 决定长期估值上限           |
| 竞争与筹码结构      |  10% | 决定短中期波动和折价       |

权重不是交易仓位。
权重用于解释研究结论中哪条证据最重要。

## Autoresearch 落地索引

1000 次 autoresearch 微实验中，474 条直接落到 `framework/`。

| 文件                           | 条数 | 主题                     | 落地方式                               |
| ------------------------------ | ---: | ------------------------ | -------------------------------------- |
| `01-business-model.md`         |   52 | 商业模式与收入质量       | 财务桥、RLDC 阈值、财报复核项          |
| `02-regulation.md`             |   56 | GENIUS Act 与监管路径    | 监管矩阵、最终规则等待项、P0 触发条件  |
| `03-competition.md`            |  206 | 渠道、Coinbase、稳定币、RWA | 竞争监控、渠道议价、收益迁移判断       |
| `04-platform-option.md`        |  160 | BSA/AML、CPN、Arc        | 平台化验证、合规摩擦、收入转化门槛     |

处置分布：keep 408 条，revise 45 条，defer 21 条。

revise 是改写后保留，白话就是原判断有价值，但必须换成可复核表达。
defer 是延后，白话就是先写成等待数据的检查点，不当成当前结论。

主框架只保留能改变动作的规则。
逐条微实验明细保留在 `../autoresearch/01-iteration-log.md`。

## 分层结论模板

每条研究结论使用固定格式：

```text
结论：CRCL 当前仍按利差股研究，平台化期权未充分财务验证。
依据：Other revenue 占比仍低，RLDC margin 与 USDC 平均流通量仍是主要解释变量。
动作：优先跟踪 Q1 2026 财报中的平均 USDC、reserve return rate、Distribution and transaction costs、Other revenue。
```

reserve return rate 是储备收益率，白话就是储备资产实际赚到的年化收益。

## 优先级定义

| 优先级 | 频率           | 用途                 |
| ------ | -------------- | -------------------- |
| P0     | 每日或事件触发 | 决定是否立即调整风险 |
| P1     | 每周           | 判断趋势是否延续     |
| P2     | 季度或重大事件 | 判断估值框架是否切换 |

P0 不等于一定交易。
P0 的含义是必须当天更新判断。

## 当前基础判断

截至 2026-05-08：

- CRCL 的主利润仍由 USDC 规模和短端利率驱动。
- CPN 与 Arc 已有采用数据，但还没形成足够财务贡献。
- GENIUS Act 提高合规确定性，也限制直接收益分享。
- 监管最终细则会影响 Coinbase、交易所、钱包等分销激励模式。
- Q1 2026 财报是下一个关键校验点，日期为 2026-05-11。

---

## 全局触发阈值索引表

**（新增 2026-05-08）**

以下综合表汇总所有会触发研究或仓位动作的核心阈值，进入框架时用于快速定位当前档位与需要执行的动作。数字均从各子文件精确提取。

### USDC 供应与市占率

| 指标 | 告警阈值（降） | 告警阈值（升） | 触发动作 | 所在文件 |
| ---- | ------------ | ------------ | ------- | ------- |
| USDC 市占率 MoM 变化 | <−1.5ppts / MoM | >+1.5ppts / MoM | 降：P1 告警，同步检查 USDC/USDT ratio；升：增长质量确认信号 | `metrics/00-metric-dictionary.md` |
| USDC 市占率（C-TRIGGER-A 级） | MoM <−3% 连续 2 个月 | — | C-TRIGGER-A：停止加仓，信用层协议，升级至 P0 | `risk/01-warning-signals.md` |
| Net mint / redeem（连续 2 周负） | 连续 2 周为负 | 连续 2 周以上为正 | 降：暂停加仓，进入观察 | `risk/01-warning-signals.md` |
| Net mint / redeem（连续 4 周负） | 连续 4 周且累计降 >2% | — | 降仓位上限 10-15ppts | `risk/01-warning-signals.md` |
| Net mint / redeem（C-TRIGGER-C 级） | 单周 >季末余额 1.5% 连续 2 周 | — | 立即执行信用层协议，不等财报确认 | `risk/01-warning-signals.md` |
| USDC 30D change | 转负（强制复盘触发） | — | 立即执行全框架复盘，不等下一定期周期 | `risk/01-warning-signals.md` |

### 储备收益率

| 指标 | 告警阈值（降） | 告警阈值（升） | 触发动作 | 所在文件 |
| ---- | ------------ | ------------ | ------- | ------- |
| Reserve return rate（绝对水平） | <3.0% | >5.0% | 降：触发收入下修；升：确认降息未传导 | `metrics/00-metric-dictionary.md` |
| Reserve return rate（信号 8 级） | 环比降 >50bps（跌破 3.3%）且 USDC 同比增速 <15% | — | 减仓 −10ppts；重算情景模型年化 RLDC；降息单次 >50bps 时升级 Bear 参数 | `risk/01-warning-signals.md` |
| 3M T-Bill yield | 下行 >50bps（单季收入影响 >2%） | 上行 >25bps（储备收入支撑） | 降：重算利率敏感度矩阵；升：确认储备收入支撑信号 | `metrics/00-metric-dictionary.md` |

### RLDC margin

| 指标 | 告警阈值（降） | 告警阈值（升） | 触发动作 | 所在文件 |
| ---- | ------------ | ------------ | ------- | ------- |
| RLDC margin（主口径 ÷ 储备收入）—唯一告警基准 | <38%（降级检查） | 连续 2 季超 40% | 降：降级检查，减仓 −15ppts，执行全框架复盘；升：加仓 +5ppts | `metrics/00-metric-dictionary.md` / `framework/01-business-model.md` |
| RLDC margin 预警区（D3 竞争评分） | <38%（D3 最高 25 分） | ≥42%（D3 满分 100 分） | 38-40%：D3 打 50 分预警档；40-42%：75 分正常档 | `metrics/05-competition-scoring-rubric.md` |
| RLDC margin Bear 倍数落点 | <35%（Bear 8×，最低） | ≥38%（Bear 12×） | 35-38%：Bear 倍数 10×；<35%：8×；检查信用层失效条件 | `valuation/00-valuation-framework.md` |

### 竞争评分（competition score）

| 指标 | 告警阈值（降） | 告警阈值（升） | 触发动作 | 所在文件 |
| ---- | ------------ | ------------ | ------- | ------- |
| 竞争评分总档位 | <55（预警区）；<40（危险档，减仓 −10-15ppts） | >75（强化档，加仓 +5ppts） | <40：重算增长率、RLDC 和长期倍数；>75：可上调平台化权重 | `metrics/05-competition-scoring-rubric.md` |
| 竞争评分单季变化 | 单季下降 >10 分或跌破 55 | — | 同步触发预警信号 3，升级至 P0 处理 | `risk/01-warning-signals.md` |
| Base 情景倍数落点（由竞争评分决定） | <55 → 15×（区间低端） | >65 → 22×（区间高端）；55-65 → 18× | 每次目标价计算时按当前评分选取倍数落点 | `valuation/00-valuation-framework.md` |

### Other revenue 占比与绝对值

| 指标 | 告警阈值（降） | 告警阈值（升） | 触发动作 | 所在文件 |
| ---- | ------------ | ------------ | ------- | ------- |
| Other revenue 占比（÷总收入，阶段判断） | <5%（利差股阶段，不给平台倍数）；<5% 且 QoQ<5% 连续 2 季（停止平台化溢价） | >15%（强平台化，估值切换条件之一满足） | <5%：按利差股单维定价；5-10%：弱平台化早期期权；10-15%：中平台化开始拆分估值 | `valuation/00-valuation-framework.md` |
| Other revenue 季度绝对值 | <3,000 万（低于 FY2026 季化节奏下沿） | >4,000 万（超 Q4 2025 季度值 0.37 亿） | 降：停止平台化溢价；RLDC 倍数降至 Base 下沿 15× | `risk/01-warning-signals.md` / `framework/01-business-model.md` |
| Other revenue FY2026 H1 累计 | <6,000 万（指引下沿 1.5 亿的 40%） | — | 停止平台化溢价，按利差股单维定价，RLDC 倍数降至 Base 下沿 15× | `risk/01-warning-signals.md` |
| Q1 2026 季度值（告警参考） | <3,000 万 | >4,000 万 | 低于 3,000 万则年化仅约 1.2 亿，低于 FY2026 指引 1.5 亿下沿 | `framework/01-business-model.md` |

### 平台化条件计数与倍数映射

| 指标 | 告警阈值（降） | 告警阈值（升） | 触发动作 | 所在文件 |
| ---- | ------------ | ------------ | ------- | ------- |
| 平台化切换 5 条件（缺任一） | 缺任一项 → 不能切换框架 | 5 项全满足 → 可切换支付基础设施股定价 | 0-1 项：Bull 未验证 25×；弱验证：28×；中验证及以上：35× | `valuation/00-valuation-framework.md` |
| 切换条件①：Other revenue >15% | 未达到 | 超过 15% | 5 条件之一满足 | `valuation/00-valuation-framework.md` |
| 切换条件②：CPN 或 Arc 可重复收入 | 无披露 | 正式财报拆分 | 5 条件之一满足 | `valuation/00-valuation-framework.md` |
| 切换条件③：平台扩张未压低 RLDC margin | RLDC margin 随平台扩张下滑 | margin 稳定或扩张 | 5 条件之一 | `valuation/00-valuation-framework.md` |
| 切换条件④：USDC 市占率稳定或上升 | 市占率持续下降 | 4 周滚动上升 | 5 条件之一 | `valuation/00-valuation-framework.md` |
| 切换条件⑤：监管不限制分销激励 | 监管限制第三方激励 | 明确无限制 | 5 条件之一 | `valuation/00-valuation-framework.md` |

### 预警信号组合触发条件

| 组合 | 触发信号组成（量化条件） | 告警档位 | 触发动作 | 所在文件 |
| ---- | ---------------------- | ------- | ------- | ------- |
| 组合 A | 净赎回 + USDC 市占率下降 + BTC/ETH 同时下跌 | 严重 | 减仓优先，不做技术性摊平；检查 C-TRIGGER 触发 | `risk/01-warning-signals.md` |
| 组合 B | USDC 同比增速 >10%；且 RLDC margin 环比降 >2ppts（如 41%→<39%）；且 Coinbase 持有占比环比升 >3ppts | 严重 | 增长质量偏渠道，保留观察仓，等下季 margin 修复 | `risk/01-warning-signals.md` |
| 组合 C | Other revenue 增长 + CPN TPV 增长 + RLDC margin 稳定（三项同时） | 正向验证 | 平台化开始被财报验证，允许提高长期估值假设 | `risk/01-warning-signals.md` |
| 组合 D | 监管利好事件 + 银行稳定币加速进入企业端 + 估值处于高位 | 警惕高买 | 政策利好被竞争稀释，不追高，等市占率和客户数据确认 | `risk/01-warning-signals.md` |

---

## 当前档位快速判断模板

**（新增 2026-05-08）**

每周填写一次，完成五分钟框架定位。

```text
快速定位（每周填写一次）：

□ RLDC margin 当前值：____%
  → 档位：Bear(<38%) / 预警(38-40%) / Base(40-42%) / Bull(>42%)
  → 仓位动作：<38% 立即降级检查、减仓 −15ppts
             40-42% 维持
             >42% 且连续 2 季 → 加仓 +5ppts

□ 竞争评分（competition score）当前值：____分
  → 档位：强化(>75) / 中性(55-74) / 预警(<55) / 危险(<40)
  → Base 倍数落点：>65分 → 22× / 55-65分 → 18× / <55分 → 15×
  → 仓位动作：>75 加仓 +5ppts / <40 减仓 −10-15ppts、重算倍数

□ USDC 供应变化 本月 MoM：____%
  → 状态：增长 / 横盘 / P1 告警(MoM<-1.5%) / C-TRIGGER-A(-3% 连续 2 月)
  → Net mint/redeem 状态：正值（安全）/ 连续 2 周负（暂停加仓）/
                          连续 4 周负（减仓）/ C-TRIGGER-C（信用层协议）

□ Other revenue 占比：____%（最新季度）
  → 平台化阶段：利差股(<5%) / 弱平台化(5-10%) / 中平台化(10-15%) / 强平台化(>15%)
  → 季度绝对值：____亿（告警下限：<0.30亿；FY2026 H1 累计下限：6,000 万）
  → 告警：占比<5% 且 QoQ<5% 连续 2 季 → 停止平台化溢价

□ 平台化切换条件满足数：____项（满分 5 项）
  → Bull 倍数档位：未验证(0-1项) → 25× / 弱验证 → 28× / 中验证及以上 → 35×
  → 条件逐一确认（√/×）：
    ① Other revenue >15%？___
    ② CPN 或 Arc 有可重复收入财报拆分？___
    ③ 平台扩张期间 RLDC margin 未明显下滑？___
    ④ USDC 市占率稳定或上升？___
    ⑤ 监管未限制核心分销激励？___

□ 储备收益率当前季化推算：____%（来源：USDXX 基金 7 日收益率代理）
  → 状态：健康(>3.6%) / 关注(3.3-3.6%) / 信号 8 告警(<3.3% 且 USDC 增速<15%)
  → 利率动作：<3.3% 且增速不足 → 减仓 −10ppts，重算利率敏感度矩阵

□ 当前情景：Bear / Base / Bull

□ 当前激活预警组合（如有）：
  □ 组合 A（净赎回 + 市占率下降 + BTC 下跌）    → 减仓优先
  □ 组合 B（USDC 增长 + RLDC 下滑 + CB 收入升） → 保留观察仓
  □ 组合 C（Other rev 增 + CPN TPV 增 + RLDC 稳）→ 允许上调长期估值
  □ 组合 D（监管利好 + 银行稳币加速 + 估值高位）→ 不追高

□ 本周需要触发的动作：___________
```

---

## 跨文件联动规则速查

**（新增 2026-05-08）**

| 触发条件 | 优先查阅文件 | 联动文件 | 联动逻辑 |
| ------- | ---------- | ------- | ------- |
| RLDC margin < 38% | `metrics/03-quarterly-earnings.md` | `risk/02-failure-conditions.md` | margin <38% → 决策树节点① → C-TRIGGER 审查；同步检查分销成本率是否持续上升和 Coinbase 协议变化 |
| RLDC margin < 35% | `valuation/00-valuation-framework.md` | `risk/02-failure-conditions.md` | Bear 倍数降至 8×（最低）；检查是否触发信用层失效条件 |
| 竞争评分穿越 40 分（危险档） | `metrics/05-competition-scoring-rubric.md` | `valuation/01-scenario-model.md` | 评分 <40 → Bear 情景触发条件之一；重算增长率和 RLDC 长期假设；Base 倍数降至区间低端 15× |
| 竞争评分穿越 75 分（强化档） | `metrics/05-competition-scoring-rubric.md` | `valuation/00-valuation-framework.md` | 评分 >75 → 加仓 +5ppts；Bull 情景权重上升；Base 倍数选区间高端 22× |
| 竞争评分单季下降 >10 分 | `metrics/05-competition-scoring-rubric.md` | `risk/01-warning-signals.md` | 同步触发预警信号 3，升级至 P0；重新审查 Bear 情景触发条件 |
| USDC 市占率 MoM <−3% 连续 2 月（C-TRIGGER-A） | `risk/01-warning-signals.md` | `risk/02-failure-conditions.md` / `valuation/01-scenario-model.md` | C-TRIGGER-A 触发 → 停止加仓 → 信用层协议 → Bear 情景参数更新 |
| Reserve return rate < 3.3%（信号 8 触发） | `metrics/00-metric-dictionary.md` | `valuation/00-valuation-framework.md` / `framework/01-business-model.md` | 减仓 −10ppts；重算利率敏感度矩阵；更新情景模型储备收益率假设行 |
| Other revenue 占比突破 15% | `framework/04-platform-option.md` | `valuation/00-valuation-framework.md` | 5 项估值切换条件之一满足 → 允许向支付基础设施股框架切换 → Bull 倍数可向 35× 靠拢 |
| Other revenue 占比 <5% 且 QoQ<5% 连续 2 季 | `risk/01-warning-signals.md` | `valuation/00-valuation-framework.md` | 停止平台化溢价 → 按利差股单维定价 → RLDC 倍数降至 Base 下沿 15× |
| Other revenue FY2026 H1 累计 <6,000 万 | `risk/01-warning-signals.md` | `valuation/00-valuation-framework.md` | 同上，停止平台化溢价；追问管理层是否下修 FY2026 指引 |
| 预警组合 B（USDC 增长 + RLDC margin 下滑 + Coinbase 收入上升） | `risk/01-warning-signals.md` | `metrics/03-quarterly-earnings.md` / `framework/01-business-model.md` | 重算渠道分成假设；检查 Coinbase 议价协议变化；暂停上调平台化权重 |
| 预警组合 C（Other rev 增长 + CPN TPV 增长 + RLDC margin 稳定） | `risk/01-warning-signals.md` | `framework/04-platform-option.md` / `valuation/00-valuation-framework.md` | 三项同时成立 → 平台化开始被财报验证 → 允许上调长期估值假设 |
| 监管文件限制第三方激励（预警信号 6） | `framework/02-regulation.md` | `valuation/00-valuation-framework.md` / `framework/01-business-model.md` | 重算 RLDC margin（分销成本率上升 5-10ppts 假设）→ 暂停平台化估值 → 降低 Bull 情景权重 |
| CPN TPV 增速 >20% 但 Other revenue QoQ <5% 连续 2 季（信号 15） | `risk/01-warning-signals.md` | `framework/04-platform-option.md` / `valuation/00-valuation-framework.md` | CPN 评级降为"使用量有证据但商业化未验证"；不上调平台化倍数；按保守口径处理 |
| 预警组合 H（USDe/RWA AUM 增长 + USDC 净赎回 + DeFi 借款走弱） | `risk/01-warning-signals.md` | `valuation/01-scenario-model.md` / `metrics/00-metric-dictionary.md` | 闲置美元系统性流出 → 降低收益竞争评分 → 检查 Bear 情景触发条件 |
