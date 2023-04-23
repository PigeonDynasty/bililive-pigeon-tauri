<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import Select from '@/components/Select.svelte'
  import rooms from '@/store/room'
  const dispatch = createEventDispatcher()
  let value
  let className = ''
  let disabled = false
  const change = detail => {
    dispatch('change', detail)
  }

  export { value, className as class, disabled }
</script>

<Select
  bind:value
  class={className}
  valueKey="room_id"
  placeholder="请选择已连接的房间"
  {disabled}
  let:item
  data={$rooms}
  on:change={e => change(e.detail)}
>
  {item['room_id']} - {item['uname']}
</Select>
