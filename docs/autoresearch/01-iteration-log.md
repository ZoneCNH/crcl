# 1000 次 Autoresearch 迭代日志

## 结论

本文件把 CRCL 研究框架拆成 1000 个微实验。

每个微实验都有假设、评估指标、处置规则和文档落点。
后续更新时，只追加新实验，不覆盖历史记录。

## 迭代结果总表

|    # | 微实验假设                                                                   | 评估指标                                                  | 处置规则                         | 落点                                  |
| ---: | ---------------------------------------------------------------------------- | --------------------------------------------------------- | -------------------------------- | ------------------------------------- |
| 0001 | 官方源优先级需要显式高于二级平台                                             | SEC / Circle / Congress / OCC / Treasury 是否覆盖核心事实 | keep：写入来源纪律               | `docs/sources.md`                     |
| 0002 | 财报日期必须用绝对日期，不用“下周”                                           | Q1 2026 日期是否写成 2026-05-11                           | keep：所有财报节点用绝对日期     | `docs/README.md`                      |
| 0003 | GENIUS Act 状态必须区分法案和 Public Law                                     | Congress.gov 是否显示 Public Law No. 119-27               | keep：写入监管框架               | `framework/02-regulation.md`          |
| 0004 | OCC 规则必须标为拟议规则                                                     | OCC 页面日期和 NPRM 状态                                  | keep：禁止当最终规则             | `framework/02-regulation.md`          |
| 0005 | Treasury / FinCEN / OFAC 规则必须标为拟议规则                                | Treasury 2026-04-08 公告                                  | keep：写入监管风险               | `sources.md`                          |
| 0006 | Circle pressroom 与 SEC exhibit 要交叉验证                                   | earnings release 与 SEC exhibit 是否一致                  | keep：季度财报复核               | `metrics/03-quarterly-earnings.md`    |
| 0007 | 链上平台数据只能当趋势源                                                     | Dune / DefiLlama 是否有可变口径                           | keep：二级平台降权               | `metrics/00-metric-dictionary.md`     |
| 0008 | 媒体报道不能直接进入结论                                                     | 是否存在官方原文链接                                      | keep：媒体只做线索               | `sources.md`                          |
| 0009 | 每条关键事实必须带日期                                                       | 文档中监管、财报、主网节点是否有日期                      | revise：缺日期时补日期           | `docs/README.md`                      |
| 0010 | 原始文档必须保留但降权                                                       | archive 是否存在三份原始文档                              | keep：归档不作为最新结论         | `docs/archive/`                       |
| 0011 | CRCL 当前核心仍是储备经济性                                                  | Reserve income 占收入大头                                 | keep：按利差股主线研究           | `framework/01-business-model.md`      |
| 0012 | P/S 会高估收入质量                                                           | Distribution and transaction costs 占比                   | keep：估值锚转向 RLDC            | `valuation/00-valuation-framework.md` |
| 0013 | 平均 USDC 比期末 USDC 更能解释收入                                           | average USDC 与 reserve income                            | keep：财报第一优先级             | `metrics/03-quarterly-earnings.md`    |
| 0014 | Reserve return rate 比 Fed 标题更精确                                        | 财报 reserve return rate                                  | keep：季度复核                   | `metrics/03-quarterly-earnings.md`    |
| 0015 | 分销成本率是隐性核心指标                                                     | RLDC margin 是否低于 38%                                  | keep：设置降级阈值               | `framework/01-business-model.md`      |
| 0016 | Coinbase 增长不必然等于 Circle 盈利增强                                      | Base supply 与 RLDC margin 是否背离                       | keep：增长质量需财报验证         | `framework/03-competition.md`         |
| 0017 | Other revenue 低占比不能支撑平台股估值                                       | Other revenue share                                       | keep：>15% 才强验证              | `framework/04-platform-option.md`     |
| 0018 | Adjusted EBITDA 只能做辅助盈利锚                                             | 调整项是否影响可比性                                      | keep：不单独作为主锚             | `valuation/00-valuation-framework.md` |
| 0019 | FY 指引比单季新闻更重要                                                      | 指引上修 / 维持 / 下修                                    | keep：季度动作规则               | `metrics/03-quarterly-earnings.md`    |
| 0020 | 单季成本异常需要管理层解释                                                   | 成本异常是否一次性                                        | defer：等电话会问答              | `metrics/03-quarterly-earnings.md`    |
| 0021 | USDC 流通量是每日第一生命线                                                  | Circle Transparency                                       | keep：P0 日检                    | `metrics/01-daily-watchlist.md`       |
| 0022 | 7D 与 30D 同向下跌比单日变化更重要                                           | 7D / 30D change                                           | keep：连续性阈值                 | `metrics/00-metric-dictionary.md`     |
| 0023 | Minted / Redeemed 比净变化更能解释压力                                       | minted、redeemed 分项                                     | keep：日检记录                   | `metrics/01-daily-watchlist.md`       |
| 0024 | 连续 2-4 周净赎回是结构风险                                                  | net mint / redeem                                         | keep：预警信号                   | `risk/01-warning-signals.md`          |
| 0025 | USDC/USD 折价优先级高于估值                                                  | 二级市场锚定                                              | keep：失效条件                   | `risk/02-failure-conditions.md`       |
| 0026 | Curve pool imbalance 是赎回压力辅助信号                                      | Curve 池比例                                              | defer：数据源需固定              | `risk/00-risk-map.md`                 |
| 0027 | CEX order book depth 可用但不可稳定依赖                                      | Coinbase / Binance API                                    | defer：需要 API 权限             | `metrics/00-metric-dictionary.md`     |
| 0028 | 交易所 USDC 余额增长质量低于支付增长                                         | exchange balances                                         | keep：周检分类                   | `metrics/02-weekly-review.md`         |
| 0029 | 链上钱包分布能提示集中度风险                                                 | holder distribution                                       | defer：口径依赖 Dune 仪表盘      | `metrics/02-weekly-review.md`         |
| 0030 | 企业支付增长质量最高但披露不足                                               | CPN / 财报披露                                            | keep：P2 追踪                    | `framework/04-platform-option.md`     |
| 0031 | 3M T-Bill 是储备收益压力主代理                                               | Treasury / FRED                                           | keep：P0 日检                    | `metrics/01-daily-watchlist.md`       |
| 0032 | SOFR 是短端美元资金价格补充                                                  | NY Fed                                                    | keep：P0 日检                    | `metrics/00-metric-dictionary.md`     |
| 0033 | Circle Reserve Fund yield 比宏观利率更贴近实际                               | BlackRock fund yield                                      | keep：P0 监控                    | `metrics/00-metric-dictionary.md`     |
| 0034 | 降息不是线性利空                                                             | USDC 增速是否抵消收益率下行                               | keep：估值框架                   | `valuation/00-valuation-framework.md` |
| 0035 | 利率下行 + USDC 不增长是双杀                                                 | reserve return rate 与 average USDC                       | keep：Bear case                  | `valuation/01-scenario-model.md`      |
| 0036 | 高利率也可能推资金去收益型竞品                                               | RWA / USDe AUM                                            | keep：竞争监控                   | `framework/03-competition.md`         |
| 0037 | 储备 WAM / WAL 是尾部流动性指标                                              | BlackRock / SEC 报告                                      | defer：月度或季度获取            | `risk/00-risk-map.md`                 |
| 0038 | Daily / weekly liquidity 是监管合规指标                                      | 储备披露和规则要求                                        | keep：风险地图                   | `risk/00-risk-map.md`                 |
| 0039 | 银行通道约束是赎回风险放大器                                                 | banking partners 变化                                     | defer：事件驱动                  | `risk/02-failure-conditions.md`       |
| 0040 | Weekend redemption constraints 只能作风险提示                                | 服务条款与用户反馈                                        | reject：无稳定公开指标           | `risk/00-risk-map.md`                 |
| 0041 | 监管不是单向利好                                                             | 合规壁垒 vs 分销限制                                      | keep：监管框架核心结论           | `framework/02-regulation.md`          |
| 0042 | 非银行发行路径清晰度影响 Circle 护城河                                       | PPSI 路径                                                 | keep：监管矩阵                   | `framework/02-regulation.md`          |
| 0043 | 收益分享禁令影响平台叙事                                                     | interest / yield 条款                                     | keep：P0 风险                    | `framework/02-regulation.md`          |
| 0044 | 第三方激励是否被视为变相 yield 是最大观察点                                  | OCC / Treasury 最终规则                                   | keep：P0 事件规则                | `risk/01-warning-signals.md`          |
| 0045 | BSA/AML 成本会影响跨境支付效率                                               | FinCEN / OFAC 要求                                        | keep：CPN 风险                   | `framework/04-platform-option.md`     |
| 0046 | 州级路径可能影响小发行人竞争                                                 | Treasury state regime 规则                                | defer：看最终规则                | `framework/02-regulation.md`          |
| 0047 | SEC 执法会影响交易所分销                                                     | SEC newsroom                                              | keep：P0 事件源                  | `sources.md`                          |
| 0048 | CFTC 行动会影响稳定币商品属性预期                                            | CFTC pressroom                                            | keep：事件源                     | `sources.md`                          |
| 0049 | Fed 稳定币讲话影响银行参与预期                                               | Fed events                                                | keep：来源索引                   | `sources.md`                          |
| 0050 | 监管评论期结论不能提前当成最终规则                                           | NPRM 状态                                                 | keep：核验纪律                   | `sources.md`                          |
| 0051 | USDT 是规模竞争，不是唯一竞争                                                | USDC / USDT ratio                                         | keep：竞争框架                   | `framework/03-competition.md`         |
| 0052 | 稳定币总盘扩张决定行业顺风                                                   | total stablecoin market cap                               | keep：周检                       | `metrics/02-weekly-review.md`         |
| 0053 | USDC 市占率连续 4 周下降是竞争预警                                           | market share trend                                        | keep：风险信号                   | `risk/01-warning-signals.md`          |
| 0054 | Bank stablecoin 抢的是企业和机构心智                                         | 银行发行公告                                              | keep：竞争结构                   | `framework/03-competition.md`         |
| 0055 | tokenized deposits 会压缩长期倍数                                            | 银行代币化存款进展                                        | keep：估值折价项                 | `valuation/00-valuation-framework.md` |
| 0056 | PYUSD、FDUSD 是份额迁移观察项                                                | 市值与链上使用                                            | keep：P2 监控                    | `framework/03-competition.md`         |
| 0057 | USDe / sUSDe 是收益竞争                                                      | AUM 与收益率                                              | keep：周检                       | `metrics/02-weekly-review.md`         |
| 0058 | BUIDL / USDY / USYC 抢机构闲置美元                                           | RWA AUM                                                   | keep：竞争监控                   | `framework/03-competition.md`         |
| 0059 | DeFi deposits 增长需要拆存款和借款                                           | Aave / Compound deposit-borrow                            | keep：周检                       | `metrics/02-weekly-review.md`         |
| 0060 | Uniswap liquidity 是交易需求辅助指标                                         | USDC pair liquidity                                       | defer：数据源要固定              | `metrics/02-weekly-review.md`         |
| 0061 | CPN 参与机构数只能证明早期采用                                               | institutions joined                                       | keep：弱验证                     | `framework/04-platform-option.md`     |
| 0062 | CPN TPV 必须和收入同步验证                                                   | TPV 与 Other revenue                                      | keep：商业验证规则               | `framework/04-platform-option.md`     |
| 0063 | CPN 收入确认方式比 TPV 更重要                                                | 财报注释 / call transcript                                | keep：财报问答                   | `metrics/03-quarterly-earnings.md`    |
| 0064 | Arc 测试网交易不等于收入                                                     | testnet tx vs mainnet revenue                             | keep：平台化风险                 | `framework/04-platform-option.md`     |
| 0065 | Arc 主网时间表是期权兑现前置条件                                             | 2026 主网目标                                             | keep：P2 追踪                    | `framework/04-platform-option.md`     |
| 0066 | Arc fee model 决定是否有独立收入                                             | gas / validator / app fees                                | keep：财报问答                   | `metrics/03-quarterly-earnings.md`    |
| 0067 | API 调用量若不披露，不能给平台倍数                                           | API metrics                                               | defer：等披露                    | `framework/04-platform-option.md`     |
| 0068 | EURC 是可选上行，不是主框架核心                                              | EURC supply / revenue                                     | defer：规模不足时不加权          | `valuation/00-valuation-framework.md` |
| 0069 | USYC 等资产产品可能扩展 Other revenue                                        | product AUM / fees                                        | defer：等财报拆分                | `framework/04-platform-option.md`     |
| 0070 | 平台化必须避免一次性 integration services 误判                               | recurring vs one-time revenue                             | keep：保守规则                   | `framework/04-platform-option.md`     |
| 0071 | Market cap / USDC circulation 只能作粗锚                                     | 市值 / USDC 流通量                                        | keep：估值锚局限                 | `valuation/00-valuation-framework.md` |
| 0072 | Market cap / RLDC 更适合收入质量                                             | 市值 / annualized RLDC                                    | keep：主估值锚                   | `valuation/00-valuation-framework.md` |
| 0073 | EV / Adjusted EBITDA 用于传统对照                                            | EV / Adj. EBITDA                                          | keep：辅助锚                     | `valuation/00-valuation-framework.md` |
| 0074 | Other revenue multiple 必须等可重复性披露                                    | recurring Other revenue                                   | keep：期权盘                     | `valuation/00-valuation-framework.md` |
| 0075 | Bull case 必须同时满足基础盘和期权盘                                         | USDC、RLDC、Other revenue                                 | keep：情景模型                   | `valuation/01-scenario-model.md`      |
| 0076 | Base case 不能因单一产品公告上修                                             | 指标是否多线共振                                          | keep：情景纪律                   | `valuation/01-scenario-model.md`      |
| 0077 | Bear case 需要把监管和筹码共振纳入                                           | rules + unlock + IV                                       | keep：情景模型                   | `valuation/01-scenario-model.md`      |
| 0078 | 估值安全边际要扣除监管折价                                                   | safety margin formula                                     | keep：估值框架                   | `valuation/00-valuation-framework.md` |
| 0079 | 平台股切换必须设置硬阈值                                                     | Other revenue >15%                                        | keep：切换条件                   | `valuation/00-valuation-framework.md` |
| 0080 | 股价上涨不等于研究框架强化                                                   | 基本面指标是否同步                                        | keep：输出纪律                   | `playbook/01-decision-template.md`    |
| 0081 | 储备异常优先级高于所有估值讨论                                               | reserve report status                                     | keep：失效条件                   | `risk/02-failure-conditions.md`       |
| 0082 | USDC/USD 折价要立刻切信用框架                                                | peg deviation                                             | keep：失效条件                   | `risk/02-failure-conditions.md`       |
| 0083 | short interest 单独上升不是结构风险                                          | short interest + price/volume                             | keep：中等预警                   | `risk/01-warning-signals.md`          |
| 0084 | borrow fee 需要券商源，不适合默认框架                                        | IBKR / broker data                                        | reject：不可公共复盘             | `risk/01-warning-signals.md`          |
| 0085 | put/call ratio 和 IV 共振才有意义                                            | options data                                              | keep：筹码信号                   | `risk/01-warning-signals.md`          |
| 0086 | lock-up expiration 是事件风险                                                | SEC filings / prospectus                                  | keep：P2 监控                    | `risk/01-warning-signals.md`          |
| 0087 | secondary offering 直接影响稀释                                              | SEC filing                                                | keep：P0/P2 事件                 | `risk/01-warning-signals.md`          |
| 0088 | insider selling 必须看持续性和规模                                           | Form 4                                                    | keep：筹码风险                   | `risk/00-risk-map.md`                 |
| 0089 | institutional ownership 是慢变量                                             | 13F                                                       | defer：季度观察                  | `risk/00-risk-map.md`                 |
| 0090 | 技术均线只能放大风控，不改基本面                                             | 20D / 50D                                                 | keep：中等预警                   | `metrics/01-daily-watchlist.md`       |
| 0091 | 日检只能回答是否重估风险                                                     | P0 指标                                                   | keep：流程纪律                   | `playbook/00-research-routine.md`     |
| 0092 | 周检必须回答增长质量                                                         | supply + share + volume                                   | keep：周报模板                   | `metrics/02-weekly-review.md`         |
| 0093 | 季检必须先算 RLDC margin                                                     | 财报计算表                                                | keep：季报模板                   | `metrics/03-quarterly-earnings.md`    |
| 0094 | 事件流程必须先确认来源再判断                                                 | official source check                                     | keep：事件流程                   | `playbook/00-research-routine.md`     |
| 0095 | 输出必须先结论再依据再动作                                                   | decision template                                         | keep：输出模板                   | `playbook/01-decision-template.md`    |
| 0096 | 每条结论必须落到增强、降级、观察                                             | action class                                              | keep：入口纪律                   | `docs/README.md`                      |
| 0097 | 缺失数据要写 missing_info，而不是脑补                                        | unavailable metric                                        | keep：输出纪律                   | `playbook/01-decision-template.md`    |
| 0098 | 文档迭代要保留来源和日期                                                     | change log rule                                           | keep：更新规则                   | `docs/README.md`                      |
| 0099 | archive 只存历史，不参与最新判断                                             | archive path                                              | keep：目录职责                   | `docs/README.md`                      |
| 0100 | 阶段性迭代后仍需继续用同一目标函数                                           | research score                                            | keep：autoresearch 方法          | `autoresearch/00-loop.md`             |
| 0101 | 商业模式与收入质量：Reserve income 依赖度 需要用 财报数值 复核               | 财报数值 + Reserve income 依赖度                          | revise：写入财务桥或季报检查     | `framework/01-business-model.md`      |
| 0102 | 商业模式与收入质量：RLDC margin 稳定性 需要用 财报数值 复核                  | 财报数值 + RLDC margin 稳定性                             | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0103 | 商业模式与收入质量：Other revenue 可重复性 需要用 财报数值 复核              | 财报数值 + Other revenue 可重复性                         | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0104 | 商业模式与收入质量：average USDC 解释力 需要用 财报数值 复核                 | 财报数值 + average USDC 解释力                            | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0105 | 商业模式与收入质量：distribution cost 弹性 需要用 财报数值 复核              | 财报数值 + distribution cost 弹性                         | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0106 | 商业模式与收入质量：Adjusted EBITDA 转化 需要用 财报数值 复核                | 财报数值 + Adjusted EBITDA 转化                           | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0107 | 商业模式与收入质量：收入确认口径 需要用 财报数值 复核                        | 财报数值 + 收入确认口径                                   | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0108 | 商业模式与收入质量：成本一次性因素 需要用 财报数值 复核                      | 财报数值 + 成本一次性因素                                 | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0109 | 商业模式与收入质量：FY 指引质量 需要用 财报数值 复核                         | 财报数值 + FY 指引质量                                    | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0110 | 商业模式与收入质量：财报披露完整性 需要用 财报数值 复核                      | 财报数值 + 财报披露完整性                                 | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0111 | 商业模式与收入质量：Reserve income 依赖度 需要用 同比与环比 复核             | 同比与环比 + Reserve income 依赖度                        | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0112 | 商业模式与收入质量：RLDC margin 稳定性 需要用 同比与环比 复核                | 同比与环比 + RLDC margin 稳定性                           | revise：写入财务桥或季报检查     | `framework/01-business-model.md`      |
| 0113 | 商业模式与收入质量：Other revenue 可重复性 需要用 同比与环比 复核            | 同比与环比 + Other revenue 可重复性                       | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0114 | 商业模式与收入质量：average USDC 解释力 需要用 同比与环比 复核               | 同比与环比 + average USDC 解释力                          | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0115 | 商业模式与收入质量：distribution cost 弹性 需要用 同比与环比 复核            | 同比与环比 + distribution cost 弹性                       | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0116 | 商业模式与收入质量：Adjusted EBITDA 转化 需要用 同比与环比 复核              | 同比与环比 + Adjusted EBITDA 转化                         | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0117 | 商业模式与收入质量：收入确认口径 需要用 同比与环比 复核                      | 同比与环比 + 收入确认口径                                 | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0118 | 商业模式与收入质量：成本一次性因素 需要用 同比与环比 复核                    | 同比与环比 + 成本一次性因素                               | defer：写入财务桥或季报检查      | `framework/01-business-model.md`      |
| 0119 | 商业模式与收入质量：FY 指引质量 需要用 同比与环比 复核                       | 同比与环比 + FY 指引质量                                  | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0120 | 商业模式与收入质量：财报披露完整性 需要用 同比与环比 复核                    | 同比与环比 + 财报披露完整性                               | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0121 | 商业模式与收入质量：Reserve income 依赖度 需要用 管理层解释 复核             | 管理层解释 + Reserve income 依赖度                        | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0122 | 商业模式与收入质量：RLDC margin 稳定性 需要用 管理层解释 复核                | 管理层解释 + RLDC margin 稳定性                           | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0123 | 商业模式与收入质量：Other revenue 可重复性 需要用 管理层解释 复核            | 管理层解释 + Other revenue 可重复性                       | revise：写入财务桥或季报检查     | `framework/01-business-model.md`      |
| 0124 | 商业模式与收入质量：average USDC 解释力 需要用 管理层解释 复核               | 管理层解释 + average USDC 解释力                          | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0125 | 商业模式与收入质量：distribution cost 弹性 需要用 管理层解释 复核            | 管理层解释 + distribution cost 弹性                       | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0126 | 商业模式与收入质量：Adjusted EBITDA 转化 需要用 管理层解释 复核              | 管理层解释 + Adjusted EBITDA 转化                         | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0127 | 商业模式与收入质量：收入确认口径 需要用 管理层解释 复核                      | 管理层解释 + 收入确认口径                                 | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0128 | 商业模式与收入质量：成本一次性因素 需要用 管理层解释 复核                    | 管理层解释 + 成本一次性因素                               | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0129 | 商业模式与收入质量：FY 指引质量 需要用 管理层解释 复核                       | 管理层解释 + FY 指引质量                                  | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0130 | 商业模式与收入质量：财报披露完整性 需要用 管理层解释 复核                    | 管理层解释 + 财报披露完整性                               | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0131 | 商业模式与收入质量：Reserve income 依赖度 需要用 SEC filing 注释 复核        | SEC filing 注释 + Reserve income 依赖度                   | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0132 | 商业模式与收入质量：RLDC margin 稳定性 需要用 SEC filing 注释 复核           | SEC filing 注释 + RLDC margin 稳定性                      | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0133 | 商业模式与收入质量：Other revenue 可重复性 需要用 SEC filing 注释 复核       | SEC filing 注释 + Other revenue 可重复性                  | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0134 | 商业模式与收入质量：average USDC 解释力 需要用 SEC filing 注释 复核          | SEC filing 注释 + average USDC 解释力                     | revise：写入财务桥或季报检查     | `framework/01-business-model.md`      |
| 0135 | 商业模式与收入质量：distribution cost 弹性 需要用 SEC filing 注释 复核       | SEC filing 注释 + distribution cost 弹性                  | defer：写入财务桥或季报检查      | `framework/01-business-model.md`      |
| 0136 | 商业模式与收入质量：Adjusted EBITDA 转化 需要用 SEC filing 注释 复核         | SEC filing 注释 + Adjusted EBITDA 转化                    | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0137 | 商业模式与收入质量：收入确认口径 需要用 SEC filing 注释 复核                 | SEC filing 注释 + 收入确认口径                            | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0138 | 商业模式与收入质量：成本一次性因素 需要用 SEC filing 注释 复核               | SEC filing 注释 + 成本一次性因素                          | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0139 | 商业模式与收入质量：FY 指引质量 需要用 SEC filing 注释 复核                  | SEC filing 注释 + FY 指引质量                             | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0140 | 商业模式与收入质量：财报披露完整性 需要用 SEC filing 注释 复核               | SEC filing 注释 + 财报披露完整性                          | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0141 | 商业模式与收入质量：Reserve income 依赖度 需要用 季度阈值 复核               | 季度阈值 + Reserve income 依赖度                          | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0142 | 商业模式与收入质量：RLDC margin 稳定性 需要用 季度阈值 复核                  | 季度阈值 + RLDC margin 稳定性                             | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0143 | 商业模式与收入质量：Other revenue 可重复性 需要用 季度阈值 复核              | 季度阈值 + Other revenue 可重复性                         | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0144 | 商业模式与收入质量：average USDC 解释力 需要用 季度阈值 复核                 | 季度阈值 + average USDC 解释力                            | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0145 | 商业模式与收入质量：distribution cost 弹性 需要用 季度阈值 复核              | 季度阈值 + distribution cost 弹性                         | revise：写入财务桥或季报检查     | `framework/01-business-model.md`      |
| 0146 | 商业模式与收入质量：Adjusted EBITDA 转化 需要用 季度阈值 复核                | 季度阈值 + Adjusted EBITDA 转化                           | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0147 | 商业模式与收入质量：收入确认口径 需要用 季度阈值 复核                        | 季度阈值 + 收入确认口径                                   | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0148 | 商业模式与收入质量：成本一次性因素 需要用 季度阈值 复核                      | 季度阈值 + 成本一次性因素                                 | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0149 | 商业模式与收入质量：FY 指引质量 需要用 季度阈值 复核                         | 季度阈值 + FY 指引质量                                    | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0150 | 商业模式与收入质量：财报披露完整性 需要用 季度阈值 复核                      | 季度阈值 + 财报披露完整性                                 | keep：写入财务桥或季报检查       | `framework/01-business-model.md`      |
| 0151 | 分销成本与渠道议价：Coinbase 分成压力 需要用 RLDC margin 复核                | RLDC margin + Coinbase 分成压力                           | revise：保留为渠道风险检查       | `framework/03-competition.md`         |
| 0152 | 分销成本与渠道议价：Binance 渠道贡献 需要用 RLDC margin 复核                 | RLDC margin + Binance 渠道贡献                            | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0153 | 分销成本与渠道议价：Base 生态绑定 需要用 RLDC margin 复核                    | RLDC margin + Base 生态绑定                               | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0154 | 分销成本与渠道议价：USDC on Platform 占比 需要用 RLDC margin 复核            | RLDC margin + USDC on Platform 占比                       | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0155 | 分销成本与渠道议价：第三方钱包激励 需要用 RLDC margin 复核                   | RLDC margin + 第三方钱包激励                              | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0156 | 分销成本与渠道议价：交易所余额质量 需要用 RLDC margin 复核                   | RLDC margin + 交易所余额质量                              | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0157 | 分销成本与渠道议价：渠道集中度 需要用 RLDC margin 复核                       | RLDC margin + 渠道集中度                                  | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0158 | 分销成本与渠道议价：分销成本率 需要用 RLDC margin 复核                       | RLDC margin + 分销成本率                                  | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0159 | 分销成本与渠道议价：合作方垂直整合 需要用 RLDC margin 复核                   | RLDC margin + 合作方垂直整合                              | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0160 | 分销成本与渠道议价：分销协议变化 需要用 RLDC margin 复核                     | RLDC margin + 分销协议变化                                | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0161 | 分销成本与渠道议价：Coinbase 分成压力 需要用 合作方披露 复核                 | 合作方披露 + Coinbase 分成压力                            | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0162 | 分销成本与渠道议价：Binance 渠道贡献 需要用 合作方披露 复核                  | 合作方披露 + Binance 渠道贡献                             | revise：保留为渠道风险检查       | `framework/03-competition.md`         |
| 0163 | 分销成本与渠道议价：Base 生态绑定 需要用 合作方披露 复核                     | 合作方披露 + Base 生态绑定                                | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0164 | 分销成本与渠道议价：USDC on Platform 占比 需要用 合作方披露 复核             | 合作方披露 + USDC on Platform 占比                        | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0165 | 分销成本与渠道议价：第三方钱包激励 需要用 合作方披露 复核                    | 合作方披露 + 第三方钱包激励                               | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0166 | 分销成本与渠道议价：交易所余额质量 需要用 合作方披露 复核                    | 合作方披露 + 交易所余额质量                               | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0167 | 分销成本与渠道议价：渠道集中度 需要用 合作方披露 复核                        | 合作方披露 + 渠道集中度                                   | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0168 | 分销成本与渠道议价：分销成本率 需要用 合作方披露 复核                        | 合作方披露 + 分销成本率                                   | defer：保留为渠道风险检查        | `framework/03-competition.md`         |
| 0169 | 分销成本与渠道议价：合作方垂直整合 需要用 合作方披露 复核                    | 合作方披露 + 合作方垂直整合                               | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0170 | 分销成本与渠道议价：分销协议变化 需要用 合作方披露 复核                      | 合作方披露 + 分销协议变化                                 | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0171 | 分销成本与渠道议价：Coinbase 分成压力 需要用 渠道余额 复核                   | 渠道余额 + Coinbase 分成压力                              | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0172 | 分销成本与渠道议价：Binance 渠道贡献 需要用 渠道余额 复核                    | 渠道余额 + Binance 渠道贡献                               | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0173 | 分销成本与渠道议价：Base 生态绑定 需要用 渠道余额 复核                       | 渠道余额 + Base 生态绑定                                  | revise：保留为渠道风险检查       | `framework/03-competition.md`         |
| 0174 | 分销成本与渠道议价：USDC on Platform 占比 需要用 渠道余额 复核               | 渠道余额 + USDC on Platform 占比                          | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0175 | 分销成本与渠道议价：第三方钱包激励 需要用 渠道余额 复核                      | 渠道余额 + 第三方钱包激励                                 | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0176 | 分销成本与渠道议价：交易所余额质量 需要用 渠道余额 复核                      | 渠道余额 + 交易所余额质量                                 | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0177 | 分销成本与渠道议价：渠道集中度 需要用 渠道余额 复核                          | 渠道余额 + 渠道集中度                                     | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0178 | 分销成本与渠道议价：分销成本率 需要用 渠道余额 复核                          | 渠道余额 + 分销成本率                                     | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0179 | 分销成本与渠道议价：合作方垂直整合 需要用 渠道余额 复核                      | 渠道余额 + 合作方垂直整合                                 | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0180 | 分销成本与渠道议价：分销协议变化 需要用 渠道余额 复核                        | 渠道余额 + 分销协议变化                                   | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0181 | 分销成本与渠道议价：Coinbase 分成压力 需要用 财报问答 复核                   | 财报问答 + Coinbase 分成压力                              | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0182 | 分销成本与渠道议价：Binance 渠道贡献 需要用 财报问答 复核                    | 财报问答 + Binance 渠道贡献                               | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0183 | 分销成本与渠道议价：Base 生态绑定 需要用 财报问答 复核                       | 财报问答 + Base 生态绑定                                  | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0184 | 分销成本与渠道议价：USDC on Platform 占比 需要用 财报问答 复核               | 财报问答 + USDC on Platform 占比                          | revise：保留为渠道风险检查       | `framework/03-competition.md`         |
| 0185 | 分销成本与渠道议价：第三方钱包激励 需要用 财报问答 复核                      | 财报问答 + 第三方钱包激励                                 | defer：保留为渠道风险检查        | `framework/03-competition.md`         |
| 0186 | 分销成本与渠道议价：交易所余额质量 需要用 财报问答 复核                      | 财报问答 + 交易所余额质量                                 | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0187 | 分销成本与渠道议价：渠道集中度 需要用 财报问答 复核                          | 财报问答 + 渠道集中度                                     | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0188 | 分销成本与渠道议价：分销成本率 需要用 财报问答 复核                          | 财报问答 + 分销成本率                                     | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0189 | 分销成本与渠道议价：合作方垂直整合 需要用 财报问答 复核                      | 财报问答 + 合作方垂直整合                                 | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0190 | 分销成本与渠道议价：分销协议变化 需要用 财报问答 复核                        | 财报问答 + 分销协议变化                                   | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0191 | 分销成本与渠道议价：Coinbase 分成压力 需要用 监管限制 复核                   | 监管限制 + Coinbase 分成压力                              | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0192 | 分销成本与渠道议价：Binance 渠道贡献 需要用 监管限制 复核                    | 监管限制 + Binance 渠道贡献                               | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0193 | 分销成本与渠道议价：Base 生态绑定 需要用 监管限制 复核                       | 监管限制 + Base 生态绑定                                  | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0194 | 分销成本与渠道议价：USDC on Platform 占比 需要用 监管限制 复核               | 监管限制 + USDC on Platform 占比                          | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0195 | 分销成本与渠道议价：第三方钱包激励 需要用 监管限制 复核                      | 监管限制 + 第三方钱包激励                                 | revise：保留为渠道风险检查       | `framework/03-competition.md`         |
| 0196 | 分销成本与渠道议价：交易所余额质量 需要用 监管限制 复核                      | 监管限制 + 交易所余额质量                                 | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0197 | 分销成本与渠道议价：渠道集中度 需要用 监管限制 复核                          | 监管限制 + 渠道集中度                                     | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0198 | 分销成本与渠道议价：分销成本率 需要用 监管限制 复核                          | 监管限制 + 分销成本率                                     | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0199 | 分销成本与渠道议价：合作方垂直整合 需要用 监管限制 复核                      | 监管限制 + 合作方垂直整合                                 | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0200 | 分销成本与渠道议价：分销协议变化 需要用 监管限制 复核                        | 监管限制 + 分销协议变化                                   | keep：保留为渠道风险检查         | `framework/03-competition.md`         |
| 0201 | USDC 供应与赎回：circulating supply 需要用 Circle Transparency 复核          | Circle Transparency + circulating supply                  | revise：写入 P0 或 P1 监控       | `metrics/00-metric-dictionary.md`     |
| 0202 | USDC 供应与赎回：7D change 需要用 Circle Transparency 复核                   | Circle Transparency + 7D change                           | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0203 | USDC 供应与赎回：30D change 需要用 Circle Transparency 复核                  | Circle Transparency + 30D change                          | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0204 | USDC 供应与赎回：minted volume 需要用 Circle Transparency 复核               | Circle Transparency + minted volume                       | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0205 | USDC 供应与赎回：redeemed volume 需要用 Circle Transparency 复核             | Circle Transparency + redeemed volume                     | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0206 | USDC 供应与赎回：net mint 需要用 Circle Transparency 复核                    | Circle Transparency + net mint                            | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0207 | USDC 供应与赎回：redemption streak 需要用 Circle Transparency 复核           | Circle Transparency + redemption streak                   | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0208 | USDC 供应与赎回：supply seasonality 需要用 Circle Transparency 复核          | Circle Transparency + supply seasonality                  | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0209 | USDC 供应与赎回：USDC share 需要用 Circle Transparency 复核                  | Circle Transparency + USDC share                          | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0210 | USDC 供应与赎回：stablecoin total cap 需要用 Circle Transparency 复核        | Circle Transparency + stablecoin total cap                | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0211 | USDC 供应与赎回：circulating supply 需要用 周度变化 复核                     | 周度变化 + circulating supply                             | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0212 | USDC 供应与赎回：7D change 需要用 周度变化 复核                              | 周度变化 + 7D change                                      | revise：写入 P0 或 P1 监控       | `metrics/00-metric-dictionary.md`     |
| 0213 | USDC 供应与赎回：30D change 需要用 周度变化 复核                             | 周度变化 + 30D change                                     | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0214 | USDC 供应与赎回：minted volume 需要用 周度变化 复核                          | 周度变化 + minted volume                                  | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0215 | USDC 供应与赎回：redeemed volume 需要用 周度变化 复核                        | 周度变化 + redeemed volume                                | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0216 | USDC 供应与赎回：net mint 需要用 周度变化 复核                               | 周度变化 + net mint                                       | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0217 | USDC 供应与赎回：redemption streak 需要用 周度变化 复核                      | 周度变化 + redemption streak                              | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0218 | USDC 供应与赎回：supply seasonality 需要用 周度变化 复核                     | 周度变化 + supply seasonality                             | defer：写入 P0 或 P1 监控        | `metrics/00-metric-dictionary.md`     |
| 0219 | USDC 供应与赎回：USDC share 需要用 周度变化 复核                             | 周度变化 + USDC share                                     | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0220 | USDC 供应与赎回：stablecoin total cap 需要用 周度变化 复核                   | 周度变化 + stablecoin total cap                           | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0221 | USDC 供应与赎回：circulating supply 需要用 趋势连续性 复核                   | 趋势连续性 + circulating supply                           | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0222 | USDC 供应与赎回：7D change 需要用 趋势连续性 复核                            | 趋势连续性 + 7D change                                    | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0223 | USDC 供应与赎回：30D change 需要用 趋势连续性 复核                           | 趋势连续性 + 30D change                                   | revise：写入 P0 或 P1 监控       | `metrics/00-metric-dictionary.md`     |
| 0224 | USDC 供应与赎回：minted volume 需要用 趋势连续性 复核                        | 趋势连续性 + minted volume                                | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0225 | USDC 供应与赎回：redeemed volume 需要用 趋势连续性 复核                      | 趋势连续性 + redeemed volume                              | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0226 | USDC 供应与赎回：net mint 需要用 趋势连续性 复核                             | 趋势连续性 + net mint                                     | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0227 | USDC 供应与赎回：redemption streak 需要用 趋势连续性 复核                    | 趋势连续性 + redemption streak                            | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0228 | USDC 供应与赎回：supply seasonality 需要用 趋势连续性 复核                   | 趋势连续性 + supply seasonality                           | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0229 | USDC 供应与赎回：USDC share 需要用 趋势连续性 复核                           | 趋势连续性 + USDC share                                   | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0230 | USDC 供应与赎回：stablecoin total cap 需要用 趋势连续性 复核                 | 趋势连续性 + stablecoin total cap                         | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0231 | USDC 供应与赎回：circulating supply 需要用 同业对比 复核                     | 同业对比 + circulating supply                             | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0232 | USDC 供应与赎回：7D change 需要用 同业对比 复核                              | 同业对比 + 7D change                                      | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0233 | USDC 供应与赎回：30D change 需要用 同业对比 复核                             | 同业对比 + 30D change                                     | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0234 | USDC 供应与赎回：minted volume 需要用 同业对比 复核                          | 同业对比 + minted volume                                  | revise：写入 P0 或 P1 监控       | `metrics/00-metric-dictionary.md`     |
| 0235 | USDC 供应与赎回：redeemed volume 需要用 同业对比 复核                        | 同业对比 + redeemed volume                                | defer：写入 P0 或 P1 监控        | `metrics/00-metric-dictionary.md`     |
| 0236 | USDC 供应与赎回：net mint 需要用 同业对比 复核                               | 同业对比 + net mint                                       | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0237 | USDC 供应与赎回：redemption streak 需要用 同业对比 复核                      | 同业对比 + redemption streak                              | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0238 | USDC 供应与赎回：supply seasonality 需要用 同业对比 复核                     | 同业对比 + supply seasonality                             | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0239 | USDC 供应与赎回：USDC share 需要用 同业对比 复核                             | 同业对比 + USDC share                                     | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0240 | USDC 供应与赎回：stablecoin total cap 需要用 同业对比 复核                   | 同业对比 + stablecoin total cap                           | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0241 | USDC 供应与赎回：circulating supply 需要用 阈值触发 复核                     | 阈值触发 + circulating supply                             | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0242 | USDC 供应与赎回：7D change 需要用 阈值触发 复核                              | 阈值触发 + 7D change                                      | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0243 | USDC 供应与赎回：30D change 需要用 阈值触发 复核                             | 阈值触发 + 30D change                                     | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0244 | USDC 供应与赎回：minted volume 需要用 阈值触发 复核                          | 阈值触发 + minted volume                                  | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0245 | USDC 供应与赎回：redeemed volume 需要用 阈值触发 复核                        | 阈值触发 + redeemed volume                                | revise：写入 P0 或 P1 监控       | `metrics/00-metric-dictionary.md`     |
| 0246 | USDC 供应与赎回：net mint 需要用 阈值触发 复核                               | 阈值触发 + net mint                                       | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0247 | USDC 供应与赎回：redemption streak 需要用 阈值触发 复核                      | 阈值触发 + redemption streak                              | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0248 | USDC 供应与赎回：supply seasonality 需要用 阈值触发 复核                     | 阈值触发 + supply seasonality                             | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0249 | USDC 供应与赎回：USDC share 需要用 阈值触发 复核                             | 阈值触发 + USDC share                                     | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0250 | USDC 供应与赎回：stablecoin total cap 需要用 阈值触发 复核                   | 阈值触发 + stablecoin total cap                           | keep：写入 P0 或 P1 监控         | `metrics/00-metric-dictionary.md`     |
| 0251 | 赎回流动性与锚定：USDC/USD peg 需要用 价格锚定 复核                          | 价格锚定 + USDC/USD peg                                   | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0252 | 赎回流动性与锚定：Curve pool imbalance 需要用 价格锚定 复核                  | 价格锚定 + Curve pool imbalance                           | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0253 | 赎回流动性与锚定：CEX depth 需要用 价格锚定 复核                             | 价格锚定 + CEX depth                                      | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0254 | 赎回流动性与锚定：redeem delay 需要用 价格锚定 复核                          | 价格锚定 + redeem delay                                   | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0255 | 赎回流动性与锚定：reserve report timing 需要用 价格锚定 复核                 | 价格锚定 + reserve report timing                          | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0256 | 赎回流动性与锚定：banking partner status 需要用 价格锚定 复核                | 价格锚定 + banking partner status                         | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0257 | 赎回流动性与锚定：cash proportion 需要用 价格锚定 复核                       | 价格锚定 + cash proportion                                | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0258 | 赎回流动性与锚定：weekly liquidity 需要用 价格锚定 复核                      | 价格锚定 + weekly liquidity                               | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0259 | 赎回流动性与锚定：settlement window 需要用 价格锚定 复核                     | 价格锚定 + settlement window                              | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0260 | 赎回流动性与锚定：stress redemption 需要用 价格锚定 复核                     | 价格锚定 + stress redemption                              | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0261 | 赎回流动性与锚定：USDC/USD peg 需要用 池子比例 复核                          | 池子比例 + USDC/USD peg                                   | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0262 | 赎回流动性与锚定：Curve pool imbalance 需要用 池子比例 复核                  | 池子比例 + Curve pool imbalance                           | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0263 | 赎回流动性与锚定：CEX depth 需要用 池子比例 复核                             | 池子比例 + CEX depth                                      | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0264 | 赎回流动性与锚定：redeem delay 需要用 池子比例 复核                          | 池子比例 + redeem delay                                   | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0265 | 赎回流动性与锚定：reserve report timing 需要用 池子比例 复核                 | 池子比例 + reserve report timing                          | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0266 | 赎回流动性与锚定：banking partner status 需要用 池子比例 复核                | 池子比例 + banking partner status                         | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0267 | 赎回流动性与锚定：cash proportion 需要用 池子比例 复核                       | 池子比例 + cash proportion                                | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0268 | 赎回流动性与锚定：weekly liquidity 需要用 池子比例 复核                      | 池子比例 + weekly liquidity                               | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0269 | 赎回流动性与锚定：settlement window 需要用 池子比例 复核                     | 池子比例 + settlement window                              | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0270 | 赎回流动性与锚定：stress redemption 需要用 池子比例 复核                     | 池子比例 + stress redemption                              | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0271 | 赎回流动性与锚定：USDC/USD peg 需要用 官方说明 复核                          | 官方说明 + USDC/USD peg                                   | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0272 | 赎回流动性与锚定：Curve pool imbalance 需要用 官方说明 复核                  | 官方说明 + Curve pool imbalance                           | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0273 | 赎回流动性与锚定：CEX depth 需要用 官方说明 复核                             | 官方说明 + CEX depth                                      | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0274 | 赎回流动性与锚定：redeem delay 需要用 官方说明 复核                          | 官方说明 + redeem delay                                   | reject：无稳定源则只做观察       | `risk/02-failure-conditions.md`       |
| 0275 | 赎回流动性与锚定：reserve report timing 需要用 官方说明 复核                 | 官方说明 + reserve report timing                          | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0276 | 赎回流动性与锚定：banking partner status 需要用 官方说明 复核                | 官方说明 + banking partner status                         | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0277 | 赎回流动性与锚定：cash proportion 需要用 官方说明 复核                       | 官方说明 + cash proportion                                | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0278 | 赎回流动性与锚定：weekly liquidity 需要用 官方说明 复核                      | 官方说明 + weekly liquidity                               | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0279 | 赎回流动性与锚定：settlement window 需要用 官方说明 复核                     | 官方说明 + settlement window                              | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0280 | 赎回流动性与锚定：stress redemption 需要用 官方说明 复核                     | 官方说明 + stress redemption                              | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0281 | 赎回流动性与锚定：USDC/USD peg 需要用 链上流量 复核                          | 链上流量 + USDC/USD peg                                   | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0282 | 赎回流动性与锚定：Curve pool imbalance 需要用 链上流量 复核                  | 链上流量 + Curve pool imbalance                           | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0283 | 赎回流动性与锚定：CEX depth 需要用 链上流量 复核                             | 链上流量 + CEX depth                                      | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0284 | 赎回流动性与锚定：redeem delay 需要用 链上流量 复核                          | 链上流量 + redeem delay                                   | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0285 | 赎回流动性与锚定：reserve report timing 需要用 链上流量 复核                 | 链上流量 + reserve report timing                          | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0286 | 赎回流动性与锚定：banking partner status 需要用 链上流量 复核                | 链上流量 + banking partner status                         | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0287 | 赎回流动性与锚定：cash proportion 需要用 链上流量 复核                       | 链上流量 + cash proportion                                | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0288 | 赎回流动性与锚定：weekly liquidity 需要用 链上流量 复核                      | 链上流量 + weekly liquidity                               | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0289 | 赎回流动性与锚定：settlement window 需要用 链上流量 复核                     | 链上流量 + settlement window                              | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0290 | 赎回流动性与锚定：stress redemption 需要用 链上流量 复核                     | 链上流量 + stress redemption                              | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0291 | 赎回流动性与锚定：USDC/USD peg 需要用 赎回通道 复核                          | 赎回通道 + USDC/USD peg                                   | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0292 | 赎回流动性与锚定：Curve pool imbalance 需要用 赎回通道 复核                  | 赎回通道 + Curve pool imbalance                           | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0293 | 赎回流动性与锚定：CEX depth 需要用 赎回通道 复核                             | 赎回通道 + CEX depth                                      | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0294 | 赎回流动性与锚定：redeem delay 需要用 赎回通道 复核                          | 赎回通道 + redeem delay                                   | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0295 | 赎回流动性与锚定：reserve report timing 需要用 赎回通道 复核                 | 赎回通道 + reserve report timing                          | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0296 | 赎回流动性与锚定：banking partner status 需要用 赎回通道 复核                | 赎回通道 + banking partner status                         | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0297 | 赎回流动性与锚定：cash proportion 需要用 赎回通道 复核                       | 赎回通道 + cash proportion                                | reject：无稳定源则只做观察       | `risk/02-failure-conditions.md`       |
| 0298 | 赎回流动性与锚定：weekly liquidity 需要用 赎回通道 复核                      | 赎回通道 + weekly liquidity                               | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0299 | 赎回流动性与锚定：settlement window 需要用 赎回通道 复核                     | 赎回通道 + settlement window                              | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0300 | 赎回流动性与锚定：stress redemption 需要用 赎回通道 复核                     | 赎回通道 + stress redemption                              | defer：无稳定源则只做观察        | `risk/02-failure-conditions.md`       |
| 0301 | 利率与储备收益：3M T-Bill 需要用 Treasury 复核                               | Treasury + 3M T-Bill                                      | revise：写入利率敏感度           | `valuation/00-valuation-framework.md` |
| 0302 | 利率与储备收益：SOFR 需要用 Treasury 复核                                    | Treasury + SOFR                                           | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0303 | 利率与储备收益：reserve fund yield 需要用 Treasury 复核                      | Treasury + reserve fund yield                             | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0304 | 利率与储备收益：rate cut path 需要用 Treasury 复核                           | Treasury + rate cut path                                  | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0305 | 利率与储备收益：yield beta 需要用 Treasury 复核                              | Treasury + yield beta                                     | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0306 | 利率与储备收益：duration exposure 需要用 Treasury 复核                       | Treasury + duration exposure                              | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0307 | 利率与储备收益：gross reserve income 需要用 Treasury 复核                    | Treasury + gross reserve income                           | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0308 | 利率与储备收益：net reserve economics 需要用 Treasury 复核                   | Treasury + net reserve economics                          | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0309 | 利率与储备收益：rate lag 需要用 Treasury 复核                                | Treasury + rate lag                                       | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0310 | 利率与储备收益：interest sensitivity 需要用 Treasury 复核                    | Treasury + interest sensitivity                           | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0311 | 利率与储备收益：3M T-Bill 需要用 NY Fed 复核                                 | NY Fed + 3M T-Bill                                        | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0312 | 利率与储备收益：SOFR 需要用 NY Fed 复核                                      | NY Fed + SOFR                                             | revise：写入利率敏感度           | `valuation/00-valuation-framework.md` |
| 0313 | 利率与储备收益：reserve fund yield 需要用 NY Fed 复核                        | NY Fed + reserve fund yield                               | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0314 | 利率与储备收益：rate cut path 需要用 NY Fed 复核                             | NY Fed + rate cut path                                    | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0315 | 利率与储备收益：yield beta 需要用 NY Fed 复核                                | NY Fed + yield beta                                       | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0316 | 利率与储备收益：duration exposure 需要用 NY Fed 复核                         | NY Fed + duration exposure                                | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0317 | 利率与储备收益：gross reserve income 需要用 NY Fed 复核                      | NY Fed + gross reserve income                             | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0318 | 利率与储备收益：net reserve economics 需要用 NY Fed 复核                     | NY Fed + net reserve economics                            | defer：写入利率敏感度            | `valuation/00-valuation-framework.md` |
| 0319 | 利率与储备收益：rate lag 需要用 NY Fed 复核                                  | NY Fed + rate lag                                         | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0320 | 利率与储备收益：interest sensitivity 需要用 NY Fed 复核                      | NY Fed + interest sensitivity                             | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0321 | 利率与储备收益：3M T-Bill 需要用 BlackRock 复核                              | BlackRock + 3M T-Bill                                     | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0322 | 利率与储备收益：SOFR 需要用 BlackRock 复核                                   | BlackRock + SOFR                                          | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0323 | 利率与储备收益：reserve fund yield 需要用 BlackRock 复核                     | BlackRock + reserve fund yield                            | revise：写入利率敏感度           | `valuation/00-valuation-framework.md` |
| 0324 | 利率与储备收益：rate cut path 需要用 BlackRock 复核                          | BlackRock + rate cut path                                 | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0325 | 利率与储备收益：yield beta 需要用 BlackRock 复核                             | BlackRock + yield beta                                    | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0326 | 利率与储备收益：duration exposure 需要用 BlackRock 复核                      | BlackRock + duration exposure                             | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0327 | 利率与储备收益：gross reserve income 需要用 BlackRock 复核                   | BlackRock + gross reserve income                          | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0328 | 利率与储备收益：net reserve economics 需要用 BlackRock 复核                  | BlackRock + net reserve economics                         | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0329 | 利率与储备收益：rate lag 需要用 BlackRock 复核                               | BlackRock + rate lag                                      | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0330 | 利率与储备收益：interest sensitivity 需要用 BlackRock 复核                   | BlackRock + interest sensitivity                          | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0331 | 利率与储备收益：3M T-Bill 需要用 财报收益率 复核                             | 财报收益率 + 3M T-Bill                                    | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0332 | 利率与储备收益：SOFR 需要用 财报收益率 复核                                  | 财报收益率 + SOFR                                         | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0333 | 利率与储备收益：reserve fund yield 需要用 财报收益率 复核                    | 财报收益率 + reserve fund yield                           | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0334 | 利率与储备收益：rate cut path 需要用 财报收益率 复核                         | 财报收益率 + rate cut path                                | revise：写入利率敏感度           | `valuation/00-valuation-framework.md` |
| 0335 | 利率与储备收益：yield beta 需要用 财报收益率 复核                            | 财报收益率 + yield beta                                   | defer：写入利率敏感度            | `valuation/00-valuation-framework.md` |
| 0336 | 利率与储备收益：duration exposure 需要用 财报收益率 复核                     | 财报收益率 + duration exposure                            | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0337 | 利率与储备收益：gross reserve income 需要用 财报收益率 复核                  | 财报收益率 + gross reserve income                         | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0338 | 利率与储备收益：net reserve economics 需要用 财报收益率 复核                 | 财报收益率 + net reserve economics                        | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0339 | 利率与储备收益：rate lag 需要用 财报收益率 复核                              | 财报收益率 + rate lag                                     | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0340 | 利率与储备收益：interest sensitivity 需要用 财报收益率 复核                  | 财报收益率 + interest sensitivity                         | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0341 | 利率与储备收益：3M T-Bill 需要用 敏感度公式 复核                             | 敏感度公式 + 3M T-Bill                                    | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0342 | 利率与储备收益：SOFR 需要用 敏感度公式 复核                                  | 敏感度公式 + SOFR                                         | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0343 | 利率与储备收益：reserve fund yield 需要用 敏感度公式 复核                    | 敏感度公式 + reserve fund yield                           | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0344 | 利率与储备收益：rate cut path 需要用 敏感度公式 复核                         | 敏感度公式 + rate cut path                                | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0345 | 利率与储备收益：yield beta 需要用 敏感度公式 复核                            | 敏感度公式 + yield beta                                   | revise：写入利率敏感度           | `valuation/00-valuation-framework.md` |
| 0346 | 利率与储备收益：duration exposure 需要用 敏感度公式 复核                     | 敏感度公式 + duration exposure                            | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0347 | 利率与储备收益：gross reserve income 需要用 敏感度公式 复核                  | 敏感度公式 + gross reserve income                         | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0348 | 利率与储备收益：net reserve economics 需要用 敏感度公式 复核                 | 敏感度公式 + net reserve economics                        | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0349 | 利率与储备收益：rate lag 需要用 敏感度公式 复核                              | 敏感度公式 + rate lag                                     | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0350 | 利率与储备收益：interest sensitivity 需要用 敏感度公式 复核                  | 敏感度公式 + interest sensitivity                         | keep：写入利率敏感度             | `valuation/00-valuation-framework.md` |
| 0351 | 储备资产与银行通道：reserve assets breakdown 需要用 储备报告 复核            | 储备报告 + reserve assets breakdown                       | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0352 | 储备资产与银行通道：WAM 需要用 储备报告 复核                                 | 储备报告 + WAM                                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0353 | 储备资产与银行通道：WAL 需要用 储备报告 复核                                 | 储备报告 + WAL                                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0354 | 储备资产与银行通道：repo exposure 需要用 储备报告 复核                       | 储备报告 + repo exposure                                  | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0355 | 储备资产与银行通道：Treasury bill share 需要用 储备报告 复核                 | 储备报告 + Treasury bill share                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0356 | 储备资产与银行通道：cash share 需要用 储备报告 复核                          | 储备报告 + cash share                                     | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0357 | 储备资产与银行通道：custody bank 需要用 储备报告 复核                        | 储备报告 + custody bank                                   | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0358 | 储备资产与银行通道：BNY relationship 需要用 储备报告 复核                    | 储备报告 + BNY relationship                               | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0359 | 储备资产与银行通道：weekend liquidity 需要用 储备报告 复核                   | 储备报告 + weekend liquidity                              | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0360 | 储备资产与银行通道：audit timing 需要用 储备报告 复核                        | 储备报告 + audit timing                                   | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0361 | 储备资产与银行通道：reserve assets breakdown 需要用 基金报告 复核            | 基金报告 + reserve assets breakdown                       | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0362 | 储备资产与银行通道：WAM 需要用 基金报告 复核                                 | 基金报告 + WAM                                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0363 | 储备资产与银行通道：WAL 需要用 基金报告 复核                                 | 基金报告 + WAL                                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0364 | 储备资产与银行通道：repo exposure 需要用 基金报告 复核                       | 基金报告 + repo exposure                                  | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0365 | 储备资产与银行通道：Treasury bill share 需要用 基金报告 复核                 | 基金报告 + Treasury bill share                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0366 | 储备资产与银行通道：cash share 需要用 基金报告 复核                          | 基金报告 + cash share                                     | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0367 | 储备资产与银行通道：custody bank 需要用 基金报告 复核                        | 基金报告 + custody bank                                   | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0368 | 储备资产与银行通道：BNY relationship 需要用 基金报告 复核                    | 基金报告 + BNY relationship                               | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0369 | 储备资产与银行通道：weekend liquidity 需要用 基金报告 复核                   | 基金报告 + weekend liquidity                              | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0370 | 储备资产与银行通道：audit timing 需要用 基金报告 复核                        | 基金报告 + audit timing                                   | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0371 | 储备资产与银行通道：reserve assets breakdown 需要用 SEC filing 复核          | SEC filing + reserve assets breakdown                     | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0372 | 储备资产与银行通道：WAM 需要用 SEC filing 复核                               | SEC filing + WAM                                          | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0373 | 储备资产与银行通道：WAL 需要用 SEC filing 复核                               | SEC filing + WAL                                          | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0374 | 储备资产与银行通道：repo exposure 需要用 SEC filing 复核                     | SEC filing + repo exposure                                | reject：月度或事件驱动复核       | `risk/00-risk-map.md`                 |
| 0375 | 储备资产与银行通道：Treasury bill share 需要用 SEC filing 复核               | SEC filing + Treasury bill share                          | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0376 | 储备资产与银行通道：cash share 需要用 SEC filing 复核                        | SEC filing + cash share                                   | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0377 | 储备资产与银行通道：custody bank 需要用 SEC filing 复核                      | SEC filing + custody bank                                 | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0378 | 储备资产与银行通道：BNY relationship 需要用 SEC filing 复核                  | SEC filing + BNY relationship                             | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0379 | 储备资产与银行通道：weekend liquidity 需要用 SEC filing 复核                 | SEC filing + weekend liquidity                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0380 | 储备资产与银行通道：audit timing 需要用 SEC filing 复核                      | SEC filing + audit timing                                 | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0381 | 储备资产与银行通道：reserve assets breakdown 需要用 托管公告 复核            | 托管公告 + reserve assets breakdown                       | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0382 | 储备资产与银行通道：WAM 需要用 托管公告 复核                                 | 托管公告 + WAM                                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0383 | 储备资产与银行通道：WAL 需要用 托管公告 复核                                 | 托管公告 + WAL                                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0384 | 储备资产与银行通道：repo exposure 需要用 托管公告 复核                       | 托管公告 + repo exposure                                  | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0385 | 储备资产与银行通道：Treasury bill share 需要用 托管公告 复核                 | 托管公告 + Treasury bill share                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0386 | 储备资产与银行通道：cash share 需要用 托管公告 复核                          | 托管公告 + cash share                                     | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0387 | 储备资产与银行通道：custody bank 需要用 托管公告 复核                        | 托管公告 + custody bank                                   | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0388 | 储备资产与银行通道：BNY relationship 需要用 托管公告 复核                    | 托管公告 + BNY relationship                               | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0389 | 储备资产与银行通道：weekend liquidity 需要用 托管公告 复核                   | 托管公告 + weekend liquidity                              | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0390 | 储备资产与银行通道：audit timing 需要用 托管公告 复核                        | 托管公告 + audit timing                                   | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0391 | 储备资产与银行通道：reserve assets breakdown 需要用 异常延迟 复核            | 异常延迟 + reserve assets breakdown                       | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0392 | 储备资产与银行通道：WAM 需要用 异常延迟 复核                                 | 异常延迟 + WAM                                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0393 | 储备资产与银行通道：WAL 需要用 异常延迟 复核                                 | 异常延迟 + WAL                                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0394 | 储备资产与银行通道：repo exposure 需要用 异常延迟 复核                       | 异常延迟 + repo exposure                                  | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0395 | 储备资产与银行通道：Treasury bill share 需要用 异常延迟 复核                 | 异常延迟 + Treasury bill share                            | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0396 | 储备资产与银行通道：cash share 需要用 异常延迟 复核                          | 异常延迟 + cash share                                     | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0397 | 储备资产与银行通道：custody bank 需要用 异常延迟 复核                        | 异常延迟 + custody bank                                   | reject：月度或事件驱动复核       | `risk/00-risk-map.md`                 |
| 0398 | 储备资产与银行通道：BNY relationship 需要用 异常延迟 复核                    | 异常延迟 + BNY relationship                               | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0399 | 储备资产与银行通道：weekend liquidity 需要用 异常延迟 复核                   | 异常延迟 + weekend liquidity                              | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0400 | 储备资产与银行通道：audit timing 需要用 异常延迟 复核                        | 异常延迟 + audit timing                                   | defer：月度或事件驱动复核        | `risk/00-risk-map.md`                 |
| 0401 | GENIUS Act 与监管路径：PPSI path 需要用 Congress 复核                        | Congress + PPSI path                                      | revise：写入监管矩阵             | `framework/02-regulation.md`          |
| 0402 | GENIUS Act 与监管路径：state issuer path 需要用 Congress 复核                | Congress + state issuer path                              | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0403 | GENIUS Act 与监管路径：reserve asset rule 需要用 Congress 复核               | Congress + reserve asset rule                             | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0404 | GENIUS Act 与监管路径：capital requirement 需要用 Congress 复核              | Congress + capital requirement                            | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0405 | GENIUS Act 与监管路径：disclosure rule 需要用 Congress 复核                  | Congress + disclosure rule                                | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0406 | GENIUS Act 与监管路径：audit requirement 需要用 Congress 复核                | Congress + audit requirement                              | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0407 | GENIUS Act 与监管路径：interest ban 需要用 Congress 复核                     | Congress + interest ban                                   | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0408 | GENIUS Act 与监管路径：yield workaround 需要用 Congress 复核                 | Congress + yield workaround                               | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0409 | GENIUS Act 与监管路径：nonbank issuer 需要用 Congress 复核                   | Congress + nonbank issuer                                 | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0410 | GENIUS Act 与监管路径：bank advantage 需要用 Congress 复核                   | Congress + bank advantage                                 | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0411 | GENIUS Act 与监管路径：PPSI path 需要用 OCC NPRM 复核                        | OCC NPRM + PPSI path                                      | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0412 | GENIUS Act 与监管路径：state issuer path 需要用 OCC NPRM 复核                | OCC NPRM + state issuer path                              | revise：写入监管矩阵             | `framework/02-regulation.md`          |
| 0413 | GENIUS Act 与监管路径：reserve asset rule 需要用 OCC NPRM 复核               | OCC NPRM + reserve asset rule                             | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0414 | GENIUS Act 与监管路径：capital requirement 需要用 OCC NPRM 复核              | OCC NPRM + capital requirement                            | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0415 | GENIUS Act 与监管路径：disclosure rule 需要用 OCC NPRM 复核                  | OCC NPRM + disclosure rule                                | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0416 | GENIUS Act 与监管路径：audit requirement 需要用 OCC NPRM 复核                | OCC NPRM + audit requirement                              | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0417 | GENIUS Act 与监管路径：interest ban 需要用 OCC NPRM 复核                     | OCC NPRM + interest ban                                   | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0418 | GENIUS Act 与监管路径：yield workaround 需要用 OCC NPRM 复核                 | OCC NPRM + yield workaround                               | defer：写入监管矩阵              | `framework/02-regulation.md`          |
| 0419 | GENIUS Act 与监管路径：nonbank issuer 需要用 OCC NPRM 复核                   | OCC NPRM + nonbank issuer                                 | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0420 | GENIUS Act 与监管路径：bank advantage 需要用 OCC NPRM 复核                   | OCC NPRM + bank advantage                                 | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0421 | GENIUS Act 与监管路径：PPSI path 需要用 最终规则 复核                        | 最终规则 + PPSI path                                      | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0422 | GENIUS Act 与监管路径：state issuer path 需要用 最终规则 复核                | 最终规则 + state issuer path                              | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0423 | GENIUS Act 与监管路径：reserve asset rule 需要用 最终规则 复核               | 最终规则 + reserve asset rule                             | revise：写入监管矩阵             | `framework/02-regulation.md`          |
| 0424 | GENIUS Act 与监管路径：capital requirement 需要用 最终规则 复核              | 最终规则 + capital requirement                            | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0425 | GENIUS Act 与监管路径：disclosure rule 需要用 最终规则 复核                  | 最终规则 + disclosure rule                                | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0426 | GENIUS Act 与监管路径：audit requirement 需要用 最终规则 复核                | 最终规则 + audit requirement                              | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0427 | GENIUS Act 与监管路径：interest ban 需要用 最终规则 复核                     | 最终规则 + interest ban                                   | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0428 | GENIUS Act 与监管路径：yield workaround 需要用 最终规则 复核                 | 最终规则 + yield workaround                               | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0429 | GENIUS Act 与监管路径：nonbank issuer 需要用 最终规则 复核                   | 最终规则 + nonbank issuer                                 | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0430 | GENIUS Act 与监管路径：bank advantage 需要用 最终规则 复核                   | 最终规则 + bank advantage                                 | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0431 | GENIUS Act 与监管路径：PPSI path 需要用 评论期变化 复核                      | 评论期变化 + PPSI path                                    | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0432 | GENIUS Act 与监管路径：state issuer path 需要用 评论期变化 复核              | 评论期变化 + state issuer path                            | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0433 | GENIUS Act 与监管路径：reserve asset rule 需要用 评论期变化 复核             | 评论期变化 + reserve asset rule                           | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0434 | GENIUS Act 与监管路径：capital requirement 需要用 评论期变化 复核            | 评论期变化 + capital requirement                          | revise：写入监管矩阵             | `framework/02-regulation.md`          |
| 0435 | GENIUS Act 与监管路径：disclosure rule 需要用 评论期变化 复核                | 评论期变化 + disclosure rule                              | defer：写入监管矩阵              | `framework/02-regulation.md`          |
| 0436 | GENIUS Act 与监管路径：audit requirement 需要用 评论期变化 复核              | 评论期变化 + audit requirement                            | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0437 | GENIUS Act 与监管路径：interest ban 需要用 评论期变化 复核                   | 评论期变化 + interest ban                                 | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0438 | GENIUS Act 与监管路径：yield workaround 需要用 评论期变化 复核               | 评论期变化 + yield workaround                             | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0439 | GENIUS Act 与监管路径：nonbank issuer 需要用 评论期变化 复核                 | 评论期变化 + nonbank issuer                               | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0440 | GENIUS Act 与监管路径：bank advantage 需要用 评论期变化 复核                 | 评论期变化 + bank advantage                               | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0441 | GENIUS Act 与监管路径：PPSI path 需要用 合规成本 复核                        | 合规成本 + PPSI path                                      | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0442 | GENIUS Act 与监管路径：state issuer path 需要用 合规成本 复核                | 合规成本 + state issuer path                              | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0443 | GENIUS Act 与监管路径：reserve asset rule 需要用 合规成本 复核               | 合规成本 + reserve asset rule                             | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0444 | GENIUS Act 与监管路径：capital requirement 需要用 合规成本 复核              | 合规成本 + capital requirement                            | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0445 | GENIUS Act 与监管路径：disclosure rule 需要用 合规成本 复核                  | 合规成本 + disclosure rule                                | revise：写入监管矩阵             | `framework/02-regulation.md`          |
| 0446 | GENIUS Act 与监管路径：audit requirement 需要用 合规成本 复核                | 合规成本 + audit requirement                              | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0447 | GENIUS Act 与监管路径：interest ban 需要用 合规成本 复核                     | 合规成本 + interest ban                                   | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0448 | GENIUS Act 与监管路径：yield workaround 需要用 合规成本 复核                 | 合规成本 + yield workaround                               | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0449 | GENIUS Act 与监管路径：nonbank issuer 需要用 合规成本 复核                   | 合规成本 + nonbank issuer                                 | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0450 | GENIUS Act 与监管路径：bank advantage 需要用 合规成本 复核                   | 合规成本 + bank advantage                                 | keep：写入监管矩阵               | `framework/02-regulation.md`          |
| 0451 | BSA AML 与制裁合规：KYC burden 需要用 Treasury 复核                          | Treasury + KYC burden                                     | revise：写入 CPN 风险            | `framework/04-platform-option.md`     |
| 0452 | BSA AML 与制裁合规：transaction monitoring 需要用 Treasury 复核              | Treasury + transaction monitoring                         | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0453 | BSA AML 与制裁合规：sanctions screening 需要用 Treasury 复核                 | Treasury + sanctions screening                            | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0454 | BSA AML 与制裁合规：wallet compliance 需要用 Treasury 复核                   | Treasury + wallet compliance                              | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0455 | BSA AML 与制裁合规：cross-border payment 需要用 Treasury 复核                | Treasury + cross-border payment                           | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0456 | BSA AML 与制裁合规：FinCEN reporting 需要用 Treasury 复核                    | Treasury + FinCEN reporting                               | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0457 | BSA AML 与制裁合规：OFAC list handling 需要用 Treasury 复核                  | Treasury + OFAC list handling                             | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0458 | BSA AML 与制裁合规：suspicious activity 需要用 Treasury 复核                 | Treasury + suspicious activity                            | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0459 | BSA AML 与制裁合规：privacy tradeoff 需要用 Treasury 复核                    | Treasury + privacy tradeoff                               | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0460 | BSA AML 与制裁合规：partner onboarding 需要用 Treasury 复核                  | Treasury + partner onboarding                             | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0461 | BSA AML 与制裁合规：KYC burden 需要用 FinCEN 复核                            | FinCEN + KYC burden                                       | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0462 | BSA AML 与制裁合规：transaction monitoring 需要用 FinCEN 复核                | FinCEN + transaction monitoring                           | revise：写入 CPN 风险            | `framework/04-platform-option.md`     |
| 0463 | BSA AML 与制裁合规：sanctions screening 需要用 FinCEN 复核                   | FinCEN + sanctions screening                              | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0464 | BSA AML 与制裁合规：wallet compliance 需要用 FinCEN 复核                     | FinCEN + wallet compliance                                | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0465 | BSA AML 与制裁合规：cross-border payment 需要用 FinCEN 复核                  | FinCEN + cross-border payment                             | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0466 | BSA AML 与制裁合规：FinCEN reporting 需要用 FinCEN 复核                      | FinCEN + FinCEN reporting                                 | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0467 | BSA AML 与制裁合规：OFAC list handling 需要用 FinCEN 复核                    | FinCEN + OFAC list handling                               | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0468 | BSA AML 与制裁合规：suspicious activity 需要用 FinCEN 复核                   | FinCEN + suspicious activity                              | defer：写入 CPN 风险             | `framework/04-platform-option.md`     |
| 0469 | BSA AML 与制裁合规：privacy tradeoff 需要用 FinCEN 复核                      | FinCEN + privacy tradeoff                                 | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0470 | BSA AML 与制裁合规：partner onboarding 需要用 FinCEN 复核                    | FinCEN + partner onboarding                               | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0471 | BSA AML 与制裁合规：KYC burden 需要用 OFAC 复核                              | OFAC + KYC burden                                         | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0472 | BSA AML 与制裁合规：transaction monitoring 需要用 OFAC 复核                  | OFAC + transaction monitoring                             | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0473 | BSA AML 与制裁合规：sanctions screening 需要用 OFAC 复核                     | OFAC + sanctions screening                                | revise：写入 CPN 风险            | `framework/04-platform-option.md`     |
| 0474 | BSA AML 与制裁合规：wallet compliance 需要用 OFAC 复核                       | OFAC + wallet compliance                                  | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0475 | BSA AML 与制裁合规：cross-border payment 需要用 OFAC 复核                    | OFAC + cross-border payment                               | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0476 | BSA AML 与制裁合规：FinCEN reporting 需要用 OFAC 复核                        | OFAC + FinCEN reporting                                   | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0477 | BSA AML 与制裁合规：OFAC list handling 需要用 OFAC 复核                      | OFAC + OFAC list handling                                 | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0478 | BSA AML 与制裁合规：suspicious activity 需要用 OFAC 复核                     | OFAC + suspicious activity                                | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0479 | BSA AML 与制裁合规：privacy tradeoff 需要用 OFAC 复核                        | OFAC + privacy tradeoff                                   | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0480 | BSA AML 与制裁合规：partner onboarding 需要用 OFAC 复核                      | OFAC + partner onboarding                                 | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0481 | BSA AML 与制裁合规：KYC burden 需要用 合规系统成本 复核                      | 合规系统成本 + KYC burden                                 | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0482 | BSA AML 与制裁合规：transaction monitoring 需要用 合规系统成本 复核          | 合规系统成本 + transaction monitoring                     | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0483 | BSA AML 与制裁合规：sanctions screening 需要用 合规系统成本 复核             | 合规系统成本 + sanctions screening                        | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0484 | BSA AML 与制裁合规：wallet compliance 需要用 合规系统成本 复核               | 合规系统成本 + wallet compliance                          | revise：写入 CPN 风险            | `framework/04-platform-option.md`     |
| 0485 | BSA AML 与制裁合规：cross-border payment 需要用 合规系统成本 复核            | 合规系统成本 + cross-border payment                       | defer：写入 CPN 风险             | `framework/04-platform-option.md`     |
| 0486 | BSA AML 与制裁合规：FinCEN reporting 需要用 合规系统成本 复核                | 合规系统成本 + FinCEN reporting                           | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0487 | BSA AML 与制裁合规：OFAC list handling 需要用 合规系统成本 复核              | 合规系统成本 + OFAC list handling                         | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0488 | BSA AML 与制裁合规：suspicious activity 需要用 合规系统成本 复核             | 合规系统成本 + suspicious activity                        | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0489 | BSA AML 与制裁合规：privacy tradeoff 需要用 合规系统成本 复核                | 合规系统成本 + privacy tradeoff                           | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0490 | BSA AML 与制裁合规：partner onboarding 需要用 合规系统成本 复核              | 合规系统成本 + partner onboarding                         | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0491 | BSA AML 与制裁合规：KYC burden 需要用 客户摩擦 复核                          | 客户摩擦 + KYC burden                                     | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0492 | BSA AML 与制裁合规：transaction monitoring 需要用 客户摩擦 复核              | 客户摩擦 + transaction monitoring                         | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0493 | BSA AML 与制裁合规：sanctions screening 需要用 客户摩擦 复核                 | 客户摩擦 + sanctions screening                            | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0494 | BSA AML 与制裁合规：wallet compliance 需要用 客户摩擦 复核                   | 客户摩擦 + wallet compliance                              | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0495 | BSA AML 与制裁合规：cross-border payment 需要用 客户摩擦 复核                | 客户摩擦 + cross-border payment                           | revise：写入 CPN 风险            | `framework/04-platform-option.md`     |
| 0496 | BSA AML 与制裁合规：FinCEN reporting 需要用 客户摩擦 复核                    | 客户摩擦 + FinCEN reporting                               | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0497 | BSA AML 与制裁合规：OFAC list handling 需要用 客户摩擦 复核                  | 客户摩擦 + OFAC list handling                             | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0498 | BSA AML 与制裁合规：suspicious activity 需要用 客户摩擦 复核                 | 客户摩擦 + suspicious activity                            | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0499 | BSA AML 与制裁合规：privacy tradeoff 需要用 客户摩擦 复核                    | 客户摩擦 + privacy tradeoff                               | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0500 | BSA AML 与制裁合规：partner onboarding 需要用 客户摩擦 复核                  | 客户摩擦 + partner onboarding                             | keep：写入 CPN 风险              | `framework/04-platform-option.md`     |
| 0501 | Coinbase 与渠道分发：Base USDC supply 需要用 Coinbase filing 复核            | Coinbase filing + Base USDC supply                        | revise：保留为渠道议价检查       | `framework/03-competition.md`         |
| 0502 | Coinbase 与渠道分发：Coinbase stablecoin revenue 需要用 Coinbase filing 复核 | Coinbase filing + Coinbase stablecoin revenue             | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0503 | Coinbase 与渠道分发：Coinbase wallet flow 需要用 Coinbase filing 复核        | Coinbase filing + Coinbase wallet flow                    | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0504 | Coinbase 与渠道分发：USDC rewards 需要用 Coinbase filing 复核                | Coinbase filing + USDC rewards                            | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0505 | Coinbase 与渠道分发：Coinbase custody 需要用 Coinbase filing 复核            | Coinbase filing + Coinbase custody                        | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0506 | Coinbase 与渠道分发：Coinbase user growth 需要用 Coinbase filing 复核        | Coinbase filing + Coinbase user growth                    | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0507 | Coinbase 与渠道分发：Base fee capture 需要用 Coinbase filing 复核            | Coinbase filing + Base fee capture                        | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0508 | Coinbase 与渠道分发：Coinbase bargaining 需要用 Coinbase filing 复核         | Coinbase filing + Coinbase bargaining                     | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0509 | Coinbase 与渠道分发：ecosystem lock-in 需要用 Coinbase filing 复核           | Coinbase filing + ecosystem lock-in                       | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0510 | Coinbase 与渠道分发：distribution dependence 需要用 Coinbase filing 复核     | Coinbase filing + distribution dependence                 | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0511 | Coinbase 与渠道分发：Base USDC supply 需要用 Base 链数据 复核                | Base 链数据 + Base USDC supply                            | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0512 | Coinbase 与渠道分发：Coinbase stablecoin revenue 需要用 Base 链数据 复核     | Base 链数据 + Coinbase stablecoin revenue                 | revise：保留为渠道议价检查       | `framework/03-competition.md`         |
| 0513 | Coinbase 与渠道分发：Coinbase wallet flow 需要用 Base 链数据 复核            | Base 链数据 + Coinbase wallet flow                        | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0514 | Coinbase 与渠道分发：USDC rewards 需要用 Base 链数据 复核                    | Base 链数据 + USDC rewards                                | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0515 | Coinbase 与渠道分发：Coinbase custody 需要用 Base 链数据 复核                | Base 链数据 + Coinbase custody                            | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0516 | Coinbase 与渠道分发：Coinbase user growth 需要用 Base 链数据 复核            | Base 链数据 + Coinbase user growth                        | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0517 | Coinbase 与渠道分发：Base fee capture 需要用 Base 链数据 复核                | Base 链数据 + Base fee capture                            | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0518 | Coinbase 与渠道分发：Coinbase bargaining 需要用 Base 链数据 复核             | Base 链数据 + Coinbase bargaining                         | defer：保留为渠道议价检查        | `framework/03-competition.md`         |
| 0519 | Coinbase 与渠道分发：ecosystem lock-in 需要用 Base 链数据 复核               | Base 链数据 + ecosystem lock-in                           | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0520 | Coinbase 与渠道分发：distribution dependence 需要用 Base 链数据 复核         | Base 链数据 + distribution dependence                     | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0521 | Coinbase 与渠道分发：Base USDC supply 需要用 RLDC margin 复核                | RLDC margin + Base USDC supply                            | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0522 | Coinbase 与渠道分发：Coinbase stablecoin revenue 需要用 RLDC margin 复核     | RLDC margin + Coinbase stablecoin revenue                 | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0523 | Coinbase 与渠道分发：Coinbase wallet flow 需要用 RLDC margin 复核            | RLDC margin + Coinbase wallet flow                        | revise：保留为渠道议价检查       | `framework/03-competition.md`         |
| 0524 | Coinbase 与渠道分发：USDC rewards 需要用 RLDC margin 复核                    | RLDC margin + USDC rewards                                | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0525 | Coinbase 与渠道分发：Coinbase custody 需要用 RLDC margin 复核                | RLDC margin + Coinbase custody                            | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0526 | Coinbase 与渠道分发：Coinbase user growth 需要用 RLDC margin 复核            | RLDC margin + Coinbase user growth                        | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0527 | Coinbase 与渠道分发：Base fee capture 需要用 RLDC margin 复核                | RLDC margin + Base fee capture                            | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0528 | Coinbase 与渠道分发：Coinbase bargaining 需要用 RLDC margin 复核             | RLDC margin + Coinbase bargaining                         | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0529 | Coinbase 与渠道分发：ecosystem lock-in 需要用 RLDC margin 复核               | RLDC margin + ecosystem lock-in                           | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0530 | Coinbase 与渠道分发：distribution dependence 需要用 RLDC margin 复核         | RLDC margin + distribution dependence                     | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0531 | Coinbase 与渠道分发：Base USDC supply 需要用 监管口径 复核                   | 监管口径 + Base USDC supply                               | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0532 | Coinbase 与渠道分发：Coinbase stablecoin revenue 需要用 监管口径 复核        | 监管口径 + Coinbase stablecoin revenue                    | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0533 | Coinbase 与渠道分发：Coinbase wallet flow 需要用 监管口径 复核               | 监管口径 + Coinbase wallet flow                           | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0534 | Coinbase 与渠道分发：USDC rewards 需要用 监管口径 复核                       | 监管口径 + USDC rewards                                   | revise：保留为渠道议价检查       | `framework/03-competition.md`         |
| 0535 | Coinbase 与渠道分发：Coinbase custody 需要用 监管口径 复核                   | 监管口径 + Coinbase custody                               | defer：保留为渠道议价检查        | `framework/03-competition.md`         |
| 0536 | Coinbase 与渠道分发：Coinbase user growth 需要用 监管口径 复核               | 监管口径 + Coinbase user growth                           | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0537 | Coinbase 与渠道分发：Base fee capture 需要用 监管口径 复核                   | 监管口径 + Base fee capture                               | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0538 | Coinbase 与渠道分发：Coinbase bargaining 需要用 监管口径 复核                | 监管口径 + Coinbase bargaining                            | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0539 | Coinbase 与渠道分发：ecosystem lock-in 需要用 监管口径 复核                  | 监管口径 + ecosystem lock-in                              | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0540 | Coinbase 与渠道分发：distribution dependence 需要用 监管口径 复核            | 监管口径 + distribution dependence                        | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0541 | Coinbase 与渠道分发：Base USDC supply 需要用 收入分成 复核                   | 收入分成 + Base USDC supply                               | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0542 | Coinbase 与渠道分发：Coinbase stablecoin revenue 需要用 收入分成 复核        | 收入分成 + Coinbase stablecoin revenue                    | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0543 | Coinbase 与渠道分发：Coinbase wallet flow 需要用 收入分成 复核               | 收入分成 + Coinbase wallet flow                           | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0544 | Coinbase 与渠道分发：USDC rewards 需要用 收入分成 复核                       | 收入分成 + USDC rewards                                   | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0545 | Coinbase 与渠道分发：Coinbase custody 需要用 收入分成 复核                   | 收入分成 + Coinbase custody                               | revise：保留为渠道议价检查       | `framework/03-competition.md`         |
| 0546 | Coinbase 与渠道分发：Coinbase user growth 需要用 收入分成 复核               | 收入分成 + Coinbase user growth                           | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0547 | Coinbase 与渠道分发：Base fee capture 需要用 收入分成 复核                   | 收入分成 + Base fee capture                               | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0548 | Coinbase 与渠道分发：Coinbase bargaining 需要用 收入分成 复核                | 收入分成 + Coinbase bargaining                            | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0549 | Coinbase 与渠道分发：ecosystem lock-in 需要用 收入分成 复核                  | 收入分成 + ecosystem lock-in                              | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0550 | Coinbase 与渠道分发：distribution dependence 需要用 收入分成 复核            | 收入分成 + distribution dependence                        | keep：保留为渠道议价检查         | `framework/03-competition.md`         |
| 0551 | 稳定币竞争结构：USDT dominance 需要用 DefiLlama 复核                         | DefiLlama + USDT dominance                                | revise：写入竞争监控             | `framework/03-competition.md`         |
| 0552 | 稳定币竞争结构：PYUSD growth 需要用 DefiLlama 复核                           | DefiLlama + PYUSD growth                                  | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0553 | 稳定币竞争结构：FDUSD share 需要用 DefiLlama 复核                            | DefiLlama + FDUSD share                                   | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0554 | 稳定币竞争结构：USDe AUM 需要用 DefiLlama 复核                               | DefiLlama + USDe AUM                                      | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0555 | 稳定币竞争结构：bank stablecoin 需要用 DefiLlama 复核                        | DefiLlama + bank stablecoin                               | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0556 | 稳定币竞争结构：tokenized deposits 需要用 DefiLlama 复核                     | DefiLlama + tokenized deposits                            | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0557 | 稳定币竞争结构：stablecoin total market 需要用 DefiLlama 复核                | DefiLlama + stablecoin total market                       | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0558 | 稳定币竞争结构：USDC ratio 需要用 DefiLlama 复核                             | DefiLlama + USDC ratio                                    | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0559 | 稳定币竞争结构：exchange liquidity 需要用 DefiLlama 复核                     | DefiLlama + exchange liquidity                            | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0560 | 稳定币竞争结构：institutional preference 需要用 DefiLlama 复核               | DefiLlama + institutional preference                      | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0561 | 稳定币竞争结构：USDT dominance 需要用 CoinGecko 复核                         | CoinGecko + USDT dominance                                | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0562 | 稳定币竞争结构：PYUSD growth 需要用 CoinGecko 复核                           | CoinGecko + PYUSD growth                                  | revise：写入竞争监控             | `framework/03-competition.md`         |
| 0563 | 稳定币竞争结构：FDUSD share 需要用 CoinGecko 复核                            | CoinGecko + FDUSD share                                   | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0564 | 稳定币竞争结构：USDe AUM 需要用 CoinGecko 复核                               | CoinGecko + USDe AUM                                      | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0565 | 稳定币竞争结构：bank stablecoin 需要用 CoinGecko 复核                        | CoinGecko + bank stablecoin                               | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0566 | 稳定币竞争结构：tokenized deposits 需要用 CoinGecko 复核                     | CoinGecko + tokenized deposits                            | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0567 | 稳定币竞争结构：stablecoin total market 需要用 CoinGecko 复核                | CoinGecko + stablecoin total market                       | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0568 | 稳定币竞争结构：USDC ratio 需要用 CoinGecko 复核                             | CoinGecko + USDC ratio                                    | defer：写入竞争监控              | `framework/03-competition.md`         |
| 0569 | 稳定币竞争结构：exchange liquidity 需要用 CoinGecko 复核                     | CoinGecko + exchange liquidity                            | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0570 | 稳定币竞争结构：institutional preference 需要用 CoinGecko 复核               | CoinGecko + institutional preference                      | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0571 | 稳定币竞争结构：USDT dominance 需要用 银行公告 复核                          | 银行公告 + USDT dominance                                 | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0572 | 稳定币竞争结构：PYUSD growth 需要用 银行公告 复核                            | 银行公告 + PYUSD growth                                   | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0573 | 稳定币竞争结构：FDUSD share 需要用 银行公告 复核                             | 银行公告 + FDUSD share                                    | revise：写入竞争监控             | `framework/03-competition.md`         |
| 0574 | 稳定币竞争结构：USDe AUM 需要用 银行公告 复核                                | 银行公告 + USDe AUM                                       | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0575 | 稳定币竞争结构：bank stablecoin 需要用 银行公告 复核                         | 银行公告 + bank stablecoin                                | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0576 | 稳定币竞争结构：tokenized deposits 需要用 银行公告 复核                      | 银行公告 + tokenized deposits                             | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0577 | 稳定币竞争结构：stablecoin total market 需要用 银行公告 复核                 | 银行公告 + stablecoin total market                        | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0578 | 稳定币竞争结构：USDC ratio 需要用 银行公告 复核                              | 银行公告 + USDC ratio                                     | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0579 | 稳定币竞争结构：exchange liquidity 需要用 银行公告 复核                      | 银行公告 + exchange liquidity                             | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0580 | 稳定币竞争结构：institutional preference 需要用 银行公告 复核                | 银行公告 + institutional preference                       | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0581 | 稳定币竞争结构：USDT dominance 需要用 RWA xyz 复核                           | RWA xyz + USDT dominance                                  | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0582 | 稳定币竞争结构：PYUSD growth 需要用 RWA xyz 复核                             | RWA xyz + PYUSD growth                                    | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0583 | 稳定币竞争结构：FDUSD share 需要用 RWA xyz 复核                              | RWA xyz + FDUSD share                                     | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0584 | 稳定币竞争结构：USDe AUM 需要用 RWA xyz 复核                                 | RWA xyz + USDe AUM                                        | revise：写入竞争监控             | `framework/03-competition.md`         |
| 0585 | 稳定币竞争结构：bank stablecoin 需要用 RWA xyz 复核                          | RWA xyz + bank stablecoin                                 | defer：写入竞争监控              | `framework/03-competition.md`         |
| 0586 | 稳定币竞争结构：tokenized deposits 需要用 RWA xyz 复核                       | RWA xyz + tokenized deposits                              | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0587 | 稳定币竞争结构：stablecoin total market 需要用 RWA xyz 复核                  | RWA xyz + stablecoin total market                         | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0588 | 稳定币竞争结构：USDC ratio 需要用 RWA xyz 复核                               | RWA xyz + USDC ratio                                      | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0589 | 稳定币竞争结构：exchange liquidity 需要用 RWA xyz 复核                       | RWA xyz + exchange liquidity                              | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0590 | 稳定币竞争结构：institutional preference 需要用 RWA xyz 复核                 | RWA xyz + institutional preference                        | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0591 | 稳定币竞争结构：USDT dominance 需要用 交易所深度 复核                        | 交易所深度 + USDT dominance                               | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0592 | 稳定币竞争结构：PYUSD growth 需要用 交易所深度 复核                          | 交易所深度 + PYUSD growth                                 | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0593 | 稳定币竞争结构：FDUSD share 需要用 交易所深度 复核                           | 交易所深度 + FDUSD share                                  | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0594 | 稳定币竞争结构：USDe AUM 需要用 交易所深度 复核                              | 交易所深度 + USDe AUM                                     | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0595 | 稳定币竞争结构：bank stablecoin 需要用 交易所深度 复核                       | 交易所深度 + bank stablecoin                              | revise：写入竞争监控             | `framework/03-competition.md`         |
| 0596 | 稳定币竞争结构：tokenized deposits 需要用 交易所深度 复核                    | 交易所深度 + tokenized deposits                           | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0597 | 稳定币竞争结构：stablecoin total market 需要用 交易所深度 复核               | 交易所深度 + stablecoin total market                      | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0598 | 稳定币竞争结构：USDC ratio 需要用 交易所深度 复核                            | 交易所深度 + USDC ratio                                   | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0599 | 稳定币竞争结构：exchange liquidity 需要用 交易所深度 复核                    | 交易所深度 + exchange liquidity                           | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0600 | 稳定币竞争结构：institutional preference 需要用 交易所深度 复核              | 交易所深度 + institutional preference                     | keep：写入竞争监控               | `framework/03-competition.md`         |
| 0601 | 链生态与使用质量：Ethereum USDC 需要用 链上供应 复核                         | 链上供应 + Ethereum USDC                                  | revise：写入周度复盘             | `metrics/02-weekly-review.md`         |
| 0602 | 链生态与使用质量：Solana USDC 需要用 链上供应 复核                           | 链上供应 + Solana USDC                                    | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0603 | 链生态与使用质量：Base USDC 需要用 链上供应 复核                             | 链上供应 + Base USDC                                      | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0604 | 链生态与使用质量：Arbitrum USDC 需要用 链上供应 复核                         | 链上供应 + Arbitrum USDC                                  | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0605 | 链生态与使用质量：Optimism USDC 需要用 链上供应 复核                         | 链上供应 + Optimism USDC                                  | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0606 | 链生态与使用质量：Polygon USDC 需要用 链上供应 复核                          | 链上供应 + Polygon USDC                                   | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0607 | 链生态与使用质量：Avalanche USDC 需要用 链上供应 复核                        | 链上供应 + Avalanche USDC                                 | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0608 | 链生态与使用质量：transaction count 需要用 链上供应 复核                     | 链上供应 + transaction count                              | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0609 | 链生态与使用质量：active addresses 需要用 链上供应 复核                      | 链上供应 + active addresses                               | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0610 | 链生态与使用质量：velocity 需要用 链上供应 复核                              | 链上供应 + velocity                                       | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0611 | 链生态与使用质量：Ethereum USDC 需要用 转账量 复核                           | 转账量 + Ethereum USDC                                    | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0612 | 链生态与使用质量：Solana USDC 需要用 转账量 复核                             | 转账量 + Solana USDC                                      | revise：写入周度复盘             | `metrics/02-weekly-review.md`         |
| 0613 | 链生态与使用质量：Base USDC 需要用 转账量 复核                               | 转账量 + Base USDC                                        | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0614 | 链生态与使用质量：Arbitrum USDC 需要用 转账量 复核                           | 转账量 + Arbitrum USDC                                    | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0615 | 链生态与使用质量：Optimism USDC 需要用 转账量 复核                           | 转账量 + Optimism USDC                                    | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0616 | 链生态与使用质量：Polygon USDC 需要用 转账量 复核                            | 转账量 + Polygon USDC                                     | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0617 | 链生态与使用质量：Avalanche USDC 需要用 转账量 复核                          | 转账量 + Avalanche USDC                                   | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0618 | 链生态与使用质量：transaction count 需要用 转账量 复核                       | 转账量 + transaction count                                | defer：写入周度复盘              | `metrics/02-weekly-review.md`         |
| 0619 | 链生态与使用质量：active addresses 需要用 转账量 复核                        | 转账量 + active addresses                                 | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0620 | 链生态与使用质量：velocity 需要用 转账量 复核                                | 转账量 + velocity                                         | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0621 | 链生态与使用质量：Ethereum USDC 需要用 活跃地址 复核                         | 活跃地址 + Ethereum USDC                                  | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0622 | 链生态与使用质量：Solana USDC 需要用 活跃地址 复核                           | 活跃地址 + Solana USDC                                    | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0623 | 链生态与使用质量：Base USDC 需要用 活跃地址 复核                             | 活跃地址 + Base USDC                                      | revise：写入周度复盘             | `metrics/02-weekly-review.md`         |
| 0624 | 链生态与使用质量：Arbitrum USDC 需要用 活跃地址 复核                         | 活跃地址 + Arbitrum USDC                                  | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0625 | 链生态与使用质量：Optimism USDC 需要用 活跃地址 复核                         | 活跃地址 + Optimism USDC                                  | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0626 | 链生态与使用质量：Polygon USDC 需要用 活跃地址 复核                          | 活跃地址 + Polygon USDC                                   | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0627 | 链生态与使用质量：Avalanche USDC 需要用 活跃地址 复核                        | 活跃地址 + Avalanche USDC                                 | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0628 | 链生态与使用质量：transaction count 需要用 活跃地址 复核                     | 活跃地址 + transaction count                              | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0629 | 链生态与使用质量：active addresses 需要用 活跃地址 复核                      | 活跃地址 + active addresses                               | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0630 | 链生态与使用质量：velocity 需要用 活跃地址 复核                              | 活跃地址 + velocity                                       | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0631 | 链生态与使用质量：Ethereum USDC 需要用 费用环境 复核                         | 费用环境 + Ethereum USDC                                  | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0632 | 链生态与使用质量：Solana USDC 需要用 费用环境 复核                           | 费用环境 + Solana USDC                                    | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0633 | 链生态与使用质量：Base USDC 需要用 费用环境 复核                             | 费用环境 + Base USDC                                      | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0634 | 链生态与使用质量：Arbitrum USDC 需要用 费用环境 复核                         | 费用环境 + Arbitrum USDC                                  | revise：写入周度复盘             | `metrics/02-weekly-review.md`         |
| 0635 | 链生态与使用质量：Optimism USDC 需要用 费用环境 复核                         | 费用环境 + Optimism USDC                                  | defer：写入周度复盘              | `metrics/02-weekly-review.md`         |
| 0636 | 链生态与使用质量：Polygon USDC 需要用 费用环境 复核                          | 费用环境 + Polygon USDC                                   | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0637 | 链生态与使用质量：Avalanche USDC 需要用 费用环境 复核                        | 费用环境 + Avalanche USDC                                 | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0638 | 链生态与使用质量：transaction count 需要用 费用环境 复核                     | 费用环境 + transaction count                              | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0639 | 链生态与使用质量：active addresses 需要用 费用环境 复核                      | 费用环境 + active addresses                               | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0640 | 链生态与使用质量：velocity 需要用 费用环境 复核                              | 费用环境 + velocity                                       | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0641 | 链生态与使用质量：Ethereum USDC 需要用 应用场景 复核                         | 应用场景 + Ethereum USDC                                  | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0642 | 链生态与使用质量：Solana USDC 需要用 应用场景 复核                           | 应用场景 + Solana USDC                                    | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0643 | 链生态与使用质量：Base USDC 需要用 应用场景 复核                             | 应用场景 + Base USDC                                      | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0644 | 链生态与使用质量：Arbitrum USDC 需要用 应用场景 复核                         | 应用场景 + Arbitrum USDC                                  | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0645 | 链生态与使用质量：Optimism USDC 需要用 应用场景 复核                         | 应用场景 + Optimism USDC                                  | revise：写入周度复盘             | `metrics/02-weekly-review.md`         |
| 0646 | 链生态与使用质量：Polygon USDC 需要用 应用场景 复核                          | 应用场景 + Polygon USDC                                   | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0647 | 链生态与使用质量：Avalanche USDC 需要用 应用场景 复核                        | 应用场景 + Avalanche USDC                                 | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0648 | 链生态与使用质量：transaction count 需要用 应用场景 复核                     | 应用场景 + transaction count                              | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0649 | 链生态与使用质量：active addresses 需要用 应用场景 复核                      | 应用场景 + active addresses                               | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0650 | 链生态与使用质量：velocity 需要用 应用场景 复核                              | 应用场景 + velocity                                       | keep：写入周度复盘               | `metrics/02-weekly-review.md`         |
| 0651 | DeFi RWA 与收益竞争：Aave USDC deposits 需要用 DefiLlama 复核                | DefiLlama + Aave USDC deposits                            | revise：保留为收益竞争检查       | `framework/03-competition.md`         |
| 0652 | DeFi RWA 与收益竞争：Compound borrow 需要用 DefiLlama 复核                   | DefiLlama + Compound borrow                               | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0653 | DeFi RWA 与收益竞争：Uniswap liquidity 需要用 DefiLlama 复核                 | DefiLlama + Uniswap liquidity                             | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0654 | DeFi RWA 与收益竞争：DeFi stablecoin TVL 需要用 DefiLlama 复核               | DefiLlama + DeFi stablecoin TVL                           | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0655 | DeFi RWA 与收益竞争：USDe yield 需要用 DefiLlama 复核                        | DefiLlama + USDe yield                                    | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0656 | DeFi RWA 与收益竞争：BUIDL AUM 需要用 DefiLlama 复核                         | DefiLlama + BUIDL AUM                                     | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0657 | DeFi RWA 与收益竞争：USDY AUM 需要用 DefiLlama 复核                          | DefiLlama + USDY AUM                                      | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0658 | DeFi RWA 与收益竞争：USYC AUM 需要用 DefiLlama 复核                          | DefiLlama + USYC AUM                                      | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0659 | DeFi RWA 与收益竞争：tokenized Treasury AUM 需要用 DefiLlama 复核            | DefiLlama + tokenized Treasury AUM                        | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0660 | DeFi RWA 与收益竞争：lending APY 需要用 DefiLlama 复核                       | DefiLlama + lending APY                                   | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0661 | DeFi RWA 与收益竞争：Aave USDC deposits 需要用 协议页面 复核                 | 协议页面 + Aave USDC deposits                             | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0662 | DeFi RWA 与收益竞争：Compound borrow 需要用 协议页面 复核                    | 协议页面 + Compound borrow                                | revise：保留为收益竞争检查       | `framework/03-competition.md`         |
| 0663 | DeFi RWA 与收益竞争：Uniswap liquidity 需要用 协议页面 复核                  | 协议页面 + Uniswap liquidity                              | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0664 | DeFi RWA 与收益竞争：DeFi stablecoin TVL 需要用 协议页面 复核                | 协议页面 + DeFi stablecoin TVL                            | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0665 | DeFi RWA 与收益竞争：USDe yield 需要用 协议页面 复核                         | 协议页面 + USDe yield                                     | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0666 | DeFi RWA 与收益竞争：BUIDL AUM 需要用 协议页面 复核                          | 协议页面 + BUIDL AUM                                      | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0667 | DeFi RWA 与收益竞争：USDY AUM 需要用 协议页面 复核                           | 协议页面 + USDY AUM                                       | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0668 | DeFi RWA 与收益竞争：USYC AUM 需要用 协议页面 复核                           | 协议页面 + USYC AUM                                       | defer：保留为收益竞争检查        | `framework/03-competition.md`         |
| 0669 | DeFi RWA 与收益竞争：tokenized Treasury AUM 需要用 协议页面 复核             | 协议页面 + tokenized Treasury AUM                         | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0670 | DeFi RWA 与收益竞争：lending APY 需要用 协议页面 复核                        | 协议页面 + lending APY                                    | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0671 | DeFi RWA 与收益竞争：Aave USDC deposits 需要用 RWA xyz 复核                  | RWA xyz + Aave USDC deposits                              | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0672 | DeFi RWA 与收益竞争：Compound borrow 需要用 RWA xyz 复核                     | RWA xyz + Compound borrow                                 | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0673 | DeFi RWA 与收益竞争：Uniswap liquidity 需要用 RWA xyz 复核                   | RWA xyz + Uniswap liquidity                               | revise：保留为收益竞争检查       | `framework/03-competition.md`         |
| 0674 | DeFi RWA 与收益竞争：DeFi stablecoin TVL 需要用 RWA xyz 复核                 | RWA xyz + DeFi stablecoin TVL                             | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0675 | DeFi RWA 与收益竞争：USDe yield 需要用 RWA xyz 复核                          | RWA xyz + USDe yield                                      | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0676 | DeFi RWA 与收益竞争：BUIDL AUM 需要用 RWA xyz 复核                           | RWA xyz + BUIDL AUM                                       | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0677 | DeFi RWA 与收益竞争：USDY AUM 需要用 RWA xyz 复核                            | RWA xyz + USDY AUM                                        | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0678 | DeFi RWA 与收益竞争：USYC AUM 需要用 RWA xyz 复核                            | RWA xyz + USYC AUM                                        | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0679 | DeFi RWA 与收益竞争：tokenized Treasury AUM 需要用 RWA xyz 复核              | RWA xyz + tokenized Treasury AUM                          | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0680 | DeFi RWA 与收益竞争：lending APY 需要用 RWA xyz 复核                         | RWA xyz + lending APY                                     | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0681 | DeFi RWA 与收益竞争：Aave USDC deposits 需要用 收益率 复核                   | 收益率 + Aave USDC deposits                               | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0682 | DeFi RWA 与收益竞争：Compound borrow 需要用 收益率 复核                      | 收益率 + Compound borrow                                  | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0683 | DeFi RWA 与收益竞争：Uniswap liquidity 需要用 收益率 复核                    | 收益率 + Uniswap liquidity                                | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0684 | DeFi RWA 与收益竞争：DeFi stablecoin TVL 需要用 收益率 复核                  | 收益率 + DeFi stablecoin TVL                              | revise：保留为收益竞争检查       | `framework/03-competition.md`         |
| 0685 | DeFi RWA 与收益竞争：USDe yield 需要用 收益率 复核                           | 收益率 + USDe yield                                       | defer：保留为收益竞争检查        | `framework/03-competition.md`         |
| 0686 | DeFi RWA 与收益竞争：BUIDL AUM 需要用 收益率 复核                            | 收益率 + BUIDL AUM                                        | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0687 | DeFi RWA 与收益竞争：USDY AUM 需要用 收益率 复核                             | 收益率 + USDY AUM                                         | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0688 | DeFi RWA 与收益竞争：USYC AUM 需要用 收益率 复核                             | 收益率 + USYC AUM                                         | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0689 | DeFi RWA 与收益竞争：tokenized Treasury AUM 需要用 收益率 复核               | 收益率 + tokenized Treasury AUM                           | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0690 | DeFi RWA 与收益竞争：lending APY 需要用 收益率 复核                          | 收益率 + lending APY                                      | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0691 | DeFi RWA 与收益竞争：Aave USDC deposits 需要用 资金迁移 复核                 | 资金迁移 + Aave USDC deposits                             | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0692 | DeFi RWA 与收益竞争：Compound borrow 需要用 资金迁移 复核                    | 资金迁移 + Compound borrow                                | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0693 | DeFi RWA 与收益竞争：Uniswap liquidity 需要用 资金迁移 复核                  | 资金迁移 + Uniswap liquidity                              | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0694 | DeFi RWA 与收益竞争：DeFi stablecoin TVL 需要用 资金迁移 复核                | 资金迁移 + DeFi stablecoin TVL                            | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0695 | DeFi RWA 与收益竞争：USDe yield 需要用 资金迁移 复核                         | 资金迁移 + USDe yield                                     | revise：保留为收益竞争检查       | `framework/03-competition.md`         |
| 0696 | DeFi RWA 与收益竞争：BUIDL AUM 需要用 资金迁移 复核                          | 资金迁移 + BUIDL AUM                                      | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0697 | DeFi RWA 与收益竞争：USDY AUM 需要用 资金迁移 复核                           | 资金迁移 + USDY AUM                                       | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0698 | DeFi RWA 与收益竞争：USYC AUM 需要用 资金迁移 复核                           | 资金迁移 + USYC AUM                                       | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0699 | DeFi RWA 与收益竞争：tokenized Treasury AUM 需要用 资金迁移 复核             | 资金迁移 + tokenized Treasury AUM                         | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0700 | DeFi RWA 与收益竞争：lending APY 需要用 资金迁移 复核                        | 资金迁移 + lending APY                                    | keep：保留为收益竞争检查         | `framework/03-competition.md`         |
| 0701 | CPN 支付网络：CPN institutions 需要用 Circle 披露 复核                       | Circle 披露 + CPN institutions                            | revise：写入平台化验证           | `framework/04-platform-option.md`     |
| 0702 | CPN 支付网络：CPN TPV 需要用 Circle 披露 复核                                | Circle 披露 + CPN TPV                                     | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0703 | CPN 支付网络：partner qualification 需要用 Circle 披露 复核                  | Circle 披露 + partner qualification                       | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0704 | CPN 支付网络：payment corridor 需要用 Circle 披露 复核                       | Circle 披露 + payment corridor                            | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0705 | CPN 支付网络：enterprise onboarding 需要用 Circle 披露 复核                  | Circle 披露 + enterprise onboarding                       | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0706 | CPN 支付网络：settlement speed 需要用 Circle 披露 复核                       | Circle 披露 + settlement speed                            | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0707 | CPN 支付网络：fee model 需要用 Circle 披露 复核                              | Circle 披露 + fee model                                   | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0708 | CPN 支付网络：revenue recognition 需要用 Circle 披露 复核                    | Circle 披露 + revenue recognition                         | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0709 | CPN 支付网络：compliance friction 需要用 Circle 披露 复核                    | Circle 披露 + compliance friction                         | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0710 | CPN 支付网络：customer concentration 需要用 Circle 披露 复核                 | Circle 披露 + customer concentration                      | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0711 | CPN 支付网络：CPN institutions 需要用 财报问答 复核                          | 财报问答 + CPN institutions                               | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0712 | CPN 支付网络：CPN TPV 需要用 财报问答 复核                                   | 财报问答 + CPN TPV                                        | revise：写入平台化验证           | `framework/04-platform-option.md`     |
| 0713 | CPN 支付网络：partner qualification 需要用 财报问答 复核                     | 财报问答 + partner qualification                          | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0714 | CPN 支付网络：payment corridor 需要用 财报问答 复核                          | 财报问答 + payment corridor                               | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0715 | CPN 支付网络：enterprise onboarding 需要用 财报问答 复核                     | 财报问答 + enterprise onboarding                          | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0716 | CPN 支付网络：settlement speed 需要用 财报问答 复核                          | 财报问答 + settlement speed                               | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0717 | CPN 支付网络：fee model 需要用 财报问答 复核                                 | 财报问答 + fee model                                      | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0718 | CPN 支付网络：revenue recognition 需要用 财报问答 复核                       | 财报问答 + revenue recognition                            | defer：写入平台化验证            | `framework/04-platform-option.md`     |
| 0719 | CPN 支付网络：compliance friction 需要用 财报问答 复核                       | 财报问答 + compliance friction                            | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0720 | CPN 支付网络：customer concentration 需要用 财报问答 复核                    | 财报问答 + customer concentration                         | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0721 | CPN 支付网络：CPN institutions 需要用 TPV 趋势 复核                          | TPV 趋势 + CPN institutions                               | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0722 | CPN 支付网络：CPN TPV 需要用 TPV 趋势 复核                                   | TPV 趋势 + CPN TPV                                        | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0723 | CPN 支付网络：partner qualification 需要用 TPV 趋势 复核                     | TPV 趋势 + partner qualification                          | revise：写入平台化验证           | `framework/04-platform-option.md`     |
| 0724 | CPN 支付网络：payment corridor 需要用 TPV 趋势 复核                          | TPV 趋势 + payment corridor                               | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0725 | CPN 支付网络：enterprise onboarding 需要用 TPV 趋势 复核                     | TPV 趋势 + enterprise onboarding                          | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0726 | CPN 支付网络：settlement speed 需要用 TPV 趋势 复核                          | TPV 趋势 + settlement speed                               | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0727 | CPN 支付网络：fee model 需要用 TPV 趋势 复核                                 | TPV 趋势 + fee model                                      | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0728 | CPN 支付网络：revenue recognition 需要用 TPV 趋势 复核                       | TPV 趋势 + revenue recognition                            | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0729 | CPN 支付网络：compliance friction 需要用 TPV 趋势 复核                       | TPV 趋势 + compliance friction                            | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0730 | CPN 支付网络：customer concentration 需要用 TPV 趋势 复核                    | TPV 趋势 + customer concentration                         | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0731 | CPN 支付网络：CPN institutions 需要用 Other revenue 复核                     | Other revenue + CPN institutions                          | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0732 | CPN 支付网络：CPN TPV 需要用 Other revenue 复核                              | Other revenue + CPN TPV                                   | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0733 | CPN 支付网络：partner qualification 需要用 Other revenue 复核                | Other revenue + partner qualification                     | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0734 | CPN 支付网络：payment corridor 需要用 Other revenue 复核                     | Other revenue + payment corridor                          | revise：写入平台化验证           | `framework/04-platform-option.md`     |
| 0735 | CPN 支付网络：enterprise onboarding 需要用 Other revenue 复核                | Other revenue + enterprise onboarding                     | defer：写入平台化验证            | `framework/04-platform-option.md`     |
| 0736 | CPN 支付网络：settlement speed 需要用 Other revenue 复核                     | Other revenue + settlement speed                          | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0737 | CPN 支付网络：fee model 需要用 Other revenue 复核                            | Other revenue + fee model                                 | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0738 | CPN 支付网络：revenue recognition 需要用 Other revenue 复核                  | Other revenue + revenue recognition                       | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0739 | CPN 支付网络：compliance friction 需要用 Other revenue 复核                  | Other revenue + compliance friction                       | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0740 | CPN 支付网络：customer concentration 需要用 Other revenue 复核               | Other revenue + customer concentration                    | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0741 | CPN 支付网络：CPN institutions 需要用 客户质量 复核                          | 客户质量 + CPN institutions                               | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0742 | CPN 支付网络：CPN TPV 需要用 客户质量 复核                                   | 客户质量 + CPN TPV                                        | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0743 | CPN 支付网络：partner qualification 需要用 客户质量 复核                     | 客户质量 + partner qualification                          | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0744 | CPN 支付网络：payment corridor 需要用 客户质量 复核                          | 客户质量 + payment corridor                               | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0745 | CPN 支付网络：enterprise onboarding 需要用 客户质量 复核                     | 客户质量 + enterprise onboarding                          | revise：写入平台化验证           | `framework/04-platform-option.md`     |
| 0746 | CPN 支付网络：settlement speed 需要用 客户质量 复核                          | 客户质量 + settlement speed                               | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0747 | CPN 支付网络：fee model 需要用 客户质量 复核                                 | 客户质量 + fee model                                      | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0748 | CPN 支付网络：revenue recognition 需要用 客户质量 复核                       | 客户质量 + revenue recognition                            | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0749 | CPN 支付网络：compliance friction 需要用 客户质量 复核                       | 客户质量 + compliance friction                            | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0750 | CPN 支付网络：customer concentration 需要用 客户质量 复核                    | 客户质量 + customer concentration                         | keep：写入平台化验证             | `framework/04-platform-option.md`     |
| 0751 | Arc 与基础设施期权：Arc testnet tx 需要用 Circle docs 复核                   | Circle docs + Arc testnet tx                              | revise：测试网不等于收入         | `framework/04-platform-option.md`     |
| 0752 | Arc 与基础设施期权：Arc mainnet timing 需要用 Circle docs 复核               | Circle docs + Arc mainnet timing                          | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0753 | Arc 与基础设施期权：validator model 需要用 Circle docs 复核                  | Circle docs + validator model                             | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0754 | Arc 与基础设施期权：gas model 需要用 Circle docs 复核                        | Circle docs + gas model                                   | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0755 | Arc 与基础设施期权：developer adoption 需要用 Circle docs 复核               | Circle docs + developer adoption                          | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0756 | Arc 与基础设施期权：enterprise pilot 需要用 Circle docs 复核                 | Circle docs + enterprise pilot                            | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0757 | Arc 与基础设施期权：USDC gas usage 需要用 Circle docs 复核                   | Circle docs + USDC gas usage                              | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0758 | Arc 与基础设施期权：fee capture 需要用 Circle docs 复核                      | Circle docs + fee capture                                 | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0759 | Arc 与基础设施期权：Arc compliance feature 需要用 Circle docs 复核           | Circle docs + Arc compliance feature                      | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0760 | Arc 与基础设施期权：mainnet delay 需要用 Circle docs 复核                    | Circle docs + mainnet delay                               | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0761 | Arc 与基础设施期权：Arc testnet tx 需要用 主网公告 复核                      | 主网公告 + Arc testnet tx                                 | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0762 | Arc 与基础设施期权：Arc mainnet timing 需要用 主网公告 复核                  | 主网公告 + Arc mainnet timing                             | revise：测试网不等于收入         | `framework/04-platform-option.md`     |
| 0763 | Arc 与基础设施期权：validator model 需要用 主网公告 复核                     | 主网公告 + validator model                                | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0764 | Arc 与基础设施期权：gas model 需要用 主网公告 复核                           | 主网公告 + gas model                                      | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0765 | Arc 与基础设施期权：developer adoption 需要用 主网公告 复核                  | 主网公告 + developer adoption                             | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0766 | Arc 与基础设施期权：enterprise pilot 需要用 主网公告 复核                    | 主网公告 + enterprise pilot                               | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0767 | Arc 与基础设施期权：USDC gas usage 需要用 主网公告 复核                      | 主网公告 + USDC gas usage                                 | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0768 | Arc 与基础设施期权：fee capture 需要用 主网公告 复核                         | 主网公告 + fee capture                                    | defer：测试网不等于收入          | `framework/04-platform-option.md`     |
| 0769 | Arc 与基础设施期权：Arc compliance feature 需要用 主网公告 复核              | 主网公告 + Arc compliance feature                         | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0770 | Arc 与基础设施期权：mainnet delay 需要用 主网公告 复核                       | 主网公告 + mainnet delay                                  | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0771 | Arc 与基础设施期权：Arc testnet tx 需要用 真实客户 复核                      | 真实客户 + Arc testnet tx                                 | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0772 | Arc 与基础设施期权：Arc mainnet timing 需要用 真实客户 复核                  | 真实客户 + Arc mainnet timing                             | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0773 | Arc 与基础设施期权：validator model 需要用 真实客户 复核                     | 真实客户 + validator model                                | revise：测试网不等于收入         | `framework/04-platform-option.md`     |
| 0774 | Arc 与基础设施期权：gas model 需要用 真实客户 复核                           | 真实客户 + gas model                                      | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0775 | Arc 与基础设施期权：developer adoption 需要用 真实客户 复核                  | 真实客户 + developer adoption                             | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0776 | Arc 与基础设施期权：enterprise pilot 需要用 真实客户 复核                    | 真实客户 + enterprise pilot                               | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0777 | Arc 与基础设施期权：USDC gas usage 需要用 真实客户 复核                      | 真实客户 + USDC gas usage                                 | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0778 | Arc 与基础设施期权：fee capture 需要用 真实客户 复核                         | 真实客户 + fee capture                                    | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0779 | Arc 与基础设施期权：Arc compliance feature 需要用 真实客户 复核              | 真实客户 + Arc compliance feature                         | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0780 | Arc 与基础设施期权：mainnet delay 需要用 真实客户 复核                       | 真实客户 + mainnet delay                                  | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0781 | Arc 与基础设施期权：Arc testnet tx 需要用 费用模型 复核                      | 费用模型 + Arc testnet tx                                 | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0782 | Arc 与基础设施期权：Arc mainnet timing 需要用 费用模型 复核                  | 费用模型 + Arc mainnet timing                             | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0783 | Arc 与基础设施期权：validator model 需要用 费用模型 复核                     | 费用模型 + validator model                                | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0784 | Arc 与基础设施期权：gas model 需要用 费用模型 复核                           | 费用模型 + gas model                                      | revise：测试网不等于收入         | `framework/04-platform-option.md`     |
| 0785 | Arc 与基础设施期权：developer adoption 需要用 费用模型 复核                  | 费用模型 + developer adoption                             | defer：测试网不等于收入          | `framework/04-platform-option.md`     |
| 0786 | Arc 与基础设施期权：enterprise pilot 需要用 费用模型 复核                    | 费用模型 + enterprise pilot                               | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0787 | Arc 与基础设施期权：USDC gas usage 需要用 费用模型 复核                      | 费用模型 + USDC gas usage                                 | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0788 | Arc 与基础设施期权：fee capture 需要用 费用模型 复核                         | 费用模型 + fee capture                                    | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0789 | Arc 与基础设施期权：Arc compliance feature 需要用 费用模型 复核              | 费用模型 + Arc compliance feature                         | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0790 | Arc 与基础设施期权：mainnet delay 需要用 费用模型 复核                       | 费用模型 + mainnet delay                                  | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0791 | Arc 与基础设施期权：Arc testnet tx 需要用 收入贡献 复核                      | 收入贡献 + Arc testnet tx                                 | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0792 | Arc 与基础设施期权：Arc mainnet timing 需要用 收入贡献 复核                  | 收入贡献 + Arc mainnet timing                             | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0793 | Arc 与基础设施期权：validator model 需要用 收入贡献 复核                     | 收入贡献 + validator model                                | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0794 | Arc 与基础设施期权：gas model 需要用 收入贡献 复核                           | 收入贡献 + gas model                                      | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0795 | Arc 与基础设施期权：developer adoption 需要用 收入贡献 复核                  | 收入贡献 + developer adoption                             | revise：测试网不等于收入         | `framework/04-platform-option.md`     |
| 0796 | Arc 与基础设施期权：enterprise pilot 需要用 收入贡献 复核                    | 收入贡献 + enterprise pilot                               | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0797 | Arc 与基础设施期权：USDC gas usage 需要用 收入贡献 复核                      | 收入贡献 + USDC gas usage                                 | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0798 | Arc 与基础设施期权：fee capture 需要用 收入贡献 复核                         | 收入贡献 + fee capture                                    | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0799 | Arc 与基础设施期权：Arc compliance feature 需要用 收入贡献 复核              | 收入贡献 + Arc compliance feature                         | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0800 | Arc 与基础设施期权：mainnet delay 需要用 收入贡献 复核                       | 收入贡献 + mainnet delay                                  | keep：测试网不等于收入           | `framework/04-platform-option.md`     |
| 0801 | 非储备收入与产品扩展：Other revenue share 需要用 财报拆分 复核               | 财报拆分 + Other revenue share                            | revise：写入平台化收入阈值       | `valuation/00-valuation-framework.md` |
| 0802 | 非储备收入与产品扩展：integration services 需要用 财报拆分 复核              | 财报拆分 + integration services                           | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0803 | 非储备收入与产品扩展：API revenue 需要用 财报拆分 复核                       | 财报拆分 + API revenue                                    | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0804 | 非储备收入与产品扩展：enterprise account fees 需要用 财报拆分 复核           | 财报拆分 + enterprise account fees                        | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0805 | 非储备收入与产品扩展：EURC scale 需要用 财报拆分 复核                        | 财报拆分 + EURC scale                                     | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0806 | 非储备收入与产品扩展：USYC fee 需要用 财报拆分 复核                          | 财报拆分 + USYC fee                                       | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0807 | 非储备收入与产品扩展：wallet services 需要用 财报拆分 复核                   | 财报拆分 + wallet services                                | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0808 | 非储备收入与产品扩展：developer platform 需要用 财报拆分 复核                | 财报拆分 + developer platform                             | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0809 | 非储备收入与产品扩展：recurring revenue 需要用 财报拆分 复核                 | 财报拆分 + recurring revenue                              | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0810 | 非储备收入与产品扩展：gross margin 需要用 财报拆分 复核                      | 财报拆分 + gross margin                                   | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0811 | 非储备收入与产品扩展：Other revenue share 需要用 收入可重复性 复核           | 收入可重复性 + Other revenue share                        | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0812 | 非储备收入与产品扩展：integration services 需要用 收入可重复性 复核          | 收入可重复性 + integration services                       | revise：写入平台化收入阈值       | `valuation/00-valuation-framework.md` |
| 0813 | 非储备收入与产品扩展：API revenue 需要用 收入可重复性 复核                   | 收入可重复性 + API revenue                                | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0814 | 非储备收入与产品扩展：enterprise account fees 需要用 收入可重复性 复核       | 收入可重复性 + enterprise account fees                    | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0815 | 非储备收入与产品扩展：EURC scale 需要用 收入可重复性 复核                    | 收入可重复性 + EURC scale                                 | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0816 | 非储备收入与产品扩展：USYC fee 需要用 收入可重复性 复核                      | 收入可重复性 + USYC fee                                   | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0817 | 非储备收入与产品扩展：wallet services 需要用 收入可重复性 复核               | 收入可重复性 + wallet services                            | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0818 | 非储备收入与产品扩展：developer platform 需要用 收入可重复性 复核            | 收入可重复性 + developer platform                         | defer：写入平台化收入阈值        | `valuation/00-valuation-framework.md` |
| 0819 | 非储备收入与产品扩展：recurring revenue 需要用 收入可重复性 复核             | 收入可重复性 + recurring revenue                          | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0820 | 非储备收入与产品扩展：gross margin 需要用 收入可重复性 复核                  | 收入可重复性 + gross margin                               | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0821 | 非储备收入与产品扩展：Other revenue share 需要用 产品公告 复核               | 产品公告 + Other revenue share                            | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0822 | 非储备收入与产品扩展：integration services 需要用 产品公告 复核              | 产品公告 + integration services                           | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0823 | 非储备收入与产品扩展：API revenue 需要用 产品公告 复核                       | 产品公告 + API revenue                                    | revise：写入平台化收入阈值       | `valuation/00-valuation-framework.md` |
| 0824 | 非储备收入与产品扩展：enterprise account fees 需要用 产品公告 复核           | 产品公告 + enterprise account fees                        | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0825 | 非储备收入与产品扩展：EURC scale 需要用 产品公告 复核                        | 产品公告 + EURC scale                                     | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0826 | 非储备收入与产品扩展：USYC fee 需要用 产品公告 复核                          | 产品公告 + USYC fee                                       | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0827 | 非储备收入与产品扩展：wallet services 需要用 产品公告 复核                   | 产品公告 + wallet services                                | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0828 | 非储备收入与产品扩展：developer platform 需要用 产品公告 复核                | 产品公告 + developer platform                             | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0829 | 非储备收入与产品扩展：recurring revenue 需要用 产品公告 复核                 | 产品公告 + recurring revenue                              | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0830 | 非储备收入与产品扩展：gross margin 需要用 产品公告 复核                      | 产品公告 + gross margin                                   | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0831 | 非储备收入与产品扩展：Other revenue share 需要用 客户留存 复核               | 客户留存 + Other revenue share                            | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0832 | 非储备收入与产品扩展：integration services 需要用 客户留存 复核              | 客户留存 + integration services                           | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0833 | 非储备收入与产品扩展：API revenue 需要用 客户留存 复核                       | 客户留存 + API revenue                                    | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0834 | 非储备收入与产品扩展：enterprise account fees 需要用 客户留存 复核           | 客户留存 + enterprise account fees                        | revise：写入平台化收入阈值       | `valuation/00-valuation-framework.md` |
| 0835 | 非储备收入与产品扩展：EURC scale 需要用 客户留存 复核                        | 客户留存 + EURC scale                                     | defer：写入平台化收入阈值        | `valuation/00-valuation-framework.md` |
| 0836 | 非储备收入与产品扩展：USYC fee 需要用 客户留存 复核                          | 客户留存 + USYC fee                                       | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0837 | 非储备收入与产品扩展：wallet services 需要用 客户留存 复核                   | 客户留存 + wallet services                                | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0838 | 非储备收入与产品扩展：developer platform 需要用 客户留存 复核                | 客户留存 + developer platform                             | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0839 | 非储备收入与产品扩展：recurring revenue 需要用 客户留存 复核                 | 客户留存 + recurring revenue                              | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0840 | 非储备收入与产品扩展：gross margin 需要用 客户留存 复核                      | 客户留存 + gross margin                                   | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0841 | 非储备收入与产品扩展：Other revenue share 需要用 毛利线索 复核               | 毛利线索 + Other revenue share                            | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0842 | 非储备收入与产品扩展：integration services 需要用 毛利线索 复核              | 毛利线索 + integration services                           | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0843 | 非储备收入与产品扩展：API revenue 需要用 毛利线索 复核                       | 毛利线索 + API revenue                                    | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0844 | 非储备收入与产品扩展：enterprise account fees 需要用 毛利线索 复核           | 毛利线索 + enterprise account fees                        | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0845 | 非储备收入与产品扩展：EURC scale 需要用 毛利线索 复核                        | 毛利线索 + EURC scale                                     | revise：写入平台化收入阈值       | `valuation/00-valuation-framework.md` |
| 0846 | 非储备收入与产品扩展：USYC fee 需要用 毛利线索 复核                          | 毛利线索 + USYC fee                                       | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0847 | 非储备收入与产品扩展：wallet services 需要用 毛利线索 复核                   | 毛利线索 + wallet services                                | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0848 | 非储备收入与产品扩展：developer platform 需要用 毛利线索 复核                | 毛利线索 + developer platform                             | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0849 | 非储备收入与产品扩展：recurring revenue 需要用 毛利线索 复核                 | 毛利线索 + recurring revenue                              | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0850 | 非储备收入与产品扩展：gross margin 需要用 毛利线索 复核                      | 毛利线索 + gross margin                                   | keep：写入平台化收入阈值         | `valuation/00-valuation-framework.md` |
| 0851 | 估值与情景模型：Market cap per USDC 需要用 市值 复核                         | 市值 + Market cap per USDC                                | revise：写入估值锚               | `valuation/01-scenario-model.md`      |
| 0852 | 估值与情景模型：Market cap per RLDC 需要用 市值 复核                         | 市值 + Market cap per RLDC                                | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0853 | 估值与情景模型：EV EBITDA 需要用 市值 复核                                   | 市值 + EV EBITDA                                          | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0854 | 估值与情景模型：Other revenue multiple 需要用 市值 复核                      | 市值 + Other revenue multiple                             | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0855 | 估值与情景模型：bull trigger 需要用 市值 复核                                | 市值 + bull trigger                                       | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0856 | 估值与情景模型：base trigger 需要用 市值 复核                                | 市值 + base trigger                                       | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0857 | 估值与情景模型：bear trigger 需要用 市值 复核                                | 市值 + bear trigger                                       | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0858 | 估值与情景模型：safety margin 需要用 市值 复核                               | 市值 + safety margin                                      | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0859 | 估值与情景模型：discount factor 需要用 市值 复核                             | 市值 + discount factor                                    | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0860 | 估值与情景模型：multiple compression 需要用 市值 复核                        | 市值 + multiple compression                               | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0861 | 估值与情景模型：Market cap per USDC 需要用 年化 RLDC 复核                    | 年化 RLDC + Market cap per USDC                           | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0862 | 估值与情景模型：Market cap per RLDC 需要用 年化 RLDC 复核                    | 年化 RLDC + Market cap per RLDC                           | revise：写入估值锚               | `valuation/01-scenario-model.md`      |
| 0863 | 估值与情景模型：EV EBITDA 需要用 年化 RLDC 复核                              | 年化 RLDC + EV EBITDA                                     | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0864 | 估值与情景模型：Other revenue multiple 需要用 年化 RLDC 复核                 | 年化 RLDC + Other revenue multiple                        | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0865 | 估值与情景模型：bull trigger 需要用 年化 RLDC 复核                           | 年化 RLDC + bull trigger                                  | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0866 | 估值与情景模型：base trigger 需要用 年化 RLDC 复核                           | 年化 RLDC + base trigger                                  | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0867 | 估值与情景模型：bear trigger 需要用 年化 RLDC 复核                           | 年化 RLDC + bear trigger                                  | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0868 | 估值与情景模型：safety margin 需要用 年化 RLDC 复核                          | 年化 RLDC + safety margin                                 | defer：写入估值锚                | `valuation/01-scenario-model.md`      |
| 0869 | 估值与情景模型：discount factor 需要用 年化 RLDC 复核                        | 年化 RLDC + discount factor                               | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0870 | 估值与情景模型：multiple compression 需要用 年化 RLDC 复核                   | 年化 RLDC + multiple compression                          | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0871 | 估值与情景模型：Market cap per USDC 需要用 情景阈值 复核                     | 情景阈值 + Market cap per USDC                            | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0872 | 估值与情景模型：Market cap per RLDC 需要用 情景阈值 复核                     | 情景阈值 + Market cap per RLDC                            | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0873 | 估值与情景模型：EV EBITDA 需要用 情景阈值 复核                               | 情景阈值 + EV EBITDA                                      | revise：写入估值锚               | `valuation/01-scenario-model.md`      |
| 0874 | 估值与情景模型：Other revenue multiple 需要用 情景阈值 复核                  | 情景阈值 + Other revenue multiple                         | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0875 | 估值与情景模型：bull trigger 需要用 情景阈值 复核                            | 情景阈值 + bull trigger                                   | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0876 | 估值与情景模型：base trigger 需要用 情景阈值 复核                            | 情景阈值 + base trigger                                   | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0877 | 估值与情景模型：bear trigger 需要用 情景阈值 复核                            | 情景阈值 + bear trigger                                   | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0878 | 估值与情景模型：safety margin 需要用 情景阈值 复核                           | 情景阈值 + safety margin                                  | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0879 | 估值与情景模型：discount factor 需要用 情景阈值 复核                         | 情景阈值 + discount factor                                | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0880 | 估值与情景模型：multiple compression 需要用 情景阈值 复核                    | 情景阈值 + multiple compression                           | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0881 | 估值与情景模型：Market cap per USDC 需要用 监管折价 复核                     | 监管折价 + Market cap per USDC                            | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0882 | 估值与情景模型：Market cap per RLDC 需要用 监管折价 复核                     | 监管折价 + Market cap per RLDC                            | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0883 | 估值与情景模型：EV EBITDA 需要用 监管折价 复核                               | 监管折价 + EV EBITDA                                      | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0884 | 估值与情景模型：Other revenue multiple 需要用 监管折价 复核                  | 监管折价 + Other revenue multiple                         | revise：写入估值锚               | `valuation/01-scenario-model.md`      |
| 0885 | 估值与情景模型：bull trigger 需要用 监管折价 复核                            | 监管折价 + bull trigger                                   | defer：写入估值锚                | `valuation/01-scenario-model.md`      |
| 0886 | 估值与情景模型：base trigger 需要用 监管折价 复核                            | 监管折价 + base trigger                                   | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0887 | 估值与情景模型：bear trigger 需要用 监管折价 复核                            | 监管折价 + bear trigger                                   | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0888 | 估值与情景模型：safety margin 需要用 监管折价 复核                           | 监管折价 + safety margin                                  | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0889 | 估值与情景模型：discount factor 需要用 监管折价 复核                         | 监管折价 + discount factor                                | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0890 | 估值与情景模型：multiple compression 需要用 监管折价 复核                    | 监管折价 + multiple compression                           | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0891 | 估值与情景模型：Market cap per USDC 需要用 成本压力 复核                     | 成本压力 + Market cap per USDC                            | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0892 | 估值与情景模型：Market cap per RLDC 需要用 成本压力 复核                     | 成本压力 + Market cap per RLDC                            | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0893 | 估值与情景模型：EV EBITDA 需要用 成本压力 复核                               | 成本压力 + EV EBITDA                                      | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0894 | 估值与情景模型：Other revenue multiple 需要用 成本压力 复核                  | 成本压力 + Other revenue multiple                         | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0895 | 估值与情景模型：bull trigger 需要用 成本压力 复核                            | 成本压力 + bull trigger                                   | revise：写入估值锚               | `valuation/01-scenario-model.md`      |
| 0896 | 估值与情景模型：base trigger 需要用 成本压力 复核                            | 成本压力 + base trigger                                   | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0897 | 估值与情景模型：bear trigger 需要用 成本压力 复核                            | 成本压力 + bear trigger                                   | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0898 | 估值与情景模型：safety margin 需要用 成本压力 复核                           | 成本压力 + safety margin                                  | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0899 | 估值与情景模型：discount factor 需要用 成本压力 复核                         | 成本压力 + discount factor                                | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0900 | 估值与情景模型：multiple compression 需要用 成本压力 复核                    | 成本压力 + multiple compression                           | keep：写入估值锚                 | `valuation/01-scenario-model.md`      |
| 0901 | 风险筹码与事件：short interest 需要用 FINRA 复核                             | FINRA + short interest                                    | revise：只影响风控不替代基本面   | `risk/01-warning-signals.md`          |
| 0902 | 风险筹码与事件：borrow fee 需要用 FINRA 复核                                 | FINRA + borrow fee                                        | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0903 | 风险筹码与事件：put call ratio 需要用 FINRA 复核                             | FINRA + put call ratio                                    | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0904 | 风险筹码与事件：option IV 需要用 FINRA 复核                                  | FINRA + option IV                                         | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0905 | 风险筹码与事件：lock-up expiry 需要用 FINRA 复核                             | FINRA + lock-up expiry                                    | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0906 | 风险筹码与事件：Form 4 selling 需要用 FINRA 复核                             | FINRA + Form 4 selling                                    | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0907 | 风险筹码与事件：secondary offering 需要用 FINRA 复核                         | FINRA + secondary offering                                | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0908 | 风险筹码与事件：relative volume 需要用 FINRA 复核                            | FINRA + relative volume                                   | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0909 | 风险筹码与事件：20D break 需要用 FINRA 复核                                  | FINRA + 20D break                                         | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0910 | 风险筹码与事件：50D break 需要用 FINRA 复核                                  | FINRA + 50D break                                         | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0911 | 风险筹码与事件：short interest 需要用 期权平台 复核                          | 期权平台 + short interest                                 | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0912 | 风险筹码与事件：borrow fee 需要用 期权平台 复核                              | 期权平台 + borrow fee                                     | revise：只影响风控不替代基本面   | `risk/01-warning-signals.md`          |
| 0913 | 风险筹码与事件：put call ratio 需要用 期权平台 复核                          | 期权平台 + put call ratio                                 | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0914 | 风险筹码与事件：option IV 需要用 期权平台 复核                               | 期权平台 + option IV                                      | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0915 | 风险筹码与事件：lock-up expiry 需要用 期权平台 复核                          | 期权平台 + lock-up expiry                                 | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0916 | 风险筹码与事件：Form 4 selling 需要用 期权平台 复核                          | 期权平台 + Form 4 selling                                 | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0917 | 风险筹码与事件：secondary offering 需要用 期权平台 复核                      | 期权平台 + secondary offering                             | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0918 | 风险筹码与事件：relative volume 需要用 期权平台 复核                         | 期权平台 + relative volume                                | defer：只影响风控不替代基本面    | `risk/01-warning-signals.md`          |
| 0919 | 风险筹码与事件：20D break 需要用 期权平台 复核                               | 期权平台 + 20D break                                      | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0920 | 风险筹码与事件：50D break 需要用 期权平台 复核                               | 期权平台 + 50D break                                      | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0921 | 风险筹码与事件：short interest 需要用 SEC filing 复核                        | SEC filing + short interest                               | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0922 | 风险筹码与事件：borrow fee 需要用 SEC filing 复核                            | SEC filing + borrow fee                                   | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0923 | 风险筹码与事件：put call ratio 需要用 SEC filing 复核                        | SEC filing + put call ratio                               | revise：只影响风控不替代基本面   | `risk/01-warning-signals.md`          |
| 0924 | 风险筹码与事件：option IV 需要用 SEC filing 复核                             | SEC filing + option IV                                    | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0925 | 风险筹码与事件：lock-up expiry 需要用 SEC filing 复核                        | SEC filing + lock-up expiry                               | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0926 | 风险筹码与事件：Form 4 selling 需要用 SEC filing 复核                        | SEC filing + Form 4 selling                               | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0927 | 风险筹码与事件：secondary offering 需要用 SEC filing 复核                    | SEC filing + secondary offering                           | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0928 | 风险筹码与事件：relative volume 需要用 SEC filing 复核                       | SEC filing + relative volume                              | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0929 | 风险筹码与事件：20D break 需要用 SEC filing 复核                             | SEC filing + 20D break                                    | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0930 | 风险筹码与事件：50D break 需要用 SEC filing 复核                             | SEC filing + 50D break                                    | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0931 | 风险筹码与事件：short interest 需要用 成交量 复核                            | 成交量 + short interest                                   | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0932 | 风险筹码与事件：borrow fee 需要用 成交量 复核                                | 成交量 + borrow fee                                       | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0933 | 风险筹码与事件：put call ratio 需要用 成交量 复核                            | 成交量 + put call ratio                                   | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0934 | 风险筹码与事件：option IV 需要用 成交量 复核                                 | 成交量 + option IV                                        | revise：只影响风控不替代基本面   | `risk/01-warning-signals.md`          |
| 0935 | 风险筹码与事件：lock-up expiry 需要用 成交量 复核                            | 成交量 + lock-up expiry                                   | defer：只影响风控不替代基本面    | `risk/01-warning-signals.md`          |
| 0936 | 风险筹码与事件：Form 4 selling 需要用 成交量 复核                            | 成交量 + Form 4 selling                                   | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0937 | 风险筹码与事件：secondary offering 需要用 成交量 复核                        | 成交量 + secondary offering                               | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0938 | 风险筹码与事件：relative volume 需要用 成交量 复核                           | 成交量 + relative volume                                  | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0939 | 风险筹码与事件：20D break 需要用 成交量 复核                                 | 成交量 + 20D break                                        | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0940 | 风险筹码与事件：50D break 需要用 成交量 复核                                 | 成交量 + 50D break                                        | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0941 | 风险筹码与事件：short interest 需要用 均线 复核                              | 均线 + short interest                                     | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0942 | 风险筹码与事件：borrow fee 需要用 均线 复核                                  | 均线 + borrow fee                                         | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0943 | 风险筹码与事件：put call ratio 需要用 均线 复核                              | 均线 + put call ratio                                     | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0944 | 风险筹码与事件：option IV 需要用 均线 复核                                   | 均线 + option IV                                          | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0945 | 风险筹码与事件：lock-up expiry 需要用 均线 复核                              | 均线 + lock-up expiry                                     | revise：只影响风控不替代基本面   | `risk/01-warning-signals.md`          |
| 0946 | 风险筹码与事件：Form 4 selling 需要用 均线 复核                              | 均线 + Form 4 selling                                     | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0947 | 风险筹码与事件：secondary offering 需要用 均线 复核                          | 均线 + secondary offering                                 | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0948 | 风险筹码与事件：relative volume 需要用 均线 复核                             | 均线 + relative volume                                    | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0949 | 风险筹码与事件：20D break 需要用 均线 复核                                   | 均线 + 20D break                                          | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0950 | 风险筹码与事件：50D break 需要用 均线 复核                                   | 均线 + 50D break                                          | keep：只影响风控不替代基本面     | `risk/01-warning-signals.md`          |
| 0951 | 研究流程与输出纪律：daily routine 需要用 模板完整性 复核                     | 模板完整性 + daily routine                                | revise：写入 playbook 或入口纪律 | `playbook/00-research-routine.md`     |
| 0952 | 研究流程与输出纪律：weekly review 需要用 模板完整性 复核                     | 模板完整性 + weekly review                                | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0953 | 研究流程与输出纪律：quarterly checklist 需要用 模板完整性 复核               | 模板完整性 + quarterly checklist                          | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0954 | 研究流程与输出纪律：event flow 需要用 模板完整性 复核                        | 模板完整性 + event flow                                   | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0955 | 研究流程与输出纪律：decision template 需要用 模板完整性 复核                 | 模板完整性 + decision template                            | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0956 | 研究流程与输出纪律：missing_info handling 需要用 模板完整性 复核             | 模板完整性 + missing_info handling                        | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0957 | 研究流程与输出纪律：source date 需要用 模板完整性 复核                       | 模板完整性 + source date                                  | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0958 | 研究流程与输出纪律：archive boundary 需要用 模板完整性 复核                  | 模板完整性 + archive boundary                             | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0959 | 研究流程与输出纪律：change log 需要用 模板完整性 复核                        | 模板完整性 + change log                                   | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0960 | 研究流程与输出纪律：review cadence 需要用 模板完整性 复核                    | 模板完整性 + review cadence                               | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0961 | 研究流程与输出纪律：daily routine 需要用 动作分类 复核                       | 动作分类 + daily routine                                  | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0962 | 研究流程与输出纪律：weekly review 需要用 动作分类 复核                       | 动作分类 + weekly review                                  | revise：写入 playbook 或入口纪律 | `playbook/00-research-routine.md`     |
| 0963 | 研究流程与输出纪律：quarterly checklist 需要用 动作分类 复核                 | 动作分类 + quarterly checklist                            | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0964 | 研究流程与输出纪律：event flow 需要用 动作分类 复核                          | 动作分类 + event flow                                     | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0965 | 研究流程与输出纪律：decision template 需要用 动作分类 复核                   | 动作分类 + decision template                              | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0966 | 研究流程与输出纪律：missing_info handling 需要用 动作分类 复核               | 动作分类 + missing_info handling                          | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0967 | 研究流程与输出纪律：source date 需要用 动作分类 复核                         | 动作分类 + source date                                    | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0968 | 研究流程与输出纪律：archive boundary 需要用 动作分类 复核                    | 动作分类 + archive boundary                               | defer：写入 playbook 或入口纪律  | `playbook/00-research-routine.md`     |
| 0969 | 研究流程与输出纪律：change log 需要用 动作分类 复核                          | 动作分类 + change log                                     | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0970 | 研究流程与输出纪律：review cadence 需要用 动作分类 复核                      | 动作分类 + review cadence                                 | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0971 | 研究流程与输出纪律：daily routine 需要用 来源日期 复核                       | 来源日期 + daily routine                                  | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0972 | 研究流程与输出纪律：weekly review 需要用 来源日期 复核                       | 来源日期 + weekly review                                  | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0973 | 研究流程与输出纪律：quarterly checklist 需要用 来源日期 复核                 | 来源日期 + quarterly checklist                            | revise：写入 playbook 或入口纪律 | `playbook/00-research-routine.md`     |
| 0974 | 研究流程与输出纪律：event flow 需要用 来源日期 复核                          | 来源日期 + event flow                                     | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0975 | 研究流程与输出纪律：decision template 需要用 来源日期 复核                   | 来源日期 + decision template                              | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0976 | 研究流程与输出纪律：missing_info handling 需要用 来源日期 复核               | 来源日期 + missing_info handling                          | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0977 | 研究流程与输出纪律：source date 需要用 来源日期 复核                         | 来源日期 + source date                                    | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0978 | 研究流程与输出纪律：archive boundary 需要用 来源日期 复核                    | 来源日期 + archive boundary                               | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0979 | 研究流程与输出纪律：change log 需要用 来源日期 复核                          | 来源日期 + change log                                     | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0980 | 研究流程与输出纪律：review cadence 需要用 来源日期 复核                      | 来源日期 + review cadence                                 | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0981 | 研究流程与输出纪律：daily routine 需要用 复盘触发 复核                       | 复盘触发 + daily routine                                  | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0982 | 研究流程与输出纪律：weekly review 需要用 复盘触发 复核                       | 复盘触发 + weekly review                                  | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0983 | 研究流程与输出纪律：quarterly checklist 需要用 复盘触发 复核                 | 复盘触发 + quarterly checklist                            | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0984 | 研究流程与输出纪律：event flow 需要用 复盘触发 复核                          | 复盘触发 + event flow                                     | revise：写入 playbook 或入口纪律 | `playbook/00-research-routine.md`     |
| 0985 | 研究流程与输出纪律：decision template 需要用 复盘触发 复核                   | 复盘触发 + decision template                              | defer：写入 playbook 或入口纪律  | `playbook/00-research-routine.md`     |
| 0986 | 研究流程与输出纪律：missing_info handling 需要用 复盘触发 复核               | 复盘触发 + missing_info handling                          | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0987 | 研究流程与输出纪律：source date 需要用 复盘触发 复核                         | 复盘触发 + source date                                    | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0988 | 研究流程与输出纪律：archive boundary 需要用 复盘触发 复核                    | 复盘触发 + archive boundary                               | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0989 | 研究流程与输出纪律：change log 需要用 复盘触发 复核                          | 复盘触发 + change log                                     | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0990 | 研究流程与输出纪律：review cadence 需要用 复盘触发 复核                      | 复盘触发 + review cadence                                 | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0991 | 研究流程与输出纪律：daily routine 需要用 维护成本 复核                       | 维护成本 + daily routine                                  | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0992 | 研究流程与输出纪律：weekly review 需要用 维护成本 复核                       | 维护成本 + weekly review                                  | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0993 | 研究流程与输出纪律：quarterly checklist 需要用 维护成本 复核                 | 维护成本 + quarterly checklist                            | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0994 | 研究流程与输出纪律：event flow 需要用 维护成本 复核                          | 维护成本 + event flow                                     | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0995 | 研究流程与输出纪律：decision template 需要用 维护成本 复核                   | 维护成本 + decision template                              | revise：写入 playbook 或入口纪律 | `playbook/00-research-routine.md`     |
| 0996 | 研究流程与输出纪律：missing_info handling 需要用 维护成本 复核               | 维护成本 + missing_info handling                          | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0997 | 研究流程与输出纪律：source date 需要用 维护成本 复核                         | 维护成本 + source date                                    | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0998 | 研究流程与输出纪律：archive boundary 需要用 维护成本 复核                    | 维护成本 + archive boundary                               | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 0999 | 研究流程与输出纪律：change log 需要用 维护成本 复核                          | 维护成本 + change log                                     | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |
| 1000 | 研究流程与输出纪律：review cadence 需要用 维护成本 复核                      | 维护成本 + review cadence                                 | keep：写入 playbook 或入口纪律   | `playbook/00-research-routine.md`     |

