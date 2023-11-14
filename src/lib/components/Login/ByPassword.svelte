<script lang="ts">
  import Eye from '@/icons/Eye.svelte'
  import EyeSlash from '@/icons/EyeSlash.svelte'
  let showPassword: boolean = false
</script>

<div class="divide-y divide-zinc-300 border border-zinc-300 rounded-lg">
  <div class="flex px-5 h-12 items-center">
    <span class="mr-5">账号</span>
    <input class="flex-1 outline-none" type="text" placeholder="请输入账号" />
  </div>
  <div class="flex px-5 h-12 items-center">
    <div class="flex-1 flex">
      <span class="mr-5">密码</span>
      <input
        class="flex-1 outline-none"
        placeholder="请输入密码"
        type={showPassword ? 'text' : 'password'}
      />
    </div>
    <div
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
  <button class="btn-sky rounded-md w-1/2 py-2">登录</button>
</div>
