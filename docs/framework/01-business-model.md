# 商业模式与财务桥

## 结论

CRCL 当前主要是 USDC 储备经济性公司，不是已经被验证的平台型支付公司。

核心财务桥：

```text
平均 USDC 流通量
  × 储备收益率
  = Reserve income
  - Distribution and transaction costs
  = RLDC
  + Other revenue
  - Operating expenses
  = Adjusted EBITDA / operating result
```

Reserve income 是储备收入，白话就是用户存美元换 USDC 后，Circle 把储备放在现金、短债、回购和基金里赚到的利息。
Distribution and transaction costs 是分销与交易成本，白话就是给 Coinbase、交易所、钱包、合作渠道的分成和交易相关成本。

## 业务四步

1. 用户存入美元，获得 1:1 USDC。
2. Circle 以现金、短债、回购、货币市场基金等方式持有储备。
3. 储备产生利息，形成 Reserve income。
4. Circle 向分销渠道支付成本，剩余经济性进入 RLDC。

这条链路决定 CRCL 的基础盘。
平台化收入决定的是估值上限，不决定当前利润底盘。

## FY2025 核心事实

| 指标                               |        FY2025 | 判断                         |
| ---------------------------------- | ------------: | ---------------------------- |
| Total revenue and reserve income   | 27.466 亿美元 | 收入总盘继续扩张             |
| Reserve income                     | 26.368 亿美元 | 核心仍是储备收入             |
| Other revenue                      |  1.098 亿美元 | 占比约 4.0%，平台化还早      |
| Distribution and transaction costs | 16.615 亿美元 | 成本增长高于收入质量必须跟踪 |
| RLDC                               | 10.830 亿美元 | RLDC margin（÷储备收入）约 41%，全口径（÷总收入）约 39% |
| Adjusted EBITDA                    |  5.822 亿美元 | 当前盈利能力已验证           |

Other revenue 是非储备收入，白话就是不是靠储备利息赚的钱。
Adjusted EBITDA 是调整后 EBITDA，白话就是剔除部分非现金和一次性项目后的经营利润近似值。

## Q4 2025 核心事实

| 指标                |      Q4 2025 | 判断               |
| ------------------- | -----------: | ------------------ |
| 期末 USDC 流通量    |   753 亿美元 | 同比增长 72%       |
| 平均 USDC 流通量    |   762 亿美元 | 同比增长 100%      |
| Reserve income      |  7.33 亿美元 | 同比增长 69%       |
| Other revenue       | 3,700 万美元 | 平台化仍处早期     |
| RLDC                |  3.09 亿美元 | RLDC margin（÷储备收入）约 42%，全口径约 40% |
| Reserve return rate |         3.8% | 同比下降 68 bps    |

bps 是 basis points，白话就是 1 个百分点的百分之一。
68 bps 等于 0.68 个百分点。

## 利率敏感度

粗算公式：

```text
每 100 亿美元平均 USDC × 每 100 bps 储备收益率
= 约 1 亿美元年化 gross reserve income
```

gross reserve income 是毛储备收入，白话就是还没扣渠道分成的钱。

按 FY2025 RLDC margin 约 41%（÷储备收入主口径）粗算：

```text
每 100 亿美元平均 USDC × 每 100 bps 储备收益率
= 约 3,900 万美元年化 RLDC
```

这是框架估算，不是精确模型。
实际结果会受储备资产结构、渠道分成、USDC on Platform 占比、交易成本影响。

## 关键驱动

| 驱动             | 正向信号             | 负向信号                    |
| ---------------- | -------------------- | --------------------------- |
| 平均 USDC 流通量 | 30D/90D 持续增长     | 连续 2-4 周净赎回           |
| 储备收益率       | 短端利率稳定         | 降息且规模不增长            |
| RLDC margin（主，÷储备收入） | 稳定在 40-42% 或扩张 | 低于 38% 且无解释（告警基准为主口径）|
| Other revenue    | 连续超指引增长       | 只靠一次性服务收入          |
| 分销成本率       | 随规模下降           | Coinbase / Binance 议价增强 |

