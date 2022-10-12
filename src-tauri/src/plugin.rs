#![allow(unused)]
use crate::fs;
use crate::packet::Packet;
use libloading::{Library, Symbol};
use std::any::Any;
use std::ffi::OsStr;
use tauri::AppHandle;
// A plugin which allows you to add extra functionality to the REST client.
pub trait Plugin: Any + Send + Sync {
    // Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    // A callback fired immediately after the plugin is loaded. Usually used
    // for initialization.
    fn on_plugin_load(&self, handle: &Option<AppHandle>) {}
    // A callback fired immediately before the plugin is unloaded. Use this if
    // you need to do any cleanup.
    fn on_plugin_unload(&self, handle: &Option<AppHandle>) {}
    // Inspect (and possibly mutate) the request before it is sent.
    fn send(&self, _request: &mut Vec<Packet>, handle: &Option<AppHandle>) {}
}
pub struct PluginManager {
    handle: Option<AppHandle>,
    plugins: Vec<Box<dyn Plugin>>,
    loaded_libraries: Vec<Library>,
}

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
    pub fn load_plugin_all(&mut self) {
        let plugins = fs::read_plugin();
        let config = fs::Config::new().read();
        for plugin in plugins {
            if plugin.plugin_type == "dylib" && Self::check_visible(&plugin, &config) {
                self.load_plugin(plugin.path);
            }
        }
    }
    fn check_visible(plugin: &fs::PluginPath, config: &serde_json::Value) -> bool {
        let mut bol = false;
        let arr = config["plugins"].as_array().unwrap();
        for plugin_config in arr {
            if plugin_config["path"] == plugin.path && plugin_config["visible"] == true {
                bol = true;
                break;
            }
        }
        bol
    }
    pub fn load_plugin<P: AsRef<OsStr>>(&mut self, path: P) -> Result<(), ()> {
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
            plugin.on_plugin_load(&self.handle);
            self.plugins.push(plugin);
            Ok(())
        }
    }

    // Iterate over the plugins, running their `send()` hook.
    pub fn send(&mut self, request: &mut Vec<Packet>) {
        println!("Firing pre_send hooks");
        for plugin in &mut self.plugins {
            println!("Firing send for {:?}", plugin.name());
            plugin.send(request, &self.handle);
        }
    }

    // Unload all plugins and loaded plugin libraries, making sure to fire
    // their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub fn unload_all(&mut self) {
        println!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            println!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload(&self.handle);
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }
    pub fn unload(&mut self, name: &str) {
        for i in 0..self.plugins.len() {
            if self.plugins[i].name() == name {
                self.plugins[i].on_plugin_unload(&self.handle);
                drop(&self.loaded_libraries[i]);
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
