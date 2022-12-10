<script lang="ts">
  import Toggle from './Toggle.svelte'
  import { WebviewWindow, currentMonitor } from '@tauri-apps/api/window'
  import ColorPicker from './ColorPicker.svelte'
  import roomIds from '../utils/roomId'
  import toast from '../utils/toast'
  let web: WebviewWindow = null
  const toggleChange = async _e => {
    if (isSideOpen && !web) {
      if (!$roomIds.includes(Number(roomId))) {
        toast('输入的房间号未连接')
        isSideOpen = false
        return
      }
      currentMonitor().then(monitor => {
        web = new WebviewWindow('sidewin-' + roomId, {
          url: '/side.html?id=' + roomId,
          // alwaysOnTop: true,
          // decorations: false,
          // resizable: false,
          title: '',
          // transparent: true,
          // x: monitor.size.width ,
          x: 0,
          y: monitor.size.height,
          height: monitor.size.height,
          width: 300
        })
        // web.once('tauri://created', () => {
        //   setTimeout(() => {
        //     web && web.emit('add-'+roomId, '<p>test</p>')
        //   }, 5000)
        // })
      })
    } else if (!isSideOpen && web) {
      await web.close()
      web = null
    }
  }
  let color = ''
  let roomId = ''
  let isSideOpen = false
</script>

<div>
  <h5>浮窗设置</h5>
  <div class="flex items-center">
    <span>房间号</span>
    <input
      class="rounded-md py-1 px-2 mx-2 shadow-md flex-1 dark:bg-black bg-white"
      placeholder="输入已连接的房间号"
      bind:value={roomId}
    />
    <Toggle id="toggle" bind:value={isSideOpen} on:change={toggleChange} />
  </div>
</div>
<ColorPicker bind:value={color} />
