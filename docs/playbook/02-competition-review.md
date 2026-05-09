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

---

## 新增（2026-05-09）

## 与05-scoring-rubric的集成说明

本文件与评分体系三个文件的角色分工如下：

| 文件 | 角色 | 职责范围 |
| ---- | ---- | -------- |
| `playbook/02-competition-review.md`（本文件） | **触发层** | 定义触发条件、执行步骤、升级规则、输出格式 |
| `metrics/05-competition-scoring-rubric.md` | **评分层** | 提供7维度逐档打分标准（0/25/50/75/100），裁定每维度具体分值 |
| `metrics/04-competition-dashboard.md` | **记录层** | 记录打分结果、评分历史追踪、validation绑定 |

**文件角色边界：**
- 不应在本文件（02）写入具体分数或定义档位阈值
- 不应在05-scoring-rubric记录触发条件或历史得分
- 不应在04-dashboard定义打分标准或执行对账流程

### 三文件执行顺序

```
1. 触发条件出现
       ↓
2. 查本文件（02）
   → 确认是否需要复盘
   → 确认触发类型（全维度重计 / 单维度更新）
       ↓
3. 需要复盘 → 用 05-scoring-rubric.md 执行打分
   ├── 全维度重计：所有7个维度重新评分，需两人独立打分
   └── 单维度更新：仅更新指定维度，其他沿用上周值，一人即可
       ↓
4. 打分完成 → 结果记录到 04-competition-dashboard.md
   ├── 更新"当前评分基准"表
   ├── 追加"评分历史追踪"一行
   └── 更新"本周快照"区域
       ↓
5. 打分异常检查（两人独立打分后）
   → 任意单维度差 ≥15分，或总分差 ≥10分
   → 回到 05-scoring-rubric.md 执行"评分异常处理规则"对账流程
   → 对账完成后方可记录到04-dashboard
       ↓
6. 复盘完成 → 执行 06-validation-matrix.md 相关WE条目校验
   ├── WE-02：D3分销议价档位 vs RLDC margin档位
   ├── WE-03：D4收益竞品档位 vs Other revenue趋势方向
   ├── WE-04：D7平台化档位 vs Other revenue share档位
   ├── WE-06：competition score总分档位 vs RLDC margin情景
   └── WE-07：分销成本率判断 vs distribution cost弹性方向
       ↓
7. 验证矩阵通过 → 输出标准格式复盘结论（见下方章节）
```

---

## 全维度重新计分触发条件

以下明确"什么情况下需要全维度重新计分（而不仅是更新单个维度）"。

**核心原则：** 全维度重计成本高（需两人独立打分+对账），仅在竞争格局可能整体变化时触发；局部变化只更新相关维度。

### 触发条件判断表

| 触发事件 | 是否全维度重计 | 说明 | 主要联动维度 |
| -------- | :----------: | ---- | ------------ |
| USDC市占率单月变化 >1.5 ppt（升或降） | ✅ 全维度 | 市占率大幅变化同时影响D1/D2/D3/D5 | D1、D2、D3、D5 |
| 主要竞品（USDT/USDe）宣布重大产品（合规化/主权背书/大型交易所独家） | ✅ 全维度 | 竞争格局可能整体重构 | D1、D2为主 |
| GENIUS Act最终规则发布 | ✅ 全维度 | 监管壁垒维度大幅变化，分销激励模式可能重写 | D6为主，D3、D2联动 |
| 季度财报发布后（Circle 10-Q/10-K） | ✅ 全维度 | RLDC margin更新影响D3，其他财务指标联动D4/D7 | D3、D7、D4 |
| Coinbase公告分销协议重大变化 | ✅ 全维度 | 分销成本假设根本性改变 | D3为主，D1联动 |
| OCC/FDIC发布限制第三方激励或yield sharing草案 | ✅ 全维度 | P0风险，监管模式根本性变化 | D6、D3全触发 |
| competition score总分跨越档位边界（如从55→39） | ✅ 全维度 | 档位边界变化需确认各维度是否同步 | 全部 |
| Base链单月USDC净增 >10%（占总USDC） | ❌ 仅D3 | 局部渠道变化，不触发全维度 | 仅D3 |
| 收益型稳定币AUM单月 >15%但USDC净赎回未出现 | ❌ 仅D4 | 局部变化，USDC供应未受冲击 | 仅D4 |
| 监管无新事件（周度常规） | ❌ D6维持上周 | 无事件周D6沿用，证据栏填"无新事件，沿用上周" | 不更新 |
| Short interest或IV单周异常波动 | ❌ 仅D7 | 股票指标局部变化，不触发全维度 | 仅D7 |

