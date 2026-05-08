# 平台化期权：CPN、Arc 与非储备收入

## 结论

CPN 和 Arc 是 CRCL 的长期估值上限，不是当前利润底盘。

平台化成立必须同时满足三项：

1. Other revenue 持续增长。
2. CPN / Arc 使用量能转化为收费收入。
3. 非储备收入增长不靠一次性 integration services。

integration services 是集成服务收入，白话就是帮客户接入系统收的钱，不一定能重复发生。

## CPN

CPN 是 Circle Payments Network，白话就是 Circle 搭建的支付网络。

截至 2026-02-20，Circle 披露：

- 55 家金融机构已加入。
- 74 家机构处于资格审核。
- 年化 TPV 为 57 亿美元。

TPV 是 Total Payment Volume，白话就是网络上跑过的支付交易额。

研究判断：

| 阶段     | 证据                         | 结论           |
| -------- | ---------------------------- | -------------- |
| 早期采用 | 参与机构增加                 | 证明需求存在   |
| 使用验证 | TPV 连续增长                 | 证明网络被使用 |
| 商业验证 | Other revenue 同步增长       | 证明能收费     |
| 估值验证 | 收入留存、毛利、客户扩张披露 | 支撑平台股倍数 |

## Arc

Arc 是 Circle 自研 Layer-1 公链，白话就是 Circle 自己控制结算体验、合规特性和费用模型的底层网络。

截至 2026-02-20，Circle 披露：

- Arc 公测有 100+ 参与方。
- 近 30 日日均 230 万笔测试交易。
- 累计交易超过 1.66 亿笔。
- 主网上线目标仍在 2026 年。

Layer-1 是底层公链，白话就是独立的基础链。

Arc 不能只看测试交易量。
测试网交易不等于真实收入。

## 平台化验证阈值

| 等级   | 条件                                             | 估值含义                     |
| ------ | ------------------------------------------------ | ---------------------------- |
| 弱验证 | Other revenue 达到 FY2026 指引 1.5-1.7 亿美元    | 平台化有早期证据             |
| 中验证 | Other revenue 占比超过 10%，且连续两个季度增长   | 市场开始给平台期权定价       |
| 强验证 | Other revenue 占比超过 15%，CPN/Arc 披露收入贡献 | 从利差股向支付基础设施股切换 |
| 失败   | TPV 或测试交易增长但收入不增长                   | 使用量没有商业化             |

## Autoresearch 平台化落地矩阵

平台化 160 次微实验落成一个规则：CPN、Arc 和合规能力只有转成可重复收入，才进入估值切换。

### BSA/AML 对 CPN 的约束

| 复核项                 | 复核来源                         | 动作规则                         |
| ---------------------- | -------------------------------- | -------------------------------- |
| KYC burden             | Treasury、FinCEN、客户摩擦       | 上升时下调 CPN 接入效率          |
| transaction monitoring | Treasury、FinCEN、合规系统成本   | 上升时上调运营成本               |
| sanctions screening    | Treasury、OFAC、钱包合规         | 限制钱包和跨境场景时降级         |
| wallet compliance      | OFAC、合规系统成本、合作方披露   | 合作方成本升高时降低网络扩张速度 |
| cross-border payment   | Treasury、FinCEN、客户摩擦       | 摩擦升高时下修跨境支付增速       |
| FinCEN reporting       | FinCEN、合规系统成本             | 披露义务增加时上调费用假设       |
| OFAC list handling     | OFAC、钱包筛查                   | 制裁名单处理变严时标记 P1        |
| suspicious activity    | FinCEN、交易监测                 | 数据不足时列入 missing_info      |
| privacy tradeoff       | Treasury、客户摩擦               | 隐私体验恶化时降低客户质量评分   |
| partner onboarding     | Treasury、FinCEN、合作方资格审核 | 审核周期拉长时降低 TPV 斜率      |

KYC 是 Know Your Customer，白话就是确认客户身份。

### CPN 商业化复核

| 复核项                | 必查证据                         | 保留条件                         |
| --------------------- | -------------------------------- | -------------------------------- |
| CPN institutions      | Circle 披露、财报问答            | 机构增长不是唯一证据             |
| CPN TPV               | TPV 趋势、Other revenue          | TPV 与收入同步增长               |
| partner qualification | 资格审核数量、客户质量           | 通过审核且产生交易               |
| payment corridor      | 走廊数量、交易质量               | 新走廊带来可复现交易             |
| enterprise onboarding | 企业客户、收入确认               | 客户愿意为 API 和结算付费        |
| settlement speed      | 客户使用、费用模型               | 速度优势能转化为收费             |
| fee model             | 财报问答、收入拆分               | 收费方式清楚且可重复             |
| revenue recognition   | SEC filing、Other revenue        | 不是一次性 integration services  |
| compliance friction   | BSA/AML 成本、客户摩擦           | 合规摩擦不压垮网络效益           |
| customer concentration | 客户质量、收入集中度             | 单一客户不决定增长叙事           |

### Arc 基础设施复核

| 复核项                 | 必查证据                         | 动作规则                         |
| ---------------------- | -------------------------------- | -------------------------------- |
| Arc testnet tx         | Circle docs、主网公告            | 测试网交易不计入收入验证         |
| Arc mainnet timing     | 主网公告、管理层时间表           | 延期则延后估值切换               |
| validator model        | Circle docs、真实客户            | 模型不清楚时不给独立倍数         |
| gas model              | 费用模型、收入贡献               | gas 不能变现时只算技术期权       |
| developer adoption     | 开发者采用、真实客户             | 没有生产客户时保持观察           |
| enterprise pilot       | 企业试点、收入贡献               | 试点必须转成付费或结算量         |
| USDC gas usage         | 链上使用、费用模型               | 使用增长要能解释 USDC 需求       |
| fee capture            | 收入贡献、财报拆分               | 收费归属不清时不提高倍数         |
| Arc compliance feature | 合规特性、客户采用               | 必须带来客户或收入改善           |
| mainnet delay          | 主网公告、管理层解释             | 延期进入 P2 风险复盘             |

## 需要拆分的收入

Other revenue 必须拆出四类：

1. CPN 支付网络收入。
2. Arc 交易、验证、开发者或结算相关收入。
3. 企业 API 与账户服务收入。
4. 一次性 integration services。

如果公司不披露细分，研究结论按保守口径处理：

```text
Other revenue 增长 = 平台化线索
Other revenue 可重复性未披露 = 不上调长期倍数
```

## 平台化风险

| 风险           | 触发信号                        | 动作           |
| -------------- | ------------------------------- | -------------- |
| 采用不等于收入 | TPV 增长但 Other revenue 不增长 | 降低平台化权重 |
| Arc 主网延期   | 2026 年主网节奏后移             | 延后估值切换   |
| 费用模型不清   | 不披露 Arc 如何收费             | 不给独立倍数   |
| 企业客户不披露 | 只披露伙伴数量不披露交易质量    | 保持观察       |
| 合规成本过高   | BSA/AML 规则压低跨境支付效率    | 下调 CPN 增速  |

## 平台化结论模板

```text
结论：平台化仍处弱验证阶段。
依据：CPN 与 Arc 有使用数据，但 Other revenue 占比仍低，收入可重复性未充分拆分。
动作：Q1 财报优先追问 CPN TPV、收入确认方式、Arc 主网时间表和收费模型。
```

## 来源

- Circle FY2025 / Q4 2025 results：`https://www.circle.com/en/pressroom/circle-reports-fourth-quarter-and-full-fiscal-year-2025-financial-results`
