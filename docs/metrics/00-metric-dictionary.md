# 指标字典

## 结论

CRCL 指标分为 P0、P1、P2 三层。

P0 是每日或事件触发指标，用于判断风险是否需要马上重估。
P1 是每周趋势指标，用于判断增长质量。
P2 是季度或事件指标，用于判断估值框架是否切换。

## P0 指标

| 指标                            | 口径                   | 来源                                   | 判断规则                               |
| ------------------------------- | ---------------------- | -------------------------------------- | -------------------------------------- |
| USDC circulating supply         | USDC 流通量            | Circle Transparency                    | 连续增长强化基础盘，连续净赎回触发风险 |
| 7D / 30D / 365D change          | 不同周期流通量变化     | Circle Transparency                    | 7D 与 30D 同跌，说明赎回压力扩大       |
| Minted / Redeemed               | 发行量与赎回量         | Circle Transparency                    | 赎回连续高于发行是风险信号             |
| Net mint / redeem               | Minted - Redeemed      | Circle Transparency                    | 连续 2-4 周为负，触发减仓检查          |
| 3M T-Bill yield                 | 3 个月美国国债收益率   | Treasury / FRED / Yahoo Finance        | 下行压低储备收入                       |
| SOFR                            | 隔夜融资利率           | NY Fed                                 | 判断短端利率传导                       |
| Circle Reserve Fund 7-day yield | 储备基金 7 日收益率    | BlackRock / fund page                  | 直接影响储备收益                       |
| BTC / ETH price                 | Crypto beta            | CoinGecko / TradingView                | 风险偏好方向                           |
| CRCL price / volume             | 股票价格与成交量       | Yahoo Finance / 交易软件               | 基本面与筹码共振验证                   |
| 监管公告                        | 法案、规则、执法、讲话 | Congress / OCC / Treasury / SEC / CFTC | 改变发行或分销经济性即 P0              |

SOFR 是 Secured Overnight Financing Rate，白话就是美元市场短期资金价格。
Crypto beta 是加密资产联动，白话就是 BTC/ETH 走势会不会带动 CRCL 风险偏好。

## P1 指标

| 指标                        | 口径                  | 来源                        | 判断规则                               |
| --------------------------- | --------------------- | --------------------------- | -------------------------------------- |
| USDC market cap share       | USDC 市占率           | DefiLlama / CoinGecko       | 连续 4 周下降是竞争压力                |
| USDC / USDT ratio           | USDC 市值 / USDT 市值 | DefiLlama / CoinGecko       | 比值上升说明相对竞争改善               |
| Stablecoin total market cap | 稳定币总市值          | DefiLlama                   | 总盘扩张支撑行业 beta                  |
| USDC by chain               | 各链 USDC 供应        | Dune / TokenTerminal        | 判断增长来自机构、DeFi、支付还是交易所 |
| USDC transfer volume        | 转账量                | Dune / CoinMetrics          | 流通量和转账量双增是高质量增长         |
| Adjusted transfer volume    | 清洗后转账量          | CoinMetrics / Dune          | 剔除刷量与自转账                       |
| Active addresses            | 活跃地址              | Dune / Santiment            | 判断真实用户活跃度                     |
| Velocity                    | 转账量 / 流通量       | Circle SEC filing / Dune    | 周转增强但不直接等于收入               |
| Exchange balances           | 交易所 USDC 余额、变化、集中度 | CoinGlass（Open API / 前台 Balance fallback） | 偏交易 beta，质量低于支付场景；30D 流出和 top3 集中度要单独看 |
| DeFi protocol deposits      | DeFi 协议 USDC 存量   | DefiLlama protocol API      | 判断资金用途                           |
| Tokenized Treasury AUM      | 代币化美债总 AUM / BUIDL AUM | RWA.xyz public treasuries page | 判断收益型产品是否分流闲置美元         |

Velocity 是周转率，白话就是同一单位 USDC 一段时间内被用了几次。

## P2 指标

