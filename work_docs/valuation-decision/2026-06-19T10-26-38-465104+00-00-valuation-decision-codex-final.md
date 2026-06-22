已按项目级 subagents 完成 `valuation-decision`：collector 先生成同一批 evidence packet，随后 data-quality、source、financial、competition、regulatory、platform、risk 全部基于同一 SQLite / `--no-collect` 上下文收口。

**结论**
结论：观察。
Bull/Base/Bear：Base 偏弱，落在 Base 下沿；不切 Bull，不进入 Bear。
仓位上限：`50-60%`，未设置 `CRCL_CURRENT_POSITION_PCT`，所以本轮只给仓位上限，不判断具体加减仓量；这不是个性化投资建议。

**Data Quality**
data-quality 不完全阻断估值复核，但阻断“增强 / keep / 上调仓位”。
本轮 batch：`decision-pack-1781864897917234000-25626`，范围 `sec,rates,market,events,status`，`ok_sources=22`、`warn_sources=1`。
阻断点：Treasury news P0 source gap；CoinGlass exchange USDC balances / 30D change / top3 concentration 三项 P1 decode gap。

**关键证据**
1. 信用层未触发：USDC/USD `0.999777`（CoinGecko，2026-06-19），Circle USDC 流通 `74.86B`、总储备 `75.10B`（Circle Transparency，2026-06-15）。
2. USDC 动能偏弱：DefiLlama supply `74.8938B`（2026-06-19），7D / 30D 为 `-0.0229% / -1.9426%`；Circle official 7D / 30D 为 `-1.3832% / -2.5514%`（2026-06-15）。
3. 财务桥托底：Q1 2026 Reserve income `652.508M`、RLDC `287.352M`、Other revenue `41.625M`、Adjusted EBITDA `151.401M`，period ended 2026-03-31，来源 SEC EDGAR Circle 10-Q。
4. RLDC margin：主口径 `RLDC / Reserve income = 44.0381%`，未触发 `38%` 降级告警；全口径 `41.3973%` 只作辅助。
5. 估值锚：CRCL price `80.23`（Yahoo Finance，2026-06-18），隐含市值约 `21.396B`，Market cap / annualized RLDC 约 `18.62x`，位于 Base 区间 `15-22x`。
6. 竞争：competition score `53.1`，中性偏弱 / 预警档；不支持 Bull `>75`，也未跌破 Bear 竞争阈值 `<40`。
7. 平台化：Other revenue share `5.9967%`，只到弱验证；未达到 `10%` 中验证或 `15%` 强验证，CPN/Arc 尚无可重复收入拆分。
8. 来源缺口：Treasury news `https://home.treasury.gov/news` 为 `network_error`；CoinGlass `https://capi.coinglass.com/api/exchange/chain/v3/balance/list?symbol=USDC` 为 decode error。

**Missing Info**
- `SOURCE_UNREACHABLE_TREASURY_NEWS`：不能声称 Treasury 当日新闻已完整核验。
- `P1_EXCHANGE_USDC_BALANCES`、`P1_EXCHANGE_USDC_BALANCE_30D_CHANGE`、`P1_EXCHANGE_USDC_TOP3_CONCENTRATION`：不能把旧 CoinGlass 数据写成当前事实。
- Other revenue 缺 CPN、Arc、企业 API、integration services 子项拆分。
- CPN fee model、take rate、Arc 主网费用模型、主网客户和收入归属仍缺。

**反证 / 解除条件**
升级解除：Treasury source 恢复且无 P0；CoinGlass 或替代源恢复；USDC 30D 转正或净赎回停止；competition score 回到 `>=55`；RLDC 主口径维持 `>40%`；Other revenue 占比升至 `>10%` 且 CPN/Arc 披露收入。
降级触发：RLDC 主口径 `<38%`；competition score `<40`；USDC 连续 2-4 周净赎回；监管明确限制第三方激励/收益分享；USDC/USD 折价 `>0.1%` 或储备/赎回通道异常。

**下一步**
下次复盘重点：Circle Transparency 下一次 7D/30D 更新、Treasury/OCC/FinCEN/OFAC/SEC 监管源、CoinGlass collector 修复、Q2 2026 财报 Other revenue 拆分、competition score 穿越 `55` / `75` / `40`。

**已保存路径**
已保存本轮汇总：`work_docs/valuation-decision/2026-06-19T10-26-38-465104+00-00-valuation-decision-subagent-summary.md`
codex exec 最终回复目标路径：`work_docs/valuation-decision/2026-06-19T10-26-38-465104+00-00-valuation-decision-codex-final.md`
