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
