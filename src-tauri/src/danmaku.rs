use crate::request::Request;
use crate::WsStream;
use crate::PLUGIN_MANAGER;
use bililive_pigeon::packet::{decode, encode, Packet};
use futures_util::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::time::Duration;
use tauri::Window;
use tokio::{sync::mpsc, task::JoinHandle, time};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;
// 弹幕 线程池
static DANMAKU_POOL: Lazy<Mutex<HashMap<i32, JoinHandle<()>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub async fn new(room_id: i32, win: &Window) {
    let key = format!("stream-{}", room_id);
    let _request = Request::new();
    let room_res = _request.get_true_roomid(room_id).await;
    let _room_id = room_res["data"]["room_id"].as_i64().unwrap();
    win.emit(&key, _room_id).unwrap();

    let res = _request.get_danmaku_hosts(_room_id).await;
    let list = res["data"]["host_list"].as_array().unwrap().to_owned();
    let token = res["data"]["token"].as_str().unwrap();
    let ws_stream = try_connect_async(list).await.unwrap();

    win.emit(&key, "connected").unwrap();
    join(ws_stream.unwrap(), room_id, _room_id, token, win).await;
}

// 递归尝试连接
fn try_connect_async(
    mut host_list: Vec<serde_json::Value>,
) -> Pin<Box<dyn Future<Output = Result<Option<WsStream>, ()>> + Send>> {
    let host = host_list.pop().unwrap();
    let url = Url::parse(&format!(
        "wss://{}:{}/sub",
        host["host"].as_str().unwrap(),
        host["wss_port"]
    ))
    .unwrap();
    Box::pin(async move {
        match connect_async(url).await {
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
                    return try_connect_async(host_list).await;
                } else {
                    return Ok(None);
                }
            }
        }
    })
}
async fn join(ws_stream: WsStream, room_id: i32, _room_id: i64, token: &str, win: &Window) {
    let (mut wx, mut rx) = ws_stream.split();
    let raw = serde_json::to_string(&serde_json::json!({
            "uid": 0,
            "roomid": _room_id,
            "protover": 3,
            "platform": "bililive_pigeon",
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
pub async fn disconnect(room_id: i32, window: &Window, payload: &str) {
    let handle = DANMAKU_POOL.lock().unwrap().remove(&room_id);
    handle.unwrap().abort();
    let key = format!("stream-{}", room_id);
    window.emit(&key, payload).unwrap();
}