## 本轮摘要

本轮 1000 次迭代没有把所有想法直接写入主文档。

保留项写入正式框架。
延后项保留为等待数据的检查点。
拒绝项不进入主框架，避免不可复盘指标污染结论。

## 后续执行规则

每次财报、监管或重大产品事件后，从本表选 20-50 个相关微实验重跑。
如果新事实改变处置结果，在本文件追加新编号，不覆盖旧判断。

---

## Defer 复审优先级矩阵（2026-05-08）

### 统计说明

截至 2026-05-08，本文件共有 140 条 defer 迭代。经按复审触发条件分组，共五类：

| 类别 | 触发条件 | 估计条数 | 最早触发时间 | 对应复审文件 |
|------|---------|---------|------------|------------|
| A | 等 Q1 2026 财报（2026-05-11 电话会） | 约 16 条 | 2026-05-11 | `metrics/03-quarterly-earnings.md` |
| B | 等监管最终规则（OCC / Treasury NPRM 最终版） | 约 5 条 | 监管公告日 TBD | `framework/02-regulation.md` |
| C | 等 Arc 主网上线 | 约 4 条 | Arc 主网发布日 TBD | `framework/04-platform-option.md` |
| D | 等数据源稳定（Coinbase API / Base 链 / Dune 口径 / 月度事件） | 约 62 条 | 每季框架自检 | `metrics/00-metric-dictionary.md` |
| E | 已被现有框架覆盖，建议标为 resolved | 约 52 条 | 立即可复审 | `risk/02-failure-conditions.md` |