| 指标                               | 口径                                    | 来源                      | 判断规则                    |
| ---------------------------------- | --------------------------------------- | ------------------------- | --------------------------- |
| Reserve income                     | 储备收入                                | SEC filings / 财报        | 当前利润核心                |
| Distribution and transaction costs | 分销与交易成本                          | SEC filings / 财报        | 成本率上升削弱规模收益      |
| RLDC                               | 收入减分销成本                          | SEC filings / 财报        | 更适合 CRCL 的收入质量锚    |
| RLDC margin（主）                  | RLDC / Reserve income                   | 自算                      | 低于 38% 触发降级检查（唯一告警基准） |
| RLDC margin 全口径（辅）           | RLDC / 总收入（含 Other revenue）       | 自算                      | Other revenue share >10% 时并行披露，仅供转型期参考，不触发操作 |
| Other revenue                      | 非储备收入                              | SEC filings / 财报        | 占比超过 15% 才强验证平台化 |
| Adjusted EBITDA                    | 调整后 EBITDA                           | SEC filings / 财报        | 经营利润能力                |
| Coinbase distribution exposure     | Coinbase 相关分销依赖                   | Circle / Coinbase filings | 判断渠道议价风险            |
| CPN TPV                            | CPN 总支付量                            | Circle disclosures        | 必须和收入同步验证          |
| Arc mainnet / usage                | Arc 主网与使用量                        | Circle blog / docs        | 测试网不能当收入            |
| Lock-up / Form 4 / offering        | 解禁、内部人、增发                      | SEC EDGAR                 | 筹码压力。**仅影响仓位操作，不参与情景判断或估值切换触发。** |

Form 4 是内部人交易披露，白话就是高管、董事、大股东买卖股票的申报。

## 计算公式

```text
Reserve income = average USDC circulation × reserve return rate
Net reserve economics = reserve income - distribution and transaction costs
RLDC margin（主） = RLDC / Reserve income          # 唯一告警基准，38% 阈值适用于此口径
RLDC margin 全口径（辅） = RLDC / 总收入          # Other revenue >10% 时并行计算，不触发告警
Other revenue share = other revenue / total revenue and reserve income
Market cap / USDC circulation = CRCL market cap / USDC circulating supply
Market cap / RLDC = CRCL market cap / annualized RLDC
Growth quality = supply growth + transfer volume growth + market share change
```

average USDC circulation 是平均 USDC 流通量，白话就是一段时间内平均有多少 USDC 在外流通。

## 数据源优先级

1. SEC filing。
2. Circle 官方披露。
3. 美国监管机构文件。
4. 交易所或链上浏览器原始数据。
5. DefiLlama、CoinGecko、Dune 等二级平台。
6. 媒体报道。

二级平台数据只能作为监控线索。
季度结论必须回到 SEC filing 和公司披露。

## 信号优先级与 P0/P1/P2 的关系

本框架有两套优先级标记，用途不同，互不替代。

### P0/P1/P2 分层：更新频率标记

| 层级 | 含义 | 操作 |
| ---- | ---- | ---- |
| P0 | 每日或事件触发监控 | 触发后先处理风险，不等财报确认 |
| P1 | 每周趋势监控 | 用于判断增长质量和竞争位置 |
| P2 | 季度或事件监控 | 用于更新估值模型和中长期判断 |

### 信号优先级序列：触发顺序标记

当多个信号同时出现时，按以下顺序处理：

```text
储备异常
  > 赎回压力
  > 监管限制收益分享或第三方激励
  > 分销成本率持续恶化
  > 市占率下降
  > 利率下行未被规模增长抵消
  > 股价破位（最低优先级）
```

前四类是**基本面风险**，后三类是**估值和交易风险**。

规则：

- 基本面风险触发时，技术反弹不改变结论。
- 股价信号只能确认风险，不能成为降级的独立依据。
- P0 分层指标并不等于信号优先级最高：例如 `CRCL price/volume` 是 P0 频率监控（每日盯），但在信号优先级序列中排最末。

### 综合评分参考（辅助判断，非主决策）

仅在需要汇总多信号判断时使用，不替代矩阵共振触发。

```text
P0 指标每项 ±3 分（利好 +3，利空 -3，不确定 0）
P1 指标每项 ±2 分
P2 指标每项 ±1 分
股价结构指标每项 ±1 分

总分 ≥ +6：多数信号正向，允许维持或加仓
总分 -5 至 +5：中性，按估值和事件风险调整仓位
总分 ≤ -6：多数信号负向，暂停加仓并检查减仓条件
```

综合评分不触发情景切换。情景切换必须满足 `valuation/01-scenario-model.md` 的矩阵共振条件。

---

## 新增（2026-05-08）

## P0/P1 指标自动告警条件表

### P0 指标告警条件（10 项）

