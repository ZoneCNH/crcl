**结论**

CRCL 估值仓位复核结论：**观察 / 低置信度 Base**。本轮不增强、不切 Bull；未确认 Bear 失效或 C-TRIGGER，但 data-quality 阻断完整 valuation-decision 判断。未设置 `CRCL_CURRENT_POSITION_PCT`，所以只输出研究框架仓位上限，不判断具体加减仓量，也不构成个性化投资建议。

已按项目级 subagents 跑完整链路：`crcl-data-collector` -> `crcl-data-quality-auditor` / `crcl-source-verifier` -> `crcl-financial-valuation` / `crcl-competition-score` / `crcl-regulatory-watch` / `crcl-platform-option` -> `crcl-risk-decision`。

**Data Quality**

data-quality：**阻断完整判断**。统一 batch 为 `decision-pack-1781683180424633000-5889`，collector 范围 `sec,rates,market,events,status`，`ok_sources=22`，`warn_sources=1`。

阻断/降权项：
- `P0 SOURCE_UNREACHABLE_FINCEN_NEWS_ROOM`：同批 packet 里 FinCEN news room 为 `network_error`。
- `P0_CIRCLE_RESERVE_FUND_7D_YIELD stale`：同批数据日期 `2026-05-29`，超过 7 天阈值。
- `P1_EXCHANGE_USDC_BALANCES`：CoinGlass 缺 `COINGLASS_API_KEY`，交易所 USDC 余额不可用。

主线程补核显示 [FinCEN news](https://www.fincen.gov/news-room) 可访问、[BlackRock USDXX](https://www.blackrock.com/cash/en-us/products/329365/circle-reserve-fund) `2026-06-16` 7-day SEC yield 为 `3.56%`，但这些未写入 batch，只能降低疑虑，不能解除本批阻断。

**关键证据**

- 财务桥：Average USDC `752.0 亿美元`，低于 Q1 基准 `760-800 亿美元`；reserve return rate 约 `3.47%`；distribution costs / reserve income `62.13%`；RLDC / reserve income `44.04%`，高于 `38%` 警戒线。
- 估值：CRCL `79.72 USD`，稀释股数 `2.66687 亿股`，Market cap / annualized RLDC 约 `18.5x`，落在 Base `15-22x` 中部。
- 竞争：competition score `51.4`，中性偏弱 / 预警档；USDC market share `23.87%`，USDC/USDT ratio `0.402`，USDC 7D net issuance `-10.5 亿美元`，30D net issuance `-19.6 亿美元`。
- 监管：GENIUS Act 已为 Public Law `119-27`，监管矩阵为 P1 观察；未确认第三方激励 / yield workaround P0。
- 平台化：Other revenue share `6.0%`；CPN annualized TPV `83 亿美元`；Arc public testnet weekly transactions `21,938,428`，但无可重复收入拆分，不能切换支付基础设施股倍数。
- 状态页：[Circle status](https://status.circle.com/api/v2/status.json) 为 `All Systems Operational`；[Base status](https://status.base.org/api/v2/status.json) 为 `Partially Degraded Service / minor`。

**仓位上限**

- 当前上限：data-quality 阻断解除前，框架仓位上限不超过 Base 下沿，**`<=60%`**。
- Bear 上限 `20-40%` 触发：RLDC margin 跌破 `38%`；competition score 跌破 `40`；USDC 连续 4 周净赎回且累计流通量下降 `>2%`；监管最终规则明确限制第三方激励或收益分享。
- 信用层上限 `0-10%` 触发：USDC/USD 折价 `>0.1%`、储备披露确认异常、核心银行通道中断、正式执法行动，或触发项目 C-TRIGGER A/B/C。

**Missing Info**

`FinCEN` 同批不可达、USDXX yield stale 未写回 batch、CoinGlass 交易所余额缺 API key、Other revenue 缺 CPN/Arc/API 拆分、Arc 仍是 testnet 使用量指标、缺 4 周同口径市场份额和链上活跃序列、未设置当前仓位。

**解除与下次触发**

解除条件：补采并消除 FinCEN 与 USDXX 两个 P0 阻断；交易所余额恢复；USDC 7D/30D net issuance 转正或停止恶化；competition score 回到 `>=55`；下一财报确认 RLDC margin `>=40%` 且 Other revenue 有更清晰可重复收入贡献。

下次复盘触发：补齐 FinCEN / USDXX / exchange balances 后重跑；USDC 净赎回继续扩大；GENIUS Act 相关最终规则发布；Circle 下一份 10-Q / earnings release；Base status 恢复或恶化。

**已保存路径**

已保存到：

`work_docs/valuation-decision/2026-06-17T07-56-57-495789+00-00-valuation-decision-codex-final.md`