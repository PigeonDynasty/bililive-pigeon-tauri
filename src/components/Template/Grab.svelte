<script lang="ts" context="module">
  export enum GrabType {
    X,
    Y,
    DEFAULT
  }
</script>

<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  let type: GrabType = GrabType.DEFAULT
  let left: number = 0
  let top: number = 0
  let percent: boolean = false
  let grabbing: boolean = false
  let className = ''

  let grabEl
  $: unit = percent ? '%' : 'px'
  const dispatch = createEventDispatcher()
  const mouseMove = e => {
    if (!grabbing) return
    left = percent
      ? ((e.clientX - grabEl.offsetLeft) * 100) / grabEl.offsetWidth
      : e.clientX - grabEl.offsetLeft
    if (left < 0) left = 0
    else if (left > (percent ? 100 : grabEl.offsetWidth))
      left = percent ? 100 : grabEl.offsetWidth

    top = percent
      ? ((e.clientY - grabEl.offsetTop) * 100) / grabEl.offsetTop
      : e.clientY - grabEl.offsetTop
    if (top < 0) top = 0
    else if (top > (percent ? 100 : grabEl.offsetHeight))
      left = percent ? 100 : grabEl.offsetHeight
    requestAnimationFrame(() => {
      dispatch('move', {
        left,
        top,
        width: grabEl.offsetWidth,
        height: grabEl.offsetHeight
      })
    })
  }
  const bgMouseDown = e => {
    grabbing = true
    mouseMove(e)
  }
  export { type, percent, left, top, grabbing, className as class }
</script>

<svelte:window
  on:mousemove={mouseMove}
  on:mouseup={() => (grabbing = false)}
  class:cursor-grabbing={grabbing}
/>
<div class={className} bind:this={grabEl} on:mousedown={bgMouseDown}>
  <slot />
  <span
    class="absolute top-1/2 -translate-y-1/2 -translate-x-1/2 leading-none"
    style:left={type === GrabType.Y ? '' : `${left}${unit}`}
    style:top={type === GrabType.X ? '' : `${top}${unit}`}
    class:cursor-grab={!grabbing}
    class:cursor-grabbing={grabbing}
    on:mousedown={() => (grabbing = true)}
  >
    <slot name="grab" />
  </span>
</div>
