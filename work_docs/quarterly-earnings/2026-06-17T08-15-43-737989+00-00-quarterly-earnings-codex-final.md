结论：**观察 / Base**。Q1 2026 财报对 CRCL 是“中性偏强化”，但不升级为“增强”：RLDC 主口径足够强，平台化和竞争项还没共振。此结论不是个性化投资建议。

已按项目规则调用 subagents：`crcl-data-collector` → `crcl-data-quality-auditor` / `crcl-source-verifier` → `crcl-financial-valuation` / `crcl-platform-option` / `crcl-competition-score` → `crcl-risk-decision`。统一 evidence packet 为 `batch_id=decision-pack-1781684283173603000-25762`。

**Data Quality**
不阻断。collector 为 `ok_sources=20`、`warn_sources=1`；唯一缺口是 `P1_EXCHANGE_USDC_BALANCES`，原因是缺 `COINGLASS_API_KEY`。这影响交易所 USDC 余额和竞争/流动性置信度，不影响 SEC 财报、RLDC、Other revenue、CPN/Arc 判断。

**关键证据**
- SEC 10-Q：Reserve income `$652.508M`，Other revenue `$41.625M`，Total revenue and reserve income `$694.133M`，Distribution and transaction costs `$405.402M`，Total distribution, transaction and other costs `$406.781M`，数据期 `2026-03-31`。SEC 原文表格列示这些项目。
- RLDC：`$287.352M`。主告警口径应为 `RLDC / Reserve income = 44.04%`，高于 38% 降级线和 40% 强化线。注意：packet 中 `41.397%` 是 `RLDC / Total revenue and reserve income` 的全口径辅助值，不应用作主告警口径。
- Average USDC：`$75.2B`；End-period USDC：`$77.049B`。期末值改善，但平均值低于财报前 `76-80B` 基准，未形成共同强化。
- Circle press release：Q1’26 期末 USDC `$77.0B`、total revenue and reserve income `$694M`、Adjusted EBITDA `$151M`；CPN annualized transaction volume `$8.3B`；Other Revenue `$42M`，Total Distribution, Transaction and Other Costs `$407M`。
- Other revenue share：`5.997%`，进入 5-10% 弱平台化区间，但未达到 `>10%` 中验证/平台切换条件。
- Coinbase 10-Q：Coinbase USDC on platform `$9.289B`，数据期 `2026-03-31`。
- Arc：当前只能作为 public testnet / 技术采用线索，Arc 官网仍暴露 testnet faucet / explorer 等入口，不能当作主网收入或收费模型证据。
- Reserve fund：BlackRock/Circle Reserve Fund N-MFP3 报告期 `2026-05-31`，WAM/WAL 均为 `12 days`。

**矩阵判断**
财务桥：中性偏强化。RLDC 主口径强，但 Average USDC 与 reserve return rate 未同步强化。

平台化：弱验证。CPN 有 `$8.3B/year` TPV，但缺 take rate、收入贡献、收入确认；Arc 仍缺主网收入、客户、费用模型。

竞争：观察。competition score `55.0`，较上周 `48.3` 上升 `+6.7`，但未穿越 `75` 增强阈值，也未跌破 `40` 降级阈值。

信用层：未触发覆盖性降级。未见 USDC/USD 折价、储备异常、核心银行通道中断或正式执法行动。

**Missing Info**
`P1_EXCHANGE_USDC_BALANCES`；CPN take rate / 收入贡献 / 客户留存；Arc 主网收入 / 客户 / 费用模型；Other revenue 拆分；competition D4/D5 的流动性和链生态完整口径。

**反证 / 解除条件**
增强条件：RLDC 主口径连续 `>40%`，Average USDC 回到并站稳 `76-80B+`，reserve return rate 不继续下行，competition score `>75`，Other revenue share 进入 `10-15%` 且披露 CPN/Arc 收入贡献。

降级条件：RLDC 主口径 `<38%`；reserve return rate `<3.3%` 且 USDC 增速不足；USDC 份额 MoM 下降 `>3%` 连续 2 个月；单周净赎回超过季末余额 `1.5%` 连续 2 周；或信用层事件触发。

**下一步**
补 `COINGLASS_API_KEY` 后复核交易所 USDC balances；下次以 Q2 2026 10-Q / earnings release、Circle Transparency、DefiLlama/CoinGecko 触发项、OCC/Treasury/FinCEN 稳定币细则、CPN/Arc 收入披露作为复盘触发。

**已保存路径**
- [evidence packet](/Users/zj/sync_disk/zjSync/量化/crcl/work_docs/quarterly-earnings/2026-06-17T08-15-43-737989+00-00-quarterly-earnings-evidence.json)
- [quarterly final](/Users/zj/sync_disk/zjSync/量化/crcl/work_docs/quarterly-earnings/2026-06-17T08-15-43-737989+00-00-quarterly-earnings-codex-final.md)