### A类：等 Q1 2026 财报（优先级 P0，2026-05-11 电话会后 24h 内复审）

| 迭代编号 | 假设核心 | 关键财务指标 | 复审后动作 |
|--------|--------|------------|---------|
| 0020 | 单季成本异常需要管理层解释 | 成本是否一次性 | 电话会后判断 keep / reject |
| 0067 | API 调用量若不披露不能给平台倍数 | API metrics 是否披露 | 有披露→keep；无→继续 defer |
| 0069 | USYC 等资产产品可能扩展 Other revenue | Other revenue 是否拆分 | 有拆分→keep；无→继续 defer |
| 0118 | 成本一次性因素 × 同比与环比 | 同比成本率变化 | 财报后标 keep 或 revise |
| 0135 | distribution cost 弹性 × SEC filing 注释 | 10-Q 注释是否更新 | 财报后标 keep 或 revise |
| 0168 | 分销成本率 × 合作方披露 | Coinbase 分成条款变化 | 有新披露→keep；无→继续 defer |
| 0185 | 第三方钱包激励 × 财报问答 | 电话会是否提及激励变化 | 电话会后判断 keep / reject |
| 0235 | redeemed volume × 同业对比 | 赎回量 vs 同业 | 数据可获→keep；否→维持 defer |
| 0518 | Coinbase 分成压力（渠道议价） | RLDC margin vs 上季度 | 财报后标 keep 或 reject |
| 0535 | 渠道议价：Binance 渠道贡献 | 渠道结构是否变化 | 财报后标 keep 或 reject |
| 0818 | 写入平台化收入阈值（1/2） | Other revenue share 是否 > 10% | 达阈值→keep；否→继续 defer |
| 0835 | 写入平台化收入阈值（2/2） | 同上 | 同上 |
| 0868 | 写入估值锚（基础盘验证 1/2） | RLDC margin + 平均 USDC | 财报后标 keep 并写入估值框架 |
| 0885 | 写入估值锚（基础盘验证 2/2） | 同上 | 同上 |
| 0918 | 风控信号只影响风控不替代基本面（1/2） | 基本面与股价是否背离 | 财报后判断 keep / revise |
| 0935 | 风控信号只影响风控不替代基本面（2/2） | 同上 | 同上 |

