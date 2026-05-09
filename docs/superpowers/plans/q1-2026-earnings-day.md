# Q1 2026 财报日战术手册

更新日期：2026-05-09 | 财报日：2026-05-11 | 类型：单次事件执行脚本

> 本文件是 `../../metrics/07-q1-2026-earnings-preview.md` 的执行提炼版。
> 预览文件是"背景与预期"，本文件是"明天怎么做"。

---

## 出发前核对（财报发布前）

**Circle Q1 2026 财报发布时间**：2026-05-11（美东时间盘后，约 16:00–17:30 ET）

**发布渠道（按优先级）**：
1. SEC EDGAR：`efts.sec.gov` 搜索 Circle Internet Group 10-Q
2. Circle IR：`investor.circle.com/news`
3. Circle pressroom：`circle.com/pressroom`

**财报前禁止**：
- 禁止根据提前流出的"消息"调仓
- 禁止把 Coinbase Q1 2026 财报（如已发布）的 USDC 数字当作 Circle RLDC 结论

---

## 财报发布后执行时序

### T+0（财报发布后 30 分钟内）

**第一步：找到 10-Q 原文**
- SEC EDGAR → 搜索"Circle Internet Group" → 最新 10-Q → 下载 PDF 或 HTM
- 确认 filing 日期为 2026-05-11，不是历史 filing

**第二步：五项优先数字（按顺序抄录）**

| # | 抄录位置 | 抄录数字 | 告警阈值 | 触发？ |
|---|---------|---------|---------|-------|
| 1 | Revenue 表 → Reserve income | ___亿美元 | — | — |
| 2 | Revenue 表 → Distribution and transaction costs | ___亿美元 | — | — |
| 3 | 计算 RLDC = 第1行 − 第2行 | ___亿美元 | — | — |
| 4 | 计算 RLDC margin = RLDC ÷ Reserve income | __._% | **< 38% 立即触发** | ☐ |
| 5 | Revenue 表 → Subscription and service fees（Other revenue） | ___百万美元 | < 3,000 万警戒 | ☐ |
| 6 | MD&A 或 Supplemental → Average USDC in circulation | ___亿美元 | < 760 亿警戒 | ☐ |
| 7 | MD&A → Reserve return rate / average reserve yield | __._% | < 3.3% + USDC增速<15%联动 | ☐ |

**第三步：10 秒判断当前情景**

```
RLDC margin ≥ 40% + Other revenue ≥ 3,500万 → 维持 Base，Bull 条件部分满足，待电话会
RLDC margin 38-40% + Other revenue ≥ 3,000万 → 维持 Base 下沿，无切换，观察
RLDC margin < 38% → ⚠️ 立即进入 Bear 检查流程（见下）
Other revenue < 3,000万 → 停止平台化溢价，等电话会确认是否指引下修
```

---

### T+0（30–90 分钟，电话会前）

**第四步：抄录稀释股数**
- 10-Q 封面 → "Shares outstanding"，记录日期和数量
- 更新 `../../valuation/00-valuation-framework.md` 稀释股数版本记录表

**第五步：检查管理层 press release**
- 通常在 10-Q 同时发布，附在 8-K 中（exhibit 99.1）
- 提取：管理层对 FY2026 Other revenue 指引是否有变化（维持 1.5–1.7 亿 / 上修 / 下修）

**第六步：预判电话会关键追问**
- 根据五项数字，标记哪 3 个问题最重要（从 `../../metrics/07-q1-2026-earnings-preview.md` 第3节10题中选）

---

### 电话会（约 T+90min–T+3h）

**听的优先级**：量化数字 > 政策表述 > 管理层语气 > 分析师追问情绪

**必须记录（现场抄录）**：

| 问题 | 管理层实际回答（关键词） | 与预期差异 |
|------|----------------------|---------|
| CPN 机构数 / 年化 TPV 最新值 | | |
| Coinbase 分成是否有变化 | | |
| OCC/Treasury 监管规则对分销影响评估 | | |
| Arc 主网时间表（维持 / 延期 / 加速） | | |
| FY2026 Other revenue 指引（维持 / 上修 / 下修） | | |

