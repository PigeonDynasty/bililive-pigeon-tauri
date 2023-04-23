import { writable } from 'svelte/store'
import { convertFileSrc } from '@tauri-apps/api/tauri'

const plugins = writable([])
const { set, update } = plugins
const pluginAppendAll = data => {
  data.forEach(plugin => {
    plugin.asset_path = convertFileSrc(plugin.path) + '?_t=' + Date.now()
    const script = document.createElement('script')
    script.type = 'text/javascript'
    script.src = plugin.asset_path
    document.body.appendChild(script)
  })
  set(data)
}
const _pluginAppendSuccess = plugin => {
  update(p => {
    const obj = p.find(item => item.asset_path === plugin.getPath())
    if (obj) {
      Object.assign(obj, plugin)
      obj.visible && obj.load()
    }
    return p
  })
}
const pluginSplice = (i: number) => {
  update(p => {
    p.splice(i, 1)
    return p
  })
}
const pluginLoad = plugin => plugin.load()
const pluginUnload = plugin => plugin.unload()
const pluginClear = () => {
  update(p => {
    p.forEach(item => {
      item.unload && item.unload()
      const script = document.querySelector(`script[src="${item.asset_path}"]`)
      script.remove()
    })
    return []
  })
}

export default plugins
export { pluginAppendAll, pluginSplice, pluginLoad, pluginUnload, pluginClear }

globalThis.RegisterBililivePlugin = plugin => {
  plugin.getPath = () => document.currentScript.getAttribute('src')
  _pluginAppendSuccess(plugin)
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
    Object.entries(config).forEach(([key, value]) => {
      this[key] = value
    })
    this.getPath = () => document.currentScript.getAttribute('src')
    _pluginAppendSuccess(this)
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
//   unload: () => {
//     console.log('unloaded 测试JS插件')
//   },
//   onStream: p => {
//     console.log('onStream', p)
//   },
//   onDanmaku: p => {
//     console.log('onDanmaku', p)
//   }
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
//   unload: () => {
//     console.log('unloaded 测试JS插件')
//   },
//   onStream: p => {
//     console.log('onStream', p)
//   },
//   onDanmaku: p => {
//     console.log('onDanmaku', p)
//   }
// })