### 全维度重计与单维度更新的执行差异

| 项目 | 全维度重计 | 单维度更新 |
| ---- | ---------- | ---------- |
| 打分人数 | 两人独立打分 | 一人打分，自我复核 |
| 对账要求 | 差≥15分触发05-scoring-rubric对账流程 | 无需对账 |
| 06-validation-matrix | 执行全部相关WE条目（WE-02至WE-07） | 仅执行与更新维度相关的WE条目 |
| 04-dashboard更新 | 更新全部7维度+总分+历史追踪 | 仅更新对应维度和总分 |
| 时限 | 触发事件后48小时内完成 | 触发事件后24小时内完成 |

---

## 复盘输出标准格式（接轨01-decision-template）

复盘完成后，按以下格式输出结论，并写入04-competition-dashboard.md"周度记录模板"区域：

```
竞争复盘结论（日期：____）
本周评分：___分（上周：___分，变化：___分）
档位变化：___→___ 或 无变化
核心驱动：（单句说明分数变化的主因，例如："D3 RLDC margin Q1=40.3%维持75分，
           D1市占横盘+0.2 ppt导致总分维持57.5"）
联动动作：
  - 估值：倍数维持/调整（如有，链接valuation/01-scenario-model.md具体段落）
  - 风险：预警触发/解除（如有，链接risk/01-warning-signals.md具体预警编号）
  - 文件更新：（列明本次复盘更新了哪些文件及更新内容摘要）
下次触发：（下次复盘的触发条件或日期，例如：
           "Q2 2026财报后全维度重计；或USDC市占单月变化>1.5 ppt提前触发"）
```

### 合格输出的四项必答问题

复盘结论必须能回答以下4个问题，否则视为不合格输出：

| 必答问题 | 对应结论字段 |
| -------- | ------------ |
| 市占率变了吗？变了多少？ | D1分数及具体数值依据 |
| 增长留给Circle了吗（渠道利润留存）？ | D3分数及RLDC margin数值 |
| 竞争变化影响估值了吗？ | 联动动作-估值字段 |
| 哪个指标会让结论反转？ | 下次触发条件字段 |

### 不合格输出示例

以下表述在复盘结论中不被接受：

- "USDC仍是合规龙头。" → 无数据支撑
- "Base增长很快，所以利好CRCL。" → 未核实RLDC margin是否同步改善
- "竞争结构暂时中性。" → 未说明分数变化原因和下次触发条件
- 只写总分不写各维度 → 无法执行WE系列validation check

### 与06-validation-matrix的对接规则

每次复盘结论写入后，在06-validation-matrix.md对应检查记录模板的"第二层（周报→财报）"补充以下内容：

```
WE-02 D3档位核查：___ 通过 / 不一致（原因：___）
WE-03 D4方向核查：___ 通过 / 不一致（原因：___）
WE-06 总分档位核查：___ 通过 / 不一致（原因：___）
WE-07 分销成本率核查：___ 通过 / 不一致（原因：___）
发现的不一致：（无 / 编号___，描述，修正摘要）
```

若WE条目出现不一致，按06-validation-matrix.md"发现不一致时的处理流程"执行（数据来源优先级：SEC filing > Circle官方 > DefiLlama > Dune），处理完成后方可视为本次复盘结束。
