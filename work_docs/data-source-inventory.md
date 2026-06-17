# CRCL 数据源完整清单

生成日期：2026-06-16

## 结论

本项目需要的数据源分成五类：市场与链上、利率与储备、SEC 财务披露、事件与监管、技术状态。

当前自动采集器已经覆盖大部分公开可自动化来源，并落到 `data/crcl_research.sqlite`。本次把原“后续补齐”里的 CPN、Arc、机构持仓和 Other revenue share 改为可自动解析；Exchange USDC balances 已从 Glassnode 切到 CoinGlass 公开 API，但该 API 需要 `COINGLASS_API_KEY`。

- `collection_batches=8`
- `source_runs=152`
- `observations=618`
- `filings=184`
- `events=18`
- `missing_items=1`

当前仍可能留在 `missing_items` 的自动化缺口是：

| 优先级 | 指标 | 当前问题 | 来源 |
| --- | --- | --- | --- |
| P1 | `P1_EXCHANGE_USDC_BALANCES` / Exchange USDC balances | 已改用 CoinGlass Exchange Balance List；官方文档确认支持 `USDC`，但无 `CG-API-KEY` 时接口返回 `API key missing` | `https://open-api-v4.coinglass.com/api/exchange/balance/list?symbol=USDC` |

## 状态说明

| 状态 | 含义 |
| --- | --- |
| 已落库 | 采集器已经自动获取并写入 `observations`、`filings` 或 `events` |
| 已访问 | 已自动访问来源并记录 source check / page title / 可达性，但没有深度结构化提取 |
| 部分 | 有替代口径或低频口径已落库，但仍缺主口径、交叉验证或人工补证 |
| 未自动化 | 文档框架需要，但当前没有结构化采集器 |
| 失败 | 有采集器，但最近运行失败并写入 `missing_items` |

## P0 日度 / 事件触发数据

