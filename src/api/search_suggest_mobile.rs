use super::Query;
use crate::error::Result;
/// 搜索联想词（App 移动端版）
/// 对应 Node.js module/search_suggest_mobile.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 搜索联想词（App 移动端版）
    /// 对应 /search/suggest/mobile
    ///
    /// 参数：keyword（必须是 keyword，兼容 keywords）
    pub async fn search_suggest_mobile(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "keyword": query.get("keyword").or_else(|| query.get("keywords")).unwrap_or("")
        });
        self.request(
            "/api/search/suggest/keyword/get",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