USDC on Platform 是平台上 USDC 余额，白话就是 Circle 或合作平台上留存的 USDC。

## Autoresearch 落地规则

商业模式 52 次微实验落成一个规则：任何财务结论必须同时过五把尺子。

| 复核尺子        | 用途                         | 不能替代的证据             |
| --------------- | ---------------------------- | -------------------------- |
| 财报数值        | 确认收入、成本、利润基数     | SEC filing 注释            |
| 同比与环比      | 区分趋势改善和季节性波动     | 管理层解释                 |
| 管理层解释      | 判断一次性因素和费用归因     | 财报数值                   |
| SEC filing 注释 | 核对收入确认、成本分类、协议 | 季度阈值                   |
| 季度阈值        | 触发强化、降级或观察动作     | 事实来源                   |

SEC filing 是 SEC 披露文件，白话就是公司正式交给监管机构的文件。

### 十项财务复核

| 复核项                    | 核心问题                         | 动作规则                         |
| ------------------------- | -------------------------------- | -------------------------------- |
| Reserve income 依赖度     | 利润是否仍主要来自储备利息       | 高依赖时继续按利差股主线研究     |
| RLDC margin 稳定性        | 渠道分成是否吞噬规模收益         | 低于 38% 且无解释时触发降级检查  |
| Other revenue 可重复性    | 平台收入是否可持续               | 未拆分时不上调长期倍数           |
| average USDC 解释力       | 平均余额是否解释收入变化         | 优先级高于期末 USDC              |
| distribution cost 弹性    | 分销成本是否随规模下降           | 上升时追问渠道议价和一次性因素   |
| Adjusted EBITDA 转化      | RLDC 是否转成经营利润            | 只作辅助锚，不替代 RLDC          |
| 收入确认口径              | Other revenue 是否一次性确认     | 不清楚时按保守口径处理           |
| 成本一次性因素            | 成本异常是否可复现               | 缺少解释时列入 missing_info      |
| FY 指引质量               | 管理层是否上修、维持或下修预期   | 比单季新闻优先级更高             |
| 财报披露完整性            | 关键口径是否足够复盘             | 披露不足时只给观察结论           |

missing_info 是缺失信息，白话就是不能脑补、必须等数据确认的部分。

## 财务结论分级

| 结论等级 | 条件                                              | 含义                   |
| -------- | ------------------------------------------------- | ---------------------- |
| 强化     | USDC 增长、RLDC margin 稳定、Other revenue 超指引 | 基础盘和期权盘同步增强 |
| 中性     | USDC 增长抵消利率下行，Other revenue 按指引       | 仍按利差股主线定价     |
| 降级     | USDC 放缓、RLDC margin 下滑、Other revenue 不增长 | 平台化叙事降级         |
| 失效     | 赎回信任受损或储备异常                            | 停止估值扩张讨论       |

## 关键假设的局限性说明

框架中有两项被标注为"保守假设"的数字，实际上无法通过公开披露独立验证，需要区别对待。

### Coinbase 占比（约 25%）

Coinbase Q1 2026 披露，Coinbase 产品中平均持有超过 190 亿美元 USDC，占 USDC 总流通量约 25% 以上。

**数据来源**：Coinbase 财报（10-Q），而非 Circle 主动披露分渠道数据。
**可验证程度**：每季度 Coinbase 财报后可更新，但 Circle 不单独拆分按渠道的 USDC 余额。
**使用规则**：
- 此数字用于判断渠道集中度方向，不用于精确计算 RLDC margin 拆分。
- Coinbase 财报每季度更新后，重新确认此比例是否变化。
- 如果 Coinbase 持有比例上升且 RLDC margin 下滑，强化渠道议价风险判断。
- 不能因为 25% 是"保守"就假设实际比例更低，它可能更高（Circle 未拆分）。

### BlackRock USDXX 储备结构

Circle Reserve Fund（USDXX）是储备资产的主要载体，BlackRock 管理，日常 7 日收益率可从 BlackRock 基金页面获取。

