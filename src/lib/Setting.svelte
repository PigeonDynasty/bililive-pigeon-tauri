<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import Switch from '../components/Switch.svelte'
  import {
    WebviewWindow,
    currentMonitor,
    LogicalSize,
    LogicalPosition
  } from '@tauri-apps/api/window'
  import { TauriEvent } from '@tauri-apps/api/event'
  import rooms from '../store/room'
  import toast from '../utils/toast'
  import RoomSelect from './components/RoomSelect.svelte'
  import Slider from '@/components/Slider.svelte'
  import ColorPicker from '@/components/ColorPicker/ColorPicker.svelte'

  let web: WebviewWindow = null
  let logicalSize: LogicalSize = new LogicalSize(0, 0)
  let sideConfig = {
    x: 0,
    y: 0,
    width: 300,
    height: 0
  }
  const defaultSideColor = {
    msg: '#facc15',
    bg: 'rgba(102, 102, 102, 0.2)',
    roomid: '',
    time: '',
    username: '',
    giftGold: '#d97706',
    giftSilver: '#78350f',
    guard: '#7c3aed',
    sc: '#e11d48'
  }
  let sideColor = { ...defaultSideColor }
  let roomId = ''
  let isSideOpen = false
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
        focus: false,
        ...sideConfig
      })
      web.once(TauriEvent.WINDOW_DESTROYED, () => {
        web = null
        isSideOpen = false
      })
      web.once('side-loaded', () => {
        web.emit('update-color', sideColor)
      })
    } else if (!isSideOpen && web) {
      await web.close()
      web = null
    }
  }
  const setSidePosition = () => {
    if (web) web.setPosition(new LogicalPosition(sideConfig.x, sideConfig.y))
    updateConfig()
  }
  const setSideSize = () => {
    if (web) web.setSize(new LogicalSize(sideConfig.width, sideConfig.height))
    updateConfig()
  }
  const colorChange = _ => {
    if (web) web.emit('update-color', sideColor)
    updateConfig()
  }
  const colorClear = key => {
    sideColor[key] = defaultSideColor[key]
    colorChange(key)
  }
  const updateConfig = () => {
    invoke('update_setting', {
      roomId: 0,
      config: JSON.stringify({
        sideConfig,
        sideColor
      })
    }) // 0 保存默认配置
  }
  onMount(async () => {
    const monitor = await currentMonitor()
    logicalSize = monitor.size.toLogical(monitor.scaleFactor)
    sideConfig = {
      x: logicalSize.width - 300,
      y: 0,
      height: logicalSize.height / 2,
      width: 300
    }
    // config
    const res: DbSetting = await invoke('get_setting', { roomId: 0 }) // 0 默认配置
    if (!res || !res.config) return
    const config = JSON.parse(res.config)
    sideConfig = config.sideConfig
    sideColor = config.sideColor
  })
</script>

<h1 class="py-2 text-lg">浮窗设置</h1>
<div class="flex items-center text-sm mb-2">
  <span class="w-20 mr-2">房间号</span>
  <RoomSelect class="flex-1 mr-2" bind:value={roomId} disabled={isSideOpen} />
  <Switch
    id="sideWin"
    bind:value={isSideOpen}
    disabled={!roomId}
    on:change={toggleChange}
  />
</div>
<h2 class="my-2">位置</h2>
<div class="flex items-center text-sm mb-2">
  <span class="w-20 mr-2">X</span>
  <Slider
    class="flex-1 -ml-3"
    bind:value={sideConfig.x}
    min={0}
    max={logicalSize.width}
    on:change={() => setSidePosition()}
  />
</div>
<div class="flex items-center text-sm mb-2">
  <span class="w-20 mr-2">Y</span>
  <Slider
    class="flex-1 -ml-3"
    bind:value={sideConfig.y}
    min={0}
    max={logicalSize.height}
    on:change={() => setSidePosition()}
  />
</div>
<h2 class="my-2">尺寸</h2>
<div class="flex items-center text-sm mb-2">
  <span class="w-20 mr-2">宽</span>
  <Slider
    class="flex-1 -ml-3"
    bind:value={sideConfig.width}
    min={0}
    max={logicalSize.width}
    on:change={() => setSideSize()}
  />
</div>
<div class="flex items-center text-sm mb-2">
  <span class="w-20 mr-2">高</span>
  <Slider
    class="flex-1 -ml-3"
    bind:value={sideConfig.height}
    min={0}
    max={logicalSize.height}
    on:change={() => setSideSize()}
  />
</div>

<h2 class="my-2">颜色</h2>
<div class="flex text-sm mb-2">
  <div class="flex-1 flex items-center">
    <span class="w-20 mr-2">消息</span>
    <ColorPicker
      bind:value={sideColor.msg}
      on:change={_ => colorChange('msg')}
      on:clear={_ => colorClear('msg')}
    />
  </div>
  <div class="flex-1 flex items-center">
    <span class="w-20 mr-2">背景</span>
    <ColorPicker
      bind:value={sideColor.bg}
      alpha
      on:change={_ => colorChange('bg')}
      on:clear={_ => colorClear('bg')}
    />
  </div>
</div>
<div class="flex text-sm mb-2">
  <div class="flex-1 flex items-center">
    <span class="w-20 mr-2">时间</span>
    <ColorPicker
      bind:value={sideColor.time}
      on:change={_ => colorChange('time')}
      on:clear={_ => colorClear('time')}
    />
  </div>
  <div class="flex-1 flex items-center">
    <span class="w-20 mr-2">用户名</span>
    <ColorPicker
      bind:value={sideColor.username}
      on:change={_ => colorChange('username')}
      on:clear={_ => colorClear('username')}
    />
  </div>
</div>
<div class="flex text-sm mb-2">
  <div class="flex-1 flex items-center">
    <span class="w-20 mr-2">金瓜子标记</span>
    <ColorPicker
      bind:value={sideColor.giftGold}
      on:change={_ => colorChange('giftGold')}
      on:clear={_ => colorClear('giftGold')}
    />
  </div>
  <div class="flex-1 flex items-center">
    <span class="w-20 mr-2">银瓜子标记</span>
    <ColorPicker
      bind:value={sideColor.giftSilver}
      on:change={_ => colorChange('giftSilver')}
      on:clear={_ => colorClear('giftSilver')}
    />
  </div>
</div>
<div class="flex text-sm mb-2">
  <div class="flex-1 flex items-center">
    <span class="w-20 mr-2">上舰标记</span>
    <ColorPicker
      bind:value={sideColor.guard}
      on:change={_ => colorChange('guard')}
      on:clear={_ => colorClear('guard')}
    />
  </div>
  <div class="flex-1 flex items-center">
    <span class="w-20 mr-2">SC标记</span>
    <ColorPicker
      bind:value={sideColor.sc}
      on:change={_ => colorChange('sc')}
      on:clear={_ => colorClear('sc')}
    />
  </div>
</div>
