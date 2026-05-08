# 研究流程

## 结论

CRCL 研究按日、周、季、事件四套流程执行。

流程的目标不是多看数据，而是减少随价格波动改观点。

## 每日流程

耗时：5 分钟。

执行：

1. 更新 USDC circulating supply、7D change、30D change。
2. 检查 minted / redeemed 和 net mint / redeem。
3. 记录 3M T-Bill、SOFR、Circle Reserve Fund yield。
4. 看 BTC / ETH 与 CRCL price / volume。
5. 搜索是否有监管、储备、合作方 P0 事件。
6. 若出现 P0 事件，按 `framework/02-regulation.md` 或 `framework/03-competition.md` 的矩阵复核。

输出：

```text
今日结论：增强 / 降级 / 观察
触发指标：
动作：
```

## 每周流程

耗时：15-30 分钟。

执行：

1. 更新稳定币总市值和 USDC 市占率。
2. 对比 USDC / USDT ratio。
3. 拆 USDC by chain，找增长来源。
4. 更新 transfer volume、active addresses、velocity。
5. 检查 DeFi、交易所、收益型稳定币资金流。
6. 对照 `framework/03-competition.md` 的渠道议价矩阵和稳定币竞争矩阵。
7. 形成一条周度结论。

输出：

```text
本周框架变化：
增长质量：
竞争变化：
风险变化：
下周重点：
```

## 季度流程

耗时：财报日 60-120 分钟。

执行：

1. 读 shareholder letter、earnings release、10-Q / 10-K。
2. 填 `metrics/03-quarterly-earnings.md` 里的计算表。
3. 计算 RLDC margin、Other revenue share。
4. 听管理层问答，提取 CPN、Arc、监管、渠道分成信息。
5. 按 `framework/01-business-model.md` 的十项财务复核逐项标记。
6. 更新估值情景。
7. 写财报结论。

shareholder letter 是股东信，白话就是公司给投资者解释经营情况的材料。

## 事件流程

触发条件：

1. 监管文件发布。
2. 储备报告异常。
3. USDC/USD 折价。
4. Coinbase 或核心渠道重大公告。
5. Arc / CPN 重大产品发布。
6. 解禁、增发、重大内部人交易。

处理顺序：

```text
确认来源
  -> 判断层级
  -> 判断是否 P0
  -> 更新风险地图
  -> 更新估值假设
  -> 输出动作
```

## Autoresearch 矩阵调用

| 事件类型   | 先查矩阵                          | 再写入文件                           |
| ---------- | --------------------------------- | ------------------------------------ |
| 财报       | `framework/01-business-model.md`  | `metrics/03-quarterly-earnings.md`   |
| 监管       | `framework/02-regulation.md`      | `risk/01-warning-signals.md`         |
| 渠道和竞争 | `framework/03-competition.md`     | `metrics/04-competition-dashboard.md` |
| CPN / Arc  | `framework/04-platform-option.md` | `metrics/03-quarterly-earnings.md`   |
| 新假设     | `autoresearch/00-loop.md`         | `autoresearch/01-iteration-log.md`   |

矩阵调用只解决复核顺序。
最终结论仍必须落到增强、降级、观察三类动作。

## 版本纪律

每次更新文档必须写清：

1. 更新日期。
2. 数据来源。
3. 哪个假设变了。
4. 哪个动作变了。

没有动作变化的更新，只能标记为观察。
