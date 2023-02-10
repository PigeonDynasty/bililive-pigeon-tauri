<script lang="ts">
  import Select from '@/components/Select.svelte'
  import rooms from '@/store/room'
  let value
  let className = ''
  export { value, className as class }
</script>

<Select
  bind:value
  class={className}
  valueKey="room_id"
  placeholder="请选择已连接的房间"
  let:item
  data={$rooms}
>
  {item['room_id']} - {item['uname']}
</Select>
