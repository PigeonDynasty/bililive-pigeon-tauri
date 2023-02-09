#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod danmaku;
mod request;

use tauri::Window;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
pub(crate) type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
use bililive_pigeon::plugin::{PluginData, PluginManager};
use bililive_pigeon::{db, doc_dir, plugin_dir, txt};
use once_cell::sync::Lazy;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::Mutex;

static PLUGIN_MANAGER: Lazy<Mutex<PluginManager>> = Lazy::new(|| Mutex::new(PluginManager::new()));
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 发起连接
#[tauri::command]
async fn connect(room_id: u32, window: Window) {
    create_dir_all(txt::room_path(&room_id)).unwrap();
    danmaku::new(room_id, &window).await;
}
// 断开连接
#[tauri::command]
async fn disconnect(room_id: u32, window: Window) {
    danmaku::disconnect(room_id, &window, "disconnect").await;
}
// 弹幕信息写入文件
#[tauri::command]
fn write_danmaku_txt(room_id: u32, date: String, data: Vec<String>) {
    txt::write_danmaku_txt(room_id, date, data);
}
// 获取插件目录
#[tauri::command]
fn get_plugin_dir() -> PathBuf {
    plugin_dir()
}
// 读取并加载插件 bol判断是否需要加载
#[tauri::command]
fn load_plugin_all(load: bool) -> Vec<PluginData> {
    let res = PLUGIN_MANAGER.lock().unwrap().load_plugin_all(load);
    res
}
// 加载插件
#[tauri::command]
fn load_plugin(name: String) {
    // PLUGIN_MANAGER
    //     .lock()
    //     .unwrap()
    //     .load_plugin(path, true)
    //     .unwrap();
    PLUGIN_MANAGER.lock().unwrap().load(&name);
}
// 卸载插件
#[tauri::command]
fn unload_plugin(name: String) {
    PLUGIN_MANAGER.lock().unwrap().unload(&name);
}
// 更新插件visible
#[tauri::command]
fn update_plugin_visible(path: String, visible: bool) {
    let mut visible_int: u8 = 0;
    if visible {
        visible_int = 1;
    }
    db::plugin::update_visible(&path, &visible_int);
}
// 获取连接历史
#[tauri::command]
fn get_history() -> Vec<db::history::DbHistory> {
    db::history::select_all()
}
fn main() {
    let doc_dir = doc_dir();
    create_dir_all(&doc_dir).unwrap();

    db::init();

    tauri::Builder::default()
        .setup(|app| {
            PLUGIN_MANAGER.lock().unwrap().set_handle(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            connect,
            disconnect,
            write_danmaku_txt,
            get_plugin_dir,
            load_plugin_all,
            load_plugin,
            unload_plugin,
            update_plugin_visible,
            get_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
