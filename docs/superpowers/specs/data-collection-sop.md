# 数据获取 SOP

文档版本：v1.0 | 类型：数据获取规格

## 定位

本文件解决"知道要追踪什么指标，但不知道怎么获取"的问题。拿到这份文件的任何人，都能独立执行 CRCL 框架中所有数据获取步骤。

数据源优先级：SEC filing → Circle 官方披露 → 美国监管机构文件 → 链上浏览器原始数据 → DefiLlama / CoinGecko / Dune 等二级平台 → 媒体报道。二级平台只能作为监控线索，季度结论必须回到 SEC filing 和公司披露。

本文件不是当前数据快照。大模型每次执行时必须写明本次运行日期和每个数据源的数据日期，不能沿用本文中的历史日期或旧样例。

---

## 1. 数据获取总览

### P0 数据（日度获取，约 12 项）

| 编号 | 指标 | 数据来源 | 获取方式 |
|------|------|---------|---------|
| P0-01 | USDC circulating supply / 7D / 30D change | Circle Transparency | circle.com/transparency |
| P0-02 | Minted / Redeemed / Net mint | Circle Transparency | circle.com/transparency |
| P0-03 | 3M T-Bill yield | U.S. Treasury | home.treasury.gov |
| P0-04 | SOFR | NY Fed | newyorkfed.org |
| P0-05 | Circle Reserve Fund 7-day yield | SEC N-MFP3 / BlackRock | SEC EDGAR 月度 filing；USDXX 基金页面作日度人工交叉验证 |
| P0-06 | USDC/USD 二级市场价格（锚定检查） | CoinGecko / Curve.fi | coingecko.com；curve.fi |
| P0-07 | BTC / ETH price | CoinGecko / TradingView | 行情平台 |
| P0-08 | CRCL price / volume | Yahoo Finance / 雪球 | 股票代码 CRCL |
| P0-09 | 监管公告（事件触发） | Congress / OCC / Treasury / SEC / CFTC | 见 §2-H |
| P0-10 | Chain status（事件触发） | status.base.org / status.solana.com / Ethereum public RPC | 各链状态页；Ethereum 用 latest block freshness |
| P0-11 | Circle 8-K / 重大公告（事件触发） | SEC EDGAR / Circle IR | edgar.sec.gov |
| P0-12 | Curve 3pool / USDC 池比例（事件触发） | Curve.fi | curve.fi |

### P1 数据（周度获取，约 12 项）

| 编号 | 指标 | 数据来源 | 获取方式 |
|------|------|---------|---------|
| P1-01 | USDC market cap share | DefiLlama / CoinGecko | defillama.com/stablecoins |
| P1-02 | USDC / USDT ratio | DefiLlama / CoinGecko | 自算：USDC 市值 ÷ USDT 市值 |
| P1-03 | Stablecoin total market cap | DefiLlama | defillama.com/stablecoins |
| P1-04 | USDC by chain（Base、Ethereum、Solana、L2） | Dune Analytics / TokenTerminal | dune.com |
| P1-05 | USDC adjusted transfer volume | Visa Onchain Analytics / Allium / CoinMetrics / Dune | visaonchainanalytics.com/transactions；CoinMetrics / Dune 作交叉验证 |
| P1-06 | Active addresses | Dune / Santiment | dune.com |
| P1-07 | USDC velocity（自算） | Circle SEC filing / Dune | filing-period：onchain volume ÷ average circulation；周度 adjusted 口径仍需 Dune |
| P1-08 | Exchange USDC balances / changes / concentration | CoinGlass（Open API 或前台公开 Balance 接口） | `capi.coinglass.com/api/exchange/chain/v3/balance/list?symbol=USDC`；历史接口用于 90D/365D 趋势 |
| P1-09 | DeFi protocol USDC deposits（Aave / Compound） | DefiLlama protocol API | `api.llama.fi/protocol/aave-v3`；`api.llama.fi/protocol/compound-v3` |
| P1-10 | Coinbase 平台 USDC 余额 / Base 链增长 | Coinbase filing / DefiLlama / TokenTerminal / Dune | 见 §2-E / §2-F |
| P1-11 | 竞品市值（PYUSD、FDUSD、USDe、Ondo USDY） | CoinGecko / Ethena / Ondo | coingecko.com |
| P1-12 | Tokenized Treasury AUM / BUIDL AUM | RWA.xyz public treasuries page | app.rwa.xyz/treasuries |

