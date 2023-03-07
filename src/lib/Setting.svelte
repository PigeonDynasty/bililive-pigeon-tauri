<script lang="ts">
  import { onMount } from 'svelte'
  import Switch from '../components/Switch.svelte'
  import {
    WebviewWindow,
    currentMonitor,
    LogicalSize,
    LogicalPosition
  } from '@tauri-apps/api/window'
  import { TauriEvent } from '@tauri-apps/api/event'
  import ColorSelect from '../components/ColorSelect.svelte'
  import DatePicker from '../components/DatePicker/DatePicker.svelte'
  import rooms from '../store/room'
  import toast from '../utils/toast'
  import TimePicker from '@/components/TimePicker.svelte'
  import RoomSelect from './components/RoomSelect.svelte'
  import Slider from '@/components/Slider.svelte'

  let web: WebviewWindow = null
  let logicalSize: LogicalSize = new LogicalSize(0, 0)
  // TODO 配置持久化
  let sideConfig = {
    x: 0,
    y: 0,
    width: 300,
    height: 0
  }
  const toggleChange = async _e => {
    if (isSideOpen && !web) {
      if (!$rooms.some(item => item.room_id === Number(roomId))) {
        toast('输入的房间号未连接')
        isSideOpen = false
        return
      }
      web = new WebviewWindow('side-' + roomId, {
        url: '/side.html?id=' + roomId,
        acceptFirstMouse: false,
        alwaysOnTop: true,
        decorations: false,
        resizable: false,
        hiddenTitle: true,
        title: '',
        transparent: true,
        center: false,
        ...sideConfig
      })
      // FIXME 鼠标穿透无效
      web.setIgnoreCursorEvents(true)
      web.once(TauriEvent.WINDOW_DESTROYED, () => {
        web = null
        isSideOpen = false
      })
    } else if (!isSideOpen && web) {
      await web.close()
      web = null
    }
  }
  const setSidePosition = () => {
    if (web) web.setPosition(new LogicalPosition(sideConfig.x, sideConfig.y))
  }
  const setSideSize = () => {
    if (web) web.setSize(new LogicalSize(sideConfig.width, sideConfig.height))
  }
  let color = ''
  let roomId = ''
  let isSideOpen = false
  let date = ''
  let time = ''
  let datetime
  onMount(async () => {
    const monitor = await currentMonitor()
    logicalSize = monitor.size.toLogical(monitor.scaleFactor)
    sideConfig = {
      x: logicalSize.width - 300,
      y: 0,
      height: logicalSize.height / 2,
      width: 300
    }
  })
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
<h2 class="my-2">位置</h2>
<div class="flex items-center text-sm mb-2">
  <span class="w-12 mr-2">X</span>
  <Slider
    class="flex-1"
    bind:value={sideConfig.x}
    min={0}
    max={logicalSize.width}
    on:change={() => setSidePosition()}
  />
</div>
<div class="flex items-center text-sm mb-2">
  <span class="w-12 mr-2">Y</span>
  <Slider
    class="flex-1"
    bind:value={sideConfig.y}
    min={0}
    max={logicalSize.height}
    on:change={() => setSidePosition()}
  />
</div>
<h2 class="my-2">尺寸</h2>
<div class="flex items-center text-sm mb-2">
  <span class="w-12 mr-2">宽</span>
  <Slider
    class="flex-1"
    bind:value={sideConfig.width}
    min={0}
    max={logicalSize.width}
    on:change={() => setSideSize()}
  />
</div>
<div class="flex items-center text-sm mb-2">
  <span class="w-12 mr-2">高</span>
  <Slider
    class="flex-1"
    bind:value={sideConfig.height}
    min={0}
    max={logicalSize.height}
    on:change={() => setSideSize()}
  />
</div>

<!-- TODO 文本颜色 -->
<!-- <h2 class="my-2">颜色</h2>
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
</div>-->
<!-- <h2 class="my-2">测试</h2>
<DatePicker bind:value={date} />
<TimePicker bind:value={time} />
<DatePicker bind:value={datetime} time /> -->