**A类中最关键 10 条**（对估值 / 风险判断影响最大，财报日必须优先处理）：

1. **0868**：估值锚直接决定 RLDC 倍数区间，财报后必须写入 `valuation/00-valuation-framework.md`
2. **0818**：平台化收入 > 15% 阈值是情景切换条件，影响牛熊情景分类
3. **0020**：电话会确认成本一次性与否，直接影响下季 RLDC margin 预期
4. **0069**：Other revenue 拆分是否披露，决定平台化期权是否可量化
5. **0067**：API 调用量披露是否达标，决定是否可以赋予平台倍数
6. **0185**：第三方钱包激励变化，直接影响渠道分销成本压力判断
7. **0135**：distribution cost 弹性，是 RLDC margin 核心驱动之一
8. **0518**：Coinbase 分成压力是当季最大渠道风险，财报后必须更新
9. **0235**：redeemed volume 同业对比，影响 P0 触发的精准度
10. **0118**：成本一次性因素，影响 RLDC margin 季度间可比性

### B类：等监管最终规则（优先级 P1）

约 5 条，触发条件为 OCC / Treasury NPRM 最终版发布（日期 TBD）：

| 迭代编号 | 假设核心 | 对应监管规则 |
|--------|--------|------------|
| 0046 | 州级路径可能影响小发行人竞争 | Treasury state regime 最终规则 |
| 0418 | 写入监管矩阵（GENIUS Act 相关 1/2） | GENIUS Act 最终立法（S.394） |
| 0435 | 写入监管矩阵（GENIUS Act 相关 2/2） | 同上 |
| 0468 | 写入 CPN 风险（BSA/AML 约束 1/2） | FinCEN 最终规则 |
| 0485 | 写入 CPN 风险（BSA/AML 约束 2/2） | 同上 |

