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
use bililive_pigeon::plugin::PluginManager;
use once_cell::sync::Lazy;
use std::sync::Mutex;
static PLUGIN_MANAGER: Lazy<Mutex<PluginManager>> = Lazy::new(|| Mutex::new(PluginManager::new()));

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 发起连接
#[tauri::command]
async fn connect(room_id: i32, window: Window) {
    danmaku::new(room_id, &window).await;
}
// 断开连接
#[tauri::command]
async fn disconnect(room_id: i32, window: Window) {
    danmaku::disconnect(room_id, &window, "disconnect").await;
}
// 加载插件
#[tauri::command]
fn load_plugin() {
    PLUGIN_MANAGER.lock().unwrap().load_plugin("/Users/takenokos/Documents/Rust/tauri-plugin-test/target/debug/libtauri_plugin_test.dylib").unwrap();
}
// 卸载插件
#[tauri::command]
fn unload_plugin() {
    PLUGIN_MANAGER.lock().unwrap().unload_all();
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            PLUGIN_MANAGER.lock().unwrap().set_handle(app.handle());
            PLUGIN_MANAGER.lock().unwrap().load_plugin_all();
            // PLUGIN_MANAGER.lock().unwrap().load_plugin("/Users/takenokos/Documents/Rust/tauri-plugin-test/target/debug/libtauri_plugin_test.dylib").unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            connect,
            disconnect,
            load_plugin,
            unload_plugin
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
