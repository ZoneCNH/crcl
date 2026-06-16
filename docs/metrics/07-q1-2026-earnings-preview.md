# Q1 2026 财报预期文件

文档版本日期：2026-05-08 | 历史财报日期：2026-05-11

> 本文件是 Q1 2026 财报前的历史预期样本，不是运行时当前事实。若用于后续季度，必须新建当季 preview，并先从 SEC EDGAR / Circle IR / Circle pressroom 刷新财报日期、最新 filing、电话会材料和关键阈值。

---

## 1. 执行摘要

**历史财报日期**：2026-05-11

**核心判断问题**：Q1 2026 的 RLDC margin 能否维持 40% 以上，同时 Other revenue 追上 FY2026 年化指引节奏（≥3,500 万/季），从而确认 Circle 仍在 Base case 而非向 Bear 滑落？

**历史情景基准**：**Base case**

- 依据：Q4 2025 RLDC margin 约 42%（来源：`../framework/01-business-model.md` Q4 2025 核心事实表），FY2025 全年 41%，均高于 38% 告警线；USDC 流通量 Q4 期末 753 亿，同比增长 72%；Other revenue 占比 ~4%，仍处利差股定价阶段，平台化期权价值未兑现但未恶化；无 C-TRIGGER 触发。
- Base 反证：若 RLDC margin 跌破 38% 或 Other revenue 低于 3,000 万且 FY2026 指引下修，框架向 Bear 滑落。

---

## 2. 五项优先检查预期表

> 数字来源：`../framework/01-business-model.md`（FY2025/Q4 2025 事实表 + Q1 2026E 外推，2026-05-08 补充）；`../metrics/03-quarterly-earnings.md`（Q1 2026 盈利前预期基准 + 快查表）；`../risk/01-warning-signals.md`（量化阈值规格）

| # | 检查项 | FY2025 全年 | Q4 2025 实际 | Q1 2026E（Base） | 告警下限（单项） | 触发动作 |
|---|--------|------------|-------------|-----------------|----------------|--------|
| 1 | **RLDC margin**（主口径：÷ Reserve income） | ~41%（10.83 ÷ 26.37 亿） | ~42%（3.09 ÷ 7.33 亿） | **39–41%**（目标 >40%） | **< 38%**（单季即触发） | 跳过决策树第5/6步，直接执行第7步（`../risk/00-risk-map.md` 信用层复核）+ 减仓 −15 ppts |
| 2 | **Reserve return rate**（储备收益率） | 全年约 4.3–4.6%（推算，待 10-K 确认） | **3.8%**（同比降 68 bps） | **3.5–3.8%**（降息预期 25–50 bps 部分传导，USDXX 约 3.5–3.8%） | < 3.3%（环比降 >50 bps 且 USDC 增速 <15%） | 触发预警信号 8；重算利率敏感度矩阵；减仓 −10 ppts |
| 3 | **Average USDC in circulation**（季内日均，亿美元） | 全年约 570–585（依 Reserve income 反算，待 10-K 确认） | **762 亿**（同比增 100%） | **790–820 亿**（Q4 末 753 亿 + 季度环比增势延续；`../metrics/03-quarterly-earnings.md` 锚定 760–800 亿） | < 760 亿（低于 Q4 期末，季度不增长） | 触发预警信号 2 观察；重算 Reserve income；若同期净赎回持续升级至 C-TRIGGER-C 审查 |
| 4 | **Other revenue**（Subscription and service fees，百万美元） | **1.098 亿**（占比 ~4.0%，利差股定价阶段） | **3,700 万** | **3,500–4,000 万**（FY2026 指引年化 1.5–1.7 亿 ÷4 ≈ 3,750–4,250 万；Q4 持平参照） | < 3,000 万（年化 <1.2 亿，低于指引下沿 1.5 亿） | 停止平台化估值溢价；RLDC 倍数降至 Base 下沿 15×；追问管理层是否下修 FY2026 指引 |
| 5 | **Distribution and transaction costs**（分销成本，亿美元）及**分销成本率**（÷ Reserve income） | **16.615 亿**；成本率 ~63%（16.615 ÷ 26.368） | Q4 约 4.24 亿（7.33 − 3.09 = 4.24；估算）；成本率 ~57.8% | **4.4–4.7 亿**；成本率维持 ~60–64%（参照 FY2025 基准） | 成本率环比上升 >2 ppts（即超 ~64%）；或分销成本 QoQ 增速 > Reserve income QoQ 增速 >5 ppts 连续 2 季 | 重算 RLDC margin；触发预警信号 4 观察；追问 Coinbase 协议变化 |