---

### T+3h（电话会结束后）

**第七步：综合判断**

```
情景判断（选一个）：
  ☐ 维持 Base case → 无仓位变动，更新文件
  ☐ 升级至 Bull → [需全部 Bull 条件中多少项满足？] → 等更多数据（不在T+3h立即切换）
  ☐ 降级至 Bear → [触发了哪些条件？] → 执行减仓 SOP

仓位动作（来自 ../../valuation/00-valuation-framework.md 仓位加减因子表）：
  RLDC margin < 38%：减仓 −15 ppts
  Other revenue < 3,000万：减仓 −0 ppts（单独不减仓，但停止加仓）
  储备收益率 < 3.3% + USDC增速<15%：减仓 −10 ppts
  RLDC margin ≥ 40% + Other revenue ≥ 3,500万 + 指引维持：可考虑 +5–10 ppts
```

---

### T+24h（次日）

**第八步：更新以下文件**（按优先级，高→低）

| 文件 | 更新内容 | 优先级 |
|------|---------|-------|
| `../../framework/01-business-model.md` | Q1 2026 实际数字填入历史季度数据基准表 | P0 |
| `../../valuation/01-scenario-model.md` | 情景参数是否需要更新（USDC假设/收益率假设） | P0 |
| `../../valuation/00-valuation-framework.md` | 稀释股数更新，倍数落点重新计算 | P0 |
| `../../metrics/03-quarterly-earnings.md` | Q1 2026 快查表填入实际值 | P1 |
| `../../risk/01-warning-signals.md` | 预警信号触发 / 解除状态更新 | P1（如触发） |
| `../../framework/04-platform-option.md` | Arc / CPN 里程碑状态更新 | P1 |

---

### T+48h（两天后）

**第九步：执行框架自检**（触发条件：每季度财报后强制）

使用 `../../README.md` 末尾的框架自检模板，逐项填写后保存到本目录：
`../../autoresearch/01-iteration-log.md`（追加一条迭代记录，编号继续）

**第十步：竞争评分更新**
- 等 Coinbase Q1 2026 财报（如尚未发布，等发布后执行）
- 使用 `../../metrics/05-competition-scoring-rubric.md` 更新 7 维度评分
- 结果记录到 `../../metrics/04-competition-dashboard.md`

---

## Bear 触发时的紧急流程

> 仅在 RLDC margin < 38% 时执行

```
T+0 立即：
  1. 停止所有追加买入
  2. 执行 ../../risk/02-failure-conditions.md 失效复核矩阵
  3. 判断是"一次性因素"还是"趋势性恶化"：
     - 一次性（如季节性赎回、高费用一次性项）→ 观察，不立即减仓
     - 趋势性（如分销成本率 QoQ 连续两季上升 > 2ppts）→ 减仓 −15 ppts

T+24h：
  4. 重建 RLDC 驱动分解（是哪个分支导致？见 ../../framework/01-business-model.md 决策树）
  5. 更新 ../../valuation/01-scenario-model.md Bear 情景参数
  6. 用 ../../playbook/01-decision-template.md 输出正式结论（必须填写）
```

---

## 相关文件索引

| 文件 | 用途 |
|------|------|
| `../../metrics/07-q1-2026-earnings-preview.md` | 完整预期基准、情景触发可核清单、10个电话会问题 |
| `../../metrics/03-quarterly-earnings.md` | 财报决策树、Q1快查表、T+0/T+24h/T+48h更新优先级 |
| `../../framework/01-business-model.md` | 历史季度基准表、RLDC驱动分解、Q1 2026E基准 |
| `../../valuation/00-valuation-framework.md` | 倍数落点规则、仓位加减因子、稀释股数记录 |
| `../../valuation/01-scenario-model.md` | 情景参数表、利率敏感度矩阵 |
| `../../risk/01-warning-signals.md` | 16条预警阈值速查表 |
| `../../risk/02-failure-conditions.md` | 硬性失效条件与紧急流程 |
| `../../playbook/01-decision-template.md` | 结论输出模板 |
