# 竞争格局

## 结论

CRCL 的竞争不是“USDC 对 USDT”一条线。

真正的竞争是六层：

1. 流动性竞争：USDT 的交易所深度和离岸美元网络。
2. 渠道竞争：Coinbase、Binance、钱包和 Base 对用户入口的控制。
3. 制度竞争：银行稳定币和 tokenized deposits 对企业结算心智的争夺。
4. 收益竞争：USDe、BUIDL、USDY、USYC 等产品分流闲置美元。
5. 链生态竞争：不同链上 USDC 增长质量不同，利润留存也不同。
6. 支付网络竞争：CPN、Arc 与银行、卡组织、支付公司争企业支付场景。

tokenized deposits 是代币化存款，白话就是银行把存款搬到链上表达。
RWA 是 Real World Assets，白话就是美债、货币基金等现实资产被搬到链上。

## 核心判断

USDT 决定稳定币市场的流动性天花板。
Coinbase 决定 CRCL 增长能留多少利润。
银行和 tokenized deposits 决定 CRCL 长期能不能吃到企业结算场景。
收益型稳定币决定闲置美元是否留在 USDC。
CPN 和 Arc 决定 CRCL 是否能从利差股切到支付基础设施股。

利差股是靠储备资产利息赚钱的公司。
支付基础设施股是靠网络、API、结算和客户黏性赚钱的公司。

## 竞争地图

| 竞争层   | 主要对手                         | 竞争焦点                         | CRCL 胜负手                | 关键风险                       |
| -------- | -------------------------------- | -------------------------------- | -------------------------- | ------------------------------ |
| 流动性   | USDT / Tether                    | 市占率、交易所深度、离岸美元需求 | 合规、机构和企业支付       | USDC 份额长期下滑              |
| 渠道     | Coinbase / Binance / 钱包        | 用户入口、分销激励、链生态       | 多渠道分发和自有网络       | RLDC margin 被渠道吃掉         |
| 制度     | 银行稳定币 / tokenized deposits  | 监管许可、企业账户、存款关系     | 非银行合规路径和开放链生态 | 银行获得低成本企业结算入口     |
| 收益     | USDe / BUIDL / USDY / USYC       | 闲置美元收益                     | 支付和结算用途             | 用户把 USDC 换成收益产品       |
| 链生态   | Base / Solana / Ethereum / L2    | 使用场景和交易活跃               | 多链原生 USDC              | 增长发生在低利润或高渠道成本链 |
| 支付网络 | 银行、卡组织、支付公司、跨境网络 | 企业支付、跨境结算、合规工作流   | CPN、Arc、API              | TPV 增长但收入不增长           |

RLDC margin 是 Revenue Less Distribution Costs margin，白话就是扣掉渠道分成后，Circle 留下来的收入比例。

## USDT / Tether：流动性竞争

USDT 是 CRCL 最大的市场结构对手。

Tether 官方披露，2025 年 USDT 流通量超过 1860 亿美元，储备资产接近 1930 亿美元，全年利润超过 100 亿美元。
这说明 USDT 的竞争力不是单纯“监管折价”，而是规模、利润、交易所深度、离岸美元需求共同形成的流动性网络。

对 CRCL 的含义：

1. USDC 不需要在所有交易场景复制 USDT。
2. CRCL 必须证明合规、透明、企业支付能带来更高质量收入。
3. 如果 USDC 市占率下降，不能只用“监管更合规”解释。
4. 如果 USDC 转账量上升但市占率下降，要拆分是真实支付还是链上刷量。

### USDT 监控阈值

| 指标                          | 强化 CRCL             | 削弱 CRCL                 | 来源                       |
| ----------------------------- | --------------------- | ------------------------- | -------------------------- |
| USDC / USDT 市值比            | 连续 4 周上升         | 连续 4 周下降             | DefiLlama / CoinGecko      |
| USDT dominance                | 下降且 USDC 份额上升  | 高位继续上升              | DefiLlama                  |
| USDC adjusted transfer volume | 供应和转账量双增      | 转账量增但供应不增        | Dune / CoinMetrics         |
| 交易所 USDC 深度              | 主要交易所深度改善    | USDT 深度继续扩大         | 交易所 API / Kaiko         |
| 离岸使用                      | USDC 在非美国场景增长 | USDT 继续吃掉新兴市场支付 | Chainalysis / Visa reports |

