<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import Popover, { PopoverPlacement } from './Popover.svelte'
  let value = 0
  let min = 0
  let max = 100
  let className = ''
  let grabbing = false
  let left = 0
  let sliderEl, popoverRef
  const dispatch = createEventDispatcher()
  $: {
    if (!grabbing && (value || min !== 0 || max !== 100)) {
      setLeft()
    }
  }
  const mouseMove = e => {
    if (!grabbing) return
    left = ((e.clientX - sliderEl.offsetLeft) * 100) / sliderEl.offsetWidth
    if (left < 0) left = 0
    else if (left > 100) left = 100
    requestAnimationFrame(() => {
      value = Math.round((left * (max - min)) / 100 + min)
      popoverRef && popoverRef.setPos()
      dispatch('change', value)
    })
  }
  const bgMouseDown = e => {
    grabbing = true
    mouseMove(e)
  }
  const setLeft = () => {
    if (value < min) value = min
    else if (value > max) value = max
    left = (100 * (value - min)) / (max - min)
    dispatch('change', value)
  }
  export { value, min, max, className as class }
</script>

<svelte:window
  on:mousemove={mouseMove}
  on:mouseup={() => (grabbing = false)}
  class:cursor-grabbing={grabbing}
/>
<div class={`inline-flex items-center ${className}`}>
  <div class="flex-1 mx-3" bind:this={sliderEl}>
    <div
      class="bg-zinc-200 rounded cursor-pointer relative"
      on:mousedown={bgMouseDown}
    >
      <div
        class="bg-sky-400 dark:bg-sky-800 h-2 rounded"
        style:width={left + '%'}
      />
      <Popover
        bind:this={popoverRef}
        bind:visible={grabbing}
        trigger="manual"
        placement={PopoverPlacement.TOP}
        class="absolute top-1/2 -translate-y-1/2 -translate-x-1/2"
        style={`left:${left}%`}
      >
        <span
          slot="trigger"
          class="w-5 h-5 border-2 border-sky-400 dark:border-sky-800 rounded-full bg-white dark:bg-black hover:scale-110"
          class:cursor-grab={!grabbing}
          class:cursor-grabbing={grabbing}
          on:mousedown={() => (grabbing = true)}
        />
        <span>{value}</span>
      </Popover>
    </div>
  </div>
  <input type="number" class="input w-20 text-center" bind:value />
</div>
