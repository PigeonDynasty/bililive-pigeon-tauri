<script lang="ts">
  import { onMount } from 'svelte'
  import Popover, { PopoverPlacement } from './Popover.svelte'
  let value = 0
  let min = 0
  let max = 100
  let className = ''
  let grabbing = false
  let left = 0
  let sliderEl, grabEl, popoverRef
  const mouseMove = e => {
    if (!grabbing) return
    left = e.clientX - sliderEl.offsetLeft - grabEl.offsetWidth / 2
    if (left < 0) left = 0
    else if (left > sliderEl.offsetWidth - grabEl.offsetWidth)
      left = sliderEl.offsetWidth - grabEl.offsetWidth
    requestAnimationFrame(() => {
      value = Math.round(
        (left / (sliderEl.offsetWidth - grabEl.offsetWidth)) * (max - min) + min
      )
      popoverRef && popoverRef.setPos()
    })
  }
  const bgMouseDown = e => {
    grabbing = true
    mouseMove(e)
  }
  const setLeft = () => {
    left =
      ((value - min) / (max - min)) *
      (sliderEl.offsetWidth - grabEl.offsetWidth)
  }
  onMount(() => {
    setLeft()
  })
  export { value, min, max, className as class }
</script>

<svelte:window
  on:mousemove={mouseMove}
  on:mouseup={() => (grabbing = false)}
  class:cursor-grabbing={grabbing}
/>
<div class={`inline-flex items-center ${className}`}>
  <div class="flex-1 relative px-[0.625rem]" bind:this={sliderEl}>
    <div
      class="bg-zinc-200 h-2 rounded cursor-pointer"
      on:mousedown={bgMouseDown}
    />
    <div
      class="absolute top-1/2 -translate-y-1/2 bg-sky-400 dark:bg-sky-800 h-2 rounded z-0 pointer-events-none"
      style:width={left + 'px'}
    />
    <Popover
      bind:this={popoverRef}
      bind:visible={grabbing}
      trigger="manual"
      placement={PopoverPlacement.TOP}
      class="absolute top-1/2 -translate-y-1/2"
      style={`left:${left}px`}
    >
      <span
        slot="trigger"
        class="w-5 h-5 border-2 border-sky-400 dark:border-sky-800 rounded-full z-10 bg-white dark:bg-black hover:scale-110"
        class:cursor-grab={!grabbing}
        class:cursor-grabbing={grabbing}
        bind:this={grabEl}
        on:mousedown={() => (grabbing = true)}
      />
      <span>{value}</span>
    </Popover>
  </div>
  <input
    type="number"
    class="input w-20 text-center"
    bind:value
    on:input={() => setLeft()}
  />
</div>
