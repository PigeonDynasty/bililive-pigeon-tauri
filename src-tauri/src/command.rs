// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use crate::danmaku;
use crate::PLUGIN_MANAGER;
use base64::{engine::general_purpose, Engine};
use bililive_pigeon_lib::plugin::PluginData;
use bililive_pigeon_lib::request::Request;
use bililive_pigeon_lib::{db, plugin_dir, txt};
use rand::rngs::OsRng;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use std::fs::create_dir_all;
use std::path::PathBuf;
use tauri::Window;

// 清理弹幕线程池
#[tauri::command]
pub async fn clear_danmaku_pool() {
    danmaku::clear_pool();
}
// 发起连接
#[tauri::command]
pub async fn connect(room_id: u32, cookie: String, window: Window) {
    create_dir_all(txt::room_path(&room_id)).expect("create_dir_all failed");
    danmaku::new(room_id, &cookie, &window).await;
}
// 断开连接
#[tauri::command]
pub async fn disconnect(room_id: u32, window: Window) {
    danmaku::disconnect(room_id, &window, "disconnect").await;
}
// 弹幕信息写入文件
#[tauri::command]
pub fn write_danmaku_txt(room_id: u32, date: String, data: Vec<String>) {
    txt::write_danmaku_txt(room_id, date, data);
}
// 获取插件目录
#[tauri::command]
pub fn get_plugin_dir() -> PathBuf {
    plugin_dir()
}
// 读取并加载插件 bol判断是否需要加载
#[tauri::command]
pub fn load_plugin_all(load: bool) -> Vec<PluginData> {
    let res = PLUGIN_MANAGER.lock().unwrap().load_plugin_all(load);
    res
}
// 加载插件
#[tauri::command]
pub fn load_plugin(name: String) {
    // PLUGIN_MANAGER
    //     .lock()
    //     .unwrap()
    //     .load_plugin(path, true)
    //     .unwrap();
    PLUGIN_MANAGER.lock().unwrap().load(&name);
}
// 卸载插件
#[tauri::command]
pub fn unload_plugin(name: String) {
    PLUGIN_MANAGER.lock().unwrap().unload(&name);
}
// 更新插件visible
#[tauri::command]
pub fn update_plugin_visible(path: String, visible: bool) {
    let mut visible_int: u8 = 0;
    if visible {
        visible_int = 1;
    }
    db::plugin::update_visible(&path, &visible_int);
}
// 获取连接历史
#[tauri::command]
pub fn get_history() -> Vec<db::history::DbHistory> {
    db::history::select_all()
}
// 获取emoji
#[tauri::command]
pub fn get_emojis(emojis: Vec<&str>) -> Vec<db::emoji::DbEmoji> {
    db::emoji::select_by_emojis(emojis)
}

// 获取房间设置
#[tauri::command]
pub fn get_setting(room_id: u32) -> Option<db::setting::DbSetting> {
    db::setting::select_by_roomid(room_id)
}
// 更新房间设置
#[tauri::command]
pub fn update_setting(room_id: u32, config: &str) {
    db::setting::update(room_id, config);
}

// 获取buvid
#[tauri::command]
pub async fn get_buvid() -> serde_json::Value {
    Request::new().get_buvid().await
}

//获取b_nut
#[tauri::command]
pub async fn get_b_nut() -> serde_json::Value {
    Request::new().get_b_nut().await
}

// 获取验证码
#[tauri::command]
pub async fn get_captcha(web_location: String) -> serde_json::Value {
    Request::new().get_captcha(web_location).await
}

// 根据密码登录
#[tauri::command]
pub async fn login_by_password(
    ua: String,
    cookie: String,
    username: String,
    password: String,
    web_location: String,
    token: String,
    challenge: String,
    validate: String,
    seccode: String,
) -> serde_json::Value {
    let req = Request::new();
    let rsa_json = req.get_login_key(web_location).await;
    let hash = rsa_json["data"]["hash"].to_string().replace("\"", "");
    let key = rsa_json["data"]["key"]
        .to_string()
        .replace("\\n", "\n")
        .replace("\"", "");
    let public_key = RsaPublicKey::from_public_key_pem(&key).expect("Failed to create public key");
    let mut rng = OsRng;
    println!("&hash + &password: {:?}", hash.clone() + &password);
    let encrypted_data = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, (hash + &password).as_bytes())
        .expect("Failed to encrypt");
    let encrypted_password = general_purpose::STANDARD.encode(&encrypted_data);
    let ua_str = if ua.is_empty() {
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.207.132.170 Safari/537.36 Edg/112.0.1722.68".to_string()
    } else {
        ua
    };
    req.login_by_password(
        ua_str,
        cookie,
        username,
        encrypted_password,
        token,
        challenge,
        validate,
        seccode,
    )
    .await
}
