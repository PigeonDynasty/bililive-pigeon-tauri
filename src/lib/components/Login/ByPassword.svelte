<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte'
  import Eye from '@/icons/Eye.svelte'
  import EyeSlash from '@/icons/EyeSlash.svelte'
  import { getWebLocation, initCaptcha } from './Captcha'
  import toast from '@/utils/toast'
  import { invoke } from '@tauri-apps/api'
  import { updateCookie } from '@/store/account'
  import account from '@/store/account'
  import { setCookieToCookieStr } from '@/utils/utils'

  let showPassword: boolean = false
  let username: string
  let password: string
  let disabled: boolean = false
  const dispatch = createEventDispatcher()

  const login = () => {
    if (!username) {
      toast('请输入账号')
      return
    }
    if (!password) {
      toast('请输入密码')
      return
    }
    disabled = true
    let webLocation: string
    getWebLocation()
      .then(res => {
        webLocation = res
        return initCaptcha(webLocation)
      })
      .then(res => {
        return invoke('login_by_password', {
          ua: navigator.userAgent,
          cookie: ($account.cookie ?? '').trim(),
          username,
          password,
          webLocation,
          token: res.data.token,
          challenge: res.data.geetest.challenge,
          validate: res.captcha.geetest_validate,
          seccode: res.captcha.geetest_seccode
        })
      })
      .then((res: any) => {
        if (!res.data.data) {
          toast('登录失败')
          return
        }
        const cookieStr = setCookieToCookieStr(res.header['set-cookie'])
        updateCookie($account.cookie + ';' + cookieStr)
        toast('登录成功')
        dispatch('success')
      })
      .finally(() => {
        disabled = false
      })
  }
  onMount(() => {
    if ($account.cookie && $account.cookie.includes('buvid3')) return
    Promise.all([invoke('get_buvid'), invoke('get_b_nut')]).then((res: any) => {
      const cookieMap = new Map()
      cookieMap.set('buvid3', res[0].data.b_3)
      cookieMap.set('buvid4', res[0].data.b_4)
      const cookie = setCookieToCookieStr(res[1]['set-cookie'], cookieMap)
      updateCookie(cookie)
    })
  })
</script>

<div class="divide-y divide-zinc-300 border border-zinc-300 rounded-lg">
  <div class="flex px-5 h-12 items-center">
    <span class="mr-5">账号</span>
    <input
      class="flex-1 outline-none"
      type="text"
      placeholder="请输入账号"
      bind:value={username}
      {disabled}
    />
  </div>
  <div class="flex px-5 h-12 items-center">
    <div class="flex-1 flex">
      <span class="mr-5">密码</span>
      <input
        class="flex-1 outline-none dark:bg-black bg-white"
        class:hidden={showPassword}
        placeholder="请输入密码"
        type="password"
        bind:value={password}
        {disabled}
      />
      <input
        class="flex-1 outline-none dark:bg-black bg-white"
        class:hidden={!showPassword}
        placeholder="请输入密码"
        type="text"
        bind:value={password}
        {disabled}
      />
    </div>
    <div
      role="button"
      tabindex="0"
      class="hover:text-sky-400 w-5 h-5 cursor-pointer"
      on:click={() => (showPassword = !showPassword)}
      on:keypress={() => {}}
    >
      {#if showPassword}
        <Eye />
      {:else}
        <EyeSlash />
      {/if}
    </div>
  </div>
</div>
<div class="flex gap-4 justify-center mt-4">
  <button
    class="btn-sky rounded-md w-1/2 py-2"
    {disabled}
    on:click={() => login()}
  >
    登录
  </button>
</div>