| 数据项 | 首选来源 | 获取地址 | 当前状态 | 落库位置 / 说明 |
| --- | --- | --- | --- | --- |
| USDC circulating supply | Circle Transparency | `https://www.circle.com/transparency` | 已落库 | `observations`：`P0_USDC_CIRCULATING_SUPPLY` |
| USDC 7D / 30D / 365D change | Circle Transparency | `https://www.circle.com/transparency` | 已落库 | `observations`：`P0_USDC_7D_CHANGE_PCT`、`P0_USDC_30D_CHANGE_PCT`；365D 变化以发行/赎回和属性口径记录 |
| USDC minted / redeemed / net issuance | Circle Transparency | `https://www.circle.com/transparency` | 已落库 | `observations`：7D / 30D / 365D issued、redeemed、net issuance |
| USDC total reserves | Circle Transparency | `https://www.circle.com/transparency` | 已落库 | `observations`：`P0_CIRCLE_USDC_TOTAL_RESERVES` |
| USDC reserve composition | Circle Transparency | `https://www.circle.com/transparency` | 已落库 | `observations`：SII deposits、other bank deposits、short Treasuries、overnight repo |
| 3M T-Bill yield | U.S. Treasury | `https://home.treasury.gov/resource-center/data-chart-center/interest-rates/pages/xml?data=daily_treasury_yield_curve&field_tdr_date_value_month=YYYYMM` | 已落库 | `observations`：`P0_TREASURY_3M_YIELD` |
| 1Y Treasury yield | U.S. Treasury | 同上 | 已落库 | `observations`：`P0_TREASURY_1Y_YIELD` |
| 5Y Treasury yield | U.S. Treasury | 同上 | 已落库 | `observations`：`P0_TREASURY_5Y_YIELD` |
| SOFR | NY Fed | `https://markets.newyorkfed.org/api/rates/secured/sofr/last/1.json` | 已落库 | `observations`：`P0_SOFR` |
| Circle Reserve Fund 7-day net yield | SEC EDGAR / BlackRock N-MFP3 | `https://data.sec.gov/submissions/CIK0000844779.json` | 已落库 | `filings` + `observations`：`P0_CIRCLE_RESERVE_FUND_7D_YIELD` |
| Circle Reserve Fund gross yield | SEC EDGAR / BlackRock N-MFP3 | 同上 | 已落库 | `observations`：`P2_CIRCLE_RESERVE_FUND_7D_GROSS_YIELD` |
| Circle Reserve Fund WAM / WAL / net assets | SEC EDGAR / BlackRock N-MFP3 | 同上 | 已落库 | `observations`：`P2_RESERVE_FUND_WAM`、`P2_RESERVE_FUND_WAL`、`P2_RESERVE_FUND_NET_ASSETS` |
| USDC/USD peg check | CoinGecko | `https://api.coingecko.com/api/v3/simple/price?ids=usd-coin,tether,bitcoin,ethereum,paypal-usd,first-digital-usd,ethena-usde,ondo-us-dollar-yield&vs_currencies=usd&include_market_cap=true&include_24hr_vol=true&include_24hr_change=true` | 已落库 | `observations`：`P0_USDC_USD_PRICE` |
| USDT/USD peg check | CoinGecko | 同上 | 已落库 | `observations`：`P0_USDT_USD_PRICE` |
| BTC / ETH price | CoinGecko | 同上 | 已落库 | `observations`：`P0_BTC_PRICE`、`P0_ETH_PRICE` |
| CRCL price / volume | Yahoo Finance | `https://query1.finance.yahoo.com/v8/finance/chart/CRCL?range=5d&interval=1d` | 已落库 | `observations`：`P0_CRCL_PRICE`、`P0_CRCL_VOLUME` |
| Curve 3pool USDC ratio / balance / liquidity | Curve API | `https://api.curve.fi/api/getPools/ethereum/main` | 已落库 | `observations`：`P0_CURVE_3POOL_USDC_RATIO`、USDC/USDT/DAI balance、total liquidity |
| Circle pressroom announcements | Circle Pressroom | `https://www.circle.com/pressroom` | 已落库 | `events`：`company_press_release` |
| Circle investor events | Circle IR | `https://investor.circle.com/events-and-presentations/default.aspx` | 已访问 | `source_runs` + `events`：只做页面可达性 / title 检查，具体电话会内容仍需人工或后续解析 |
| Circle 8-K / major filings | SEC EDGAR submissions | `https://data.sec.gov/submissions/CIK0001876042.json` | 已落库 | `filings` + `observations`：最新 8-K、10-Q、10-K、Form 4 元数据 |
| Base status | Statuspage | `https://status.base.org/api/v2/status.json` | 已落库 | `observations`：`P0_CHAIN_STATUS_BASE` |
| Solana status | Statuspage | `https://status.solana.com/api/v2/status.json` | 已落库 | `observations`：`P0_CHAIN_STATUS_SOLANA` |
| Circle service status | Statuspage | `https://status.circle.com/api/v2/status.json` | 已落库 | `observations`：`P0_CIRCLE_STATUS` |
| Ethereum latest block freshness | Public Ethereum JSON-RPC | `https://ethereum.publicnode.com` | 已落库 | `observations`：`P0_CHAIN_STATUS_ETHEREUM`；不是官方 Ethereum Foundation status page |

## P1 周度数据