**注：** RLDC margin < 38% 无论其他指标如何，单项触发降级检查。两项以上低于告警阈值触发完整情景降级检查（对照 `../valuation/01-scenario-model.md`）。

---

## 3. 管理层电话会重点追踪

> 优先级顺序：量化信息 > 政策表述 > 管理层语气 > 分析师追问。见 `../metrics/03-quarterly-earnings.md` 听电话会优先级顺序。

| # | 追踪问题 | 所属矩阵 | 影响哪个指标/哪个阈值判断 |
|---|---------|---------|--------------------------|
| 1 | **CPN 机构数、年化 TPV 更新值？是否披露收费模式或实际收入贡献？** | 平台化矩阵 | 影响 Other revenue 拆分（CPN 子项假设）；Q2 里程碑判断（见 `../framework/04-platform-option.md`）；若无商业化收入则列为 missing_info，不上调平台化倍数 |
| 2 | **分销成本率：是否有 Coinbase 或其他渠道新的合同变化或分成比例调整？** | 财务矩阵 | 影响 RLDC margin 驱动因素分解（分支 2）；若 Coinbase 持有 USDC 占比超 30%（基准 ~25%）且 RLDC margin 同比降 >2 ppts，触发预警信号 7 |
| 3 | **GENIUS Act 拟议规则（OCC 2026-02-25）及 BSA/AML 规则（Treasury/FinCEN 2026-04-08）：管理层如何评估对分销激励和 RLDC margin 的具体影响？是否预期成本上升？** | 监管矩阵 | 影响 `../framework/02-regulation.md` 监管口径；若明确限制第三方激励，触发预警信号 6（下修 RLDC margin 至 30–35%）；若中性/有利，强化 Base 情景 |
| 4 | **Arc 主网：时间表是否有更新？是否有企业客户试点案例或收入确认计划？** | 平台化矩阵 | 影响 `../framework/04-platform-option.md` Arc 收入假设（Base: Q3 2026 上线；Bull: Q2 2026 上线）；延期则 Bear Arc 收入归零 |
| 5 | **USDC 增长来源：交易、DeFi、支付、企业结算各占比？是否有 Base 链渠道内循环 vs 外部新增的拆分？** | 财务矩阵 + 竞争矩阵 | 影响 average USDC 增长质量判断（RLDC margin 分支 3：Base 增长但总市占率不升属渠道内循环，不支持估值上修）；影响预警信号 13 |
| 6 | **FY2026 Other revenue 指引：维持 1.5–1.7 亿，还是上修/下修？** | 财务矩阵 | 影响 Other revenue 阈值判断（<1.5 亿下沿即触发平台化失效复核）；影响 `../valuation/01-scenario-model.md` Other revenue 假设（Base 1.6 亿） |
| 7 | **储备收益率展望：Q2 2026 的降息预期传导速度？WAM（加权平均久期）有无调整？** | 财务矩阵 | 影响 `../valuation/00-valuation-framework.md` 利率敏感度矩阵；确认 Q2 Base 储备收益率假设是否需要下修 |
| 8 | **Coinbase USDC 持有比例（来自 Coinbase Q1 2026 财报）是否变化？管理层是否提及渠道议价情况？** | 财务矩阵 + 竞争矩阵 | 影响预警信号 7 量化阈值（超 30% + RLDC margin 降 >2 ppts 联动）；影响 `../framework/01-business-model.md` Coinbase 占比假设（历史样本约 25%，运行时需刷新） |
| 9 | **竞争：USDT、银行稳定币（tokenized deposits）、USDe 等对 USDC 市占率的影响？管理层如何回应？** | 竞争矩阵 | 影响预警信号 3（市占率下降）和信号 16（银行稳定币采用）；影响 `../metrics/04-competition-dashboard.md` competition score 更新 |
| 10 | **锁定期及股权结构：是否有解禁计划更新？内部人 Form 4 近期有无集中减持信号？** | 筹码与事件矩阵 | 影响仓位加减因子（见 `../valuation/00-valuation-framework.md` 仓位管理规则）；锁定期解禁规模 >10% 流通量触发 P2 减仓 −5–10 ppts |

