# 季度财报检查清单

## 结论

季度财报决定 CRCL 是继续按利差股定价，还是开始获得平台基础设施倍数。

先看收入质量，再看叙事。
先看平均 USDC，再看期末 USDC。

## 五项第一优先级

| 顺序 | 指标                               | 目的             |
| ---- | ---------------------------------- | ---------------- |
| 1    | Average USDC in circulation        | 判断收入基数     |
| 2    | Reserve return rate                | 判断利率压力     |
| 3    | Distribution and transaction costs | 判断分销成本     |
| 4    | RLDC margin                        | 判断规模收益留存 |
| 5    | Other revenue                      | 判断平台化验证   |

Average USDC in circulation 是平均流通量，白话就是季度内平均有多少 USDC 在外面。
期末值可能被短期波动影响，平均值更能解释储备收入。

## 计算表

```text
Total revenue and reserve income =
Reserve income =
Other revenue =
Distribution and transaction costs =
RLDC = Total revenue and reserve income - Distribution and transaction costs
RLDC margin = RLDC / Total revenue and reserve income
Other revenue share = Other revenue / Total revenue and reserve income
Adjusted EBITDA =
Adjusted EBITDA margin =
```

## 阈值

| 指标                | 强化           | 中性               | 降级                 |
| ------------------- | -------------- | ------------------ | -------------------- |
| Average USDC        | 同比和环比增长 | 同比增长、环比平稳 | 环比下降且无解释     |
| Reserve return rate | 稳定或高于预期 | 小幅下降           | 大幅下降且规模未抵消 |
| RLDC margin         | 高于 40%       | 38-40%             | 低于 38%             |
| Other revenue share | 超过 10%       | 5-10%              | 低于 5% 且无增长     |
| FY 指引             | 上修           | 维持               | 下修                 |

## 管理层问答

必须追踪以下问题：

1. CPN 的 TPV、客户数、收费模式和收入确认方式。
2. Arc 主网时间表、费用模型、企业客户试点。
3. Coinbase、Binance、其他渠道的分成变化。
4. GENIUS Act、OCC、Treasury 规则对收益分享和分销激励的影响。
5. USDC 增长来自交易、DeFi、支付还是企业结算。
6. 锁定期、二次发行、内部人减持节奏。

## Autoresearch 十项财务复核

财报结论必须把 `framework/01-business-model.md` 的十项复核填完。

| 复核项                    | 本季判断 | missing_info |
| ------------------------- | -------- | ------------ |
| Reserve income 依赖度     |          |              |
| RLDC margin 稳定性        |          |              |
| Other revenue 可重复性    |          |              |
| average USDC 解释力       |          |              |
| distribution cost 弹性    |          |              |
| Adjusted EBITDA 转化      |          |              |
| 收入确认口径              |          |              |
| 成本一次性因素            |          |              |
| FY 指引质量               |          |              |
| 财报披露完整性            |          |              |

未披露项写 `missing_info`。
不能用管理层叙事替代 SEC filing 或财报数值。

## 财报结论模板

```text
结论：Qx 财报强化 / 中性 / 降级 CRCL 框架。
依据：
1. Average USDC：
2. Reserve return rate：
3. RLDC margin：
4. Other revenue share：
5. 十项财务复核：
6. CPN / Arc 进展：
动作：
```

## Q1 2026 特别检查

Q1 2026 财报发布日期为 2026-05-11。

本次财报是 IPO 后早期公开市场验证节点。
优先判断市场是否高估平台化速度。

重点：

- Other revenue 是否接近 FY2026 1.5-1.7 亿美元年化节奏。
- RLDC margin 是否维持 38-40%。
- CPN 和 Arc 是否给出更硬的收入线索。
- 监管规则是否影响分销激励。
- 管理层是否上修或下修 USDC 增长假设。