### P2 数据（季度 / 事件获取，约 12 项）

| 编号 | 指标 | 数据来源 | 获取频率 |
|------|------|---------|---------|
| P2-01 | Reserve income | SEC 10-Q / 10-K | 季度财报发布后 T+24h |
| P2-02 | Distribution and transaction costs | SEC 10-Q / 10-K | 同上 |
| P2-03 | RLDC / RLDC margin | 自算（基于 SEC filing） | 同上 |
| P2-04 | Other revenue / Other revenue share | SEC 10-Q / 10-K | 同上 |
| P2-05 | Adjusted EBITDA | SEC 10-Q / 10-K | 同上 |
| P2-06 | Coinbase distribution exposure | Circle / Coinbase SEC filings | 季度 |
| P2-07 | CPN TPV | Circle 披露 / 财报 | 季度 |
| P2-08 | Arc mainnet 进度 / 使用量 | Circle 博客 / 开发者文档 | 事件触发 |
| P2-09 | Reserve Fund WAM / WAL | BlackRock / SEC | 月度 |
| P2-10 | Insider selling / Lock-up（Form 4） | SEC EDGAR | 事件触发 |
| P2-11 | Institutional ownership（13F） | SEC EDGAR | 季度 |
| P2-12 | 合规成本比率（自算） | SEC 10-Q 运营费用明细 | 季度 |

---

## 2. 各数据源获取 SOP

---

### A. Circle Transparency（USDC 供应、储备）

**URL**：`https://www.circle.com/transparency`

**获取项**：

- USDC 流通量（circulating supply）
- 7D / 30D / 365D 变化
- Minted / Redeemed（净增发/赎回）
- 储备构成（月度 Reserve Report PDF）

**获取频率**：P0 每日

**操作步骤**：

1. 访问 `circle.com/transparency`
2. 记录页面顶部"USDC in Circulation"数字（精确到亿位）
3. 记录"7 Day Change"和"30 Day Change"百分比
4. 记录当周 Minted（发行量）和 Redeemed（赎回量）；计算 Net = Minted − Redeemed
5. 如需储备构成，查找页面内"Reserve Report"PDF（月度发布）；提取现金、国债、回购比例
6. 将数据填入 `../../metrics/01-daily-watchlist.md` 记录模板，附日期戳

**备用源**：DefiLlama stablecoins 页面（`defillama.com/stablecoins`）→ 搜索 USDC → Market Cap 列

**数据质量标准**：

- 单日变化超 ±2%：必须用 DefiLlama 交叉验证
- 连续 2 周 Net mint < 0（净赎回）：升级为 P0 风险，执行 `../../metrics/01-daily-watchlist.md` 触发阈值流程

---

### B. DefiLlama（稳定币市占率）

**URL**：`https://defillama.com/stablecoins`

**获取项**：

- USDC 市值、USDT 市值、稳定币总市值
- USDC 市占率（周度 4 周趋势）
- 各链 USDC 分布（周度）

**获取频率**：P0 每日（锚定检查）；P1 每周（市占率、链分布）

**操作步骤**：

1. 访问 `defillama.com/stablecoins`
2. 记录 USDC 和 USDT 的 Market Cap 列数字
3. 计算市占率：`USDC Market Cap ÷ Total Stablecoins Market Cap × 100%`
4. 计算 USDC/USDT ratio：`USDC Market Cap ÷ USDT Market Cap`
5. 点击 USDC 名称 → 进入详情页 → 查看"Chain Distribution"板块 → 记录 Base / Ethereum / Solana / Arbitrum / Polygon 各链余额
6. 连续 4 周将市占率填入记录表，计算趋势方向

**与 CoinGecko 的差异处理**：两者数据通常一致，若差 > 1%，以 DefiLlama 为准（更实时）；记录差异原因

**告警触发**：

- 月度市占率变化 > ±1.5 ppts：标注竞争预警
- 连续 4 周市占率下降：升级为 P1 告警，更新 `../../metrics/04-competition-dashboard.md`

---

### C. U.S. Treasury（储备收益率）

**URL**：`https://home.treasury.gov/resource-center/data-chart-center/interest-rates`

**获取项**：3M T-Bill yield、1Y Treasury、5Y Treasury

**获取频率**：P0 每日

**操作步骤**：

