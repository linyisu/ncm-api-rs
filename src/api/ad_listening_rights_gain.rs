use super::Query;
use crate::error::Result;
/// 看广告免费听歌 - 领取免费听权益
/// 对应 Node.js module/ad_listening_rights_gain.js
use crate::request::{ApiClient, ApiResponse, CheckToken, CryptoType};
use serde_json::{json, Map, Value};

/// 字段可能是 JSON 字符串也可能是对象
fn as_value(v: Option<&Value>) -> Option<Value> {
    match v? {
        Value::String(s) => serde_json::from_str(s).ok(),
        other => Some(other.clone()),
    }
}

fn get_path<'a>(v: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut cur = v;
    for key in path {
        cur = cur.get(key)?;
    }
    Some(cur)
}

fn get_string(v: &Value, path: &[&str]) -> Option<String> {
    get_path(v, path)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn opt_i64(query: &Query, key: &str) -> Option<i64> {
    query
        .get(key)
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<i64>().ok())
}

impl ApiClient {
    /// 领取看广告免费听/云贝权益
    /// 对应 /ad/listening/rights/gain
    ///
    /// 未显式传入 `reqUid` / `contextInfo` / `creativeType` 时，会先调用 `ad_get` 自动补齐。
    /// 参数：reqUid, contextInfo, creativeType, generalRightsInfo, rightsGainMethod 等，
    /// 其余字段见 Node.js 版说明。
    pub async fn ad_listening_rights_gain(&self, query: &Query) -> Result<ApiResponse> {
        let time = chrono::Utc::now().timestamp_millis();
        let mut req_uid = query.get_or("reqUid", "");
        let mut context_info = query.get("contextInfo").map(str::to_string);
        let mut creative_type = opt_i64(query, "creativeType");
        let mut general_rights_info = query.get("generalRightsInfo").map(str::to_string);
        let mut hint = Map::new();

        if req_uid.is_empty() || context_info.is_none() || creative_type.is_none() {
            let mut ad_query = query.clone();
            if ad_query.get("type_ids").is_none() {
                ad_query = ad_query.param("type_ids", r#"["400002_0"]"#);
            }
            if let Ok(ad_res) = self.ad_get(&ad_query).await {
                if let Some(ad) = ad_res
                    .body
                    .get("ads")
                    .and_then(Value::as_object)
                    .and_then(|m| m.values().next())
                {
                    if req_uid.is_empty() {
                        req_uid = as_value(ad.get("extJson"))
                            .and_then(|ext| get_string(&ext, &["contextInfo", "req_id"]))
                            .or_else(|| {
                                as_value(ad.get("adExtMap"))
                                    .and_then(|m| get_string(&m, &["req_id"]))
                            })
                            .or_else(|| {
                                as_value(ad.get("adLogId"))
                                    .and_then(|l| get_string(&l, &["requestId"]))
                            })
                            .or_else(|| get_string(ad, &["reqId"]))
                            .or_else(|| {
                                ad_res
                                    .body
                                    .get("extra")
                                    .and_then(|e| e.get("reqId"))
                                    .and_then(Value::as_str)
                                    .map(str::to_string)
                            })
                            .unwrap_or_default();
                    }

                    if context_info.is_none() {
                        context_info = as_value(ad.get("extJson"))
                            .and_then(|ext| ext.get("contextInfo").cloned())
                            .or_else(|| {
                                as_value(ad.get("adLogId"))
                                    .and_then(|log| log.get("contextInfo").cloned())
                            })
                            .or_else(|| ad.get("showContext").cloned())
                            .map(|ci| match ci {
                                Value::String(s) => s,
                                other => other.to_string(),
                            });
                    }

                    if creative_type.is_none() {
                        creative_type = ad.get("creativeType").and_then(|v| {
                            v.as_i64()
                                .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
                        });
                    }

                    if general_rights_info.is_none() {
                        general_rights_info = ad.get("generalRightsInfo").map(|v| match v {
                            Value::String(s) => s.clone(),
                            other => other.to_string(),
                        });
                    }

                    for key in ["listeningRightHintInfo", "generalRightsInfo"] {
                        if let Some(v) = as_value(ad.get(key)).and_then(|v| v.as_object().cloned())
                        {
                            for (k, v) in v {
                                hint.insert(k, v);
                            }
                        }
                    }
                }
            }
        }

        let hint_i64 = |key: &str| hint.get(key).and_then(Value::as_i64);
        let rights_gain_method = opt_i64(query, "rightsGainMethod")
            .or_else(|| hint_i64("rightsGainMethod"))
            .unwrap_or(2);

        let mut params = Map::new();
        let mut put = |key: &str, value: Value| {
            params.insert(key.to_string(), value);
        };

        put("reqUid", Value::String(req_uid));
        put(
            "exposureTime",
            json!(opt_i64(query, "exposureTime").unwrap_or(time)),
        );
        if let Some(uid) = opt_i64(query, "uid") {
            put("userId", json!(uid));
        }
        put(
            "clickTime",
            json!(opt_i64(query, "clickTime").unwrap_or(time)),
        );
        if let Some(v) = opt_i64(query, "extraRightsType").or_else(|| hint_i64("extraRightsType")) {
            put("extraRightsType", json!(v));
        }
        put(
            "playContinuously",
            Value::Bool(query.get("playContinuously").is_some()),
        );
        if let Some(v) = opt_i64(query, "source").or_else(|| hint_i64("source")) {
            put("source", json!(v));
        }
        if let Some(v) = creative_type {
            put("creativeType", json!(v));
        } else {
            put("creativeType", json!(36));
        }
        put("rightsGainMethod", json!(rights_gain_method));
        if let Some(v) =
            opt_i64(query, "extraRightsGainMethod").or_else(|| hint_i64("extraRightsGainMethod"))
        {
            put("extraRightsGainMethod", json!(v));
        }
        if let Some(v) = opt_i64(query, "extraRightsGainDuration")
            .or_else(|| hint_i64("extraRightsGainDuration"))
        {
            put("extraRightsGainDuration", json!(v));
        }
        if let Some(v) =
            opt_i64(query, "nextRightsGainDuration").or_else(|| hint_i64("nextRightsGainDuration"))
        {
            put("nextRightsGainDuration", json!(v));
        }
        if let Some(v) = opt_i64(query, "rightsGainType").or_else(|| hint_i64("rightsGainType")) {
            put("rightsGainType", json!(v));
        }
        if let Some(v) =
            opt_i64(query, "rightsGainDuration").or_else(|| hint_i64("rightsGainDuration"))
        {
            put("rightsGainDuration", json!(v));
        }
        if let Some(v) = opt_i64(query, "gainMethodStep") {
            put("gainMethodStep", json!(v));
        }
        if let Some(v) = general_rights_info {
            put("generalRightsInfo", Value::String(v));
        }
        if let Some(v) = query.get("rightsExtJson").map(str::to_string).or_else(|| {
            hint.get("rightsExtJson")
                .and_then(Value::as_str)
                .map(str::to_string)
        }) {
            put("rightsExtJson", Value::String(v));
        }
        if let Some(v) = query
            .get("appInfo")
            .and_then(|s| serde_json::from_str(s).ok())
        {
            put("appInfo", v);
        }
        if let Some(v) = context_info {
            put("contextInfo", Value::String(v));
        }
        if let Some(v) = opt_i64(query, "installed") {
            put("installed", json!(v));
        }
        if rights_gain_method == 3 || rights_gain_method == 6 {
            put(
                "sniffTime",
                json!(opt_i64(query, "sniffTime").unwrap_or(time)),
            );
        }

        let req_param = Value::Object(params).to_string();
        let data = json!({ "reqParam": req_param });
        let res = self
            .request(
                "/api/ad/listening/rights/gain",
                data,
                query.to_option_token(CryptoType::Xeapi, CheckToken::V3),
            )
            .await?;

        Ok(ApiResponse {
            status: 200,
            body: json!({ "code": 200, "data": res.body }),
            cookie: res.cookie,
        })
    }
}
