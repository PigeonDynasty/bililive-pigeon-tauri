<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import Select from '@/components/Select.svelte'
  let value
  let className = ''
  const filter = (val, data) =>
    data.filter(
      item =>
        String(item.room_id).includes(val) ||
        item.uname.toLowerCase().includes(val.toLowerCase())
    )

  const getData = async () => await invoke('get_history')
  export { value, className as class }
</script>

<Select
  bind:value
  class={className}
  autocomplete
  valueKey="room_id"
  placeholder="请输入房间号"
  let:item
  {filter}
  dataMethod={getData}
>
  {item['room_id']} - {item['uname']}
</Select>
