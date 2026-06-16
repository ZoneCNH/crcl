# GENIUS Act 最终规则响应计划

更新日期：2026-05-09 | 类型：战略情景响应计划

---

## 定位说明

本文件是预建响应剧本。OCC/Treasury GENIUS Act 最终规则正式发布时，研究者不需要临时思考第一步，直接按本计划执行。

---

## 1. 事件定义

- **触发条件**：OCC 或 Treasury/FinCEN 发布 GENIUS Act 最终规则（Final Rule），在 Federal Register 正式刊登（非拟议规则 NPRM）
- **预期时间窗口**：2026 Q3（基于 `../../framework/02-regulation.md` 监管事件监听日历）
- **来源确认**：Federal Register（federalregister.gov）+ OCC 官网（occ.gov）+ Treasury 官网（home.treasury.gov）三源同步确认

运行时基准状态（执行前从 Federal Register、OCC、Treasury、FinCEN、FDIC 刷新；下表为历史待核模板）：

| 机构 | 运行时状态 | 预期下一步 |
|------|---------|-----------|
| OCC | GENIUS Act 拟议规则（NPRM）已发布（2026-02-25） | 评论期截止后发布最终规则 |
| Treasury/FinCEN | BSA/AML 拟议规则已发布（2026-04-08） | 2026 Q3 评论期截止后最终规则 |
| FDIC | PPSI 最终规则待发布 | 2026 Q3–Q4 |

> ⚠️ 评论期内的意见不视为已生效依据，不更新框架监管状态。只有 Federal Register 正式刊登的最终规则才触发本计划。

---

## 2. 最终规则发布后 72 小时执行清单

### T+0（发布后 2 小时内）

- [ ] 确认：Federal Register 正式刊登（Final Rule，非 NPRM）
- [ ] 下载并保存 Federal Register 原文（PDF 存档，备注刊登日期和 Docket Number）
- [ ] 记录关键信息：
  - 生效日期（Effective Date）
  - 合规截止日期（Compliance Date）
  - 规则主管机构（OCC / Treasury / 联合发布）

### T+0（当天，6 小时内）

扫描五个核心条款，对应 `../../framework/02-regulation.md` 监管复核矩阵：

- [ ] **条款 1**：是否限制收益分享 / 第三方激励（yield workaround 条款）
  - 明确允许"服务费"模式 → 宽松档
  - 禁止任何间接收益分享 → 严格档（立即触发 P0 预警）
- [ ] **条款 2**：PPSI（Permitted Payment Stablecoin Issuer）许可路径是否明确
  - 审批周期 6–9 个月 → 宽松档
  - 审批周期 18 个月以上 → 严格档
- [ ] **条款 3**：储备资产要求是否比拟议规则更严格
  - 等同拟议规则 → 宽松档
  - 更严格期限要求或范围限制 → 严格档
- [ ] **条款 4**：BSA/AML 合规成本是否有量化要求
  - 等同拟议 → 宽松档（+5% opex）
  - 更高监控标准 → 严格档（+20% opex）
- [ ] **条款 5**：银行稳定币的竞争地位如何界定
  - 限制银行直接发行 → 宽松档（竞争格局不变）
  - 允许银行自由发行 → 严格档（enterprise 支付竞争恶化）

将以上五条结果对应"监管成本三档情景表"（来自 `../../framework/02-regulation.md`），确定最终规则档位：宽松 / 中性 / 严格。

### T+24h

- [ ] **量化影响**：按三档情景计算对 RLDC margin 的影响（参照 `../../framework/02-regulation.md` 情景量化表）

  | 情景 | RLDC margin 影响 | 其他影响 |
  |------|-----------------|---------|
  | 宽松 | <0.5ppts | CPN 接入摩擦低 |
  | 中性 | -0.5 至 -1ppts | 需新增 KYC 层 |
  | 严格 | -1.5 至 -3ppts | 触发 P0 预警 |

- [ ] **P0 判断**：检查是否触发 `../../risk/01-warning-signals.md` 信号 6（监管条款限制收益分享或让银行获得成本优势）
  - 触发 → P0 预警，当日重算估值，暂停基于当前 RLDC margin 的目标价计算
  - 不触发 → P1 或 P2，按周度更新成本假设
- [ ] 更新 `../../framework/02-regulation.md`：GENIUS Act 相关规则状态从"拟议规则"改为"最终规则"，记录生效日期

### T+48h

- [ ] **联动评估**：Coinbase 分销激励是否受影响

  按 `../../framework/02-regulation.md` 三种法律解释情景判断：
  - 解释 A（奖励属于"用户激励"）→ 零变化，维持当前分销成本假设
  - 解释 B（奖励属于"变相利息"）→ P0 预警，分销成本结构重组，立即重算 RLDC margin
  - 解释 C（规则明确允许"服务费"模式）→ 微调（<1ppts 变化），不触发 P0

- [ ] 更新 `../../valuation/01-scenario-model.md`：根据三档情景判断是否调整监管折价参数
- [ ] 输出：使用 `../../playbook/01-decision-template.md` **标准模板**输出正式结论（见第 6 节示例）

---

## 3. 核心条款判断矩阵

