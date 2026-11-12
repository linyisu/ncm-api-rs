use super::Query;
use crate::error::Result;
/// 评论默认文案
/// 对应 Node.js module/comment_default_text.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 评论默认文案
    /// 对应 /comment/default/text
    ///
    /// 参数：threadId 评论区 ID
    pub async fn comment_default_text(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "threadId": query.get_or("threadId", "")
        });
        self.request(
            "/api/comment/default/text",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
