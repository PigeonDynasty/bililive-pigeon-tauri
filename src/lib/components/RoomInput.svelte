<script lang="ts">
  import Popover from '@/components/Popover.svelte'
  import Input from '@/components/Input.svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  let value
  let className = ''
  let visible = false
  let data = []
  $: showData = value
    ? data.filter(
        item =>
          String(item.room_id).includes(value) ||
          item.uname.toLowerCase().includes(value.toLowerCase())
      )
    : data
  const toggle = async bol => {
    if (!bol) return
    data = await invoke('get_history')
    if (data.length === 0) visible = false
  }
  const click = item => {
    value = String(item.room_id)
    visible = false
  }
  const input = () => {
    visible = showData.length > 0
  }
  export { value, className as class }
</script>

<Popover
  bind:visible
  trigger="manual"
  class={className}
  popoverClass="min-w-[13rem]"
  on:toggle={e => toggle(e.detail)}
>
  <Input
    slot="trigger"
    bind:value
    placeholder="请输入房间号"
    on:input={() => input()}
  />
  <ul class="max-h-36 text-sm -mx-2">
    {#each showData as item}
      <li
        class="px-2 py-1 cursor-pointer hover:bg-sky-50 hover:dark:bg-sky-900 hover:dark:bg-opacity-50 whitespace-nowrap"
        on:click={() => click(item)}
      >
        {item.room_id} - {item.uname}
      </li>
    {/each}
  </ul>
</Popover>
