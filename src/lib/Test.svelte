<script lang="ts">
  // import { WebviewWindow } from '@tauri-apps/api/window'
  import { invoke } from '@tauri-apps/api/tauri'
  import { appWindow } from '@tauri-apps/api/window'
  const listeners = {}
  function connect(roomId: number) {
    invoke('connect', { roomId })
    if (listeners[roomId]) {
      // 防止重复监听
      listeners[roomId]['stream']()
      listeners[roomId]['danmaku']()
    }
    listeners[roomId]['stream'] = appWindow.listen('stream-' + roomId, ev => {
      console.log(ev)
    })
    listeners[roomId]['danmaku'] = appWindow.listen('danmaku-' + roomId, ev => {
      console.log(ev)
    })
  }
  function disconnect(roomId: number) {
    invoke('disconnect', { roomId })
    // 断开连接 解除监听
    if (listeners[roomId]) {
      listeners[roomId]['stream']()
      listeners[roomId]['danmaku']()
    }
  }
  // 24042761 BLG CRISP
  // 697 守护茶茶
  // 22259479 烤鱼
  // 5050 老E
  // 213 C酱
  function loadPlugin() {
    invoke('load_plugin')
  }
  function unloadPlugin() {
    invoke('unload_plugin')
  }
</script>

<div class="row">
  <button on:click={loadPlugin}>加载插件</button>
  <button on:click={unloadPlugin}>卸载插件</button>
</div>
<div class="row">
  <button on:click={() => connect(15536)}>本气黑猫 connect</button>
  <button on:click={() => disconnect(15536)}>本气黑猫 disconnect</button>
  <button on:click={() => connect(5050)}>老E connect</button>
  <button on:click={() => disconnect(5050)}>老E disconnect</button>
  <button on:click={() => connect(213)}>C酱 connect</button>
  <button on:click={() => disconnect(213)}>C酱 disconnect</button>
</div>
