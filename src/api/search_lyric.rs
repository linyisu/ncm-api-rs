use super::Query;
use crate::error::Result;
/// 按歌词搜索
/// 对应 Node.js module/search_lyric.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 按歌词搜索
    /// 对应 /search/lyric
    ///
    /// 参数：keyword（兼容 keywords）, limit（默认 30）, offset（默认 0）
    pub async fn search_lyric(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "keyword": query.get("keyword").or_else(|| query.get("keywords")).unwrap_or(""),
            "limit": query.get_or("limit", "30"),
            "offset": query.get_or("offset", "0")
        });
        self.request(
            "/api/search/resource/lyric",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