**已知的不确定性**：
- 基金的具体资产组成（Treasury 比例、回购协议比例、现金比例）随货币政策和流动性需求变化。
- WAM（加权平均久期）和 WAL（加权平均寿命）月度披露，但非实时，赎回压力时期代表性可能滞后。
- 货币政策转向时，基金经理可能调整久期，影响储备收益率对利率变化的敏感度。

**使用规则**：
- 储备收益率假设不能只用 3M T-Bill 作为代理，应同时参考 USDXX 基金页面实际收益率。
- 快速降息时，基金久期调整可能使储备收益率下行滞后或提前，不能线性外推。
- 储备结构假设发生变化的信号：BlackRock 基金报告中 WAM 明显拉长，或现金比例大幅下降。

## 跨文件术语标准化

框架中多处使用相近术语，以下为唯一定义，使用时以本表为准。

### 财务术语

| 术语 | 唯一定义 | 常见混用形式 | 澄清 |
| ---- | -------- | ------------ | ---- |
| **RLDC** | Revenue Less Distribution Costs = Total revenue and reserve income − Distribution and transaction costs | "净收入"、"毛利润"、"Revenue after costs" | RLDC 不是毛利润，也不是净利润，是扣分销后的收入口径 |
| **RLDC margin（主口径）** | RLDC ÷ Reserve income | "利润率"、"RLDC/Total revenue" | 唯一告警基准，38% 阈值基于此口径 |
| **RLDC margin（全口径）** | RLDC ÷ Total revenue and reserve income | "Revenue margin" | Other revenue >10% 时作辅助参考，不替代主口径 |
| **Reserve income** | 储备资产产生的利息收入（含现金、短债、回购、MMF） | "储备利息"、"reserve earnings" | 不包含 Other revenue |
| **Other revenue** | 非储备收入（CPN、Arc、企业 API、integration services） | "平台收入"、"non-reserve revenue" | 必须区分可重复部分和一次性 integration services |
| **Average USDC in circulation** | 季度内日均 USDC 流通量 | "USDC supply"、"期末 USDC" | 优先于期末值用于解释 Reserve income |
| **Distribution and transaction costs** | 给分销渠道的分成 + 交易相关成本 | "分销费"、"渠道成本" | 包含 Coinbase 等所有渠道成本 |

### 评分与分级术语

| 术语 | 唯一定义 | 常见混用形式 | 澄清 |
| ---- | -------- | ------------ | ---- |
| **P0/P1/P2（更新频率）** | P0=日更，P1=周更，P2=季更/事件更新 | "优先级 P0"、"P0 风险" | P0/P1/P2 在本框架中表示更新频率，不表示信号严重程度 |
| **信号优先级** | 储备异常 > 赎回 > 监管 > 分销 > 市占率 > 利率 > 股价 | "P0 信号"、"关键信号" | 用"信号优先级"描述严重程度，不用 P0/P1/P2 |
| **competition_score** | 0-100 分，7 维度加权，每周更新 | "竞争评分"、"竞争得分" | competition_score 不触发情景切换，只影响仓位因子 |
| **credit_score** | 0-100 分，4 维度，独立于 competition_score | "信用评分" | 低于 40 提高 C-TRIGGER 关注频率，低于 20 直接执行清仓 |
| **C-TRIGGER-A/B/C** | 三个独立信用层触发条件，优先于情景判断 | "信用触发"、"赎回触发" | 触发后立即执行，不等矩阵共振 |

### 情景术语

| 术语 | 唯一定义 | 澄清 |
| ---- | -------- | ---- |
| **情景切换** | Bull ↔ Base ↔ Bear 的转变，需要多矩阵共振 | 单一指标不触发情景切换 |
| **仓位调整** | 在情景仓位上限内根据加减因子调整持仓比例 | 仓位调整不等于情景切换 |
| **框架降级** | 估值倍数或核心假设下修，但情景不一定切换 | RLDC margin <38% 触发降级检查，不直接触发切换 |
| **信用层失效** | C-TRIGGER 触发后，信用层进入协议模式 | 信用层失效后，情景框架结论暂时搁置 |

## 宏观敏感度矩阵