### C类：等 Arc 主网上线（优先级 P1）

约 4 条，触发条件为 Arc mainnet 正式发布：

| 迭代编号 | 假设核心 | 触发后动作 |
|--------|--------|---------|
| 0718 | 写入平台化验证（Arc 收入 1/2） | 主网上线后 90 天内有 revenue 披露→keep |
| 0735 | 写入平台化验证（Arc 收入 2/2） | 同上 |
| 0768 | 测试网交易不等于收入（1/2） | 主网上线后，有 fee 收入披露→revise；否→keep（原判断成立） |
| 0785 | 测试网交易不等于收入（2/2） | 同上 |

### D类：等数据源稳定（优先级 P2，每季框架自检）

共约 62 条，分两组：

**D1 组（约 14 条）**：等具体数据可获

| 迭代编号 | 假设核心 | 数据阻塞点 |
|--------|--------|---------|
| 0026 | Curve pool imbalance 辅助信号 | 需固定 Dune 仪表盘口径 |
| 0027 | CEX order book depth | 需 Coinbase / Binance API 权限 |
| 0029 | 链上钱包分布集中度 | Dune 口径不稳定 |
| 0037 | 储备 WAM / WAL 尾部流动性 | 月度 BlackRock / SEC 报告 |
| 0039 | 银行通道约束放大赎回风险 | 事件驱动，无常规数据源 |
| 0060 | Uniswap USDC 流动性 | 数据源口径需固定 |
| 0218 | supply seasonality × 周度变化 | 需多季度数据积累 |
| 0318 | 写入利率敏感度（1/2） | 等财报 + Fed 数据对照 |
| 0335 | 写入利率敏感度（2/2） | 同上 |
| 0568 | 写入竞争监控（1/2） | 需稳定竞品数据源 |
| 0585 | 写入竞争监控（2/2） | 同上 |
| 0618 | 写入周度复盘（1/2） | 需稳定链生态数据 |
| 0635 | 写入周度复盘（2/2） | 同上 |
| 0668 | 收益竞争检查（1/2） | 需 RWA / DeFi AUM 稳定来源 |
| 0685 | 收益竞争检查（2/2） | 同上 |

