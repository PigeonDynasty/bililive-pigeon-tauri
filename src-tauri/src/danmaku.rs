use crate::db::{gift as db_gift, history as db_history};
use crate::WsStream;
use crate::PLUGIN_MANAGER;
use bililive_pigeon_lib::packet::{decode, encode, Packet};
use bililive_pigeon_lib::request::Request;
use futures_util::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::time::Duration;
use tauri::api::http::header::{HeaderValue, COOKIE};
use tauri::Window;
use tokio::{sync::mpsc, task::JoinHandle, time};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;
// 弹幕 线程池
static DANMAKU_POOL: Lazy<Mutex<HashMap<u32, JoinHandle<()>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn clear_pool() {
    for (_key, handle) in DANMAKU_POOL.lock().unwrap().iter() {
        handle.abort();
    }
    DANMAKU_POOL.lock().unwrap().clear();
}

pub async fn new(room_id: u32, cookie: &str, win: &Window) {
    let key = format!("stream-{}", room_id);
    let _request = Request::new();
    let room_res = _request.get_room_info(room_id).await;
    let _room_id = room_res["data"]["room_info"]["room_id"].as_u64().unwrap();
    let uid = room_res["data"]["room_info"]["uid"].to_string();
    let uname = room_res["data"]["anchor_info"]["base_info"]["uname"]
        .as_str()
        .unwrap();
    win.emit(
        &key,
        serde_json::json!({
                "room_id": room_id,
                "true_room_id":_room_id,
                "uid":uid,
                "uname": uname
        }),
    )
    .unwrap();
    db_history::update(room_id, &uid, uname);

    // 检查线程池 防止重复连接
    if DANMAKU_POOL.lock().unwrap().contains_key(&room_id) {
        win.emit(&key, "connected").unwrap();
        win.emit(&key, "joined").unwrap();
        panic!("{}已连接", room_id);
    }

    let res = _request.get_danmaku_hosts(_room_id, cookie).await;
    let list = res["data"]["host_list"].as_array().unwrap().to_owned();
    let token = res["data"]["token"].as_str().unwrap();
    // let sessdata = Regex::new(r"\bSESSDATA=([^;]+)\b")
    //     .unwrap()
    //     .captures(cookie)
    //     .unwrap()
    //     .get(1)
    //     .unwrap()
    //     .as_str();
    // println!("sessdata={:?}", sessdata);
    let ws_stream = try_connect_async(list, cookie).await.unwrap();

    win.emit(&key, "connected").unwrap();

    let buvid = Regex::new(r"\bbuvid3=([^;]+)\b")
        .unwrap()
        .captures(cookie)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let uid = Regex::new(r"\bDedeUserID=([^;]+)\b")
        .unwrap()
        .captures(cookie)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    join(
        ws_stream.unwrap(),
        uid,
        room_id,
        _room_id,
        buvid,
        token,
        win,
    )
    .await;
}

