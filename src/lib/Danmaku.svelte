<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
  import { html2text } from '../utils/utils'
  import dayjs from 'dayjs'
  import { fade } from 'svelte/transition'
  import { updateRoomInfo } from '../store/room'
  import { addGift } from '../store/gift'
  import Eye from '@/icons/Eye.svelte'
  import ArrowSmallDown from '@/icons/ArrowSmallDown.svelte'
  import plugins from '../store/plugin'
  import { formatInteractTypeName, formatGuardName } from '../utils/danmaku'
  import account from '@/store/account'
  import danmakuWorker from '@/workers/danmaku?worker'

  let roomId: string | number
  let listener = null
  let count = 0
  let msg = [] // 弹幕数据
  let currentTime = Date.now()
  const estimatedItemHeight: number = 24 // 预估每个项的高度 这里用单行文本高度
  let viewNum: number = 0 // 可视数量
  let boxEl // 盒子 dom对象
  let ulEl // 盒子内部显示的弹幕消息盒子
  let couldToBottom: boolean = true // 判断能否自动滚动到底部
  let pt = 0
  let pb = 0

  const worker = new danmakuWorker()
  const onMessage = e => {
    const { type, data, innerHeight, height, length, paddingTop } = e.data
    switch (type) {
      case 'added': {
        if (couldToBottom) {
          getLast()
        } else {
          pb = height - pt - innerHeight
        }
        break
      }
      case 'get':
        msg = [...data]
        const lastItem = data[data.length - 1]
        couldToBottom = lastItem ? lastItem.index === length : true
        pt = paddingTop || 0

        if (couldToBottom) {
          pb = 0
          requestAnimationFrame(() => {
            boxEl && (boxEl.scrollTop = boxEl.scrollHeight)
            couldToBottom = true
          })
        } else {
          pb = height - pt - innerHeight
        }
        break
    }
  }
  const checkScroll = _e => {
    const now = Date.now()
    if (now - currentTime > 100) {
      currentTime = now
      requestAnimationFrame(scrollHandler)
    }
  }
  const resizeObserver = new ResizeObserver(_entries => {
    ulEl.querySelectorAll('li').forEach((e, i) => {
      if (e.offsetHeight > 0 && msg[i].height !== e.offsetHeight) {
        msg[i].height = e.offsetHeight
        worker.postMessage({
          type: 'update',
          tableName: roomId,
          data: [msg[i]]
        })
      }
    })
  })
  const boxResizeObserver = new ResizeObserver(_entries => {
    if (boxEl.offsetHeight)
      viewNum = Math.ceil(boxEl.offsetHeight / estimatedItemHeight) * 3
  })
  const intersectionObserver = new IntersectionObserver(entries => {
    // 如果 intersectionRatio 为 0，则目标在视野外，
    // 我们不需要做任何事情。
    if (entries[0].intersectionRatio <= 0) return
    requestAnimationFrame(() => {
      if (couldToBottom) {
        toBottom()
      } else {
        boxEl && (boxEl.scrollTop = pt)
      }
    })
  })
  const scrollHandler = () => {
    worker.postMessage({
      type: 'getByTop',
      tableName: roomId,
      data: {
        top: boxEl.scrollTop,
        viewNum: viewNum
      }
    })
  }
  const updateMsg = (str: string) => {
    worker.postMessage({
      type: 'add',
      tableName: roomId,
      data: {
        msg: str,
        height: estimatedItemHeight
      }
    })
    invoke('write_danmaku_txt', {
      roomId,
      date: dayjs().format('YYYY-MM-DD'),
      data: [html2text(str)]
    })
  }
  const getLast = () => {
    worker.postMessage({
      type: 'getLast',
      tableName: roomId,
      data: {
        viewNum: viewNum
      }
    })
  }
  const toBottom = () => {
    boxEl && (boxEl.scrollTop = boxEl.scrollHeight)
    couldToBottom = true
    pb = 0
    requestAnimationFrame(() => {
      getLast()
    })
  }

  const danmakuTime = (t?: string | number) => {
    return `<span class="danmaku-time">[${dayjs(t).format('HH:mm:ss')}]</span>`
  }
  const formatDanmuMsg = async (info): Promise<string> => {
    const msg =
      info[0][13] === '{}'
        ? await replaceEmoji(info[1])
        : `<img class="danmaku-emoji${
            info[0][13].bulge_display === 1 ? '-custom' : ''
          }"  referrerpolicy="no-referrer" alt="${info[1]}" src="${
            info[0][13].url
          }@${info[0][13].bulge_display === 1 ? '80' : '40'}h.webp"/>`
    return `${danmakuTime(info[0][4])} <span class="danmaku-username">${
      info[2][1]
    }</span>：${msg}`
  }
  const replaceEmoji = async (tem: string): Promise<string> => {
    const reg = /\[(.+?)\]/g
    const keys: String[] = Array.from(new Set(tem.match(reg)))
    if (keys.length === 0) return tem
    const emojis: DbEmoji[] = await invoke('get_emojis', {
      emojis: keys
    })
    emojis.forEach((emoji: DbEmoji) => {
      tem = tem.replaceAll(
        emoji.emoji,
        `<img class="danmaku-emoji" referrerpolicy="no-referrer" alt="${emoji.emoji}" src="${emoji.url}@40h.webp"/>`
      )
    })
    return tem
  }
  const emit2plugins = (type: string, payload: any) => {
    $plugins.forEach(plugin => {
      if (plugin.visible && plugin[`on${type}`]) plugin[`on${type}`](payload)
    })
  }
  onMount(async () => {
    if (!roomId) return
    resizeObserver.observe(ulEl)
    // 初始化容器最大容纳值
    boxResizeObserver.observe(boxEl)
    intersectionObserver.observe(boxEl)
    viewNum = Math.ceil(boxEl.offsetHeight / estimatedItemHeight) * 3
    worker.addEventListener('message', onMessage)
    updateMsg(`${danmakuTime()} 开始连接...`)
    invoke('connect', { roomId, cookie: ($account.cookie ?? '').trim() })
    if (listener) {
      // 防止重复监听
      listener['stream']()
      listener['danmaku']()
    } else {
      listener = {}
    }
    listener['stream'] = await appWindow.listen('stream-' + roomId, ev => {
      let str = danmakuTime() + ' '
      switch (ev.payload) {
        case 'connected':
          str += '连接弹幕服务器成功'
          break
        case 'joined':
          str += '连接房间成功'
          break
        case 'closed':
          str += '意外关闭，请关闭页签重新连接'
          break
        case 'disconnect':
          str += '已断开连接'
          break
        default:
          str += `真实房间号：<span class="stream-roomid">${ev.payload['true_room_id']}</span>`
          updateRoomInfo(ev.payload)
          break
      }
      updateMsg(str)
      emit2plugins('Stream', ev.payload)
    })
    listener['danmaku'] = await appWindow.listen(
      'danmaku-' + roomId,
      (ev: any) => {
        ev.payload.forEach(async payload => {
          switch (payload.op) {
            case 3: //气人值
              // count = payload.body.count
              break
            case 5: // wss消息
              let str = ''
              switch (payload.body.cmd) {
                case 'LIVE': {
                  // 开播
                  str = `${danmakuTime()} 开播`
                  break
                }
                case 'PREPARING': {
                  // 关播
                  str = `${danmakuTime()} 关播`
                  break
                }
                case 'DANMU_MSG': {
                  // 用户发送的消息
                  str = await formatDanmuMsg(payload.body.info)
                  break
                }
                case 'WATCHED_CHANGE': {
                  // 多少人看过
                  const data = payload.body.data
                  count = data.num
                  break
                }
                case 'SEND_GIFT': {
                  // 礼物
                  const gift = payload.body.data
                  str = `<i class="danmaku-gift-${
                    gift['coin_type']
                  }">【礼物】</i>${danmakuTime(
                    gift['timestamp'] * 1000
                  )} <span class="danmaku-username">${gift['uname']}</span> ${
                    gift['action']
                  } ${gift['num']} 个 ${gift['giftName']}`
                  addGift(
                    roomId,
                    gift['uid'],
                    gift['uname'],
                    gift['giftName'],
                    gift['num'],
                    gift['coin_type']
                  )
                  break
                }
                case 'GUARD_BUY': {
                  // 上舰
                  const guard = payload.body.data
                  console.log(guard)
                  // FIXME 上舰时间
                  str = `<i class="danmaku-guard">【上舰】</i>${danmakuTime()} <span class="danmaku-username">${
                    guard['username']
                  }</span> 购买 ${guard['num']} 个月 ${formatGuardName(
                    guard['guard_level']
                  )}`
                  break
                }
                case 'SUPER_CHAT_MESSAGE': //sc
                case 'SUPER_CHAT_MESSAGE_JP': {
                  const sc = payload.body.data
                  str = `<i class="danmaku-sc">【SC：${
                    sc['price']
                  }】</i>${danmakuTime(
                    sc['ts'] * 1000
                  )} <span class="danmaku-username">${
                    sc['user_info']['uname']
                  }</span>： <span style="color:${sc['message_font_color']};">${
                    sc['message']
                  }`
                  break
                }
                case 'WELCOME_GUARD': {
                  const wlGuard = payload.body.data
                  // FIXME 进入房间时间
                  console.log(wlGuard)
                  str = `${danmakuTime()} 欢迎 <i class="danmaku-guard">【${formatGuardName(
                    wlGuard['guard_level']
                  )}】</i> <span class="danmaku-username">${
                    wlGuard['username']
                  }</span> 进入直播间`
                  break
                }
                case 'INTERACT_WORD': {
                  // 进入、关注、分享
                  const iw = payload.body.data
                  str = `${danmakuTime(
                    iw['timestamp'] * 1000
                  )} <span class="danmaku-username">${
                    iw['uname']
                  }</span> <strong>${formatInteractTypeName(
                    iw['msg_type']
                  )}</strong> 直播间`
                  break
                }
                case 'WARNING': {
                  // 警告信息
                  str = `${danmakuTime()} ${payload.body.msg || 'warning'}`
                  break
                }
                case 'CUT_OFF': {
                  // 切断直播
                  str = `${danmakuTime()} ${payload.body.msg || 'cut off'}`
                  break
                }
                default: // 其他数据
                  if (payload.body.cmd.includes('DANMU_MSG')) {
                    // 遇上了奇怪的 DANMU_MSG 似乎是活动的类型
                    str = await formatDanmuMsg(payload.body.info)
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
        emit2plugins('Danmaku', ev.payload)
      }
    )
  })
  onDestroy(() => {
    const sideWindow = WebviewWindow.getByLabel('side-' + roomId)
    sideWindow && sideWindow.close()
    resizeObserver.unobserve(ulEl)
    boxResizeObserver.unobserve(boxEl)
    intersectionObserver.unobserve(boxEl)
    invoke('disconnect', { roomId })
    worker.postMessage({
      type: 'close',
      tableName: roomId
    })
    worker.removeEventListener('message', onMessage)
    worker.terminate()
    // 断开连接 解除监听
    if (listener) {
      listener['stream']()
      listener['danmaku']()
      listener = null
    }
  })
  export { roomId }
</script>

<div
  class="rounded border border-solid h-full bg-white dark:bg-black shadow-md relative"
>
  <div class="overflow-y-auto h-full" bind:this={boxEl} on:scroll={checkScroll}>
    <ul
      class="danmaku-msg overflow-hidden py-1 px-2"
      bind:this={ulEl}
      style:padding-top={pt + 'px'}
      style:padding-bottom={pb + 'px'}
    >
      {#each msg as d, _i}
        <li in:fade>
          {@html d.msg}
        </li>
      {/each}
    </ul>
  </div>
  <span
    class="absolute top-2 right-6 px-2 flex items-center shadow-md rounded-lg bg-gray-100 dark:bg-gray-800 text-sm"
    title="看过"
  >
    <i class="w-4 h-4 mr-2">
      <Eye />
    </i>
    {count}
  </span>
  {#if !couldToBottom}
    <button
      class="btn-sky text-xs leading-3 rounded-full py-1 px-2 scale-90 absolute right-1 bottom-1"
      transition:fade
      on:click={toBottom}
    >
      <i class="w-3 h-3 float-left mr-1">
        <ArrowSmallDown />
      </i>
      回到底部
    </button>
  {/if}
</div>

<style lang="postcss">
  .danmaku-msg {
    --danmaku-msg: initial;
    --stream-roomid: inherit;
    --danmaku-time: inherit;
    --danmaku-username: inherit;
    --danmaku-gift-gold: #d97706;
    --danmaku-gift-silver: #78350f;
    --danmaku-guard: #7c3aed;
    --danmaku-sc: #e11d48;
    color: var(--danmaku-msg);
  }
  .danmaku-msg :global(.stream-roomid) {
    color: var(--stream-roomid);
  }
  .danmaku-msg :global(.danmaku-time) {
    color: var(--danmaku-time);
  }
  .danmaku-msg :global(.danmaku-username) {
    color: var(--danmaku-username);
  }
  .danmaku-msg :global(.danmaku-gift-gold) {
    color: var(--danmaku-gift-gold);
  }
  .danmaku-msg :global(.danmaku-gift-silver) {
    color: var(--danmaku-gift-silver);
  }
  .danmaku-msg :global(.danmaku-guard) {
    color: var(--danmaku-guard);
  }
  .danmaku-msg :global(.danmaku-sc) {
    color: var(--danmaku-sc);
  }
  .danmaku-msg :global(.danmaku-emoji) {
    @apply inline-block h-5;
  }
  .danmaku-msg :global(.danmaku-emoji-custom) {
    @apply inline-block h-10 rounded;
  }
</style>
