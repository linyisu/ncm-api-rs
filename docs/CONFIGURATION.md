# 请求配置

本文说明 `Query` 的常用字段、Cookie、代理、IP 伪装、请求头和错误处理。完整的接口参数见 [API.md](API.md)。

## Query 常用字段

所有接口统一通过 `Query` 传参：

```rust
use ncm_api_rs::Query;

let query = Query::new()
    .param("id", "1969519579")
    .param("level", "exhigh");
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `params` | `HashMap<String, String>` | 业务参数，通常用 `.param(k, v)` 添加 |
| `cookie` | `Option<String>` | 本次请求使用的 Cookie，优先级高于客户端 Cookie |
| `proxy` | `Option<String>` | HTTP / SOCKS5 代理地址 |
| `real_ip` | `Option<String>` | 通过 `X-Real-IP` / `X-Forwarded-For` 伪装 IP |
| `random_cn_ip` | `bool` | 使用进程级随机中国 IP |
| `ua` | `Option<String>` | 覆盖 User-Agent |
| `domain` | `Option<String>` | 覆盖请求域名 |
| `e_r` | `Option<bool>` | `weapi` / `eapi` 返回体解密 |
| `headers` | `HashMap<String, String>` | 额外请求头，内部同名头会覆盖这里设置的值 |
| `timeout` | `Option<u64>` | 单次请求超时，单位毫秒 |

设置非链式字段时直接赋值：

```rust
let mut query = Query::new().param("id", "1969519579");
query.timeout = Some(10_000);
query.headers.insert("X-Test".to_string(), "1".to_string());
```

## Cookie

### 创建客户端时传入

```rust
use ncm_api_rs::create_client;

let client = create_client(Some("MUSIC_U=xxx; __csrf=xxx".to_string()));
```

### 单次请求传入

```rust
let query = Query::new().cookie("MUSIC_U=xxx; __csrf=xxx");
let result = client.user_account(&query).await?;
```

`Query::cookie()` 的优先级高于客户端 Cookie，适合多账号切换或临时使用某个账号。

### 登录后保存 Cookie

```rust
use ncm_api_rs::{create_client, Query};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client(None);

    let query = Query::new()
        .param("phone", "13xxx")
        .param("password", "xxx");
    let login = client.login_cellphone(&query).await?;

    let cookies = login.cookie.join("; ");

    let query = Query::new().cookie(&cookies);
    let account = client.user_account(&query).await?;
    println!("{}", account.body);
    Ok(())
}
```

部分接口还要求在 Cookie 中带上 `__csrf`，直接使用登录接口返回的完整 Cookie 最稳妥。

## realIP

在海外服务器或部分云服务上，网易可能返回 `460 cheating` 或 `-462` 风控错误。此时可以传入国内 IP：

```rust
let mut query = Query::new().param("id", "1969519579");
query.real_ip = Some("116.25.146.177".to_string());
let result = client.song_url_v1(&query).await?;
```

HTTP 服务中对应参数名为 `realIP`。

## randomCNIP

不手动指定 IP 时，可以让 SDK 使用进程启动时生成的随机中国 IP：

```rust
let mut query = Query::new().param("id", "1969519579");
query.random_cn_ip = true;
let result = client.song_url_v1(&query).await?;
```

设置环境变量后默认开启：

```bash
export ENABLE_RANDOM_CN_IP=true
```

显式传 `randomCNIP=false` 可以关闭：

```rust
let query = Query::new()
    .param("id", "1969519579")
    .param("randomCNIP", "false");
```

HTTP 服务中对应参数名为 `randomCNIP`。

## 代理

```rust
let mut query = Query::new().param("id", "33894312");
query.proxy = Some("http://127.0.0.1:7890".to_string());
// 也支持 SOCKS5
// query.proxy = Some("socks5://127.0.0.1:1080".to_string());
let result = client.song_url(&query).await?;
```

## 请求头与超时

```rust
let mut query = Query::new().param("id", "1969519579");
query.headers.insert("X-Request-Id".to_string(), "demo".to_string());
query.timeout = Some(15_000);
let result = client.song_detail(&query).await?;
```

内部会计算 `Cookie`、`User-Agent` 等请求头；同名时以内部值为准。

## 返回体解密

`weapi` / `eapi` 接口可以要求返回解密后的 JSON：

```rust
let mut query = Query::new().param("id", "1969519579");
query.e_r = Some(true);
let result = client.song_url_v1(&query).await?;
```

## 指定加密方式

个别接口支持覆盖默认加密通道：

```rust
let query = Query::new()
    .param("id", "1969519579")
    .param("crypto", "neapi");
```

可选值：`weapi`、`eapi`、`linuxapi`、`api`、`xeapi`、`neapi`。一般不需要手动设置。

## 错误处理

非 200 响应会返回 `NcmError::Response`，其中保留上游的 status、body 和 cookie：

```rust
use ncm_api_rs::NcmError;

match client.song_detail(&query).await {
    Ok(result) => println!("{}", result.body),
    Err(NcmError::Response { response, .. }) => {
        eprintln!("status: {}", response.status);
        eprintln!("body: {}", response.body);
        eprintln!("cookie: {:?}", response.cookie);
    }
    Err(err) => {
        if err.is_auth_required() {
            eprintln!("需要登录");
        }
        eprintln!("请求失败: {err}");
    }
}
```

常用辅助方法：

- `err.status_code() -> Option<i64>`
- `err.is_auth_required() -> bool`
- `err.response() -> Option<&ApiResponse>`

## 常见错误

| 状态码 | 常见原因 | 处理方式 |
|--------|----------|----------|
| `301` | 未登录或 Cookie 失效 | 重新登录，或检查 `MUSIC_U` / `__csrf` |
| `400` | 参数缺失或格式错误 | 检查接口参数和资源 id |
| `460` | 地区/IP 风控 | 设置 `real_ip` 或 `random_cn_ip` |
| `503` | 请求过于频繁或被限流 | 降低频率，必要时使用代理 |
| `-462` | 登录/注册风控 | 透传客户端 IP，避免使用服务器出口 IP |

登录接口不要频繁调用；登录状态仍然有效时不要重复登录。
