# 竞争仪表盘

## 结论

本文件用于每周填数，目标是把 `framework/03-competition.md` 的竞争判断变成可复盘 dashboard。

dashboard 是仪表盘，白话就是固定表格，每周按同一口径填，避免只凭印象判断竞争强弱。

## 使用规则

每周只填最新一行，不覆盖旧记录。
数据没有官方或稳定来源时填 `missing_info`，不要脑补。

missing_info 是缺失信息，白话就是暂时没有可靠数据。

## 数据获取 SOP

### 更新频率与时间窗口

| 数据类型 | 更新频率 | 最晚更新时间 | 说明 |
| -------- | -------- | ------------ | ---- |
| 链上供应量（USDC、USDT） | 每周 | 周五收盘前 | DefiLlama 日数据取周四快照 |
| 市占率（市值比） | 每周 | 同上 | DefiLlama / CoinGecko 同步取，取较低值 |
| USDC circulating supply | 每周 | 同上 | Circle Transparency 页面，非链上估算 |
| Base USDC supply | 每周 | 同上 | Dune Analytics 或 TokenTerminal |
| 收益型稳定币 AUM（USDe、BUIDL 等） | 每周 | 同上 | RWA.xyz + DefiLlama 叉验 |
| RLDC margin | 每季度 | 财报后 48 小时内 | Circle SEC filing 自算，不用第三方估算 |
| CPN TPV | 每季度 | 财报后 48 小时内 | Circle earnings disclosure，未披露填 missing_info |
| 银行稳定币事件 | 实时 | 事件发生后 24 小时 | OCC / FDIC / 银行新闻稿，无事件填"无" |
| Coinbase products average USDC | 每季度 | Coinbase 财报后 48 小时内 | Coinbase 10-Q / earnings release |

### 逐项获取步骤

#### 1. stablecoin total market cap / USDC market share / USDT dominance

```text
来源：DefiLlama（https://defillama.com/stablecoins）
步骤：
  1. 打开 DefiLlama stablecoins 页面
  2. 记录总市值（Total stablecoin market cap）
  3. 记录 USDC 排名和市值，计算 USDC / total = 市占率
  4. 记录 USDT 市值，计算 USDT / total = USDT dominance
  5. 计算 USDC / USDT ratio

交叉验证：CoinGecko stablecoins 页面同步取，两个来源偏差 >1% 时填 missing_info 并标注偏差
快照日期：取周四 UTC 23:59 数据，不用实时值
```

#### 2. USDC circulating supply

```text
来源：Circle Transparency 页面（https://www.circle.com/en/transparency）
步骤：
  1. 查看最新 attestation 报告日期
  2. 记录报告中的 USDC total supply
  3. 与 DefiLlama USDC supply 对比，差异 >2% 时优先使用 Circle 官方数据并标注
注意：链上聚合数据可能有延迟或计算口径差异，Circle 官方数据是最权威来源
```

#### 3. Base USDC supply

```text
来源：Dune Analytics（首选）或 TokenTerminal
Dune 查询示例：搜索 "USDC Base chain supply" 找到常用 dashboard
步骤：
  1. 记录 Base 链 USDC supply（日期与其他指标对齐）
  2. 计算 Base USDC / USDC circulating supply = Base 占比
  3. 与上周对比，计算环比变化
异常处理：Dune dashboard 数据停更超 3 天，改用 TokenTerminal，并在证据栏标注来源切换
```

#### 4. RLDC margin（季度财报后才更新）

```text
来源：Circle SEC filing（10-Q / 10-K）
计算步骤：
  1. 找到 Reserve income（储备收入）
  2. 找到 Total revenue and reserve income
  3. 找到 Distribution and transaction costs
  4. RLDC = Total revenue and reserve income − Distribution and transaction costs
  5. RLDC margin（主） = RLDC ÷ Reserve income
  6. RLDC margin 全口径（辅） = RLDC ÷ Total revenue（仅当 Other revenue >10% 时填）
注意：不用管理层 presentation 中的"non-GAAP"口径，必须从 GAAP 财报数值自算
```

#### 5. USDe / sUSDe AUM、BUIDL / USDY / USYC AUM

```text
来源（首选）：RWA.xyz（https://rwa.xyz）
来源（交叉）：各项目官网 + DefiLlama
步骤：
  1. RWA.xyz 查看 tokenized treasury 总 AUM
  2. 分别记录 BUIDL、USDY、USYC
  3. DefiLlama 查看 USDe / sUSDe AUM（Protocol → Ethena）
  4. 与上周对比，判断是否有大额 USDC 流出对应
```

#### 6. 银行稳定币 / tokenized deposits 事件

```text
来源：OCC 新闻稿（https://www.occ.gov/news-issuances）
      FDIC 新闻稿（https://www.fdic.gov/news/press-releases）
      主要银行 IR 页面
步骤：
  1. 检查上周是否有 OCC / FDIC 规则更新或银行稳定币公告
  2. 有则记录事件摘要、发布机构、日期
  3. 无则填"无"，不填 missing_info
注意：只记录官方公告，不记录新闻媒体推测
```

### 异常处理规则

| 情形 | 处理方式 |
| ---- | -------- |
| 数据来源暂时不可访问 | 等待 24 小时，超时填 missing_info，标注来源和尝试时间 |
| 两个来源数值偏差 >2% | 填两个数值，标注偏差，不取平均，不自行判断孰优 |
| 季度数据（RLDC）尚未披露 | 填上季度值 + 标注"待Q_财报更新" |
| 链上数据与官方数据矛盾 | 优先官方披露数据，链上数据注明仅供参考 |
| 新项目 AUM 首次出现 | 增加新行，标注来源，不追溯历史值 |

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
| RLDC margin（主，÷储备收入）   |        |        |      | Circle filings（自算）     |      |
| RLDC margin 全口径（÷总收入）  |        |        |      | 仅 Other revenue >10% 时填 |      |
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