| 数据项 | 首选来源 | 获取地址 | 当前状态 | 落库位置 / 说明 |
| --- | --- | --- | --- | --- |
| Stablecoin total market cap | DefiLlama | `https://stablecoins.llama.fi/stablecoins?includePrices=true` | 已落库 | `observations`：`P1_STABLECOIN_TOTAL_MARKET_CAP` |
| USDC market cap share | DefiLlama；CoinGecko 交叉验证 | 同上；CoinGecko simple price API | 已落库 | `observations`：`P1_USDC_MARKET_CAP_SHARE` |
| USDC / USDT ratio | DefiLlama | `https://stablecoins.llama.fi/stablecoins?includePrices=true` | 已落库 | `observations`：`P1_USDC_USDT_RATIO` |
| USDT market cap | DefiLlama | 同上 | 已落库 | `observations`：`P1_USDT_MARKET_CAP` |
| USDC by chain | DefiLlama；Dune / TokenTerminal 交叉验证 | `https://stablecoins.llama.fi/stablecoins?includePrices=true`；`https://dune.com` | 部分 | DefiLlama 链分布已落库：Ethereum、Base、Solana、Arbitrum、Polygon、Avalanche；Dune 固定 query / 截图未自动化 |
| USDC adjusted transfer volume | Visa Onchain Analytics / Allium | `https://app-server-dp-xjpv5b26pq-uw.a.run.app/api/v1/explorer/results/data?format=json`；页面入口 `https://visaonchainanalytics.com/transactions` | 已落库 | `observations`：`P1_USDC_ADJUSTED_TRANSFER_VOLUME`，含最近完整 UTC 日和 30D 合计属性 |
| USDC active addresses | CoinMetrics Community API；Dune / Santiment 交叉验证 | `https://community-api.coinmetrics.io/v4/timeseries/asset-metrics?assets=usdc&metrics=AdrActCnt,TxCnt,TxTfrCnt&frequency=1d&page_size=1` | 已落库 | `observations`：`P1_USDC_ACTIVE_ADDRESSES`；Dune / Santiment 未自动化 |
| USDC transaction count | CoinMetrics Community API | 同上 | 已落库 | `observations`：`P1_USDC_TRANSACTION_COUNT` |
| USDC transfer count | CoinMetrics Community API | 同上 | 已落库 | `observations`：`P1_USDC_TRANSFER_COUNT` |
| USDC adjusted transfer value 交叉验证 | CoinMetrics Pro / Dune | CoinMetrics Pro；`https://dune.com` | 部分 | 主自动源已改用 Visa / Allium；CoinMetrics Community API 对 `TxTfrValAdjUSD` 需要付费凭证 |
| USDC velocity | Circle SEC filing；Dune / CoinMetrics Pro 周度 adjusted 口径 | Circle SEC filings；`https://dune.com` | 部分 | filing-period velocity 已从 Circle filing 落库：`P1_USDC_VELOCITY`；周度 adjusted velocity 未自动化 |
| Exchange USDC balances | CoinGlass Exchange Balance List | `https://open-api-v4.coinglass.com/api/exchange/balance/list?symbol=USDC`；文档入口 `https://docs.coinglass.com/reference/exchange-balance-list` | 需凭证 | `observations`：`P1_EXCHANGE_USDC_BALANCES`；公开 API 支持 USDC，但需环境变量 `COINGLASS_API_KEY` |
| Aave V3 USDC deposits | DefiLlama protocol API | `https://api.llama.fi/protocol/aave-v3` | 已落库 | `observations`：协议总额和链级拆分 |
| Compound V3 USDC deposits | DefiLlama protocol API | `https://api.llama.fi/protocol/compound-v3` | 已落库 | `observations`：协议总额和链级拆分 |
| Aave + Compound aggregate USDC deposits | DefiLlama protocol API | `https://api.llama.fi/protocol/aave-v3`；`https://api.llama.fi/protocol/compound-v3` | 已落库 | `observations`：`P1_DEFI_PROTOCOL_USDC_DEPOSITS` |
| Coinbase stablecoin revenue / USDC exposure | Coinbase SEC filings | `https://data.sec.gov/submissions/CIK0001679788.json` | 已落库 | `observations`：stablecoin revenue、receivable、financing receivable、client custodial funds、USDC on platform |
| Base USDC growth | DefiLlama chain distribution；Dune / TokenTerminal 交叉验证 | DefiLlama stablecoins API；`https://dune.com` | 部分 | Base 链 USDC 余额已从 DefiLlama 落库；Dune / TokenTerminal 未自动化 |
| PYUSD market cap | CoinGecko | CoinGecko simple price API | 已落库 | `observations`：`P1_COMPETITOR_PYUSD_MARKET_CAP` |
| FDUSD market cap | CoinGecko | CoinGecko simple price API | 已落库 | `observations`：`P1_COMPETITOR_FDUSD_MARKET_CAP` |
| USDe market cap | CoinGecko | CoinGecko simple price API | 已落库 | `observations`：`P1_COMPETITOR_USDE_MARKET_CAP` |
| Ondo USDY market cap | CoinGecko | CoinGecko simple price API | 已落库 | `observations`：`P1_COMPETITOR_USDY_MARKET_CAP` |
| Tokenized Treasury AUM | RWA.xyz | `https://app.rwa.xyz/treasuries`；官方 API 备选 `https://api.rwa.xyz/v4/assets` + `RWA_API_KEY` | 已落库 | `observations`：`P1_TOKENIZED_TREASURY_AUM` |
| BlackRock BUIDL AUM | RWA.xyz | `https://app.rwa.xyz/treasuries` | 已落库 | `observations`：`P1_TOKENIZED_TREASURY_BUIDL_AUM` |
| CPN TPV | Circle earnings disclosure / pressroom | `https://www.circle.com/pressroom/circle-reports-first-quarter-2026-results` | 已落库 | `observations`：`P1_CPN_ANNUALIZED_TPV`；Q1 2026 pressroom 披露 trailing 30 day annualized TPV |
| Bank stablecoin events | OCC / FDIC / bank releases | OCC、FDIC、银行新闻稿 | 已访问 | OCC / FDIC 页面已 source check；银行个体新闻稿未系统化 |
| Tokenized deposits events | FDIC / bank releases | `https://www.fdic.gov/news/press-releases/`；银行公告 | 已访问 | FDIC 页面已 source check；具体银行事件仍需人工或后续事件解析 |

