<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import { appWindow } from '@tauri-apps/api/window'
  import { dateFormat, html2text } from '../utils/utils'
  import { fade } from 'svelte/transition'
  let roomId: string | number
  let listener = null
  let count = 0
  let msg = [] // 弹幕数据
  let ulEl // ul dom对象
  let couldScroll: boolean = true // 判断能否自动滚动到底部
  const checkCouldScroll = () => {
    couldScroll = ulEl
      ? ulEl.clientHeight + ulEl.scrollTop >= ulEl.scrollHeight
      : false
  }
  const toBottom = () => {
    setTimeout(() => {
      ulEl && (ulEl.scrollTop = ulEl.scrollHeight)
    }, 300)
  }

  let txt_index = 0 // 记录保存数据第N条
  let interval = null
  const write_danmaku = () => {
    const end = msg.length
    if (end === txt_index) return
    // console.log('write:', txt_index, end, msg.slice(txt_index, end))
    invoke('write_danmaku_txt', {
      roomId,
      date: dateFormat(new Date(), 'yyyy-MM-dd'),
      data: msg.slice(txt_index, end).map(str => html2text(str))
    })
    txt_index = end
  }
  onMount(async () => {
    if (!roomId) return
    msg = ['开始连接...']
    invoke('connect', { roomId })
    if (listener) {
      // 防止重复监听
      listener['stream']()
      listener['danmaku']()
    } else {
      listener = {}
    }
    listener['stream'] = await appWindow.listen('stream-' + roomId, ev => {
      switch (ev.payload) {
        case 'connected':
          msg = [...msg, '连接弹幕服务器成功']
          break
        case 'joined':
          msg = [...msg, `连接房间成功`]
          break
        case 'closed':
          msg = [...msg, `意外关闭，请关闭页签重新连接`]
          break
        case 'disconnect':
          msg = [...msg, `已断开连接`]
          break
        default:
          msg = [...msg, `真实房间号：${ev.payload}`]
          break
      }
      couldScroll && toBottom()
    })
    listener['danmaku'] = await appWindow.listen(
      'danmaku-' + roomId,
      (ev: any) => {
        ev.payload.forEach(payload => {
          console.log('op:', payload.op)
          switch (payload.op) {
            case 3: //气人值
              // count = payload.body.count
              break
            case 5: // wss消息
              // console.log(payload.body)
              switch (payload.body.cmd) {
                case 'DANMU_MSG': // 用户发送的消息
                  const info = payload.body.info
                  msg = [
                    ...msg,
                    `[${dateFormat(info[0][4], 'hh:mm:ss')}] ${info[2][1]}：${
                      info[1]
                    }`
                  ]
                  break
                case 'WATCHED_CHANGE': // 多少人看过
                  const data = payload.body.data
                  count = data.num
                  break
                case 'SEND_GIFT': // 礼物
                  const gift = payload.body.data
                  msg = [
                    ...msg,
                    `<i class="${
                      gift['coin_type'] === 'gold'
                        ? 'text-amber-600'
                        : 'text-amber-900'
                    }">【礼物】</i>[${dateFormat(
                      gift['timestamp'] * 1000,
                      'hh:mm:ss'
                    )}] ${gift['uname']} ${gift['action']} ${gift['num']} 个 ${
                      gift['giftName']
                    }`
                  ]
                  break
                case 'SUPER_CHAT_MESSAGE': //sc
                case 'SUPER_CHAT_MESSAGE_JP':
                  const sc = payload.body.data
                  msg = [
                    ...msg,
                    `<i class="text-rose-600">【SC：${
                      sc['price']
                    }】</i>[${dateFormat(sc['ts'] * 1000, 'hh:mm:ss')}] ${
                      sc['user_info']['uname']
                    }： <span style="color:${sc['message_font_color']};">${
                      sc['message']
                    }`
                  ]
                  break
                default: // 其他数据
                  if (payload.body.cmd.includes('DANMU_MSG')) {
                    // 遇上了奇怪的 DANMU_MSG 似乎是活动的类型
                    const info = payload.body.info
                    msg = [
                      ...msg,
                      `[${dateFormat(info[0][4], 'hh:mm:ss')}] ${info[2][1]}：${
                        info[1]
                      }`
                    ]
                  }
              }
              break
          }
        })
        if (!couldScroll) console.log(roomId)
        couldScroll && toBottom()
      }
    )
    // 定时写入文件 30s
    interval = setInterval(() => {
      write_danmaku()
    }, 30 * 1000)
  })
  onDestroy(() => {
    interval && clearInterval(interval)
    write_danmaku()
    invoke('disconnect', { roomId })
    // 断开连接 解除监听
    if (listener) {
      listener['stream']()
      listener['danmaku']()
      listener = null
    }
  })
  export { roomId, write_danmaku }
</script>

<div
  class="rounded border border-solid relative h-full bg-white dark:bg-black shadow-md"
>
  <ul
    class="overflow-y-auto h-full py-1 px-2"
    bind:this={ulEl}
    on:scroll={checkCouldScroll}
  >
    {#each msg as d, i}
      <li in:fade>
        {@html d}
      </li>
    {/each}
  </ul>
  <span
    class="absolute top-2 right-6 px-2 flex items-center shadow-md rounded-lg bg-gray-100 dark:bg-gray-800 text-sm"
    title="看过"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="w-4 h-4 mr-2"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z"
      />
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
      />
    </svg>
    {count}
  </span>
  {#if !couldScroll}
    <button
      class="btn-primary text-xs leading-3 rounded-full py-1 px-2 scale-90 absolute right-1 bottom-1"
      transition:fade
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
  {/if}
</div>
