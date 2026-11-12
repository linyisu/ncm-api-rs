# 迁移说明

本文档记录与上游 `api-enhanced` 同步过程中产生的不兼容变更，以及从旧版本升级时需要注意的行为变化。

当前对应版本：同步 `api-enhanced` 至 2026-10（PR #4），并继续同步至 `api-enhanced` v4.41.1（2026-11-06）。同步状态见 [UPSTREAM_SYNC.md](UPSTREAM_SYNC.md)。

---

## 2026-11 继续同步

同步 `api-enhanced` v4.41.1 中 v9.6.13 客户端新增的 7 个接口：

- `banner_v3`
- `comment_default_text`
- `comment_hotwall_square_info`
- `event_search`
- `search_lyric`
- `search_song_list_page`
- `search_suggest_mobile`
- `ad_listening_rights_gain`（补齐此前未同步的看广告领取权益接口）

同时同步了上游的 cookie 读取逻辑：当 `query.cookie` 为空时，会尝试从 `query.headers` 中读取 `Cookie` 请求头。该行为不影响现有调用方式。

---

## 加密与请求层

### 新增 `xeapi` / `neapi` 加密通道

- `CryptoType` 新增 `Xeapi`、`Neapi` 两个变体。
- 部分接口默认通道发生变化：`song_url_v1`、`song_url_v1_302`、`song_download_url_v1`、评论、红心、签到等接口按上游切换为 `xeapi` 或 `neapi`。
- `eapi` 默认域名调整为 `interfacepc.music.163.com`。
- 新增 `osx` 系统配置，User-Agent 与设备参数同步上游。

一般情况下调用方不需要修改代码。需要强制指定通道时，可以给 `Query` 传 `crypto`：

```rust
let query = Query::new()
    .param("id", "1969519579")
    .param("crypto", "neapi");
```

### `Query` 新增字段

```rust
pub struct Query {
    // ...
    pub headers: HashMap<String, String>,
    pub timeout: Option<u64>, // 毫秒
}
```

- `headers`：附加请求头。内部计算的同名头（如 `Cookie`、`User-Agent`）会覆盖这里设置的值。
- `timeout`：单次请求超时，单位毫秒；`None` 或 `0` 表示不设置。
- `Query` 同时补全了 `ua`、`domain`、`e_r` 等请求选项，可通过对应字段或 `param()` 设置。

### `RequestOption.check_token` 类型变更

`check_token` 从 `bool` 改为 `CheckToken` 枚举：

```rust
pub enum CheckToken {
    #[default]
    None,
    /// 静态易盾 token
    Static,
    /// 易盾 v3 实时 token
    V3,
}
```

旧代码：

```rust
let option = RequestOption {
    // ...
    check_token: true,
    ..Default::default()
};
```

新代码：

```rust
let option = RequestOption {
    // ...
    check_token: CheckToken::Static, // 或 CheckToken::V3
    ..Default::default()
};
```

### 连接复用与重试

`interface*.music.163.com` 会主动关闭空闲连接，并可能重置复用中的连接。请求层现在：

- 空闲连接 4 秒后主动丢弃；
- 复用连接在未收到任何响应即失败时，换新连接重试一次；
- 超时、读取响应体或解码失败不重试，避免重复执行写操作。

这是内部行为变化，调用方式不变。

---

## 错误处理

上游返回非 200 时，`request()` 现在统一返回：

```rust
NcmError::Response {
    msg: String,
    response: Box<ApiResponse>,
}
```

不再直接返回 `AuthRequired` / `Api` / `RateLimited` 等变体。可以通过新增的辅助方法判断：

```rust
match client.some_api(&query).await {
    Err(NcmError::Response { response, .. }) => {
        let status = response.status;
        let body = &response.body;
        let cookies = &response.cookie;
        // HTTP 服务会把原始 body / cookie / status 原样返回给客户端
    }
    Err(err) => {
        if err.is_auth_required() {
            // 需要登录（status 301）
        }
        if let Some(status) = err.status_code() {
            // 301 / 400 / 503 / 504 等
        }
        if let Some(response) = err.response() {
            // 取原始响应
        }
    }
    Ok(result) => { /* ... */ }
}
```