CRCL 的核心盈利依赖短端美元利率和 USDC 规模，宏观敏感度分析必须同时考虑两个变量的交叉影响。

### 利率 × USDC 规模交叉矩阵

| | USDC 增长加速（+20%+ YoY） | USDC 增长平稳（+5-15% YoY） | USDC 增长停滞（+0-5% YoY）或下降 |
|---|---|---|---|
| **利率上升（+50 bps+）** | 双重利好：规模 × 利率共振，Reserve income 大幅增长，RLDC 扩张 | 利率弥补规模放缓，总体 Reserve income 平稳，RLDC margin 受分销成本压力 | 利率上升仅部分弥补规模损失，净效应取决于弹性比较 |
| **利率持平（±25 bps）** | 规模驱动增长，Base case 核心情景 | 维持现状，RLDC 基本稳定，情景不变 | 规模停滞压制 Reserve income，RLDC 下行压力 |
| **利率下降（-50 bps+）** | 规模增长需>20% 才能抵消 50 bps 下行 | 利率 × 规模双向压力，RLDC 收缩，估值倍数收窄 | 双重利空：规模 + 利率同时走弱，Bear case 核心情景 |

### 利率敏感度定量参照

```text
每 100 亿美元平均 USDC × 每 100 bps 储备收益率
= 约 1 亿美元年化 gross reserve income
= 约 3,900 万美元年化 RLDC（按 FY2025 RLDC margin 约 41%）

降息 25 bps 对 FY2026 RLDC 的冲击（Base case：850 亿美元 USDC）：
  = 850 亿 × 0.25% × 41% ≈ 8,700 万美元/年
  = 约 2,200 万美元/季

弥补方案（粗算）：
  需新增 USDC 约 = 8,700 万 ÷ (3.6% × 41%) ≈ 590 亿美元
  即：降息 25 bps，若 USDC 不能增长约 590 亿（约 +70%），Reserve income 净减少
  实际含义：降息环境下，USDC 规模增长是关键补偿变量
```

### 宏观信号优先级

| 宏观信号 | 对 CRCL 的影响方向 | 观察频率 | 来源 |
| -------- | ------------------ | -------- | ---- |
| 联储利率决议 | 直接影响储备收益率假设 | 每次 FOMC（约 6-8 周） | 美联储官网 |
| 3M T-Bill 收益率 | 储备收益率代理指标 | 每周 | U.S. Treasury |
| SOFR 走势 | 短端流动性成本基准 | 每日 | NY Fed |
| 美元 DXY 指数 | 离岸美元需求代理，影响 USDT/USDC 市外需求 | 每周 | 交易所 |
| 加密总市值 / BTC 价格 | Crypto beta，影响稳定币总盘需求 | 每周 | CoinGecko |

## Q1 2026 财报优先检查

2026-05-11 的 Q1 2026 财报先看五项：

1. 平均 USDC 流通量，而不是只看期末值。
2. Reserve return rate，确认降息预期是否已经传导。
3. Distribution and transaction costs，计算分销成本率。
4. RLDC margin，低于 38% 触发降级检查。
5. Other revenue，确认是否接近 FY2026 指引节奏。

如果 Other revenue 增长但 RLDC margin 下滑，结论不是直接利好。
这代表平台化收入必须足够大，才能抵消渠道拿走的经济性。

---

## 历史季度数据基准表

**（新增 2026-05-08）**

以下数据用于季报对比和关键指标趋势跟踪。FY2025 全年与 Q4 2025 数据来源于本文件现有财务数据，FY2024 及 FY2025 部分推算指标无文件明确记载，标注"待从 SEC filing 补充"。

