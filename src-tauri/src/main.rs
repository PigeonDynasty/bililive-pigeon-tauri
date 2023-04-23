#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod command;
mod danmaku;

use bililive_pigeon::plugin::PluginManager;
use bililive_pigeon::{db, doc_dir};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
pub(crate) type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
use std::fs::create_dir_all;

static PLUGIN_MANAGER: Lazy<Mutex<PluginManager>> = Lazy::new(|| Mutex::new(PluginManager::new()));

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
            command::greet,
            command::connect,
            command::disconnect,
            command::write_danmaku_txt,
            command::get_plugin_dir,
            command::load_plugin_all,
            command::load_plugin,
            command::unload_plugin,
            command::update_plugin_visible,
            command::get_history,
            command::get_emojis,
            command::get_setting,
            command::update_setting
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
