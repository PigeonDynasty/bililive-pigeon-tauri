use std::error::Error;
use std::fs::{create_dir_all, metadata, read_dir, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use tauri::api::path::document_dir;
// 读取目录下文件
fn read_all_path(root_path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut path_list = vec![root_path
        .as_path()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()];
    let mut start_index = 0;
    loop {
        let list_len = path_list.len();
        for index in start_index..path_list.len() {
            let path = &path_list[index];
            if metadata(path)?.is_dir() {
                for child_dir in read_dir(&path)? {
                    path_list.push(String::from(
                        child_dir?.path().as_os_str().to_str().expect(""),
                    ));
                }
            }
        }
        if list_len == start_index {
            break;
        }
        start_index = list_len;
    }
    return Ok(path_list);
}

pub struct PluginPath {
    pub plugin_type: String,
    pub path: String,
}
// 读取插件目录下的插件
pub fn read_plugin() -> Vec<PluginPath> {
    let mut plugin_dir = document_dir().unwrap();
    plugin_dir.push("Bililive-tauri");
    plugin_dir.push("plugins");
    create_dir_all(&plugin_dir).unwrap();
    let plugins = read_all_path(&plugin_dir).unwrap();
    let mut res: Vec<PluginPath> = vec![];
    for path in plugins {
        let mut plugin_type = "";
        if path.ends_with(".dylib") {
            // 动态库
            plugin_type = "dylib";
        } else if path.ends_with(".js") {
            // js插件
            plugin_type = "js";
        }
        res.push(PluginPath {
            plugin_type: String::from(plugin_type),
            path: path,
        })
    }
    res
}
// 读取配置文件
pub struct Config {
    buf: File,
}
impl Config {
    pub fn new() -> Self {
        let mut config_path = document_dir().unwrap();
        config_path.push("Bililive-tauri");
        config_path.push("config.json");
        // let buf = File::create(config_path).expect("config create failed");
        let buf = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(config_path)
            .expect("config create failed");
        Self { buf }
    }
    pub fn read(&mut self) -> serde_json::Value {
        let mut json_str = String::new();
        self.buf.read_to_string(&mut json_str).unwrap();
        if json_str == "" {
            let default_json = serde_json::json!({"plugins":[]});
            self.write(&default_json);
            json_str = serde_json::to_string(&default_json).unwrap();
        }
        let config: serde_json::Value =
            serde_json::from_str(&json_str).expect("config.json read failed!");
        println!("{:?}", config);
        config
    }
    // 写入配置文件
    pub fn write(&mut self, json: &serde_json::Value) {
        self.buf
            .write(serde_json::to_string(&json).unwrap().as_bytes())
            .expect("config write failed");
    }
}
