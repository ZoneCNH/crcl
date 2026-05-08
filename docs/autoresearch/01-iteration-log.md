# 100 次 Autoresearch 迭代日志

## 结论

本文件把 CRCL 研究框架拆成 100 个微实验。

每个微实验都有假设、评估指标、处置规则和文档落点。
后续更新时，只追加新实验，不覆盖历史记录。

## 迭代结果总表

|   # | 微实验假设                                     | 评估指标                                                  | 处置规则                     | 落点                                  |
| --: | ---------------------------------------------- | --------------------------------------------------------- | ---------------------------- | ------------------------------------- |
| 001 | 官方源优先级需要显式高于二级平台               | SEC / Circle / Congress / OCC / Treasury 是否覆盖核心事实 | keep：写入来源纪律           | `docs/sources.md`                     |
| 002 | 财报日期必须用绝对日期，不用“下周”             | Q1 2026 日期是否写成 2026-05-11                           | keep：所有财报节点用绝对日期 | `docs/README.md`                      |
| 003 | GENIUS Act 状态必须区分法案和 Public Law       | Congress.gov 是否显示 Public Law No. 119-27               | keep：写入监管框架           | `framework/02-regulation.md`          |
| 004 | OCC 规则必须标为拟议规则                       | OCC 页面日期和 NPRM 状态                                  | keep：禁止当最终规则         | `framework/02-regulation.md`          |
| 005 | Treasury / FinCEN / OFAC 规则必须标为拟议规则  | Treasury 2026-04-08 公告                                  | keep：写入监管风险           | `sources.md`                          |
| 006 | Circle pressroom 与 SEC exhibit 要交叉验证     | earnings release 与 SEC exhibit 是否一致                  | keep：季度财报复核           | `metrics/03-quarterly-earnings.md`    |
| 007 | 链上平台数据只能当趋势源                       | Dune / DefiLlama 是否有可变口径                           | keep：二级平台降权           | `metrics/00-metric-dictionary.md`     |
| 008 | 媒体报道不能直接进入结论                       | 是否存在官方原文链接                                      | keep：媒体只做线索           | `sources.md`                          |
| 009 | 每条关键事实必须带日期                         | 文档中监管、财报、主网节点是否有日期                      | revise：缺日期时补日期       | `docs/README.md`                      |
| 010 | 原始文档必须保留但降权                         | archive 是否存在三份原始文档                              | keep：归档不作为最新结论     | `docs/archive/`                       |
| 011 | CRCL 当前核心仍是储备经济性                    | Reserve income 占收入大头                                 | keep：按利差股主线研究       | `framework/01-business-model.md`      |
| 012 | P/S 会高估收入质量                             | Distribution and transaction costs 占比                   | keep：估值锚转向 RLDC        | `valuation/00-valuation-framework.md` |
| 013 | 平均 USDC 比期末 USDC 更能解释收入             | average USDC 与 reserve income                            | keep：财报第一优先级         | `metrics/03-quarterly-earnings.md`    |
| 014 | Reserve return rate 比 Fed 标题更精确          | 财报 reserve return rate                                  | keep：季度复核               | `metrics/03-quarterly-earnings.md`    |
| 015 | 分销成本率是隐性核心指标                       | RLDC margin 是否低于 38%                                  | keep：设置降级阈值           | `framework/01-business-model.md`      |
| 016 | Coinbase 增长不必然等于 Circle 盈利增强        | Base supply 与 RLDC margin 是否背离                       | keep：增长质量需财报验证     | `framework/03-competition.md`         |
| 017 | Other revenue 低占比不能支撑平台股估值         | Other revenue share                                       | keep：>15% 才强验证          | `framework/04-platform-option.md`     |
| 018 | Adjusted EBITDA 只能做辅助盈利锚               | 调整项是否影响可比性                                      | keep：不单独作为主锚         | `valuation/00-valuation-framework.md` |
| 019 | FY 指引比单季新闻更重要                        | 指引上修 / 维持 / 下修                                    | keep：季度动作规则           | `metrics/03-quarterly-earnings.md`    |
| 020 | 单季成本异常需要管理层解释                     | 成本异常是否一次性                                        | defer：等电话会问答          | `metrics/03-quarterly-earnings.md`    |
| 021 | USDC 流通量是每日第一生命线                    | Circle Transparency                                       | keep：P0 日检                | `metrics/01-daily-watchlist.md`       |
| 022 | 7D 与 30D 同向下跌比单日变化更重要             | 7D / 30D change                                           | keep：连续性阈值             | `metrics/00-metric-dictionary.md`     |
| 023 | Minted / Redeemed 比净变化更能解释压力         | minted、redeemed 分项                                     | keep：日检记录               | `metrics/01-daily-watchlist.md`       |
| 024 | 连续 2-4 周净赎回是结构风险                    | net mint / redeem                                         | keep：预警信号               | `risk/01-warning-signals.md`          |
| 025 | USDC/USD 折价优先级高于估值                    | 二级市场锚定                                              | keep：失效条件               | `risk/02-failure-conditions.md`       |
| 026 | Curve pool imbalance 是赎回压力辅助信号        | Curve 池比例                                              | defer：数据源需固定          | `risk/00-risk-map.md`                 |
| 027 | CEX order book depth 可用但不可稳定依赖        | Coinbase / Binance API                                    | defer：需要 API 权限         | `metrics/00-metric-dictionary.md`     |
| 028 | 交易所 USDC 余额增长质量低于支付增长           | exchange balances                                         | keep：周检分类               | `metrics/02-weekly-review.md`         |
| 029 | 链上钱包分布能提示集中度风险                   | holder distribution                                       | defer：口径依赖 Dune 仪表盘  | `metrics/02-weekly-review.md`         |
| 030 | 企业支付增长质量最高但披露不足                 | CPN / 财报披露                                            | keep：P2 追踪                | `framework/04-platform-option.md`     |
| 031 | 3M T-Bill 是储备收益压力主代理                 | Treasury / FRED                                           | keep：P0 日检                | `metrics/01-daily-watchlist.md`       |
| 032 | SOFR 是短端美元资金价格补充                    | NY Fed                                                    | keep：P0 日检                | `metrics/00-metric-dictionary.md`     |
| 033 | Circle Reserve Fund yield 比宏观利率更贴近实际 | BlackRock fund yield                                      | keep：P0 监控                | `metrics/00-metric-dictionary.md`     |
| 034 | 降息不是线性利空                               | USDC 增速是否抵消收益率下行                               | keep：估值框架               | `valuation/00-valuation-framework.md` |
| 035 | 利率下行 + USDC 不增长是双杀                   | reserve return rate 与 average USDC                       | keep：Bear case              | `valuation/01-scenario-model.md`      |
| 036 | 高利率也可能推资金去收益型竞品                 | RWA / USDe AUM                                            | keep：竞争监控               | `framework/03-competition.md`         |
| 037 | 储备 WAM / WAL 是尾部流动性指标                | BlackRock / SEC 报告                                      | defer：月度或季度获取        | `risk/00-risk-map.md`                 |
| 038 | Daily / weekly liquidity 是监管合规指标        | 储备披露和规则要求                                        | keep：风险地图               | `risk/00-risk-map.md`                 |
| 039 | 银行通道约束是赎回风险放大器                   | banking partners 变化                                     | defer：事件驱动              | `risk/02-failure-conditions.md`       |
| 040 | Weekend redemption constraints 只能作风险提示  | 服务条款与用户反馈                                        | reject：无稳定公开指标       | `risk/00-risk-map.md`                 |
| 041 | 监管不是单向利好                               | 合规壁垒 vs 分销限制                                      | keep：监管框架核心结论       | `framework/02-regulation.md`          |
| 042 | 非银行发行路径清晰度影响 Circle 护城河         | PPSI 路径                                                 | keep：监管矩阵               | `framework/02-regulation.md`          |
| 043 | 收益分享禁令影响平台叙事                       | interest / yield 条款                                     | keep：P0 风险                | `framework/02-regulation.md`          |
| 044 | 第三方激励是否被视为变相 yield 是最大观察点    | OCC / Treasury 最终规则                                   | keep：P0 事件规则            | `risk/01-warning-signals.md`          |
| 045 | BSA/AML 成本会影响跨境支付效率                 | FinCEN / OFAC 要求                                        | keep：CPN 风险               | `framework/04-platform-option.md`     |
| 046 | 州级路径可能影响小发行人竞争                   | Treasury state regime 规则                                | defer：看最终规则            | `framework/02-regulation.md`          |
| 047 | SEC 执法会影响交易所分销                       | SEC newsroom                                              | keep：P0 事件源              | `sources.md`                          |
| 048 | CFTC 行动会影响稳定币商品属性预期              | CFTC pressroom                                            | keep：事件源                 | `sources.md`                          |
| 049 | Fed 稳定币讲话影响银行参与预期                 | Fed events                                                | keep：来源索引               | `sources.md`                          |
| 050 | 监管评论期结论不能提前当成最终规则             | NPRM 状态                                                 | keep：核验纪律               | `sources.md`                          |
| 051 | USDT 是规模竞争，不是唯一竞争                  | USDC / USDT ratio                                         | keep：竞争框架               | `framework/03-competition.md`         |
| 052 | 稳定币总盘扩张决定行业顺风                     | total stablecoin market cap                               | keep：周检                   | `metrics/02-weekly-review.md`         |
| 053 | USDC 市占率连续 4 周下降是竞争预警             | market share trend                                        | keep：风险信号               | `risk/01-warning-signals.md`          |
| 054 | Bank stablecoin 抢的是企业和机构心智           | 银行发行公告                                              | keep：竞争结构               | `framework/03-competition.md`         |
| 055 | tokenized deposits 会压缩长期倍数              | 银行代币化存款进展                                        | keep：估值折价项             | `valuation/00-valuation-framework.md` |
| 056 | PYUSD、FDUSD 是份额迁移观察项                  | 市值与链上使用                                            | keep：P2 监控                | `framework/03-competition.md`         |
| 057 | USDe / sUSDe 是收益竞争                        | AUM 与收益率                                              | keep：周检                   | `metrics/02-weekly-review.md`         |
| 058 | BUIDL / USDY / USYC 抢机构闲置美元             | RWA AUM                                                   | keep：竞争监控               | `framework/03-competition.md`         |
| 059 | DeFi deposits 增长需要拆存款和借款             | Aave / Compound deposit-borrow                            | keep：周检                   | `metrics/02-weekly-review.md`         |
| 060 | Uniswap liquidity 是交易需求辅助指标           | USDC pair liquidity                                       | defer：数据源要固定          | `metrics/02-weekly-review.md`         |
| 061 | CPN 参与机构数只能证明早期采用                 | institutions joined                                       | keep：弱验证                 | `framework/04-platform-option.md`     |
| 062 | CPN TPV 必须和收入同步验证                     | TPV 与 Other revenue                                      | keep：商业验证规则           | `framework/04-platform-option.md`     |
| 063 | CPN 收入确认方式比 TPV 更重要                  | 财报注释 / call transcript                                | keep：财报问答               | `metrics/03-quarterly-earnings.md`    |
| 064 | Arc 测试网交易不等于收入                       | testnet tx vs mainnet revenue                             | keep：平台化风险             | `framework/04-platform-option.md`     |
| 065 | Arc 主网时间表是期权兑现前置条件               | 2026 主网目标                                             | keep：P2 追踪                | `framework/04-platform-option.md`     |
| 066 | Arc fee model 决定是否有独立收入               | gas / validator / app fees                                | keep：财报问答               | `metrics/03-quarterly-earnings.md`    |
| 067 | API 调用量若不披露，不能给平台倍数             | API metrics                                               | defer：等披露                | `framework/04-platform-option.md`     |
| 068 | EURC 是可选上行，不是主框架核心                | EURC supply / revenue                                     | defer：规模不足时不加权      | `valuation/00-valuation-framework.md` |
| 069 | USYC 等资产产品可能扩展 Other revenue          | product AUM / fees                                        | defer：等财报拆分            | `framework/04-platform-option.md`     |
| 070 | 平台化必须避免一次性 integration services 误判 | recurring vs one-time revenue                             | keep：保守规则               | `framework/04-platform-option.md`     |
| 071 | Market cap / USDC circulation 只能作粗锚       | 市值 / USDC 流通量                                        | keep：估值锚局限             | `valuation/00-valuation-framework.md` |
| 072 | Market cap / RLDC 更适合收入质量               | 市值 / annualized RLDC                                    | keep：主估值锚               | `valuation/00-valuation-framework.md` |
| 073 | EV / Adjusted EBITDA 用于传统对照              | EV / Adj. EBITDA                                          | keep：辅助锚                 | `valuation/00-valuation-framework.md` |
| 074 | Other revenue multiple 必须等可重复性披露      | recurring Other revenue                                   | keep：期权盘                 | `valuation/00-valuation-framework.md` |
| 075 | Bull case 必须同时满足基础盘和期权盘           | USDC、RLDC、Other revenue                                 | keep：情景模型               | `valuation/01-scenario-model.md`      |
| 076 | Base case 不能因单一产品公告上修               | 指标是否多线共振                                          | keep：情景纪律               | `valuation/01-scenario-model.md`      |
| 077 | Bear case 需要把监管和筹码共振纳入             | rules + unlock + IV                                       | keep：情景模型               | `valuation/01-scenario-model.md`      |
| 078 | 估值安全边际要扣除监管折价                     | safety margin formula                                     | keep：估值框架               | `valuation/00-valuation-framework.md` |
| 079 | 平台股切换必须设置硬阈值                       | Other revenue >15%                                        | keep：切换条件               | `valuation/00-valuation-framework.md` |
| 080 | 股价上涨不等于研究框架强化                     | 基本面指标是否同步                                        | keep：输出纪律               | `playbook/01-decision-template.md`    |
| 081 | 储备异常优先级高于所有估值讨论                 | reserve report status                                     | keep：失效条件               | `risk/02-failure-conditions.md`       |
| 082 | USDC/USD 折价要立刻切信用框架                  | peg deviation                                             | keep：失效条件               | `risk/02-failure-conditions.md`       |
| 083 | short interest 单独上升不是结构风险            | short interest + price/volume                             | keep：中等预警               | `risk/01-warning-signals.md`          |
| 084 | borrow fee 需要券商源，不适合默认框架          | IBKR / broker data                                        | reject：不可公共复盘         | `risk/01-warning-signals.md`          |
| 085 | put/call ratio 和 IV 共振才有意义              | options data                                              | keep：筹码信号               | `risk/01-warning-signals.md`          |
| 086 | lock-up expiration 是事件风险                  | SEC filings / prospectus                                  | keep：P2 监控                | `risk/01-warning-signals.md`          |
| 087 | secondary offering 直接影响稀释                | SEC filing                                                | keep：P0/P2 事件             | `risk/01-warning-signals.md`          |
| 088 | insider selling 必须看持续性和规模             | Form 4                                                    | keep：筹码风险               | `risk/00-risk-map.md`                 |
| 089 | institutional ownership 是慢变量               | 13F                                                       | defer：季度观察              | `risk/00-risk-map.md`                 |
| 090 | 技术均线只能放大风控，不改基本面               | 20D / 50D                                                 | keep：中等预警               | `metrics/01-daily-watchlist.md`       |
| 091 | 日检只能回答是否重估风险                       | P0 指标                                                   | keep：流程纪律               | `playbook/00-research-routine.md`     |
| 092 | 周检必须回答增长质量                           | supply + share + volume                                   | keep：周报模板               | `metrics/02-weekly-review.md`         |
| 093 | 季检必须先算 RLDC margin                       | 财报计算表                                                | keep：季报模板               | `metrics/03-quarterly-earnings.md`    |
| 094 | 事件流程必须先确认来源再判断                   | official source check                                     | keep：事件流程               | `playbook/00-research-routine.md`     |
| 095 | 输出必须先结论再依据再动作                     | decision template                                         | keep：输出模板               | `playbook/01-decision-template.md`    |
| 096 | 每条结论必须落到增强、降级、观察               | action class                                              | keep：入口纪律               | `docs/README.md`                      |
| 097 | 缺失数据要写 missing_info，而不是脑补          | unavailable metric                                        | keep：输出纪律               | `playbook/01-decision-template.md`    |
| 098 | 文档迭代要保留来源和日期                       | change log rule                                           | keep：更新规则               | `docs/README.md`                      |
| 099 | archive 只存历史，不参与最新判断               | archive path                                              | keep：目录职责               | `docs/README.md`                      |
| 100 | 100 次迭代后仍需继续用同一目标函数             | research score                                            | keep：autoresearch 方法      | `autoresearch/00-loop.md`             |

## 本轮摘要

本轮 100 次迭代没有把所有想法直接写入主文档。

保留项写入正式框架。
延后项保留为等待数据的检查点。
拒绝项不进入主框架，避免不可复盘指标污染结论。

## 后续执行规则

每次财报、监管或重大产品事件后，从本表选 5-10 个相关微实验重跑。
如果新事实改变处置结果，在本文件追加新编号，不覆盖旧判断。