1. 访问 Treasury 官网 → Resource Center → Interest Rates → **Daily Treasury Par Yield Curve Rates**
2. 找到当日数据行，记录 3-Month、1-Year、5-Year 收益率（精确到 2 位小数，单位 %）
3. 与上周同日对比，计算变化（bps）：`本日 − 上周 = Δbps`
4. 将 3M T-Bill 值填入日监模板
5. 若当日单日变化 > 10 bps：在日监清单中加注"利率快速变动，关注储备收入影响"

**备用源**：

- FRED（`fred.stlouisfed.org`） → 搜索"3-Month Treasury Bill"
- Yahoo Finance → 搜索"^IRX"（3M T-Bill 实时值）

**NY Fed SOFR 同步获取**：访问 `https://www.newyorkfed.org/markets/reference-rates/sofr` → 记录当日 SOFR 值

**告警触发**：

- 3M T-Bill 下行 > 50 bps（单季）：标注储备收入下修风险
- 3M T-Bill 上行 > 25 bps：标注储备收入支撑，但需确认是否已传导至储备基金收益率

---

### D. SEC N-MFP3 / BlackRock（Circle Reserve Fund 收益率）

**URL**：

- 机器源：`https://data.sec.gov/submissions/CIK0000844779.json`
- 人工交叉验证：搜索"BlackRock USD Institutional Digital Liquidity Fund"或"Circle Reserve Fund"；基金代码 USDXX

**获取项**：Circle Reserve Fund 7-day yield、gross yield、WAM、WAL、net assets

**获取频率**：P0 每日检查；SEC N-MFP3 月度 filing 更新

**操作步骤**：

1. 运行 Rust collector：`cargo run --release -- collect --source rates`
2. 从 SEC EDGAR BlackRock Funds 最新 `N-MFP3` 主文档读取 Circle Reserve Fund：`Item B.8` 为 7-day net yield，`Item A.19` 为 gross yield，`Item A.11/A.12` 为 WAM/WAL
3. 如需日度网页交叉验证，再搜索 BlackRock 官网或 YCharts，找到 USDXX 基金详情页
4. 对比 3M T-Bill，判断是否已传导：正常情况 USDXX 收益率应接近 3M T-Bill
5. 禁止沿用旧值：每次记录必须标注获取日期和数据日期；SEC N-MFP3 是月度 filing 口径，不等同于 BlackRock 页面当日值

**告警触发**：

- 7-day yield < 3.0%：标注储备收益压力，触发收入下修检查
- 7-day yield > 5.0%：确认利率高位，验证是否降息未传导

---

### E. SEC EDGAR（财报数据）

**URL**：`https://efts.sec.gov/LATEST/search-index?q=%22Circle+Internet+Group%22&forms=10-Q,10-K,8-K`

或使用 EDGAR 全文搜索：`https://efts.sec.gov/LATEST/search-index`

**Circle CIK 号**：1876042（从 S-1 确认，EDGAR 搜索优先使用 CIK，不用硬链接）

**获取项**：

- 10-Q（季报）：Reserve income、Distribution costs、RLDC、Other revenue、Adjusted EBITDA
- 10-K（年报）：同上 + 全年合规成本、渠道分成披露
- 8-K（重大事件）：财报发布、合作公告、监管回应
- Form 4（内部人交易）：高管/大股东买卖申报
- Coinbase filing：AOP by asset 表格中的 USDC 行，用于 Coinbase 平台 USDC 余额和渠道集中度检查

**获取频率**：P2 季度（10-Q / 10-K）；P0 事件触发（8-K、Form 4）

**操作步骤**：

1. 访问 `edgar.sec.gov` → 搜索框输入"Circle Internet Group"或 CIK 1876042
2. 筛选表单类型：10-Q / 10-K / 8-K / Form 4
3. 点击最新 filing → 下载主文件（htm 或 pdf），保存本地，命名格式：`CRCL_10Q_YYYYQX.pdf`
4. 提取关键数字（参照 `metrics/03-quarterly-earnings.md` 中的提取位置说明）：
   - `Total revenue and reserve income`（总收入行）
   - `Total distribution, transaction and other costs`（总分销成本行）
   - `RLDC = 总收入 − 总分销成本`
   - `RLDC margin = RLDC ÷ Total revenue and reserve income`
   - `Other revenue`（单独科目）
   - `Adjusted EBITDA`（非 GAAP 指标，见管理层 MD&A 段落）
