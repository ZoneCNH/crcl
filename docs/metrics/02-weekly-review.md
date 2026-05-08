# 每周复盘

## 结论

每周复盘用于判断增长质量，不用于复述价格。

核心问题：

```text
USDC 是否增长？
增长来自哪里？
增长是否带来更高使用强度？
增长是否被竞争对手或渠道成本吞噬？
竞争结构是否强化？
```

## 每周指标

| 模块       | 指标                                                      | 判断                       |
| ---------- | --------------------------------------------------------- | -------------------------- |
| 稳定币总盘 | Stablecoin total market cap                               | 总盘扩张才有行业顺风       |
| 相对份额   | USDC market share                                         | 份额上升说明竞争改善       |
| USDT 对比  | USDC / USDT ratio                                         | 观察最大竞品差距           |
| 链上分布   | Ethereum / Solana / Base / Arbitrum / Polygon / Avalanche | 判断增长来源               |
| 使用强度   | Transfer volume / adjusted transfer volume                | 供应增长是否变成真实使用   |
| 活跃度     | Transaction count / active addresses                      | 判断用户活跃               |
| DeFi 用途  | Aave / Compound deposits and borrow                       | 判断资金是停泊还是借贷活跃 |
| 交易所用途 | Exchange balances                                         | 判断是否偏交易场景         |
| 收益型竞争 | USDe / BUIDL / USDY / USYC AUM                            | 判断资金是否追收益流出     |
| 渠道经济性 | Base USDC supply / Coinbase products average USDC         | 判断增长是否被渠道拿走利润 |
| 银行竞争   | bank stablecoin / tokenized deposits events               | 判断企业结算心智是否被抢   |

adjusted transfer volume 是清洗后转账量，白话就是尽量剔除刷量、自转账后的真实转账。

## 链上分布解释

| 链                  | 可能含义             | 风险                               |
| ------------------- | -------------------- | ---------------------------------- |
| Ethereum            | 机构、DeFi、深流动性 | Gas 成本高，活跃度解释要谨慎       |
| Solana              | 高频支付、交易、应用 | 周期性和 meme 交易影响大           |
| Base                | Coinbase 生态绑定    | 增长可能被 Coinbase 拿走更多经济性 |
| Arbitrum / Optimism | L2 DeFi 与低成本结算 | 受 DeFi 周期影响                   |
| Polygon             | 应用与支付场景       | 增长质量需看交易额                 |
| Avalanche           | 区域金融和 DeFi      | 规模相对小，波动大                 |

Gas 是链上交易费，白话就是发起交易要付的网络费用。

## 竞争评分

每周计算一次 competition score。

```text
competition score =
  25% USDC relative share
  + 20% distribution economics
  + 15% chain usage quality
  + 15% regulated enterprise adoption
  + 10% yield-product leakage
  + 10% bank/tokenized deposit pressure
  + 5% stock and options crowding
```

distribution economics 是分销经济性，白话就是 Coinbase、Binance、钱包等渠道带来增长后，Circle 自己能留下多少收入。

| 分数   | 结论         | 动作                        |
| ------ | ------------ | --------------------------- |
| 75-100 | 竞争结构强化 | 可上调平台化权重            |
| 55-74  | 中性偏强     | 维持框架，等财报验证        |
| 40-54  | 中性偏弱     | 暂停上调估值倍数            |
| 0-39   | 竞争结构恶化 | 重算增长率、RLDC 和长期倍数 |

## Autoresearch 矩阵复核

每周复盘必须把 `framework/03-competition.md` 的两张矩阵跑一遍。

| 矩阵         | 必填项                                                     | 动作触发                 |
| ------------ | ---------------------------------------------------------- | ------------------------ |
| 渠道议价矩阵 | Coinbase 分成压力、Base 生态绑定、USDC on Platform、分销成本率 | 增长留存下降时降级       |
| 稳定币竞争矩阵 | USDT dominance、PYUSD / FDUSD、USDe、RWA、银行稳定币        | 份额或资金迁移恶化时降级 |

USDC on Platform 是平台上 USDC 余额，白话就是 Circle 或合作平台上留存的 USDC。

## 每周结论分级

| 级别 | 条件                              | 结论           |
| ---- | --------------------------------- | -------------- |
| 强化 | USDC 供应、市占率、转账量三者同升 | 增长质量高     |
| 中性 | USDC 增长但市占率横盘             | 行业 beta 推动 |
| 偏弱 | USDC 增长但市占率下降             | 相对竞争力不足 |
| 降级 | USDC 下滑、市占率下滑、赎回扩大   | 结构风险       |

## 周报模板

```text
周度区间：
USDC 供应变化：
稳定币总市值变化：
USDC 市占率变化：
USDC / USDT ratio：
competition score：
主要链增长来源：
Base USDC supply 与 Coinbase 相关变化：
渠道议价矩阵结论：
转账量与活跃地址：
DeFi / 交易所 / 支付场景判断：
收益型稳定币 / RWA AUM：
稳定币竞争矩阵结论：
银行稳定币 / tokenized deposits 事件：
竞争对手变化：
结论：增强 / 降级 / 观察
下周重点：
```

## 禁止事项

禁止只因为 Base 链增长就上调 CRCL 结论。
必须等财报验证 RLDC margin 是否稳定。

禁止把测试网交易量当成真实收入。
Arc 的主网、客户、费用模型和收入贡献必须分别确认。