// 递归尝试连接
fn try_connect_async(
    mut host_list: Vec<serde_json::Value>,
    cookie: &str,
) -> Pin<Box<dyn Future<Output = Result<Option<WsStream>, ()>> + Send>> {
    let host = host_list.pop().unwrap();
    let url = Url::parse(&format!(
        "wss://{}:{}/sub",
        host["host"].as_str().unwrap(),
        host["wss_port"]
    ))
    .unwrap();
    let mut req = url.into_client_request().unwrap();

    // let _cookie =
    //     HeaderValue::from_str(&format!("SESSDATA={}", sessdata)).expect("invalid sessdata");
    let headers = HeaderValue::from_str(cookie).expect("invalid cookie");
    req.headers_mut().append(COOKIE, headers);
    let _cookie = cookie.to_owned();
    Box::pin(async move {
        match connect_async(req).await {
            Ok((stream, _response)) => {
                // 打印连接情况
                // println!("Status code: {}", response.status());
                // for (ref header, _value) in response.headers() {
                //   println!("{}: {:?}", header, _value);
                // }
                Ok(Some(stream))
            }
            Err(_e) => {
                println!("Error={:?}", _e);
                if host_list.len() > 0 {
                    return try_connect_async(host_list, &_cookie).await;
                    // return Ok(None);
                } else {
                    return Ok(None);
                }
            }
        }
    })
}
async fn join(
    ws_stream: WsStream,
    uid: &str,
    room_id: u32,
    _room_id: u64,
    buvid: &str,
    token: &str,
    win: &Window,
) {
    let (mut wx, mut rx) = ws_stream.split();
    let raw = serde_json::to_string(&serde_json::json!({
            "uid":uid.parse::<i32>().unwrap(),
            // "uid": 0,
            "roomid": _room_id,
            "protover": 3,
            "buvid":buvid,
            "platform": "web",
            "type":2,
            "key": token,
    }))
    .unwrap();
    // 发送进房消息
    wx.send(Message::Binary(encode(&raw, 7))).await.unwrap();
    // 进房成功
    let _auth_reply = match rx.next().await {
        Some(Ok(Message::Binary(_auth_reply_bin))) => {
            println!("join success: {:?}", room_id);
            let key = format!("stream-{}", room_id);
            win.emit(&key, "joined").unwrap();
        }
        other @ _ => {
            println!("other={:?}", other);
        }
    };
    // mpsc 线程通信
    let (pack_tx, mut pack_rx) = mpsc::channel::<Vec<Packet>>(64);
    // 心跳包定时时长
    let mut interval = time::interval(Duration::from_secs(30));
    let handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                msg = rx.next() => {
                    match msg{
                        Some(msg) => {
                            match msg.unwrap(){
                                Message::Binary(bin) => {
                                    let packet = decode(bin);
                                    for p in &packet{
                                        match p.op{
                                            5=>{
                                                match p.body["cmd"].as_str().unwrap(){
                                                    "SEND_GIFT"=>{
                                                        let data=&p.body["data"];
                                                        db_gift::update(
                                                            &room_id,
                                                            data["timestamp"].as_u64().unwrap(),
                                                            &data["uid"].to_string(),
                                                            data["uname"].as_str().unwrap(),
                                                            data["giftName"].as_str().unwrap(),
                                                            data["num"].as_u64().unwrap(),
                                                            data["coin_type"].as_str().unwrap()
                                                        );
                                                    }
                                                    _=>{}
                                                }
                                            }
                                            _=>{}
                                        }
                                    }
                                    pack_tx.send(packet).await.unwrap_or_default();
                                }
                                Message::Close(_f) => {
                                    let packet = vec![Packet {
                                    ver: 0,
                                    op: 0,
                                    body: serde_json::json!({"close":"close"}),
                                    }];
                                    pack_tx.send(packet).await.unwrap_or_default();
                                }
                                Message::Ping(_) | Message::Pong(_) => {}
                                _msg @ _ => {}
                            }
                        }
                        None => break,
                    }
                }
                _ = interval.tick() => {
                    wx.send(Message::Binary(encode("", 2))).await.unwrap();
                }
            }
        }
    });
    DANMAKU_POOL.lock().unwrap().insert(room_id, handle);
    while let Some(mut packets) = pack_rx.recv().await {
        for packet in packets.clone() {
            if packet.op == 0 {
                disconnect(room_id, &win, "closed").await;
                break;
            }
        }
        PLUGIN_MANAGER.lock().unwrap().send(&mut packets);
        let key = format!("danmaku-{}", room_id);
        win.emit(&key, packets).unwrap();
    }
}
pub async fn disconnect(room_id: u32, window: &Window, payload: &str) {
    let handle = DANMAKU_POOL.lock().unwrap().remove(&room_id);
    handle.unwrap().abort();
    let key = format!("stream-{}", room_id);
    window.emit(&key, payload).unwrap();
}