| 季度 | USDC 均量（亿$） | 储备收益率 | RLDC margin（÷储备收入） | Other rev（亿$） | Adj. EBITDA（亿$） | 情景档位 |
| ---- | --------------- | --------- | ----------------------- | --------------- | ----------------- | ------- |
| FY2024 全年 | 待从 SEC filing 补充 | 待从 SEC filing 补充 | 待从 SEC filing 补充 | 待从 SEC filing 补充 | 待从 SEC filing 补充 | — |
| FY2025 全年 | 待从 SEC filing 补充（依据 Reserve income 26.368 亿反算，预计约 570-580 亿） | 待从 SEC filing 补充（推算约 4.3-4.6%） | ~41%（10.830 ÷ 26.368） | 1.098 | 5.822 | Base |
| Q4 2025 | 762 | 3.8% | ~42%（3.09 ÷ 7.33） | 0.37 | 待从 SEC filing 补充 | Base |
| Q1 2026E | ~790-820（推算） | ~3.6%（预估，见外推说明） | >40% 目标（Base 约 41%） | 0.35-0.40 | ~1.3-1.6（推算） | Base |

**数据说明：**

- **FY2025 USDC 均量**：文件中未直接披露全年加权平均流通量。依据 Reserve income（26.368 亿）除以推算全年储备收益率（~4.5%），反算约 585 亿；需从 10-K 中 "Weighted-average USDC in circulation" 一行确认。
- **FY2025 储备收益率**：Q4 2025 实际为 3.8%，全年利率环境高于 Q4（2025 年 Q1-Q3 短端利率维持高位），推算全年约 4.3-4.6%。精确值待 10-K 确认。
- **Q4 2025 Adj. EBITDA**：文件中未单独拆分单季 EBITDA，FY2025 全年为 5.822 亿，Q4 单季需从 SEC 10-Q 提取。
- **Q1 2026E**：基于 Q4 2025 数据与 FY2026 Base 情景参数（年均 USDC 850 亿）合理外推，见下方"Q1 2026 财报预期基准"章节。

---

## RLDC margin 驱动因素分解路径

**（新增 2026-05-08）**

"如果 RLDC margin 下滑，先查什么"的决策链。分析优先顺序：外部利率因素 → 成本结构 → 规模质量 → 平台化补偿。

```text
RLDC margin 下滑
│
├─ 【分支 1】储备收益率下降？（外部因素，Circle 不可控）
│   → 查：3M T-Bill / SOFR 走势 + USDXX 基金 7 日收益率
│   → 检查文件：metrics/00-metric-dictionary.md（P0：3M T-Bill yield、SOFR）
│               valuation/00-valuation-framework.md（利率敏感度矩阵）
│   → 关键指标：Reserve return rate 季度值；USDXX 7 日收益率代理
│   → 判断标准：环比降 >50bps 且 USDC 增速 <15% → 外部利率驱动
│               粗算：850 亿 × 50bps × 41% ≈ 1.7 亿/年 RLDC 损失
│               需 USDC 量增 ~13% 才能完全弥补（参见利率敏感度矩阵）
│
├─ 【分支 2】分销成本率上升？（成本结构，需追溯渠道协议）
│   → 查：Distribution costs ÷ Reserve income 季度比率 + Coinbase 财报
│   → 检查文件：metrics/03-quarterly-earnings.md（成本议价层 / 第 4 层）
│               risk/01-warning-signals.md（信号 7：Coinbase 议价权增强）
│   → 关键指标：Coinbase 持有 USDC 占总流通量比例（来自 Coinbase 10-Q）
│               分销成本率季度环比变化
│   → 判断标准：Coinbase 持有占比环比升 >5ppts（超 30%）且 RLDC margin
│               同比降 >2ppts → 渠道议价博弈，下修长期 RLDC margin 假设
│               基准值：FY2025 分销成本率 ~63%（16.615 ÷ 26.368）
│
├─ 【分支 3】USDC 规模增长质量低？（渠道内循环，非外部真实增长）
│   → 查：Base 链 USDC 占比 vs 全网总市占率变化
│   → 检查文件：metrics/02-weekly-review.md（链生态结构层 / 第 7 层）
│               metrics/05-competition-scoring-rubric.md（D5 链生态健康）
│   → 关键指标：Base 链 USDC 30D 增速；USDC 总市占率 MoM 变化
│   → 判断标准：Base 30D 增速 >20% 但总市占率 MoM <+0.5ppts →
│               增长来自 Coinbase 生态内循环，非外部新需求，不支撑估值上修
│
└─ 【分支 4】Other revenue 不足以弥补？（平台化补偿不够）
    → 查：CPN TPV 增速 + Other revenue 季度同比增速
    → 检查文件：framework/04-platform-option.md（平台化期权层 / 第 6 层）
                risk/01-warning-signals.md（信号 15：CPN TPV 增但 Other rev 不增）
    → 关键指标：Other revenue 季度值；CPN TPV 年化增速（基准：2026-02-20 年化 57 亿）
    → 判断标准：CPN TPV 增速 >20% 但 Other revenue QoQ <5% 连续 2 季 →
                平台化商业化未兑现；CPN 评级降为"使用量有证据但收入未验证"

结论规则：
  ① 以上四个分支不互斥，可同时存在。
  ② 单季 RLDC margin 下滑先听管理层解释再判断原因归属。
  ③ 连续 2 季未修复且无合理解释 → 执行降级检查，减仓 −15ppts。
```

