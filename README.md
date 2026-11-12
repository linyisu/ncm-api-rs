<div align="center">

# ncm-api-rs

**网易云音乐 API Rust 原生实现**

[![Rust](https://img.shields.io/badge/Rust-1.89+-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-WTFPL-brightgreen.svg)](LICENSE)
[![Tokio](https://img.shields.io/badge/async-tokio-blue?logo=rust)](https://tokio.rs/)

</div>

---

## 简介

本仓库 fork 自 [SPlayer-Dev/ncm-api-rs](https://github.com/SPlayer-Dev/ncm-api-rs)，是 [NeteaseCloudMusicApi Enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced) 的 Rust 原生实现，无需 Node.js 运行时。

项目通过请求签名、Cookie 和伪造请求头调用网易云音乐官方 API，提供与 Node.js 版本一致的接口命名和参数风格。

所有接口统一通过 `Query` 传参：

```rust
let query = Query::new()
    .param("id", "1969519579")
    .param("level", "exhigh");
```

## 特性

- 纯 Rust 实现，无需 Node.js
- 支持 weapi / eapi / linuxapi / xeapi / neapi
- 450+ 接口，覆盖登录、用户、歌曲、歌单、评论、电台、推荐、云盘、VIP 等
- 支持 Cookie、代理、realIP、随机中国 IP、自定义请求头和超时
- 可选 Axum HTTP 服务，接口路径与 Node.js 版一致
- 每个接口独立文件，便于维护和扩展

## 环境要求

- Rust 1.89 或更新版本
- Tokio 异步运行时

## 安装

当前只通过 Git 依赖：

```toml
[dependencies]
ncm-api-rs = { git = "https://github.com/linyisu/ncm-api-rs.git" }
tokio = { version = "1", features = ["full"] }
```

本地开发时也可以使用路径依赖：

```toml
[dependencies]
ncm-api-rs = { path = "../ncm-api-rs" }
```

需要 HTTP 服务时开启 `server` feature：

```toml
[dependencies]
ncm-api-rs = { git = "https://github.com/linyisu/ncm-api-rs.git", features = ["server"] }
```

## 快速开始

```rust
use ncm_api_rs::{create_client, Query};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client(None);

    // 搜索歌曲
    let query = Query::new()
        .param("keywords", "晴天 周杰伦")
        .param("type", "1")
        .param("limit", "3");
    let search = client.cloudsearch(&query).await?;
    println!("{}", search.body);

    // 获取歌曲详情
    let query = Query::new().param("ids", "186016");
    let detail = client.song_detail(&query).await?;
    println!("{}", detail.body);

    // 获取播放链接
    let query = Query::new()
        .param("id", "186016")
        .param("level", "standard");
    let url = client.song_url_v1(&query).await?;
    println!("{}", url.body);

    Ok(())
}
```

更多示例：

```bash
cargo run --example basic
cargo run --example search_test
```

## 常用配置

### 带 Cookie

```rust
let client = create_client(Some("MUSIC_U=xxx; __csrf=xxx".to_string()));

// 或只对单次请求生效
let query = Query::new().cookie("MUSIC_U=xxx; __csrf=xxx");
let account = client.user_account(&query).await?;
```

### 代理与 IP

```rust
let mut query = Query::new().param("id", "1969519579");
query.proxy = Some("socks5://127.0.0.1:1080".to_string());
query.real_ip = Some("116.25.146.177".to_string());
query.random_cn_ip = true;
```

Cookie 优先级、realIP、随机中国 IP、请求头、超时、返回体解密和错误处理的完整说明见 [docs/CONFIGURATION.md](docs/CONFIGURATION.md)。

## 文档导航

| 文档 | 内容 |
|------|------|
| [docs/API.md](docs/API.md) | 完整接口参数与调用示例 |
| [docs/API_INDEX.md](docs/API_INDEX.md) | 按方法名查找接口 |
| [docs/CONFIGURATION.md](docs/CONFIGURATION.md) | Cookie、代理、IP、超时、错误处理 |
| [docs/HTTP_SERVER.md](docs/HTTP_SERVER.md) | HTTP 服务模式 |
| [docs/MIGRATION.md](docs/MIGRATION.md) | 不兼容变更与升级说明 |
| [docs/UPSTREAM_SYNC.md](docs/UPSTREAM_SYNC.md) | 上游同步状态与记录 |
| [CONTRIBUTING.md](CONTRIBUTING.md) | 本地开发、测试、新增接口 |

## 常用接口

| 方法 | 说明 |
|------|------|
| `search` / `cloudsearch` | 搜索 |
| `song_detail` | 歌曲详情 |
| `song_url_v1` | 获取播放链接 |
| `lyric` | 获取歌词 |
| `login_cellphone` | 手机号登录 |
| `login_qr_create` / `login_qr_check` | 二维码登录 |
| `user_account` | 当前账号信息 |
| `user_playlist` | 用户歌单 |
| `playlist_detail` | 歌单详情 |
| `album` | 专辑详情 |
| `artist_detail` | 歌手详情 |
| `recommend_songs` | 每日推荐 |

完整列表见 [docs/API_INDEX.md](docs/API_INDEX.md)。

## HTTP 服务

```bash
cargo run --features server --bin ncm-server
```

默认监听 `0.0.0.0:3000`。环境变量、路由规则、前端调用示例和 Axum 集成方式见 [docs/HTTP_SERVER.md](docs/HTTP_SERVER.md)。

## 开发

```bash
cargo fmt
cargo clippy --features server --all-targets -- -D warnings
cargo test --features server
```

新增接口、路由注册、文档生成和测试要求见 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 上游同步

当前同步 `api-enhanced` `v4.41.1`（2026-11-06）。

- 已实现上游 `module/` 下的全部接口
- 仅 `register_checktoken_v2` 未实现，因为它依赖 jsdom 运行易盾 SDK
- 同步历史和新增接口清单见 [docs/UPSTREAM_SYNC.md](docs/UPSTREAM_SYNC.md)

## 注意事项

- 本项目仅供学习使用，请尊重版权，不要用于商业或破坏版权行为
- 不要频繁调用登录接口，避免触发风控
- 国外服务器或部分云服务可能返回 `460` / `-462`，可配置 `real_ip` 或 `random_cn_ip`
- 返回 301 通常表示未登录或 Cookie 失效

## 致谢

- [SPlayer-Dev/ncm-api-rs](https://github.com/SPlayer-Dev/ncm-api-rs) - 本仓库的上游 fork 来源
- [NeteaseCloudMusicApi Enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced) - Node.js 实现
- [Binaryify/NeteaseCloudMusicApi](https://github.com/Binaryify/NeteaseCloudMusicApi) - 原始项目

## License

[WTFPL](LICENSE)
