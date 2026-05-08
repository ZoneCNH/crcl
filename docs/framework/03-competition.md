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

## Autoresearch 渠道议价矩阵

渠道与 Coinbase 100 次微实验落成一个规则：供应增长必须和利润留存一起看。

| 复核对象             | 必查证据                                      | 负向动作                         |
| -------------------- | --------------------------------------------- | -------------------------------- |
| Coinbase 分成压力    | RLDC margin、Coinbase filing、收入分成        | 重算 Coinbase 经济性             |
| Binance 渠道贡献     | 渠道余额、合作方披露、监管限制                | 降低新增渠道质量评分             |
| Base 生态绑定        | Base 链数据、Base USDC supply、RLDC margin    | 判定为生态内循环                 |
| USDC on Platform 占比 | 财报问答、渠道余额、分销协议                  | 上调渠道集中度风险               |
| 第三方钱包激励       | 监管口径、USDC rewards、合作方披露            | 触发变相 yield 检查              |
| 交易所余额质量       | 交易所余额、交易深度、稳定币市占率            | 标记为低质量迁移                 |
| 渠道集中度           | Coinbase custody、wallet flow、user growth    | 提高分销折价                     |
| 分销成本率           | Distribution and transaction costs、RLDC margin | 低于阈值时暂停上调倍数           |
| 合作方垂直整合       | Base fee capture、ecosystem lock-in           | 评估利润是否被合作方截留         |
| 分销协议变化         | SEC filing、财报问答、监管限制                | 进入 P0 或 P1 事件复核           |

Coinbase filing 是 Coinbase 披露文件，白话就是用合作方自己的财报和监管文件反查 Circle 的渠道风险。

## Autoresearch 稳定币竞争矩阵

稳定币竞争 50 次微实验落成一个规则：竞争不是只看市值，要看份额、收益迁移和机构替代。

| 竞争项                  | 复核来源                         | 动作规则                         |
| ----------------------- | -------------------------------- | -------------------------------- |
| USDT dominance          | DefiLlama、CoinGecko、交易所深度 | 上升且 USDC 份额下降时降级       |
| PYUSD / FDUSD growth    | DefiLlama、CoinGecko、链上使用   | 只在份额迁移明确时纳入 P2        |
| USDe AUM                | DefiLlama、RWA.xyz、DeFi 借贷    | 伴随 USDC 净赎回时列入 P1        |
| BUIDL / USDY / USYC AUM | RWA.xyz、机构公告                | 抢闲置美元时下修现金管理叙事     |
| bank stablecoin         | 银行公告、OCC / FDIC             | 获得企业客户时下调企业支付倍数   |
| tokenized deposits      | FDIC、银行试点、企业采用         | 替代受监管账户余额时提高折价     |
| stablecoin total market | DefiLlama、CoinGecko             | 总盘扩张但 USDC 不增时视为弱增长 |
| exchange liquidity      | 交易所 API、Kaiko                | USDC 深度下降时削弱交易场景权重  |

FDIC 是 Federal Deposit Insurance Corporation，白话就是美国存款保险机构。

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

## 支付公司竞争：Stripe、Shopify、Amazon Pay

CPN 和 Arc 的真实竞争场景不只是稳定币对稳定币，还包括现有支付公司的替代风险。

### 为何需要将支付公司纳入竞争范围

支付公司不发行稳定币，但直接争夺 CRCL 的商业化场景：

- **企业跨境结算**：Stripe Treasury、Shopify Balance、Amazon Pay 已提供低摩擦法币支付；CPN 要争夺的是同一批企业客户。
- **API 支付基础设施**：Stripe 是 API 驱动支付的标杆。Circle 的企业 API 和 CPN 定位高度重叠。
- **嵌入式金融**：Shopify 对中小商户的嵌入式支付锁定，与 Arc 的目标生态存在竞争。

### 与支付公司的竞争判断框架

| 竞争维度 | CRCL 优势 | CRCL 劣势 | 竞争判断 |
| -------- | --------- | --------- | -------- |
| 合规与监管 | GENIUS Act 稳定币框架提供差异化合规路径 | 银行和支付公司有现成合规体系，新规对 CRCL 成本负担更大 | 中性，视最终规则细则 |
| 跨境速度 | 稳定币 24/7 结算，无对应节假日延迟 | 法币轨道已有 SWIFT gpi、RTP 等提速工具 | 对 CRCL 偏有利，但窗口收窄 |
| 客户获取 | 加密原生企业、DeFi 协议、Web3 公司 | Stripe/Shopify 对 Web2 企业客户的覆盖更广 | CRCL 短期在 Web3 场景有优势 |
| 费率竞争 | 稳定币结算成本理论上更低 | Stripe/Shopify 有规模效应，可接受更低利润率 | 长期不确定 |
| API 开发者体验 | Circle API 有一定开发者基础 | Stripe 是 API 体验的行业基准，生态成熟 | Stripe 明显领先 |

### 支付公司纳入竞争评分的规则

支付公司事件不直接影响 competition_score，但触发以下情况时需更新框架：

| 触发事件 | 影响层级 | 动作 |
| -------- | -------- | ---- |
| Stripe 或 Shopify 宣布稳定币支付集成 | 支付网络竞争层（CPN 直接竞争） | 重新评估 CPN TPV 增长潜力，降低支付网络层分数 |
| Amazon Pay 宣布稳定币或 CBDC 兼容 | 企业结算心智竞争 | 降低 regulated enterprise adoption 分数 |
| Stripe Treasury 或 Shopify Balance 大规模扩张 | 法币替代 CPN 场景 | 追问 CPN 差异化定位，检查 Other revenue 是否受影响 |
| 大型支付公司推出自有稳定币 | 全竞争层 | 升级为 P0/P1 监控，触发框架自检 |

### 监控频率

支付公司纳入每季度竞争复盘，不进入每周数据更新范围。
每季度财报后检查一次 Stripe、Shopify 的支付量和产品动态。

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
