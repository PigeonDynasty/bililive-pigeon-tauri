<script lang="ts">
  import Dialog from '@/components/Dialog.svelte'
  let visible: boolean = false
</script>

<button
  class="btn-sky w-4 h-4 rounded-full text-xs"
  on:click={() => (visible = true)}
>
  ?
</button>
<Dialog bind:visible title="获取Cookie">
  <article>
    <p>
      1. 进入
      <a
        class="text-sky-400"
        href="https://live.bilibili.com"
        rel="noreferrer"
        target="_blank"
      >
        https://live.bilibili.com
      </a>
    </p>
    <p>2. 右键->检查 打开控制台,或者使用快捷键打开：</p>
    <p>&emsp;1) Windows: <strong>F12 / Ctrl+ Shift+ I </strong></p>
    <p>&emsp;2) Mac: <strong>F12 / Command + Option + I</strong></p>
    <p>
      3. 切换到 <strong>Network(网络)</strong> 标签,然后刷新页面
    </p>
    <p>
      4. 点击HTTP请求,找到
      <strong>Headers(标头) -> Request Headers(请求标头) -> Cookie</strong>
      ,复制对应内容,见下图:
    </p>

    <img src="/img/cookie-helper.png" alt="cookie helper" />
    <small class="text-rose-400">
      tip: 请注意复制完整,图片马赛克位置内容没有截全
    </small>
  </article>
</Dialog>