---

## Q1 2026 财报预期基准（2026-05-11）

**（新增 2026-05-08）**

以下为 2026-05-11 Q1 2026 财报当天直接对比使用的预期基准，以 FY2025 全年和 Q4 2025 数据为基础合理外推。

| 科目 | Base 预期 | 告警下限 | 触发动作 | 数据来源 |
| ---- | -------- | ------- | ------- | ------- |
| 平均 USDC 流通量（亿$） | ~790-820 | <760（低于 Q4 2025 期末 753 亿，季度不增长） | 检查净赎回持续性，触发预警信号 2 观察 | Circle Transparency / SEC 10-Q |
| 储备收益率（Reserve return rate） | ~3.6% | <3.3% | 信号 8 告警；减仓 −10ppts；重算利率敏感度矩阵 | SEC 10-Q 自算 / USDXX 基金页 |
| Reserve income（亿$） | ~7.1-7.4 | <6.5 | 规模与利率双降；触发 Bear 情景参数复核 | SEC 10-Q |
| Distribution and transaction costs（亿$） | ~4.4-4.7 | >4.9（分销成本率超 FY2025 基准约 63%） | 检查 Coinbase 协议变化；触发预警信号 4 观察 | SEC 10-Q |
| RLDC（亿$） | ~2.9-3.1 | <2.6（对应 RLDC margin <38% 于约 7 亿储备收入基础上） | 降级检查触发；减仓 −15ppts；执行全框架复盘 | SEC 10-Q 自算 |
| RLDC margin（÷储备收入，主口径） | >40%（目标 41%） | <38% | 唯一告警基准；触发降级检查和减仓操作 | SEC 10-Q 自算 |
| Other revenue（亿$） | 0.35-0.40 | <0.30（年化 <1.2 亿，低于 FY2026 指引下沿 1.5 亿） | 停止平台化估值溢价；按利差股基准定价；追问管理层是否下修全年指引 | SEC 10-Q |
| Adjusted EBITDA（亿$） | ~1.3-1.6 | <1.1（同比下滑，需追问成本结构） | 辅助参考，不直接触发仓位操作，需追问管理层一次性因素 | SEC 10-Q |

**外推逻辑说明：**

1. **储备收益率 ~3.6%**：Q4 2025 实际值 3.8%，当前利率环境（3M T-Bill ~4.2-4.5%，USDXX 与 T-Bill spread 约 50bp）下继续小幅下行，外推约 3.6%。
2. **平均 USDC ~800 亿**：Q4 2025 期末 753 亿、期内均值 762 亿；FY2026 Base 情景年均 850 亿。Q1 起始偏低，合理区间 790-820 亿。
3. **RLDC margin >40%**：Q4 2025 实际约 42%，FY2025 全年约 41%；Other revenue 季度小幅波动，目标维持 40-42% 正常区间。
4. **Other revenue 0.35-0.40 亿**：Q4 2025 为 0.37 亿；FY2026 全年指引 1.5-1.7 亿，季化节奏 0.375-0.425 亿，基本持平或略升。
5. **告警下限来源**：各行告警阈值均对应 `risk/01-warning-signals.md` 量化规格与 `metrics/00-metric-dictionary.md` 告警条件表，不单独设新规则。
