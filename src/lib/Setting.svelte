<script lang="ts">
  import Switch from '../components/Switch.svelte'
  import { WebviewWindow, currentMonitor } from '@tauri-apps/api/window'
  import { TauriEvent } from '@tauri-apps/api/event'
  import ColorSelect from '../components/ColorSelect.svelte'
  import DatePicker from '../components/DatePicker/DatePicker.svelte'
  import rooms from '../store/room'
  import toast from '../utils/toast'
  import TimePicker from '@/components/TimePicker.svelte'
  import RoomSelect from './components/RoomSelect.svelte'
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
  let datetime
</script>

<h1 class="py-2 text-lg">浮窗设置</h1>
<div class="flex items-center text-sm mb-2">
  <span class="w-12 mr-2">房间号</span>
  <RoomSelect class="flex-1 mr-2" bind:value={roomId} />
  <Switch
    id="sideWin"
    bind:value={isSideOpen}
    disabled={!roomId}
    on:change={toggleChange}
  />
</div>
<h2 class="my-2">颜色</h2>
<div class="flex text-sm mb-2">
  <div class="flex-1 flex items-center">
    <span class="w-12 mr-2">SC标记</span>
    <ColorSelect bind:value={color} />
  </div>
  <div class="flex-1 flex items-center">
    <span class="w-12 mr-2">SC标记</span>
    <ColorSelect bind:value={color} />
  </div>
</div>
<div class="flex text-sm mb-2">
  <div class="flex-1 flex items-center">
    <span class="w-12 mr-2">SC标记</span>
    <ColorSelect bind:value={color} />
  </div>
  <div class="flex-1 flex items-center">
    <span class="w-12 mr-2">SC标记</span>
    <ColorSelect bind:value={color} />
  </div>
</div>
<h2 class="my-2">位置</h2>
<div class="flex text-sm mb-2">
  <div class="flex-1 flex items-center">
    <span class="w-12 mr-2">X</span>
  </div>
  <div class="flex-1 flex items-center">
    <span class="w-12 mr-2">Y</span>
  </div>
</div>

<DatePicker bind:value={date} />
<TimePicker bind:value={time} />
<DatePicker bind:value={datetime} time />
