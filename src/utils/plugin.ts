import { writable } from 'svelte/store'
import { convertFileSrc } from '@tauri-apps/api/tauri'
function createPlugins() {
  const plugin_list = writable([])
  const { set, update } = plugin_list
  return {
    data: plugin_list,
    appendAll: data => {
      data.forEach(plugin => {
        plugin.asset_path = convertFileSrc(plugin.path) + '?_t=' + Date.now()
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
    unload: plugin => plugin.unload(),
    clear: () => {
      update(p => {
        p.forEach(item => {
          item.unload && item.unload()
          const script = document.querySelector(
            `script[src="${item.asset_path}"]`
          )
          script.remove()
        })
        return []
      })
    }
  }
}
const plugins = createPlugins()

export default plugins

globalThis.RegisterBililivePlugin = plugin => {
  plugin.getPath = () => document.currentScript.getAttribute('src')
  plugins.appendSuccess(plugin)
}
globalThis.BililivePlugin = class BililivePlugin {
  name: string
  author: string
  description: string
  contact: string
  load: Function
  unload: Function
  getPath: Function
  constructor(config) {
    console.log(config)
    Object.entries(config).forEach(([key, value]) => {
      this[key] = value
    })
    this.getPath = () => document.currentScript.getAttribute('src')
    plugins.appendSuccess(this)
  }
}

// example plugin 1
// new BililivePlugin({
//   name: '测试JS插件',
//   author: '测试作者',
//   description: '说明',
//   contact: '联系方式',
//   test: () => {
//     console.log('test this', this)
//   },
//   load: function () {
//     console.log('loaded 测试JS插件')
//     this.test()
//   },
//   unload: () => { console.log('unloaded 测试JS插件') }
// })

// example plugin 2
// window.RegisterBililivePlugin({
//   name: '测试JS插件',
//   author: '测试作者',
//   description: '说明',
//   contact: '联系方式',
//   test: () => {
//     console.log('test this', this)
//   },
//   load: function () {
//     console.log('loaded 测试JS插件')
//     this.test()
//   },
//   unload: () => { console.log('unloaded 测试JS插件') }
// })
