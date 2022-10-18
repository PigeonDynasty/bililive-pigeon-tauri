<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import { appWindow } from '@tauri-apps/api/window'
  export let roomId: string | number
  let listener = null
  const data = new Array(100) // 弹幕数据

  let ulEl // ul dom对象
  let couldScroll: boolean = false // 判断能否自动滚动到底部
  const checkCouldScroll = () => {
    couldScroll = ulEl.clientHeight + ulEl.scrollTop >= ulEl.scrollHeight
  }
  const toBottom = () => {
    ulEl.scrollTop = ulEl.scrollHeight
  }
  onMount(() => {
    if (!roomId) return
    // invoke('connect', { roomId })
    // if (listener) {
    //   // 防止重复监听
    //   listener['stream']()
    //   listener['danmaku']()
    // }
    // listener['stream'] = appWindow.listen('stream-' + roomId, ev => {
    //   console.log(ev)
    //   couldScroll && (ulEl.scrollTop = ulEl.scrollHeight)
    // })
    // listener['danmaku'] = appWindow.listen('danmaku-' + roomId, ev => {
    //   console.log(ev)
    // })
  })
  onDestroy(() => {
    // invoke('disconnect', { roomId })
    // // 断开连接 解除监听
    // if (listener) {
    //   listener['stream']()
    //   listener['danmaku']()
    //   listener = null
    // }
  })
</script>

<div
  class="border-sky-400 rounded border-[1px] border-solid relative h-full dark:border-sky-800"
>
  <ul
    class="overflow-y-auto h-full py-1 px-2"
    bind:this={ulEl}
    on:scroll={checkCouldScroll}
  >
    {#each data as d, i}
      <li>
        {i + 1}
      </li>
    {/each}
  </ul>

  <button
    class="btn-primary text-xs leading-3 rounded-2xl py-1 px-2 scale-90 absolute right-1 bottom-1"
    hidden={couldScroll}
    on:click={toBottom}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="2.5"
      stroke="currentColor"
      class="w-3 h-3 float-left mr-1"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M12 4.5v15m0 0l6.75-6.75M12 19.5l-6.75-6.75"
      />
    </svg>
    回到底部
  </button>
</div>
