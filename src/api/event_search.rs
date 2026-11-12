use super::Query;
use crate::error::Result;
/// 动态搜索
/// 对应 Node.js module/event_search.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 动态搜索
    /// 对应 /event/search
    ///
    /// 参数：keyword（兼容 keywords）, limit（默认 30）, offset（默认 0）
    pub async fn event_search(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "keyword": query.get("keyword").or_else(|| query.get("keywords")).unwrap_or(""),
            "limit": query.get_or("limit", "30"),
            "offset": query.get_or("offset", "0")
        });
        self.request(
            "/api/event/search/list/get/v1",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