5. 对 Coinbase 10-Q / 10-K，读取 Assets on Platform by asset 表格的 USDC 行；该值是期末 customer USDC on platform，不等同于平均 USDC balances
6. 8-K 事件触发：财报日前 3 天每日检查，发现后立即阅读并更新框架相关文件
7. Form 4 事件触发：检查内部人交易方向，大额减持记录至 P2 筹码压力项

**合规成本比率计算**：

```text
合规成本比率 = 运营费用中 compliance / legal / regulatory affairs 相关科目之和 ÷ Reserve income
```

> 2%：触发分销经济性重算；环比上升 > 0.5 ppts：标记监管成本压力

---

### F. Visa / Allium / Dune / CoinMetrics（链上数据）

**URL**：`https://visaonchainanalytics.com/transactions`；`https://dune.com`

**获取项**：USDC adjusted transfer volume、Active addresses、各链 USDC 供应

**获取频率**：P1 每周

**推荐仪表盘**：

- 自动化主源：Visa Onchain Analytics stablecoin transfers（Allium shared query）
- 稳定币转账量：搜索"rchen8 stablecoin"→ 找到 rchen8 的稳定币仪表盘
- USDC by Chain：搜索"USDC Supply by Chain"
- Base USDC Supply：搜索"Base USDC Supply"
- USDC Adjusted Transfer Volume：搜索"USDC adjusted transfer volume"
- Active addresses：搜索"USDC active addresses"

**操作步骤**：

1. 运行 Rust collector：`cargo run --release -- collect --source market`
2. `P1_USDC_ADJUSTED_TRANSFER_VOLUME` 使用 Visa Onchain Analytics / Allium 的 USDC `Adjusted` 口径，记录最近完整 UTC 日的 `usd_amount`，并保存 30D 合计属性
3. 需要周报截图或人工交叉验证时，访问 `dune.com` → 搜索对应仪表盘关键词
4. 记录当周数字（精确到亿级）并标注日期
5. 保存仪表盘截图或书签（每周固定截图，防止仪表盘口径变化）
6. 对比上周数字，计算 7D 变化

**口径说明**：

- Adjusted transfer volume = 剔除合约内部转账后的净转账量（更接近真实使用）
- 自动化口径限定 `base_asset = USDC`、`tag = Adjusted`，不要直接使用 Visa 页面顶部全稳定币合计卡片替代 USDC 专项数字
- Dune 各仪表盘口径可能不统一，每次记录时注明使用的仪表盘名称和作者

**与 CoinMetrics / Dune 差异处理**：容忍度 ±20%，超过须人工核查口径差异；CoinMetrics / Dune 适合作为交叉验证，不覆盖自动化主源的日期戳

**备用源**：

- CoinMetrics（`coinmetrics.io`）：需注册，API 更稳定
- TokenTerminal（`tokenterminal.com`）：链和协议层数据，口径变化要记录
- Etherscan / SolanaFM / Arbiscan：各链浏览器原始数据

**告警触发**：

- Adjusted transfer volume 连续 4 周下降：P1 预警
- Active addresses 连续 4 周下降 > 10%：P1 预警

---

### G. CoinGecko（市场数据与竞品监控）

**URL**：`https://www.coingecko.com`

**获取项**：USDC / USDT 价格（锚定检查）、市值排名、稳定币板块市值、竞品（PYUSD、FDUSD、USDe、Ondo USDY）市值

**获取频率**：P0 每日（锚定检查）；P1 每周（竞品监控）

**操作步骤**：

1. 访问 `coingecko.com` → 搜索"USDC"→ 记录当前价格（应 ≈ 1.000）
2. 记录 USDC 市值，与 DefiLlama 对比（差 > 1% 时以 DefiLlama 为准并记录差异）
3. 搜索 USDT：记录价格和市值
4. 每周访问 Stablecoins 板块，记录竞品市值（PYUSD、FDUSD、USDe）
5. 访问 `ethena.fi`：记录 sUSDe 当前 APY（收益型稳定币分流压力指标）
6. 访问 `app.rwa.xyz/treasuries`：记录 Tokenized Treasuries 总 AUM 和 BUIDL AUM

**锚定异常触发**：

- USDC 价格 < 0.995 或 > 1.005：立即升级为 P0，检查 Curve 3pool 池比例（`curve.fi`）和 Circle 储备状态
- 持续折价（> 1 天）：停止平台化估值讨论，优先评估信用风险

