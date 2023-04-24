<script lang="ts">
  import { onMount } from 'svelte'
  import { Tabs, Tab } from './components/Tabs'
  import Danmaku from './lib/Danmaku.svelte'
  import Gift from './lib/Gift.svelte'
  import Plugin from './lib/Plugin.svelte'
  import Setting from './lib/Setting.svelte'
  // import Test from './lib/Test.svelte'
  import RoomInput from './lib/components/RoomInput.svelte'
  import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
  import { TauriEvent } from '@tauri-apps/api/event'
  import rooms, { addRoomId, delByRoomId } from './store/room'
  import { invoke } from '@tauri-apps/api/tauri'
  let roomId = ''
  $: disabled = !roomId || isNaN(Number(roomId))
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
  appWindow.once(TauriEvent.WINDOW_CLOSE_REQUESTED, () => {
    Object.entries(roomRefs).forEach(([key, value]) => {
      if (!value) return
      let danmaku = value as Danmaku
      danmaku.writeDanmaku()
      const sideWindow = WebviewWindow.getByLabel('side-' + key)
      sideWindow && sideWindow.close()
    })
    // 关闭
    appWindow.close()
  })
  onMount(() => {
    invoke('clear_danmaku_pool')
  })
</script>

<main class="flex h-full p-1">
  <!-- 隐藏的div 用以打包动态加载的tailwind css -->
  <div class="hidden" />
  <div class="w-1/2 px-1">
    <Tabs class="h-full">
      <Tab header="礼物统计" key="gift">
        <Gift />
      </Tab>
      <Tab header="插件" key="plugin">
        <Plugin />
      </Tab>
      <Tab header="设置" key="setting">
        <Setting />
      </Tab>
      <!-- <Tab header="测试" key="test">
        <Test />
      </Tab> -->
    </Tabs>
  </div>
  <div class="w-1/2 px-1 flex flex-col">
    <div class="flex mb-1">
      <RoomInput class="flex-1" bind:value={roomId} />
      <button
        class="btn-default rounded-md py-1 px-3 ml-2"
        {disabled}
        on:click={connect}
      >
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
