use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::api::http::{
    header::{HeaderMap, COOKIE, REFERER, USER_AGENT},
    Body::Form,
    Client, ClientBuilder, FormBody, FormPart, HttpRequestBuilder, ResponseType,
};
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
    fn header_map_to_json(&self, headers: &HeaderMap) -> Value {
        let mut map = HashMap::new();

        for (key, value) in headers.iter() {
            let key_str = key.as_str().to_string();
            let value_str = value
                .to_str()
                .unwrap_or("Non-ASCII header value")
                .to_string();

            // 如果键已经存在，合并值
            match map.get_mut(&key_str) {
                None => {
                    map.insert(key_str, value_str);
                }
                Some(existing_value) => {
                    // 合并为&&分隔的字符串
                    *existing_value = format!("{}&&{}", existing_value, value_str);
                }
            }
        }

        // 将 HashMap 转换为 JSON
        json!(map)
    }

    async fn get(
        &self,
        request_builder: HttpRequestBuilder,
        response_type: Option<ResponseType>,
    ) -> Value {
        let request = request_builder.response_type(response_type.unwrap_or(ResponseType::Json));
        match self.client.send(request).await {
            Ok(response) => {
                let headers = response.headers().clone();
                let result = response.read().await.unwrap();
                if result.status == 200 {
                    json!({
                            "header":self.header_map_to_json(&headers),
                            "data":  result.data,
                    })
                } else {
                    panic!("error while getting data from bilibili");
                }
            }
            Err(error) => panic!("error while sending http request: {}", error),
        }
    }

    // 获取wss服务器列表
    pub async fn get_danmaku_hosts(&self, id: u64, cookie: &str) -> Value {
        let mut query_map = HashMap::new();
        let mut header_map = HeaderMap::new();
        query_map.insert("id".to_string(), id.to_string());
        query_map.insert("type".to_string(), "0".to_string());
        header_map.insert(COOKIE, cookie.parse().unwrap());
        let request_builder = HttpRequestBuilder::new(
            "GET",
            "https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo",
        )
        .unwrap()
        .query(query_map)
        .headers(header_map);
        let result = self.get(request_builder, None).await;
        json!(result["data"])
    }
    // 获取真实房间号
    // pub async fn get_true_roomid(&self, id: u32) -> Value {
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
    // pub async fn get_room_play_info(&self, id: u32) -> Value {
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
    pub async fn get_room_info(&self, id: u32) -> Value {
        let mut query_map = HashMap::new();
        query_map.insert("room_id".to_string(), id.to_string());
        let request_builder = HttpRequestBuilder::new(
            "GET",
            "https://api.live.bilibili.com/xlive/web-room/v1/index/getInfoByRoom",
        )
        .unwrap()
        .query(query_map);
        let result = self.get(request_builder, None).await;
        json!(result["data"])
    }
    // emoji列表 需要登录
    // pub async fn get_emoji_list(&self) -> Value {
    //     let mut query_map = HashMap::new();
    //     query_map.insert("platform".to_string(), "pc".to_string());
    //     query_map.insert("room_id".to_string(), 5440.to_string());
    //     let request_builder = HttpRequestBuilder::new(
    //         "GET",
    //         "https://api.live.bilibili.com/xlive/web-ucenter/v2/emoticon/GetEmoticons",
    //     )
    //     .unwrap()
    //     .query(query_map);
    //     self.get(request_builder).await
    // }
    // 获取buvid
    pub async fn get_buvid(&self) -> Value {
        let request_builder =
            HttpRequestBuilder::new("GET", "https://api.bilibili.com/x/frontend/finger/spi")
                .unwrap();
        let result = self.get(request_builder, None).await;
        json!(result["data"])
    }
    // 获取b_nut
    pub async fn get_b_nut(&self) -> Value {
        let request_builder = HttpRequestBuilder::new("HEAD", "https://www.bilibili.com/").unwrap();
        let result = self.get(request_builder, Some(ResponseType::Text)).await;
        json!(result["header"])
    }
    // 申请captcha验证码
    pub async fn get_captcha(&self, web_location: String) -> Value {
        let mut query_map = HashMap::new();
        query_map.insert("source".to_string(), "main_web".to_string());
        query_map.insert("web_location".to_string(), web_location);
        let request_builder = HttpRequestBuilder::new(
            "GET",
            "https://passport.bilibili.com/x/passport-login/captcha",
        )
        .unwrap()
        .query(query_map);
        let result = self.get(request_builder, None).await;
        json!(result["data"])
    }
    // 申请密码登录的pubKey&sault
    pub async fn get_login_key(&self, web_location: String) -> Value {
        let mut query_map = HashMap::new();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        query_map.insert("_".to_string(), time.to_string());
        query_map.insert("web_location".to_string(), web_location);

        let request_builder = HttpRequestBuilder::new(
            "GET",
            "https://passport.bilibili.com/x/passport-login/web/key",
        )
        .unwrap()
        .query(query_map);
        let result = self.get(request_builder, None).await;
        json!(result["data"])
    }
    // 登录
    pub async fn login_by_password(
        &self,
        ua: String,
        cookie: String,
        username: String,
        password: String,
        token: String,
        challenge: String,
        validate: String,
        seccode: String,
    ) -> Value {
        let mut header_map = HeaderMap::new();
        header_map.insert(USER_AGENT, ua.parse().unwrap());
        header_map.insert(COOKIE, cookie.parse().unwrap());
        header_map.insert(
            REFERER,
            "https://passport.bilibili.com/login".parse().unwrap(),
        );
        let mut form_data = HashMap::new();
        form_data.insert("username".to_string(), FormPart::Text(username));
        form_data.insert("password".to_string(), FormPart::Text(password));
        form_data.insert("keep".to_string(), FormPart::Text("0".to_string()));
        form_data.insert("token".to_string(), FormPart::Text(token));
        form_data.insert("challenge".to_string(), FormPart::Text(challenge));
        form_data.insert("validate".to_string(), FormPart::Text(validate));
        form_data.insert("seccode".to_string(), FormPart::Text(seccode));
        let form_body = FormBody::new(form_data);
        let request_builder = HttpRequestBuilder::new(
            "POST",
            "https://passport.bilibili.com/x/passport-login/web/login",
        )
        .unwrap()
        .headers(header_map)
        .body(Form(form_body));
        self.get(request_builder, None).await
    }
}
