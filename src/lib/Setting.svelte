<script lang="ts">
  import Switch from '../components/Switch.svelte'
  import { WebviewWindow, currentMonitor } from '@tauri-apps/api/window'
  import { TauriEvent } from '@tauri-apps/api/event'
  import ColorSelect from '../components/ColorSelect.svelte'
  import DatePicker from '../components/DatePicker/DatePicker.svelte'
  import rooms from '../store/room'
  import toast from '../utils/toast'
  import TimePicker from '@/components/TimePicker.svelte'
  let web: WebviewWindow = null
  const toggleChange = async _e => {
    if (isSideOpen && !web) {
      if (!$rooms.some(item => item.room_id === Number(roomId))) {
        toast('输入的房间号未连接')
        isSideOpen = false
        return
      }
      currentMonitor().then(monitor => {
        web = new WebviewWindow('side-' + roomId, {
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
        web.once(TauriEvent.WINDOW_DESTROYED, () => {
          web = null
          isSideOpen = false
        })
      })
    } else if (!isSideOpen && web) {
      await web.close()
      web = null
    }
  }
  let color = ''
  let roomId = ''
  let isSideOpen = false
  let date = ''
  let time = ''
</script>

<div>
  <h5>浮窗设置</h5>
  <div class="flex items-center">
    <span>房间号</span>
    <input
      class="input mx-2"
      placeholder="输入已连接的房间号"
      bind:value={roomId}
    />
    <Switch id="toggle" bind:value={isSideOpen} on:change={toggleChange} />
  </div>
</div>
<ColorSelect bind:value={color} />
<DatePicker bind:value={date} />
<TimePicker bind:value={time} />