---

## 4. 情景触发条件快核

> 来源：`../valuation/01-scenario-model.md`（Bull/Base/Bear 触发条件 + 情景切换表）；下表状态基于 2026-05-08 历史已知数据，仅作 Q1 预案样本

| 情景 | 触发条件 | Q1 财报可核？ | 判断方式 | 历史状态（2026-05-08） |
|-----|---------|------------|---------|----------------------|
| **Bull** | ① RLDC margin 稳在 40% 以上 | ✅ 可核 | SEC 10-Q 自算：RLDC ÷ Reserve income | Q4 2025 约 42%，当时待 Q1 确认 |
| **Bull** | ② Other revenue 超过 FY2026 指引上沿（>4,250 万/季） | ✅ 可核 | SEC 10-Q：Subscription and service fees | Q4 2025 为 3,700 万，尚低于上沿 |
| **Bull** | ③ USDC 30D change 持续为正 | ✅ 可核 | Circle Transparency 净铸造/销毁数据 | Q4 2025 末 753 亿，趋势待确认 |
| **Bull** | ④ USDC 市占率连续 4 周上升 | ⚠️ 部分可核 | DefiLlama 周快照（非财报披露） | 需链上数据，财报不直接披露 |
| **Bull** | ⑤ CPN TPV 与收入同步增长 | ⚠️ 部分可核 | 需管理层披露具体数字；Other revenue 财报可核但子项不一定拆分 | 历史披露：2026-02-20 年化 TPV 约 57 亿，当时待 Q1 更新确认 |
| **Bull** | ⑥ Arc 主网按期落地并披露客户案例 | ✅ 可核（公告） | 管理层公告 + SEC filing | 尚无主网宣布，延期风险存在 |
| **Bull** | ⑦ 监管最终规则未实质压制分销激励 | ⚠️ 部分可核 | OCC/Treasury 规则仍在征询阶段（拟议规则 2026-02-25 & 2026-04-08），最终规则未发布 | 征询期，尚无最终规则；管理层措辞可参考 |
| **Base** | ① RLDC margin 维持 38–40% | ✅ 可核 | SEC 10-Q 自算 | Q4 2025 约 42%，处 Bull/Base 边界以上 |
| **Base** | ② Other revenue 达到 FY2026 1.5–1.7 亿年化节奏 | ✅ 可核 | SEC 10-Q（Q1 季度值 × 4 年化）；Q1 基准 3,500–4,000 万 | Q4 2025 为 3,700 万，基本在轨 |
| **Base** | ③ USDC 随稳定币总盘增长，市占率横盘 | ⚠️ 部分可核 | DefiLlama 周快照，非 10-Q 直接披露 | 需链上数据持续确认 |
| **Base** | ④ CPN/Arc 有进展但收入贡献不清晰 | ✅ 可核 | 管理层电话会量化信息；Other revenue 财报值 | 历史状态：使用量增长，收入未单独拆分 |
| **Bear** | ① RLDC margin 下滑（< 38%） | ✅ 可核 | SEC 10-Q 自算（决策树节点①） | 历史状态高于阈值；Q1 需重新确认 |
| **Bear** | ② OCC/Treasury 最终规则限制第三方激励 | ❌ Q1 不可核 | 最终规则未发布，需等后续监管流程 | 仍在征询阶段，拟议规则 2026-04-08 |
| **Bear** | ③ USDC 连续 2–4 周净赎回 | ✅ 可核 | Circle Transparency 日更数据；10-Q 期末 USDC | 需实时监控，Q1 末值从财报可核 |
| **Bear** | ④ competition score 低于 40 | ❌ Q1 不可核 | 需 7 维度评分完整更新（含链上数据、竞争对手财报） | 需等竞争对手财报（Coinbase 等）后更新 |
| **情景切换规则** | 单一指标不触发情景切换；需多矩阵共振 | — | 财务强化+竞争强化+平台化中验证→Bull；任意两项（财务降级/竞争恶化/监管压制/平台化失败）→Bear | 历史情景：**Base**，无切换信号 |

