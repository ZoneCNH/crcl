# 来源索引

## 结论

CRCL 框架只把官方来源作为最终事实锚。

二级数据平台用于监控，不用于覆盖财报、监管文件和公司正式披露。

本文件是来源入口，不是事实快照。大模型每次运行时必须重新访问或核验下列来源；表中的历史日期和旧链接只说明曾经使用过，不代表当前仍然最新。

## 公司与财务来源

| 来源                            | 链接                                                                                                            | 用途                                             |
| ------------------------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ |
| Circle FY2025 / Q4 2025 results | `https://www.circle.com/en/pressroom/circle-reports-fourth-quarter-and-full-fiscal-year-2025-financial-results` | FY2025、Q4 2025 财务与经营指标                   |
| Circle pressroom / earnings archive | `https://www.circle.com/pressroom`                                                                            | 最新财报公告、历史财报公告、事件公告             |
| Circle investor events          | `https://investor.circle.com/events-and-presentations/default.aspx`                                             | earnings call、webcast、投资者材料               |
| Circle Transparency             | `https://www.circle.com/transparency`                                                                           | USDC 流通量、储备透明度                          |
| SEC EDGAR                       | `https://www.sec.gov/edgar/search/`                                                                             | 10-K、10-Q、Form 4、S-1、13F                     |
| Coinbase investor relations     | `https://investor.coinbase.com/`                                                                                | Coinbase products USDC、Base、stablecoin revenue |
| Tether official attestations    | `https://tether.io/en/transparency/`                                                                            | USDT 储备、流通量、利润、Treasury exposure       |

earnings call 是财报电话会，白话就是管理层解释财报并回答分析师问题。

## 监管来源

| 来源                                   | 链接                                                                                                                   | 用途                                                       |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------- |
| GENIUS Act                             | `https://www.congress.gov/bill/119th-congress/senate-bill/1582`                                                        | 法案状态、文本、Public Law 信息                            |
| GENIUS Act public law text             | `https://www.congress.gov/bill/119th-congress/senate-bill/1582/text/pl`                                                | 正式法律文本                                               |
| OCC GENIUS Act NPRM                    | `https://www.occ.gov/news-issuances/news-releases/2026/nr-occ-2026-9.html`                                             | OCC 支付稳定币拟议规则                                     |
| Treasury / FinCEN / OFAC proposed rule | `https://home.treasury.gov/news/press-releases/sb0435`                                                                 | BSA/AML 与制裁合规拟议规则                                 |
| FDIC GENIUS Act proposed rule          | `https://www.fdic.gov/news/press-releases/2026/fdic-approves-proposal-implement-genius-act-requirements-and-standards` | FDIC-supervised PPSI、tokenized deposits、储备存款保险口径 |
| Federal Reserve                        | `https://www.federalreserve.gov/newsevents.htm`                                                                        | Fed 对稳定币、支付、银行监管表态                           |
| SEC newsroom                           | `https://www.sec.gov/newsroom`                                                                                         | 证券监管、执法和披露规则                                   |
| CFTC RSS                               | `https://www.cftc.gov/rss.xml`                                                                                         | 商品监管和执法动态                                         |

NPRM 是 Notice of Proposed Rulemaking，白话就是监管机构发布草案并征求意见。

## 市场与链上来源

| 来源                  | 用途                                   | 注意事项             |
| --------------------- | -------------------------------------- | -------------------- |
| DefiLlama             | 稳定币总市值、市占率、DeFi TVL         | 用于趋势监控         |
| CoinGecko             | 稳定币市值、价格、竞品规模             | 用于交叉验证         |
| Dune                  | 链上分布、转账量、地址活跃度           | 仪表盘口径要逐项核对 |
| CoinMetrics           | 清洗后链上交易量                       | 更适合严谨量化口径   |
| CoinGlass             | 交易所 USDC balance、24h/7d/30d 变化、历史余额、集中度 | 前台公开接口用于监控；Open API key 仅作交叉验证，需记录前端解密/字段变更风险 |
| TokenTerminal         | 链和协议层收入、余额、活跃度           | 口径变化要记录       |
| RWA.xyz               | tokenized treasuries 与 RWA AUM        | 用于收益型竞争监控   |
| Kaiko / exchange APIs | 交易所深度、流动性、USDC/USDT 交易质量 | 需要记录口径和权限   |
| NY Fed                | SOFR                                   | 短端美元资金价格     |
| U.S. Treasury         | 3M T-Bill                              | 储备收益压力         |

