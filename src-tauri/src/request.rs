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
    pub async fn get_danmaku_hosts(&self, id: u64) -> serde_json::Value {
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
    // 获取真实房间号
    // pub async fn get_true_roomid(&self, id: u32) -> serde_json::Value {
    //     let mut query_map = HashMap::new();
    //     query_map.insert("id".to_string(), id.to_string());
    //     let request_builder = HttpRequestBuilder::new(
    //         "GET",
    //         "https://api.live.bilibili.com/room/v1/Room/room_init",
    //     )
    //     .unwrap()
    //     .query(query_map);
    //     self.get(request_builder).await
    // }
    // 获取房间直播信息
    // pub async fn get_room_play_info(&self, id: u32) -> serde_json::Value {
    //     let mut query_map = HashMap::new();
    //     query_map.insert("room_id".to_string(), id.to_string());
    //     query_map.insert("protocol".to_string(), "0,1".to_string());
    //     query_map.insert("format".to_string(), "0,1".to_string());
    //     query_map.insert("codec".to_string(), "0,1".to_string());
    //     query_map.insert("qn".to_string(), "0".to_string());
    //     query_map.insert("platform".to_string(), "web".to_string());
    //     query_map.insert("ptype".to_string(), "8".to_string());
    //     query_map.insert("dolby".to_string(), "5".to_string());
    //     query_map.insert("panorama".to_string(), "1".to_string());
    //     let request_builder = HttpRequestBuilder::new(
    //         "GET",
    //         "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo",
    //     )
    //     .unwrap()
    //     .query(query_map);
    //     self.get(request_builder).await
    // }
    // 获取房间信息
    pub async fn get_room_info(&self, id: u32) -> serde_json::Value {
        let mut query_map = HashMap::new();
        query_map.insert("room_id".to_string(), id.to_string());
        let request_builder = HttpRequestBuilder::new(
            "GET",
            "https://api.live.bilibili.com/xlive/web-room/v1/index/getInfoByRoom",
        )
        .unwrap()
        .query(query_map);
        self.get(request_builder).await
    }
}
