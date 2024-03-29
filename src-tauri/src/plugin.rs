use crate::db::plugin as db_plugin;
use crate::fs;
use crate::packet::Packet;
use libloading::{Library, Symbol};
use serde::Serialize;
use std::any::Any;
use std::ffi::OsStr;
use tauri::AppHandle;
// A plugin which allows you to add extra functionality to the REST client.
pub trait Plugin: Any + Send + Sync {
    // Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    fn author(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn contact(&self) -> &'static str;
    // A callback fired immediately after the plugin is loaded. Usually used
    // for initialization.
    fn load(&self, _handle: &Option<AppHandle>) {}
    // A callback fired immediately before the plugin is unloaded. Use this if
    // you need to do any cleanup.
    fn unload(&self, _handle: &Option<AppHandle>) {}
    // Inspect (and possibly mutate) the request before it is sent.
    fn send(&self, _request: &mut Vec<Packet>, _handle: &Option<AppHandle>) {}
}
#[derive(Serialize)]
pub struct PluginData {
    name: Option<String>,
    author: Option<String>,
    description: Option<String>,
    contact: Option<String>,
    plugin_type: PluginType,
    path: String,
    visible: bool,
}
pub struct PluginInfo {
    name: Option<String>,
    author: Option<String>,
    description: Option<String>,
    contact: Option<String>,
}
struct BoxPlugin {
    visible: bool,
    plugin: Box<dyn Plugin>,
}
pub struct PluginManager {
    handle: Option<AppHandle>,
    plugins: Vec<BoxPlugin>,
    loaded_libraries: Vec<Library>,
}
#[allow(dropping_references)]
impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            handle: None,
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }
    pub fn set_handle(&mut self, handel: AppHandle) {
        self.handle = Some(handel)
    }
    /**
     * @description: 加载插件目录
     * @param {bool} need_load true 初始化 需要加载插件 false 只获取目录
     * @return {*}
     */
    pub fn load_plugin_all(&mut self, need_load: bool) -> Vec<PluginData> {
        drop(&self);
        self.plugins = vec![];
        self.loaded_libraries = vec![];
        // 开始加载
        let plugins = read_plugin_dir();
        let config = db_plugin::select_all();
        let mut res = vec![];
        for plugin in plugins {
            let mut bol = false;
            let mut plugin_info = None;
            for plugin_config in &config {
                if plugin_config.path == plugin.path && plugin_config.visible == 1 {
                    bol = true;
                    break;
                }
            }
            match plugin.plugin_type {
                PluginType::Dylib => {
                    let info = self.load_plugin(&plugin.path, need_load && bol).unwrap();
                    plugin_info = Some(info);
                }
                PluginType::Js => {}
            }
            match plugin_info {
                Some(info) => res.push(PluginData {
                    name: info.name,
                    author: info.author,
                    description: info.description,
                    contact: info.contact,
                    plugin_type: plugin.plugin_type,
                    path: plugin.path,
                    visible: bol,
                }),
                None => res.push(PluginData {
                    name: None,
                    author: None,
                    description: None,
                    contact: None,
                    plugin_type: plugin.plugin_type,
                    path: plugin.path,
                    visible: bol,
                }),
            }
        }
        res
    }
    pub fn load_plugin<P: AsRef<OsStr>>(&mut self, path: P, bol: bool) -> Result<PluginInfo, ()> {
        unsafe {
            type PluginCreate = unsafe fn() -> *mut dyn Plugin;
            let lib = Library::new(path.as_ref()).expect("Unable to load the plugin");
            self.loaded_libraries.push(lib);
            let lib = self.loaded_libraries.last().unwrap();
            let constructor: Symbol<PluginCreate> = lib
                .get(b"_plugin_create")
                .expect("The `_plugin_create` symbol wasn't found.");
            let boxed_raw = constructor();
            let plugin = Box::from_raw(boxed_raw);
            println!("Loaded plugin: {}", plugin.name());
            let name = plugin.name();
            let author = plugin.author();
            let description = plugin.description();
            let contact = plugin.contact();
            if bol == true {
                plugin.load(&self.handle);
            }
            self.plugins.push(BoxPlugin {
                visible: bol,
                plugin: plugin,
            });
            Ok(PluginInfo {
                name: Some(name.to_owned()),
                author: Some(author.to_owned()),
                description: Some(description.to_owned()),
                contact: Some(contact.to_owned()),
            })
        }
    }

    // Iterate over the plugins, running their `send()` hook.
    pub fn send(&mut self, request: &mut Vec<Packet>) {
        for box_plugin in &mut self.plugins {
            if box_plugin.visible == true {
                println!("Firing send for {:?}", box_plugin.plugin.name());
                box_plugin.plugin.send(request, &self.handle);
            }
        }
    }

    // Unload all plugins and loaded plugin libraries, making sure to fire
    // their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub fn unload_all(&mut self) {
        for mut box_plugin in self.plugins.drain(..) {
            println!("Firing on_plugin_unload for {:?}", box_plugin.plugin.name());
            box_plugin.plugin.unload(&self.handle);
            box_plugin.visible = false;
        }
        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }
    pub fn unload(&mut self, name: &str) {
        for i in 0..self.plugins.len() {
            let plugin = &self.plugins[i].plugin;
            if plugin.name() == name {
                plugin.unload(&self.handle);
                self.plugins[i].visible = false;
                drop(&self.loaded_libraries[i]);
            }
        }
    }
    pub fn load(&mut self, name: &str) {
        for i in 0..self.plugins.len() {
            let plugin = &self.plugins[i].plugin;
            if plugin.name() == name {
                plugin.load(&self.handle);
                self.plugins[i].visible = true;
            }
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload_all();
        }
    }
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

use crate::plugin_dir;
use std::fs::create_dir_all;

#[derive(Serialize)]
enum PluginType {
    Dylib,
    Js,
}

struct PluginPath {
    pub plugin_type: PluginType,
    pub path: String,
}
// 读取插件目录下的插件
fn read_plugin_dir() -> Vec<PluginPath> {
    let plugin_dir = plugin_dir();
    create_dir_all(&plugin_dir).unwrap();
    let plugins = fs::read_all_path(&plugin_dir).unwrap();
    let mut res: Vec<PluginPath> = vec![];
    for path in plugins {
        let mut plugin_type: Option<PluginType> = None;
        if path.ends_with(".dylib") || path.ends_with(".so") || path.ends_with(".dll") {
            // 动态库
            plugin_type = Some(PluginType::Dylib);
        } else if path.ends_with(".js") {
            // js插件
            plugin_type = Some(PluginType::Js);
        }
        if plugin_type.is_some() {
            res.push(PluginPath {
                plugin_type: plugin_type.unwrap(),
                path: path,
            })
        }
    }
    res
}
