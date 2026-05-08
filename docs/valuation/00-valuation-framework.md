# 估值框架

## 结论

CRCL 不能只看 P/S。

P/S 会高估储备收入质量，因为储备收入要先支付渠道分成。
更好的估值锚是 Market cap / RLDC、Market cap / net reserve economics、Other revenue multiple。

multiple 是估值倍数，白话就是市场愿意为一美元收入或利润付多少钱。

## 三段式估值

```text
基础盘 = average USDC circulation × reserve return rate × RLDC margin
期权盘 = CPN + Arc + enterprise settlement + API + other revenue
折价项 = regulation + distribution dependency + rate pressure + unlock / dilution
```

enterprise settlement 是企业结算，白话就是企业之间用 Circle 网络转账清算。

## 基础盘

基础盘来自储备经济性。

关键公式：

```text
Reserve income = average USDC circulation × reserve return rate
RLDC = total revenue and reserve income - distribution and transaction costs
RLDC margin = RLDC / total revenue and reserve income
```

判断规则：

| 条件                                    | 倍数含义           |
| --------------------------------------- | ------------------ |
| USDC 增长、收益率稳定、RLDC margin 扩张 | 基础盘倍数上修     |
| USDC 增长但收益率下滑                   | 看规模是否抵消利率 |
| USDC 放缓且收益率下滑                   | 双杀，倍数下修     |
| RLDC margin 低于 38%                    | 渠道成本风险上升   |

## 期权盘

期权盘来自平台化收入。

Other revenue 是最直接的财务验证指标。
CPN 和 Arc 是业务线索，但必须转化为可重复收入。

| 阶段     | Other revenue share | 估值处理               |
| -------- | ------------------: | ---------------------- |
| 利差股   |                 <5% | 不给明显平台倍数       |
| 弱平台化 |               5-10% | 给早期期权价值         |
| 中平台化 |              10-15% | 开始拆分估值           |
| 强平台化 |                >15% | 可向支付基础设施股切换 |

## 折价项

| 折价项   | 触发条件                     | 估值动作         |
| -------- | ---------------------------- | ---------------- |
| 监管折价 | 收益分享或第三方激励受限     | 下修 RLDC margin |
| 渠道折价 | Coinbase 议价权增强          | 下修长期利润留存 |
| 利率折价 | 储备收益率下行且 USDC 不增长 | 下修收入         |
| 竞争折价 | USDC 市占率连续下降          | 下修增长率       |
| 筹码折价 | 解禁、增发、内部人减持       | 下修短期风险承受 |

## Autoresearch 估值联动

1000 次 autoresearch 中，165 条落到 `valuation/`。

| 估值文件                         | 条数 | 主题                     | 落地规则                           |
| -------------------------------- | ---: | ------------------------ | ---------------------------------- |
| `00-valuation-framework.md`      |  111 | 估值锚、利率敏感度、折价 | 主锚用 RLDC，折价项来自矩阵触发     |
| `01-scenario-model.md`           |   54 | Bull/Base/Bear 切换      | 情景切换必须由多矩阵共振触发        |

估值动作必须从矩阵输出，不从单条新闻输出。

| 矩阵             | 影响估值项               | 估值动作                         |
| ---------------- | ------------------------ | -------------------------------- |
| 财务十项复核     | 基础盘、RLDC 倍数        | 调整收入质量和利润留存           |
| 监管矩阵         | 监管折价、渠道折价       | 重算分销激励和合规成本           |
| 渠道议价矩阵     | 渠道折价、RLDC margin    | 下修长期利润留存                 |
| 稳定币竞争矩阵   | 增长率、长期倍数         | 下修 USDC 增速或相对份额假设     |
| 平台化验证矩阵   | Other revenue multiple   | 决定是否给平台期权或切换主框架   |
| 筹码与事件矩阵   | 短期风险承受             | 只调风控，不替代基本面估值       |

Other revenue multiple 是非储备收入倍数，白话就是市场愿意为平台化收入付多少估值。

## 估值锚

| 指标                               | 用途                         | 局限             |
| ---------------------------------- | ---------------------------- | ---------------- |
| Market cap / USDC circulation      | 判断市场对每美元 USDC 的定价 | 忽略利率和成本   |
| Market cap / RLDC                  | 判断扣分销后的收入定价       | 仍不是利润       |
| EV / Adjusted EBITDA               | 传统盈利估值                 | 受一次性调整影响 |
| Market cap / net reserve economics | 更贴近储备业务价值           | 需要季度手动计算 |
| Other revenue multiple             | 平台化收入估值               | 需要可重复性验证 |

EV 是 enterprise value，白话就是公司市值加净债务后的企业价值。

## 安全边际公式

```text
估值安全边际 =
  净储备经济性增长
  + Other revenue 增长
  - 利率压力
  - 分销成本压力
  - 监管折价
  - 筹码压力
```

安全边际不是精确数字。
它用于判断风险收益是否还站得住。

## 估值切换条件

CRCL 从利差股切换到支付基础设施股，需要同时满足：

1. Other revenue 占比超过 15%。
2. CPN 或 Arc 披露可重复收入贡献。
3. RLDC margin 没有因为平台扩张明显下滑。
4. USDC 市占率稳定或上升。
5. 监管没有限制核心分销激励。

缺一项，只能给期权价值，不能完全切换估值框架。
