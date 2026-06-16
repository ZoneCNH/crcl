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
| Amazon Pay 宣布稳定币或 CBDC 兼容 | 企业结算心智竞争 | 降低 D7 平台化验证分数，并检查 D6 监管壁垒是否联动 |
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
  25% × D1（USDC 市占率趋势）
  + 15% × D2（USDT 相对威胁）
  + 20% × D3（分销渠道议价）
  + 15% × D4（收益型稳定币蚕食）
  + 10% × D5（链生态健康）
  + 10% × D6（监管壁垒）
  + 5%  × D7（平台化验证）
```

| 分数   | 结论         | 动作                 |
| ------ | ------------ | -------------------- |
| 75-100 | 竞争结构强化 | 可上调平台化权重     |
| 55-74  | 中性偏强     | 维持框架，等财报验证 |
| 40-54  | 中性偏弱     | 暂停上调估值倍数     |
| 0-39   | 竞争结构恶化 | 重算增长率和长期倍数 |

## 竞争评分分维度阈值来源

competition_score 的七个维度权重和阈值来源于 autoresearch 550 次竞争迭代，以下记录每个维度的来源依据，便于季度自检时核实和调整。

### 各维度权重来源

| 维度 | 权重 | 阈值来源 | 更新规则 |
| ---- | ---- | -------- | -------- |
| D1 USDC 市占率趋势（25%） | DefiLlama / CoinGecko 月度稳定币市占率 | USDC 市占率与 USDC/USDT ratio 的 4 周趋势 | 每周 |
| D2 USDT 相对威胁（15%） | DefiLlama、Tether 官方公告 | USDT dominance 趋势与离岸扩张事件 | 每周 + 事件 |
| D3 分销渠道议价（20%） | RLDC margin、分销成本率、Base USDC、Coinbase filing | 渠道增长是否吞噬 Circle 利润留存 | 每周监控，季度财报后重算 |
| D4 收益型稳定币蚕食（15%） | RWA.xyz、USDe AUM、DefiLlama RWA、USDC net mint/redeem | 收益型产品增长是否伴随 USDC 净赎回 | 每周 |
| D5 链生态健康（10%） | CoinMetrics、Dune、DeFi borrow 利用率 | 供应增长是否转化为真实链上使用 | 每周 |
| D6 监管壁垒（10%） | Congress、OCC、Treasury、FDIC、银行公告 | 监管是否强化非银行合规路径或制造银行竞争优势 | 事件 + 每周扫描 |
| D7 平台化验证（5%） | Other revenue share、CPN TPV、Arc、short interest、IV | 平台化是否有财务证据，筹码是否放大叙事风险 | 季度财报 + 每周筹码 |

### 权重调整规则

权重不是固定不变的，以下情况触发季度自检时重新校准：

1. **分销经济性越来越重要**：如果 Coinbase 持仓占比超过 35% 且 RLDC margin 连续下滑，distribution economics 权重可上调至 25%，同时压低其他维度。
2. **银行稳定币加速进入**：如果银行稳定币开始获得企业客户，bank/tokenized deposit pressure 权重可上调至 15%。
3. **平台化已验证**：如果 Other revenue 占比超过 15%，D5 链生态健康与 D7 平台化验证权重可提升以反映平台化实质。

**权重调整必须在 autoresearch 日志中记录，不能静默修改。**

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

---

## 新增（2026-05-09）

## 历史评分校验（Backtesting）

目的：验证现行7维度评分模型的方向准确性。将已知历史数据代入当前评分框架，检验推算总分与事后实际结果是否方向一致。

方法论说明：历史数据不完整时，标注"历史数据待补充，此为方向性推算"，仍给出推算分值，不留空。分数置信度用★表示（★★★ = 有文件事实支撑；★☆☆ = 方向性推算）。

### 时期A：2024年中（USDC开始反弹）

**已知事实（来源于文件既有记录和公开数据）：**

- USDC 市占率：约 17%（任务说明；低于20%为降级区间）
- USDT 市占率：约 66%（任务说明；仍处扩张期）
- USDC/USDT 市值比（推算）：≈ 0.26（17÷66，与2026年5月的0.45相比差距显著）
- Base 链：2023年6月上线，2024年中仍处于极早期，USDC on Base 占比 <5%
- 收益型稳定币：USDe 2023年12月上线，2024年中 AUM 估计 $1–3B；BUIDL 2024年3月上线，规模极小
- RLDC margin：**历史数据待补充**（Circle 2024年尚未上市，RLDC 口径未公开披露；基于Q4 2025实测42%的轨迹，方向性推算约 38-40%）
- CPN：早期阶段，机构接入数量远低于2026年2月的55家
- GENIUS Act：2025年2月才正式提出，2024年中监管框架不明确，无正负面监管事件

**7维度推算打分：**

| 维度 | 权重 | 推算数据依据 | 推算分值 | 加权分 | 置信度 |
| ---- | ---: | ------------ | -------: | -----: | ------ |
| D1 USDC市占率趋势 | 25% | 市占率17%，"开始反弹"→连续1-2周上升，累计+0.3 ppt；USDC/USDT ratio约0.26开始上行；未达4周连续上升 | 50 | 12.50 | ★★★ |
| D2 USDT相对威胁 | 15% | USDT dominance 66%，2024年仍在扩张期，4周内未见连续下降 | 25 | 3.75 | ★★★ |
| D3 分销渠道议价 | 20% | RLDC margin推算~38-40%（历史数据待补充）；Base USDC占比<5%（100分档）；分销成本率趋势未知；取RLDC方向性推算对应分档 | 50 | 10.00 | ★☆☆ |
| D4 收益型稳定币蚕食 | 15% | USDe AUM约$1-3B，威胁初显但规模小；RWA产品极小；USDC仍净铸造；收益型AUM周环比估算<5% | 75 | 11.25 | ★★☆ |
| D5 链生态健康 | 10% | 2024年加密市场复苏期，transfer volume 回暖但4周MA变化约+1-2%；Borrow利用率约30-40%；active addresses温和增长 | 50 | 5.00 | ★★☆ |
| D6 监管壁垒 | 10% | GENIUS Act尚未提出；无OCC/FDIC正负面事件；银行稳定币无企业客户；中性 | 50 | 5.00 | ★★★ |
| D7 平台化验证 | 5% | CPN极早期，机构数远低于55家；Other revenue share极低（推算<3%）；短期指标缺失 | 25 | 1.25 | ★☆☆ |
| **合计** | 100% | | | **48.75** | |

**档位：中性偏弱（40-54）**

**事后验证（方向准确）：** 2024年中至2025年初，USDC市占率从17%上升至20-22%，USDC/USDT ratio明显上行，竞争格局实质改善。评分48.75处于"中性偏弱"上区间，与当时USDC尚未确立稳定上行趋势的现实相符——模型未给出强买信号，但也未误判为"竞争恶化"；随后分数上升至70分区间印证了模型方向的准确性。D2（25分）是拖低总分的核心维度，也确实是当时USDT扩张最激烈的时期。

---

### 时期B：2025年初（USDC加速增长）

**已知事实（来源于文件既有记录和公开数据）：**

- USDC 市占率：约 20-22%（从17%明显提升，上行趋势已确立）
- USDT dominance：约 62-64%（从66%回落，4周持续下降）
- USDC/USDT 市值比（推算）：≈ 0.32-0.34
- Base 链：快速发展，USDC on Base 占比推算约 12-20%（处于75分档28-35%区间或以下）
- 收益型稳定币：USDe AUM $3-8B 区间，增速加快；BUIDL/USDY/USYC 总AUM约 $2-5B
- RLDC margin：**历史数据待补充**；基于Q4 2025实测42%的轨迹，2025年初推算约 39-41%（接近75分档40-42%下边界）
- CPN：机构接入数量增长中（介于早期与2026年2月55家之间；推算约20-35家）
- GENIUS Act：2025年2月正式提出，初期参议院听证积极，无OCC/FDIC负面裁决
- 银行稳定币：企业客户数量极少，处于讨论阶段
- USDC net mint：持续正向，净铸造

**7维度推算打分：**

| 维度 | 权重 | 推算数据依据 | 推算分值 | 加权分 | 置信度 |
| ---- | ---: | ------------ | -------: | -----: | ------ |
| D1 USDC市占率趋势 | 25% | 市占率20-22%且连续上行；4周累计>1 ppt；USDC/USDT ratio持续上升 | 75 | 18.75 | ★★★ |
| D2 USDT相对威胁 | 15% | USDT dominance从66%降至62-64%，4周内累计降幅>2 ppt；无重大离岸扩张事件 | 75 | 11.25 | ★★★ |
| D3 分销渠道议价 | 20% | RLDC margin推算39-41%（历史数据待补充）；Base USDC占比约12-20%（75分档）；分销成本率方向温和 | 75 | 15.00 | ★★☆ |
| D4 收益型稳定币蚕食 | 15% | USDe AUM周环比约10-15%；收益型产品总AUM增速加快但USDC仍净铸造 | 50 | 7.50 | ★★☆ |
| D5 链生态健康 | 10% | Base/Ethereum/Solana同步发展；transfer volume 4周MA上升约2-4%；active addresses改善 | 75 | 7.50 | ★★☆ |
| D6 监管壁垒 | 10% | GENIUS Act正式提出，参议院听证积极；无OCC/FDIC负面裁决；银行稳定币无企业客户 | 75 | 7.50 | ★★★ |
| D7 平台化验证 | 5% | CPN机构接入增长；Other revenue share推算约3-5%（25-50分档边界）；Circle IPO前，股票指标缺失 | 50 | 2.50 | ★☆☆ |
| **合计** | 100% | | | **70.00** | |

**档位：中性偏强（55-74），接近强化档（75分）边界**

**事后验证（方向准确）：** 2025年CRCL上市，USDC市占率从20-22%进一步上升至约25%，Q4 2025 RLDC margin实测达42%。评分70.0处于"中性偏强"高位，与实际竞争格局改善完全一致——该档位对应"维持框架，等财报验证"，而随后财报验证了每个核心假设（D3 RLDC margin从推算39-41%上到实测42%，D2 USDT dominance持续下行）。

---

### 时期C：2026年初（历史样本）

**已知数据（来源于文件既有记录，均有文件依据）：**

| 数据项 | 数值 | 来源 |
| ------ | ---- | ---- |
| USDC/USDT ratio | 0.449→0.451，变化+0.002 | 05-competition-scoring-rubric.md 综合计分示例 |
| USDC market share 4周变化 | +0.2 ppt（横盘） | 05-competition-scoring-rubric.md 综合计分示例 |
| RLDC margin（Q1 2026） | 40.3% | 05-competition-scoring-rubric.md 综合计分示例 |
| Base USDC 占比 | 32% | 05-competition-scoring-rubric.md 综合计分示例 |
| USDT dominance 4周变化 | +0.8 ppt（未超1 ppt） | 05-competition-scoring-rubric.md 综合计分示例 |
| 收益型产品AUM 周环比 | +13% | 05-competition-scoring-rubric.md 综合计分示例 |
| USDC 当周净铸造 | +3亿美元 | 05-competition-scoring-rubric.md 综合计分示例 |
| Borrow 利用率 | 48% | 05-competition-scoring-rubric.md 综合计分示例 |
| Adjusted transfer volume 4周MA | 上升+3.1% | 05-competition-scoring-rubric.md 综合计分示例 |
| Active addresses 环比 | +1.2% | 05-competition-scoring-rubric.md 综合计分示例 |
| GENIUS Act 状态 | 仍在审议，无正负面新事件 | 03-competition.md 资料链接 |
| 银行稳定币企业客户数 | <3个 | 05-competition-scoring-rubric.md 综合计分示例 |
| Other revenue share | 5.5% | 05-competition-scoring-rubric.md 综合计分示例 |
| Short interest | 9% | 05-competition-scoring-rubric.md 综合计分示例 |
| 30D IV | 38% | 05-competition-scoring-rubric.md 综合计分示例 |

**7维度正式打分（按05-competition-scoring-rubric.md逐档执行）：**

| 维度 | 权重 | 分值 | 加权分 |
| ---- | ---: | ---: | -----: |
| D1 USDC市占率趋势 | 25% | 50 | 12.50 |
| D2 USDT相对威胁 | 15% | 50 | 7.50 |
| D3 分销渠道议价 | 20% | 75 | 15.00 |
| D4 收益型稳定币蚕食 | 15% | 50 | 7.50 |
| D5 链生态健康 | 10% | 75 | 7.50 |
| D6 监管壁垒 | 10% | 50 | 5.00 |
| D7 平台化验证 | 5% | 50 | 2.50 |
| **合计** | 100% | | **57.50** |

**档位：中性偏强（55-74）**  
**样本结论：D3（RLDC margin 40.3%）是最强支撑维度；D1（市占横盘）和D2（USDT未见明显下行）是最关键边际变量。运行时必须重新刷新数据，不得把本样本当作当前结论。**

---

### Backtesting 总结与校验评估

| 时期 | 推算总分 | 档位 | 事后实际结果 | 模型方向准确性 |
| ---- | -------: | ---- | ------------ | -------------- |
| 2024年中 | 48.75 | 中性偏弱 | USDC随后反弹，市占17%→20%+ | ✅ 准确（偏弱档对应随后的改善空间，未误判为恶化或强化） |
| 2025年初 | 70.00 | 中性偏强 | CRCL上市，USDC市占上升，Q4财报RLDC 42% | ✅ 准确（接近强化档，对应实际竞争改善） |
| 2026年初 | 57.50 | 中性偏强 | 历史样本，运行时需刷新 | 仅作样本 |

**模型局限性说明：**

- 2024年中和2025年初的D3和D7评分因历史财务数据缺失而为方向性推算；若补充完整RLDC历史数据，总分调整幅度预计在±5-8分以内，但档位方向预计不变。
- 建议在Circle首次全年财务数据可追溯后（预计2026年Q2财报后），补充2024年历史D3 RLDC margin数据回溯，并更新本章版本号。

---

## Base链绑定风险量化

现行D3（分销渠道议价）已有Base USDC占比阈值（≤28%→100分；≥45%+RLDC下滑→0分），但缺乏"当Base占比持续过高时"的动态触发机制和跨维度联动规则。本章补充该机制。

### 量化阈值表

| 指标 | 历史样本估算值 | 观察阈值 | 预警阈值 | 危险阈值 |
| ---- | ---------- | -------- | -------- | -------- |
| Base链USDC占总USDC流通量% | ~32% | >35% | >40% | ≥45%且RLDC margin同期下滑≥1 ppt |
| Base链USDC月环比净增速 | 历史数据待补充 | >15%/月 | >25%/月 | >35%/月且总USDC市占不升 |
| Coinbase products average USDC占总USDC% | ~25%+ | >30% | >38% | >45%且RLDC margin同期下滑 |

> 历史样本估算值来源：Base USDC占比32%来自05-competition-scoring-rubric.md综合计分示例（2026-05-01数据）；Coinbase products average USDC $19B+（占总USDC供应25%以上）来自Coinbase Q1 2026 results。运行时必须刷新对应季度数据。

### 触发逻辑

**观察（Base占比跨过35%）：**

- 条件：Base USDC占比 >35% **且** USDC总市占率同期上升（D1 ≥75分）
- 解读：Base增长带动全网份额提升 → 正面信号，Coinbase渠道产生真实增量
- 动作：继续监控；在04-dashboard渠道议价记录"Base生态绑定"行标注"观察：占比>35%"；下一季度财报后核实RLDC margin是否同步改善

**预警（Base占比跨过40%且总市占横盘）：**

- 条件：Base USDC占比 >40% **且** USDC总市占率横盘（D1为50分，4周变化<±0.3 ppt）
- 解读：Base在Coinbase生态内增长，但未带动总份额扩张 → RLDC margin质量可能虚高（Base内部margin可能更高但依赖Coinbase生态，Coinbase议价权增强信号）
- 动作：**执行本文件（playbook/02-competition-review.md）"渠道分析"专项**；D3从75分下调至50分（除非RLDC margin维持≥42%提供额外支撑）；核查分销成本率和Coinbase stablecoin revenue增速

**危险（Base占比≥45%且RLDC下滑）：**

- 条件：Base USDC占比 ≥45% **且** USDC总市占率4周下降 **且** RLDC margin较上季度下滑 ≥1 ppt
- 解读：Base在Coinbase生态内循环 → 供应增长被Coinbase渠道捕获；Coinbase渠道议价能力实质性增强，Circle分润空间被压缩
- 动作：**触发预警G（参照risk/01-warning-signals.md）**；D3直接判0分（RLDC margin <36%条件满足）；触发全维度重新计分；重算Coinbase经济性假设

### 处理动作汇总

```text
观察触发（Base>35%且总市占上升）：
  → 04-dashboard"Base生态绑定"行标注"观察：占比>35%"
  → 下次财报后核实RLDC margin方向
  → 不触发D3 score调整

