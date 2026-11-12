# HTTP 服务模式

除了作为 Rust 库调用，本仓库还提供一个可选的 Axum HTTP 服务，接口路径与 Node.js 版保持一致。

该功能由 `server` feature 控制，默认不编译。

## 启动

```bash
# 直接运行，默认监听 0.0.0.0:3000
cargo run --features server --bin ncm-server

# 或先编译再运行
cargo build --release --features server --bin ncm-server
./target/release/ncm-server
```

## 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `NCM_HOST` | 监听地址 | `0.0.0.0` |
| `NCM_PORT` | 监听端口 | `3000` |
| `CORS_ALLOW_ORIGIN` | 允许的 Origin，多个用逗号分隔 | `*` |
| `ENABLE_RANDOM_CN_IP` | 为 `true` 时默认使用随机中国 IP | 关闭 |

```bash
NCM_HOST=127.0.0.1 \
NCM_PORT=8080 \
CORS_ALLOW_ORIGIN=http://localhost:5173 \
cargo run --features server --bin ncm-server
```

```bash
CORS_ALLOW_ORIGIN=https://a.com,https://b.com \
cargo run --features server --bin ncm-server
```

## 请求参数

除业务参数外，HTTP 请求还支持以下通用参数：

| 参数 | 说明 |
|------|------|
| `cookie` | 请求使用的 Cookie |
| `realIP` | 伪装客户端 IP |
| `randomCNIP` | 是否使用随机中国 IP，传 `false` 可关闭 |
| `proxy` | HTTP / SOCKS5 代理 |
| `ua` | 覆盖 User-Agent |
| `domain` | 覆盖请求域名 |
| `e_r` | 是否解密返回体 |
| `crypto` | 覆盖加密通道 |
| `timeout` | 请求超时，单位毫秒 |
| `headers` | 额外请求头，JSON 对象字符串 |

## 前端调用

方法名中的下划线默认转换为斜杠，例如 `song_detail` 对应 `/song/detail`。

```js
import axios from 'axios'

const baseURL = 'http://127.0.0.1:3000'

// 搜索歌曲
const search = await axios.get(`${baseURL}/cloudsearch`, {
  params: { keywords: '海阔天空', type: 1, limit: 10 },
})

// 歌曲详情
const detail = await axios.get(`${baseURL}/song/detail`, {
  params: { ids: '347230' },
})

// 手机号登录
const login = await axios.post(`${baseURL}/login/cellphone`, {
  phone: '138xxxx8000',
  password: 'xxx',
})

// 需要登录的接口可以显式传 cookie
const playlists = await axios.get(`${baseURL}/user/playlist`, {
  params: { uid: '32953014', cookie: 'MUSIC_U=xxx' },
})
```

## 路由映射

| 方法名 | HTTP 路由 | 说明 |
|--------|-----------|------|
| `song_detail` | `/song/detail` | 下划线转斜杠 |
| `login_cellphone` | `/login/cellphone` | 下划线转斜杠 |
| `daily_signin` | `/daily_signin` | 特殊路由，保留下划线 |
| `fm_trash` | `/fm_trash` | 特殊路由，保留下划线 |
| `personal_fm` | `/personal_fm` | 特殊路由，保留下划线 |
| `avatar_upload` | `/avatar/upload` | POST multipart/form-data |
| `voice_upload` | `/voice/upload` | POST multipart/form-data |

所有路由同时支持 GET 和 POST；上传接口仅支持 POST。

## 集成到现有 Axum 项目

```rust
use ncm_api_rs::{
    create_client,
    server::{build_app, build_app_with_config, ServerConfig},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 默认配置
    let app = build_app(create_client(None));

    // 或自定义配置
    let config = ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        cors_origin: Some("http://localhost:5173".to_string()),
        rate_limit: 60,
        rate_limit_window: 60,
    };
    let app = build_app_with_config(create_client(None), &config);

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 8080)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

`ServerConfig` 字段：

| 字段 | 说明 | 默认值 |
|------|------|--------|
| `host` | 监听地址 | `0.0.0.0` |
| `port` | 监听端口 | `3000` |
| `cors_origin` | 允许的 Origin，`None` 表示允许所有 | `None` |
| `rate_limit` | 时间窗口内最大请求数，`0` 表示不限流 | `0` |
| `rate_limit_window` | 限流时间窗口，单位秒 | `60` |

新增 API 时会由 `build.rs` 在编译期自动注册路由，具体流程见 [CONTRIBUTING.md](../CONTRIBUTING.md)。
