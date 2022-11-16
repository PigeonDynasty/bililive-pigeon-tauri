import { writable } from 'svelte/store'
import { convertFileSrc } from '@tauri-apps/api/tauri'
function createPlugins() {
  const plugin_list = writable([])
  const { set, update } = plugin_list
  return {
    data: plugin_list,
    appendAll: data => {
      data.forEach(plugin => {
        plugin.asset_path = convertFileSrc(plugin.path)
        const script = document.createElement('script')
        script.type = 'text/javascript'
        script.src = plugin.asset_path
        document.body.appendChild(script)
      })
      set(data)
    },
    appendSuccess: plugin => {
      update(p => {
        const obj = p.find(item => item.asset_path === plugin.getPath())
        if (obj) {
          Object.assign(obj, plugin)
          obj.visible && obj.load()
        }
        return p
      })
    },
    splice: (i: number) => {
      update(p => {
        p.splice(i, 1)
        return p
      })
    },
    load: plugin => plugin.load(),
    unload: plugin => plugin.unload()
  }
}
const plugins = createPlugins()

globalThis.BililivePlugin = plugin => {
  plugin.getPath = () => document.currentScript.getAttribute('src')
  plugins.appendSuccess(plugin)
}

export default plugins

// Define the plugin
// const plugin = {
//   name: '测试JS插件',
//   description: ''
//   contact:'',
//   load: ()=> { console.log('loaded 测试JS插件')}
//   unload:()=>{ console.log('unloaded 测试JS插件')}
// };

// Register the plugin
// globalThis.BililivePlugin(plugin)
