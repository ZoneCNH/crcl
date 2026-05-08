# 竞争仪表盘

## 结论

本文件用于每周填数，目标是把 `framework/03-competition.md` 的竞争判断变成可复盘 dashboard。

dashboard 是仪表盘，白话就是固定表格，每周按同一口径填，避免只凭印象判断竞争强弱。

## 使用规则

每周只填最新一行，不覆盖旧记录。
数据没有官方或稳定来源时填 `missing_info`，不要脑补。

missing_info 是缺失信息，白话就是暂时没有可靠数据。

## 本周快照

```text
周度区间：
填表日期：
数据源日期：
结论：竞争结构强化 / 中性偏强 / 中性偏弱 / 竞争结构恶化
competition score：
动作：增强 / 降级 / 观察
```

## Competition Score

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

| 维度                            | 权重 | 本周分数 | 证据                                                      | 动作 |
| ------------------------------- | ---: | -------: | --------------------------------------------------------- | ---- |
| USDC relative share             |  25% |          | USDC market share、USDC / USDT ratio                      |      |
| distribution economics          |  20% |          | RLDC margin、Base USDC、Coinbase products USDC            |      |
| chain usage quality             |  15% |          | adjusted transfer volume、active addresses、USDC by chain |      |
| regulated enterprise adoption   |  15% |          | CPN TPV、企业客户、银行稳定币事件                         |      |
| yield-product leakage           |  10% |          | USDe、BUIDL、USDY、USYC AUM                               |      |
| bank/tokenized deposit pressure |  10% |          | FDIC/OCC/银行公告、tokenized deposits 试点                |      |
| stock and options crowding      |   5% |          | short interest、IV、lock-up、Form 4                       |      |

## 阈值解释

|   总分 | 结论         | 动作                        |
| -----: | ------------ | --------------------------- |
| 75-100 | 竞争结构强化 | 可上调平台化权重            |
|  55-74 | 中性偏强     | 维持框架，等财报验证        |
|  40-54 | 中性偏弱     | 暂停上调估值倍数            |
|   0-39 | 竞争结构恶化 | 重算增长率、RLDC 和长期倍数 |

## 指标记录

| 指标                           | 本周值 | 上周值 | 变化 | 来源                       | 判断 |
| ------------------------------ | -----: | -----: | ---: | -------------------------- | ---- |
| stablecoin total market cap    |        |        |      | DefiLlama                  |      |
| USDC market share              |        |        |      | DefiLlama / CoinGecko      |      |
| USDT dominance                 |        |        |      | DefiLlama                  |      |
| USDC / USDT ratio              |        |        |      | 自算                       |      |
| USDC circulating supply        |        |        |      | Circle Transparency        |      |
| Base USDC supply               |        |        |      | Dune / TokenTerminal       |      |
| Coinbase products average USDC |        |        |      | Coinbase filings           |      |
| RLDC margin                    |        |        |      | Circle filings             |      |
| adjusted transfer volume       |        |        |      | CoinMetrics / Dune         |      |
| active addresses               |        |        |      | Dune / Santiment           |      |
| PYUSD / FDUSD supply           |        |        |      | DefiLlama / CoinGecko      |      |
| USDe / sUSDe AUM               |        |        |      | Ethena / DefiLlama         |      |
| BUIDL / USDY / USYC AUM        |        |        |      | RWA.xyz                    |      |
| CPN TPV                        |        |        |      | Circle disclosures         |      |
| bank stablecoin events         |        |        |      | OCC / FDIC / bank releases |      |
| tokenized deposits events      |        |        |      | FDIC / bank releases       |      |

## 渠道议价记录

| 复核项                | 本周证据 | 判断 | 动作 |
| --------------------- | -------- | ---- | ---- |
| Coinbase 分成压力     |          |      |      |
| Base 生态绑定         |          |      |      |
| USDC on Platform 占比 |          |      |      |
| 第三方钱包激励        |          |      |      |
| 交易所余额质量        |          |      |      |
| 渠道集中度            |          |      |      |
| 分销成本率            |          |      |      |
| 分销协议变化          |          |      |      |

## 稳定币竞争记录

| 复核项                  | 本周证据 | 判断 | 动作 |
| ----------------------- | -------- | ---- | ---- |
| USDT dominance          |          |      |      |
| PYUSD / FDUSD growth    |          |      |      |
| USDe AUM                |          |      |      |
| BUIDL / USDY / USYC AUM |          |      |      |
| bank stablecoin         |          |      |      |
| tokenized deposits      |          |      |      |
| exchange liquidity      |          |      |      |

## 快速判定

| 触发                                | 结论             | 动作                      |
| ----------------------------------- | ---------------- | ------------------------- |
| USDC market share 连续 4 周上升     | 相对竞争改善     | 检查 RLDC margin 是否同步 |
| USDC market share 连续 4 周下降     | 竞争结构转弱     | 降低增长质量评分          |
| Base USDC 增长但总 USDC 份额不升    | Coinbase 内循环  | 暂停上调平台权重          |
| Base USDC 增长且 RLDC margin 下滑   | 渠道吃掉经济性   | 重算分销成本假设          |
| USDe / RWA AUM 上升且 USDC 净赎回   | 闲置美元流失     | 降低收益竞争评分          |
| CPN TPV 增长但 Other revenue 不增长 | 平台化商业化不足 | 保持利差股主框架          |
| 银行稳定币获得企业采用              | 制度竞争加强     | 下调长期平台倍数          |

## 周度记录模板

```text
周度区间：
competition score：
本周最强证据：
本周最弱证据：
USDC relative share：
distribution economics：
渠道议价矩阵：
chain usage quality：
regulated enterprise adoption：
yield-product leakage：
稳定币竞争矩阵：
bank/tokenized deposit pressure：
stock and options crowding：
结论：
动作：
missing_info：
下次复盘触发：
```
