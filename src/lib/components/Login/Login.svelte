<script lang="ts">
  import Dialog from '@/components/Dialog.svelte'
  import ByPassword from './ByPassword.svelte'
  let visible: boolean = false
</script>

<button class="btn-sky py-1 px-2" on:click={() => (visible = true)}>
  登录
</button>
<Dialog bind:visible class="login-content">
  <div class="flex px-12 py-7 w-full items-center justify-center text-center">
    <!-- <div class="text-center">
      <p class="text-lg mb-4">扫描二维码登录</p>
      <div class="qrcode-box">
        <div class="qrcode relative hover:bg-white">
          <div
            class="qrcode-main w-40 h-40 mx-auto duration-500"
            title="https://passport.bilibili.com/h5-app/passport/login/scan?navhide=1&amp;qrcode_key=008c3449bec85c31f28b2255622c9e3d&amp;from=main-mini"
          >
            <img
              alt="Scan me!"
              src="data:image/png;base64,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"
            />
          </div>
          <img
            src="https://s1.hdslb.com/bfs/seed/jinkela/short/mini-login/img/qr-tips.51ff2bcf.png"
            alt="Scan Tips"
            class="qrcode-tips absolute block w-80 h-40 bottom-0 left-1/2 -translate-x-1/2 opacity-0 duration-500 object-cover max-w-none"
          />
        </div>
        <div class="bili-mini-scan-refresh hidden">
          <div />
          <p>二维码已过期</p>
          <p>请点击刷新</p>
        </div>
        <div class="bili-mini-scan-success hidden">
          <div />
          <p>扫码成功</p>
          <p class="line2">请在手机登录</p>
        </div>
      </div>
      <p class="bili-mini-scan-text mt-4">
        请使用
        <a
          class="text-sky-400"
          rel="noreferrer"
          href="https://app.bilibili.com/"
          target="_blank"
        >
          哔哩哔哩客户端
        </a>
      </p>
      <p class="bili-mini-scan-text">扫码登录或扫码下载APP</p>
    </div> -->
    <!-- <div class="w-[1px] mx-10 h-64 block bg-zinc-200" /> -->
    <div class="w-2/3">
      <div class="flex divide-x divide-zinc-200 mb-4">
        <span class="text-sky-400 flex-1 text-lg"> 密码登录 </span>
        <!-- <span class="flex-1 text-lg"> 短信登录 </span> -->
      </div>
      <ByPassword />
      <!-- <div class="third-party-login-wrapper">
        <div class="title">其他方式登录</div>
        <div class="sns">
          <span class="btn wechat">微信登录</span>
          <span class="btn weibo">微博登录</span>
          <span class="btn qq">QQ登录</span>
        </div>
      </div> -->
    </div>
  </div>
  <div class="text-xs text-zinc-400">
    <!-- <p>未注册过哔哩哔哩的手机号，我们将自动帮你注册账号</p> -->
    <p>
      登录或完成注册即代表你同意
      <a
        class="text-sky-400"
        rel="noreferrer"
        target="_blank"
        href="https://www.bilibili.com/protocal/licence.html"
      >
        用户协议
      </a>
      和
      <a
        class="text-sky-400"
        rel="noreferrer"
        target="_blank"
        href="https://www.bilibili.com/blackboard/privacy-pc.html"
      >
        隐私政策
      </a>
    </p>
  </div>
</Dialog>

<style lang="postcss">
  :global(.login-content) {
    background-image: url(/img/22.png), url(/img/33.png);
    background-position: 0 100%, 100% 100%;
    background-repeat: no-repeat, no-repeat;
    background-size: 14%;
  }
  /* .qrcode:hover .qrcode-main {
    @apply opacity-0;
  }
  .qrcode:hover .qrcode-tips {
    @apply opacity-100;
  } */
</style>
