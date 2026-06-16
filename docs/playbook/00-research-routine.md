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

---

## 新增（2026-05-08）

### 四类事件处理 SOP

事件流程不再统一处理，按类型分四条独立 SOP 执行。每类 SOP 有独立触发条件、处理步骤和时间要求。

---

#### 类型1：监管事件（OCC/Treasury/Congress新规）

```text
触发：Congress.gov、OCC官网、Federal Register出现CRCL相关新规

处理步骤：
1. 确认来源（官方页面截图存档，非媒体二手）
2. 判断层级：最终规则 > 拟议规则 > 国会听证
3. 比对 framework/02-regulation.md 的10项复核矩阵
4. 量化影响：对照"监管成本三档情景表"判断属于宽松/中性/严格
5. 更新 risk/00-risk-map.md 监管风险优先级
6. 输出结论（套用 playbook/01-decision-template.md 风险事件模板）

时间要求：
  最终规则当天输出结论
  拟议规则48h内输出结论
  国会听证72h内判断是否影响拟议规则方向

禁止：将评论期内的意见或草案当成已生效规则处理。
```

---

#### 类型2：储备/流动性事件（USDC净赎回、储备异常）

```text
触发：C-TRIGGER A/B/C任一触发，或单日赎回>1亿美元

处理步骤：
1. 核实数据来源（Circle Transparency + The Block双源，缺一不用）
2. 判断是否达到C-TRIGGER阈值（见 risk/01-warning-signals.md 速查表）
3. 检查储备资产构成是否正常（T-Bill vs Repo比例是否偏移）
4. 更新 metrics/01-daily-watchlist.md 当日条目
5. 触发框架自检（见 docs/README.md 框架自检SOP）
6. 输出结论（套用 playbook/01-decision-template.md 风险事件模板）

时间要求：当日输出结论，不跨日。

优先级：信用层事件优先于所有估值信号，不等周度复盘。
```

---

#### 类型3：竞争事件（对手重大动作、市占率突破）

```text
触发：USDC市占率MoM变化>±1.5%，或主要竞争对手宣布重大产品/合规进展

处理步骤：
1. 更新 metrics/04-competition-dashboard.md 当周数据
2. 用 metrics/05-competition-scoring-rubric.md 重新计分
3. 与上周评分对比：
   - 变化≤10分：记录观察，不触发专项复盘
   - 变化>10分：触发专项复盘（playbook/02-competition-review.md）
4. 判断是否触发C-TRIGGER组合预警（A/B/C/D）
5. 输出结论（套用 playbook/01-decision-template.md 周度复盘模板）

时间要求：
  48h内完成评分更新
  72h内输出结论

注意：市占率下降必须同步检查是否为绝对规模下降，或仅为竞争对手增速更快。
```

---

#### 类型4：产品事件（CPN/Arc里程碑、管理层声明）

```text
触发：Circle官方宣布CPN机构数变化、Arc主网进展、Other revenue披露

处理步骤：
1. 核实来源（Circle官网/IR/SEC filing，非媒体二手报道）
2. 对照 framework/04-platform-option.md 里程碑表：是否达到当季目标？
3. 判断平台化验证等级是否升级（弱→中→强）
   - 单项里程碑未达成：列入missing_info，不触发降级
   - 缺2项以上：当前验证等级下移一级
4. 更新 valuation/01-scenario-model.md Other revenue拆分假设
5. 输出结论（套用 playbook/01-decision-template.md 标准模板）

时间要求：官方宣布后24h内输出结论。

禁止：以TPV增长代替Other revenue增长作为平台化验证依据。
```
