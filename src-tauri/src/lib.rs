pub mod db;
pub mod fs;
pub mod packet;
pub mod plugin;
pub mod request;
pub mod txt;

use std::path::PathBuf;
use tauri::api::path::document_dir;

pub fn doc_dir() -> PathBuf {
    let mut path = document_dir().unwrap();
    path.push("BililivePigeon");
    path
}
pub fn plugin_dir() -> PathBuf {
    let mut path = doc_dir();
    path.push("plugins");
    path
}
pub fn db_path() -> PathBuf {
    let mut path = doc_dir();
    path.push("db");
    path
}

pub fn emoji_path() -> PathBuf {
    let mut path = doc_dir();
    path.push("emoji.json");
    path
}
