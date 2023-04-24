// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use crate::danmaku;
use crate::PLUGIN_MANAGER;
use bililive_pigeon::plugin::PluginData;
use bililive_pigeon::{db, plugin_dir, txt};
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
pub async fn connect(room_id: u32, window: Window) {
    create_dir_all(txt::room_path(&room_id)).unwrap();
    danmaku::new(room_id, &window).await;
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
