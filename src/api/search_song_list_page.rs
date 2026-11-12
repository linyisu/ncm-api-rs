use super::Query;
use crate::error::Result;
/// 新版单曲搜索（分页）
/// 对应 Node.js module/search_song_list_page.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 新版单曲搜索（分页）
    /// 对应 /search/song/list/page
    ///
    /// 参数：keyword（兼容 keywords）, limit（默认 30）, offset（默认 0）, scene（默认 normal）
    pub async fn search_song_list_page(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "keyword": query.get("keyword").or_else(|| query.get("keywords")).unwrap_or(""),
            "limit": query.get_or("limit", "30"),
            "offset": query.get_or("offset", "0"),
            "scene": query.get_or("scene", "normal")
        });
        self.request(
            "/api/search/song/list/page",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