TVL 是 Total Value Locked，白话就是协议里锁定的资金规模。
AUM 是 Assets Under Management，白话就是管理资产规模。

## 来源有效性检查

来源 URL 会随公司或监管机构的网站结构变化而失效。每次框架自检时按以下方式核查：

### 检查规则

| 来源类型 | 检查方式 | 失效信号 | 处理 |
| -------- | -------- | -------- | ---- |
| Circle pressroom / investor relations | 访问页面确认内容是否当前版本 | 页面 404 或内容已过期 | 搜索 Circle 官网最新链接替换 |
| SEC EDGAR | EDGAR search 搜索公司 CIK 号（1876042）查最新 filing | 链接指向旧 filing | 换用 EDGAR 搜索入口，不用硬链接 |
| OCC / Treasury / Congress | 访问确认页面状态和文件日期 | 规则状态变化（如 NPRM → Final Rule） | 更新链接并在"运行时事实刷新清单"和来源检查记录中标注新状态 |
| DefiLlama / CoinGecko / Dune | 访问页面确认数据仍在更新 | 数据停更超 30 天 | 换用替代来源，或在使用时标注"数据截至 YYYY-MM-DD" |
| RWA.xyz | 同上 | 同上 | 同上 |

### 来源检查记录

| 检查日期 | 发现问题 | 处理结果 |
| -------- | -------- | -------- |
| 2026-05-08（历史快照） | 无 | 初始建档时全部来源核验有效 |

每次框架自检或重大事件复核时，需要追加新的检查记录。禁止用上表历史记录证明当前来源仍有效。

## 数据获取执行规格

各来源的具体获取步骤（URL、操作顺序、备用源、告警触发）已整理至：

→ **`superpowers/specs/data-collection-sop.md`**

包含 11 个数据源的 SOP（Circle Transparency、DefiLlama、Treasury、Dune Analytics、SEC EDGAR、OCC/Congress/FinCEN 等）及每周操作日历。

## 核验纪律

1. 财务数字以 SEC filing 和 Circle earnings release 为准。
2. 监管事实以 Congress、OCC、Treasury、FinCEN、OFAC、SEC、CFTC 为准。
3. 链上数据至少用两个来源交叉验证。
4. 媒体报道只能作为线索，不能作为最终结论。
5. 每个关键事实必须带日期，避免把旧规则当成当前规则。

## 运行时事实刷新清单

以下项目每次运行前刷新。若无法刷新，必须标记 `missing_info`：

| 事实类型 | 刷新动作 | 输出要求 |
| -------- | -------- | -------- |
| 最新财报 | 查 SEC EDGAR 最新 10-Q / 10-K / 8-K，并与 Circle IR 对照 | 写明 filing 类型、filing 日期、核心数字来源 |
| 最新 USDC 供应 | 查 Circle Transparency，并用 DefiLlama / CoinGecko 交叉验证异常值 | 写明数据日期、7D/30D、net mint/redeem |
| 最新监管状态 | 查 Congress、OCC、Treasury、FinCEN、OFAC、FDIC | 区分法律、拟议规则、最终规则、评论期和媒体线索 |
| 最新竞争数据 | 查 DefiLlama、CoinGecko、CoinMetrics、Dune、RWA.xyz | 写明快照日期和口径，偏差超阈值填 `missing_info` |
| 最新筹码数据 | 查 SEC Form 4 / 13F、FINRA short interest | 只影响仓位纪律，不直接改基本面情景 |
