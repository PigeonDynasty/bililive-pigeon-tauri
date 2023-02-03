<script lang="ts">
  import Popover from './Popover.svelte'
  import ScrollPicker from './ScrollPicker.svelte'
  let visible = false
  let value
</script>

<Popover bind:visible trigger="manual" popoverClass="w-60">
  <ScrollPicker bind:value />
</Popover>