adjusted transfer volume 是清洗后转账量，白话就是尽量剔除刷量、自转账后的真实转账。

## Coinbase：渠道伙伴和议价对手

Coinbase 对 CRCL 是双重角色。

它是 USDC 增长引擎，也是 CRCL 的利润分配对手。
Coinbase 2026 Q1 披露，Coinbase 产品中平均持有超过 190 亿美元 USDC，占 USDC 总流通量 25% 以上；Base 处理了全球链上稳定币交易量的 62%。
这说明 Coinbase 不是普通渠道，而是 USDC 使用和分销的系统性入口。

### Coinbase 三个问题

1. Base 链 USDC 增长是谁的利润？
2. Coinbase 持有 USDC 越多，CRCL 的 RLDC margin 是否稳定？
3. 监管如果限制第三方激励，Coinbase 分销模式是否需要重写？

### 判断公式

```text
Base USDC 增长
  + RLDC margin 稳定或上升
  = 高质量增长

Base USDC 增长
  + RLDC margin 下滑
  = 渠道拿走更多经济性
```

### Coinbase 监控阈值

| 指标                           | 正向信号                     | 负向信号                   |
| ------------------------------ | ---------------------------- | -------------------------- |
| Coinbase products average USDC | 增长且 RLDC margin 稳定      | 增长但 RLDC margin 下滑    |
| Coinbase stablecoin revenue    | 增长但 Circle 经济性同步改善 | 增长主要由分成驱动         |
| Base USDC supply               | 和总 USDC、市占率同步增长    | 只在 Base 增长，总份额不升 |
| Base stablecoin volume share   | 带动真实结算和 CPN           | 只带动 Coinbase 入口       |
| 监管口径                       | 第三方激励不被限制           | 激励被视作变相 yield       |

变相 yield 是绕开规则支付收益，白话就是不叫利息但经济效果像利息。

## Binance 和其他交易所：增量入口但议价不透明

Binance、钱包、做市商和其他交易所能带来流通量，但经济性不如官方供应数据直观。

对 CRCL 的关键不是“有没有新增渠道”，而是新增渠道的单位经济性。

单位经济性是 unit economics，白话就是每新增 1 美元 USDC，Circle 能留下多少收入。

### 交易所渠道判断

| 情形                                  | 判断               |
| ------------------------------------- | ------------------ |
| USDC 交易所余额增长，RLDC margin 稳定 | 有效分销           |
| USDC 交易所余额增长，市占率不升       | 低质量迁移         |
| 交易所给 USDC 激励，监管未限制        | 短期利好供应       |
| 交易所激励被监管限制                  | 重新计算分销成本   |
| 交易所 USDC 深度下降                  | 交易场景竞争力削弱 |

## 银行稳定币与 tokenized deposits：制度竞争

银行稳定币不一定先抢 crypto 交易场景。
它们更可能先抢企业结算、机构现金管理、受监管钱包和 B2B 支付。

FDIC 2026-04-07 拟议规则明确讨论两件事：

1. 支付稳定币储备存款的保险适用方式。
2. tokenized deposits 满足 deposit 定义时，按存款处理。

这对 CRCL 的竞争含义是：银行可能不需要复制 USDC 的开放生态，也能在企业和机构场景拿到更低获客成本。

B2B 是 Business-to-Business，白话就是企业对企业。

### 银行竞争路径