## P2 季度 / 事件数据

| 数据项 | 首选来源 | 获取地址 | 当前状态 | 落库位置 / 说明 |
| --- | --- | --- | --- | --- |
| Circle reserve income | SEC 10-Q / 10-K | `https://data.sec.gov/submissions/CIK0001876042.json` | 已落库 | `observations`：`P2_CIRCLE_RESERVE_INCOME` |
| Circle distribution and transaction costs | SEC 10-Q / 10-K | 同上 | 已落库 | `observations`：`P2_CIRCLE_DISTRIBUTION_TRANSACTION_COSTS`、`P2_CIRCLE_TOTAL_DISTRIBUTION_TRANSACTION_OTHER_COSTS` |
| Circle RLDC | 自算，基于 SEC filing | 同上 | 已落库 | `observations`：`P2_CIRCLE_RLDC` |
| Circle RLDC margin | 自算，基于 SEC filing | 同上 | 已落库 | `observations`：`P2_CIRCLE_RLDC_MARGIN` |
| Circle other revenue | SEC 10-Q / 10-K | 同上 | 已落库 | `observations`：`P2_CIRCLE_OTHER_REVENUE` |
| Circle other revenue share | 自算，基于 SEC filing | 同上 | 已落库 | `observations`：`P2_CIRCLE_OTHER_REVENUE_SHARE`；由 Other revenue / total revenue and reserve income 派生 |
| Circle adjusted EBITDA | SEC 10-Q / 10-K | 同上 | 已落库 | `observations`：`P2_CIRCLE_ADJUSTED_EBITDA`、`P2_CIRCLE_ADJUSTED_EBITDA_MARGIN` |
| Circle total revenue and reserve income | SEC 10-Q / 10-K | 同上 | 已落库 | `observations`：`P2_CIRCLE_TOTAL_REVENUE_AND_RESERVE_INCOME` |
| Circle USDC average / end-period circulation | SEC 10-Q / 10-K | 同上 | 已落库 | `observations`：`P2_CIRCLE_USDC_IN_CIRCULATION_AVG_PERIOD`、`P2_CIRCLE_USDC_IN_CIRCULATION_END_PERIOD` |
| Circle onchain transaction volume | SEC 10-Q / 10-K | 同上 | 已落库 | `observations`：`P1_USDC_ONCHAIN_TRANSACTION_VOLUME` |
| CRCL basic / diluted / common shares | SEC 10-Q / 10-K inline XBRL | 同上 | 已落库 | `observations`：`P2_CRCL_BASIC_SHARES_OUTSTANDING`、`P2_CRCL_DILUTED_SHARES_OUTSTANDING`、`P2_CRCL_SHARES_OUTSTANDING` |
| Coinbase distribution exposure | Circle / Coinbase SEC filings | Circle CIK `0001876042`；Coinbase CIK `0001679788` | 已落库 | Coinbase stablecoin revenue、USDC on platform、client custodial funds 等已落库；精确渠道分成仍依赖披露口径 |
| CPN TPV / customer count / fee model | Circle disclosure / earnings call | Circle IR、SEC 8-K、10-Q、earnings materials；Q1 2026 pressroom | 部分 | TPV 已自动提取；客户数、收费模式仍需 transcript / presentation 或人工标注 |
| Arc mainnet timing / usage / fee model | Arc 官网；Circle pressroom | `https://www.arc.io/`；兜底 `https://www.circle.com/pressroom/circle-reports-fourth-quarter-and-full-fiscal-year-2025-financial-results` | 部分 | `observations`：Arc public network / mainnet status、testnet daily average transactions、total transactions；测试网数据不应直接当收入 |
| Reserve Fund WAM / WAL | SEC N-MFP3 / BlackRock | `https://data.sec.gov/submissions/CIK0000844779.json` | 已落库 | `observations`：`P2_RESERVE_FUND_WAM`、`P2_RESERVE_FUND_WAL` |
| Insider selling / Form 4 | SEC EDGAR | `https://data.sec.gov/submissions/CIK0001876042.json`；`https://data.sec.gov/submissions/CIK0001679788.json` | 已落库 | `filings`：Circle / Coinbase Form 4 元数据；交易方向/金额深度解释未结构化 |
| Institutional ownership / 13F | MarketBeat；SEC EDGAR 原始 filing | `https://www.marketbeat.com/stocks/NYSE/CRCL/institutional-ownership/`；SEC EDGAR 13F | 已落库 | `observations`：`P2_CRCL_INSTITUTIONAL_*`；MarketBeat 用作公开聚合源，SEC 仍适合做原始 filing 复核 |
| Compliance cost ratio | SEC 10-Q operating expense detail | Circle 10-Q / 10-K；可能需正文表格或人工标注 compliance/legal/regulatory affairs | 未自动化 | 当前无 compliance cost ratio observation；需要定义 taxonomy / 文本抽取规则 |
| CRCL short interest | FINRA / exchange-listed source | 当前自动源：`https://api.finra.org/data/group/otcMarket/name/consolidatedShortInterest` | 已落库 | `observations`：`P2_FINRA_SHORT_INTEREST`、days to cover、change pct；注意该源文档中标注过“不是 exchange-listed NYSE CRCL dataset”的口径风险 |
| Lock-up / offering | SEC S-1 / 424B / 8-K / Form 4 | SEC EDGAR | 部分 | 相关 filing 元数据可取；锁定期条款和解禁日期仍需正文解析或人工核对 |