**D2 组（约 48 条，0351–0400 系列）**：月度或事件驱动复核，属"储备资产与银行通道"主题。银行通道事件（如合作银行变更、监管要求）发生时批量复审；无事件时每季框架自检扫一遍。

### E类：已被现有框架覆盖（建议立即标为 resolved）

共约 52 条，分两组：

**E1 组（约 4 条）**：

| 迭代编号 | 假设核心 | 已覆盖文件 |
|--------|--------|---------|
| 0068 | EURC 规模不足时不加权 | `valuation/00-valuation-framework.md` 已有 Other revenue < 15% 规则 |
| 0089 | institutional ownership 是慢变量 | `risk/00-risk-map.md` 已有 13F 季度观察 |
| 0968 | decision template × 复盘触发 | `playbook/01-decision-template.md` 新增错误复盘 SOP 已覆盖 |
| 0985 | decision template × 复盘触发 | 同上 |

**E2 组（约 48 条，0251–0300 系列）**："无稳定源则只做观察"——赎回流动性 / 锚定主题。该系列核心结论已在 `risk/02-failure-conditions.md` 中以"若无官方源，维持观察"形式记录。建议在下次季度自检时批量标记为 resolved（前提：`risk/02-failure-conditions.md` 未发生结构变化）。

### Defer 解决后动作规则

| 复审结果 | 条件 | 动作 |
|---------|-----|-----|
| 标记为 resolved（keep） | 触发条件已满足，有稳定来源，改变或强化现有框架 | 将结论写入对应框架文件；本日志处置更新为 `keep` |
| 重新设定 defer | 触发条件未满足，或数据仍不可获取 | 更新 defer 理由，设定新触发条件（不允许无条件延后） |
| 标记为 rejected | 触发后发现假设不成立，或框架已有更好覆盖 | 本日志处置更新为 `rejected`；不写入框架文件 |

**无限 defer 禁止规则**：同一条目连续两次框架自检仍维持 defer，必须二选一：升级为 keep（接受不完整数据条件下的弱置信结论）或降级为 reject（判定该假设在现有数据条件下不可验证）。
