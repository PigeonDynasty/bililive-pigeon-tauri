<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import Grab, { GrabType } from './Template/Grab.svelte'
  import Popover, { PopoverPlacement } from './Popover.svelte'
  let value = 0
  let min = 0
  let max = 100
  let className = ''
  let grabbing = false
  let left = 0
  let popoverRef
  const dispatch = createEventDispatcher()
  $: {
    if (!grabbing && (value || min !== 0 || max !== 100)) {
      setLeft()
    }
  }
  const move = _ => {
    value = Math.round((left * (max - min)) / 100 + min)
    popoverRef && popoverRef.setPos()
    dispatch('change', value)
  }

  const setLeft = () => {
    if (value < min) value = min
    else if (value > max) value = max
    left = (100 * (value - min)) / (max - min)
    dispatch('change', value)
  }
  export { value, min, max, className as class }
</script>

<div class={`inline-flex items-center ${className}`}>
  <div class="flex-1 mx-3">
    <Grab
      class="bg-zinc-200 rounded cursor-pointer relative"
      type={GrabType.X}
      percent
      bind:left
      bind:grabbing
      on:move={move}
    >
      <div
        class="bg-sky-400 dark:bg-sky-800 h-2 rounded"
        style:width={left + '%'}
      />
      <Popover
        slot="grab"
        bind:this={popoverRef}
        bind:visible={grabbing}
        trigger="manual"
        placement={PopoverPlacement.TOP}
      >
        <span
          slot="trigger"
          class="w-5 h-5 border-2 border-sky-400 dark:border-sky-800 rounded-full bg-white dark:bg-black hover:scale-110"
        />
        <span>{value}</span>
      </Popover>
    </Grab>
  </div>
  <input type="number" class="input w-20 text-center" bind:value />
</div>
