import { writable } from 'svelte/store'

function createPlugins() {
  const { set, update } = writable([])
  return {
    appendAll: data => {
      data.forEach(plugin => {
        const script = document.createElement('script')
        script.type = 'text/javascript'
        script.src = plugin.path
        document.body.appendChild(script)
      })
      set(data)
    },
    appendSuccess: plugin => {
      update(p => {
        p.push(plugin)
        const obj = p.find(item => item.path === plugin.path())
        if (obj) obj.name = plugin.name
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
  plugin.path = () => document.currentScript.getAttribute('src')
  plugins.appendSuccess(plugin)
}

export { plugins }

// Define the plugin
// const plugin = {
//   name: '测试JS插件',
//   load: ()=> { console.log('loaded 测试JS插件')}
//   unload:()=>{ console.log('unloaded 测试JS插件')}
// };

// Register the plugin
// globalThis.BililivePlugin(squaredPlugin)