| 条款 | 宽松判定 | 严格判定 | 对 RLDC margin 影响 |
|------|---------|---------|-------------------|
| 收益分享限制 | 明确允许"服务费"模式，未将交易所奖励归类为 yield | 禁止任何间接收益分享，交易所激励被定义为 yield workaround | 宽松 0ppts / 严格 -2 至 -3ppts，触发 P0 |
| PPSI 路径 | 6–9 个月审批，联邦路径清晰 | 18 个月以上审批，门槛高于现行 | 宽松不影响 / 严格压制 CPN 扩张速度，审批周期拖长 2–3 季度 |
| 储备要求 | 等同拟议规则，储备范围不变 | 更严格期限要求（如强制超短久期），压缩储备收益率 | 宽松不影响 / 严格 +0.3% 资金成本（约 -0.5ppts RLDC） |
| BSA/AML | 等同拟议，无额外量化监控要求 | 更高监控标准，要求实时链上监测 | 宽松 +5% opex / 严格 +20% opex，压缩 Adjusted EBITDA |
| 银行稳定币 | 限制银行直接发行或设置高门槛 | 允许银行自由发行，享受监管成本优势 | 宽松竞争格局不变 / 严格降低 competition score 5–10 分 |

---

## 4. 情景切换规则

| 最终规则档位 | 对框架情景的影响 | 具体动作 |
|------------|---------------|---------|
| **宽松**（无重大限制，明确允许现有模式） | Bull 情景监管条件满足 | 上调监管壁垒维度竞争评分；`../../valuation/01-scenario-model.md` Bull 第 7 条标记为满足 |
| **中性**（部分新增要求，不限制分销激励） | 维持 Base 情景，监管壁垒得分小幅上调 | 更新合规成本假设；P1 级别观察；不触发情景切换 |
| **严格**（收益分享受限 / 银行获低成本优势） | Bear 情景触发条件之一满足 | P0 预警；当日重算 RLDC margin；评估其他 Bear 条件是否同步满足 |

**矩阵共振提醒**（来自 `../../valuation/01-scenario-model.md`）：情景切换须满足矩阵共振，不能由单一监管事件独立触发情景降级。严格规则 + 其他 Bear 条件中任意两项同时满足 → 执行 Bear 降级流程。

---

## 5. 风险项

| 风险 | 信号 | 动作 |
|------|------|------|
| 最终规则比拟议更严格（超预期） | Federal Register 正文中出现对"间接激励"的明确定义 | 立即升级 P0，暂停目标价计算，启动 RLDC margin 重算 |
| OCC 与 Treasury 规则存在冲突 | 两机构规则文本中关键定义不一致 | 等待监管机构联合说明或 No-Action Letter；将冲突项标注为 missing_info |
| 规则生效日期早于预期 | 合规截止日期 <90 天 | 评估 Circle 现有合规路径是否需要立即调整，追问下季财报 |
| 银行稳定币条款出现新让步 | 允许银行用表内储备发行稳定币 | 更新 competition score，检查 Bear 情景第 9 条（银行稳定币获企业采用）是否满足 |

---

## 6. 决策输出示例（使用标准模板）

```text
结论：GENIUS Act 最终规则已发布，判定为 [宽松/中性/严格] 档，[维持 Base / 上调监管壁垒 / 触发 P0]。

依据：
1. Federal Register [日期] 正式刊登，Final Rule（非 NPRM），生效日期 [日期]
2. 收益分享条款：[明确允许服务费模式 / 未定义 / 禁止间接激励] → [宽松/中性/严格]
3. PPSI 路径：[6–9 月 / 12–18 月 / >18 月] → [宽松/中性/严格]

矩阵复核：
1. 财务桥：RLDC margin 受监管影响预估 [X ppts]，参数更新至 [Y%]
2. 监管：三档情景判定为 [宽松/中性/严格]；Coinbase 激励属解释 [A/B/C]
3. 竞争：competition score 监管壁垒维度 [上调/维持/下调] [X] 分
4. 平台化：CPN PPSI 审批周期影响扩张节奏，预计 [X] 季度

动作：
[维持目标价框架 / 更新合规成本假设 / P0：暂停目标价计算，重算 RLDC margin]

风险：
银行稳定币条款执行细则待观察；解释 B 风险未完全排除

下次复盘触发：
Coinbase earnings call 是否提及奖励模式合规调整；或 OCC No-Action Letter 发布
```

---

## 7. 文件更新清单

| 文件 | 更新内容 | 更新时间 |
|------|---------|---------|
| `../../framework/02-regulation.md` | 规则状态（拟议→最终）、三档情景判定结果、生效日期 | T+0 当天 |
| `../../risk/01-warning-signals.md` | 信号 6 触发/解除状态更新（注明来源和日期） | T+24h |
| `../../valuation/01-scenario-model.md` | 监管折价参数、Bull 第 7 条满足状态 | T+48h |
| `../../metrics/04-competition-dashboard.md` | 监管壁垒维度得分、银行稳定币竞争评分 | T+48h |
| `../../autoresearch/01-iteration-log.md` | B 类 defer（监管观察）项目 resolved 或状态更新 | T+72h |

---

## 相关文件索引

- `../../framework/02-regulation.md`：GENIUS Act 主线、OCC/Treasury 拟议规则、监管成本三档情景、监管事件监听日历、Coinbase 分销关系影响
- `../../risk/01-warning-signals.md`：信号 6（监管条款限制收益分享）量化阈值和响应动作
- `../../valuation/01-scenario-model.md`：Bear 触发条件第 4 条、Bull 触发条件第 7 条
- `../../playbook/01-decision-template.md`：标准决策输出模板、多矩阵冲突处理规则
- `../../metrics/04-competition-dashboard.md`：监管壁垒维度得分
- `../../autoresearch/01-iteration-log.md`：迭代日志，B 类 defer 项目