---

## 5. 财报当天执行清单

> 操作纪律来源：`../metrics/03-quarterly-earnings.md`（T+0/T+24h/T+48h 更新优先级清单 + 决策树时间约束）

### T+0（财报发布后 30 分钟内）：必须填写的 5 个数字

填写来源：SEC 10-Q Income Statement + MD&A（正式提交通常在盘后）

| 序号 | 指标 | SEC 来源科目 | 实际值 | 基准预期 | 判断 |
|-----|------|-----------|--------|---------|------|
| ① | Average USDC in circulation（亿美元） | 10-Q MD&A "average USDC in circulation" | ___ | 790–820 亿 | 强化 / 中性 / 降级 |
| ② | RLDC margin 主（%）= RLDC ÷ Reserve income | 10-Q Income Statement 自算 | ___ | 39–41% | ≥ 40% / 38–40% / **< 38%** |
| ③ | Other revenue（百万美元） | 10-Q：Subscription and service fees | ___ | 35–40 M | 强化 / 中性 / 降级 |
| ④ | Reserve return rate（%）= Reserve income ÷（Average USDC × 4）年化 | 10-Q 自算 | ___ | 3.5–3.8% | 强化 / 中性 / 降级 |
| ⑤ | Distribution and transaction costs（亿美元）+ 成本率（÷ Reserve income） | 10-Q Income Statement 费用明细 | ___ | 4.4–4.7 亿；成本率 ~60–64% | 成本率：___% |

### T+0（电话会开始前）：决策树节点 ①②③ 输入值

执行 `../metrics/03-quarterly-earnings.md` 财报处理决策树，顺序判断：

```text
节点 ①：RLDC margin（②）< 38%？
  填入实际值：___%
  → 是（< 38%）：跳第5/6步，准备 risk/00-risk-map.md 信用层复核材料，暂停仓位操作
  → 否，继续节点②

节点 ②：RLDC margin 是否在 38%–40%（预警区间）？
  → 是：发 P1 预警；执行完整五项检查（第4至7步）；输出：中性 + 预警结论
  → 否（≥ 40%）：继续节点③

节点 ③：Other revenue（③）是否 > 3,500 万（Q1 2026 阈值）？
  填入实际值：___M
  → 是：执行第4/6步（简化复核），跳第5步；输出：增强 / 维持结论
  → 否：执行第4至7步（完整复核）；输出：观察结论（Other revenue 落后年化节奏）
```

### T+电话会（实时追踪）：10 个追踪问题

按第 3 节优先级顺序，实时记录以下内容：

1. CPN 年化 TPV 最新数字？披露收费模式或收入金额？
2. 分销成本率有无变化？Coinbase 协议有无重谈信号？
3. GENIUS Act / OCC 拟议规则：管理层对 RLDC margin 影响的具体表态（偏乐观 / 谨慎 / 中性）？
4. Arc 主网时间表：维持 Q3 2026 / 提前 / 延期？
5. USDC 增长来源拆分：交易 / DeFi / 支付 / 企业结算各占比？
6. FY2026 Other revenue 指引：维持 / 上修 / 下修？幅度？
7. Q2 2026 储备收益率展望（降息预期传导）？
8. Coinbase 持有 USDC 比例有无变化？
9. 竞争：对 USDT 扩张 / 银行稳定币 / USDe 竞争的回应？
10. 锁定期 / 股权计划有无更新（内部人持股或 10b5-1 计划）？

