<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
  import { dateFormat, html2text } from '../utils/utils'
  import { fade } from 'svelte/transition'
  import { addRoomId, delRoomId } from '../utils/roomId'

  let roomId: string | number
  let listener = null
  let count = 0
  let msg = [] // 弹幕数据

  let currentTime = Date.now()

  let lastIndex: number = 0
  let endIndex: number = 0
  let itemHeight: number = 24 // 每个项的高度 暂定
  let viewNum: number = 0 // 可视数量
  let boxEl // 盒子 dom对象
  let ulEl // 盒子内部显示的弹幕消息盒子
  let couldToBottom: boolean = true // 判断能否自动滚动到底部
  let heightCache: number[] = []
  let topCache: number[] = []
  $: msgHeight = heightCache.reduce((prev, next, i) => {
    topCache[i] = prev
    return prev + next || 0
  }, 0)

  $: showMsg = msg.slice(lastIndex, endIndex + 1) // 显示的弹幕数据

  const checkScroll = _e => {
    const now = Date.now()
    if (now - currentTime > 30) {
      currentTime = now
      window.requestAnimationFrame(scrollHandler)
    }
  }

  const resizeObserver = new ResizeObserver(_entries => {
    ulEl.querySelectorAll('li').forEach((e, i) => {
      if (!heightCache[lastIndex + i])
        heightCache[lastIndex + i] = e.offsetHeight
    })
  })
  const getStartIndex = (top: number) => {
    let index = -1
    let left = 0,
      right = topCache.length - 1,
      mid = Math.floor((left + right) / 2)
    // 判断 有可循环项时进入 二分查找
    while (right - left > 1) {
      // 目标数在左侧
      if (top < topCache[mid]) {
        right = mid
      } else if (top > topCache[mid]) {
        // 目标数在右侧
        left = mid
      } else {
        index = mid
        return index
      }
      mid = Math.floor((left + right) / 2)
    }
    index = left
    return index
  }
  const scrollHandler = () => {
    const startIndex = getStartIndex(boxEl.scrollTop)
    console.log('f', lastIndex, startIndex, viewNum)
    if (lastIndex === startIndex) return
    lastIndex = startIndex
    endIndex = startIndex + viewNum - 1
    if (endIndex >= msg.length - 1) {
      endIndex = msg.length - 1
      // 已经是最后一个
      couldToBottom = true
    } else {
      couldToBottom = false
    }
    console.log('e', lastIndex, endIndex)
  }
  const updateMsg = (str: string) => {
    msg = [...msg, str]
    heightCache[msg.length - 1] = itemHeight
    topCache[msg.length - 1] = topCache[msg.length - 2] + itemHeight
    couldToBottom && toBottom()
  }
  const toBottom = () => {
    endIndex = msg.length - 1
    lastIndex = endIndex - viewNum < 0 ? 0 : endIndex - viewNum
    window.requestAnimationFrame(() => {
      console.log('m', msg.length, msgHeight, topCache[endIndex])
      boxEl && (boxEl.scrollTop = boxEl.scrollHeight)
    })
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
      // .filter(str => str[0] === '[' || str[0] === '【') // 过滤系统信息
    })
    txt_index = end
  }
  onMount(async () => {
    if (!roomId) return
    resizeObserver.observe(ulEl)
    // 初始化容器最大容纳值
    viewNum = Math.ceil(boxEl.offsetHeight / itemHeight) + 1
    addRoomId(roomId)
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
      let str = ''
      switch (ev.payload) {
        case 'connected':
          str = '连接弹幕服务器成功'
          break
        case 'joined':
          str = '连接房间成功'
          break
        case 'closed':
          str = '意外关闭，请关闭页签重新连接'
          break
        case 'disconnect':
          str = '已断开连接'
          break
        default:
          str = `真实房间号：${ev.payload}`
          break
      }
      updateMsg(str)
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
              let str = ''
              switch (payload.body.cmd) {
                case 'LIVE': // 开播
                  str = `${dateFormat(Date.now(), 'hh:mm:ss')} 开播`
                  break
                case 'PREPARING': // 关播
                  str = `${dateFormat(Date.now(), 'hh:mm:ss')} 关播`
                  break
                case 'DANMU_MSG': // 用户发送的消息
                  const info = payload.body.info
                  str = `[${dateFormat(info[0][4], 'hh:mm:ss')}] ${
                    info[2][1]
                  }：${info[1]}`
                  break
                case 'WATCHED_CHANGE': // 多少人看过
                  const data = payload.body.data
                  count = data.num
                  break
                case 'SEND_GIFT': // 礼物
                  const gift = payload.body.data
                  str = `<i class="${
                    gift['coin_type'] === 'gold'
                      ? 'text-amber-600'
                      : 'text-amber-900'
                  }">【礼物】</i>[${dateFormat(
                    gift['timestamp'] * 1000,
                    'hh:mm:ss'
                  )}] ${gift['uname']} ${gift['action']} ${gift['num']} 个 ${
                    gift['giftName']
                  }`
                  break
                case 'GUARD_BUY': // 上舰
                  const guard = payload.body.data
                  console.log(guard)
                  const guardName =
                    guard['guard_level'] === 3
                      ? '舰长'
                      : guard['guard_level'] === 2
                      ? '提督'
                      : guard['guard_level'] === 1
                      ? '总督'
                      : ''
                  // FIXME 上舰时间
                  str = `<i class="text-violet-600">【上舰】</i>[${dateFormat(
                    Date.now(),
                    'hh:mm:ss'
                  )}] ${guard['username']} 购买 ${
                    guard['num']
                  } 个月 ${guardName}`
                  break
                case 'SUPER_CHAT_MESSAGE': //sc
                case 'SUPER_CHAT_MESSAGE_JP':
                  const sc = payload.body.data
                  str = `<i class="text-rose-600">【SC：${
                    sc['price']
                  }】</i>[${dateFormat(sc['ts'] * 1000, 'hh:mm:ss')}] ${
                    sc['user_info']['uname']
                  }： <span style="color:${sc['message_font_color']};">${
                    sc['message']
                  }`
                  break
                default: // 其他数据
                  if (payload.body.cmd.includes('DANMU_MSG')) {
                    // 遇上了奇怪的 DANMU_MSG 似乎是活动的类型
                    const info = payload.body.info
                    str = `[${dateFormat(info[0][4], 'hh:mm:ss')}] ${
                      info[2][1]
                    }：${info[1]}`
                  }
              }
              if (str) {
                updateMsg(str)
                const sideWindow = WebviewWindow.getByLabel('side-' + roomId)
                sideWindow && sideWindow.emit('add-' + roomId, str)
              }
              break
          }
        })
      }
    )
    // 定时写入文件 30s
    interval = setInterval(() => {
      write_danmaku()
    }, 30 * 1000)
  })
  onDestroy(() => {
    resizeObserver.unobserve(ulEl)
    interval && clearInterval(interval)
    write_danmaku()
    invoke('disconnect', { roomId })
    // 断开连接 解除监听
    if (listener) {
      listener['stream']()
      listener['danmaku']()
      listener = null
    }
    delRoomId(roomId)
  })
  export { roomId, write_danmaku }
</script>

<div
  class="rounded border border-solid h-full bg-white dark:bg-black shadow-md relative"
>
  <div
    class="overflow-y-auto h-full"
    id="box"
    bind:this={boxEl}
    on:scroll={checkScroll}
  >
    <ul
      class="overflow-hidden py-1 px-2"
      bind:this={ulEl}
      style:padding-top={topCache[lastIndex] + 'px'}
      style:padding-bottom={msgHeight - topCache[endIndex] - 24 + 'px'}
    >
      {#each showMsg as d, _i}
        <li in:fade>
          {@html d}
        </li>
      {/each}
    </ul>
  </div>
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
  {#if !couldToBottom}
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