`NcmError` 新增方法：

- `status_code() -> Option<i64>`
- `is_auth_required() -> bool`
- `response() -> Option<&ApiResponse>`

---

## 接口参数变更

### `comment_reply`

旧参数为 `threadId` / `commentId`，现在按上游改为 `id` / `type` / `cid`：

```rust
let query = Query::new()
    .param("id", "347230")
    .param("type", "0")
    .param("cid", "123456789")
    .param("content", "回复内容");
let result = client.comment_reply(&query).await?;
```

### `comment_delete`

现在使用 `id` / `type` / `cid`，其中 `type` 缺省为 `0`（歌曲），`cid` 兼容旧参数名 `commentId`。

### `voice_upload`

新增封面图片参数 `cover_img`，对应 Node.js 版的 `imgFile`。函数签名变为：

```rust
pub async fn voice_upload(
    &self,
    query: &Query,
    file_name: &str,
    file_data: Vec<u8>,
    file_mimetype: Option<&str>,
    cover_img: Option<Vec<u8>>,
) -> Result<ApiResponse>
```

不传封面时传 `None` 即可。封面文件名和 MIME 类型可通过 `img_name` / `img_mimetype` 参数指定。

### 二维码登录

- `login_qr_key` 返回结构改为与 Node.js 版一致：`{ data: { unikey }, code }`。原来直接读取 `body.unikey` 的代码需要改为 `body.data.unikey`。
- `login_qr_create` 支持 `platform=web`（会附加 `chainId`，供网易云 App 扫码）和 `qrimg=true`（返回 SVG data URL）。
- `login_qr_check` 在授权成功（803）时会把 set-cookie 拼接后放入 `body.cookie`。

### 登录与游客注册

- `login_cellphone`：有 `captcha` 时不再额外发送 `password`；返回 body 中补充 `cookie`，并将 `avatarImgId_str` 改名为 `avatarImgIdStr`。
- `register_anonimous`：使用客户端当前设备 ID 注册，成功时在 body 中返回拼接后的 `cookie`。

### `playlist_tracks`

改为走 `weapi`，并在上游返回 512 时按 Node.js 版逻辑将 `trackIds` 重复一遍后重试一次。

### `cloud_import`

现在会先调用云盘检查接口获取 `songId`，再执行导入。调用参数不变，`id` 仍可作为检查阶段的歌曲 ID（默认 `-2`）。

---

## 返回值变化

- `vip_sign` 返回体包含 `taskSign`、`checkinDetail`、`signed`、`message`，以两个上游接口都返回 `code=200` 作为打卡成功。
- `register_neapikey` 返回 `version` / `updated` / `circleTime`，不返回密钥本身。
- `register_xeapikey` 返回最新公钥及当前 `deviceId`。
- `user_event_all` 自动翻页，返回 `events`、`retrievedCount`、`unavailableCount`、`sizeMismatch`、`pageCount` 等字段。

---

## HTTP 服务

- 新增参数透传：`randomCNIP`、`ua`、`domain`、`e_r`、`crypto`、`timeout`、`headers`。
- 透传客户端 IP：优先 `realIP` 参数，其次 `X-Forwarded-For` / 连接地址；本机请求使用进程级随机中国 IP，避免服务器出口 IP 触发登录风控。
- 服务启动时自动注册游客身份，未登录请求携带 `MUSIC_A`。
- 上游出错时原样返回上游 body / cookie / 状态码，与 Node.js 版一致。

---

## 尚未同步

以下接口依赖 Node.js 运行时能力，暂未同步：

- `register_checktoken_v2`：依赖 jsdom 运行易盾 SDK，当前以静态 token 兜底。
