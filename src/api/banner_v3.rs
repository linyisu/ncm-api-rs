use super::Query;
use crate::error::Result;
/// 首页轮播（v3）
/// 对应 Node.js module/banner_v3.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 首页轮播（v3）
    /// 对应 /banner/v3
    ///
    /// 参数：clientType 客户端类型（默认 android）
    pub async fn banner_v3(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "clientType": query.get_or("clientType", "android")
        });
        self.request(
            "/api/banner/get/v3",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
