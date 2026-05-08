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
| RLDC margin                        | RLDC / total revenue and reserve income | 自算                      | 低于 38% 触发降级检查       |
| Other revenue                      | 非储备收入                              | SEC filings / 财报        | 占比超过 15% 才强验证平台化 |
| Adjusted EBITDA                    | 调整后 EBITDA                           | SEC filings / 财报        | 经营利润能力                |
| Coinbase distribution exposure     | Coinbase 相关分销依赖                   | Circle / Coinbase filings | 判断渠道议价风险            |
| CPN TPV                            | CPN 总支付量                            | Circle disclosures        | 必须和收入同步验证          |
| Arc mainnet / usage                | Arc 主网与使用量                        | Circle blog / docs        | 测试网不能当收入            |
| Lock-up / Form 4 / offering        | 解禁、内部人、增发                      | SEC EDGAR                 | 筹码压力                    |

Form 4 是内部人交易披露，白话就是高管、董事、大股东买卖股票的申报。

## 计算公式

```text
Reserve income = average USDC circulation × reserve return rate
Net reserve economics = reserve income - distribution and transaction costs
RLDC margin = RLDC / total revenue and reserve income
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
