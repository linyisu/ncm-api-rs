# 贡献指南

## 环境要求

- Rust 1.89 或更新版本
- 能访问网易云音乐 API 的网络环境（部分集成测试需要联网）

## 构建与测试

```bash
# 检查库代码
cargo check

# 检查 HTTP 服务
cargo check --features server --all-targets

# 格式化
cargo fmt

# 静态检查
cargo clippy --features server --all-targets -- -D warnings

# 单元测试与联网集成测试
cargo test --features server
```

`tests/integration.rs` 会访问真实网易云接口；如果网络不可用或触发风控，失败是预期内的。提交前至少保证：

```bash
cargo fmt --check
cargo clippy --features server --all-targets -- -D warnings
cargo test --features server
```

## Feature

| feature | 默认 | 说明 |
|---------|------|------|
| `server` | 否 | 编译 Axum HTTP 服务和 `ncm-server` 二进制 |

只使用库时不需要开启 `server`。

## 示例

```bash
cargo run --example basic
cargo run --example search_test
```

示例代码在 `examples/` 下。新增常用场景时可以添加示例，而不是只写进文档。

## 项目结构

```text
src/
  api/            # 每个接口一个文件，impl ApiClient
  request.rs      # 请求、加密、Cookie、连接复用
  request/        # transport 等请求辅助
  crypto.rs       # weapi / eapi / linuxapi / xeapi / neapi
  error.rs        # NcmError
  server/         # Axum HTTP 服务，可选 feature
  util/           # cookie、config、IP、device 等
build.rs          # 扫描 src/api/mod.rs，生成 HTTP 路由
docs/             # 文档
examples/         # 可运行示例
tests/            # 单元与集成测试
```

## 新增一个 API

### 1. 创建接口文件

例如 `src/api/your_new_api.rs`：

```rust
use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 接口说明
    /// 对应 /your/endpoint
    pub async fn your_new_api(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "id": query.get_or("id", ""),
        });
        self.request(
            "/api/your/endpoint",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
```

### 2. 在 `src/api/mod.rs` 注册

```rust
mod your_new_api;
```

### 3. 路由

`build.rs` 会在编译期扫描 `src/api/mod.rs` 的模块声明，并自动生成 HTTP 路由。默认规则是把方法名中的 `_` 转成 `/`：

```text
your_new_api -> /your/new/api
```

如果上游路由是特殊形式，在 `build.rs` 的 `EXTRA_ROUTES`（额外路径）或 `SPECIAL_ROUTES`（保留下划线的路径）中补充映射。

### 4. 更新文档

在 `docs/API.md` 中添加接口小节，然后重新生成索引：

```bash
python3 scripts/gen_api_index.py
```

如果是有意为之的不兼容变更，同时更新 `docs/MIGRATION.md`。

### 5. 验证

```bash
cargo fmt
cargo clippy --features server --all-targets -- -D warnings
cargo test --features server
```

## 文档约定

- 分类标题使用 `##`
- 接口标题统一使用 `### method_name - 中文说明`
- 每个接口标题前保留 `<a id="method-method_name"></a>`，供索引链接
- 参数说明统一使用 `**必选参数 :**` / `**可选参数 :**`
- 示例统一使用 `**Rust 调用例子 :**`

`docs/API_INDEX.md` 由脚本生成，不要手工编辑。

## 上游同步

同步流程、当前版本和待同步项见 [docs/UPSTREAM_SYNC.md](docs/UPSTREAM_SYNC.md)。

## 提交前检查

- [ ] 新接口已在 `src/api/mod.rs` 注册
- [ ] 特殊路由已加入 `build.rs` 的 `EXTRA_ROUTES` / `SPECIAL_ROUTES`
- [ ] `docs/API.md` 已更新
- [ ] `docs/API_INDEX.md` 已重新生成
- [ ] `cargo fmt --check` 通过
- [ ] `cargo clippy --features server --all-targets -- -D warnings` 通过
- [ ] 相关测试通过
