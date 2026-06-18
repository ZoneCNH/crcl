# CoinGlass 前台公开接口核查

时间：2026-06-18 09:58 +0800

## 结论

观察：CoinGlass `https://www.coinglass.com/zh/Balance` 页面使用的前台接口已找到，并可从命令行复现获取 USDC 交易所余额数据。它不是 CoinGlass 文档里的 Open API key 接口，而是网页前端调用的 `capi.coinglass.com` 接口，响应数据通过前端公开 JS 包里的 AES + gzip 逻辑解密。

## 接口

基础域名：

```text
https://capi.coinglass.com
```

余额列表：

```text
GET /api/exchange/chain/v3/balance/list?symbol=USDC&t=<ms_timestamp>
```

历史余额：

```text
GET /api/exchange/chain/v3/balance?symbol=USDC&exName=all&t=<ms_timestamp>
```

历史余额抽样版本：

```text
GET /api/exchange/chain/v3/balance?symbol=USDC&exName=all&size=300&resolution=7&t=<ms_timestamp>
```

必要请求头：

```text
Accept: application/json
language: zh
encryption: true
cache-ts-v2: 1
Referer: https://www.coinglass.com/zh/Balance
User-Agent: Mozilla/5.0
```

## 前端证据

页面包：

```text
https://s3.coinglass.com/v1/cg/_next/static/chunks/pages/Balance-8d37c3b6823d7951.js
```

该页面调用：

```text
_Po({ symbol }) -> /api/exchange/chain/v3/balance/list
QNf(params) -> /api/exchange/chain/v3/balance
```

请求层包里解出的 baseURL：

```text
Kt = https://capi.coinglass.com
```

## 解密方式

响应头包含：

```text
encryption: true
v: 55 | 66 | 77
user: <encrypted-session-key>
```

前端逻辑：

1. 根据 `v` 选择种子：
   - `55 -> 170b070da9654622`
   - `66 -> d6537d845a964081`
   - `77 -> 863f08689c97435b`
2. `key0 = btoa(seed).slice(0, 16)`
3. 用 AES-128-ECB + PKCS7 解密响应头 `user`，再 gzip inflate，得到 16 字节 `key1`
4. 用 `key1` 解密 JSON 里的 `data`，再 gzip inflate，得到明文 JSON

## 实测明文样例

余额列表接口返回明文数组，2026-06-18 09:58 +0800 实测前两行：

```json
[
  {
    "exchangeName": "Binance",
    "symbol": "USDC",
    "balance": 6029734027.408648,
    "balanceChange": 74699839.066147,
    "d7BalanceChange": 292441533.851022,
    "d30BalanceChange": -672807083.888629
  },
  {
    "exchangeName": "Coinbase Pro",
    "symbol": "USDC",
    "balance": 4061366073.419404,
    "balanceChange": -1858279.908968,
    "d7BalanceChange": -86941142.860774,
    "d30BalanceChange": -1433938679.686311
  }
]
```

历史接口返回对象，包含 `dataMap`，按交易所映射到余额时间序列。

## Data Quality

不阻断：网页前台接口可拿到 CoinGlass USDC exchange balance 数据。

注意：该接口是前端公开接口，不是官方文档 Open API；字段、加密方式、限频或域名可能随网页发布变动。生产采集应保留失败降级和结构校验。

## Missing Info

- 是否允许长期使用前台接口，需要再核 CoinGlass 服务条款和采集频率限制。
- 需要把明文字段映射到本项目 `P1_EXCHANGE_USDC_BALANCES` 的标准 schema。

## 下一步

建议把现有 CoinGlass Open API key 路径保留为优先路径，同时新增 front-end fallback：

1. 请求 `capi.coinglass.com/api/exchange/chain/v3/balance/list`
2. 按响应头 `v/user/encryption` 解密
3. 校验至少包含 Binance、Coinbase Pro、OKX、Bybit 等关键交易所
4. 写入同一数据源健康检查，标注来源为 `coinglass_frontend_balance`