## 监管与来源检查清单

| 事项 | 来源 | 获取地址 | 当前状态 | 说明 |
| --- | --- | --- | --- | --- |
| GENIUS Act status | GovInfo / Congress | `https://www.govinfo.gov/bulkdata/BILLSTATUS/119/s/BILLSTATUS-119s1582.xml`；人工入口 `https://www.congress.gov/bill/119th-congress/senate-bill/1582` | 已访问 | 自动记录 bill status summary；深度条款解释仍需人工 |
| GENIUS Act public law text | Congress | `https://www.congress.gov/bill/119th-congress/senate-bill/1582/text/pl` | 未自动化 | 当前采集器用 GovInfo bill status XML，不拉 public law text 正文 |
| OCC GENIUS Act / stablecoin rules | OCC | `https://www.occ.gov/news-events/newsroom/news-issuances-by-year/news-releases/index-news-releases.html` | 已访问 | source check / title；新规正文解析未结构化 |
| Treasury news / proposed rules | Treasury | `https://home.treasury.gov/news` | 已访问 | source check / title |
| FinCEN news room | FinCEN | `https://www.fincen.gov/news-room` | 已访问 | source check / title |
| OFAC recent actions | OFAC | `https://ofac.treasury.gov/recent-actions` | 已访问 | source check / title |
| Federal Reserve news | Federal Reserve | `https://www.federalreserve.gov/newsevents.htm` | 已访问 | source check / title |
| SEC newsroom | SEC | `https://www.sec.gov/newsroom` | 已访问 | source check / title |
| CFTC press releases | CFTC RSS | `https://www.cftc.gov/rss.xml` | 已访问 | source check / title |
| FDIC press releases | FDIC | `https://www.fdic.gov/news/press-releases/` | 已访问 | source check / title |
| Circle investor events | Circle IR | `https://investor.circle.com/events-and-presentations/default.aspx` | 已访问 | source check / title |