记录格式：文字原话 + 是否可验证（参照 `../metrics/03-quarterly-earnings.md` 电话会记录模板）

### T+24h：输出结论格式

直接套用 `../playbook/01-decision-template.md` 财报模板：

```text
结论：Q1 2026 财报 强化 / 中性 / 降级 CRCL 框架。

依据：
1. Average USDC：___亿 | 基准 790–820 亿 | 判断：强化/中性/降级
2. Reserve return rate：___% | 基准 3.5–3.8% | 判断：强化/中性/降级
3. RLDC margin（主）：___% | 基准 39–41% | 判断：强化/中性/降级
4. Other revenue share：___M | 基准 35–40M | 判断：强化/中性/降级
5. 十项财务复核：（见 ../framework/01-business-model.md 十项复核模板）
6. CPN / Arc 进展：

矩阵复核：
1. 财务桥：
2. 监管（GENIUS Act / OCC 拟议规则）：
3. 竞争（市占率 / competition score）：
4. 平台化（CPN / Arc / Other revenue 可重复性）：

动作：

风险：

下次复盘触发：
```

### T+48h：更新文件清单（按优先级）

| 优先级 | 文件路径 | 更新内容 | 操作规则 |
|------|--------|---------|---------|
| 第 1 步（T+0） | `../metrics/03-quarterly-earnings.md` | 计算表填入实际数值；Q1 2026 复盘填写模板 | 只填数字，不写结论 |
| 第 2 步（T+0） | `../framework/01-business-model.md` | FY/季度核心事实表：更新 RLDC、margin、Other revenue 实际值；历史季度数据基准表 Q1 2026E → 实际值 | 更新实际值 |
| 第 3 步（T+0） | `../metrics/04-competition-dashboard.md` | RLDC margin 行（季度数据唯一更新时机） | 季度数据行 |
| 第 4 步（T+24h） | `../metrics/03-quarterly-earnings.md` | 十项财务复核完整填写；财报结论模板；Q1 2026 预期差对比（本文件第 2 节 vs 实际） | 必须完成，不可省略 |
| 第 5 步（T+24h，如情景升降级） | `../valuation/01-scenario-model.md` | 情景输出模板；Bull/Base/Bear 触发条件复核 | 仅矩阵共振触发情景切换时更新 |
| 第 6 步（T+24h，强制） | `../valuation/00-valuation-framework.md` | 目标价计算模板；稀释股数从当季 10-Q / 10-K 更新 | 每季度强制更新一次 |
| 第 7 步（T+24h，条件） | `../risk/00-risk-map.md` | 信用层触发条件复核 | RLDC margin < 38% 或 competition score 变化时 |
| 第 8 步（T+48h） | `../framework/04-platform-option.md` | CPN TPV / Arc 时间表最新进展 | 管理层给出新数字或时间表 |
| 第 9 步（T+48h） | `../framework/02-regulation.md` | 监管口径变化（GENIUS Act / OCC / BSA/AML 拟议规则管理层表态） | 管理层有新表态时 |
| 第 10 步（T+48h） | `../valuation/00-valuation-framework.md` | 倍数区间更新；可比公司倍数核对 | 情景切换或管理层上修/下修指引 |

---

## 6. 关键风险项（Q1 2026 特有）

### 风险 1：GENIUS Act 拟议规则落地时间窗口

- **背景**：GENIUS Act 已于 2025-07-18 成为 Public Law No. 119-27；OCC 已于 **2026-02-25** 发布拟议规则；Treasury / FinCEN / OFAC 已于 **2026-04-08** 发布稳定币 BSA/AML 与制裁合规拟议规则。
- **Q1 特有风险**：Q1 2026（1 月–3 月）处于 OCC 拟议规则征询期（2026-02-25 发布，征询截止日期待确认）和 Treasury/FinCEN 规则（2026-04-08 发布，约 60 天征询期）重叠窗口。管理层对**最终规则是否限制第三方分销激励**的表态，将直接影响 RLDC margin 长期假设。
- **Q1 财报判断规则**：若管理层明确表示规则"中性或有利"→ 维持 Base；若表示"需要调整分销激励结构"→ 触发预警信号 6，下修 RLDC margin 假设至 30–35%，暂停平台化期权估值。
- **来源参照**：`../framework/02-regulation.md`；`../README.md` 运行时研究基准