---

### H. 监管来源（事件驱动日常监控）

**获取频率**：P0 事件触发（每日检查新闻标题，发现事件后深度阅读）

#### H-1. Congress.gov（GENIUS Act 进展）

**URL**：`https://www.congress.gov/bill/119th-congress/senate-bill/1582`

**操作步骤**：

1. 访问法案页面，查看"Actions"栏最新动态
2. 关注：法案状态（Committee / Floor / Signed）、条款修订、最终法律文本
3. 建议设置 Congress.gov 邮件提醒（关键词：GENIUS Act、stablecoin）

**重点监控条款**：收益分享限制、非银行发行资格、储备资产要求、第三方激励限制

#### H-2. OCC（拟议规则 / 最终规则）

**URL**：`https://www.occ.gov/news-issuances/news-releases/`

**操作步骤**：

1. 每日访问 OCC 新闻页面，查找涉及"stablecoin"、"payment stablecoin"的新规或声明
2. 发现 NPRM（拟议规则）时：下载 PDF，提取关键条款，更新 `framework/02-regulation.md`
3. 历史记录：OCC 于 2026-02-25 发布 GENIUS Act 拟议规则（`nr-occ-2026-9.html`）；运行时必须重新确认是否已有 Final Rule

#### H-3. Treasury / FinCEN / OFAC

**Treasury URL**：`https://home.treasury.gov/news`

**FinCEN URL**：`https://www.fincen.gov/news-room`

**OFAC URL**：`https://ofac.treasury.gov/recent-actions`

**操作步骤**：

1. 每日检查 Treasury 新闻页面标题（BSA/AML、稳定币、制裁相关）
2. 历史记录：Treasury / FinCEN / OFAC 于 2026-04-08 发布 BSA/AML 与制裁合规拟议规则；运行时必须重新确认状态
3. 发现新规后：立即执行 `playbook/00-research-routine.md` 监管事件 SOP

#### H-4. Federal Reserve / SEC / CFTC

| 机构 | URL | 监控重点 |
|------|-----|---------|
| Federal Reserve | `federalreserve.gov/newsevents.htm` | 鲍威尔等理事对稳定币讲话；鹰/鸽派措辞 |
| SEC | `sec.gov/newsroom` | 执法行动、Wells 通知、稳定币界定 |
| CFTC | `cftc.gov/rss.xml` | 商品 vs 证券界定；执法和解 |
| FDIC | `fdic.gov/news/press-releases/` | GENIUS Act FDIC 版拟议规则、储备存款保险口径 |

**发现监管事件的标准动作**：

1. 立即记录事件标题、机构、日期
2. 判断影响维度：收益分享限制 / 非银行发行资格 / 储备要求 / 第三方激励
3. 更新 `framework/02-regulation.md` 对应条款
4. 若涉及分销经济性变化：重算 RLDC margin，并在日监清单中标注

---

### I. Circle Investor Relations（产品 / 财报 / 公告）

**URL**：`https://investor.circle.com/events-and-presentations/default.aspx`

**获取项**：财报发布日期、管理层演讲、CPN 合作伙伴公告、Arc 技术进展

**获取频率**：P0 事件触发（每日检查 IR 页面更新）

**操作步骤**：

1. 每日访问 IR 页面，检查"Events and Presentations"是否有新增
2. 财报日前 3 天：每日额外检查 SEC EDGAR 是否有 8-K 预发布
3. 财报发布当天：只更新事实表，不急着改长期结论；财报后 24h 内重算核心指标
4. 电话会（earnings call）后：更新 CPN TPV、Arc 进度、渠道分成判断
5. 记录规则：只记录真实客户、真实交易、真实收入；管理层只强调叙事、不披露口径 → 按负面信号处理

**Arc 进度追踪**：

- URL：`https://circle.com/arc`（或 Circle 开发者文档）
- 区分测试网（testnet）和主网（mainnet），测试网数据不能当收入
- 关注：是否产生非储备收入（Other revenue）；gas / fee 经济模型

---

### J. 链上技术可用性（事件触发）

**获取项**：Base、Ethereum、Solana 主链在线状态与故障时长

**获取频率**：P0 事件触发（实时监控，建议订阅状态页邮件通知）