| 指标 | 运行时值 / 历史基准 | 告警阈值（升） | 告警阈值（降） | 数据源 | 刷新频率 |
| ---- | --------- | ------------ | ------------ | ------ | -------- |
| USDC circulating supply | Q4 2025 末 ~753 亿美元 | 7D+30D 同时增长；突破 800 亿 | 连续 2 周净赎回为负；7D+30D 同跌 | Circle Transparency | 每日 |
| 7D / 30D / 365D change | 参照 Q4 2025 末水平 | 7D 与 30D 同向上升 | 7D 与 30D 同时下降 | Circle Transparency | 每日 |
| Minted / Redeemed | 季度净发行为正 | 发行量连续高于赎回量 | 赎回连续 2-4 周高于发行 | Circle Transparency | 每日 |
| Net mint / redeem | 参照近 4 周均值 | 连续 2 周以上为正值 | 连续 2-4 周为负 → 触发减仓检查 | Circle Transparency | 每日 |
| 3M T-Bill yield | ~4.2-4.5%（Q4 2025 水平） | 上行 > 25 bps（储备收入支撑） | 下行 > 50 bps（单季收入影响 > 2%） | Treasury / FRED | 每日 |
| SOFR | 参照 3M T-Bill 方向 | 同向上行 | 30 日均值低于 3.5% | NY Fed | 每日 |
| Circle Reserve Fund 7-day yield | ~4.0-4.5%（Q4 2025 水平） | > 5.0% | < 3.0% | BlackRock fund page | 每日 |
| BTC / ETH price | 参照近期均价 | 双资产同时大涨（风险偏好强） | 双资产同时大跌 > 10% | CoinGecko / TradingView | 每日 |
| CRCL price / volume | 参照近期均价 | 放量突破关键阻力位 | 放量跌破关键支撑位 | Yahoo Finance / 交易软件 | 每日 |
| 监管公告 | 无新规则生效 | 非银行路径清晰；收益分享不受限 | 第三方激励被认定为变相 yield | Congress / OCC / Treasury | 事件触发 |

### P1 指标告警条件（10 项）

| 指标 | 运行时值 / 历史基准 | 告警阈值（升） | 告警阈值（降） | 数据源 | 刷新频率 |
| ---- | --------- | ------------ | ------------ | ------ | -------- |
| USDC market cap share | ~25-27%（Q4 2025 估） | 月度变化 > +1.5 ppts | 月度变化 > -1.5 ppts 或连续 4 周下降 | DefiLlama / CoinGecko | 每周 |
| USDC / USDT ratio | ~0.25-0.30（Q4 2025 估） | 比值连续上升 4 周 | 比值连续下降 4 周 | DefiLlama / CoinGecko | 每周 |
| Stablecoin total market cap | 参照季末总盘规模 | 总市值创历史新高 | 连续 4 周萎缩 > 5% | DefiLlama | 每周 |
| USDC by chain | 参照各链季末余额 | Base / Ethereum / Solana 同增 | 任意主链连续 4 周下降 > 5% | Dune / TokenTerminal | 每周 |
| USDC transfer volume | 参照近 4 周均值 | 量价同增（流通量 + 转账量双升） | 转账量连续 4 周下降 | Dune / CoinMetrics | 每周 |
| Adjusted transfer volume | 参照清洗后近 4 周均值 | 清洗后量增且活跃地址同增 | 清洗后量连续下降 | CoinMetrics / Dune | 每周 |
| Active addresses | 参照季末活跃地址数 | 连续 4 周创新高 | 连续 4 周下降 > 10% | Dune / Santiment | 每周 |
| Velocity | 参照近期均值 | 周转率提升且活跃地址同增 | 周转率持续下降 | Circle SEC filing / Dune | 每周 |
| Exchange balances | 参照季末交易所余额 | 交易所余额大幅增加（交易 beta 强） | 交易所余额 30D 持续流出 > 10%，或 top3 集中度异常上升 | CoinGlass | 每周 |
| DeFi protocol deposits | 参照季末 DeFi 存量 | 存量大幅增加（Aave / Compound） | 连续 4 周净流出 | DefiLlama protocol API | 每周 |

### 补充：框架关键临界值（衍生指标）

以下临界值来自框架多处引用，不在上表 P0/P1 分层，但优先级等同 P1 告警：

