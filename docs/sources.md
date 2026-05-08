# 来源索引

## 结论

CRCL 框架只把官方来源作为最终事实锚。

二级数据平台用于监控，不用于覆盖财报、监管文件和公司正式披露。

## 公司与财务来源

| 来源                            | 链接                                                                                                            | 用途                                             |
| ------------------------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ |
| Circle FY2025 / Q4 2025 results | `https://www.circle.com/en/pressroom/circle-reports-fourth-quarter-and-full-fiscal-year-2025-financial-results` | FY2025、Q4 2025 财务与经营指标                   |
| Circle Q1 2026 results date     | `https://www.circle.com/pressroom/circle-to-announce-q1-2026-financial-results-on-may-11-2026`                  | Q1 2026 财报日期                                 |
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
| CFTC pressroom                         | `https://www.cftc.gov/PressRoom/PressReleases`                                                                         | 商品监管和执法动态                                         |

NPRM 是 Notice of Proposed Rulemaking，白话就是监管机构发布草案并征求意见。

## 市场与链上来源

| 来源                  | 用途                                   | 注意事项             |
| --------------------- | -------------------------------------- | -------------------- |
| DefiLlama             | 稳定币总市值、市占率、DeFi TVL         | 用于趋势监控         |
| CoinGecko             | 稳定币市值、价格、竞品规模             | 用于交叉验证         |
| Dune                  | 链上分布、转账量、地址活跃度           | 仪表盘口径要逐项核对 |
| CoinMetrics           | 清洗后链上交易量                       | 更适合严谨量化口径   |
| TokenTerminal         | 链和协议层收入、余额、活跃度           | 口径变化要记录       |
| RWA.xyz               | tokenized treasuries 与 RWA AUM        | 用于收益型竞争监控   |
| Kaiko / exchange APIs | 交易所深度、流动性、USDC/USDT 交易质量 | 需要记录口径和权限   |
| NY Fed                | SOFR                                   | 短端美元资金价格     |
| U.S. Treasury         | 3M T-Bill                              | 储备收益压力         |

TVL 是 Total Value Locked，白话就是协议里锁定的资金规模。
AUM 是 Assets Under Management，白话就是管理资产规模。

## 核验纪律

1. 财务数字以 SEC filing 和 Circle earnings release 为准。
2. 监管事实以 Congress、OCC、Treasury、FinCEN、OFAC、SEC、CFTC 为准。
3. 链上数据至少用两个来源交叉验证。
4. 媒体报道只能作为线索，不能作为最终结论。
5. 每个关键事实必须带日期，避免把旧规则当成当前规则。

## 当前已核验事实

截至 2026-05-08：

- Circle FY2025 / Q4 2025 results 已发布，来源为 Circle pressroom 和 SEC exhibit。
- Q1 2026 results 发布日期为 2026-05-11，来源为 Circle pressroom。
- GENIUS Act 于 2025-07-18 成为 Public Law No. 119-27，来源为 Congress.gov。
- OCC 于 2026-02-25 发布 GENIUS Act 拟议规则，来源为 OCC。
- Treasury / FinCEN / OFAC 于 2026-04-08 发布 BSA/AML 与制裁合规拟议规则，来源为 Treasury。
