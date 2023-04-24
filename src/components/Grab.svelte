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
  $: grabClass = () => {
    const classList = [
      'absolute -translate-y-1/2 -translate-x-1/2 leading-none'
    ]
    switch (type) {
      case GrabType.X:
        classList.push('top-1/2')
        break
      case GrabType.Y:
        classList.push('left-1/2')
        break
    }
    return classList.join(' ')
  }
  const dispatch = createEventDispatcher()
  const _mouseXY = {
    x: null,
    y: null
  }
  const mouseMove = e => {
    if (!grabbing || (_mouseXY.x === e.x && _mouseXY.y === e.y)) return
    document.body.classList.add('select-none')
    _mouseXY.x = e.x
    _mouseXY.y = e.y
    const rect = grabEl.getBoundingClientRect()
    left = percent ? ((e.x - rect.left) * 100) / rect.width : e.x - rect.left
    if (left < 0) left = 0
    else if (left > (percent ? 100 : rect.width))
      left = percent ? 100 : rect.width

    top = percent ? ((e.y - rect.top) * 100) / rect.height : e.y - rect.top
    if (top < 0) top = 0
    else if (top > (percent ? 100 : rect.height))
      top = percent ? 100 : rect.height
    requestAnimationFrame(() => {
      const pos: Grab.Pos = {
        left,
        top,
        width: rect.width,
        height: rect.height
      }
      dispatch('move', pos)
    })
  }
  const mouseUp = _e => {
    grabbing = false
    document.body.classList.remove('select-none')
  }
  const bgMouseDown = e => {
    grabbing = true
    mouseMove(e)
  }
  export { type, percent, left, top, grabbing, className as class }
</script>

<svelte:window
  on:mousemove={mouseMove}
  on:mouseup={mouseUp}
  class:cursor-grabbing={grabbing}
/>
<div
  class={`cursor-pointer relative ${className}`}
  bind:this={grabEl}
  on:mousedown={bgMouseDown}
>
  <slot />
  <span
    class={grabClass()}
    style:left={type === GrabType.Y ? '' : `${left}${unit}`}
    style:top={type === GrabType.X ? '' : `${top}${unit}`}
    class:cursor-grab={!grabbing}
    class:cursor-grabbing={grabbing}
    on:mousedown={() => (grabbing = true)}
  >
    <slot name="grab" />
  </span>
</div>