| 衍生指标 | 告警条件 | 数据来源 | 触发动作 |
| -------- | -------- | -------- | -------- |
| Reserve return rate（年化，÷ Average USDC） | < 3.0% 或 > 5.0% | SEC 10-Q 自算 | < 3.0%：触发收入下修；> 5.0%：确认降息未传导 |
| RLDC margin（主） | < 38% | SEC 10-Q 自算 | 已有；见 metrics/03 决策树；减仓 15 ppts |
| Competition score（来自 metrics/04） | 穿越 75 或 40 分边界 | metrics/04-competition-dashboard.md | > 75：加仓 +5 ppts；< 40：减仓 10-15 ppts |

---

## 扩展指标（新增 3 项）

### 指标1：合规成本比率

| 字段 | 内容 |
| ---- | ---- |
| 定义 | 合规及风控成本 ÷ 储备总收入（Reserve income） |
| 计算 | SEC 10-Q 运营费用中 compliance / legal 相关科目之和 ÷ Reserve income |
| 数据来源 | SEC 10-Q 运营费用明细（compliance、legal、regulatory affairs 相关科目） |
| 告警阈值 | > 2%：触发分销经济性重算（与 RLDC margin 联动）；环比上升 > 0.5 ppts：标记监管成本压力 |
| 更新频率 | 季度（财报发布后 T+24h 完成） |
| 优先级 | P2 |
| 与框架联动 | 告警时更新 framework/02-regulation.md 合规成本折价项；重新检查 RLDC margin 是否受成本推高影响 |

### 指标2：链上技术可用性

| 字段 | 内容 |
| ---- | ---- |
| 定义 | USDC 主要部署链（Base、Ethereum、Solana）的在线状态与故障时长 |
| 计算 | 各链官方 status 页面故障事件记录；以"连续故障小时数"为量化单位 |
| 数据来源 | Base Status（status.base.org）、Solana Status（status.solana.com）、Circle 博客公告 |
| 告警阈值 | 任一主链故障 > 4 小时：触发流动性风险评估；Base 故障 > 2 小时：额外触发 CPN 中断检查 |
| 更新频率 | P0（事件触发，实时监控） |
| 优先级 | P0 |
| 与框架联动 | 触发 risk/00-risk-map.md 技术风险项；故障超 24 小时需更新 framework/04-platform-option.md Arc 可用性记录 |

### 指标3：Coinbase 渠道集中度

| 字段 | 内容 |
| ---- | ---- |
| 定义 | Coinbase 平台 USDC 余额 ÷ USDC 总流通量（估算值） |
| 计算 | Coinbase 10-Q 披露 "USDC on platform" ÷ Circle 透明度报告 USDC 总流通量 |
| 数据来源 | Coinbase 10-Q（USDC on platform 科目）+ Circle 月度透明度报告 |
| 告警阈值 | > 35%：触发渠道集中度预警（分销依赖风险上升）；< 20%：触发 Base 增长质量复核（Coinbase 以外增长是否可持续） |
| 更新频率 | P1（季度精确值，月度估算值） |
| 优先级 | P1 |
| 与框架联动 | 超过 35% 时更新 valuation/00-valuation-framework.md 渠道折价项；结合 risk/00-risk-map.md 渠道依赖风险 |

---

## 指标版本记录

### v1.1 — 2026-05-08 更新

| 变更类型 | 内容 |
| -------- | ---- |
| 新增章节 | P0/P1 指标自动告警条件表（P0 10 项 + P1 10 项 + 3 项框架关键临界值） |
| 新增章节 | 扩展指标：合规成本比率（P2）、链上技术可用性（P0）、Coinbase 渠道集中度（P1） |
| 补充依据 | framework/02-regulation.md 合规成本背景；risk/00-risk-map.md 技术风险与渠道风险 |
| 重点告警 | USDC 市占率月度变化 > ±1.5% 告警；储备收益率 < 3.0% 或 > 5.0% 告警；竞争评分穿越 75/40 告警（来自 metrics/04） |
| 指标总计 | P0：10 项（原有）+ 1 项扩展（链上技术可用性）= 11 项；P1：10 项（原有）+ 1 项扩展（Coinbase 渠道集中度）= 11 项；P2：11 项（原有）+ 1 项扩展（合规成本比率）= 12 项 |

自检提示：下次季度自检时，对照本版本记录确认各告警阈值是否需要随市场环境调整。