| 路径               | 对 CRCL 的威胁   | 监控指标                     |
| ------------------ | ---------------- | ---------------------------- |
| 银行发行稳定币     | 抢企业支付心智   | FDIC / OCC / 银行公告        |
| tokenized deposits | 抢受监管账户余额 | FDIC 规则和大行试点          |
| 银行托管稳定币储备 | 提高银行议价权   | 储备托管关系                 |
| 银行结算网络上链   | 降低 CPN 差异化  | 企业客户迁移                 |
| 银行和卡组织合作   | 抢跨境和商户场景 | Visa / Mastercard / 银行公告 |

## 收益型稳定币和 RWA：闲置美元竞争

收益型稳定币和 RWA 产品不是直接替代支付稳定币。
它们抢的是闲置美元、DeFi 抵押品和机构短久期资金。

当短端利率高、用户追收益时，USDC 持有人会比较三件事：

1. USDC 的流动性和稳定性。
2. USDe / sUSDe 的收益率和风险。
3. BUIDL / USDY / USYC 等 RWA 产品的机构可接受度。

### 收益竞争判断

| 指标                        | CRCL 正向            | CRCL 负向                |
| --------------------------- | -------------------- | ------------------------ |
| USDe / sUSDe AUM            | 增长不影响 USDC 供应 | AUM 增长伴随 USDC 净赎回 |
| BUIDL / USDY / USYC AUM     | 机构资金总盘扩大     | 闲置 USDC 被 RWA 吸走    |
| DeFi USDC lending APY       | 借款需求强           | 只有存款高、借款弱       |
| Aave / Compound USDC borrow | 借贷活跃             | 资金停泊                 |
| Tokenized treasury AUM      | 与 USDC 一起增长     | 替代 USDC 现金管理       |

AUM 是 Assets Under Management，白话就是管理资产规模。

## 链生态：供应增长不等于高质量增长

USDC by chain 必须拆用途。

同样是 10 亿美元新增 USDC，增长质量可能完全不同：

| 链                  | 可能含义                        | 质量判断                               |
| ------------------- | ------------------------------- | -------------------------------------- |
| Ethereum            | 机构、DeFi、深流动性            | 看 adjusted transfer volume 和大额地址 |
| Solana              | 高频应用、支付、交易            | 看交易是否可持续，防刷量               |
| Base                | Coinbase 入口、Agentic commerce | 必须看 RLDC margin 和 Coinbase 分成    |
| Arbitrum / Optimism | L2 DeFi 与低成本结算            | 看 DeFi 借款和交易深度                 |
| Polygon             | 商户、应用和支付场景            | 看真实商户结算                         |
| Avalanche           | 区域金融和 DeFi                 | 看规模能否持续                         |

Agentic commerce 是代理式商业，白话就是 AI agent 或自动化程序参与支付和交易。

## 支付网络竞争：CPN 和 Arc 的对手不只是稳定币

CPN 和 Arc 的竞争对手包括银行、卡组织、跨境支付网络、企业财资系统、钱包和 L2 生态。

平台化验证必须回答：

1. CPN TPV 是否变成 Other revenue。
2. Arc 主网是否带来真实 USDC 使用量。
3. 企业客户是否愿意为 API、结算、合规和速度付费。
4. 银行和卡组织是否用更低成本复制同类能力。

TPV 是 Total Payment Volume，白话就是支付网络上跑过的交易额。

## 竞争评分模型

每周给竞争格局打 0-100 分。

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

| 分数   | 结论         | 动作                 |
| ------ | ------------ | -------------------- |
| 75-100 | 竞争结构强化 | 可上调平台化权重     |
| 55-74  | 中性偏强     | 维持框架，等财报验证 |
| 40-54  | 中性偏弱     | 暂停上调估值倍数     |
| 0-39   | 竞争结构恶化 | 重算增长率和长期倍数 |

## 竞争预警

| 预警                                | 级别 | 触发                |
| ----------------------------------- | ---- | ------------------- |
| USDC 市占率连续 4 周下降            | P1   | 相对竞争力恶化      |
| USDC 增长但 RLDC margin 低于 38%    | P1   | 渠道吃掉经济性      |
| Base USDC 增长但总 USDC 份额不升    | P1   | Coinbase 生态内循环 |
| 银行稳定币获得明确企业客户          | P2   | 企业结算被抢心智    |
| USDe / RWA AUM 增长伴随 USDC 净赎回 | P1   | 闲置美元流失        |
| 监管限制第三方激励                  | P0   | 分销模式重写        |
| CPN TPV 增长但 Other revenue 不增长 | P2   | 平台化商业化不足    |

