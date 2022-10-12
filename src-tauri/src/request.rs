use std::collections::HashMap;
use tauri::api::http::{Client, ClientBuilder, HttpRequestBuilder, ResponseType};
pub struct Request {
    client: Client,
}
impl Request {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .max_redirections(3)
            .build()
            .expect("error while building http client");
        Self { client }
    }
    async fn get(&self, request_builder: HttpRequestBuilder) -> serde_json::Value {
        let request = request_builder.response_type(ResponseType::Json);
        match self.client.send(request).await {
            Ok(response) => {
                let result = response.read().await.unwrap();
                if result.status == 200 {
                    result.data
                } else {
                    panic!("error while getting data from bilibili");
                }
            }
            Err(error) => panic!("error while sending http request: {}", error),
        }
    }

    // 获取wss服务器列表
    pub async fn get_danmaku_hosts(&self, id: i32) -> serde_json::Value {
        let mut query_map = HashMap::new();
        query_map.insert("id".to_string(), id.to_string());
        query_map.insert("type".to_string(), "0".to_string());
        let request_builder = HttpRequestBuilder::new(
            "GET",
            "https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo",
        )
        .unwrap()
        .query(query_map);
        self.get(request_builder).await
    }
}
