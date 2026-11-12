# API 方法索引

> 共 455 个方法。参数说明和调用示例见 [API.md](API.md)。
>
> 本文件由 `scripts/gen_api_index.py` 生成，请勿手工编辑。

## 登录相关

| 方法 | 说明 |
|------|------|
| [`login`](API.md#method-login) | 邮箱登录 |
| [`login_cellphone`](API.md#method-login_cellphone) | 手机登录 |
| [`login_qr_key`](API.md#method-login_qr_key) | 二维码 key 生成 |
| [`login_qr_create`](API.md#method-login_qr_create) | 二维码生成 |
| [`login_qr_check`](API.md#method-login_qr_check) | 二维码检测扫码状态 |
| [`register_anonimous`](API.md#method-register_anonimous) | 游客登录 |
| [`login_refresh`](API.md#method-login_refresh) | 刷新登录 |
| [`login_status`](API.md#method-login_status) | 登录状态 |
| [`logout`](API.md#method-logout) | 退出登录 |
| [`captcha_sent`](API.md#method-captcha_sent) | 发送验证码 |
| [`captcha_verify`](API.md#method-captcha_verify) | 验证验证码 |
| [`register_cellphone`](API.md#method-register_cellphone) | 注册(修改密码) |
| [`cellphone_existence_check`](API.md#method-cellphone_existence_check) | 检测手机号码是否已注册 |
| [`activate_init_profile`](API.md#method-activate_init_profile) | 初始化昵称 |
| [`nickname_check`](API.md#method-nickname_check) | 重复昵称检测 |
| [`rebind`](API.md#method-rebind) | 更换绑定手机 |
| [`captcha_safe_sent`](API.md#method-captcha_safe_sent) | 发送安全验证码 |
| [`captcha_sent_v1`](API.md#method-captcha_sent_v1) | 发送验证码 v1 |
| [`device_list`](API.md#method-device_list) | 登录设备列表 |
| [`device_kickoff`](API.md#method-device_kickoff) | 强制下线设备 |
| [`deviceinfo_center_upload`](API.md#method-deviceinfo_center_upload) | 上报设备中心设备名称 |
| [`register_checktoken_v3`](API.md#method-register_checktoken_v3) | 获取易盾 v3 token |
| [`register_neapikey`](API.md#method-register_neapikey) | 刷新 neapi 密钥配置 |
| [`register_xeapikey`](API.md#method-register_xeapikey) | 刷新 xeapi 公钥 |

## 用户相关

| 方法 | 说明 |
|------|------|
| [`user_detail`](API.md#method-user_detail) | 获取用户详情 |
| [`user_detail_new`](API.md#method-user_detail_new) | 获取用户详情（新版） |
| [`user_account`](API.md#method-user_account) | 获取账号信息 |
| [`user_subcount`](API.md#method-user_subcount) | 获取用户信息 , 歌单，收藏，mv, dj 数量 |
| [`user_level`](API.md#method-user_level) | 获取用户等级信息 |
| [`user_binding`](API.md#method-user_binding) | 获取用户绑定信息 |
| [`user_bindingcellphone`](API.md#method-user_bindingcellphone) | 用户绑定手机 |
| [`user_replacephone`](API.md#method-user_replacephone) | 用户更换绑定手机 |
| [`user_update`](API.md#method-user_update) | 更新用户信息 |
| [`avatar_upload`](API.md#method-avatar_upload) | 更新头像 |
| [`user_playlist`](API.md#method-user_playlist) | 获取用户歌单 |
| [`user_playlist_create`](API.md#method-user_playlist_create) | 用户的创建歌单列表 |
| [`user_playlist_collect`](API.md#method-user_playlist_collect) | 用户的收藏歌单列表 |
| [`user_follows`](API.md#method-user_follows) | 获取用户关注列表 |
| [`user_followeds`](API.md#method-user_followeds) | 获取用户粉丝列表 |
| [`user_follow_mixed`](API.md#method-user_follow_mixed) | 当前账号关注的用户/歌手 |
| [`user_mutualfollow_get`](API.md#method-user_mutualfollow_get) | 用户是否互相关注 |
| [`user_event`](API.md#method-user_event) | 获取用户动态 |
| [`user_record`](API.md#method-user_record) | 获取用户播放记录 |
| [`user_dj`](API.md#method-user_dj) | 获取用户电台 |
| [`user_audio`](API.md#method-user_audio) | 用户电台 |
| [`user_comment_history`](API.md#method-user_comment_history) | 获取用户历史评论 |
| [`user_medal`](API.md#method-user_medal) | 用户徽章 |
| [`user_social_status`](API.md#method-user_social_status) | 用户状态 |
| [`user_social_status_edit`](API.md#method-user_social_status_edit) | 用户状态 - 编辑 |
| [`user_social_status_rcmd`](API.md#method-user_social_status_rcmd) | 用户状态 - 相同状态的用户 |
| [`user_social_status_support`](API.md#method-user_social_status_support) | 用户状态 - 支持设置的状态 |
| [`follow`](API.md#method-follow) | 关注/取消关注用户 |
| [`pl_count`](API.md#method-pl_count) | 私信和通知接口 |
| [`countries_code_list`](API.md#method-countries_code_list) | 国家编码列表 |
| [`setting`](API.md#method-setting) | 设置 |
| [`get_userids`](API.md#method-get_userids) | 根据 nickname 获取 userid |
| [`event_privacy`](API.md#method-event_privacy) | 修改动态可见权限 |
| [`user_event_all`](API.md#method-user_event_all) | 获取当前用户全部动态 |

## 歌曲相关

| 方法 | 说明 |
|------|------|
| [`song_detail`](API.md#method-song_detail) | 获取歌曲详情 |
| [`song_url`](API.md#method-song_url) | 获取音乐 url |
| [`song_url_v1`](API.md#method-song_url_v1) | 获取音乐 url - 新版 |
| [`song_url_v1_302`](API.md#method-song_url_v1_302) | 302 到音乐 url - 新版 |
| [`song_url_ncmget`](API.md#method-song_url_ncmget) | 歌曲播放链接 (ncmget) |
| [`song_url_match`](API.md#method-song_url_match) | 直接获取灰色歌曲链接 |
| [`song_download_url`](API.md#method-song_download_url) | 获取客户端歌曲下载 url |
| [`song_download_url_v1`](API.md#method-song_download_url_v1) | 获取客户端歌曲下载链接 - 新版 |
| [`check_music`](API.md#method-check_music) | 音乐是否可用 |
| [`lyric`](API.md#method-lyric) | 获取歌词 |
| [`lyric_new`](API.md#method-lyric_new) | 获取逐字歌词 |
| [`like`](API.md#method-like) | 喜欢音乐 |
| [`likelist`](API.md#method-likelist) | 喜欢音乐列表 |
| [`song_like_check`](API.md#method-song_like_check) | 歌曲是否喜爱 |
| [`scrobble`](API.md#method-scrobble) | 听歌打卡 |
| [`song_order_update`](API.md#method-song_order_update) | 调整歌曲顺序 |
| [`song_chorus`](API.md#method-song_chorus) | 副歌时间 |
| [`song_wiki_summary`](API.md#method-song_wiki_summary) | 歌曲百科摘要 |
| [`song_music_detail`](API.md#method-song_music_detail) | 歌曲音质详情 |
| [`song_red_count`](API.md#method-song_red_count) | 歌曲红心数量 |
| [`song_dynamic_cover`](API.md#method-song_dynamic_cover) | 歌曲动态封面 |
| [`song_downlist`](API.md#method-song_downlist) | 会员下载歌曲记录 |
| [`song_monthdownlist`](API.md#method-song_monthdownlist) | 会员本月下载歌曲记录 |
| [`song_singledownlist`](API.md#method-song_singledownlist) | 已购买单曲下载记录 |
| [`song_purchased`](API.md#method-song_purchased) | 已购单曲 |
| [`song_lyrics_mark`](API.md#method-song_lyrics_mark) | 歌词摘录 - 歌词摘录信息 |
| [`song_lyrics_mark_add`](API.md#method-song_lyrics_mark_add) | 歌词摘录 - 添加/修改摘录歌词 |
| [`song_lyrics_mark_del`](API.md#method-song_lyrics_mark_del) | 歌词摘录 - 删除摘录歌词 |
| [`song_lyrics_mark_user_page`](API.md#method-song_lyrics_mark_user_page) | 歌词摘录 - 我的歌词本 |
| [`audio_match`](API.md#method-audio_match) | 听歌识曲 |
| [`like_v1`](API.md#method-like_v1) | 红心/取消红心歌曲 v1 |
| [`song_like`](API.md#method-song_like) | 喜欢歌曲（旧版） |
| [`relay_play_state_submit`](API.md#method-relay_play_state_submit) | 提交歌曲播放状态 |
| [`scrobble_v1`](API.md#method-scrobble_v1) | 听歌打卡 v1（NCBL 加密） |
| [`song_copyright_rcmd`](API.md#method-song_copyright_rcmd) | 灰色歌曲其他版本推荐 |
| [`song_creators`](API.md#method-song_creators) | 歌曲创作者信息 |
| [`song_simi_get`](API.md#method-song_simi_get) | 插播相似歌曲 |
| [`song_wiki_info`](API.md#method-song_wiki_info) | 歌曲百科 |

## 搜索相关

| 方法 | 说明 |
|------|------|
| [`search`](API.md#method-search) | 搜索 |
| [`cloudsearch`](API.md#method-cloudsearch) | 搜索（综合） |
| [`search_default`](API.md#method-search_default) | 默认搜索关键词 |
| [`search_hot`](API.md#method-search_hot) | 热搜列表(简略) |
| [`search_hot_detail`](API.md#method-search_hot_detail) | 热搜列表(详细) |
| [`search_suggest`](API.md#method-search_suggest) | 搜索建议 |
| [`search_multimatch`](API.md#method-search_multimatch) | 搜索多重匹配 |
| [`search_match`](API.md#method-search_match) | 本地歌曲文件匹配网易云歌曲信息 |
| [`search_suggest_pc`](API.md#method-search_suggest_pc) | 搜索建议（PC 端） |
| [`search_suggest_mobile`](API.md#method-search_suggest_mobile) | 搜索联想词（App 移动端） |
| [`search_song_list_page`](API.md#method-search_song_list_page) | 新版单曲搜索（分页） |
| [`search_lyric`](API.md#method-search_lyric) | 按歌词搜索 |
| [`event_search`](API.md#method-event_search) | 动态搜索 |

## 歌单相关

| 方法 | 说明 |
|------|------|
| [`playlist_create`](API.md#method-playlist_create) | 新建歌单 |
| [`playlist_delete`](API.md#method-playlist_delete) | 删除歌单 |
| [`playlist_subscribe`](API.md#method-playlist_subscribe) | 收藏/取消收藏歌单 |
| [`playlist_subscribers`](API.md#method-playlist_subscribers) | 歌单收藏者 |
| [`playlist_detail`](API.md#method-playlist_detail) | 获取歌单详情 |
| [`playlist_detail_dynamic`](API.md#method-playlist_detail_dynamic) | 歌单详情动态 |
| [`playlist_detail_rcmd_get`](API.md#method-playlist_detail_rcmd_get) | 相关歌单推荐 |
| [`playlist_track_all`](API.md#method-playlist_track_all) | 获取歌单所有歌曲 |
| [`playlist_tracks`](API.md#method-playlist_tracks) | 对歌单添加或删除歌曲 |
| [`playlist_track_add`](API.md#method-playlist_track_add) | 收藏视频到视频歌单 |
| [`playlist_track_delete`](API.md#method-playlist_track_delete) | 删除视频歌单里的视频 |
| [`playlist_update`](API.md#method-playlist_update) | 更新歌单 |
| [`playlist_desc_update`](API.md#method-playlist_desc_update) | 更新歌单描述 |
| [`playlist_name_update`](API.md#method-playlist_name_update) | 更新歌单名 |
| [`playlist_tags_update`](API.md#method-playlist_tags_update) | 更新歌单标签 |
| [`playlist_cover_update`](API.md#method-playlist_cover_update) | 歌单封面上传 |
| [`playlist_order_update`](API.md#method-playlist_order_update) | 调整歌单顺序 |
| [`playlist_update_playcount`](API.md#method-playlist_update_playcount) | 歌单更新播放量 |
| [`playlist_catlist`](API.md#method-playlist_catlist) | 歌单分类 |
| [`playlist_hot`](API.md#method-playlist_hot) | 热门歌单分类 |
| [`playlist_category_list`](API.md#method-playlist_category_list) | 歌单分类列表 |
| [`playlist_highquality_tags`](API.md#method-playlist_highquality_tags) | 精品歌单标签列表 |
| [`playlist_privacy`](API.md#method-playlist_privacy) | 公开隐私歌单 |
| [`playlist_mylike`](API.md#method-playlist_mylike) | 获取点赞过的视频 |
| [`playlist_video_recent`](API.md#method-playlist_video_recent) | 最近播放的视频 |
| [`playlist_import_name_task_create`](API.md#method-playlist_import_name_task_create) | 歌单导入 - 元数据/文字/链接导入 |
| [`playlist_import_task_status`](API.md#method-playlist_import_task_status) | 歌单导入 - 任务状态 |
| [`related_playlist`](API.md#method-related_playlist) | 相关歌单 |

## 评论相关

| 方法 | 说明 |
|------|------|
| [`comment_music`](API.md#method-comment_music) | 歌曲评论 |
| [`comment_album`](API.md#method-comment_album) | 专辑评论 |
| [`comment_playlist`](API.md#method-comment_playlist) | 歌单评论 |
| [`comment_mv`](API.md#method-comment_mv) | mv 评论 |
| [`comment_dj`](API.md#method-comment_dj) | 电台节目评论 |
| [`comment_video`](API.md#method-comment_video) | 视频评论 |
| [`comment_event`](API.md#method-comment_event) | 获取动态评论 |
| [`comment_floor`](API.md#method-comment_floor) | 楼层评论 |
| [`comment_hot`](API.md#method-comment_hot) | 热门评论 |
| [`comment_like`](API.md#method-comment_like) | 给评论点赞 |
| [`comment_new`](API.md#method-comment_new) | 新版评论接口 |
| [`comment_hug_list`](API.md#method-comment_hug_list) | 评论抱一抱列表 |
| [`comment_info_list`](API.md#method-comment_info_list) | 评论统计数据 |
| [`hug_comment`](API.md#method-hug_comment) | 抱一抱评论 |
| [`starpick_comments_summary`](API.md#method-starpick_comments_summary) | 云村星评馆 - 简要评论 |
| [`comment_add`](API.md#method-comment_add) | 发送评论 |
| [`comment_reply`](API.md#method-comment_reply) | 回复评论 |
| [`comment_delete`](API.md#method-comment_delete) | 删除评论 |
| [`comment_report`](API.md#method-comment_report) | 举报评论 |
| [`comment`](API.md#method-comment) | 通用评论接口 |
| [`comment_default_text`](API.md#method-comment_default_text) | 评论默认文案 |
| [`comment_hotwall_square_info`](API.md#method-comment_hotwall_square_info) | 热评墙广场信息 |

## 歌手相关

| 方法 | 说明 |
|------|------|
| [`artists`](API.md#method-artists) | 获取歌手单曲 |
| [`artist_detail`](API.md#method-artist_detail) | 获取歌手详情 |
| [`artist_detail_dynamic`](API.md#method-artist_detail_dynamic) | 歌手详情动态 |
| [`artist_songs`](API.md#method-artist_songs) | 歌手全部歌曲 |
| [`artist_album`](API.md#method-artist_album) | 获取歌手专辑 |
| [`artist_desc`](API.md#method-artist_desc) | 获取歌手描述 |
| [`artist_mv`](API.md#method-artist_mv) | 获取歌手 mv |
| [`artist_list`](API.md#method-artist_list) | 歌手分类列表 |
| [`artist_sub`](API.md#method-artist_sub) | 收藏/取消收藏歌手 |
| [`artist_sublist`](API.md#method-artist_sublist) | 收藏的歌手列表 |
| [`artist_top_song`](API.md#method-artist_top_song) | 歌手热门 50 首歌曲 |
| [`artist_fans`](API.md#method-artist_fans) | 歌手粉丝 |
| [`artist_follow_count`](API.md#method-artist_follow_count) | 歌手粉丝数量 |
| [`artist_new_mv`](API.md#method-artist_new_mv) | 关注歌手新 MV |
| [`artist_new_song`](API.md#method-artist_new_song) | 关注歌手新歌 |
| [`artist_video`](API.md#method-artist_video) | 获取歌手视频 |
| [`artist_new_song_mv_list_v2`](API.md#method-artist_new_song_mv_list_v2) | 关注歌手新歌与 MV（v2） |
| [`artist_new_song_playall`](API.md#method-artist_new_song_playall) | 关注歌手新歌播放全部 |
| [`fans_group_detail`](API.md#method-fans_group_detail) | 乐迷团详情 |
| [`fans_group_feed_recommend`](API.md#method-fans_group_feed_recommend) | 乐迷团推荐笔记 |
| [`fans_group_user_group_detail`](API.md#method-fans_group_user_group_detail) | 用户所在乐迷团详情 |
| [`fans_group_user_groups`](API.md#method-fans_group_user_groups) | 用户加入的乐迷团列表 |

## 专辑相关

| 方法 | 说明 |
|------|------|
| [`album`](API.md#method-album) | 获取专辑内容 |
| [`album_detail`](API.md#method-album_detail) | 数字专辑详情 |
| [`album_detail_dynamic`](API.md#method-album_detail_dynamic) | 专辑动态信息 |
| [`album_sub`](API.md#method-album_sub) | 收藏/取消收藏专辑 |
| [`album_sublist`](API.md#method-album_sublist) | 获取已收藏专辑列表 |
| [`album_newest`](API.md#method-album_newest) | 最新专辑 |
| [`album_new`](API.md#method-album_new) | 全部新碟 |
| [`album_list`](API.md#method-album_list) | 数字专辑-新碟上架 |
| [`album_list_style`](API.md#method-album_list_style) | 数字专辑-语种风格馆 |
| [`album_privilege`](API.md#method-album_privilege) | 获取专辑歌曲的音质 |
| [`album_songsaleboard`](API.md#method-album_songsaleboard) | 数字专辑&数字单曲-榜单 |

## MV 相关

| 方法 | 说明 |
|------|------|
| [`mv_all`](API.md#method-mv_all) | 全部 mv |
| [`mv_first`](API.md#method-mv_first) | 最新 mv |
| [`mv_exclusive_rcmd`](API.md#method-mv_exclusive_rcmd) | 网易出品 mv |
| [`mv_detail`](API.md#method-mv_detail) | 获取 mv 数据 |
| [`mv_detail_info`](API.md#method-mv_detail_info) | 获取 mv 点赞转发评论数数据 |
| [`mv_url`](API.md#method-mv_url) | mv 地址 |
| [`mv_sub`](API.md#method-mv_sub) | 收藏/取消收藏 MV |
| [`mv_sublist`](API.md#method-mv_sublist) | 收藏的 MV 列表 |
| [`personalized_mv`](API.md#method-personalized_mv) | 推荐 mv |

## 视频相关

| 方法 | 说明 |
|------|------|
| [`video_group_list`](API.md#method-video_group_list) | 获取视频标签列表 |
| [`video_category_list`](API.md#method-video_category_list) | 获取视频分类列表 |
| [`video_group`](API.md#method-video_group) | 获取视频标签/分类下的视频 |
| [`video_timeline_all`](API.md#method-video_timeline_all) | 获取全部视频列表 |
| [`video_timeline_recommend`](API.md#method-video_timeline_recommend) | 获取推荐视频 |
| [`video_detail`](API.md#method-video_detail) | 视频详情 |
| [`video_detail_info`](API.md#method-video_detail_info) | 获取视频点赞转发评论数数据 |
| [`video_url`](API.md#method-video_url) | 获取视频播放地址 |
| [`video_sub`](API.md#method-video_sub) | 收藏视频 |
| [`related_allvideo`](API.md#method-related_allvideo) | 相关视频 |
| [`mlog_url`](API.md#method-mlog_url) | 获取 mlog 播放地址 |
| [`mlog_to_video`](API.md#method-mlog_to_video) | 将 mlog id 转为视频 id |
| [`mlog_music_rcmd`](API.md#method-mlog_music_rcmd) | 歌曲相关视频 |

## 电台相关

| 方法 | 说明 |
|------|------|
| [`dj_banner`](API.md#method-dj_banner) | 电台 banner |
| [`dj_personalize_recommend`](API.md#method-dj_personalize_recommend) | 电台个性推荐 |
| [`dj_subscriber`](API.md#method-dj_subscriber) | 电台订阅者列表 |
| [`dj_catelist`](API.md#method-dj_catelist) | 电台分类 |
| [`dj_category_excludehot`](API.md#method-dj_category_excludehot) | 电台非热门类型 |
| [`dj_category_recommend`](API.md#method-dj_category_recommend) | 电台推荐类型 |
| [`dj_detail`](API.md#method-dj_detail) | 电台详情 |
| [`dj_hot`](API.md#method-dj_hot) | 热门电台 |
| [`dj_radio_hot`](API.md#method-dj_radio_hot) | 类别热门电台 |
| [`dj_program`](API.md#method-dj_program) | 电台节目 |
| [`dj_program_detail`](API.md#method-dj_program_detail) | 电台节目详情 |
| [`dj_program_toplist`](API.md#method-dj_program_toplist) | 电台节目榜 |
| [`dj_program_toplist_hours`](API.md#method-dj_program_toplist_hours) | 24 小时节目榜 |
| [`dj_recommend`](API.md#method-dj_recommend) | 电台推荐 |
| [`dj_recommend_type`](API.md#method-dj_recommend_type) | 电台分类推荐 |
| [`dj_sub`](API.md#method-dj_sub) | 电台订阅 |
| [`dj_sublist`](API.md#method-dj_sublist) | 电台订阅列表 |
| [`dj_toplist`](API.md#method-dj_toplist) | 新晋电台榜/热门电台榜 |
| [`dj_toplist_hours`](API.md#method-dj_toplist_hours) | 24 小时主播榜 |
| [`dj_toplist_newcomer`](API.md#method-dj_toplist_newcomer) | 主播新人榜 |
| [`dj_toplist_pay`](API.md#method-dj_toplist_pay) | 付费精品电台 |
| [`dj_toplist_popular`](API.md#method-dj_toplist_popular) | 最热主播榜 |
| [`dj_paygift`](API.md#method-dj_paygift) | 付费精选 |
| [`dj_today_perfered`](API.md#method-dj_today_perfered) | 电台今日优选 |
| [`dj_radio_top`](API.md#method-dj_radio_top) | 电台排行榜 |
| [`dj_difm_all_style_channel`](API.md#method-dj_difm_all_style_channel) | DIFM 电台分类 |
| [`dj_difm_channel_subscribe`](API.md#method-dj_difm_channel_subscribe) | 收藏 DIFM 频道 |
| [`dj_difm_channel_unsubscribe`](API.md#method-dj_difm_channel_unsubscribe) | 取消收藏 DIFM 频道 |
| [`dj_difm_playing_tracks_list`](API.md#method-dj_difm_playing_tracks_list) | DIFM 播放列表 |
| [`dj_difm_subscribe_channels_get`](API.md#method-dj_difm_subscribe_channels_get) | DIFM 收藏列表 |
| [`radio_sport_get`](API.md#method-radio_sport_get) | 跑步漫游 |

## 推荐相关

| 方法 | 说明 |
|------|------|
| [`personalized`](API.md#method-personalized) | 推荐歌单 |
| [`personalized_newsong`](API.md#method-personalized_newsong) | 推荐新音乐 |
| [`personalized_djprogram`](API.md#method-personalized_djprogram) | 推荐电台 |
| [`personalized_privatecontent`](API.md#method-personalized_privatecontent) | 独家放送(入口列表) |
| [`personalized_privatecontent_list`](API.md#method-personalized_privatecontent_list) | 独家放送列表 |
| [`recommend_songs`](API.md#method-recommend_songs) | 每日推荐歌曲 |
| [`recommend_resource`](API.md#method-recommend_resource) | 每日推荐歌单 |
| [`recommend_songs_dislike`](API.md#method-recommend_songs_dislike) | 每日推荐歌曲不感兴趣 |
| [`history_recommend_songs`](API.md#method-history_recommend_songs) | 获取历史日推可用日期列表 |
| [`history_recommend_songs_detail`](API.md#method-history_recommend_songs_detail) | 获取历史日推详情数据 |
| [`program_recommend`](API.md#method-program_recommend) | 推荐节目 |
| [`homepage_block_page`](API.md#method-homepage_block_page) | 首页-发现 |
| [`homepage_dragon_ball`](API.md#method-homepage_dragon_ball) | 首页圆形图标入口列表 |
| [`banner`](API.md#method-banner) | 首页 Banner |
| [`banner_v3`](API.md#method-banner_v3) | 首页轮播（v3） |
| [`daily_signin`](API.md#method-daily_signin) | 签到 |
| [`personal_fm`](API.md#method-personal_fm) | 私人 FM |
| [`personal_fm_mode`](API.md#method-personal_fm_mode) | 私人 FM 模式选择 |
| [`fm_trash`](API.md#method-fm_trash) | 垃圾桶 |
| [`playmode_intelligence_list`](API.md#method-playmode_intelligence_list) | 心动模式/智能播放 |
| [`playmode_song_vector`](API.md#method-playmode_song_vector) | 歌曲向量 |
| [`simi_song`](API.md#method-simi_song) | 相似歌曲 |
| [`simi_artist`](API.md#method-simi_artist) | 相似歌手 |
| [`simi_playlist`](API.md#method-simi_playlist) | 相似歌单 |
| [`simi_mv`](API.md#method-simi_mv) | 相似 MV |
| [`simi_user`](API.md#method-simi_user) | 最近听了这首歌的用户 |
| [`recommend_category_configs`](API.md#method-recommend_category_configs) | 每日风格推荐配置 |
| [`recommend_category_set`](API.md#method-recommend_category_set) | 设置每日风格推荐 |
| [`recommend_category_songs`](API.md#method-recommend_category_songs) | 每日风格推荐歌曲 |
| [`recommend_songs_v1`](API.md#method-recommend_songs_v1) | 每日推荐歌曲 v1 |

## 排行榜相关

| 方法 | 说明 |
|------|------|
| [`toplist`](API.md#method-toplist) | 所有榜单 |
| [`toplist_detail`](API.md#method-toplist_detail) | 所有榜单内容摘要 |
| [`toplist_detail_v2`](API.md#method-toplist_detail_v2) | 所有榜单内容摘要 V2 |
| [`toplist_artist`](API.md#method-toplist_artist) | 歌手榜 |
| [`top_song`](API.md#method-top_song) | 新歌速递 |
| [`top_album`](API.md#method-top_album) | 新碟上架 |
| [`top_artists`](API.md#method-top_artists) | 热门歌手 |
| [`top_mv`](API.md#method-top_mv) | MV 排行 |
| [`top_playlist`](API.md#method-top_playlist) | 歌单 (网友精选碟) |
| [`top_playlist_highquality`](API.md#method-top_playlist_highquality) | 获取精品歌单 |
| [`top_list`](API.md#method-top_list) | 排行榜详情 (旧版) |
| [`chart_detail`](API.md#method-chart_detail) | 排行榜详情 |
| [`chart_song_detail`](API.md#method-chart_song_detail) | 排行榜歌曲列表 |

## 云盘相关

| 方法 | 说明 |
|------|------|
| [`user_cloud`](API.md#method-user_cloud) | 云盘数据 |
| [`user_cloud_detail`](API.md#method-user_cloud_detail) | 云盘数据详情 |
| [`user_cloud_del`](API.md#method-user_cloud_del) | 云盘歌曲删除 |
| [`cloud`](API.md#method-cloud) | 云盘上传 |
| [`cloud_import`](API.md#method-cloud_import) | 云盘导入歌曲 |
| [`cloud_match`](API.md#method-cloud_match) | 云盘歌曲信息匹配纠正 |
| [`cloud_lyric_get`](API.md#method-cloud_lyric_get) | 获取云盘歌词 |
| [`cloud_upload_token`](API.md#method-cloud_upload_token) | 获取上传凭证 |
| [`cloud_upload_complete`](API.md#method-cloud_upload_complete) | 完成上传导入 |
| [`song_cloud_download`](API.md#method-song_cloud_download) | 云盘歌曲下载链接 |

## 私信相关

| 方法 | 说明 |
|------|------|
| [`msg_private`](API.md#method-msg_private) | 通知-私信 |
| [`msg_private_history`](API.md#method-msg_private_history) | 私信内容 |
| [`msg_comments`](API.md#method-msg_comments) | 通知-评论 |
| [`msg_forwards`](API.md#method-msg_forwards) | 通知-@我 |
| [`msg_notices`](API.md#method-msg_notices) | 通知-通知 |
| [`msg_recentcontact`](API.md#method-msg_recentcontact) | 最近联系人 |
| [`send_text`](API.md#method-send_text) | 发送私信 |
| [`send_song`](API.md#method-send_song) | 发送私信(带歌曲) |
| [`send_playlist`](API.md#method-send_playlist) | 发送私信(带歌单) |
| [`send_album`](API.md#method-send_album) | 发送私信(带专辑) |
| [`share_resource`](API.md#method-share_resource) | 分享到动态 |
| [`resource_like`](API.md#method-resource_like) | 资源点赞 |
| [`event`](API.md#method-event) | 获取动态列表 |
| [`event_del`](API.md#method-event_del) | 删除用户动态 |
| [`event_forward`](API.md#method-event_forward) | 转发用户动态 |

## VIP/会员

| 方法 | 说明 |
|------|------|
| [`vip_info`](API.md#method-vip_info) | 获取 VIP 信息 |
| [`vip_info_v2`](API.md#method-vip_info_v2) | 获取 VIP 信息(app 端) |
| [`vip_sign`](API.md#method-vip_sign) | 黑胶乐签打卡 |
| [`vip_sign_info`](API.md#method-vip_sign_info) | 黑胶乐签打卡信息 |
| [`vip_tasks`](API.md#method-vip_tasks) | VIP 任务 |
| [`vip_timemachine`](API.md#method-vip_timemachine) | 黑胶时光机 |
| [`vip_growthpoint`](API.md#method-vip_growthpoint) | VIP 成长值 |
| [`vip_growthpoint_details`](API.md#method-vip_growthpoint_details) | VIP 成长值获取记录 |
| [`vip_growthpoint_get`](API.md#method-vip_growthpoint_get) | 领取 VIP 成长值 |
| [`vip_growthpoint_getall`](API.md#method-vip_growthpoint_getall) | 一键领取会员成长值 |
| [`vip_sign_history`](API.md#method-vip_sign_history) | 黑胶乐签历史/状态 |
| [`vip_sign_detail`](API.md#method-vip_sign_detail) | 黑胶乐签打卡详情 |
| [`vip_tasks_v1`](API.md#method-vip_tasks_v1) | 会员任务（新版） |

## 云贝

| 方法 | 说明 |
|------|------|
| [`yunbei`](API.md#method-yunbei) | 云贝签到信息 |
| [`yunbei_info`](API.md#method-yunbei_info) | 云贝账户信息 |
| [`yunbei_sign`](API.md#method-yunbei_sign) | 云贝签到 |
| [`yunbei_today`](API.md#method-yunbei_today) | 云贝今日签到信息 |
| [`yunbei_expense`](API.md#method-yunbei_expense) | 云贝支出 |
| [`yunbei_receipt`](API.md#method-yunbei_receipt) | 云贝收入 |
| [`yunbei_tasks`](API.md#method-yunbei_tasks) | 云贝所有任务 |
| [`yunbei_tasks_todo`](API.md#method-yunbei_tasks_todo) | 云贝 todo 任务 |
| [`yunbei_task_finish`](API.md#method-yunbei_task_finish) | 云贝完成任务 |
| [`yunbei_rcmd_song`](API.md#method-yunbei_rcmd_song) | 云贝推歌 |
| [`yunbei_rcmd_song_history`](API.md#method-yunbei_rcmd_song_history) | 云贝推歌历史记录 |
| [`yunbei_task_list_v1`](API.md#method-yunbei_task_list_v1) | 云贝广告任务状态 |
| [`yunbei_task_recommend_song`](API.md#method-yunbei_task_recommend_song) | 云贝任务推荐歌曲 |
| [`yunbei_task_finish_v1`](API.md#method-yunbei_task_finish_v1) | 完成云贝广告任务 |

## 听歌足迹

| 方法 | 说明 |
|------|------|
| [`listen_data_year_report`](API.md#method-listen_data_year_report) | 年度听歌足迹 |
| [`listen_data_today_song`](API.md#method-listen_data_today_song) | 今日收听 |
| [`listen_data_total`](API.md#method-listen_data_total) | 总收听时长 |
| [`listen_data_realtime_report`](API.md#method-listen_data_realtime_report) | 本周/本月收听时长 |
| [`listen_data_report`](API.md#method-listen_data_report) | 周/月/年收听报告 |
| [`recent_listen_list`](API.md#method-recent_listen_list) | 最近听歌列表 |
| [`record_recent_song`](API.md#method-record_recent_song) | 最近播放-歌曲 |
| [`record_recent_album`](API.md#method-record_recent_album) | 最近播放-专辑 |
| [`record_recent_playlist`](API.md#method-record_recent_playlist) | 最近播放-歌单 |
| [`record_recent_dj`](API.md#method-record_recent_dj) | 最近播放-播客 |
| [`record_recent_video`](API.md#method-record_recent_video) | 最近播放-视频 |
| [`record_recent_voice`](API.md#method-record_recent_voice) | 最近播放-声音 |
| [`listen_data_song_play_rank`](API.md#method-listen_data_song_play_rank) | 歌曲播放排行 |

## 风格/曲风

| 方法 | 说明 |
|------|------|
| [`style_list`](API.md#method-style_list) | 曲风列表 |
| [`style_detail`](API.md#method-style_detail) | 曲风详情 |
| [`style_song`](API.md#method-style_song) | 曲风-歌曲 |
| [`style_album`](API.md#method-style_album) | 曲风-专辑 |
| [`style_artist`](API.md#method-style_artist) | 曲风-歌手 |
| [`style_playlist`](API.md#method-style_playlist) | 曲风-歌单 |
| [`style_preference`](API.md#method-style_preference) | 曲风偏好 |

## 数字专辑

| 方法 | 说明 |
|------|------|
| [`digital_album_detail`](API.md#method-digital_album_detail) | 数字专辑详情 |
| [`digital_album_ordering`](API.md#method-digital_album_ordering) | 购买数字专辑 |
| [`digital_album_purchased`](API.md#method-digital_album_purchased) | 我的数字专辑 |
| [`digital_album_sales`](API.md#method-digital_album_sales) | 数字专辑销量 |

## 声音/播客

| 方法 | 说明 |
|------|------|
| [`voice_upload`](API.md#method-voice_upload) | 播客上传声音 |
| [`voice_delete`](API.md#method-voice_delete) | 播客删除 |
| [`voice_detail`](API.md#method-voice_detail) | 播客声音详情 |
| [`voice_lyric`](API.md#method-voice_lyric) | 获取声音歌词 |
| [`voicelist_list`](API.md#method-voicelist_list) | 播客声音列表 |
| [`voicelist_detail`](API.md#method-voicelist_detail) | 播客列表详情 |
| [`voicelist_search`](API.md#method-voicelist_search) | 播客列表搜索 |
| [`voicelist_list_search`](API.md#method-voicelist_list_search) | 播客声音搜索 |
| [`voicelist_trans`](API.md#method-voicelist_trans) | 播客声音排序 |
| [`voicelist_my_created`](API.md#method-voicelist_my_created) | 我创建的播客声音 |

## 助眠解压

| 方法 | 说明 |
|------|------|
| [`sati_tag_list`](API.md#method-sati_tag_list) | 标签列表 |
| [`sati_resource_list`](API.md#method-sati_resource_list) | 标签下资源列表 |
| [`sati_resource_list_more`](API.md#method-sati_resource_list_more) | 同类推荐 |
| [`sati_resource_sub`](API.md#method-sati_resource_sub) | 收藏/取消收藏资源 |
| [`sati_resource_sub_list`](API.md#method-sati_resource_sub_list) | 收藏列表 |
| [`sati_timescene_resources_get`](API.md#method-sati_timescene_resources_get) | 时间场景推荐资源 |

## 音乐人

| 方法 | 说明 |
|------|------|
| [`musician_sign`](API.md#method-musician_sign) | 音乐人签到 |
| [`musician_tasks`](API.md#method-musician_tasks) | 音乐人任务 |
| [`musician_tasks_new`](API.md#method-musician_tasks_new) | 音乐人任务(新) |
| [`musician_vip_tasks`](API.md#method-musician_vip_tasks) | 音乐人黑胶会员任务 |
| [`musician_data_overview`](API.md#method-musician_data_overview) | 音乐人数据概况 |
| [`musician_play_trend`](API.md#method-musician_play_trend) | 音乐人播放趋势 |
| [`musician_cloudbean`](API.md#method-musician_cloudbean) | 账号云豆数 |
| [`musician_cloudbean_obtain`](API.md#method-musician_cloudbean_obtain) | 领取云豆 |

## 粉丝中心

| 方法 | 说明 |
|------|------|
| [`fanscenter_overview_get`](API.md#method-fanscenter_overview_get) | 粉丝中心概览 |
| [`fanscenter_trend_list`](API.md#method-fanscenter_trend_list) | 粉丝趋势列表 |
| [`fanscenter_basicinfo_age_get`](API.md#method-fanscenter_basicinfo_age_get) | 粉丝年龄分布 |
| [`fanscenter_basicinfo_gender_get`](API.md#method-fanscenter_basicinfo_gender_get) | 粉丝性别分布 |
| [`fanscenter_basicinfo_province_get`](API.md#method-fanscenter_basicinfo_province_get) | 粉丝省份分布 |

## UGC 百科

| 方法 | 说明 |
|------|------|
| [`ugc_song_get`](API.md#method-ugc_song_get) | 歌曲简要百科信息 |
| [`ugc_artist_get`](API.md#method-ugc_artist_get) | 歌手简要百科信息 |
| [`ugc_album_get`](API.md#method-ugc_album_get) | 专辑简要百科信息 |
| [`ugc_mv_get`](API.md#method-ugc_mv_get) | MV 简要百科信息 |
| [`ugc_detail`](API.md#method-ugc_detail) | 用户贡献内容 |
| [`ugc_artist_search`](API.md#method-ugc_artist_search) | 搜索歌手(百科) |
| [`ugc_user_devote`](API.md#method-ugc_user_devote) | 用户贡献条目、积分、云贝数量 |

## 一起听

| 方法 | 说明 |
|------|------|
| [`listentogether_room_create`](API.md#method-listentogether_room_create) | 创建一起听房间 |
| [`listentogether_room_check`](API.md#method-listentogether_room_check) | 检查一起听房间 |
| [`listentogether_accept`](API.md#method-listentogether_accept) | 接受一起听邀请 |
| [`listentogether_status`](API.md#method-listentogether_status) | 一起听状态 |
| [`listentogether_heatbeat`](API.md#method-listentogether_heatbeat) | 一起听心跳 |
| [`listentogether_play_command`](API.md#method-listentogether_play_command) | 一起听播放命令 |
| [`listentogether_sync_list_command`](API.md#method-listentogether_sync_list_command) | 一起听同步列表命令 |
| [`listentogether_sync_playlist_get`](API.md#method-listentogether_sync_playlist_get) | 获取一起听同步播放列表 |
| [`listentogether_end`](API.md#method-listentogether_end) | 结束一起听 |

## 广播电台

| 方法 | 说明 |
|------|------|
| [`broadcast_category_region_get`](API.md#method-broadcast_category_region_get) | 广播电台分类/地区信息 |
| [`broadcast_channel_list`](API.md#method-broadcast_channel_list) | 广播电台全部电台 |
| [`broadcast_channel_currentinfo`](API.md#method-broadcast_channel_currentinfo) | 广播电台信息 |
| [`broadcast_channel_collect_list`](API.md#method-broadcast_channel_collect_list) | 广播电台我的收藏 |
| [`broadcast_sub`](API.md#method-broadcast_sub) | 广播电台订阅 |

## 云小编

| 方法 | 说明 |
|------|------|
| [`middle_play_lottery_remain_chance`](API.md#method-middle_play_lottery_remain_chance) | 抽奖剩余次数 |
| [`middle_play_do_lottery`](API.md#method-middle_play_do_lottery) | 每日抽奖 |
| [`rep_ugc_activity_get`](API.md#method-rep_ugc_activity_get) | 云小编活动信息 |
| [`rep_ugc_activity_collect`](API.md#method-rep_ugc_activity_collect) | 领取任务积分 |
| [`rep_ugc_user_get`](API.md#method-rep_ugc_user_get) | 云小编用户详情 |
| [`rep_ugc_user_sign`](API.md#method-rep_ugc_user_sign) | 云小编每日签到 |
| [`rep_ugc_user_vip`](API.md#method-rep_ugc_user_vip) | 会员任务状态 |
| [`rep_ugc_user_collect_vip`](API.md#method-rep_ugc_user_collect_vip) | 领取一日会员 |
| [`rep_ugc_exam_info_get`](API.md#method-rep_ugc_exam_info_get) | 考试状态 |
| [`rep_ugc_exam_start`](API.md#method-rep_ugc_exam_start) | 开始考试 |
| [`rep_ugc_exam_question_single_get`](API.md#method-rep_ugc_exam_question_single_get) | 考试取题 |
| [`rep_ugc_exam_submit`](API.md#method-rep_ugc_exam_submit) | 提交答案 |
| [`rep_ugc_exam_result_get`](API.md#method-rep_ugc_exam_result_get) | 考试结果 |
| [`thinktank_audit_resource_detail`](API.md#method-thinktank_audit_resource_detail) | 获取审核任务 |
| [`thinktank_audit_resource_update`](API.md#method-thinktank_audit_resource_update) | 提交审核任务 |

## 其他

| 方法 | 说明 |
|------|------|
| [`hot_topic`](API.md#method-hot_topic) | 获取热门话题 |
| [`topic_detail`](API.md#method-topic_detail) | 获取话题详情 |
| [`topic_detail_event_hot`](API.md#method-topic_detail_event_hot) | 获取话题详情热门动态 |
| [`topic_sublist`](API.md#method-topic_sublist) | 收藏的专栏 |
| [`calendar`](API.md#method-calendar) | 音乐日历 |
| [`batch`](API.md#method-batch) | 批量请求接口 |
| [`api`](API.md#method-api) | 通用 API 请求 |
| [`inner_version`](API.md#method-inner_version) | 内部版本接口 |
| [`weblog`](API.md#method-weblog) | 日志上报 |
| [`eapi_decrypt`](API.md#method-eapi_decrypt) | EAPI 解密 |
| [`sign_happy_info`](API.md#method-sign_happy_info) | 乐签信息 |
| [`signin_progress`](API.md#method-signin_progress) | 签到进度 |
| [`summary_annual`](API.md#method-summary_annual) | 年度听歌报告 |
| [`threshold_detail_get`](API.md#method-threshold_detail_get) | 阈值详情获取 |
| [`creator_authinfo_get`](API.md#method-creator_authinfo_get) | 创作者认证信息获取 |
| [`sheet_list`](API.md#method-sheet_list) | 乐谱列表 |
| [`sheet_preview`](API.md#method-sheet_preview) | 乐谱内容 |
| [`aidj_content_rcmd`](API.md#method-aidj_content_rcmd) | 私人 DJ 推荐内容 |
| [`music_first_listen_info`](API.md#method-music_first_listen_info) | 回忆坐标 |
| [`verify_get_qr`](API.md#method-verify_get_qr) | 验证接口-二维码生成 |
| [`verify_qrcodestatus`](API.md#method-verify_qrcodestatus) | 验证接口-二维码检测 |
| [`ad_get`](API.md#method-ad_get) | 获取广告 |
| [`ad_listening_rights`](API.md#method-ad_listening_rights) | 免费听时长状态 |
| [`ad_listening_rights_gain`](API.md#method-ad_listening_rights_gain) | 领取免费听权益 |
| [`decrypt`](API.md#method-decrypt) | 通用解密 |
| [`lbs_city_code`](API.md#method-lbs_city_code) | 多级行政区划数据 |
