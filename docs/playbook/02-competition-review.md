# 竞争专项复盘

## 结论

竞争专项复盘每周执行一次，财报周和监管事件周必须执行。

目标不是证明 CRCL 比对手好，而是判断竞争结构有没有改变收入、成本、估值或风险动作。

## 输入文件

| 文件                                  | 用途                    |
| ------------------------------------- | ----------------------- |
| `framework/03-competition.md`         | 竞争逻辑和判断框架      |
| `metrics/04-competition-dashboard.md` | 每周填数和 score        |
| `metrics/02-weekly-review.md`         | 周度增长质量复盘        |
| `risk/01-warning-signals.md`          | 竞争相关预警            |
| `valuation/01-scenario-model.md`      | Bull/Base/Bear 情景切换 |
| `sources.md`                          | 来源核验                |

## 执行步骤

1. 更新 stablecoin total market cap、USDC market share、USDT dominance。
2. 计算 USDC / USDT ratio。
3. 更新 Base USDC supply 和 Coinbase products average USDC。
4. 检查 RLDC margin 的最近财报值。
5. 更新 USDC by chain、adjusted transfer volume、active addresses。
6. 更新 USDe、BUIDL、USDY、USYC AUM。
7. 搜索银行稳定币和 tokenized deposits 事件。
8. 填 `metrics/04-competition-dashboard.md`。
9. 计算 competition score。
10. 对照渠道议价矩阵和稳定币竞争矩阵。
11. 输出增强、降级或观察。

## 判断顺序

先看份额，再看利润留存。
先看渠道经济性，再看链上热度。
先看官方源，再看 Dune 和媒体。
先看是否改变动作，再写结论。

Dune 是链上数据平台，白话就是别人做好的链上查询和仪表盘；口径必须核对。

## 矩阵复核清单

| 矩阵       | 复核重点                                      | 不合格信号                  |
| ---------- | --------------------------------------------- | --------------------------- |
| 渠道议价   | Coinbase 分成、Base 绑定、USDC rewards、渠道集中度 | 供应增长但 RLDC margin 下滑 |
| 稳定币竞争 | USDT dominance、PYUSD、FDUSD、USDe、RWA、银行稳定币 | 总盘扩张但 USDC 份额下降    |
| 平台化     | CPN TPV、Other revenue、Arc 主网、费用模型    | 使用量增长但收入不增长      |
| 监管       | 第三方激励、yield workaround、BSA/AML         | 激励被认定为变相 yield      |

USDC rewards 是 USDC 奖励，白话就是合作方用奖励吸引用户持有或使用 USDC。

## 升级规则

| 触发                              | 升级到         | 动作                              |
| --------------------------------- | -------------- | --------------------------------- |
| competition score 高于 75         | Bull case 检查 | 看 Other revenue 和 RLDC 是否同步 |
| competition score 低于 40         | Bear case 检查 | 重算增长率和长期倍数              |
| Base USDC 增长但 RLDC margin 下滑 | P1 风险        | 检查 Coinbase 议价权              |
| USDC 市占率连续 4 周下降          | P1 风险        | 暂停上调估值倍数                  |
| 监管限制第三方激励                | P0 风险        | 重算分销经济性                    |
| 银行稳定币获得企业客户            | P2 风险        | 下调企业支付期权权重              |

## 输出模板

```text
结论：

依据：
1. USDC relative share：
2. distribution economics：
3. chain usage quality：
4. regulated enterprise adoption：
5. yield-product leakage：
6. bank/tokenized deposit pressure：
7. stock and options crowding：

动作：

风险：

missing_info：

下次复盘触发：
```

## 不合格输出

以下输出不合格：

- “USDC 仍是合规龙头。”
- “Base 增长很快，所以利好 CRCL。”
- “Tether 不透明，所以 USDC 更好。”
- “银行稳定币短期威胁不大。”

合格输出必须回答：

1. 市占率变了吗。
2. 增长留给 Circle 了吗。
3. 竞争变化影响估值了吗。
4. 哪个指标会让结论反转。

## 复盘后的写入位置

| 复盘结果 | 写入位置                              |
| -------- | ------------------------------------- |
| 日常趋势 | `metrics/04-competition-dashboard.md` |
| 周度结论 | `metrics/02-weekly-review.md`         |
| 风险触发 | `risk/01-warning-signals.md`          |
| 情景切换 | `valuation/01-scenario-model.md`      |
| 新来源   | `sources.md`                          |
| 新假设   | `autoresearch/01-iteration-log.md`    |