## 情景推演

### Bull case

条件：

1. USDC 市占率连续 4 周上升。
2. USDC / USDT ratio 上升。
3. RLDC margin 稳定在 38-40% 以上。
4. Base、Solana、Ethereum 增长同时出现。
5. Other revenue 和 CPN TPV 同步增长。
6. 银行稳定币没有抢走企业结算叙事。

结论：CRCL 竞争结构强化，可提高平台化权重。

### Base case

条件：

1. USDC 随稳定币总盘增长。
2. 市占率横盘。
3. RLDC margin 稳定。
4. Coinbase 继续贡献供应，但议价压力可控。
5. 收益型竞品增长但没有触发 USDC 净赎回。

结论：仍按利差股加平台期权研究。

### Bear case

条件：

1. USDC 市占率连续下降。
2. Base 增长但 RLDC margin 下滑。
3. USDe、BUIDL、USDY、USYC 等产品分流闲置美元。
4. 银行稳定币或 tokenized deposits 获得企业结算采用。
5. 监管限制第三方激励或收益分享。

结论：竞争结构恶化，长期倍数下修。

## 周度竞争复盘模板

```text
结论：

依据：
1. USDC / USDT ratio：
2. USDC market share：
3. stablecoin total market cap：
4. Base USDC supply 与 RLDC margin：
5. 收益型稳定币 / RWA AUM：
6. 银行稳定币 / tokenized deposits 事件：

动作：

missing_info：
```

missing_info 是缺失信息，白话就是不能脑补、必须等数据确认的部分。

## 竞争来源

| 来源                                          | 用途                                              |
| --------------------------------------------- | ------------------------------------------------- |
| Circle FY2025 / Q4 2025 results               | USDC、RLDC、分销成本、链上交易量                  |
| Circle 2025 annual report / SEC               | Coinbase、Binance、Stablecoin Ecosystem Agreement |
| Coinbase Q1 2026 results                      | Coinbase products USDC、Base 稳定币交易量         |
| Tether Q4 2025 attestation / official release | USDT 规模、储备、利润、Treasury exposure          |
| OCC GENIUS Act NPRM                           | PPSI、储备、收益限制、非银行路径                  |
| FDIC GENIUS Act proposed rule                 | tokenized deposits、银行发行和储备保险口径        |
| DefiLlama stablecoins                         | 稳定币总盘、市占率、USDT dominance                |
| RWA.xyz                                       | Tokenized treasury 和收益型竞品 AUM               |

## 资料链接

- Circle FY2025 / Q4 2025 results：`https://www.circle.com/en/pressroom/circle-reports-fourth-quarter-and-full-fiscal-year-2025-financial-results`
- Circle FY2025 SEC filing：`https://www.sec.gov/Archives/edgar/data/1876042/000187604226000116/a2025annualreport0331.pdf`
- Coinbase Q1 2026 results：`https://investor.coinbase.com/news/news-details/2026/Coinbase-Q1-Financial-Results-Show-Resilient-Financial-Performance-Driven-by-New-All-Time-High-Crypto-Trading-Volume-Market-Share/default.aspx`
- Tether Q4 2025 official release：`https://tether.io/news/tether-delivers-10b-profits-in-2025-6-3b-in-excess-reserves-and-record-141-billion-exposure-in-u-s-treasury-holdings/`
- OCC GENIUS Act NPRM：`https://www.occ.gov/news-issuances/news-releases/2026/nr-occ-2026-9.html`
- FDIC GENIUS Act proposed rule：`https://www.fdic.gov/news/press-releases/2026/fdic-approves-proposal-implement-genius-act-requirements-and-standards`
- DefiLlama stablecoins：`https://defillama.com/stablecoins`
