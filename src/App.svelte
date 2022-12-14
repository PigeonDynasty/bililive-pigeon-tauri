<script lang="ts">
  import Greet from './lib/Greet.svelte'
  import Danmaku from './lib/Danmaku.svelte'
  import { Tabs, Tab } from './components/Tabs'
  import Plugin from './lib/Plugin.svelte'
  import Setting from './lib/Setting.svelte'
  import { appWindow } from '@tauri-apps/api/window'
  import { TauriEvent } from '@tauri-apps/api/event'
  import roomIds, { addRoomId, delRoomId } from './utils/roomId'

  let rooms = []
  let roomId = ''
  let tabsRef
  let roomRefs = {}

  roomIds.subscribe(value => {
    rooms = value
  })

  const connect = () => {
    addRoomId(roomId)
    tabsRef.selectTab('room_' + roomId)
  }

  const tabClose = e => {
    const { key } = e.detail
    const id = key.replace('room_', '')
    delRoomId(id)
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
  <div class="flex-1 px-1">
    <Tabs class="h-full">
      <Tab header="Info" key="info">
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
      <Tab header="用户" key="user">用户1</Tab>
      <Tab header="插件" key="plugin">
        <Plugin />
      </Tab>
      <Tab header="设置" key="setting">
        <Setting />
      </Tab>
    </Tabs>
  </div>
  <div class="flex flex-1 px-1 flex-col">
    <div class="flex mb-1">
      <input
        class="rounded-md border-0 py-1 px-2 shadow-md flex-1 dark:bg-black bg-white"
        placeholder="输入房间号"
        bind:value={roomId}
      />
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
      {#each rooms as roomid ('room_' + roomid)}
        <Tab header={roomid} key={'room_' + roomid}>
          <Danmaku bind:this={roomRefs[roomid]} roomId={Number(roomid)} />
        </Tab>
      {/each}
    </Tabs>
  </div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    /* color: #0f0f0f;
    background-color: #f6f6f6; */

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .row {
    display: flex;
    justify-content: center;
  }

  a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
  }

  a:hover {
    color: #535bf2;
  }

  h1 {
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    a:hover {
      color: #24c8db;
    }
  }

  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