| 链 | 状态页 URL | 告警阈值 |
|----|-----------|---------|
| Base | `https://status.base.org` | 故障 > 2h：额外触发 CPN 中断检查 |
| Solana | `https://status.solana.com` | 故障 > 4h：触发流动性风险评估 |
| Ethereum | Ethereum 基金会 / Etherscan 公告 | 故障 > 4h：触发流动性风险评估 |

**操作步骤**：

1. 订阅各链状态页的邮件 / RSS 告警
2. 发现故障时：记录故障时长和受影响功能（USDC 转账是否中断）
3. 故障超 24h：更新 `framework/04-platform-option.md` Arc 可用性记录

---

### K. 储备基金安全（月度）

**获取项**：Circle Reserve Fund WAM（加权平均期限）/ WAL（加权平均寿命）、每日/每周流动性比例

**获取频率**：月度（SEC N-MFP3 / BlackRock 基金定期报告）；每周（Circle Transparency 储备构成）

**操作步骤**：

1. 运行 Rust collector：`cargo run --release -- collect --source rates`
2. 从 SEC EDGAR BlackRock Funds 最新 `N-MFP3` 主文档读取 Circle Reserve Fund：`Item A.11/A.12` 为 WAM/WAL，`Item A.13` 为每日/每周流动性
3. 如需人工交叉验证，再访问 BlackRock USDXX 基金详情页 → 找到月度报告 → 记录 WAM、WAL
4. 访问 `circle.com/transparency` → 查看储备构成：现金比例、国债比例、回购比例
5. 验证流动性要求：每日流动性 ≥ 10%，每周流动性 ≥ 30%（GENIUS Act OCC 规则要求）
6. WAM 拉长（> 60 天）时标注：压力期赎回风险上升

---

### L. FINRA（short interest / days to cover）

**URL**：`https://api.finra.org/data/group/otcMarket/name/consolidatedShortInterest`

**获取项**：CRCL short interest、days to cover、short-interest change

**获取频率**：双周（FINRA short interest 发布后）

**操作步骤**：

1. 运行 Rust collector：`cargo run --release -- collect --source market`
2. Query API 使用 `symbolCode=CRCL` 过滤，取最新 `settlementDate`
3. 记录 `currentShortPositionQuantity`、`daysToCoverQuantity`、`changePercent`
4. 确认返回行 `marketClassCode=NYSE`、`issuerServicesGroupExchangeCode=A`
5. short interest 上升但基本面无变化时，只作为仓位和波动风险，不直接改基本面情景

---

## 3. 数据获取时间表（每周操作日历）

| 星期 | 必做任务 | 数据来源 | 预计时间 |
|------|---------|---------|---------|
| **周一** | P0 日监：USDC 供应、T-Bill、SOFR、储备基金收益率 | Circle / Treasury / NY Fed / BlackRock | 10 分钟 |
| **周一** | USDC/USD 锚定检查 | CoinGecko / Curve.fi | 2 分钟 |
| **周一** | 监管新闻扫描（OCC / Treasury / Congress） | 各监管机构官网 | 5 分钟 |
| **周三** | P0 日监更新（同周一流程） | 同上 | 10 分钟 |
| **周三** | Circle IR 页面检查 | investor.circle.com | 2 分钟 |
| **周五** | P0 日监（最终） | 同上 | 10 分钟 |
| **周五** | P1 周监：市占率、Dune 链上数据、竞品市值 | DefiLlama / Dune / CoinGecko | 20 分钟 |
| **周五** | 执行 `../../metrics/06-validation-matrix.md` 数据预检 | 先查上周遗留 `missing_info` 和口径冲突 | 10 分钟 |
| **周五** | 更新 `../../metrics/04-competition-dashboard.md` 竞争评分 | 参照 `../../metrics/05-competition-scoring-rubric.md` D1-D7 | 20 分钟 |
| **周五** | 执行 `../../metrics/06-validation-matrix.md` 结论验收 | 复核 dashboard、周报、财报和估值口径是否一致 | 10 分钟 |
| **周五** | 输出本周周报 | `../../metrics/02-weekly-review.md` 模板 | 15 分钟 |
| **财报日** | 下载最新 10-Q / 8-K；财报后 24h 内重算核心指标 | SEC EDGAR / Circle IR | 60 分钟 |
| **财报后电话会** | 更新 CPN TPV、Arc 进度、渠道分成判断 | 电话会录音 / 文字稿 | 30 分钟 |

---

## 4. 数据质量检查规则

