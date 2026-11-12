# NCM API Rust SDK 接口文档

> 本文档对应 [NeteaseCloudMusicApi Enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced) 的 Rust SDK 版本，涵盖所有已实现的接口。
>
> 所有接口方法均挂载在 `ApiClient` 上，通过 `Query` 对象传参。使用前请先阅读 [README](../README.md) 了解基本用法。
>
> 不兼容变更与升级注意事项请参阅 [MIGRATION.md](MIGRATION.md)。

---

## 目录

- [登录相关](#登录相关)
- [用户相关](#用户相关)
- [歌曲相关](#歌曲相关)
- [搜索相关](#搜索相关)
- [歌单相关](#歌单相关)
- [评论相关](#评论相关)
- [歌手相关](#歌手相关)
- [专辑相关](#专辑相关)
- [MV 相关](#mv-相关)
- [视频相关](#视频相关)
- [电台相关](#电台相关)
- [推荐相关](#推荐相关)
- [排行榜相关](#排行榜相关)
- [云盘相关](#云盘相关)
- [私信相关](#私信相关)
- [VIP/会员](#vip会员)
- [云贝](#云贝)
- [听歌足迹](#听歌足迹)
- [风格/曲风](#风格曲风)
- [数字专辑](#数字专辑)
- [声音/播客](#声音播客)
- [助眠解压](#助眠解压)
- [云小编](#云小编)
- [音乐人](#音乐人)
- [粉丝中心](#粉丝中心)
- [UGC 百科](#ugc-百科)
- [一起听](#一起听)
- [广播电台](#广播电台)
- [其他](#其他)

---

## 登录相关

---

#### 邮箱登录

说明 : 登录有三个接口，不要频繁调用登录接口，不然可能会被风控，登录状态还存在就不要重复调用登录接口

**必选参数 :** `email` : 163 网易邮箱 ; `password` : 密码

**可选参数 :** `md5_password` : md5 加密后的密码，传入后 `password` 将失效

**方法名 :** `login`

**调用例子 :**
```rust
let query = Query::new()
    .param("email", "xxx@163.com")
    .param("password", "yyy");
let result = client.login(&query).await?;
```

---

#### 手机登录

说明 : 手机号码登录

**必选参数 :** `phone` : 手机号码

**可选参数 :**

- `password` : 密码
- `countrycode` : 国家码，用于国外手机号登录，例如美国传入 `1`
- `md5_password` : md5 加密后的密码，传入后 `password` 参数将失效
- `captcha` : 验证码，使用 `captcha_sent` 接口传入手机号获取验证码，调用此接口传入验证码可使用验证码登录，传入后 `password` 参数将失效
- `sca` : 安全验证码，风控要求时传入

登录成功时，返回 body 中会补充拼接后的 `cookie` 字段，并包含 `avatarImgIdStr`。

**方法名 :** `login_cellphone`

**调用例子 :**
```rust
let query = Query::new()
    .param("phone", "13xxx")
    .param("password", "yyy");
let result = client.login_cellphone(&query).await?;

// 或使用验证码登录
let query = Query::new()
    .param("phone", "13xxx")
    .param("captcha", "1234");
let result = client.login_cellphone(&query).await?;
```

---

#### 二维码 key 生成

说明 : 调用此接口可生成一个二维码登录用的 key。返回结构为 `{ data: { unikey }, code }`，`unikey` 位于 `body.data.unikey`（与 Node.js 版一致）。

**方法名 :** `login_qr_key`

**调用例子 :**
```rust
let query = Query::new();
let result = client.login_qr_key(&query).await?;
let unikey = result.body["data"]["unikey"].as_str().unwrap_or_default();
```

---

#### 二维码生成

说明 : 调用此接口传入上一个接口生成的 key 可生成二维码图片的 base64 和二维码信息，可使用 base64 展示图片，或者使用二维码信息内容自行使用第三方二维码生成库渲染二维码

**必选参数 :** `key` : 由 `login_qr_key` 接口生成

**可选参数 :** `qrimg` : 传入后会额外返回二维码图片（SVG data URL）; `platform` : 传 `web` 时会在二维码 URL 中附加 `chainId`，供网易云 App 扫码

**方法名 :** `login_qr_create`

**调用例子 :**
```rust
let query = Query::new()
    .param("key", "xxx")
    .param("qrimg", "true")
    .param("platform", "web");
let result = client.login_qr_create(&query).await?;
```

---

#### 二维码检测扫码状态

说明 : 轮询此接口可获取二维码扫码状态，800 为二维码过期，801 为等待扫码，802 为待确认，803 为授权登录成功（803 状态码下会返回 cookies）。授权成功时，set-cookie 会拼接后放入 `body.cookie`。

**必选参数 :** `key` : 由 `login_qr_key` 接口生成

**方法名 :** `login_qr_check`

**调用例子 :**
```rust
let query = Query::new().param("key", "xxx");
let result = client.login_qr_check(&query).await?;
```

---

#### 游客登录

说明 : 直接调用此接口，可获取游客 cookie，如果遇到其他接口未登录状态报 400 状态码需要验证的错误，可使用此接口获取游客 cookie 避免报错。接口使用客户端当前设备 ID 注册，成功时会在 body 中返回拼接后的 `cookie`。

**方法名 :** `register_anonimous`

**调用例子 :**
```rust
let query = Query::new();
let result = client.register_anonimous(&query).await?;
```

---

#### 刷新登录

说明 : 调用此接口，可刷新登录状态，返回内容包含新的 cookie（不支持刷新二维码登录的 cookie）

**方法名 :** `login_refresh`

**调用例子 :**
```rust
let query = Query::new();
let result = client.login_refresh(&query).await?;
```

---

#### 登录状态

说明 : 调用此接口，可获取登录状态

**方法名 :** `login_status`

**调用例子 :**
```rust
let query = Query::new();
let result = client.login_status(&query).await?;
```

---

#### 退出登录

说明 : 调用此接口，可退出登录

**方法名 :** `logout`

**调用例子 :**
```rust
let query = Query::new();
let result = client.logout(&query).await?;
```

---

#### 发送验证码

说明 : 调用此接口，传入手机号码，可发送验证码

**必选参数 :** `phone` : 手机号码

**可选参数 :** `ctcode` : 国家区号，默认 86 即中国

**方法名 :** `captcha_sent`

**调用例子 :**
```rust
let query = Query::new().param("phone", "13xxx");
let result = client.captcha_sent(&query).await?;
```

---

#### 验证验证码

说明 : 调用此接口，传入手机号码和验证码，可校验验证码是否正确

**必选参数 :** `phone` : 手机号码 ; `captcha` : 验证码

**可选参数 :** `ctcode` : 国家区号，默认 86 即中国

**方法名 :** `captcha_verify`

**调用例子 :**
```rust
let query = Query::new()
    .param("phone", "13xxx")
    .param("captcha", "1597");
let result = client.captcha_verify(&query).await?;
```

---

#### 注册(修改密码)

说明 : 调用此接口，传入手机号码和验证码、密码、昵称，可注册网易云音乐账号（同时可修改密码）

**必选参数 :** `captcha` : 验证码 ; `phone` : 手机号码 ; `password` : 密码 ; `nickname` : 昵称

**可选参数 :** `countrycode` : 国家码，用于国外手机号，例如美国传入 `1`，默认 86 即中国

**方法名 :** `register_cellphone`

**调用例子 :**
```rust
let query = Query::new()
    .param("phone", "13xxx")
    .param("password", "xxxxx")
    .param("captcha", "1234")
    .param("nickname", "binary1345");
let result = client.register_cellphone(&query).await?;
```

---

#### 检测手机号码是否已注册

说明 : 调用此接口，可检测手机号码是否已注册

**必选参数 :** `phone` : 手机号码

**可选参数 :** `countrycode` : 国家码，用于国外手机号，例如美国传入 `1`，默认 86 即中国

**方法名 :** `cellphone_existence_check`

**调用例子 :**
```rust
let query = Query::new().param("phone", "13xxx");
let result = client.cellphone_existence_check(&query).await?;
```

---

#### 初始化昵称

说明 : 刚注册的账号（需登录），调用此接口，可初始化昵称

**必选参数 :** `nickname` : 昵称

**方法名 :** `activate_init_profile`

**调用例子 :**
```rust
let query = Query::new().param("nickname", "testUser2019");
let result = client.activate_init_profile(&query).await?;
```

---

#### 重复昵称检测

说明 : 调用此接口，可检测昵称是否重复，并提供备用昵称

**必选参数 :** `nickname` : 昵称

**方法名 :** `nickname_check`

**调用例子 :**
```rust
let query = Query::new().param("nickname", "binaryify");
let result = client.nickname_check(&query).await?;
```

---

#### 更换绑定手机

说明 : 调用此接口，可更换绑定手机（流程：先发送验证码到原手机号码，再发送验证码到新手机号码然后再调用此接口）

**必选参数 :** `oldcaptcha` : 原手机验证码 ; `captcha` : 新手机验证码 ; `phone` : 手机号码 ; `ctcode` : 国家区号，默认 86 即中国

**方法名 :** `rebind`

**调用例子 :**
```rust
let query = Query::new()
    .param("phone", "xxx")
    .param("oldcaptcha", "1234")
    .param("captcha", "5678");
let result = client.rebind(&query).await?;
```

---

#### 发送安全验证码

说明 : 发送安全验证码（风控验证场景）

**可选参数 :** `ctcode` : 国家码，默认 `86`

**方法名 :** `captcha_safe_sent`

**调用例子 :**
```rust
let query = Query::new().param("ctcode", "86");
let result = client.captcha_safe_sent(&query).await?;
```

---

#### 发送验证码 v1

说明 : 客户端登录场景的手机验证码发送接口

**必选参数 :** `phone` : 手机号

**可选参数 :** `ctcode` : 国家码，默认 `86`

**方法名 :** `captcha_sent_v1`

**调用例子 :**
```rust
let query = Query::new().param("phone", "13xxx");
let result = client.captcha_sent_v1(&query).await?;
```

---

#### 登录设备列表

说明 : 登录后获取当前账号的登录设备列表

**参数 :** 无

**方法名 :** `device_list`

**调用例子 :**
```rust
let query = Query::new();
let result = client.device_list(&query).await?;
```

---

#### 强制下线设备

说明 : 登录后强制下线指定的登录设备

**必选参数 :** `deviceKey` : 设备 key，可从 `device_list` 返回中获取

**可选参数 :** `captcha` : 短信验证码，服务端要求验证时传入

**方法名 :** `device_kickoff`

**调用例子 :**
```rust
let query = Query::new()
    .param("deviceKey", "xxx")
    .param("captcha", "1234");
let result = client.device_kickoff(&query).await?;
```

---

#### 上报设备中心设备名称

说明 : 修改当前设备在「登录设备管理」中显示的名称

**必选参数 :** `deviceName` : 设备名称（兼容旧参数名 `name`）

**方法名 :** `deviceinfo_center_upload`

**调用例子 :**
```rust
let query = Query::new().param("deviceName", "我的电脑");
let result = client.deviceinfo_center_upload(&query).await?;
```

---

#### 获取易盾 v3 token

说明 : 实时获取易盾 v3 反作弊 token（不缓存）

**参数 :** 无

**方法名 :** `register_checktoken_v3`

**调用例子 :**
```rust
let query = Query::new();
let result = client.register_checktoken_v3(&query).await?;
// result.body: { code, token, registered }
```

---

#### 刷新 neapi 密钥配置

说明 : 向服务端重新拉取 neapi 配置并刷新本地缓存，密钥本身不会返回

**参数 :** 无

**方法名 :** `register_neapikey`

**调用例子 :**
```rust
let query = Query::new();
let result = client.register_neapikey(&query).await?;
// result.body: { version, updated, circleTime }
```

---

#### 刷新 xeapi 公钥

说明 : 重新向服务端申请 xeapi 公钥并刷新本地缓存

**参数 :** 无

**方法名 :** `register_xeapikey`

**调用例子 :**
```rust
let query = Query::new();
let result = client.register_xeapikey(&query).await?;
```

---


### 用户相关

---

#### 获取用户详情

说明 : 登录后调用此接口，传入用户 id，可以获取用户详情

**必选参数 :** `uid` : 用户 id

**方法名 :** `user_detail`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_detail(&query).await?;
```

---

#### 获取用户详情（新版）

说明 : 登录后调用此接口，传入用户 id，可以获取用户详情（新版接口，返回更全面的信息）

**必选参数 :** `uid` : 用户 id

**方法名 :** `user_detail_new`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_detail_new(&query).await?;
```

---

#### 获取账号信息

说明 : 登录后调用此接口，可获取用户账号信息

**方法名 :** `user_account`

**调用例子 :**
```rust
let query = Query::new();
let result = client.user_account(&query).await?;
```

---

#### 获取用户信息 , 歌单，收藏，mv, dj 数量

说明 : 登录后调用此接口，可以获取用户信息

**方法名 :** `user_subcount`

**调用例子 :**
```rust
let query = Query::new();
let result = client.user_subcount(&query).await?;
```

---

#### 获取用户等级信息

说明 : 登录后调用此接口，可以获取用户等级信息，包含当前登录天数，听歌次数，下一等级需要的登录天数和听歌次数，当前等级进度

**方法名 :** `user_level`

**调用例子 :**
```rust
let query = Query::new();
let result = client.user_level(&query).await?;
```

---

#### 获取用户绑定信息

说明 : 登录后调用此接口，可以获取用户绑定信息

**必选参数 :** `uid` : 用户 id

**方法名 :** `user_binding`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_binding(&query).await?;
```

---

#### 用户绑定手机

说明 : 登录后调用此接口，可以绑定手机号码

**必选参数 :** `phone` : 手机号码 ; `captcha` : 验证码

**可选参数 :** `password` : 密码 ; `countrycode` : 国家地区代码，默认 86

**方法名 :** `user_bindingcellphone`

**调用例子 :**
```rust
let query = Query::new()
    .param("phone", "13xxx")
    .param("captcha", "1234");
let result = client.user_bindingcellphone(&query).await?;
```

---

#### 用户更换绑定手机

说明 : 登录后调用此接口，可以更换绑定手机

**必选参数 :** `phone` : 手机号码 ; `oldcaptcha` : 原手机号码的验证码 ; `captcha` : 新手机号码的验证码

**可选参数 :** `countrycode` : 国家地区代码，默认 86

**方法名 :** `user_replacephone`

**调用例子 :**
```rust
let query = Query::new()
    .param("phone", "xxx")
    .param("captcha", "1234")
    .param("oldcaptcha", "2345");
let result = client.user_replacephone(&query).await?;
```

---

#### 更新用户信息

说明 : 登录后调用此接口，传入相关信息，可以更新用户信息

**必选参数 :** `gender` : 性别 0:保密 1:男性 2:女性 ; `birthday` : 出生日期，时间戳 unix timestamp ; `nickname` : 用户昵称 ; `province` : 省份 id ; `city` : 城市 id ; `signature` : 用户签名

**方法名 :** `user_update`

**调用例子 :**
```rust
let query = Query::new()
    .param("gender", "0")
    .param("signature", "测试签名")
    .param("city", "440300")
    .param("nickname", "binary")
    .param("birthday", "1525918298004")
    .param("province", "440000");
let result = client.user_update(&query).await?;
```

---

#### 更新头像

说明 : 登录后调用此接口，可更新头像

**可选参数 :** `imgSize` : 图片尺寸，默认为 300 ; `imgX` : 水平裁剪偏移，方形图片可不传，默认为 0 ; `imgY` : 垂直裁剪偏移，方形图片可不传，默认为 0

**方法名 :** `avatar_upload`

**调用例子 :**
```rust
let query = Query::new().param("imgSize", "200");
let result = client.avatar_upload(&query).await?;
```

---

#### 获取用户歌单

说明 : 登录后调用此接口，传入用户 id，可以获取用户歌单

**必选参数 :** `uid` : 用户 id

**可选参数 :** `limit` : 返回数量，默认为 30 ; `offset` : 偏移数量，用于分页，默认为 0

**方法名 :** `user_playlist`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_playlist(&query).await?;
```

---

#### 用户的创建歌单列表

说明 : 调用此接口，传入用户 id，获取用户的创建歌单列表

**必选参数 :** `uid` : 用户 id

**可选参数 :** `limit` : 返回数量，默认为 100 ; `offset` : 偏移数量，用于分页，默认为 0

**方法名 :** `user_playlist_create`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_playlist_create(&query).await?;
```

---

#### 用户的收藏歌单列表

说明 : 调用此接口，传入用户 id，获取用户的收藏歌单列表

**必选参数 :** `uid` : 用户 id

**可选参数 :** `limit` : 返回数量，默认为 100 ; `offset` : 偏移数量，用于分页，默认为 0

**方法名 :** `user_playlist_collect`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_playlist_collect(&query).await?;
```

---

#### 获取用户关注列表

说明 : 登录后调用此接口，传入用户 id，可以获取用户关注列表

**必选参数 :** `uid` : 用户 id

**可选参数 :** `limit` : 返回数量，默认为 30 ; `offset` : 偏移数量，用于分页，默认为 0

**方法名 :** `user_follows`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_follows(&query).await?;
```

---

#### 获取用户粉丝列表

说明 : 登录后调用此接口，传入用户 id，可以获取用户粉丝列表

**必选参数 :** `uid` : 用户 id

**可选参数 :** `limit` : 返回数量，默认为 30 ; `offset` : 偏移数量，用于分页，默认为 0

**方法名 :** `user_followeds`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_followeds(&query).await?;
```

---

#### 当前账号关注的用户/歌手

说明 : 调用此接口，可获得当前账号关注的用户/歌手

**可选参数 :** `size` : 返回数量，默认为 30 ; `cursor` : 返回数据的 cursor，默认为 0，传入上一次返回结果的 cursor 将会返回下一页的数据 ; `scene` : 场景，0 表示所有关注，1 表示关注的歌手，2 表示关注的用户，默认为 0

**方法名 :** `user_follow_mixed`

**调用例子 :**
```rust
let query = Query::new().param("scene", "1");
let result = client.user_follow_mixed(&query).await?;
```

---

#### 用户是否互相关注

说明 : 登录后调用此接口，传入用户 id，可判断用户是否互相关注

**必选参数 :** `uid` : 用户 id

**方法名 :** `user_mutualfollow_get`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_mutualfollow_get(&query).await?;
```

---

#### 获取用户动态

说明 : 登录后调用此接口，传入用户 id，可以获取用户动态

**必选参数 :** `uid` : 用户 id

**可选参数 :** `limit` : 返回数量，默认为 30 ; `lasttime` : 返回数据的 `lasttime`，默认 -1，传入上一次返回结果的 lasttime，将会返回下一页的数据

**方法名 :** `user_event`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_event(&query).await?;
```

---

#### 获取用户播放记录

说明 : 登录后调用此接口，传入用户 id，可获取用户播放记录

**必选参数 :** `uid` : 用户 id

**可选参数 :** `type` : type=1 时只返回 weekData，type=0 时返回 allData

**方法名 :** `user_record`

**调用例子 :**
```rust
let query = Query::new()
    .param("uid", "32953014")
    .param("type", "1");
let result = client.user_record(&query).await?;
```

---

#### 获取用户电台

说明 : 登录后调用此接口，传入用户 id，可以获取用户电台

**必选参数 :** `uid` : 用户 id

**方法名 :** `user_dj`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_dj(&query).await?;
```

---

#### 用户电台

说明 : 调用此接口，传入用户 id 可获取用户创建的电台

**必选参数 :** `uid` : 用户 id

**方法名 :** `user_audio`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_audio(&query).await?;
```

---

#### 获取用户历史评论

说明 : 登录后调用此接口，传入用户 id，可以获取用户历史评论

**必选参数 :** `uid` : 用户 id

**可选参数 :** `limit` : 返回数量，默认为 10 ; `time` : 上一条数据的 time，第一页不需要传，默认为 0

**方法名 :** `user_comment_history`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_comment_history(&query).await?;
```

---

#### 用户徽章

说明 : 调用此接口，传入用户 id，获取用户徽章

**必选参数 :** `uid` : 用户 id

**方法名 :** `user_medal`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_medal(&query).await?;
```

---

#### 用户状态

说明 : 登录后调用此接口，传入用户 id，获取用户社交状态

**必选参数 :** `uid` : 用户 id

**方法名 :** `user_social_status`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.user_social_status(&query).await?;
```

---

#### 用户状态 - 编辑

说明 : 登录后调用此接口，编辑当前用户状态，所需参数可在 `user_social_status_support` 接口获取

**方法名 :** `user_social_status_edit`

**调用例子 :**
```rust
let query = Query::new();
let result = client.user_social_status_edit(&query).await?;
```

---

#### 用户状态 - 相同状态的用户

说明 : 登录后调用此接口，获取相同状态的用户

**方法名 :** `user_social_status_rcmd`

**调用例子 :**
```rust
let query = Query::new();
let result = client.user_social_status_rcmd(&query).await?;
```

---

#### 用户状态 - 支持设置的状态

说明 : 登录后调用此接口，获取支持设置的状态

**方法名 :** `user_social_status_support`

**调用例子 :**
```rust
let query = Query::new();
let result = client.user_social_status_support(&query).await?;
```

---

#### 关注/取消关注用户

说明 : 登录后调用此接口，传入用户 id 和操作 t，可关注/取消关注用户

**必选参数 :** `id` : 用户 id ; `t` : `1` 为关注，其他为取消关注

**方法名 :** `follow`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "32953014")
    .param("t", "1");
let result = client.follow(&query).await?;
```

---

#### 私信和通知接口

说明 : 登录后调用此接口，可获取私信和通知数量信息

**方法名 :** `pl_count`

**调用例子 :**
```rust
let query = Query::new();
let result = client.pl_count(&query).await?;
```

---

#### 国家编码列表

说明 : 调用此接口，可获取国家编码列表

**方法名 :** `countries_code_list`

**调用例子 :**
```rust
let query = Query::new();
let result = client.countries_code_list(&query).await?;
```

---

#### 设置

说明 : 登录后调用此接口，可获取用户设置

**方法名 :** `setting`

**调用例子 :**
```rust
let query = Query::new();
let result = client.setting(&query).await?;
```

---

#### 根据 nickname 获取 userid

说明 : 使用此接口，传入用户昵称，可获取对应的用户 id，支持批量获取，多个昵称用分号(;)隔开

**必选参数 :** `nicknames` : 用户昵称，多个用分号(;)隔开

**方法名 :** `get_userids`

**调用例子 :**
```rust
let query = Query::new().param("nicknames", "test1;test2");
let result = client.get_userids(&query).await?;
```

---

#### 修改动态可见权限

说明 : 登录后修改本人动态的可见权限

**必选参数 :** `evId` : 动态 id ; `privacy` : 可见权限，取值 `0` / `1` / `2` / `6`

**方法名 :** `event_privacy`

**调用例子 :**
```rust
let query = Query::new()
    .param("evId", "A_EV_2_xxx")
    .param("privacy", "1");
let result = client.event_privacy(&query).await?;
```

---

#### 获取当前用户全部动态

说明 : 登录后自动翻页获取当前用户可枚举的全部动态，并按动态 id 去重。返回体包含 `events`、`retrievedCount`、`unavailableCount`、`sizeMismatch`、`pageCount`、`more`、`lasttime`。

**参数 :** 无

**方法名 :** `user_event_all`

**调用例子 :**
```rust
let query = Query::new();
let result = client.user_event_all(&query).await?;
```

---


### 歌曲相关

---

#### 获取歌曲详情

说明 : 调用此接口，传入音乐 id（支持多个 id，用逗号隔开），可获得歌曲详情

**必选参数 :** `ids` : 音乐 id，如 `ids=347230` 或者 `ids=347230,347231`

**方法名 :** `song_detail`

**调用例子 :**
```rust
let query = Query::new().param("ids", "347230,347231");
let result = client.song_detail(&query).await?;
```

---

#### 获取音乐 url

说明 : 使用歌单详情接口后，能得到的音乐的 id，但不能得到的音乐 url，调用此接口，传入的音乐 id（可多个，用逗号隔开），可以获取对应的音乐的 url，未登录状态或者非会员返回试听片段

**必选参数 :** `id` : 音乐 id

**可选参数 :** `br` : 码率，默认设置了 999000 即最大码率，如果要 320k 则可设置为 320000，其他类推

**方法名 :** `song_url`

**调用例子 :**
```rust
let query = Query::new().param("id", "1969519579");
let result = client.song_url(&query).await?;
```

---

#### 获取音乐 url - 新版

说明 : 使用注意事项同上

**必选参数 :** `id` : 音乐 id

**可选参数 :** `level` : 播放音质等级，常用取值：`standard` 标准、`higher` 较高、`exhigh` 极高、`lossless` 无损、`hires` Hi-Res、`jyeffect` 高清臻音、`vivid` 臻音全景声、`jymaster` 超清母带、`sky` 沉浸环绕声、`dolby` 杜比全景声，默认 `standard` ; `immerseType` : `level=sky` 时的沉浸声类型，可选 `c512` / `ste2` / `aac2` / `c51` / `ste` / `aac`，默认 `c51`

`level=vivid` 时会自动附带 Android 平台 cookie（`os=android`、`appver=9.5.61`）。

**方法名 :** `song_url_v1`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "1969519579")
    .param("level", "exhigh");
let result = client.song_url_v1(&query).await?;
```

---

#### 302 到音乐 url - 新版

说明 : 只允许传入单个 id，会使用 302 重定向请求到目标 url

**必选参数 :** `id` : 音乐 id ; `level` : 播放音质等级（同 `song_url_v1`，支持 `vivid`）

`level=vivid` 时会自动附带 Android 平台 cookie，且回退请求使用 `encodeType=mp3`。

**方法名 :** `song_url_v1_302`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "1969519579")
    .param("level", "exhigh");
let result = client.song_url_v1_302(&query).await?;
```

---

#### 歌曲播放链接 (ncmget)

说明 : ncmget 占位接口，始终返回空数据

**方法名 :** `song_url_ncmget`

**调用例子 :**
```rust
let query = Query::new();
let result = client.song_url_ncmget(&query).await?;
```

---

#### 直接获取灰色歌曲链接

说明 : 网易云歌曲解灰功能。注意：此功能依赖外部 unblockmusic-utils，Rust SDK 暂不支持，调用会返回不支持的错误。

**必选参数 :** `id` : 音乐 id

**可选参数 :** `source` : 选择要解灰的音源，不支持多音源

**方法名 :** `song_url_match`

**调用例子 :**
```rust
let query = Query::new().param("id", "1969519579");
let result = client.song_url_match(&query).await?;
// 注意: Rust SDK 暂不支持此功能，会返回 500 状态码
```

---

#### 获取客户端歌曲下载 url

说明 : 使用 `/song/url` 接口获取的是歌曲试听 url，但存在部分歌曲在非 VIP 账号上可以下载无损音质而不能试听无损音质，使用此接口可使非 VIP 账号获取这些歌曲的无损音频

**必选参数 :** `id` : 音乐 id（仅支持单首歌曲）

**可选参数 :** `br` : 码率，默认设置了 999000 即最大码率，如果要 320k 则可设置为 320000，其他类推

**方法名 :** `song_download_url`

**调用例子 :**
```rust
let query = Query::new().param("id", "1969519579");
let result = client.song_download_url(&query).await?;
```

---

#### 获取客户端歌曲下载链接 - 新版

说明 : 使用 `song_url_v1` 接口获取的是歌曲试听 url，非 VIP 账号最高只能获取极高音质，但免费类型的歌曲（fee == 0）使用本接口可最高获取 Hi-Res 音质的 url

**必选参数 :** `id` : 音乐 id ; `level` : 播放音质等级（同 `song_url_v1`，支持 `vivid`）

`level=vivid` 时会自动附带 Android 平台 cookie（`os=android`、`appver=9.5.61`）。

**方法名 :** `song_download_url_v1`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "2155423468")
    .param("level", "hires");
let result = client.song_download_url_v1(&query).await?;
```

---

#### 音乐是否可用

说明 : 调用此接口，传入歌曲 id，可获取音乐是否可用，返回 `{ success: true, message: 'ok' }` 或者 `{ success: false, message: '亲爱的,暂无版权' }`

**必选参数 :** `id` : 歌曲 id

**可选参数 :** `br` : 码率，默认设置了 999000 即最大码率，如果要 320k 则可设置为 320000，其他类推

**方法名 :** `check_music`

**调用例子 :**
```rust
let query = Query::new().param("id", "1969519579");
let result = client.check_music(&query).await?;
```

---

#### 获取歌词

说明 : 调用此接口，传入音乐 id 可获得对应音乐的歌词（不需要登录）

**必选参数 :** `id` : 音乐 id

**方法名 :** `lyric`

**调用例子 :**
```rust
let query = Query::new().param("id", "33894312");
let result = client.lyric(&query).await?;
```

---

#### 获取逐字歌词

说明 : 此接口的 `yrc` 字段即为逐字歌词（可能有歌曲不包含逐字歌词）

**必选参数 :** `id` : 音乐 id

**方法名 :** `lyric_new`

**调用例子 :**
```rust
let query = Query::new().param("id", "1824020871");
let result = client.lyric_new(&query).await?;
```

---

#### 喜欢音乐

说明 : 调用此接口，传入音乐 id，可喜欢该音乐

**必选参数 :** `id` : 歌曲 id

**可选参数 :** `like` : 布尔值，默认为 true 即喜欢，若传 false 则取消喜欢

**方法名 :** `like`

**调用例子 :**
```rust
let query = Query::new().param("id", "347230");
let result = client.like(&query).await?;
```

---

#### 喜欢音乐列表

说明 : 调用此接口，传入用户 id，可获取已喜欢音乐 id 列表（id 数组）

**必选参数 :** `uid` : 用户 id

**方法名 :** `likelist`

**调用例子 :**
```rust
let query = Query::new().param("uid", "32953014");
let result = client.likelist(&query).await?;
```

---

#### 歌曲是否喜爱

说明 : 调用此接口，传入歌曲 id 列表，可检查歌曲是否被喜爱

**必选参数 :** `ids` : 歌曲 id 列表，如 `[2058263032,1497529942]`

**方法名 :** `song_like_check`

**调用例子 :**
```rust
let query = Query::new().param("ids", "[2058263032,1497529942]");
let result = client.song_like_check(&query).await?;
```

---

#### 听歌打卡

说明 : 调用此接口，传入音乐 id，来源 id，歌曲时间 time，更新听歌排行数据

**必选参数 :** `id` : 歌曲 id ; `sourceid` : 歌单或专辑 id

**可选参数 :** `time` : 歌曲播放时间，单位为秒

**方法名 :** `scrobble`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "518066366")
    .param("sourceid", "36780169")
    .param("time", "291");
let result = client.scrobble(&query).await?;
```

---

#### 调整歌曲顺序

说明 : 登录后调用此接口，可以根据歌曲 id 顺序调整歌曲顺序

**必选参数 :** `pid` : 歌单 id ; `ids` : 歌曲 id 列表

**方法名 :** `song_order_update`

**调用例子 :**
```rust
let query = Query::new()
    .param("pid", "2039116066")
    .param("ids", "[5268328,1219871]");
let result = client.song_order_update(&query).await?;
```

---

#### 副歌时间

说明 : 调用此接口，传入歌曲 id，获取副歌时间

**必选参数 :** `id` : 歌曲 id

**方法名 :** `song_chorus`

**调用例子 :**
```rust
let query = Query::new().param("id", "2058263032");
let result = client.song_chorus(&query).await?;
```

---

#### 歌曲百科摘要

说明 : 调用此接口，传入歌曲 id，可获取歌曲百科摘要信息

**必选参数 :** `id` : 歌曲 id

**方法名 :** `song_wiki_summary`

**调用例子 :**
```rust
let query = Query::new().param("id", "1958384591");
let result = client.song_wiki_summary(&query).await?;
```

---

#### 歌曲音质详情

说明 : 调用此接口获取歌曲各个音质的文件信息，与获取歌曲详情接口相比，多出高清环绕声、沉浸环绕声、超清母带等音质的信息

**必选参数 :** `id` : 歌曲 id

**方法名 :** `song_music_detail`

**调用例子 :**
```rust
let query = Query::new().param("id", "2082700997");
let result = client.song_music_detail(&query).await?;
```

---

#### 歌曲红心数量

说明 : 调用此接口获取歌曲的红心用户数量

**必选参数 :** `id` : 歌曲 id

**方法名 :** `song_red_count`

**调用例子 :**
```rust
let query = Query::new().param("id", "186016");
let result = client.song_red_count(&query).await?;
```

---

#### 歌曲动态封面

说明 : 登录后调用此接口，传入歌曲 id，获取歌曲动态封面

**必选参数 :** `id` : 歌曲 id

**方法名 :** `song_dynamic_cover`

**调用例子 :**
```rust
let query = Query::new().param("id", "2101179024");
let result = client.song_dynamic_cover(&query).await?;
```

---

#### 会员下载歌曲记录

说明 : 调用此接口，可获得当前账号会员下载歌曲记录

**可选参数 :** `limit` : 返回数量，默认为 20 ; `offset` : 偏移数量，用于分页，默认为 0

**方法名 :** `song_downlist`

**调用例子 :**
```rust
let query = Query::new();
let result = client.song_downlist(&query).await?;
```

---

#### 会员本月下载歌曲记录

说明 : 调用此接口，可获得当前账号会员本月下载歌曲记录

**可选参数 :** `limit` : 返回数量，默认为 20 ; `offset` : 偏移数量，用于分页，默认为 0

**方法名 :** `song_monthdownlist`

**调用例子 :**
```rust
let query = Query::new();
let result = client.song_monthdownlist(&query).await?;
```

---

#### 已购买单曲下载记录

说明 : 调用此接口，可获得当前账号已购买单曲下载记录

**可选参数 :** `limit` : 返回数量，默认为 20 ; `offset` : 偏移数量，用于分页，默认为 0

**方法名 :** `song_singledownlist`

**调用例子 :**
```rust
let query = Query::new();
let result = client.song_singledownlist(&query).await?;
```

---

#### 已购单曲

说明 : 登录后调用此接口可获取已购买的单曲

**可选参数 :** `limit` : 返回数量，默认为 20 ; `offset` : 偏移数量，用于分页，默认为 0

**方法名 :** `song_purchased`

**调用例子 :**
```rust
let query = Query::new().param("limit", "10");
let result = client.song_purchased(&query).await?;
```

---

#### 歌词摘录 - 歌词摘录信息

说明 : 登录后调用此接口，传入歌曲 id，获取歌词摘录信息

**必选参数 :** `id` : 歌曲 id

**方法名 :** `song_lyrics_mark`

**调用例子 :**
```rust
let query = Query::new().param("id", "2058263032");
let result = client.song_lyrics_mark(&query).await?;
```

---

#### 歌词摘录 - 添加/修改摘录歌词

说明 : 登录后调用此接口，传入歌曲 id，可以添加/修改摘录歌词

**必选参数 :** `id` : 歌曲 id ; `data` : 存储歌词摘录信息的对象数组的字符串

**可选参数 :** `markId` : 若需要修改摘录信息，则需要填入此参数

**方法名 :** `song_lyrics_mark_add`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "2058263032")
    .param("data", "[{\"translateType\":1,\"startTimeStamp\":800,\"translateLyricsText\":\"让我逃走吧\",\"originalLyricsText\":\"逃がしてくれって声を枯らした\"}]");
let result = client.song_lyrics_mark_add(&query).await?;
```

---

#### 歌词摘录 - 删除摘录歌词

说明 : 登录后调用此接口，传入摘录歌词 id，删除摘录歌词

**必选参数 :** `id` : 摘录歌词 id

**方法名 :** `song_lyrics_mark_del`

**调用例子 :**
```rust
let query = Query::new().param("id", "2083850");
let result = client.song_lyrics_mark_del(&query).await?;
```

---

#### 歌词摘录 - 我的歌词本

说明 : 登录后调用此接口，获取我的歌词本

**可选参数 :** `limit` : 返回数量，默认为 20 ; `offset` : 偏移数量，用于分页，默认为 0

**方法名 :** `song_lyrics_mark_user_page`

**调用例子 :**
```rust
let query = Query::new();
let result = client.song_lyrics_mark_user_page(&query).await?;
```

---

#### 听歌识曲

说明 : 使用此接口，上传音频文件或者麦克风采集声音可识别对应歌曲信息

**必选参数 :** `duration` : 音频时长，单位秒 ; `audioFP` : 音频指纹

**方法名 :** `audio_match`

**调用例子 :**
```rust
let query = Query::new()
    .param("duration", "30")
    .param("audioFP", "...");
let result = client.audio_match(&query).await?;
```

---

#### 红心/取消红心歌曲 v1

说明 : 新版红心接口，使用 xeapi 通道和 v3 反作弊 token

**必选参数 :** `id` : 歌曲 id

**可选参数 :** `like` : 默认为 `true`，传 `false` 表示取消红心

**方法名 :** `like_v1`

**调用例子 :**
```rust
let query = Query::new().param("id", "347230");
let result = client.like_v1(&query).await?;

// 取消红心
let query = Query::new().param("id", "347230").param("like", "false");
let result = client.like_v1(&query).await?;
```

---

#### 喜欢歌曲（旧版）

说明 : 旧版喜欢/取消喜欢歌曲接口（weapi）。新代码建议使用 `like` 或 `like_v1`

**必选参数 :** `id` : 歌曲 id ; `uid` : 用户 id

**可选参数 :** `like` : 默认为 `true`，传 `false` 表示取消喜欢

**方法名 :** `song_like`

**调用例子 :**
```rust
let query = Query::new().param("id", "347230").param("uid", "32953014");
let result = client.song_like(&query).await?;
```

---

#### 提交歌曲播放状态

说明 : 上报歌曲播放状态，用于最近播放、播放进度同步等场景

**必选参数 :** `id` : 歌曲 id

**可选参数 :** `sessionId` : 会话 id，不传则随机生成 ; `progress` : 播放进度，默认 `0` ; `playMode` : 播放模式，默认 `list_loop` ; `type` : 资源类型，默认 `song`

**方法名 :** `relay_play_state_submit`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "1969519579")
    .param("progress", "30.5")
    .param("playMode", "list_loop");
let result = client.relay_play_state_submit(&query).await?;
```

---

#### 听歌打卡 v1（NCBL 加密）

说明 : 仿桌面客户端 PLV/PLD 上报的听歌打卡，使用 NCBL 加密上传。需要登录，且 query 中必须携带包含 `MUSIC_U` 的 cookie。PLV 与 PLD 分两次上传。

**必选参数 :** `id` : 歌曲 id ; `time` : 播放时长（秒，必须大于 0）

**可选参数 :** `total` : 歌曲总时长（秒），不传时使用 `time` ; `sourceid` / `sourceId` : 来源 id，默认使用歌曲 id ; `source` : 来源名，默认 `list` ; `bitrate` : 码率，默认 `320` ; `level` : 音质等级，默认 `exhigh`

**方法名 :** `scrobble_v1`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "1969519579")
    .param("time", "180")
    .param("total", "240")
    .param("sourceid", "36780169");
let result = client.scrobble_v1(&query).await?;
```

---

#### 灰色歌曲其他版本推荐

说明 : 获取灰色歌曲可播放的其他版本推荐

**必选参数 :** `id` : 歌曲 id（兼容 `songid`）

**方法名 :** `song_copyright_rcmd`

**调用例子 :**
```rust
let query = Query::new().param("id", "1969519579");
let result = client.song_copyright_rcmd(&query).await?;
```

---

#### 歌曲创作者信息

说明 : 获取歌曲的创作者（作曲、作词、编曲等）信息

**必选参数 :** `id` : 歌曲 id

**方法名 :** `song_creators`

**调用例子 :**
```rust
let query = Query::new().param("id", "1969519579");
let result = client.song_creators(&query).await?;
```

---

#### 插播相似歌曲

说明 : 获取播放页插播推荐使用的相似歌曲

**必选参数 :** `id` : 歌曲 id

**方法名 :** `song_simi_get`

**调用例子 :**
```rust
let query = Query::new().param("id", "1969519579");
let result = client.song_simi_get(&query).await?;
```

---

#### 歌曲百科

说明 : 获取歌曲百科信息

**必选参数 :** `id` : 歌曲 id

**方法名 :** `song_wiki_info`

**调用例子 :**
```rust
let query = Query::new().param("id", "1969519579");
let result = client.song_wiki_info(&query).await?;
```

---


### 搜索相关

---

#### 搜索

说明 : 调用此接口，传入搜索关键词可以搜索该音乐 / 专辑 / 歌手 / 歌单 / 用户，关键词可以多个，以空格隔开（不需要登录），可通过 `song_url` 接口传入歌曲 id 获取具体的播放链接

**必选参数 :** `keywords` : 关键词

**可选参数 :** `limit` : 返回数量，默认为 30 ; `offset` : 偏移数量，用于分页，默认为 0 ; `type` : 搜索类型，默认为 1 即单曲，取值意义：1: 单曲，10: 专辑，100: 歌手，1000: 歌单，1002: 用户，1004: MV，1006: 歌词，1009: 电台，1014: 视频，1018: 综合，2000: 声音

**方法名 :** `search`

**调用例子 :**
```rust
let query = Query::new()
    .param("keywords", "海阔天空")
    .param("type", "1")
    .param("limit", "30");
let result = client.search(&query).await?;
```

---

#### 搜索（综合）

说明 : 调用此接口，传入搜索关键词可以搜索该音乐 / 专辑 / 歌手 / 歌单 / 用户（返回结果更全面）

**必选参数 :** `keywords` : 关键词

**可选参数 :** `limit` : 返回数量，默认为 30 ; `offset` : 偏移数量，用于分页，默认为 0 ; `type` : 搜索类型（同 `search`）

**方法名 :** `cloudsearch`

**调用例子 :**
```rust
let query = Query::new()
    .param("keywords", "海阔天空");
let result = client.cloudsearch(&query).await?;
```

---

#### 默认搜索关键词

说明 : 调用此接口，可获取默认搜索关键词

**方法名 :** `search_default`

**调用例子 :**
```rust
let query = Query::new();
let result = client.search_default(&query).await?;
```

---

#### 热搜列表(简略)

说明 : 调用此接口，可获取热门搜索列表

**方法名 :** `search_hot`

**调用例子 :**
```rust
let query = Query::new();
let result = client.search_hot(&query).await?;
```

---

#### 热搜列表(详细)

说明 : 调用此接口，可获取热门搜索列表（详细版）

**方法名 :** `search_hot_detail`

**调用例子 :**
```rust
let query = Query::new();
let result = client.search_hot_detail(&query).await?;
```

---

#### 搜索建议

说明 : 调用此接口，传入搜索关键词可获得搜索建议，搜索结果同时包含单曲、歌手、歌单信息

**必选参数 :** `keywords` : 关键词

**可选参数 :** `type` : 如果传 `mobile` 则返回移动端数据

**方法名 :** `search_suggest`

**调用例子 :**
```rust
let query = Query::new().param("keywords", "海阔天空");
let result = client.search_suggest(&query).await?;
```

---

#### 搜索多重匹配

说明 : 调用此接口，传入搜索关键词可获得搜索结果

**必选参数 :** `keywords` : 关键词

**方法名 :** `search_multimatch`

**调用例子 :**
```rust
let query = Query::new().param("keywords", "海阔天空");
let result = client.search_multimatch(&query).await?;
```

---

#### 本地歌曲文件匹配网易云歌曲信息

说明 : 调用此接口可以为本地歌曲文件搜索匹配歌曲 ID、专辑封面等信息

**必选参数 :** `title` : 文件的标题信息 ; `album` : 文件的专辑信息 ; `artist` : 文件的艺术家信息 ; `duration` : 文件的时长，单位为秒 ; `md5` : 文件的 md5

**方法名 :** `search_match`

**调用例子 :**
```rust
let query = Query::new()
    .param("title", "富士山下")
    .param("album", "")
    .param("artist", "陈奕迅")
    .param("duration", "259.21")
    .param("md5", "bd708d006912a09d827f02e754cf8e56");
let result = client.search_match(&query).await?;
```

---

#### 搜索建议（PC 端）

说明 : PC 端搜索建议接口

**必选参数 :** `keyword` : 关键词

**方法名 :** `search_suggest_pc`

**调用例子 :**
```rust
let query = Query::new().param("keyword", "周杰伦");
let result = client.search_suggest_pc(&query).await?;
```

---


## 歌单相关

### 新建歌单

说明 : 调用此接口 , 传入歌单名字可新建歌单

**必选参数 :** `name` : 歌单名

**可选参数 :** `privacy` : 是否设置为隐私歌单，默认否，传'10'则设置成隐私歌单

`type` : 歌单类型,默认'NORMAL',传 'VIDEO'则为视频歌单,传 'SHARED'则为共享歌单

**方法名 :** `playlist_create`

**调用例子 :**
```rust
let query = Query::new().param("name", "测试歌单");
let result = client.playlist_create(&query).await?;

// 创建视频歌单
let query = Query::new().param("name", "test").param("type", "VIDEO");
let result = client.playlist_create(&query).await?;
```

### 删除歌单

说明 : 调用此接口 , 传入歌单 id 可删除歌单

**必选参数 :** `id` : 歌单 id,可多个,用逗号隔开

**方法名 :** `playlist_delete`

**调用例子 :**
```rust
let query = Query::new().param("id", "2947311456");
let result = client.playlist_delete(&query).await?;
```

### 收藏/取消收藏歌单

说明 : 调用此接口 , 传入类型和歌单 id 可收藏歌单或者取消收藏歌单

**必选参数 :** `t` : 类型,1:收藏,2:取消收藏

`id` : 歌单 id

**方法名 :** `playlist_subscribe`

**调用例子 :**
```rust
let query = Query::new().param("t", "1").param("id", "106697785");
let result = client.playlist_subscribe(&query).await?;
```

### 歌单收藏者

说明 : 调用此接口 , 传入歌单 id 可获取歌单的所有收藏者

**必选参数 :** `id` : 歌单 id

**可选参数 :** `limit` : 取出评论数量 , 默认为 20

`offset` : 偏移数量 , 用于分页 , 如 :( 评论页数 -1)*20, 其中 20 为 limit 的值

**方法名 :** `playlist_subscribers`

**调用例子 :**
```rust
let query = Query::new().param("id", "544215255").param("limit", "30");
let result = client.playlist_subscribers(&query).await?;
```

### 获取歌单详情

说明 : 歌单能看到歌单名字, 但看不到具体歌单内容 , 调用此接口 , 传入歌单 id, 可以获取对应歌单内的所有的音乐(未登录状态只能获取不完整的歌单,登录后是完整的)，但是返回的 trackIds 是完整的，tracks 则是不完整的，可拿全部 trackIds 请求一次 `song/detail` 接口获取所有歌曲的详情

**必选参数 :** `id` : 歌单 id

**可选参数 :** `s` : 歌单最近的 s 个收藏者,默认为 8

**方法名 :** `playlist_detail`

**调用例子 :**
```rust
let query = Query::new().param("id", "24381616");
let result = client.playlist_detail(&query).await?;
```

### 歌单详情动态

说明 : 调用后可获取歌单详情动态部分,如评论数,是否收藏,播放数

**必选参数 :** `id` : 歌单 id

**方法名 :** `playlist_detail_dynamic`

**调用例子 :**
```rust
let query = Query::new().param("id", "24381616");
let result = client.playlist_detail_dynamic(&query).await?;
```

### 相关歌单推荐

说明 : 调用此接口, 传入歌单 id, 获取相关歌单推荐

**必选参数 :** `id` : 歌单 id

**方法名 :** `playlist_detail_rcmd_get`

**调用例子 :**
```rust
let query = Query::new().param("id", "8039587836");
let result = client.playlist_detail_rcmd_get(&query).await?;
```

### 获取歌单所有歌曲

说明 : 由于网易云接口限制，歌单详情只会提供 10 首歌，通过调用此接口，传入对应的歌单 `id`，即可获得对应的所有歌曲

**必选参数 :** `id` : 歌单 id

**可选参数 :** `limit` : 限制获取歌曲的数量，默认值为当前歌单的歌曲数量

`offset` : 默认值为 0

**方法名 :** `playlist_track_all`

**调用例子 :**
```rust
let query = Query::new().param("id", "24381616").param("limit", "10").param("offset", "0");
let result = client.playlist_track_all(&query).await?;
```

### 对歌单添加或删除歌曲

说明 : 调用此接口 , 可以添加歌曲到歌单或者从歌单删除某首歌曲 ( 需要登录 )

**必选参数 :** `op` : 从歌单增加单曲为 add, 删除为 del

`pid` : 歌单 id

`tracks` : 歌曲 id,可多个,用逗号隔开

**方法名 :** `playlist_tracks`

**调用例子 :**
```rust
let query = Query::new()
    .param("op", "add")
    .param("pid", "24381616")
    .param("tracks", "347231");
let result = client.playlist_tracks(&query).await?;
```

### 收藏视频到视频歌单

说明 : 调用此接口 , 可收藏视频到视频歌单 ( 需要登录 )

**必选参数 :** `pid` : 歌单 id

`ids` : 视频 id,支持多个,用逗号隔开

**方法名 :** `playlist_track_add`

**调用例子 :**
```rust
let query = Query::new().param("pid", "5271999357").param("ids", "186041");
let result = client.playlist_track_add(&query).await?;
```

### 删除视频歌单里的视频

说明 : 调用此接口 , 可删除视频歌单里的视频 ( 需要登录 )

**必选参数 :** `pid` : 歌单 id

`ids` : 视频 id,支持多个,用逗号隔开

**方法名 :** `playlist_track_delete`

**调用例子 :**
```rust
let query = Query::new().param("pid", "5271999357").param("ids", "186041");
let result = client.playlist_track_delete(&query).await?;
```

### 更新歌单

说明 : 登录后调用此接口,可以更新用户歌单

**必选参数 :** `id` : 歌单 id

`name` : 歌单名字

`desc` : 歌单描述

`tags` : 歌单 tag ,多个用 `;` 隔开,只能用官方规定标签

**方法名 :** `playlist_update`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "24381616")
    .param("name", "新歌单")
    .param("desc", "描述")
    .param("tags", "欧美");
let result = client.playlist_update(&query).await?;
```

### 更新歌单描述

说明 : 登录后调用此接口,可以单独更新用户歌单描述

**必选参数 :** `id` : 歌单 id

`desc` : 歌单描述

**方法名 :** `playlist_desc_update`

**调用例子 :**
```rust
let query = Query::new().param("id", "24381616").param("desc", "描述");
let result = client.playlist_desc_update(&query).await?;
```

### 更新歌单名

说明 : 登录后调用此接口,可以单独更新用户歌单名

**必选参数 :** `id` : 歌单 id

`name` : 歌单名

**方法名 :** `playlist_name_update`

**调用例子 :**
```rust
let query = Query::new().param("id", "24381616").param("name", "歌单名");
let result = client.playlist_name_update(&query).await?;
```

### 更新歌单标签

说明 : 登录后调用此接口,可以单独更新用户歌单标签

**必选参数 :** `id` : 歌单 id

`tags` : 歌单标签

**方法名 :** `playlist_tags_update`

**调用例子 :**
```rust
let query = Query::new().param("id", "24381616").param("tags", "学习");
let result = client.playlist_tags_update(&query).await?;
```

### 歌单封面上传

说明 : 登录后调用此接口,使用 `'Content-Type': 'multipart/form-data'` 上传图片 formData(name 为'imgFile'),可更新歌单封面

**必选参数 :** `id` : 歌单 id

**可选参数 :** `imgSize` : 图片尺寸,默认为 300

`imgX` : 水平裁剪偏移,方形图片可不传,默认为 0

`imgY` : 垂直裁剪偏移,方形图片可不传,默认为 0

**方法名 :** `playlist_cover_update`

**调用例子 :**
```rust
let query = Query::new().param("id", "3143833470").param("imgSize", "200");
let result = client.playlist_cover_update(&query).await?;
```

### 调整歌单顺序

说明 : 登录后调用此接口,可以根据歌单 id 顺序调整歌单顺序

**必选参数 :** `ids` : 歌单 id 列表

**方法名 :** `playlist_order_update`

**调用例子 :**
```rust
let query = Query::new().param("ids", "[111,222]");
let result = client.playlist_order_update(&query).await?;
```

### 歌单更新播放量

说明 : 调用后可更新歌单播放量

**必选参数 :** `id` : 歌单 id

**方法名 :** `playlist_update_playcount`

**调用例子 :**
```rust
let query = Query::new().param("id", "24381616");
let result = client.playlist_update_playcount(&query).await?;
```

### 歌单分类

说明 : 调用此接口,可获取歌单分类,包含 category 信息

**方法名 :** `playlist_catlist`

**调用例子 :**
```rust
let query = Query::new();
let result = client.playlist_catlist(&query).await?;
```

### 热门歌单分类

说明 : 调用此接口,可获取歌单分类,包含 category 信息

**方法名 :** `playlist_hot`

**调用例子 :**
```rust
let query = Query::new();
let result = client.playlist_hot(&query).await?;
```

### 歌单分类列表

说明 : 调用此接口,可获取歌单分类列表

**可选参数 :** `cat` : 分类标签, 默认为 "全部"

`limit` : 返回数量 , 默认为 24

**方法名 :** `playlist_category_list`

**调用例子 :**
```rust
let query = Query::new().param("cat", "华语").param("limit", "10");
let result = client.playlist_category_list(&query).await?;
```

### 精品歌单标签列表

说明 : 调用此接口 , 可获取精品歌单标签列表

**方法名 :** `playlist_highquality_tags`

**调用例子 :**
```rust
let query = Query::new();
let result = client.playlist_highquality_tags(&query).await?;
```

### 公开隐私歌单

说明 : 可以调用此接口将当前用户的隐私歌单公开

**必选参数 :** `id` : 歌单 ID

**方法名 :** `playlist_privacy`

**调用例子 :**
```rust
let query = Query::new().param("id", "12345678");
let result = client.playlist_privacy(&query).await?;
```

### 获取点赞过的视频

说明 : 调用此接口, 可获取获取点赞过的视频

**方法名 :** `playlist_mylike`

**调用例子 :**
```rust
let query = Query::new();
let result = client.playlist_mylike(&query).await?;
```

### 最近播放的视频

说明 : 调用此接口 , 可获取最近播放的视频 ( 需要登录 )

**方法名 :** `playlist_video_recent`

**调用例子 :**
```rust
let query = Query::new();
let result = client.playlist_video_recent(&query).await?;
```

### 歌单导入 - 元数据/文字/链接导入

说明 : 登录后调用此接口, 支持通过元数据/文字/链接三种方式生成歌单; 三种方式不可同时调用

**可选参数 :** `local` : json 类型的字符串 (元数据导入)

`text` : 导入的文字 (文字导入)

`link` : 存有歌单链接的数组类型的字符串 (链接导入)

`importStarPlaylist` : 是否导入"我喜欢的音乐", 此项为 true 则不生成新的歌单

`playlistName` : 生成的歌单名, 仅文字导入和链接导入支持

**方法名 :** `playlist_import_name_task_create`

**调用例子 :**
```rust
// 文字导入
let query = Query::new().param("text", "海阔天空 Beyond");
let result = client.playlist_import_name_task_create(&query).await?;
```

### 歌单导入 - 任务状态

说明 : 调用此接口, 传入导入歌单任务 id, 获取任务状态

**必选参数 :** `id` : 任务 id

**方法名 :** `playlist_import_task_status`

**调用例子 :**
```rust
let query = Query::new().param("id", "123834369");
let result = client.playlist_import_task_status(&query).await?;
```

### 相关歌单

说明 : 请替换为相关歌单推荐接口(`playlist_detail_rcmd_get`); 本接口通过 html 抓取内容, 现已无法抓取歌单

**必选参数 :** `id` : 歌单 id

**方法名 :** `related_playlist`

**调用例子 :**
```rust
let query = Query::new().param("id", "1");
let result = client.related_playlist(&query).await?;
```

---

## 评论相关

### 歌曲评论

说明 : 调用此接口 , 传入音乐 id 和 limit 参数 , 可获得该音乐的所有评论 ( 不需要登录 )

**必选参数 :** `id` : 音乐 id

**可选参数 :** `limit` : 取出评论数量 , 默认为 20

`offset` : 偏移数量 , 用于分页 , 如 :( 评论页数 -1)*20, 其中 20 为 limit 的值

`before` : 分页参数,取上一页最后一项的 `time` 获取下一页数据(获取超过 5000 条评论的时候需要用到)

**方法名 :** `comment_music`

**调用例子 :**
```rust
let query = Query::new().param("id", "186016").param("limit", "1");
let result = client.comment_music(&query).await?;
```

### 专辑评论

说明 : 调用此接口 , 传入专辑 id 和 limit 参数 , 可获得该专辑的所有评论 ( 不需要登录 )

**必选参数 :** `id` : 专辑 id

**可选参数 :** `limit` : 取出评论数量 , 默认为 20

`offset` : 偏移数量 , 用于分页 , 如 :( 评论页数 -1)*20, 其中 20 为 limit 的值

`before` : 分页参数,取上一页最后一项的 `time` 获取下一页数据(获取超过 5000 条评论的时候需要用到)

**方法名 :** `comment_album`

**调用例子 :**
```rust
let query = Query::new().param("id", "32311");
let result = client.comment_album(&query).await?;
```

### 歌单评论

说明 : 调用此接口 , 传入歌单 id 和 limit 参数 , 可获得该歌单的所有评论 ( 不需要登录 )

**必选参数 :** `id` : 歌单 id

**可选参数 :** `limit` : 取出评论数量 , 默认为 20

`offset` : 偏移数量 , 用于分页 , 如 :( 评论页数 -1)*20, 其中 20 为 limit 的值

`before` : 分页参数,取上一页最后一项的 `time` 获取下一页数据(获取超过 5000 条评论的时候需要用到)

**方法名 :** `comment_playlist`

**调用例子 :**
```rust
let query = Query::new().param("id", "705123491");
let result = client.comment_playlist(&query).await?;
```

### mv 评论

说明 : 调用此接口 , 传入 mv id 和 limit 参数 , 可获得该 mv 的所有评论 ( 不需要登录 )

**必选参数 :** `id` : mv id

**可选参数 :** `limit` : 取出评论数量 , 默认为 20

`offset` : 偏移数量 , 用于分页 , 如 :( 评论页数 -1)*20, 其中 20 为 limit 的值

`before` : 分页参数,取上一页最后一项的 `time` 获取下一页数据(获取超过 5000 条评论的时候需要用到)

**方法名 :** `comment_mv`

**调用例子 :**
```rust
let query = Query::new().param("id", "5436712");
let result = client.comment_mv(&query).await?;
```

### 电台节目评论

说明 : 调用此接口 , 传入电台节目 id 和 limit 参数 , 可获得该电台节目的所有评论 ( 不需要登录 )

**必选参数 :** `id` : 电台节目的 id

**可选参数 :** `limit` : 取出评论数量 , 默认为 20

`offset` : 偏移数量 , 用于分页 , 如 :( 评论页数 -1)*20, 其中 20 为 limit 的值

`before` : 分页参数,取上一页最后一项的 `time` 获取下一页数据(获取超过 5000 条评论的时候需要用到)

**方法名 :** `comment_dj`

**调用例子 :**
```rust
let query = Query::new().param("id", "794062371");
let result = client.comment_dj(&query).await?;
```

### 视频评论

说明 : 调用此接口 , 传入视频 id 和 limit 参数 , 可获得该视频的所有评论 ( 不需要登录 )

**必选参数 :** `id` : 视频的 id

**可选参数 :** `limit` : 取出评论数量 , 默认为 20

`offset` : 偏移数量 , 用于分页 , 如 :( 评论页数 -1)*20, 其中 20 为 limit 的值

`before` : 分页参数,取上一页最后一项的 `time` 获取下一页数据(获取超过 5000 条评论的时候需要用到)

**方法名 :** `comment_video`

**调用例子 :**
```rust
let query = Query::new().param("id", "89ADDE33C0AAE8EC14B99F6750DB954D");
let result = client.comment_video(&query).await?;
```

### 获取动态评论

说明 : 登录后调用此接口 , 可以获取动态下评论

**必选参数 :** `threadId` : 动态 id，可通过 `/event`，`/user/event` 接口获取

**方法名 :** `comment_event`

**调用例子 :**
```rust
let query = Query::new().param("threadId", "A_EV_2_6559519868_32953014");
let result = client.comment_event(&query).await?;
```

### 楼层评论

说明 : 调用此接口 , 传入资源 parentCommentId 和资源类型 type 和资源 id 参数, 可获得该资源的歌曲楼层评论

**必选参数 :** `parentCommentId` : 楼层评论 id

`id` : 资源 id

`type` : 数字 , 资源类型 , 对应歌曲 , mv, 专辑 , 歌单 , 电台, 视频对应以下类型: 0: 歌曲, 1: mv, 2: 歌单, 3: 专辑, 4: 电台节目, 5: 视频, 6: 动态, 7: 电台

**可选参数 :** `limit` : 取出评论数量 , 默认为 20

`time` : 分页参数,取上一页最后一项的 `time` 获取下一页数据

**方法名 :** `comment_floor`

**调用例子 :**
```rust
let query = Query::new()
    .param("parentCommentId", "1438569889")
    .param("id", "29764564")
    .param("type", "0");
let result = client.comment_floor(&query).await?;
```

### 热门评论

说明 : 调用此接口 , 传入 type, 资源 id 可获得对应资源热门评论 ( 不需要登录 )

**必选参数 :** `id` : 资源 id

`type` : 数字 , 资源类型 , 对应歌曲 , mv, 专辑 , 歌单 , 电台, 视频对应以下类型: 0: 歌曲, 1: mv, 2: 歌单, 3: 专辑, 4: 电台节目, 5: 视频, 6: 动态, 7: 电台

**可选参数 :** `limit` : 取出评论数量 , 默认为 20

`offset` : 偏移数量 , 用于分页 , 如 :( 评论页数 -1)*20, 其中 20 为 limit 的值

`before` : 分页参数,取上一页最后一项的 `time` 获取下一页数据(获取超过 5000 条评论的时候需要用到)

**方法名 :** `comment_hot`

**调用例子 :**
```rust
let query = Query::new().param("id", "186016").param("type", "0");
let result = client.comment_hot(&query).await?;
```

### 给评论点赞

说明 : 调用此接口 , 传入 type, 资源 id, 和评论 id cid 和 是否点赞参数 t 即可给对应评论点赞 ( 需要登录 )

**必选参数 :** `id` : 资源 id, 如歌曲 id,mv id

`cid` : 评论 id

`t` : 是否点赞 , 1 为点赞 ,0 为取消点赞

`type` : 数字 , 资源类型 , 对应歌曲 , mv, 专辑 , 歌单 , 电台, 视频对应以下类型: 0: 歌曲, 1: mv, 2: 歌单, 3: 专辑, 4: 电台节目, 5: 视频, 6: 动态, 7: 电台

**方法名 :** `comment_like`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "29178366")
    .param("cid", "12840183")
    .param("t", "1")
    .param("type", "0");
let result = client.comment_like(&query).await?;
```

### 新版评论接口

说明 : 调用此接口 , 传入资源类型和资源 id,以及排序方式,可获取对应资源的评论

**必选参数 :** `id` : 资源 id, 如歌曲 id,mv id

`type` : 数字 , 资源类型 , 对应歌曲 , mv, 专辑 , 歌单 , 电台, 视频对应以下类型: 0: 歌曲, 1: mv, 2: 歌单, 3: 专辑, 4: 电台节目, 5: 视频, 6: 动态, 7: 电台

**可选参数 :** `pageNo` : 分页参数,第 N 页,默认为 1

`pageSize` : 分页参数,每页多少条数据,默认 20

`sortType` : 排序方式, 1:按推荐排序, 2:按热度排序, 3:按时间排序

`cursor` : 当 `sortType` 为 3 时且页数不是第一页时需传入,值为上一条数据的 time

**方法名 :** `comment_new`

**调用例子 :**
```rust
let query = Query::new()
    .param("type", "0")
    .param("id", "1407551413")
    .param("sortType", "3");
let result = client.comment_new(&query).await?;
```

### 评论抱一抱列表

说明 : 调用此接口,可获取评论抱一抱列表

**必选参数 :** `uid` : 用户 id

`cid` : 评论 id

`sid` : 资源 id

**可选参数 :** `page` : 页数

`cursor` : 上一页返回的 cursor,默认-1,第一页不需要传

`idCursor` : 上一页返回的 idCursor,默认-1,第一页不需要传

`pageSize` : 每页页数,默认 100

**方法名 :** `comment_hug_list`

**调用例子 :**
```rust
let query = Query::new()
    .param("uid", "285516405")
    .param("cid", "1167145843")
    .param("sid", "863481066")
    .param("pageSize", "2")
    .param("page", "1");
let result = client.comment_hug_list(&query).await?;
```

### 评论统计数据

说明 : 调用此接口 , 传入资源类型和资源 id 列表 , 可批量获取对应资源的评论统计数据 ( 不需要登录 )

**必选参数 :** `type` : 数字 , 资源类型 , 对应以下类型: 0: 歌曲, 1: mv, 2: 歌单, 3: 专辑, 4: 电台节目, 5: 视频, 6: 动态, 7: 电台

`ids` : 资源 id 列表 , 多个 id 用逗号分隔 , 如 `186016,347230`

**方法名 :** `comment_info_list`

**调用例子 :**
```rust
let query = Query::new().param("type", "0").param("ids", "186016,347230");
let result = client.comment_info_list(&query).await?;
```

### 抱一抱评论

说明 : 调用此接口,可抱一抱评论

**必选参数 :** `uid` : 用户 id

`cid` : 评论 id

`sid` : 资源 id

**方法名 :** `hug_comment`

**调用例子 :**
```rust
let query = Query::new()
    .param("uid", "285516405")
    .param("cid", "1167145843")
    .param("sid", "863481066");
let result = client.hug_comment(&query).await?;
```

### 云村星评馆 - 简要评论

说明 : 调用此接口可以获取首页推荐的星评馆评论信息

**方法名 :** `starpick_comments_summary`

**调用例子 :**
```rust
let query = Query::new();
let result = client.starpick_comments_summary(&query).await?;
```

---

### 发送评论

说明 : 发送评论。使用 xeapi 通道和易盾 v3 token，需要登录

**必选参数 :** `id` : 资源 id ; `type` : 资源类型，`0` 为歌曲，其他类型见 `RESOURCE_TYPE_MAP` ; `content` : 评论内容

**方法名 :** `comment_add`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("id", "347230")
    .param("type", "0")
    .param("content", "好听");
let result = client.comment_add(&query).await?;
```

---

### 回复评论

说明 : 回复指定评论。使用 xeapi 通道和易盾 v3 token，需要登录

**必选参数 :** `id` : 资源 id ; `type` : 资源类型 ; `cid` : 被回复的评论 id ; `content` : 回复内容

> 注意：参数已按上游改为 `id` / `type` / `cid`，旧版的 `threadId` / `commentId` 不再使用。

**方法名 :** `comment_reply`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("id", "347230")
    .param("type", "0")
    .param("cid", "123456789")
    .param("content", "回复内容");
let result = client.comment_reply(&query).await?;
```

---

### 删除评论

说明 : 删除自己发送的评论。使用 xeapi 通道，需要登录

**必选参数 :** `id` : 资源 id ; `type` : 资源类型 ; `cid` : 评论 id（兼容旧参数名 `commentId`）

**方法名 :** `comment_delete`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("id", "347230")
    .param("type", "0")
    .param("cid", "123456789");
let result = client.comment_delete(&query).await?;
```

---

### 举报评论

说明 : 举报指定评论

**必选参数 :** `id` : 资源 id ; `cid` : 评论 id

**可选参数 :** `reason` : 举报原因

**方法名 :** `comment_report`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("id", "347230")
    .param("cid", "123456789")
    .param("reason", "垃圾广告");
let result = client.comment_report(&query).await?;
```

---

### 通用评论接口

说明 : 兼容 Node.js 版 `/comment` 的通用入口，根据 `t` 分发到发送、删除、回复。新代码建议直接使用 `comment_add` / `comment_delete` / `comment_reply`

**必选参数 :** `t` : `1` 发送、`0` 删除、`2` 回复 ; `type` : 资源类型

**按操作需要 :** `id` : 资源 id ; `content` : 评论内容 ; `commentId` : 评论 id ; 动态评论时传 `threadId`

**方法名 :** `comment`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("t", "1")
    .param("type", "0")
    .param("id", "347230")
    .param("content", "好听");
let result = client.comment(&query).await?;
```

---


## 歌手相关

### 获取歌手单曲

说明 : 调用此接口 , 传入歌手 id, 可获得歌手部分信息和热门歌曲

**必选参数 :** `id` : 歌手 id, 可由搜索接口获得

**方法名 :** `artists`

**调用例子 :**
```rust
let query = Query::new().param("id", "6452");
let result = client.artists(&query).await?;
```

### 获取歌手详情

说明 : 调用此接口 , 传入歌手 id, 可获得获取歌手详情

**必选参数 :** `id` : 歌手 id

**方法名 :** `artist_detail`

**调用例子 :**
```rust
let query = Query::new().param("id", "11972054");
let result = client.artist_detail(&query).await?;
```

### 歌手详情动态

说明 : 调用后可获取歌手详情动态部分,如是否关注,视频数

**必选参数 :** `id` : 歌手 id

**方法名 :** `artist_detail_dynamic`

**调用例子 :**
```rust
let query = Query::new().param("id", "15396");
let result = client.artist_detail_dynamic(&query).await?;
```

### 歌手全部歌曲

说明 : 调用此接口,可获取歌手全部歌曲

**必选参数 :** `id` : 歌手 id

**可选参数 :** `order` : `hot` ,`time` 按照热门或者时间排序

`limit` : 取出歌单数量 , 默认为 50

`offset` : 偏移数量 , 用于分页 , 如 :( 评论页数 -1)*50, 其中 50 为 limit 的值

**方法名 :** `artist_songs`

**调用例子 :**
```rust
let query = Query::new().param("id", "6452");
let result = client.artist_songs(&query).await?;
```

### 获取歌手专辑

说明 : 调用此接口 , 传入歌手 id, 可获得歌手专辑内容

**必选参数 :** `id` : 歌手 id

**可选参数 :** `limit` : 取出数量 , 默认为 30

`offset` : 偏移数量 , 用于分页 , 如 :( 页数 -1)*30, 其中 30 为 limit 的值 , 默认为 0

**方法名 :** `artist_album`

**调用例子 :**
```rust
let query = Query::new().param("id", "6452").param("limit", "5");
let result = client.artist_album(&query).await?;
```

### 获取歌手描述

说明 : 调用此接口 , 传入歌手 id, 可获得歌手描述

**必选参数 :** `id` : 歌手 id

**方法名 :** `artist_desc`

**调用例子 :**
```rust
let query = Query::new().param("id", "6452");
let result = client.artist_desc(&query).await?;
```

### 获取歌手 mv

说明 : 调用此接口 , 传入歌手 id, 可获得歌手 mv 信息 , 具体 mv 播放地址可调用 `mv_detail` 传入此接口获得的 mvid 来拿到

**必选参数 :** `id` : 歌手 id, 可由搜索接口获得

**方法名 :** `artist_mv`

**调用例子 :**
```rust
let query = Query::new().param("id", "6452");
let result = client.artist_mv(&query).await?;
```

### 歌手分类列表

说明 : 调用此接口,可获取歌手分类列表

**可选参数 :** `limit` : 返回数量 , 默认为 30

`offset` : 偏移数量，用于分页 , 如 :( 页数 -1)*30, 其中 30 为 limit 的值 , 默认为 0

`initial` : 按首字母索引查找参数, 热门传-1,#传 0

`type` : 取值: -1:全部, 1:男歌手, 2:女歌手, 3:乐队

`area` : 取值: -1:全部, 7:华语, 96:欧美, 8:日本, 16:韩国, 0:其他

**方法名 :** `artist_list`

**调用例子 :**
```rust
let query = Query::new()
    .param("type", "1")
    .param("area", "96")
    .param("initial", "b");
let result = client.artist_list(&query).await?;
```

### 收藏/取消收藏歌手

说明 : 调用此接口,可收藏歌手

**必选参数 :** `id` : 歌手 id

`t` : 操作,1 为收藏,其他为取消收藏

**方法名 :** `artist_sub`

**调用例子 :**
```rust
let query = Query::new().param("id", "6452").param("t", "1");
let result = client.artist_sub(&query).await?;
```

### 收藏的歌手列表

说明 : 调用此接口,可获取收藏的歌手列表

**可选参数 :** `limit` : 取出数量 , 默认为 25

`offset` : 偏移数量 , 用于分页 , 如 :( 评论页数 -1)*25, 其中 25 为 limit 的值

**方法名 :** `artist_sublist`

**调用例子 :**
```rust
let query = Query::new();
let result = client.artist_sublist(&query).await?;
```

### 歌手热门 50 首歌曲

说明 : 调用此接口,可获取歌手热门 50 首歌曲

**必选参数 :** `id` : 歌手 id

**方法名 :** `artist_top_song`

**调用例子 :**
```rust
let query = Query::new().param("id", "6452");
let result = client.artist_top_song(&query).await?;
```

### 歌手粉丝

说明 : 调用此接口 , 传入歌手 id, 可获取歌手粉丝

**必选参数 :** `id` : 歌手 id

**可选参数 :** `limit` : 取出数量 , 默认为 20

`offset` : 偏移数量 , 用于分页

**方法名 :** `artist_fans`

**调用例子 :**
```rust
let query = Query::new().param("id", "2116").param("limit", "10").param("offset", "0");
let result = client.artist_fans(&query).await?;
```

### 歌手粉丝数量

说明 : 调用此接口 , 传入歌手 id, 可获取歌手粉丝数量

**必选参数 :** `id` : 歌手 id

**方法名 :** `artist_follow_count`

**调用例子 :**
```rust
let query = Query::new().param("id", "2116");
let result = client.artist_follow_count(&query).await?;
```

### 关注歌手新 MV

说明 : 登录后调用此接口可获取关注歌手新 MV

**可选参数 :** `limit` : 取出数量 , 默认为 20

`before` : 上一页数据返回的 publishTime 的数据

**方法名 :** `artist_new_mv`

**调用例子 :**
```rust
let query = Query::new().param("limit", "1");
let result = client.artist_new_mv(&query).await?;
```

### 关注歌手新歌

说明 : 登录后调用此接口可获取关注歌手新歌

**可选参数 :** `limit` : 取出数量 , 默认为 20

`before` : 上一页数据返回的 publishTime 的数据

**方法名 :** `artist_new_song`

**调用例子 :**
```rust
let query = Query::new().param("limit", "1");
let result = client.artist_new_song(&query).await?;
```

### 获取歌手视频

说明 : 调用此接口 , 传入歌手 id, 可获得歌手视频

**必选参数 :** `id` : 歌手 id

**可选参数 :** `size` : 返回数量 , 默认为 10

`cursor` : 返回数据的 cursor, 默认为 0 , 传入上一次返回结果的 cursor,将会返回下一页的数据

`order` : 排序方法, 0 表示按时间排序, 1 表示按热度排序, 默认为 0

**方法名 :** `artist_video`

**调用例子 :**
```rust
let query = Query::new().param("id", "2116");
let result = client.artist_video(&query).await?;
```

---

### 关注歌手新歌与 MV（v2）

说明 : 登录后获取关注歌手的新歌曲和 MV

**可选参数 :** `startTimestamp` / `before` : 起始时间戳（毫秒），默认当前时间 ; `sourceType` : 来源类型，默认 `1` ; `limit` : 数量，默认 `10` ; `firstRequest` : 是否首次请求，默认 `true`

**方法名 :** `artist_new_song_mv_list_v2`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("limit", "20")
    .param("firstRequest", "false");
let result = client.artist_new_song_mv_list_v2(&query).await?;
```

---

### 关注歌手新歌播放全部

说明 : 登录后获取所有关注歌手最近的 50 首新歌

**参数 :** 无

**方法名 :** `artist_new_song_playall`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.artist_new_song_playall(&query).await?;
```

---

### 乐迷团详情

说明 : 获取歌手乐迷团详情

**必选参数 :** `groupId` : 乐迷团 id（兼容 `id`）

**可选参数 :** `scene` : 场景标识

**方法名 :** `fans_group_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("groupId", "123456");
let result = client.fans_group_detail(&query).await?;
```

---

### 乐迷团推荐笔记

说明 : 获取乐迷团推荐笔记列表

**必选参数 :** `fansGroupId` : 乐迷团 id（兼容 `groupId`）

**可选参数 :** `artistSelf` : 是否歌手本人，默认 `0` ; `cursor` : 分页游标，默认 `0` ; `size` : 数量，默认 `10`

**方法名 :** `fans_group_feed_recommend`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("fansGroupId", "123456")
    .param("size", "20");
let result = client.fans_group_feed_recommend(&query).await?;
```

---

### 用户所在乐迷团详情

说明 : 获取当前用户在某乐迷团中的详情

**必选参数 :** `groupId` : 乐迷团 id（兼容 `id`）

**方法名 :** `fans_group_user_group_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("groupId", "123456");
let result = client.fans_group_user_group_detail(&query).await?;
```

---

### 用户加入的乐迷团列表

说明 : 登录后获取当前用户加入的全部歌手乐迷团

**参数 :** 无

**方法名 :** `fans_group_user_groups`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.fans_group_user_groups(&query).await?;
```

---


## 专辑相关

### 获取专辑内容

说明 : 调用此接口 , 传入专辑 id, 可获得专辑内容

**必选参数 :** `id` : 专辑 id

**方法名 :** `album`

**调用例子 :**
```rust
let query = Query::new().param("id", "32311");
let result = client.album(&query).await?;
```

### 数字专辑详情

说明 : 调用此接口 , 传入数字专辑 id 可获取数字专辑详情(和歌单详情有差异)

**必选参数 :** `id` : 专辑 id

**方法名 :** `album_detail`

**调用例子 :**
```rust
let query = Query::new().param("id", "84547195");
let result = client.album_detail(&query).await?;
```

### 专辑动态信息

说明 : 调用此接口 , 传入专辑 id, 可获得专辑动态信息,如是否收藏,收藏数,评论数,分享数

**必选参数 :** `id` : 专辑 id

**方法名 :** `album_detail_dynamic`

**调用例子 :**
```rust
let query = Query::new().param("id", "32311");
let result = client.album_detail_dynamic(&query).await?;
```

### 收藏/取消收藏专辑

说明 : 调用此接口,可收藏/取消收藏专辑

**必选参数 :** `id` : 专辑 id

`t` : 1 为收藏,其他为取消收藏

**方法名 :** `album_sub`

**调用例子 :**
```rust
let query = Query::new().param("id", "32311").param("t", "1");
let result = client.album_sub(&query).await?;
```

### 获取已收藏专辑列表

说明 : 调用此接口 , 可获得已收藏专辑列表

**可选参数 :** `limit` : 取出数量 , 默认为 25

`offset` : 偏移数量 , 用于分页 , 如 :( 页数 -1)*25, 其中 25 为 limit 的值 , 默认为 0

**方法名 :** `album_sublist`

**调用例子 :**
```rust
let query = Query::new();
let result = client.album_sublist(&query).await?;
```

### 最新专辑

说明 : 调用此接口 ，获取云音乐首页新碟上架数据

**方法名 :** `album_newest`

**调用例子 :**
```rust
let query = Query::new();
let result = client.album_newest(&query).await?;
```

### 全部新碟

说明 : 登录后调用此接口 ,可获取全部新碟

**可选参数 :** `limit` : 返回数量 , 默认为 30

`offset` : 偏移数量，用于分页 , 如 :( 页数 -1)*30, 其中 30 为 limit 的值 , 默认为 0

`area` : ALL:全部,ZH:华语,EA:欧美,KR:韩国,JP:日本

**方法名 :** `album_new`

**调用例子 :**
```rust
let query = Query::new().param("area", "KR").param("limit", "10");
let result = client.album_new(&query).await?;
```

### 数字专辑-新碟上架

说明 : 调用此接口 ,可获取数字专辑-新碟上架

**可选参数 :** `limit` : 返回数量 , 默认为 30

`offset` : 偏移数量，用于分页 , 如 :( 页数 -1)*30, 其中 30 为 limit 的值 , 默认为 0

**方法名 :** `album_list`

**调用例子 :**
```rust
let query = Query::new().param("limit", "10");
let result = client.album_list(&query).await?;
```

### 数字专辑-语种风格馆

说明 : 调用此接口 ,可获取语种风格馆数字专辑列表

**可选参数 :** `limit` : 返回数量 , 默认为 30

`offset` : 偏移数量，用于分页 , 如 :( 页数 -1)*30, 其中 30 为 limit 的值 , 默认为 0

`area` : 地区 Z_H:华语,E_A:欧美,KR:韩国,JP:日本

**方法名 :** `album_list_style`

**调用例子 :**
```rust
let query = Query::new().param("area", "Z_H").param("offset", "2");
let result = client.album_list_style(&query).await?;
```

### 获取专辑歌曲的音质

说明 : 调用后可获取专辑歌曲的音质

**必选参数 :** `id` : 专辑 id

**方法名 :** `album_privilege`

**调用例子 :**
```rust
let query = Query::new().param("id", "168223858");
let result = client.album_privilege(&query).await?;
```

### 数字专辑&数字单曲-榜单

说明 : 调用此接口 ,可获取数字专辑&数字单曲-榜单

**可选参数 :** `limit` : 返回数量 , 默认为 30

`offset` : 偏移数量，用于分页 , 如 :( 页数 -1)*30, 其中 30 为 limit 的值 , 默认为 0

`albumType` : 0 为数字专辑, 1 为数字单曲

`type` : daily:日榜,week:周榜,year:年榜,total:总榜

**方法名 :** `album_songsaleboard`

**调用例子 :**
```rust
let query = Query::new()
    .param("type", "year")
    .param("year", "2020")
    .param("albumType", "0");
let result = client.album_songsaleboard(&query).await?;
```

---

## MV 相关

### 全部 mv

说明 : 调用此接口 , 可获取全部 mv

**可选参数 :** `area` : 地区,可选值为全部,内地,港台,欧美,日本,韩国,不填则为全部

`type` : 类型,可选值为全部,官方版,原生,现场版,网易出品,不填则为全部

`order` : 排序,可选值为上升最快,最热,最新,不填则为上升最快

`limit` : 取出数量 , 默认为 30

`offset` : 偏移数量 , 用于分页 , 如 :( 页数 -1)*50, 其中 50 为 limit 的值 , 默认为 0

**方法名 :** `mv_all`

**调用例子 :**
```rust
let query = Query::new().param("area", "港台");
let result = client.mv_all(&query).await?;
```

### 最新 mv

说明 : 调用此接口 , 可获取最新 mv

**可选参数 :** `area` : 地区,可选值为全部,内地,港台,欧美,日本,韩国,不填则为全部

`limit` : 取出数量 , 默认为 30

**方法名 :** `mv_first`

**调用例子 :**
```rust
let query = Query::new().param("limit", "10");
let result = client.mv_first(&query).await?;
```

### 网易出品 mv

说明 : 调用此接口 , 可获取网易出品 mv

**可选参数 :** `limit` : 取出数量 , 默认为 30

`offset` : 偏移数量 , 用于分页 , 如 :( 页数 -1)*30, 其中 30 为 limit 的值 , 默认为 0

**方法名 :** `mv_exclusive_rcmd`

**调用例子 :**
```rust
let query = Query::new().param("limit", "10");
let result = client.mv_exclusive_rcmd(&query).await?;
```

### 获取 mv 数据

说明 : 调用此接口 , 传入 mvid ( 在搜索音乐的时候传 type=1004 获得 ) , 可获取对应 MV 数据 , 数据包含 mv 名字 , 歌手 , 发布时间 , mv 视频地址等数据 , 其中 mv 视频网易做了防盗链处理 , 可能不能直接播放 , 需要播放的话需要调用 `mv_url` 接口

**必选参数 :** `mvid` : mv 的 id

**方法名 :** `mv_detail`

**调用例子 :**
```rust
let query = Query::new().param("mvid", "5436712");
let result = client.mv_detail(&query).await?;
```

### 获取 mv 点赞转发评论数数据

说明 : 调用此接口 , 传入 mvid ( 在搜索音乐的时候传 type=1004 获得 ) , 可获取对应 MV 点赞转发评论数数据

**必选参数 :** `mvid` : mv 的 id

**方法名 :** `mv_detail_info`

**调用例子 :**
```rust
let query = Query::new().param("mvid", "5436712");
let result = client.mv_detail_info(&query).await?;
```

### mv 地址

说明 : 调用此接口 , 传入 mv id,可获取 mv 播放地址

**必选参数 :** `id` : mv id

**可选参数 :** `r` : 分辨率,默认 1080,可从 `mv_detail` 接口获取分辨率列表

**方法名 :** `mv_url`

**调用例子 :**
```rust
let query = Query::new().param("id", "5436712");
let result = client.mv_url(&query).await?;

// 指定分辨率
let query = Query::new().param("id", "10896407").param("r", "1080");
let result = client.mv_url(&query).await?;
```

### 收藏/取消收藏 MV

说明 : 调用此接口,可收藏/取消收藏 MV

**必选参数 :** `mvid` : MV id

`t` : 1 为收藏,其他为取消收藏

**方法名 :** `mv_sub`

**调用例子 :**
```rust
let query = Query::new().param("mvid", "5436712").param("t", "1");
let result = client.mv_sub(&query).await?;
```

### 收藏的 MV 列表

说明 : 调用此接口,可获取收藏的 MV 列表

**方法名 :** `mv_sublist`

**调用例子 :**
```rust
let query = Query::new();
let result = client.mv_sublist(&query).await?;
```

### 推荐 mv

说明 : 调用此接口 , 可获取推荐 mv

**方法名 :** `personalized_mv`

**调用例子 :**
```rust
let query = Query::new();
let result = client.personalized_mv(&query).await?;
```

### mv 排行

说明 : 调用此接口 , 可获取 mv 排行

**可选参数 :** `limit` : 取出数量 , 默认为 30

`area` : 地区,可选值为内地,港台,欧美,日本,韩国,不填则为全部

`offset` : 偏移数量 , 用于分页 , 如 :( 页数 -1)*30, 其中 30 为 limit 的值 , 默认为 0

**方法名 :** `top_mv`

**调用例子 :**
```rust
let query = Query::new().param("limit", "10");
let result = client.top_mv(&query).await?;
```

---

## 视频相关

### 获取视频标签列表

说明 : 调用此接口 , 可获取视频标签列表

**方法名 :** `video_group_list`

**调用例子 :**
```rust
let query = Query::new();
let result = client.video_group_list(&query).await?;
```

### 获取视频分类列表

说明 : 调用此接口 , 可获取视频分类列表

**方法名 :** `video_category_list`

**调用例子 :**
```rust
let query = Query::new();
let result = client.video_category_list(&query).await?;
```

### 获取视频标签/分类下的视频

说明 : 调用此接口 , 传入标签/分类 `id`,可获取到相关的视频,分页参数只能传入 offset

**必选参数 :** `id` : videoGroup 的 id

**可选参数 :** `offset` : 默认 0

**方法名 :** `video_group`

**调用例子 :**
```rust
let query = Query::new().param("id", "9104");
let result = client.video_group(&query).await?;
```

### 获取全部视频列表

说明 : 调用此接口,可获取视频分类列表,分页参数只能传入 offset

**可选参数 :** `offset` : 默认 0

**方法名 :** `video_timeline_all`

**调用例子 :**
```rust
let query = Query::new();
let result = client.video_timeline_all(&query).await?;
```

### 获取推荐视频

说明 : 调用此接口, 可获取推荐视频,分页参数只能传入 offset

**可选参数 :** `offset` : 默认 0

**方法名 :** `video_timeline_recommend`

**调用例子 :**
```rust
let query = Query::new().param("offset", "10");
let result = client.video_timeline_recommend(&query).await?;
```

### 视频详情

说明 : 调用此接口 , 可获取视频详情

**必选参数 :** `id` : 视频 的 id

**方法名 :** `video_detail`

**调用例子 :**
```rust
let query = Query::new().param("id", "89ADDE33C0AAE8EC14B99F6750DB954D");
let result = client.video_detail(&query).await?;
```

### 获取视频点赞转发评论数数据

说明 : 调用此接口 , 传入 vid ( 视频 id ) , 可获取对应视频点赞转发评论数数据

**必选参数 :** `vid` : 视频 id

**方法名 :** `video_detail_info`

**调用例子 :**
```rust
let query = Query::new().param("vid", "89ADDE33C0AAE8EC14B99F6750DB954D");
let result = client.video_detail_info(&query).await?;
```

### 获取视频播放地址

说明 : 调用此接口 , 传入视频 id,可获取视频播放地址

**必选参数 :** `id` : 视频 的 id

**方法名 :** `video_url`

**调用例子 :**
```rust
let query = Query::new().param("id", "89ADDE33C0AAE8EC14B99F6750DB954D");
let result = client.video_url(&query).await?;
```

### 收藏视频

说明 : 调用此接口,可收藏视频

**必选参数 :** `id` : 视频 id

`t` : 1 为收藏,其他为取消收藏

**方法名 :** `video_sub`

**调用例子 :**
```rust
let query = Query::new()
    .param("id", "89ADDE33C0AAE8EC14B99F6750DB954D")
    .param("t", "1");
let result = client.video_sub(&query).await?;
```

### 相关视频

说明 : 调用此接口 , 可获取相关视频

**必选参数 :** `id` : 视频 的 id

**方法名 :** `related_allvideo`

**调用例子 :**
```rust
let query = Query::new().param("id", "89ADDE33C0AAE8EC14B99F6750DB954D");
let result = client.related_allvideo(&query).await?;
```

### 获取 mlog 播放地址

说明 : 调用此接口 , 传入 mlog id, 可获取 mlog 播放地址

**必选参数 :** `id` : mlog id

**可选参数 :** `res` : 分辨率 , 默认为 1080

**方法名 :** `mlog_url`

**调用例子 :**
```rust
let query = Query::new().param("id", "a1qOVPTWKS1ZrK8");
let result = client.mlog_url(&query).await?;
```

### 将 mlog id 转为视频 id

说明 : 调用此接口 , 传入 mlog id, 可获取 video id，然后通过 `video_url` 获取播放地址

**必选参数 :** `id` : mlog id

**方法名 :** `mlog_to_video`

**调用例子 :**
```rust
let query = Query::new().param("id", "a1qOVPTWKS1ZrK8");
let result = client.mlog_to_video(&query).await?;
```

### 歌曲相关视频

说明 : 可以调用此接口获取歌曲相关视频 (区别于 MV)， 有些歌曲没有 MV 但是有用户上传的与此歌曲相关的 Mlog。此功能仅在网易云音乐 APP 上存在。请注意：此接口偶尔会在相关视频后返回不相关视频，请合理使用。

**必选参数 :** `songid` : 歌曲 ID

**可选参数 :** `mvid` : 如果定义，此 mvid 对应的 MV 将会作为第一个返回

`limit` : 取出的 Mlog 数量, 不包含第一个 mvid

**方法名 :** `mlog_music_rcmd`

**调用例子 :**
```rust
let query = Query::new().param("songid", "186016");
let result = client.mlog_music_rcmd(&query).await?;
```

---

## 电台相关

### dj_banner - 电台 banner

说明 : 调用此接口,可获取电台 banner

**方法名 :** `dj_banner`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.dj_banner(&query).await.unwrap();
```

---

### dj_personalize_recommend - 电台个性推荐

说明 : 调用此接口,可获取电台个性推荐列表

**可选参数 :** `limit` : 返回数量,默认为 6,总条数最多 6 条

**方法名 :** `dj_personalize_recommend`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "5");
let result = client.dj_personalize_recommend(&query).await.unwrap();
```

---

### dj_subscriber - 电台订阅者列表

说明 : 调用此接口,可获取电台订阅者列表

**必选参数 :** `id` : 电台 id

**可选参数 :** `time` : 分页参数,默认 -1 ; `limit` : 返回数量,默认为 20

**方法名 :** `dj_subscriber`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "335425050");
let result = client.dj_subscriber(&query).await.unwrap();

// 分页
let query = Query::new()
    .param("id", "335425050")
    .param("time", "1602761825390");
let result = client.dj_subscriber(&query).await.unwrap();
```

---

### dj_catelist - 电台分类

说明 : 登录后调用此接口,可获得电台类型

**方法名 :** `dj_catelist`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.dj_catelist(&query).await.unwrap();
```

---

### dj_category_excludehot - 电台非热门类型

说明 : 登录后调用此接口,可获得电台非热门类型

**方法名 :** `dj_category_excludehot`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.dj_category_excludehot(&query).await.unwrap();
```

---

### dj_category_recommend - 电台推荐类型

说明 : 登录后调用此接口,可获得电台推荐类型

**方法名 :** `dj_category_recommend`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.dj_category_recommend(&query).await.unwrap();
```

---

### dj_detail - 电台详情

说明 : 登录后调用此接口,传入 `rid`,可获得对应电台的详情介绍

**必选参数 :** `rid` : 电台的 id

**方法名 :** `dj_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("rid", "336355127");
let result = client.dj_detail(&query).await.unwrap();
```

---

### dj_hot - 热门电台

说明 : 调用此接口,可获取热门电台

**可选参数 :** `limit` : 返回数量,默认为 30 ; `offset` : 偏移数量,默认为 0

**方法名 :** `dj_hot`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.dj_hot(&query).await.unwrap();
```

---

### dj_radio_hot - 类别热门电台

说明 : 调用此接口,可获取类别热门电台

**可选参数 :** `limit` : 返回数量,默认为 30 ; `offset` : 偏移数量,默认为 0 ; `cateId` : 类别 id,可通过 `/dj/category/recommend` 接口获取

**方法名 :** `dj_radio_hot`

**Rust 调用例子 :**

```rust
let query = Query::new().param("cateId", "2001");
let result = client.dj_radio_hot(&query).await.unwrap();
```

---

### dj_program - 电台节目

说明 : 登录后调用此接口,传入 `rid`,可查看对应电台的电台节目以及对应的 id。注意此接口返回的 mp3Url 已经无效,需通过 `/song/url` 接口传入节目 mainTrackId 获取音频

**必选参数 :** `rid` : 电台的 id

**可选参数 :** `limit` : 返回数量,默认为 30 ; `offset` : 偏移数量,默认为 0 ; `asc` : 排序方式,默认为 `false` (新 => 老),设置 `true` 可改为 老 => 新

**方法名 :** `dj_program`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("rid", "336355127")
    .param("limit", "40");
let result = client.dj_program(&query).await.unwrap();
```

---

### dj_program_detail - 电台节目详情

说明 : 调用此接口传入电台节目 id,可获得电台节目详情

**必选参数 :** `id` : 电台节目的 id

**方法名 :** `dj_program_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "1367665101");
let result = client.dj_program_detail(&query).await.unwrap();
```

---

### dj_program_toplist - 电台节目榜

说明 : 登录后调用此接口,可获得电台节目榜

**可选参数 :** `limit` : 返回数量,默认为 100 ; `offset` : 偏移数量,默认为 0

**方法名 :** `dj_program_toplist`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.dj_program_toplist(&query).await.unwrap();
```

---

### dj_program_toplist_hours - 24 小时节目榜

说明 : 调用此接口,可获取 24 小时节目榜

**可选参数 :** `limit` : 返回数量,默认为 100 (不支持 offset)

**方法名 :** `dj_program_toplist_hours`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.dj_program_toplist_hours(&query).await.unwrap();
```

---

### dj_recommend - 电台推荐

说明 : 登录后调用此接口,可获得推荐电台

**方法名 :** `dj_recommend`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.dj_recommend(&query).await.unwrap();
```

---

### dj_recommend_type - 电台分类推荐

说明 : 登录后调用此接口,传入分类,可获得对应类型电台列表

**必选参数 :** `type` : 电台类型,数字,可通过 `dj_catelist` 获取,id 对应此接口的 type

**方法名 :** `dj_recommend_type`

**Rust 调用例子 :**

```rust
let query = Query::new().param("type", "1"); // 明星做主播
let result = client.dj_recommend_type(&query).await.unwrap();
```

---

### dj_sub - 电台订阅

说明 : 登录后调用此接口,传入 `rid`,可订阅/取消订阅电台

**必选参数 :** `rid` : 电台的 id ; `t` : 1 为订阅,0 为取消订阅

**方法名 :** `dj_sub`

**Rust 调用例子 :**

```rust
// 订阅
let query = Query::new()
    .param("rid", "336355127")
    .param("t", "1");
let result = client.dj_sub(&query).await.unwrap();

// 取消订阅
let query = Query::new()
    .param("rid", "336355127")
    .param("t", "0");
let result = client.dj_sub(&query).await.unwrap();
```

---

### dj_sublist - 电台订阅列表

说明 : 登录后调用此接口,可获取订阅的电台列表

**方法名 :** `dj_sublist`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.dj_sublist(&query).await.unwrap();
```

---

### dj_toplist - 新晋电台榜/热门电台榜

说明 : 登录后调用此接口,可获得新晋电台榜/热门电台榜

**可选参数 :** `limit` : 返回数量,默认为 100 ; `offset` : 偏移数量,默认为 0 ; `type` : 榜单类型,`new` 为新晋电台榜,`hot` 为热门电台榜

**方法名 :** `dj_toplist`

**Rust 调用例子 :**

```rust
let query = Query::new().param("type", "hot");
let result = client.dj_toplist(&query).await.unwrap();

let query = Query::new()
    .param("type", "new")
    .param("limit", "1");
let result = client.dj_toplist(&query).await.unwrap();
```

---

### dj_toplist_hours - 24 小时主播榜

说明 : 调用此接口,可获取 24 小时主播榜

**可选参数 :** `limit` : 返回数量,默认为 100 (不支持 offset)

**方法名 :** `dj_toplist_hours`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "30");
let result = client.dj_toplist_hours(&query).await.unwrap();
```

---

### dj_toplist_newcomer - 主播新人榜

说明 : 调用此接口,可获取主播新人榜

**可选参数 :** `limit` : 返回数量,默认为 100 (不支持 offset)

**方法名 :** `dj_toplist_newcomer`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "30");
let result = client.dj_toplist_newcomer(&query).await.unwrap();
```

---

### dj_toplist_pay - 付费精品电台

说明 : 调用此接口,可获取付费精品电台

**可选参数 :** `limit` : 返回数量,默认为 100 (不支持 offset)

**方法名 :** `dj_toplist_pay`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "30");
let result = client.dj_toplist_pay(&query).await.unwrap();
```

---

### dj_toplist_popular - 最热主播榜

说明 : 调用此接口,可获取最热主播榜

**可选参数 :** `limit` : 返回数量,默认为 100 (不支持 offset)

**方法名 :** `dj_toplist_popular`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "30");
let result = client.dj_toplist_popular(&query).await.unwrap();
```

---

### dj_paygift - 付费精选

说明 : 可以获取付费精选的电台列表

**可选参数 :** `limit` : 返回数量,默认为 30 ; `offset` : 偏移数量,默认为 0

**方法名 :** `dj_paygift`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("limit", "10")
    .param("offset", "20");
let result = client.dj_paygift(&query).await.unwrap();
```

---

### dj_today_perfered - 电台今日优选

说明 : 登录后调用此接口,可获得电台今日优选

**方法名 :** `dj_today_perfered`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.dj_today_perfered(&query).await.unwrap();
```

---

### dj_radio_top - 电台排行榜

说明 : 调用此接口可以获取电台排行榜

**可选参数 :** `djRadioId` : 电台 id ; `sortIndex` : 排序 1:播放数 2:点赞数 3:评论数 4:分享数 5:收藏数,默认 1 ; `dataGapDays` : 天数 7:一周 30:一个月 90:三个月,默认 7 ; `dataType` : 默认 3

**方法名 :** `dj_radio_top`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("djRadioId", "336355127")
    .param("sortIndex", "1")
    .param("dataGapDays", "7");
let result = client.dj_radio_top(&query).await.unwrap();
```

---

### dj_difm_all_style_channel - DIFM 电台分类

说明 : 获取 DIFM 电台的全部风格分类频道

**可选参数 :** `sources` : 来源数组字符串，默认 `[0]`

**方法名 :** `dj_difm_all_style_channel`

**Rust 调用例子 :**

```rust
let query = Query::new().param("sources", "[0]");
let result = client.dj_difm_all_style_channel(&query).await.unwrap();
```

---

### dj_difm_channel_subscribe - 收藏 DIFM 频道

说明 : 收藏 DIFM 电台频道，需要登录

**必选参数 :** `id` : 频道 id

**方法名 :** `dj_difm_channel_subscribe`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "123456");
let result = client.dj_difm_channel_subscribe(&query).await.unwrap();
```

---

### dj_difm_channel_unsubscribe - 取消收藏 DIFM 频道

说明 : 取消收藏 DIFM 电台频道，需要登录

**必选参数 :** `id` : 频道 id

**方法名 :** `dj_difm_channel_unsubscribe`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "123456");
let result = client.dj_difm_channel_unsubscribe(&query).await.unwrap();
```

---

### dj_difm_playing_tracks_list - DIFM 播放列表

说明 : 获取 DIFM 频道的播放列表

**必选参数 :** `channelId` : 频道 id

**可选参数 :** `limit` : 数量，默认 `5` ; `source` : 来源，默认 `0`

**方法名 :** `dj_difm_playing_tracks_list`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("channelId", "123456")
    .param("limit", "10");
let result = client.dj_difm_playing_tracks_list(&query).await.unwrap();
```

---

### dj_difm_subscribe_channels_get - DIFM 收藏列表

说明 : 获取已收藏的 DIFM 频道列表，需要登录

**可选参数 :** `sources` : 来源数组字符串，默认 `[0]`

**方法名 :** `dj_difm_subscribe_channels_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.dj_difm_subscribe_channels_get(&query).await.unwrap();
```

---

### radio_sport_get - 跑步漫游

说明 : 根据步频获取跑步漫游电台

**可选参数 :** `bpm` : 步频，默认 `50`

**方法名 :** `radio_sport_get`

**Rust 调用例子 :**

```rust
let query = Query::new().param("bpm", "120");
let result = client.radio_sport_get(&query).await.unwrap();
```

---


## 🎯 推荐相关

### personalized - 推荐歌单

说明 : 调用此接口,可获取推荐歌单

**可选参数 :** `limit` : 取出数量,默认为 30 (不支持 offset)

**方法名 :** `personalized`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.personalized(&query).await.unwrap();
```

---

### personalized_newsong - 推荐新音乐

说明 : 调用此接口,可获取推荐新音乐

**可选参数 :** `limit` : 取出数量,默认为 10 (不支持 offset)

**方法名 :** `personalized_newsong`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.personalized_newsong(&query).await.unwrap();
```

---

### personalized_djprogram - 推荐电台

说明 : 调用此接口,可获取推荐电台

**方法名 :** `personalized_djprogram`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.personalized_djprogram(&query).await.unwrap();
```

---

### personalized_privatecontent - 独家放送(入口列表)

说明 : 调用此接口,可获取独家放送

**方法名 :** `personalized_privatecontent`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.personalized_privatecontent(&query).await.unwrap();
```

---

### personalized_privatecontent_list - 独家放送列表

说明 : 调用此接口,可获取独家放送列表

**可选参数 :** `limit` : 返回数量,默认为 60 ; `offset` : 偏移数量,默认为 0

**方法名 :** `personalized_privatecontent_list`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("limit", "1")
    .param("offset", "2");
let result = client.personalized_privatecontent_list(&query).await.unwrap();
```

---

### recommend_songs - 每日推荐歌曲

说明 : 调用此接口,可获得每日推荐歌曲 (需要登录)

**方法名 :** `recommend_songs`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.recommend_songs(&query).await.unwrap();
```

---

### recommend_resource - 每日推荐歌单

说明 : 调用此接口,可获得每日推荐歌单 (需要登录)

**方法名 :** `recommend_resource`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.recommend_resource(&query).await.unwrap();
```

---

### recommend_songs_dislike - 每日推荐歌曲不感兴趣

说明 : 日推歌曲标记为不感兴趣 (同时会返回一个新推荐歌曲,需要登录)

**必选参数 :** `id` : 歌曲 id

**方法名 :** `recommend_songs_dislike`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "168091");
let result = client.recommend_songs_dislike(&query).await.unwrap();
```

---

### history_recommend_songs - 获取历史日推可用日期列表

说明 : 调用此接口,可获得历史日推可用日期列表

**方法名 :** `history_recommend_songs`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.history_recommend_songs(&query).await.unwrap();
```

---

### history_recommend_songs_detail - 获取历史日推详情数据

说明 : 调用此接口,传入当日日期,可获得当日历史日推数据

**必选参数 :** `date` : 日期,通过历史日推可用日期列表接口获取,不能任意日期

**方法名 :** `history_recommend_songs_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("date", "2020-06-21");
let result = client.history_recommend_songs_detail(&query).await.unwrap();
```

---

### program_recommend - 推荐节目

说明 : 调用此接口,可获取推荐电台节目

**可选参数 :** `limit` : 取出数量,默认为 10 ; `offset` : 偏移数量,默认为 0

**方法名 :** `program_recommend`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "5");
let result = client.program_recommend(&query).await.unwrap();
```

---

### homepage_block_page - 首页-发现

说明 : 调用此接口,可获取 APP 首页信息

**可选参数 :** `refresh` : 是否刷新数据,默认为 false ; `cursor` : 上一条数据返回的 cursor

**方法名 :** `homepage_block_page`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.homepage_block_page(&query).await.unwrap();
```

---

### homepage_dragon_ball - 首页圆形图标入口列表

说明 : 调用此接口,可获取 APP 首页圆形图标入口列表

**方法名 :** `homepage_dragon_ball`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.homepage_dragon_ball(&query).await.unwrap();
```

---

### banner - 首页 Banner

说明 : 调用此接口,可获取 banner(轮播图)数据

**可选参数 :** `type` : 资源类型, 0: pc, 1: android, 2: iphone, 3: ipad,默认为 0

**方法名 :** `banner`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.banner(&query).await.unwrap();

let query = Query::new().param("type", "2");
let result = client.banner(&query).await.unwrap();
```

---

### daily_signin - 签到

说明 : 调用此接口,可签到 (需要登录)。安卓端签到可获得 3 点经验,web/PC 端签到可获得 2 点经验

**可选参数 :** `type` : 签到类型,默认 0(安卓端签到),1 为 web/PC 签到

**方法名 :** `daily_signin`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.daily_signin(&query).await.unwrap();
```

---

### personal_fm - 私人 FM

说明 : 私人 FM (需要登录)

**方法名 :** `personal_fm`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.personal_fm(&query).await.unwrap();
```

---

### personal_fm_mode - 私人 FM 模式选择

说明 : 调用此接口返回私人 FM 内容,并可以选择模式

**必选参数 :** `mode` : 模式 (aidj, DEFAULT, FAMILIAR, EXPLORE, SCENE_RCMD)

**可选参数 :** `submode` : 当 mode 为 SCENE_RCMD 时可为 (EXERCISE, FOCUS, NIGHT_EMO)

**方法名 :** `personal_fm_mode`

**Rust 调用例子 :**

```rust
let query = Query::new().param("mode", "FAMILIAR");
let result = client.personal_fm_mode(&query).await.unwrap();
```

---

### fm_trash - 垃圾桶

说明 : 调用此接口,传入音乐 id,可把该音乐从私人 FM 中移除至垃圾桶

**必选参数 :** `id` : 歌曲 id

**方法名 :** `fm_trash`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "347230");
let result = client.fm_trash(&query).await.unwrap();
```

---

### playmode_intelligence_list - 心动模式/智能播放

说明 : 登录后调用此接口,可获取心动模式/智能播放列表

**必选参数 :** `id` : 歌曲 id ; `pid` : 歌单 id

**可选参数 :** `sid` : 要开始播放的歌曲的 id

**方法名 :** `playmode_intelligence_list`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("id", "33894312")
    .param("pid", "24381616");
let result = client.playmode_intelligence_list(&query).await.unwrap();

let query = Query::new()
    .param("id", "33894312")
    .param("pid", "24381616")
    .param("sid", "36871368");
let result = client.playmode_intelligence_list(&query).await.unwrap();
```

---

### playmode_song_vector - 歌曲向量

说明 : 调用此接口,可获取歌曲向量数据

**方法名 :** `playmode_song_vector`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.playmode_song_vector(&query).await.unwrap();
```

---

### simi_song - 相似歌曲

说明 : 调用此接口,传入歌曲 id,可获得相似歌曲

**必选参数 :** `id` : 歌曲 id

**方法名 :** `simi_song`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "347230");
let result = client.simi_song(&query).await.unwrap();
```

---

### simi_artist - 相似歌手

说明 : 调用此接口,传入歌手 id,可获得相似歌手

**必选参数 :** `id` : 歌手 id

**方法名 :** `simi_artist`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "6452");
let result = client.simi_artist(&query).await.unwrap();
```

---

### simi_playlist - 相似歌单

说明 : 调用此接口,传入歌曲 id,可获得相似歌单

**必选参数 :** `id` : 歌曲 id

**方法名 :** `simi_playlist`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "347230");
let result = client.simi_playlist(&query).await.unwrap();
```

---

### simi_mv - 相似 MV

说明 : 调用此接口,传入 mvid,可获取相似 MV

**必选参数 :** `mvid` : MV id

**方法名 :** `simi_mv`

**Rust 调用例子 :**

```rust
let query = Query::new().param("mvid", "5436712");
let result = client.simi_mv(&query).await.unwrap();
```

---

### simi_user - 最近听了这首歌的用户

说明 : 调用此接口,传入歌曲 id,可获取最近 5 个听了这首歌的用户

**必选参数 :** `id` : 歌曲 id

**方法名 :** `simi_user`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "347230");
let result = client.simi_user(&query).await.unwrap();
```

---

### recommend_category_configs - 每日风格推荐配置

说明 : 获取每日风格推荐的场景配置列表

**参数 :** 无

**方法名 :** `recommend_category_configs`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.recommend_category_configs(&query).await.unwrap();
```

---

### recommend_category_set - 设置每日风格推荐

说明 : 设置每日风格推荐的标签和场景

**必选参数 :** `tags` : 标签 id，多个用逗号分隔 ; `category` : 场景 id

**方法名 :** `recommend_category_set`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("tags", "1001,1002")
    .param("category", "1");
let result = client.recommend_category_set(&query).await.unwrap();
```

---

### recommend_category_songs - 每日风格推荐歌曲

说明 : 获取每日风格推荐歌曲

**参数 :** 无

**方法名 :** `recommend_category_songs`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.recommend_category_songs(&query).await.unwrap();
```

---

### recommend_songs_v1 - 每日推荐歌曲 v1

说明 : 新版每日推荐歌曲接口

**可选参数 :** `ispush` : 是否推送，默认 `false`

**方法名 :** `recommend_songs_v1`

**Rust 调用例子 :**

```rust
let query = Query::new().param("ispush", "true");
let result = client.recommend_songs_v1(&query).await.unwrap();
```

---


## 🏆 排行榜相关

### toplist - 所有榜单

说明 : 调用此接口,可获取所有榜单

**方法名 :** `toplist`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.toplist(&query).await.unwrap();
```

---

### toplist_detail - 所有榜单内容摘要

说明 : 调用此接口,可获取所有榜单内容摘要

**方法名 :** `toplist_detail`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.toplist_detail(&query).await.unwrap();
```

---

### toplist_detail_v2 - 所有榜单内容摘要 V2

说明 : 调用此接口,可获取所有榜单内容摘要(V2 版本)

**方法名 :** `toplist_detail_v2`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.toplist_detail_v2(&query).await.unwrap();
```

---

### toplist_artist - 歌手榜

说明 : 调用此接口,可获取排行榜中的歌手榜

**可选参数 :** `type` : 地区, 1: 华语, 2: 欧美, 3: 韩国, 4: 日本

**方法名 :** `toplist_artist`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.toplist_artist(&query).await.unwrap();
```

---

### top_song - 新歌速递

说明 : 调用此接口,可获取新歌速递

**必选参数 :** `type` : 地区类型 id, 全部:0, 华语:7, 欧美:96, 日本:8, 韩国:16

**方法名 :** `top_song`

**Rust 调用例子 :**

```rust
let query = Query::new().param("type", "96");
let result = client.top_song(&query).await.unwrap();
```

---

### top_album - 新碟上架

说明 : 调用此接口,可获取新碟上架列表

**可选参数 :** `area` : ALL/ZH/EA/KR/JP ; `type` : new(全部)/hot(热门),默认 new ; `year` : 年 ; `month` : 月 ; `limit` / `offset`

**方法名 :** `top_album`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("offset", "0")
    .param("limit", "30")
    .param("year", "2019")
    .param("month", "6");
let result = client.top_album(&query).await.unwrap();
```

---

### top_artists - 热门歌手

说明 : 调用此接口,可获取热门歌手数据

**可选参数 :** `limit` : 取出数量,默认为 50 ; `offset` : 偏移数量,默认为 0

**方法名 :** `top_artists`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("offset", "0")
    .param("limit", "30");
let result = client.top_artists(&query).await.unwrap();
```

---

### top_mv - MV 排行

说明 : 调用此接口,可获取 MV 排行

**可选参数 :** `limit` : 取出数量,默认为 30 ; `area` : 地区(内地/港台/欧美/日本/韩国) ; `offset` : 偏移数量

**方法名 :** `top_mv`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "10");
let result = client.top_mv(&query).await.unwrap();
```

---

### top_playlist - 歌单 (网友精选碟)

说明 : 调用此接口,可获取网友精选碟歌单

**可选参数 :** `order` : 可选值 'new' 和 'hot',默认 'hot' ; `cat` : tag,如"华语"/"古风"等,默认"全部" ; `limit` / `offset`

**方法名 :** `top_playlist`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("limit", "10")
    .param("order", "new");
let result = client.top_playlist(&query).await.unwrap();
```

---

### top_playlist_highquality - 获取精品歌单

说明 : 调用此接口,可获取精品歌单

**可选参数 :** `cat` : tag,如"华语"/"古风"等,默认"全部" ; `limit` : 取出歌单数量,默认为 50 ; `before` : 分页参数,取上一页最后一个歌单的 `updateTime` 获取下一页数据

**方法名 :** `top_playlist_highquality`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("before", "1503639064232")
    .param("limit", "3");
let result = client.top_playlist_highquality(&query).await.unwrap();
```

---

### top_list - 排行榜详情 (旧版)

说明 : 调用此接口,传入榜单 id,可获取不同排行榜数据。推荐使用歌单详情接口 `playlist_detail` 传入排行榜 id 获取排行榜详情

**必选参数 :** `id` : 榜单 id,通过所有榜单接口获取

**方法名 :** `top_list`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "2809577409");
let result = client.top_list(&query).await.unwrap();
```

---

### chart_detail - 排行榜详情

说明 : 获取指定维度音乐排行榜详情。该接口免登录

**必选参数 :** `chartCode` : 榜单编码

**可选参数 :** `targetId` : 目标 id ; `targetType` : 目标类型

**方法名 :** `chart_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("chartCode", "19723756");
let result = client.chart_detail(&query).await.unwrap();
```

---

### chart_song_detail - 排行榜歌曲列表

说明 : 获取指定维度音乐排行榜的歌曲列表。该接口免登录

**必选参数 :** `chartCode` : 榜单编码

**可选参数 :** `targetId` : 目标 id ; `targetType` : 目标类型

**方法名 :** `chart_song_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("chartCode", "19723756");
let result = client.chart_song_detail(&query).await.unwrap();
```

---


## 云盘相关

### user_cloud - 云盘数据

说明 : 登录后调用此接口,可获取云盘数据,获取的数据没有对应 url,需要再调用一次 `song_url_v1` 获取 url

**可选参数 :** `limit` : 返回数量,默认为 30 ; `offset` : 偏移数量,默认为 0

**方法名 :** `user_cloud`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.user_cloud(&query).await.unwrap();
```

---

### user_cloud_detail - 云盘数据详情

说明 : 登录后调用此接口,传入云盘歌曲 id,可获取云盘数据详情

**必选参数 :** `id` : 歌曲 id,可多个,用逗号隔开

**方法名 :** `user_cloud_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "5374627");
let result = client.user_cloud_detail(&query).await.unwrap();
```

---

### user_cloud_del - 云盘歌曲删除

说明 : 登录后调用此接口,可删除云盘歌曲

**必选参数 :** `id` : 歌曲 id,可多个,用逗号隔开

**方法名 :** `user_cloud_del`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "5374627");
let result = client.user_cloud_del(&query).await.unwrap();
```

---

### cloud - 云盘上传

说明 : 登录后调用此接口,使用 `multipart/form-data` 上传 mp3,可上传歌曲到云盘

**方法名 :** `cloud`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.cloud(&query).await.unwrap();
```

---

### cloud_import - 云盘导入歌曲

说明 : 登录后调用此接口,可云盘导入歌曲而无需上传文件。以下情况可导入成功: 1.文件已经有用户上传至云盘 2.文件是网易云音乐自己的音源。

接口会先调用云盘检查接口取得 `songId`，再执行导入，调用方不需要自行先查询 `songId`。

**必选参数 :** `song` : 歌名/文件名 ; `fileType` : 文件后缀 ; `fileSize` : 文件大小 ; `bitrate` : 文件比特率 ; `md5` : 文件 MD5

**可选参数 :** `id` : 歌曲 ID(情况 2 时必须正确填写) ; `artist` : 歌手,默认为未知 ; `album` : 专辑,默认为未知

**方法名 :** `cloud_import`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("song", "最伟大的作品")
    .param("artist", "周杰伦")
    .param("album", "最伟大的作品")
    .param("fileType", "flac")
    .param("fileSize", "50412168")
    .param("bitrate", "1652")
    .param("md5", "d02b8ab79d91c01167ba31e349fe5275");
let result = client.cloud_import(&query).await.unwrap();
```

---

### cloud_match - 云盘歌曲信息匹配纠正

说明 : 登录后调用此接口,可对云盘歌曲信息匹配纠正,如需取消匹配,asid 需要传 0

**必选参数 :** `uid` : 用户 id ; `sid` : 云盘的歌曲 id ; `asid` : 要匹配的歌曲 id

**方法名 :** `cloud_match`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("uid", "32953014")
    .param("sid", "aaa")
    .param("asid", "bbb");
let result = client.cloud_match(&query).await.unwrap();

// 取消匹配
let query = Query::new()
    .param("uid", "32953014")
    .param("sid", "bbb")
    .param("asid", "0");
let result = client.cloud_match(&query).await.unwrap();
```

---

### cloud_lyric_get - 获取云盘歌词

说明 : 调用此接口,获取云盘歌曲的歌词,歌词来自此文件的音乐元数据 LYRICS 标签

**可选参数 :** `uid` : 用户 id ; `sid` : 云盘的歌曲 id

**方法名 :** `cloud_lyric_get`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("uid", "1")
    .param("sid", "aaa");
let result = client.cloud_lyric_get(&query).await.unwrap();
```

---

### cloud_upload_token - 获取上传凭证

说明 : 客户端直传模式下,调用此接口获取上传凭证

**必选参数 :** `md5` : 文件 MD5 值 ; `fileSize` : 文件大小(字节) ; `filename` : 文件名

**方法名 :** `cloud_upload_token`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("md5", "d02b8ab79d91c01167ba31e349fe5275")
    .param("fileSize", "50412168")
    .param("filename", "song.mp3");
let result = client.cloud_upload_token(&query).await.unwrap();
```

---

### cloud_upload_complete - 完成上传导入

说明 : 客户端直传模式下,上传文件后调用此接口完成导入

**必选参数 :** `songId` : 歌曲 ID ; `resourceId` : 资源 ID ; `md5` : 文件 MD5 ; `filename` : 文件名

**可选参数 :** `song` : 歌曲名 ; `artist` : 艺术家 ; `album` : 专辑名

**方法名 :** `cloud_upload_complete`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("songId", "123456")
    .param("resourceId", "res_123")
    .param("md5", "d02b8ab79d91c01167ba31e349fe5275")
    .param("filename", "song.mp3")
    .param("song", "歌曲名")
    .param("artist", "歌手名");
let result = client.cloud_upload_complete(&query).await.unwrap();
```

---

### song_cloud_download - 云盘歌曲下载链接

说明 : 从云盘获取歌曲下载链接

**必选参数 :** `id` : 云盘歌曲 id

**方法名 :** `song_cloud_download`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "123456");
let result = client.song_cloud_download(&query).await.unwrap();
```

---


## 💌 私信相关

### msg_private - 通知-私信

说明 : 登录后调用此接口,可获取私信

**可选参数 :** `limit` : 返回数量,默认为 30 ; `offset` : 偏移数量,默认为 0

**方法名 :** `msg_private`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "3");
let result = client.msg_private(&query).await.unwrap();
```

---

### msg_private_history - 私信内容

说明 : 登录后调用此接口,可获取私信内容

**必选参数 :** `uid` : 用户 id

**可选参数 :** `limit` : 返回数量,默认为 30 ; `before` : 分页参数,取上一页最后一项的 `time` 获取下一页数据

**方法名 :** `msg_private_history`

**Rust 调用例子 :**

```rust
let query = Query::new().param("uid", "9003"); // 云音乐小秘书
let result = client.msg_private_history(&query).await.unwrap();
```

---

### msg_comments - 通知-评论

说明 : 登录后调用此接口,可获取评论通知

**必选参数 :** `uid` : 用户的 id,只能和登录账号的 id 一致

**可选参数 :** `limit` : 返回数量,默认为 30 ; `before` : 分页参数,取上一页最后一个的 `updateTime` 获取下一页数据

**方法名 :** `msg_comments`

**Rust 调用例子 :**

```rust
let query = Query::new().param("uid", "32953014");
let result = client.msg_comments(&query).await.unwrap();
```

---

### msg_forwards - 通知-@我

说明 : 登录后调用此接口,可获取 @我 数据

**可选参数 :** `limit` : 返回数量,默认为 30 ; `offset` : 偏移数量,默认为 0

**方法名 :** `msg_forwards`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "3");
let result = client.msg_forwards(&query).await.unwrap();
```

---

### msg_notices - 通知-通知

说明 : 登录后调用此接口,可获取通知

**可选参数 :** `limit` : 返回数量,默认为 30 ; `lasttime` : 返回数据的 `time`,默认 -1,传入上一次返回结果的 time 将会返回下一页的数据

**方法名 :** `msg_notices`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "3");
let result = client.msg_notices(&query).await.unwrap();
```

---

### msg_recentcontact - 最近联系人

说明 : 登录后调用此接口,可获取最近联系人

**方法名 :** `msg_recentcontact`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.msg_recentcontact(&query).await.unwrap();
```

---

### send_text - 发送私信

说明 : 登录后调用此接口,传入用户 id 和要发送的信息,可以发送私信 (注:不能发送私信给自己)

**必选参数 :** `user_ids` : 用户 id,多个需用逗号隔开 ; `msg` : 要发送的信息

**方法名 :** `send_text`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("user_ids", "32953014")
    .param("msg", "test");
let result = client.send_text(&query).await.unwrap();
```

---

### send_song - 发送私信(带歌曲)

说明 : 登录后调用此接口,传入用户 id、信息和音乐 id,可以发送音乐私信

**必选参数 :** `user_ids` : 用户 id ; `id` : 音乐 id ; `msg` : 要发送的信息

**方法名 :** `send_song`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("user_ids", "1")
    .param("id", "351318")
    .param("msg", "测试");
let result = client.send_song(&query).await.unwrap();
```

---

### send_playlist - 发送私信(带歌单)

说明 : 登录后调用此接口,传入用户 id、信息和歌单 id,可以发送带歌单的私信 (注:不能发送重复的歌单)

**必选参数 :** `user_ids` : 用户 id ; `msg` : 要发送的信息 ; `playlist` : 歌单 id

**方法名 :** `send_playlist`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("msg", "test")
    .param("user_ids", "475625142")
    .param("playlist", "705123491");
let result = client.send_playlist(&query).await.unwrap();
```

---

### send_album - 发送私信(带专辑)

说明 : 登录后调用此接口,传入用户 id、信息和专辑 id,可以发送专辑私信

**必选参数 :** `user_ids` : 用户 id ; `id` : 专辑 id ; `msg` : 要发送的信息

**方法名 :** `send_album`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("user_ids", "1")
    .param("id", "351318")
    .param("msg", "测试");
let result = client.send_album(&query).await.unwrap();
```

---

### share_resource - 分享到动态

说明 : 登录后调用此接口,可以分享文本、歌曲、歌单、mv、电台、电台节目、专辑到动态

**必选参数 :** `id` : 资源 id

**可选参数 :** `type` : 资源类型,默认 song,可传 `song`/`playlist`/`mv`/`djradio`/`djprogram`/`album` ; `msg` : 内容,140 字限制

**方法名 :** `share_resource`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("id", "1297494209")
    .param("msg", "测试");
let result = client.share_resource(&query).await.unwrap();
```

---

### resource_like - 资源点赞

说明 : 调用此接口,可对 MV、电台、视频等资源点赞

**必选参数 :** `type` : 资源类型 (0:歌曲, 1:mv, 2:歌单, 3:专辑, 4:电台节目, 5:视频, 6:动态, 7:电台) ; `t` : 1 为点赞,其他为取消 ; `id` : 资源 id

**方法名 :** `resource_like`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("t", "1")
    .param("type", "1")
    .param("id", "5436712");
let result = client.resource_like(&query).await.unwrap();
```

---

### event - 获取动态列表

说明 : 调用此接口,可获取各种动态

**可选参数 :** `pagesize` : 每页数据,默认 20 ; `lasttime` : 默认 -1,传入上一次返回结果的 lasttime 将会返回下一页的数据

**方法名 :** `event`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("pagesize", "30")
    .param("lasttime", "1556740526369");
let result = client.event(&query).await.unwrap();
```

---

### event_del - 删除用户动态

说明 : 登录后调用此接口,可以删除用户动态

**必选参数 :** `evId` : 动态 id

**方法名 :** `event_del`

**Rust 调用例子 :**

```rust
let query = Query::new().param("evId", "6712917601");
let result = client.event_del(&query).await.unwrap();
```

---

### event_forward - 转发用户动态

说明 : 登录后调用此接口,可以转发用户动态

**必选参数 :** `uid` : 用户 id ; `evId` : 动态 id ; `forwards` : 转发的评论

**方法名 :** `event_forward`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("evId", "6712917601")
    .param("uid", "32953014")
    .param("forwards", "测试内容");
let result = client.event_forward(&query).await.unwrap();
```

---

## 👑 VIP/会员

### vip_info - 获取 VIP 信息

说明 : 登录后调用此接口,可获取当前 VIP 信息

**可选参数 :** `uid` : 用户 id

**方法名 :** `vip_info`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.vip_info(&query).await.unwrap();

let query = Query::new().param("uid", "32953014");
let result = client.vip_info(&query).await.unwrap();
```

---

### vip_info_v2 - 获取 VIP 信息(app 端)

说明 : 登录后调用此接口,可获取当前 VIP 信息 (app 端)

**可选参数 :** `uid` : 用户 id

**方法名 :** `vip_info_v2`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.vip_info_v2(&query).await.unwrap();
```

---

### vip_sign - 黑胶乐签打卡

说明 : 登录后调用此接口,进行黑胶乐签打卡。返回体包含 `taskSign`、`checkinDetail`、`signed`、`message`，两个上游接口都返回 `code=200` 时 `signed=true`。

**方法名 :** `vip_sign`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.vip_sign(&query).await.unwrap();
```

---

### vip_sign_info - 黑胶乐签打卡信息

说明 : 登录后调用此接口,获取黑胶乐签打卡信息

**方法名 :** `vip_sign_info`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.vip_sign_info(&query).await.unwrap();
```

---

### vip_tasks - VIP 任务

说明 : 登录后调用此接口,可获取会员任务

**方法名 :** `vip_tasks`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.vip_tasks(&query).await.unwrap();
```

---

### vip_timemachine - 黑胶时光机

说明 : 调用此接口,可获得黑胶时光机数据

**可选参数 :** `startTime` : 开始时间 ; `endTime` : 结束时间 ; `limit` : 返回数量,默认为 60

**方法名 :** `vip_timemachine`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.vip_timemachine(&query).await.unwrap();

// 获取 2021 年 12 月数据
let query = Query::new()
    .param("startTime", "1638288000000")
    .param("endTime", "1640966399999")
    .param("limit", "10");
let result = client.vip_timemachine(&query).await.unwrap();
```

---

### vip_growthpoint - VIP 成长值

说明 : 登录后调用此接口,可获取当前会员成长值

**方法名 :** `vip_growthpoint`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.vip_growthpoint(&query).await.unwrap();
```

---

### vip_growthpoint_details - VIP 成长值获取记录

说明 : 登录后调用此接口,可获取会员成长值领取记录

**可选参数 :** `limit` : 取出数量,默认为 20 ; `offset` : 偏移数量

**方法名 :** `vip_growthpoint_details`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "10");
let result = client.vip_growthpoint_details(&query).await.unwrap();
```

---

### vip_growthpoint_get - 领取 VIP 成长值

说明 : 登录后调用此接口,可获取已完成的会员任务的成长值奖励

**必选参数 :** `ids` : 通过 `vip_tasks` 获取到的 `unGetIds`

**方法名 :** `vip_growthpoint_get`

**Rust 调用例子 :**

```rust
let query = Query::new().param("ids", "7043206830_7");
let result = client.vip_growthpoint_get(&query).await.unwrap();
```

---

### vip_growthpoint_getall - 一键领取会员成长值

说明 : 登录后一键领取所有会员成长值任务奖励

**参数 :** 无

**方法名 :** `vip_growthpoint_getall`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.vip_growthpoint_getall(&query).await.unwrap();
```

---

### vip_sign_history - 黑胶乐签历史/状态

说明 : 获取黑胶乐签打卡历史或状态信息

**可选参数 :** `type` : `0` 用户信息栏、`1` 黑胶乐签，默认 `0`

**方法名 :** `vip_sign_history`

**Rust 调用例子 :**

```rust
let query = Query::new().param("type", "1");
let result = client.vip_sign_history(&query).await.unwrap();
```

---

### vip_sign_detail - 黑胶乐签打卡详情

说明 : 获取黑胶乐签打卡详情

**可选参数 :** `timestamp` : 打卡日期时间戳，对应请求中的 `signDayTime`

**方法名 :** `vip_sign_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("timestamp", "1757000000000");
let result = client.vip_sign_detail(&query).await.unwrap();
```

---

### vip_tasks_v1 - 会员任务（新版）

说明 : 登录后获取新版会员任务中心的任务进度

**可选参数 :** `id` : 用户 id，对应请求中的 `userId`

**方法名 :** `vip_tasks_v1`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.vip_tasks_v1(&query).await.unwrap();
```

---


## 🌟 云贝

### yunbei - 云贝签到信息

说明 : 登录后调用此接口可获取云贝签到信息(连续签到天数,第二天全部可获得的云贝)

**方法名 :** `yunbei`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.yunbei(&query).await.unwrap();
```

---

### yunbei_info - 云贝账户信息

说明 : 登录后调用此接口可获取云贝账户信息(账户云贝数)

**方法名 :** `yunbei_info`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.yunbei_info(&query).await.unwrap();
```

---

### yunbei_sign - 云贝签到

说明 : 登录后调用此接口可进行云贝签到

**方法名 :** `yunbei_sign`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.yunbei_sign(&query).await.unwrap();
```

---

### yunbei_today - 云贝今日签到信息

说明 : 登录后调用此接口可获取云贝今日签到信息(今日签到获取的云贝数)

**方法名 :** `yunbei_today`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.yunbei_today(&query).await.unwrap();
```

---

### yunbei_expense - 云贝支出

说明 : 登录后调用此接口可获取云贝支出

**可选参数 :** `limit` : 取出数量,默认为 10 ; `offset` : 偏移数量

**方法名 :** `yunbei_expense`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.yunbei_expense(&query).await.unwrap();
```

---

### yunbei_receipt - 云贝收入

说明 : 登录后调用此接口可获取云贝收入

**可选参数 :** `limit` : 取出数量,默认为 10 ; `offset` : 偏移数量

**方法名 :** `yunbei_receipt`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.yunbei_receipt(&query).await.unwrap();
```

---

### yunbei_tasks - 云贝所有任务

说明 : 登录后调用此接口可获取云贝所有任务

**方法名 :** `yunbei_tasks`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.yunbei_tasks(&query).await.unwrap();
```

---

### yunbei_tasks_todo - 云贝 todo 任务

说明 : 登录后调用此接口可获取云贝 todo 任务

**方法名 :** `yunbei_tasks_todo`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.yunbei_tasks_todo(&query).await.unwrap();
```

---

### yunbei_task_finish - 云贝完成任务

说明 : 调用此接口完成云贝任务

**必选参数 :** `userTaskId` : 任务 id

**可选参数 :** `depositCode` : 任务 depositCode

**方法名 :** `yunbei_task_finish`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("userTaskId", "5146243240")
    .param("depositCode", "0");
let result = client.yunbei_task_finish(&query).await.unwrap();
```

---

### yunbei_rcmd_song - 云贝推歌

说明 : 登录后调用此接口,传入歌曲 id,可以进行云贝推歌

**必选参数 :** `id` : 歌曲 id

**可选参数 :** `reason` : 推歌理由 ; `yunbeiNum` : 云贝数量,默认 10

**方法名 :** `yunbei_rcmd_song`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "65528");
let result = client.yunbei_rcmd_song(&query).await.unwrap();

let query = Query::new()
    .param("id", "65528")
    .param("reason", "人间好声音推荐给你听");
let result = client.yunbei_rcmd_song(&query).await.unwrap();
```

---

### yunbei_rcmd_song_history - 云贝推歌历史记录

说明 : 登录后调用此接口,可以获得云贝推歌历史记录

**可选参数 :** `size` : 返回数量,默认为 20 ; `cursor` : 传入上一次返回结果的 cursor,将会返回下一页的数据

**方法名 :** `yunbei_rcmd_song_history`

**Rust 调用例子 :**

```rust
let query = Query::new().param("size", "10");
let result = client.yunbei_rcmd_song_history(&query).await.unwrap();
```

---

### yunbei_task_list_v1 - 云贝广告任务状态

说明 : 查询今日云贝广告任务状态，需要登录

**参数 :** 无

**方法名 :** `yunbei_task_list_v1`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.yunbei_task_list_v1(&query).await.unwrap();
```

---

### yunbei_task_recommend_song - 云贝任务推荐歌曲

说明 : 获取云贝广告任务推荐歌曲

**可选参数 :** `offset` : 偏移量，默认 `0` ; `limit` : 数量，默认 `10`

**方法名 :** `yunbei_task_recommend_song`

**Rust 调用例子 :**

```rust
let query = Query::new().param("offset", "0").param("limit", "20");
let result = client.yunbei_task_recommend_song(&query).await.unwrap();
```

---

### yunbei_task_finish_v1 - 完成云贝广告任务

说明 : 完成云贝广告任务并领取云贝，单日最多 10 次

**可选参数 :** `yunbeiAmount` : 单次可得云贝，默认 `150`

**方法名 :** `yunbei_task_finish_v1`

**Rust 调用例子 :**

```rust
let query = Query::new().param("yunbeiAmount", "150");
let result = client.yunbei_task_finish_v1(&query).await.unwrap();
```

---


## 👣 听歌足迹

### listen_data_year_report - 年度听歌足迹

说明 : 登录后调用此接口,获取年度听歌足迹

**方法名 :** `listen_data_year_report`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listen_data_year_report(&query).await.unwrap();
```

---

### listen_data_today_song - 今日收听

说明 : 登录后调用此接口,获取今日收听

**方法名 :** `listen_data_today_song`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listen_data_today_song(&query).await.unwrap();
```

---

### listen_data_total - 总收听时长

说明 : 登录后调用此接口,获取总收听时长;相关接口可能需要 VIP 权限

**方法名 :** `listen_data_total`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listen_data_total(&query).await.unwrap();
```

---

### listen_data_realtime_report - 本周/本月收听时长

说明 : 登录后调用此接口,获取本周/本月收听时长

**必选参数 :** `type` : 维度类型,周 `week`,月 `month`;今年没结束不支持今年的数据

**方法名 :** `listen_data_realtime_report`

**Rust 调用例子 :**

```rust
let query = Query::new().param("type", "month");
let result = client.listen_data_realtime_report(&query).await.unwrap();
```

---

### listen_data_report - 周/月/年收听报告

说明 : 登录后调用此接口,获取周/月/年收听报告

**必选参数 :** `type` : 维度类型,周 `week`,月 `month`,年 `year`

**可选参数 :** `endTime` : 周:每周周六 0 点的时间戳 ; 月:每月最后一天 0 点的时间戳 ; 年:每年最后一天 0 点的时间戳。不填就是本周/月的

**方法名 :** `listen_data_report`

**Rust 调用例子 :**

```rust
let query = Query::new().param("type", "month");
let result = client.listen_data_report(&query).await.unwrap();
```

---

### recent_listen_list - 最近听歌列表

说明 : 调用后可获取最近听歌列表

**方法名 :** `recent_listen_list`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.recent_listen_list(&query).await.unwrap();
```

---

### record_recent_song - 最近播放-歌曲

说明 : 调用此接口,可获得最近播放-歌曲

**可选参数 :** `limit` : 返回数量,默认为 100

**方法名 :** `record_recent_song`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.record_recent_song(&query).await.unwrap();
```

---

### record_recent_album - 最近播放-专辑

说明 : 调用此接口,可获得最近播放-专辑

**可选参数 :** `limit` : 返回数量,默认为 100

**方法名 :** `record_recent_album`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.record_recent_album(&query).await.unwrap();
```

---

### record_recent_playlist - 最近播放-歌单

说明 : 调用此接口,可获得最近播放-歌单

**可选参数 :** `limit` : 返回数量,默认为 100

**方法名 :** `record_recent_playlist`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.record_recent_playlist(&query).await.unwrap();
```

---

### record_recent_dj - 最近播放-播客

说明 : 调用此接口,可获得最近播放-播客

**可选参数 :** `limit` : 返回数量,默认为 100

**方法名 :** `record_recent_dj`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.record_recent_dj(&query).await.unwrap();
```

---

### record_recent_video - 最近播放-视频

说明 : 调用此接口,可获得最近播放-视频

**可选参数 :** `limit` : 返回数量,默认为 100

**方法名 :** `record_recent_video`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.record_recent_video(&query).await.unwrap();
```

---

### record_recent_voice - 最近播放-声音

说明 : 调用此接口,可获得最近播放-声音

**可选参数 :** `limit` : 返回数量,默认为 100

**方法名 :** `record_recent_voice`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "1");
let result = client.record_recent_voice(&query).await.unwrap();
```

---

### listen_data_song_play_rank - 歌曲播放排行

说明 : 获取听歌足迹中的歌曲播放排行（Top 20）

**可选参数 :** `type` : `week` 或 `month`，默认 `month` ; `endTime` : 截止时间，不传为本周/本月

**方法名 :** `listen_data_song_play_rank`

**Rust 调用例子 :**

```rust
let query = Query::new().param("type", "week");
let result = client.listen_data_song_play_rank(&query).await.unwrap();
```

---


## 🎨 风格/曲风

### style_list - 曲风列表

说明 : 调用此接口获取曲风列表及其对应的 `tagId`

**方法名 :** `style_list`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.style_list(&query).await.unwrap();
```

---

### style_detail - 曲风详情

说明 : 调用此接口可以获取该曲风的描述信息

**必选参数 :** `tagId` : 曲风 ID

**方法名 :** `style_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("tagId", "1000");
let result = client.style_detail(&query).await.unwrap();
```

---

### style_song - 曲风-歌曲

说明 : 调用此接口可以获取该曲风对应的歌曲

**必选参数 :** `tagId` : 曲风 ID

**可选参数 :** `size` : 返回数量,默认为 20 ; `cursor` : 分页 cursor,默认为 0 ; `sort` : 排序方式,0:按热度排序,1:按时间排序

**方法名 :** `style_song`

**Rust 调用例子 :**

```rust
let query = Query::new().param("tagId", "1000");
let result = client.style_song(&query).await.unwrap();

let query = Query::new()
    .param("tagId", "1010")
    .param("sort", "1");
let result = client.style_song(&query).await.unwrap();
```

---

### style_album - 曲风-专辑

说明 : 调用此接口可以获取该曲风对应的专辑

**必选参数 :** `tagId` : 曲风 ID

**可选参数 :** `size` : 返回数量,默认为 20 ; `cursor` : 分页 cursor,默认为 0 ; `sort` : 排序方式,0:按热度排序,1:按时间排序

**方法名 :** `style_album`

**Rust 调用例子 :**

```rust
let query = Query::new().param("tagId", "1000");
let result = client.style_album(&query).await.unwrap();
```

---

### style_artist - 曲风-歌手

说明 : 调用此接口可以获取该曲风对应的歌手

**必选参数 :** `tagId` : 曲风 ID

**可选参数 :** `size` : 返回数量,默认为 20 ; `cursor` : 分页 cursor,默认为 0

**方法名 :** `style_artist`

**Rust 调用例子 :**

```rust
let query = Query::new().param("tagId", "1000");
let result = client.style_artist(&query).await.unwrap();
```

---

### style_playlist - 曲风-歌单

说明 : 调用此接口可以获取该曲风对应的歌单

**必选参数 :** `tagId` : 曲风 ID

**可选参数 :** `size` : 返回数量,默认为 20 ; `cursor` : 分页 cursor,默认为 0

**方法名 :** `style_playlist`

**Rust 调用例子 :**

```rust
let query = Query::new().param("tagId", "1000");
let result = client.style_playlist(&query).await.unwrap();
```

---

### style_preference - 曲风偏好

说明 : 登录后调用此接口获取我的曲风偏好

**方法名 :** `style_preference`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.style_preference(&query).await.unwrap();
```

---

## 💿 数字专辑

### digital_album_detail - 数字专辑详情

说明 : 调用此接口,传入专辑 id,可获取数字专辑信息

**必选参数 :** `id` : 专辑 id

**方法名 :** `digital_album_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "120605500");
let result = client.digital_album_detail(&query).await.unwrap();
```

---

### digital_album_ordering - 购买数字专辑

说明 : 登录后调用此接口,可获取购买数字专辑的地址,把地址生成二维码后可扫描购买专辑

**必选参数 :** `id` : 专辑的 id ; `payment` : 支付方式,0 为支付宝,3 为微信 ; `quantity` : 购买的数量

**方法名 :** `digital_album_ordering`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("id", "86286082")
    .param("payment", "3")
    .param("quantity", "1");
let result = client.digital_album_ordering(&query).await.unwrap();
```

---

### digital_album_purchased - 我的数字专辑

说明 : 登录后调用此接口,可获取我的数字专辑

**可选参数 :** `limit` / `offset`

**方法名 :** `digital_album_purchased`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "10");
let result = client.digital_album_purchased(&query).await.unwrap();
```

---

### digital_album_sales - 数字专辑销量

说明 : 调用此接口,传入专辑 id,可获取数字专辑销量

**必选参数 :** `ids` : 专辑 id,支持多个,用 `,` 隔开

**方法名 :** `digital_album_sales`

**Rust 调用例子 :**

```rust
let query = Query::new().param("ids", "120605500");
let result = client.digital_album_sales(&query).await.unwrap();

// 多个专辑
let query = Query::new().param("ids", "120605500,125080528");
let result = client.digital_album_sales(&query).await.unwrap();
```

---

## 🎙️ 声音/播客

### voice_upload - 播客上传声音

说明 : 可以上传声音到播客

**必选参数 :** `voiceListId` : 播客 id ; `coverImgId` : 播客封面 id ; `categoryId` : 分类 id ; `secondCategoryId` : 次级分类 id ; `description` : 声音介绍

**可选参数 :** `songName` : 声音名称 ; `privacy` : 设为隐私声音 ; `publishTime` : 定时发布时间戳 ; `autoPublish` : 是否发布动态(传入 1) ; `autoPublishText` : 动态文案 ; `orderNo` : 排序,默认为 1 ; `composedSongs` : 包含歌曲(歌曲 id),多个用逗号隔开 ; `img_name` / `img_mimetype` : 封面文件名 / MIME 类型，配合 `cover_img` 参数使用

**方法名 :** `voice_upload`

注意：`voice_upload` 需要额外传入音频文件二进制和可选的封面图片二进制。传入 `cover_img` 时会先上传封面并用返回的图片 id 覆盖 `coverImgId`（对应 Node.js 版的 `imgFile`）。

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("voiceListId", "123456")
    .param("coverImgId", "789")
    .param("categoryId", "1")
    .param("secondCategoryId", "2")
    .param("description", "声音介绍");
let audio = std::fs::read("song.mp3")?;
let cover = std::fs::read("cover.jpg").ok();
let result = client
    .voice_upload(&query, "song.mp3", audio, Some("audio/mpeg"), cover)
    .await?;
```

---

### voice_delete - 播客删除

说明 : 可以删除播客声音

**必选参数 :** `ids` : 播客 id(voiceListId),多个以逗号隔开

**方法名 :** `voice_delete`

**Rust 调用例子 :**

```rust
let query = Query::new().param("ids", "123456");
let result = client.voice_delete(&query).await.unwrap();
```

---

### voice_detail - 播客声音详情

说明 : 获取播客里的声音详情

**必选参数 :** `id` : 播客声音 id(voiceId)

**方法名 :** `voice_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "123456");
let result = client.voice_detail(&query).await.unwrap();
```

---

### voice_lyric - 获取声音歌词

说明 : 调用此接口可以获取声音歌词

**必选参数 :** `id` : 声音 id

**方法名 :** `voice_lyric`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "123456");
let result = client.voice_lyric(&query).await.unwrap();
```

---

### voicelist_list - 播客声音列表

说明 : 可以获取播客里的声音

**必选参数 :** `voiceListId` : 播客 id

**可选参数 :** `limit` : 取出数量,默认为 200 ; `offset` : 偏移数量

**方法名 :** `voicelist_list`

**Rust 调用例子 :**

```rust
let query = Query::new().param("voiceListId", "123456");
let result = client.voicelist_list(&query).await.unwrap();
```

---

### voicelist_detail - 播客列表详情

说明 : 可以获取播客封面、分类、名称、简介等

**必选参数 :** `id` : 播客 id(voiceListId)

**方法名 :** `voicelist_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "123456");
let result = client.voicelist_detail(&query).await.unwrap();
```

---

### voicelist_search - 播客列表搜索

说明 : 可以获取播客列表

**可选参数 :** `limit` : 取出数量,默认为 200 ; `offset` : 偏移数量 ; `podcastName` : 播客名称

**方法名 :** `voicelist_search`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.voicelist_search(&query).await.unwrap();
```

---

### voicelist_list_search - 播客声音搜索

说明 : 可以搜索播客里的声音

**可选参数 :** `displayStatus` : 状态(ONLINE/AUDITING/ONLY_SELF_SEE/SCHEDULE_PUBLISH 等) ; `limit` : 每次返回数量,默认 20(最多 200) ; `name` : 搜索关键词 ; `offset` : 偏移量 ; `radioId` : 播客 id ; `type` : PUBLIC/PRIVATE ; `voiceFeeType` : -1:全部, 0:免费, 1:收费

**方法名 :** `voicelist_list_search`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("displayStatus", "ONLINE")
    .param("limit", "20");
let result = client.voicelist_list_search(&query).await.unwrap();
```

---

### voicelist_trans - 播客声音排序

说明 : 调整声音在列表中的顺序

**必选参数 :** `position` : 位置,最小为 1 ; `programId` : 播客声音 id(voiceId) ; `radioId` : 电台 id(voiceListId)

**可选参数 :** `limit` / `offset`

**方法名 :** `voicelist_trans`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("position", "1")
    .param("programId", "123")
    .param("radioId", "456");
let result = client.voicelist_trans(&query).await.unwrap();
```

---

### voicelist_my_created - 我创建的播客声音

说明 : 获取当前登录用户创建的播客声音列表

**可选参数 :** `limit` : 数量，默认 `20`

**方法名 :** `voicelist_my_created`

**Rust 调用例子 :**

```rust
let query = Query::new().param("limit", "20");
let result = client.voicelist_my_created(&query).await.unwrap();
```

---


## 🧘 助眠解压

### sati_tag_list - 标签列表

说明 : 获取助眠解压的标签列表

**参数 :** 无

**方法名 :** `sati_tag_list`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.sati_tag_list(&query).await.unwrap();
```

---

### sati_resource_list - 标签下资源列表

说明 : 获取指定标签下的助眠解压资源列表

**可选参数 :** `tag` : 标签 id

**方法名 :** `sati_resource_list`

**Rust 调用例子 :**

```rust
let query = Query::new().param("tag", "123");
let result = client.sati_resource_list(&query).await.unwrap();
```

---

### sati_resource_list_more - 同类推荐

说明 : 获取某个助眠解压资源的同类推荐

**必选参数 :** `id` : 资源 id

**方法名 :** `sati_resource_list_more`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "123456");
let result = client.sati_resource_list_more(&query).await.unwrap();
```

---

### sati_resource_sub - 收藏/取消收藏资源

说明 : 收藏或取消收藏助眠解压资源，需要登录

**必选参数 :** `id` : 资源 id

**可选参数 :** `cancel` : 默认 `false`，传 `true` 表示取消收藏

**方法名 :** `sati_resource_sub`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "123456");
let result = client.sati_resource_sub(&query).await.unwrap();
```

---

### sati_resource_sub_list - 收藏列表

说明 : 获取助眠解压资源的收藏列表，需要登录

**参数 :** 无

**方法名 :** `sati_resource_sub_list`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.sati_resource_sub_list(&query).await.unwrap();
```

---

### sati_timescene_resources_get - 时间场景推荐资源

说明 : 获取特定时间场景的助眠解压推荐资源

**参数 :** 无

**方法名 :** `sati_timescene_resources_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.sati_timescene_resources_get(&query).await.unwrap();
```

---


## 🎵 音乐人

### musician_sign - 音乐人签到

说明 : 音乐人登录后调用此接口,可以完成"登录音乐人中心"任务,然后通过 `musician_cloudbean_obtain` 接口可以领取相应的云豆

**方法名 :** `musician_sign`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.musician_sign(&query).await.unwrap();
```

---

### musician_tasks - 音乐人任务

说明 : 音乐人登录后调用此接口,可获取音乐人任务。返回数据中 `status` 字段: 0 未开始, 10 进行中, 20 完成但未领取云豆, 100 完成并已领取

**方法名 :** `musician_tasks`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.musician_tasks(&query).await.unwrap();
```

---

### musician_tasks_new - 音乐人任务(新)

说明 : 音乐人登录后调用此接口,可获取音乐人任务(新版)

**方法名 :** `musician_tasks_new`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.musician_tasks_new(&query).await.unwrap();
```

---

### musician_vip_tasks - 音乐人黑胶会员任务

说明 : 音乐人登录后调用此接口,可获取音乐人黑胶会员任务。返回数据中 `missionStatus` 为 100 表示任务完成

**方法名 :** `musician_vip_tasks`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.musician_vip_tasks(&query).await.unwrap();
```

---

### musician_data_overview - 音乐人数据概况

说明 : 音乐人登录后调用此接口,可获取统计数据概况

**方法名 :** `musician_data_overview`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.musician_data_overview(&query).await.unwrap();
```

---

### musician_play_trend - 音乐人播放趋势

说明 : 音乐人登录后调用此接口,可获取歌曲播放趋势

**必选参数 :** `startTime` : 开始时间 ; `endTime` : 结束时间

**方法名 :** `musician_play_trend`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("startTime", "2021-05-24")
    .param("endTime", "2021-05-30");
let result = client.musician_play_trend(&query).await.unwrap();
```

---

### musician_cloudbean - 账号云豆数

说明 : 音乐人登录后调用此接口,可获取账号云豆数

**方法名 :** `musician_cloudbean`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.musician_cloudbean(&query).await.unwrap();
```

---

### musician_cloudbean_obtain - 领取云豆

说明 : 音乐人登录后调用此接口,可领取已完成的音乐人任务的云豆奖励

**必选参数 :** `id` : 任务 id,通过 `musician_tasks` 获取到的 `userMissionId` ; `period` : 通过 `musician_tasks` 获取

**方法名 :** `musician_cloudbean_obtain`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("id", "7036416928")
    .param("period", "1");
let result = client.musician_cloudbean_obtain(&query).await.unwrap();
```

---

## 👥 粉丝中心

### fanscenter_overview_get - 粉丝中心概览

说明 : 调用此接口获取粉丝中心概览数据

**方法名 :** `fanscenter_overview_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.fanscenter_overview_get(&query).await.unwrap();
```

---

### fanscenter_trend_list - 粉丝趋势列表

说明 : 调用此接口获取粉丝趋势列表数据

**方法名 :** `fanscenter_trend_list`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.fanscenter_trend_list(&query).await.unwrap();
```

---

### fanscenter_basicinfo_age_get - 粉丝年龄分布

说明 : 调用此接口获取粉丝年龄分布数据

**方法名 :** `fanscenter_basicinfo_age_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.fanscenter_basicinfo_age_get(&query).await.unwrap();
```

---

### fanscenter_basicinfo_gender_get - 粉丝性别分布

说明 : 调用此接口获取粉丝性别分布数据

**方法名 :** `fanscenter_basicinfo_gender_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.fanscenter_basicinfo_gender_get(&query).await.unwrap();
```

---

### fanscenter_basicinfo_province_get - 粉丝省份分布

说明 : 调用此接口获取粉丝省份分布数据

**方法名 :** `fanscenter_basicinfo_province_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.fanscenter_basicinfo_province_get(&query).await.unwrap();
```

---

## 📚 UGC 百科

### ugc_song_get - 歌曲简要百科信息

说明 : 登录后调用此接口,传入歌曲 id,可获取对应的歌曲简要百科信息

**必选参数 :** `id` : 歌曲 id

**方法名 :** `ugc_song_get`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "2058263032");
let result = client.ugc_song_get(&query).await.unwrap();
```

---

### ugc_artist_get - 歌手简要百科信息

说明 : 登录后调用此接口,传入歌手 id,可获取对应的歌手简要百科信息

**必选参数 :** `id` : 歌手 id

**方法名 :** `ugc_artist_get`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "15396");
let result = client.ugc_artist_get(&query).await.unwrap();
```

---

### ugc_album_get - 专辑简要百科信息

说明 : 登录后调用此接口,传入专辑 id,可获取对应的专辑简要百科信息

**必选参数 :** `id` : 专辑 id

**方法名 :** `ugc_album_get`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "168223858");
let result = client.ugc_album_get(&query).await.unwrap();
```

---

### ugc_mv_get - MV 简要百科信息

说明 : 登录后调用此接口,传入 MV id,可获取对应的 MV 简要百科信息

**必选参数 :** `id` : MV id

**方法名 :** `ugc_mv_get`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "14572641");
let result = client.ugc_mv_get(&query).await.unwrap();
```

---

### ugc_detail - 用户贡献内容

说明 : 登录后调用此接口,可获取当前登录用户贡献内容

**必选参数 :** `type` : 内容种类。曲库纠错 - 歌手:1, 专辑:2, 歌曲:3, MV:4, 歌词:5, 翻译:6 ; 曲库补充 - 专辑:101, MV:103

**可选参数 :** `limit` : 取出数量,默认为 10 ; `offset` : 偏移数量 ; `auditStatus` : 审核状态(待审核:0, 未采纳:-5, 审核中:1, 部分审核通过:4, 审核通过:5) ; `order` : 排序,默认降序 desc,顺序 asc

**方法名 :** `ugc_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("type", "1");
let result = client.ugc_detail(&query).await.unwrap();
```

---

### ugc_artist_search - 搜索歌手(百科)

说明 : 登录后调用此接口,传入歌手名关键字或者歌手 id,可获取搜索到的歌手信息

**必选参数 :** `keyword` : 关键字或歌手 id

**可选参数 :** `limit` : 取出条目数量,默认为 40

**方法名 :** `ugc_artist_search`

**Rust 调用例子 :**

```rust
let query = Query::new().param("keyword", "sasakure");
let result = client.ugc_artist_search(&query).await.unwrap();
```

---

### ugc_user_devote - 用户贡献条目、积分、云贝数量

说明 : 登录后调用此接口,可获取当前登录用户贡献条目、积分、云贝数量

**方法名 :** `ugc_user_devote`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.ugc_user_devote(&query).await.unwrap();
```

---

## 🎧 一起听

### listentogether_room_create - 创建一起听房间

说明 : 登录后调用此接口,可创建一起听房间

**方法名 :** `listentogether_room_create`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listentogether_room_create(&query).await.unwrap();
```

---

### listentogether_room_check - 检查一起听房间

说明 : 调用此接口,可检查一起听房间状态

**方法名 :** `listentogether_room_check`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listentogether_room_check(&query).await.unwrap();
```

---

### listentogether_accept - 接受一起听邀请

说明 : 调用此接口,可接受一起听邀请

**方法名 :** `listentogether_accept`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listentogether_accept(&query).await.unwrap();
```

---

### listentogether_status - 一起听状态

说明 : 调用此接口,可获取一起听状态

**方法名 :** `listentogether_status`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listentogether_status(&query).await.unwrap();
```

---

### listentogether_heatbeat - 一起听心跳

说明 : 调用此接口,发送一起听心跳以保持连接

**方法名 :** `listentogether_heatbeat`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listentogether_heatbeat(&query).await.unwrap();
```

---

### listentogether_play_command - 一起听播放命令

说明 : 调用此接口,发送一起听播放命令

**方法名 :** `listentogether_play_command`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listentogether_play_command(&query).await.unwrap();
```

---

### listentogether_sync_list_command - 一起听同步列表命令

说明 : 调用此接口,发送一起听同步列表命令

**方法名 :** `listentogether_sync_list_command`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listentogether_sync_list_command(&query).await.unwrap();
```

---

### listentogether_sync_playlist_get - 获取一起听同步播放列表

说明 : 调用此接口,获取一起听同步播放列表

**方法名 :** `listentogether_sync_playlist_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listentogether_sync_playlist_get(&query).await.unwrap();
```

---

### listentogether_end - 结束一起听

说明 : 调用此接口,可结束一起听

**方法名 :** `listentogether_end`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.listentogether_end(&query).await.unwrap();
```

---

## 📡 广播电台

### broadcast_category_region_get - 广播电台分类/地区信息

说明 : 调用此接口,获取广播电台分类/地区信息

**方法名 :** `broadcast_category_region_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.broadcast_category_region_get(&query).await.unwrap();
```

---

### broadcast_channel_list - 广播电台全部电台

说明 : 调用此接口,获取广播电台全部电台

**可选参数 :** `categoryId` : 类别 id,默认为 0 ; `regionId` : 地区 id,默认为 0

**方法名 :** `broadcast_channel_list`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.broadcast_channel_list(&query).await.unwrap();
```

---

### broadcast_channel_currentinfo - 广播电台信息

说明 : 调用此接口,传入电台 id,获取广播电台信息

**必选参数 :** `id` : 电台 id

**方法名 :** `broadcast_channel_currentinfo`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "5");
let result = client.broadcast_channel_currentinfo(&query).await.unwrap();
```

---

### broadcast_channel_collect_list - 广播电台我的收藏

说明 : 调用此接口,获取广播电台我的收藏

**可选参数 :** `limit` : 返回数量,默认为 99999

**方法名 :** `broadcast_channel_collect_list`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.broadcast_channel_collect_list(&query).await.unwrap();
```

---

### broadcast_sub - 广播电台订阅

说明 : 调用此接口,可订阅/取消订阅广播电台

**方法名 :** `broadcast_sub`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.broadcast_sub(&query).await.unwrap();
```

---

## 🧑‍💻 云小编

> 云小编相关接口均需要登录。考试流程示例：`rep_ugc_exam_start` → `rep_ugc_exam_question_single_get` → `rep_ugc_exam_submit` → `rep_ugc_exam_result_get`。

### middle_play_lottery_remain_chance - 抽奖剩余次数

说明 : 查询云小编每日抽奖的剩余次数

**可选参数 :** `activityId` : 活动 id，默认 `6501202`

**方法名 :** `middle_play_lottery_remain_chance`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.middle_play_lottery_remain_chance(&query).await.unwrap();
```

---

### middle_play_do_lottery - 每日抽奖

说明 : 进行云小编每日抽奖

**可选参数 :** `activityId` : 活动 id，默认 `6501202` ; `drawCount` : 抽奖次数，默认 `1`

**方法名 :** `middle_play_do_lottery`

**Rust 调用例子 :**

```rust
let query = Query::new().param("drawCount", "1");
let result = client.middle_play_do_lottery(&query).await.unwrap();
```

---

### rep_ugc_activity_get - 云小编活动信息

说明 : 获取云小编活动信息

**参数 :** 无

**方法名 :** `rep_ugc_activity_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.rep_ugc_activity_get(&query).await.unwrap();
```

---

### rep_ugc_activity_collect - 领取任务积分

说明 : 领取云小编任务积分

**可选参数 :** `activityId` : 活动 id，默认 `5001`，可从 `rep_ugc_activity_get` 获取

**方法名 :** `rep_ugc_activity_collect`

**Rust 调用例子 :**

```rust
let query = Query::new().param("activityId", "5001");
let result = client.rep_ugc_activity_collect(&query).await.unwrap();
```

---

### rep_ugc_user_get - 云小编用户详情

说明 : 获取当前用户在云小编中的详情

**参数 :** 无

**方法名 :** `rep_ugc_user_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.rep_ugc_user_get(&query).await.unwrap();
```

---

### rep_ugc_user_sign - 云小编每日签到

说明 : 云小编每日签到

**参数 :** 无

**方法名 :** `rep_ugc_user_sign`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.rep_ugc_user_sign(&query).await.unwrap();
```

---

### rep_ugc_user_vip - 会员任务状态

说明 : 查询云小编会员任务状态。返回的 `data.status` 为 `10` / `20` 时可领取 1 日黑胶会员，`30` 表示已领取

**参数 :** 无

**方法名 :** `rep_ugc_user_vip`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.rep_ugc_user_vip(&query).await.unwrap();
```

---

### rep_ugc_user_collect_vip - 领取一日会员

说明 : 领取云小编任务的一日黑胶会员

**可选参数 :** `activityId` : 活动 id，默认 `5001`

**方法名 :** `rep_ugc_user_collect_vip`

**Rust 调用例子 :**

```rust
let query = Query::new().param("activityId", "5001");
let result = client.rep_ugc_user_collect_vip(&query).await.unwrap();
```

---

### rep_ugc_exam_info_get - 考试状态

说明 : 获取云小编考试状态

**必选参数 :** `examType` : 考试类型，可选 `musicalStyleEnter` / `languageEnter` / `oriSingerEnter` / `emotionEnter`

**方法名 :** `rep_ugc_exam_info_get`

**Rust 调用例子 :**

```rust
let query = Query::new().param("examType", "musicalStyleEnter");
let result = client.rep_ugc_exam_info_get(&query).await.unwrap();
```

---

### rep_ugc_exam_start - 开始考试

说明 : 开始云小编考试

**必选参数 :** `examType` : 考试类型

**方法名 :** `rep_ugc_exam_start`

**Rust 调用例子 :**

```rust
let query = Query::new().param("examType", "musicalStyleEnter");
let result = client.rep_ugc_exam_start(&query).await.unwrap();
```

---

### rep_ugc_exam_question_single_get - 考试取题

说明 : 获取云小编考试的下一道题

**必选参数 :** `examType` : 考试类型 ; `taskId` : 考试任务 id

**方法名 :** `rep_ugc_exam_question_single_get`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("examType", "musicalStyleEnter")
    .param("taskId", "123456");
let result = client.rep_ugc_exam_question_single_get(&query).await.unwrap();
```

---

### rep_ugc_exam_submit - 提交答案

说明 : 提交单题答案

**必选参数 :** `examType` : 考试类型 ; `taskId` : 考试任务 id ; `questionId` : 题目 id ; `answer` : 答案，`A` 表示正确，`B` 表示错误

**方法名 :** `rep_ugc_exam_submit`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("examType", "musicalStyleEnter")
    .param("taskId", "123456")
    .param("questionId", "789")
    .param("answer", "A");
let result = client.rep_ugc_exam_submit(&query).await.unwrap();
```

---

### rep_ugc_exam_result_get - 考试结果

说明 : 获取云小编考试结果

**必选参数 :** `examType` : 考试类型 ; `taskId` : 考试任务 id

**方法名 :** `rep_ugc_exam_result_get`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("examType", "musicalStyleEnter")
    .param("taskId", "123456");
let result = client.rep_ugc_exam_result_get(&query).await.unwrap();
```

---

### thinktank_audit_resource_detail - 获取审核任务

说明 : 获取云小编审核任务

**可选参数 :** `type` : `1` 曲风、`2` 语种、`3` 原唱、`4` 情绪标签，默认 `4`

**方法名 :** `thinktank_audit_resource_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("type", "4");
let result = client.thinktank_audit_resource_detail(&query).await.unwrap();
```

---

### thinktank_audit_resource_update - 提交审核任务

说明 : 提交云小编审核结果

**必选参数 :** `taskId` : 任务 id ; `judgement` : `1` 同意、`2` 否决、`3` 跳过

**可选参数 :** `type` : 任务类型，默认 `4`

**方法名 :** `thinktank_audit_resource_update`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("taskId", "123456")
    .param("judgement", "1");
let result = client.thinktank_audit_resource_update(&query).await.unwrap();
```

---


## 🔧 其他

### hot_topic - 获取热门话题

说明 : 调用此接口,可获取热门话题

**可选参数 :** `limit` : 取出数量,默认为 20 ; `offset` : 偏移数量

**方法名 :** `hot_topic`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("limit", "30")
    .param("offset", "30");
let result = client.hot_topic(&query).await.unwrap();
```

---

### topic_detail - 获取话题详情

说明 : 调用此接口,可获取话题详情

**必选参数 :** `actid` : 话题 id

**方法名 :** `topic_detail`

**Rust 调用例子 :**

```rust
let query = Query::new().param("actid", "111551188");
let result = client.topic_detail(&query).await.unwrap();
```

---

### topic_detail_event_hot - 获取话题详情热门动态

说明 : 调用此接口,可获取话题详情热门动态

**必选参数 :** `actid` : 话题 id

**方法名 :** `topic_detail_event_hot`

**Rust 调用例子 :**

```rust
let query = Query::new().param("actid", "111551188");
let result = client.topic_detail_event_hot(&query).await.unwrap();
```

---

### topic_sublist - 收藏的专栏

说明 : 调用此接口,可获取收藏的专栏

**可选参数 :** `limit` : 取出数量,默认为 50 ; `offset` : 偏移数量

**方法名 :** `topic_sublist`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("limit", "2")
    .param("offset", "1");
let result = client.topic_sublist(&query).await.unwrap();
```

---

### calendar - 音乐日历

说明 : 登录后调用此接口,传入开始和结束时间,可获取音乐日历

**可选参数 :** `startTime` : 开始时间戳 ; `endTime` : 结束时间戳

**方法名 :** `calendar`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("startTime", "1606752000000")
    .param("endTime", "1609430399999");
let result = client.calendar(&query).await.unwrap();
```

---

### batch - 批量请求接口

说明 : 登录后调用此接口,传入接口和对应原始参数,可批量请求接口

**方法名 :** `batch`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.batch(&query).await.unwrap();
```

---

### api - 通用 API 请求

说明 : 通用 API 请求接口

**方法名 :** `api`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.api(&query).await.unwrap();
```

---

### inner_version - 内部版本接口

说明 : 调用此接口,可获得内部版本号

**方法名 :** `inner_version`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.inner_version(&query).await.unwrap();
```

---

### weblog - 日志上报

说明 : 调用此接口进行日志上报

**方法名 :** `weblog`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.weblog(&query).await.unwrap();
```

---

### eapi_decrypt - EAPI 解密

说明 : 调用此接口,可进行 EAPI 解密

**方法名 :** `eapi_decrypt`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.eapi_decrypt(&query).await.unwrap();
```

---

### sign_happy_info - 乐签信息

说明 : 调用此接口,可获取乐签信息

**方法名 :** `sign_happy_info`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.sign_happy_info(&query).await.unwrap();
```

---

### signin_progress - 签到进度

说明 : 调用此接口,可获得签到进度

**可选参数 :** `moduleId` : 模块 id,默认为 '1207signin-1207signin'

**方法名 :** `signin_progress`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("moduleId", "1207signin-1207signin");
let result = client.signin_progress(&query).await.unwrap();
```

---

### summary_annual - 年度听歌报告

说明 : 登录后调用此接口,可获取当前登录用户年度听歌报告,目前支持 2017-2024 年的报告

**必选参数 :** `year` : 报告年份

**方法名 :** `summary_annual`

**Rust 调用例子 :**

```rust
let query = Query::new().param("year", "2024");
let result = client.summary_annual(&query).await.unwrap();
```

---

### threshold_detail_get - 阈值详情获取

说明 : 调用此接口获取阈值详情

**方法名 :** `threshold_detail_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.threshold_detail_get(&query).await.unwrap();
```

---

### creator_authinfo_get - 创作者认证信息获取

说明 : 调用此接口获取创作者认证信息

**方法名 :** `creator_authinfo_get`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.creator_authinfo_get(&query).await.unwrap();
```

---

### sheet_list - 乐谱列表

说明 : 调用此接口可以获取歌曲的乐谱列表

**必选参数 :** `id` : 歌曲 ID

**方法名 :** `sheet_list`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "1815684465");
let result = client.sheet_list(&query).await.unwrap();
```

---

### sheet_preview - 乐谱内容

说明 : 登录后调用此接口获取乐谱的内容

**必选参数 :** `id` : 乐谱 ID

**方法名 :** `sheet_preview`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "143190");
let result = client.sheet_preview(&query).await.unwrap();
```

---

### aidj_content_rcmd - 私人 DJ 推荐内容

说明 : 调用此接口可以获取私人 DJ 的推荐内容(包括 DJ 声音和推荐歌曲)

**可选参数 :** `longitude` / `latitude` : 当前的经纬度

**方法名 :** `aidj_content_rcmd`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.aidj_content_rcmd(&query).await.unwrap();
```

---

### music_first_listen_info - 回忆坐标

说明 : 可以获取当前歌曲的回忆坐标信息(见手机 APP 百科页的回忆坐标功能)

**必选参数 :** `id` : 歌曲 ID

**方法名 :** `music_first_listen_info`

**Rust 调用例子 :**

```rust
let query = Query::new().param("id", "186016");
let result = client.music_first_listen_info(&query).await.unwrap();
```

---

### verify_get_qr - 验证接口-二维码生成

说明 : 进行某些操作可能会触发验证,调用此接口生成二维码,使用 app 扫码后可解除验证

**必选参数 :** `vid` : verifyId ; `type` : verifyType ; `token` : verifyToken ; `evid` : params 的 event_id ; `sign` : params 的 sign

**方法名 :** `verify_get_qr`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("vid", "xxx")
    .param("type", "1")
    .param("token", "xxx")
    .param("evid", "xxx")
    .param("sign", "xxx");
let result = client.verify_get_qr(&query).await.unwrap();
```

---

### verify_qrcodestatus - 验证接口-二维码检测

说明 : 传入 `verify_get_qr` 接口返回的 `qr` 字符串,可检测二维码扫描状态

**必选参数 :** `qr` : `verify_get_qr` 接口返回的 `qr` 字符串

返回状态说明: qrCodeStatus:0 - 二维码生成成功 ; qrCodeStatus:10 - 已扫描 ; qrCodeStatus:20 - 验证成功 ; qrCodeStatus:21 - 二维码已失效

**方法名 :** `verify_qrcodestatus`

**Rust 调用例子 :**

```rust
let query = Query::new().param("qr", "xxx");
let result = client.verify_qrcodestatus(&query).await.unwrap();
```

---

### ad_get - 获取广告

说明 : 获取广告，并在 `extra.reqId` 中返回广告上报所需的 `req_id`

**可选参数 :** `type_ids` : 广告位类型 ID 的 JSON 数组字符串，默认 `["400002_0"]`

**方法名 :** `ad_get`

**Rust 调用例子 :**

```rust
let query = Query::new().param("type_ids", r#"["400002_0"]"#);
let result = client.ad_get(&query).await.unwrap();
```

---

### ad_listening_rights - 免费听时长状态

说明 : 获取免费听时长状态

**参数 :** 无

**方法名 :** `ad_listening_rights`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.ad_listening_rights(&query).await.unwrap();
```

---

### decrypt - 通用解密

说明 : 解密 eapi / weapi / linuxapi / xeapi 的请求或返回数据，主要用于调试

**必选参数 :** `data` : 待解密内容（兼容 `hexString`）

**可选参数 :** `crypto` : 加密方式，可选 `eapi` / `weapi` / `linuxapi` / `xeapi` / `api`，默认 `eapi` ; `isReq` : 是否为请求数据，默认 `true`，传 `false` 表示解密返回数据

> 说明：`weapi` 请求数据需要 RSA 私钥、`xeapi` 请求数据涉及 X25519 密钥交换，均暂不支持；两者的返回数据解密可用。

**方法名 :** `decrypt`

**Rust 调用例子 :**

```rust
let query = Query::new()
    .param("crypto", "eapi")
    .param("data", "XXXX")
    .param("isReq", "true");
let result = client.decrypt(&query).await.unwrap();
```

---

### lbs_city_code - 多级行政区划数据

说明 : 获取多级行政区划数据

**可选参数 :** `bizCode` : 业务编码

**方法名 :** `lbs_city_code`

**Rust 调用例子 :**

```rust
let query = Query::new();
let result = client.lbs_city_code(&query).await.unwrap();
```

---


## 致谢 & License

### 致谢

- [NeteaseCloudMusicApi Enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced) - 原始 Node.js 实现
- [Binaryify/NeteaseCloudMusicApi](https://github.com/Binaryify/NeteaseCloudMusicApi) - 原始项目

### License

本项目基于 [MIT License](LICENSE) 开源。
