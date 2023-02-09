<script lang="ts">
  import { Tabs, Tab } from './components/Tabs'
  // import Greet from './lib/Greet.svelte'
  import Danmaku from './lib/Danmaku.svelte'
  import Gift from './lib/Gift.svelte'
  import Plugin from './lib/Plugin.svelte'
  import Setting from './lib/Setting.svelte'
  import { appWindow } from '@tauri-apps/api/window'
  import { TauriEvent } from '@tauri-apps/api/event'
  import rooms, { addRoomId, delByRoomId } from './store/room'

  let roomId = ''
  let tabsRef
  let roomRefs = {}

  $: tabHeader = room => room.room_id + (room.uname ? `-${room.uname}` : '')
  const connect = () => {
    addRoomId(roomId)
    tabsRef.selectTab('room_' + roomId)
  }

  const tabClose = e => {
    const { key } = e.detail
    const id = key.replace('room_', '')
    delByRoomId(id)
    delete roomRefs[id]
  }
  // 自定义关闭事件
  appWindow.listen(TauriEvent.WINDOW_CLOSE_REQUESTED, () => {
    Object.entries(roomRefs).forEach(([_key, value]) => {
      value && (value as Danmaku).write_danmaku()
    })
    // 关闭
    appWindow.close()
  })
</script>

<main class="flex h-full py-2 px-1">
  <!-- 隐藏的div用来防止tailwindcss动态赋css时候不显示相关样式 -->
  <div class="hidden text-amber-600 text-amber-900 text-violet-600" />
  <div class="w-1/2 px-1">
    <Tabs class="h-full">
      <!-- <Tab header="Info" key="info">
        <h1>Welcome to Tauri!</h1>
        <div class="row">
          <a href="https://vitejs.dev" target="_blank">
            <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
          </a>
          <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
          </a>
          <a href="https://svelte.dev" target="_blank">
            <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
          </a>
        </div>

        <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>
      </Tab>
      <Tab header="Greet" key="greet">
        <Greet />
      </Tab>
      <Tab header="用户" key="user">用户1</Tab> -->
      <Tab header="礼物统计" key="gift">
        <Gift />
      </Tab>
      <Tab header="插件" key="plugin">
        <Plugin />
      </Tab>
      <Tab header="设置" key="setting">
        <Setting />
      </Tab>
    </Tabs>
  </div>
  <div class="w-1/2 px-1 flex flex-col">
    <div class="flex mb-1">
      <input class="input" placeholder="输入房间号" bind:value={roomId} />
      <button class="btn-default rounded-md py-1 px-3 ml-2" on:click={connect}>
        连接
      </button>
    </div>
    <Tabs
      class="flex-1 overflow-hidden"
      closeable
      bind:this={tabsRef}
      on:close={tabClose}
    >
      {#each $rooms as room ('room_' + room.room_id)}
        <Tab header={tabHeader(room)} key={'room_' + room.room_id}>
          <Danmaku
            bind:this={roomRefs[room.room_id]}
            roomId={Number(room.room_id)}
          />
        </Tab>
      {/each}
    </Tabs>
  </div>
</main>