### 风险 2：Q1 2026 平均 5Y Treasury 收益率与储备收益率传导

- **利率环境估算**：基于 Q4 2025 5Y Treasury 约 4.3%（`../valuation/01-scenario-model.md` 利率敏感度矩阵基准参数），若 Q1 2026 降息 25–50 bps（传导系数约 85%），USDXX 7 日收益率约 **3.6–3.8%**，Reserve return rate 约 **3.5–3.8%**。若降息快于预期（单次 >50 bps），传导至 Reserve return rate 约 **3.0–3.4%**，触发预警信号 8（重算利率敏感度矩阵 + 减仓 −10 ppts）。
- **Q1 特有风险**：USDXX 的 WAM（加权平均久期）通常 30–60 天，降息时储备收益率下行可能滞后 1–2 个月；若管理层披露 WAM 有所拉长，Q2 传导风险更大。
- **需确认项**：Q1 财报发布时同步查 BlackRock USDXX 基金最新 7 日收益率（日更，非财报科目），作为 Reserve return rate 的代理验证指标。

### 风险 3：Coinbase 分销协议是否有到期/续签可能

- **背景**：Coinbase 是最大 USDC 分销渠道，Q1 2026 历史基准占总流通量约 25%（来源 Coinbase Q1 2026 10-Q）。复用本模板时必须改为对应季度的最新 Coinbase filing。若协议有续签谈判，分销成本率可能出现跳变。
- **Q1 特有风险**：Coinbase 自身业务扩张（包括 Base 链生态增长）可能增强议价权；若 Coinbase 持有 USDC 占比超过 30%（信号 7 量化阈值），需重点追问是否存在协议条款变化。
- **历史判断规则**：Coinbase Q1 2026 财报（约 2026-05-08 前后）披露 USDC 持有量后，与 Circle 财报联动判断。若 Coinbase 持有超 30% + Circle RLDC margin 同比降 >2 ppts → 预警信号 7 触发（见 `../risk/01-warning-signals.md`）。复用到未来季度时，需改用对应季度 Coinbase filing。

### 风险 4：Other revenue 增速是否追上 FY2026 指引节奏

- **Q1 特有风险**：FY2026 指引为 1.5–1.7 亿，Q1 季化目标为 3,750–4,250 万。Q4 2025 实际为 3,700 万，略低于指引季化中值。若 Q1 仍低于 3,500 万（观察阈值），且 CPN / Arc 无商业化信号，H1 累计将低于指引下沿的 40%（即 <6,000 万），触发 `../risk/01-warning-signals.md` 信号 5 量化阈值（平台化失效复核）。
- **判断规则**：Q1 实际 < 3,000 万 → 停止平台化估值溢价；Q1 在 3,000–3,500 万 → 观察结论，等 Q2 确认；Q1 > 4,500 万 → 平台化进展强化。

### 风险 5：IPO 后锁定期解禁筹码压力

- **背景**：CRCL IPO 后，通常 6 个月后锁定期到期；历史 Q1 样本在 2026-05-08 仍需从 S-1/Prospectus 确认具体解禁日期（见 `../README.md` IPO 后股权结构表）。运行时必须查最新 S-1、424B、Form 4 和 13F。
- **Q1 特有风险**：若锁定期在 Q2 2026 初（约 2026-04 至 05 月）到期，财报发布恰好临近解禁窗口，可能叠加解禁筹码压力。短期内多位高管连续减持需触发 P0 预警（预警信号 12）。
- **注意**：筹码信号独立于情景判断，只调仓位（−5–10 ppts），不修改估值倍数或情景框架。