### 必须执行的交叉验证

| 场景 | 触发条件 | 验证动作 |
|------|---------|---------|
| USDC 供应单日变化 > ±2% | Circle Transparency 数据异常 | 用 DefiLlama USDC 市值对比 |
| USDC/USD 价格 < 0.995 或 > 1.005 | CoinGecko 检测到脱锚 | 同时查 Curve 3pool 池比例 |
| 链上数据突变 | Visa / Allium、Dune 或 CoinMetrics 数字异常跳变 | 对比另一个链上数据源的同口径数据 |
| DefiLlama vs CoinGecko 市值差 > 1% | 两源不一致 | 以 DefiLlama 为准，记录差异原因 |
| 3M T-Bill 单日变化 > 10 bps | Treasury 数据显示快速变动 | FRED 备用源确认，日监标注 |

### 口径变化识别

- **Visa / Allium / Dune 仪表盘**：自动化查询固定 shared query；Dune 每周保存截图，对比曲线形状是否突变；发现口径变化时记录日期并切换备用源
- **DefiLlama / CoinGecko**：如数据停更超 30 天，标注"数据截至 YYYY-MM-DD"并切换备用源
- **来源 URL 失效**：EDGAR 搜索优先用 CIK（1876042），不用硬链接；其他来源按 `sources.md` 检查规则处理

### 数据记录纪律

- 每条记录必须有日期戳（格式：YYYY-MM-DD）
- 禁止用"昨天的数据"代替今日记录
- 每个关键事实必须带来源标注
- 媒体报道只能作为线索，不能作为最终结论

---

## 5. 常见数据获取问题 FAQ

**Q：DefiLlama 和 CoinGecko 的 USDC 市值不一致怎么办？**
→ 优先用 DefiLlama（更实时），记录两者差异和日期，无需更改结论。

**Q：Dune 仪表盘访问很慢或报错？**
→ adjusted transfer volume 先看 Rust collector 的 Visa / Allium 自动化结果；截图或交叉验证备用方案：① CoinMetrics API（需注册）；② TokenTerminal；③ 各链浏览器（Etherscan / SolanaFM / Arbiscan）直接查。

**Q：Circle 财报数据在哪里找 RLDC？**
→ 参照 `metrics/03-quarterly-earnings.md` 中的提取位置说明；RLDC 不是单独科目，需自算：Total revenue and reserve income − Total distribution, transaction and other costs。

**Q：SEC EDGAR 找不到 Circle 的 10-Q？**
→ 用 CIK 号搜索：1876042（从 S-1 确认）。搜索框选"10-Q"表单类型后按日期排序。

**Q：Circle Reserve Fund 收益率页面找不到？**
→ 在 BlackRock 官网搜索"USD Institutional Digital Liquidity Fund"或 USDXX；备用：YCharts 搜索基金代码。

**Q：监管公告太多，怎么判断哪些影响 CRCL？**
→ 优先看：① 是否限制收益分享或第三方激励；② 是否改变非银行发行资格；③ 是否改变储备资产构成要求。这三项直接影响 Circle 收入模式，立即升级为 P0 处理。

**Q：链上数据 Visa / Allium vs Dune / CoinMetrics 差异很大？**
→ 容忍度 ±20%；超过时用各自口径单独列出，不合并计算；自动化主表保留 Visa / Allium 最近完整 UTC 日，周报可附 Dune / CoinMetrics 差异说明。

**Q：财报数字和管理层 IR 演示文稿不一致？**
→ 以 SEC 10-Q / 10-K 中的 GAAP 财务报表为准；管理层演示中的非 GAAP 指标（如 Adjusted EBITDA）以公司定义为准，但需注明口径。

---

## 版本记录

| 版本 | 日期 | 变更内容 |
|------|------|---------|
| v1.1 | 2026-06-15 | 增加 Visa Onchain Analytics / Allium 作为 USDC adjusted transfer volume 自动化主源，Glassnode Studio public metadata 作为 exchange USDC balances 自动化源。 |
| v1.0 | 2026-05-09 | 初始建立。覆盖 P0/P1/P2 全量数据获取 SOP，含 12 个数据源（A-L）、每周操作日历、数据质量规则、FAQ。来源核验记录仅代表初始建档快照。 |

> 每次数据来源变化（URL 失效、口径调整、新增来源）时，在版本记录中追加一行，注明变更原因和新链接。