## 当前已落库指标类别

| 类别 | 指标数 | observation 数 | 说明 |
| --- | ---: | ---: | --- |
| `stablecoin_supply` | 5 | 45 | USDC supply、change、stablecoin total cap |
| `mint_redeem_flow` | 9 | 45 | Circle issued / redeemed / net issuance |
| `reserve_composition` | 7 | 31 | Circle reserve composition + SEC segregated cash |
| `reserve_fund` | 5 | 20 | BlackRock / Circle Reserve Fund |
| `rates` | 4 | 16 | Treasury yields + SOFR |
| `peg_check` | 2 | 12 | USDC / USDT price |
| `peg_liquidity` | 5 | 25 | Curve 3pool |
| `crypto_beta` | 2 | 12 | BTC / ETH |
| `equity_market` | 5 | 27 | CRCL price / volume / short interest |
| `competition` | 7 | 42 | USDC share、USDT、PYUSD、FDUSD、USDe、USDY |
| `chain_distribution` | 6 | 36 | USDC by chain |
| `chain_activity` | 6 | 29 | active addresses、transaction/transfer count、adjusted transfer volume、filing-period velocity |
| `defi_adoption` | 27 | 135 | Aave / Compound USDC deposits |
| `rwa_treasuries` | 2 | 10 | tokenized Treasury AUM / BUIDL |
| `channel_dependence` | 5 | 15 | Coinbase stablecoin and custody metrics |
| `income_statement` | 11 | 31 | Circle revenue/cost/RLDC/EBITDA/Other revenue share |
| `operating_indicator` | 2 | 6 | Circle USDC period average/end |
| `sec_filing` | 9 | 36 | latest filing metadata observations |
| `institutional_ownership` | 7 | 21 | MarketBeat CRCL institutional ownership 聚合指标 |
| `platform_metrics` | 6 | 13 | CPN TPV、Arc public/testnet metrics |
| `technical_status` | 4 | 11 | Base / Solana / Circle / Ethereum status |

## 后续补齐优先级

| 优先级 | 要补什么 | 建议做法 |
| --- | --- | --- |
| P1 | Exchange USDC balances | 已改为 CoinGlass Exchange Balance List；配置 `COINGLASS_API_KEY` 后可自动落库，CryptoQuant 页面当前有登录/Cloudflare 门槛 |
| P1 | Dune 固定 query / 截图 | 增加 `DUNE_API_KEY` 和固定 query id；把截图/口径版本记录到 `events` 或单独表 |
| P2 | Compliance cost ratio | 为 Circle 10-Q/10-K 的 compliance/legal/regulatory affairs 成本建立文本/XBRL 抽取规则；无法稳定抽取时先人工录入 |