---

## 7. 框架自检触发

财报后必须执行框架自检（触发条件：**每季度财报后强制自检**）。来源：`../README.md` 框架自检 SOP。

```text
自检日期：（财报后 T+24h 填写）
触发原因：本次刷新到的当季财报发布（填写最新财报日期），每季度强制触发

检查项：（逐项填写）
  业务模式假设（../framework/01-business-model.md）：
    FY2025/Q4 2025 基准数字是否已用 Q1 2026 实际值更新？
    有变化 / 无变化，说明：

  RLDC margin 阈值（../metrics/00-metric-dictionary.md）：
    38% 告警线是否仍合理？（参考近 4 季实际值：Q4 2025 ~42%，Q3 待填，Q2 待填，Q1 2026 实际___）
    有变化 / 无变化，说明：

  竞争评分权重（../framework/03-competition.md）：
    各维度权重是否反映当前竞争格局（含 GENIUS Act 落地后新入场者）？
    有变化 / 无变化，说明：

  情景参数（../valuation/01-scenario-model.md）：
    Bull/Base/Bear 的 USDC 假设是否需要根据 Q1 实际值更新？
    （参数表：Bear 700亿 / Base 850亿 / Bull 1,050亿；Q1 实际___亿对比）
    有变化 / 无变化，说明：

  目标价倍数（../valuation/00-valuation-framework.md）：
    Market cap / RLDC 倍数区间是否仍与可比公司匹配？
    （Bear 8-12× / Base 15-22× / Bull 25-40×）
    稀释股数是否已从最新 10-Q / 10-K 更新（运行时刷新后填写）？
    有变化 / 无变化，说明：

  风险优先级（../risk/00-risk-map.md）：
    OCC/Treasury 拟议规则发布后，监管风险 P0/P1 分类是否需要重排？
    有变化 / 无变化，说明：

  数据源有效性（../sources.md）：
    有无过期 URL 或来源不再更新的情况（特别是 BlackRock USDXX 基金页面）？
    有失效链接 / 全部有效

变更动作：
  需更新文件：（填写 T+48h 更新清单中已触发的文件）
  需新建任务：

下次自检触发：下一次已公告财报后（运行时从 Circle IR / SEC EDGAR 刷新日期）或情景切换时
```

---

## 数据来源清单

| 来源文件 | 引用内容 | 路径 |
|---------|---------|------|
| 商业模式与财务桥 | FY2025 核心事实表、Q4 2025 事实表、Q1 2026E 预期基准（2026-05-08 新增）、RLDC margin 驱动因素分解、历史季度数据基准表 | `../framework/01-business-model.md` |
| 季度财报检查清单 | 五项优先检查阈值表、Q1 2026 专用快查表、财报处理决策树、Q1 2026 盈利前预期基准、T+0/T+24h/T+48h 更新优先级清单 | `../metrics/03-quarterly-earnings.md` |
| 估值框架 | 情景参数表（Bear/Base/Bull）、利率敏感度矩阵（降息传导矩阵）、倍数区间、仓位加减因子表 | `../valuation/00-valuation-framework.md` |
| 情景模型 | Bull/Base/Bear 触发条件、情景切换表、Other revenue 子项假设拆分、利率敏感度矩阵（2026-05-08 补充） | `../valuation/01-scenario-model.md` |
| 预警信号 | 16 条预警信号量化阈值、分销成本率基准 ~63%、赎回级别定义、组合触发条件 | `../risk/01-warning-signals.md` |
| 研究框架入口 | 运行时研究基准、框架自检 SOP 及输出模板、IPO 后股权结构与锁定期时间表 | `../README.md` |
| 决策输出模板 | 财报结论输出格式 | `../playbook/01-decision-template.md` |

> 所有外部数据（实际财报数值、Coinbase 10-Q USDC 持有量、BlackRock USDXX 基金收益率）须在财报发布后 T+0 当晚从 SEC EDGAR 和官方页面获取，不从本文件外推。
