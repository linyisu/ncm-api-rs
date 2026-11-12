# 上游同步记录

本仓库是 [SPlayer-Dev/ncm-api-rs](https://github.com/SPlayer-Dev/ncm-api-rs) 的 fork，代码来源为 [NeteaseCloudMusicApiEnhanced/api-enhanced](https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced)。

## 当前状态

| 项目 | 值 |
|------|----|
| 上游仓库 | `NeteaseCloudMusicApiEnhanced/api-enhanced` |
| 已同步版本 | `v4.41.1` |
| 上游最新提交 | `4a7d557`（2026-11-06） |
| 本仓库同步日期 | 2026-11-13 |

## 同步历史

| 时间 | 上游版本/范围 | 内容 |
|------|---------------|------|
| 2026-10 | `v4.31.0` → 2026-10-05 | PR #4：新增 xeapi / neapi 加密、66+ 接口、错误响应与 IP 透传等 |
| 2026-11 | `v4.41.1` | v9.6.13 客户端新增接口、cookie 请求头兼容、`ad_listening_rights_gain` |

## 2026-10 同步新增

| 方法名 | 说明 |
|--------|------|
| `ad_get` | 获取广告，并在 extra.reqId 中提取广告 req_id |
| `ad_listening_rights` | 获取免费听时长状态 |
| `artist_new_song_mv_list_v2` | 获取关注歌手的新歌曲和 MV |
| `artist_new_song_playall` | 获取所有关注歌手最近的 50 首新歌 |
| `captcha_safe_sent` | 发送安全验证码 |
| `captcha_sent_v1` | 发送验证码 v1 |
| `chart_detail` | 获取指定维度音乐排行榜详情 |
| `chart_song_detail` | 获取指定维度音乐排行榜列表 |
| `comment_add` | 发送评论 |
| `comment_report` | 举报评论 |
| `decrypt` | 通用解密 |
| `device_kickoff` | 强制下线设备 |
| `device_list` | 登录设备列表 |
| `deviceinfo_center_upload` | 上报设备中心设备名称 |
| `event_privacy` | 修改本人动态的可见权限 |
| `fans_group_detail` | 乐迷团详情 |
| `fans_group_feed_recommend` | 乐迷团推荐笔记 |
| `fans_group_user_group_detail` | 用户所处乐迷团详情 |
| `fans_group_user_groups` | 当前登录用户加入的全部歌手乐迷团列表 |
| `lbs_city_code` | 多级行政区划数据 |
| `like_v1` | 红心与取消红心歌曲 v1 |
| `listen_data_song_play_rank` | 听歌足迹 - 歌曲播放排行 (Top20) |
| `middle_play_do_lottery` | 云小编每日抽奖 |
| `middle_play_lottery_remain_chance` | 云小编抽奖剩余次数查询 |
| `radio_sport_get` | 跑步漫游 |
| `recommend_category_configs` | 每日风格推荐场景配置列表 |
| `recommend_category_set` | 每日风格推荐场景配置设置 |
| `recommend_category_songs` | 每日推荐风格歌曲 |
| `recommend_songs_v1` | 每日推荐歌曲 v1 |
| `register_checktoken_v3` | 实时获取易盾 v3 反作弊 token（不缓存） |
| `register_neapikey` | 刷新 neapi 密钥配置 |
| `register_xeapikey` | 重新向服务端申请 xeapi 公钥并刷新本地缓存 |
| `relay_play_state_submit` | 提交歌曲播放状态 |
| `rep_ugc_activity_collect` | 云小编领取任务积分 |
| `rep_ugc_activity_get` | 云小编活动信息 |
| `rep_ugc_exam_info_get` | 云小编考试状态 |
| `rep_ugc_exam_question_single_get` | 云小编考试取题 |
| `rep_ugc_exam_result_get` | 云小编考试结果 |
| `rep_ugc_exam_start` | 云小编考试开始 |
| `rep_ugc_exam_submit` | 云小编考试提交 |
| `rep_ugc_user_collect_vip` | 云小编领取一日会员 |
| `rep_ugc_user_get` | 云小编获取用户详情 |
| `rep_ugc_user_sign` | 云小编每日签到 |
| `rep_ugc_user_vip` | 云小编查询会员任务状态 |
| `sati_resource_list` | 助眠解压 - 标签下资源列表 |
| `sati_resource_list_more` | 助眠解压 - 同类推荐 |
| `sati_resource_sub` | 助眠解压 - 收藏 |
| `sati_resource_sub_list` | 助眠解压 - 收藏列表 |
| `sati_tag_list` | 助眠解压 - 标签列表 |
| `sati_timescene_resources_get` | 助眠解压 - 特定时间场景推荐资源 |
| `scrobble_v1` | 听歌打卡（NCBL 加密版） |
| `song_cloud_download` | 从云盘获取歌曲下载链接 |
| `song_copyright_rcmd` | 灰色歌曲的其他版本推荐 |
| `song_creators` | 歌曲创作者信息 |
| `song_simi_get` | 插播相似歌曲 |
| `song_wiki_info` | 歌曲百科 |
| `thinktank_audit_resource_detail` | 云小编获取任务 |
| `thinktank_audit_resource_update` | 云小编提交任务 |
| `user_event_all` | 自动翻页获取当前登录用户全部动态 |
| `vip_growthpoint_getall` | 一键领取所有会员成长值 |
| `vip_sign_detail` | 黑胶乐签打卡详情 |
| `vip_sign_history` | 黑胶乐签打卡历史 / 状态查询 |
| `vip_tasks_v1` | 会员任务 - 新版 |
| `yunbei_task_finish_v1` | 云贝广告任务 - 完成任务领取云贝 |
| `yunbei_task_list_v1` | 云贝广告任务 - 查询今日任务状态 |
| `yunbei_task_recommend_song` | 云贝广告任务 - 获取推荐歌曲 |

## 2026-11 同步新增

同步 `api-enhanced` v4.41.1 中 v9.6.13 客户端新增接口：

| 方法名 | 说明 |
|--------|------|
| `banner_v3` | 首页轮播 v3 |
| `comment_default_text` | 评论默认文案 |
| `comment_hotwall_square_info` | 热评墙广场信息 |
| `event_search` | 动态搜索 |
| `search_lyric` | 按歌词搜索 |
| `search_song_list_page` | 新版单曲搜索（分页） |
| `search_suggest_mobile` | 搜索联想词（App 移动端） |

同时补齐了此前未同步的：

| 方法名 | 说明 |
|--------|------|
| `ad_listening_rights_gain` | 领取看广告免费听/云贝权益 |

## 尚未同步

| 上游模块 | 原因 |
|----------|------|
| `register_checktoken_v2` | 依赖 jsdom 运行易盾 Watchman SDK；当前使用静态 token 或 v3 token 兜底 |

## 路由别名

部分上游模块使用驼峰路径，Rust 方法名使用下划线。`build.rs` 负责映射：

| Rust 方法 | 上游路由 |
|-----------|----------|
| `digital_album_detail` | `/digitalAlbum/detail` |
| `digital_album_ordering` | `/digitalAlbum/ordering` |
| `digital_album_purchased` | `/digitalAlbum/purchased` |
| `digital_album_sales` | `/digitalAlbum/sales` |
| `dj_radio_top` | `/djRadio/top` |
| `verify_get_qr` | `/verify/getQr` |
| `rep_ugc_user_collect_vip` | `/rep/ugc/user/collect-vip` |

## 如何继续同步

```bash
# 添加上游远程（只需要一次）
git remote add upstream https://github.com/NeteaseCloudMusicApiEnhanced/api-enhanced.git

# 拉取最新代码
git fetch upstream main

# 查看变动
git log --oneline <上次同步提交>..upstream/main -- module util public/docs/home.md
```

同步时重点检查：

1. `module/` 下新增或修改的接口
2. `util/` 下加密、Cookie、请求层变化
3. `public/docs/home.md` 的参数说明
4. 是否需要更新 `src/api/mod.rs`、`build.rs`、`docs/API.md`
5. 重新生成方法索引：

```bash
python3 scripts/gen_api_index.py
```

不兼容变更记录到 [MIGRATION.md](MIGRATION.md)。