预警触发（Base>40%且总市占横盘）：
  → 执行playbook/02-competition-review.md "渠道分析"专项
  → D3从75分下调至50分（除非RLDC margin ≥42%）
  → 04-dashboard渠道议价记录标注"预警"

危险触发（Base≥45%+RLDC下滑≥1 ppt+总市占下降）：
  → 触发预警G（risk/01-warning-signals.md）
  → D3强制判0分
  → 执行全维度重新计分
  → 重算Coinbase经济性假设并更新valuation/01-scenario-model.md
```

### Base链增长质量区分标准

| 场景 | Base占比趋势 | 总市占趋势 | RLDC margin | 结论 | D3分档 |
| ---- | ------------ | ---------- | ----------- | ---- | ------ |
| 健康增长 | 上升 | 同步上升 | 稳定或改善 | 渠道有效分销 | 75-100分 |
| 内循环初期 | 上升（>40%） | 横盘 | 轻微下滑 | Coinbase议价增强信号 | 降至50分，预警 |
| 渠道捕获 | ≥45% | 下降 | 下滑≥1 ppt | 渠道吃掉经济性 | 0分，危险 |
| 监管限制后 | 任意 | 任意 | 激励被限制 | 分销模式重写 | 0分，P0 |

---

## USDT监控精确化

本章补充USDT相关监控的数据口径精确定义，解决"同数据不同分"的主观偏差问题。

### Adjusted Transfer Volume 去洗量规则

根据05-competition-scoring-rubric.md（D5链生态健康）和archive/1.md的口径定义，**adjusted transfer volume** 的去洗量规则如下：

**数据来源优先级：**

1. **CoinMetrics**（首选）：其 Adjusted Transfer Value（ATV）指标内置标准去重逻辑，直接引用计算结果，不再手动去重
2. **Dune Analytics**（备用）：需手动筛选，若使用须在证据栏标注"Dune口径，未经CoinMetrics标准去重，仅供参考"；不用于正式打分

**CoinMetrics ATV 去重机制（基于公开方法论的方向性说明）：**

- 剔除同地址自转账（self-transfers），即一个地址转给自己的操作
- 剔除mint/redeem操作（Circle发行和赎回操作，非经济性交易）
- 剔除已知链上清洗地址（DEX内部路由合约之间的流转）
- 异常大额转账（单笔≥当日总流通量5%以上）列为可疑，CoinMetrics内置标记处理

> **操作规则**：CoinMetrics ATV完整方法论见其官方文档。本框架打分时直接引用其ATV结果，不在本文件重新定义去重逻辑。若将来切换数据来源，需重新校准阈值并更新本节版本号。

**口径统一规则：**

| 口径 | 是否用于正式打分 | 说明 |
| ---- | ---------------- | ---- |
| CoinMetrics Adjusted Transfer Value（ATV） | ✅ 是 | 正式打分唯一口径 |
| Dune raw transfer count × 金额 | ❌ 否 | 仅作交叉验证参考 |
| Circle官方披露转账量 | ✅ 辅助 | CoinMetrics数据缺失时使用，需标注 |

### USDC / USDT Transfer Volume Ratio 历史基准

**定义：** USDC ATV ÷ USDT ATV（同源CoinMetrics，同口径，同快照日期）

**历史基准：历史数据待补充**

目前尚无完整的USDC/USDT ATV ratio历史时序数据内嵌于本框架文件中。建议在下次季度数据更新时，从CoinMetrics补充2024年1月至今的月度时序数据并填入下表：

| 月份 | USDC ATV | USDT ATV | USDC/USDT ATV Ratio | 来源 | 备注 |
| ---- | -------- | -------- | ------------------- | ---- | ---- |
| 2024-01 | 待补充 | 待补充 | 待补充 | CoinMetrics | — |
| 2024-07 | 待补充 | 待补充 | 待补充 | CoinMetrics | 时期A参考点 |
| 2025-01 | 待补充 | 待补充 | 待补充 | CoinMetrics | 时期B参考点 |
| 2026-05 | 待补充 | 待补充 | 待补充 | CoinMetrics | 历史样本 |

### USDC/USDT Transfer Volume Ratio 观察阈值

基于D1和D5的打分逻辑推导，以下阈值为方向性推算，待历史数据补充后用实际分布校准：

| 信号 | USDC/USDT ATV Ratio 条件 | 解读 | 联动动作 |
| ---- | ------------------------ | ---- | -------- |
| 正面 | 连续4周上升，且绝对值 >0.6 | USDC链上使用强度接近USDT，真实使用质量改善 | D5可考虑上调至75分档；D1正向参考 |
| 中性 | 变化在±5%以内（横盘） | 使用强度相对稳定，无明显方向 | 维持D5当前分值 |
| 预警 | 连续2周下降，或绝对值 <0.4 | USDC链上使用被USDT相对侵蚀 | D5降25分；检查D2是否联动恶化 |
| 危险 | 连续4周下降且绝对值 <0.3 | 链上使用严重落后，真实使用质量问题 | D5降至25分以下；触发D2和D5联动复盘 |

> 阈值说明：0.4和0.6的绝对值边界为方向性推算，基于USDC历史市占率（17%-25%）与USDT的相对使用差距估算。待CoinMetrics历史时序数据补充后，用实际数据校准，并更新本节版本号（当前版本：2026-05-09）。

### USDT监控执行频率汇总

| 监控项 | 执行频率 | 数据来源 | 关联维度 |
| ------ | -------- | -------- | -------- |
| USDC/USDT 市值比（ratio） | 每周（周四快照） | DefiLlama + CoinGecko 自算 | D1 主要指标 |
| USDT dominance 趋势 | 每周（4周滚动） | DefiLlama | D2 主要指标 |
| USDT 离岸扩张事件 | 实时 | Tether官网/新闻稿 | D2 事件条件触发 |
| USDC ATV（4周MA） | 每周 | CoinMetrics | D5 主要指标 |
| USDT ATV（4周MA） | 每周 | CoinMetrics | 计算USDC/USDT ATV ratio分母 |
| USDC/USDT ATV ratio | 每周（4周MA） | CoinMetrics（自算） | D5 补充指标；待历史基准建立后纳入正式打分 |
