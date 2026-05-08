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
| Velocity                    | 转账量 / 流通量       | 自算                        | 周转增强但不直接等于收入               |
| Exchange balances           | 交易所 USDC 余额      | Nansen / TokenTerminal      | 偏交易 beta，质量低于支付场景          |
| DeFi protocol deposits      | DeFi 协议 USDC 存量   | DefiLlama / Aave / Compound | 判断资金用途                           |

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
