<script lang="ts" context="module">
  export enum PopoverPlacement {
    BOTTOM,
    BOTTOM_LEFT,
    BOTTOM_RIGHT,
    TOP,
    TOP_LEFT,
    TOP_RIGHT,
    LEFT,
    LEFT_TOP,
    LEFT_BOTTOM,
    RIGHT,
    RIGHT_TOP,
    RIGHT_BOTTOM
  }
</script>

<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { fade } from 'svelte/transition'

  const dispatch = createEventDispatcher()
  let triggerEl
  let popoverEl
  let className: string = ''
  let style: string = ''
  let popoverClass: string = ''
  let placement: PopoverPlacement = PopoverPlacement.BOTTOM_LEFT
  let offsetLeft: number = 0
  let offsetTop: number = 0
  let gap: number = 2
  let trigger: 'click' | 'manual' | 'hover' = 'click'

  let visible: boolean = false
  let pos = {
    left: 0,
    top: 0
  }
  $: if (trigger === 'manual' && visible) {
    setPos()
    dispatch('toggle', visible)
  }

  const toggleOpen = _e => {
    if (trigger !== 'click') return
    visible = !visible
    setPos()
    dispatch('toggle', visible)
  }
  const mouseHover = bol => {
    if (trigger !== 'hover') return
    visible = bol
    setPos()
    dispatch('toggle', bol)
  }
  const setPos = () => {
    if (!triggerEl || !visible) return
    const rect = triggerEl.getBoundingClientRect()
    requestAnimationFrame(() => {
      switch (placement) {
        case PopoverPlacement.BOTTOM:
          pos.left = rect.left + rect.width / 2 - popoverEl.offsetWidth / 2
          pos.top = rect.bottom + gap
          break
        case PopoverPlacement.BOTTOM_LEFT:
          pos.left = rect.left
          pos.top = rect.bottom + gap
          break
        case PopoverPlacement.BOTTOM_RIGHT:
          pos.left = rect.right - popoverEl.offsetWidth
          pos.top = rect.bottom + gap
          break
        case PopoverPlacement.TOP:
          pos.left = rect.left + rect.width / 2 - popoverEl.offsetWidth / 2
          pos.top = rect.top - popoverEl.offsetHeight - gap
          break
        case PopoverPlacement.TOP_LEFT:
          pos.left = rect.left
          pos.top = rect.top - popoverEl.offsetHeight - gap
          break
        case PopoverPlacement.TOP_RIGHT:
          pos.left = rect.right - popoverEl.offsetWidth
          pos.top = rect.top - popoverEl.offsetHeight - gap
          break
        case PopoverPlacement.LEFT:
          pos.left = rect.left - popoverEl.offsetWidth - gap
          pos.top = rect.top + rect.height / 2 - popoverEl.offsetHeight / 2
          break
        case PopoverPlacement.LEFT_TOP:
          pos.left = rect.left - popoverEl.offsetWidth - gap
          pos.top = rect.top
          break
        case PopoverPlacement.LEFT_BOTTOM:
          pos.left = rect.left - popoverEl.offsetWidth - gap
          pos.top = rect.bottom - popoverEl.offsetHeight
          break
        case PopoverPlacement.RIGHT:
          pos.left = rect.right + gap
          pos.top = rect.top + rect.height / 2 - popoverEl.offsetHeight / 2
          break
        case PopoverPlacement.RIGHT_TOP:
          pos.left = rect.right + gap
          pos.top = rect.top
          break
        case PopoverPlacement.RIGHT_BOTTOM:
          pos.left = rect.right + gap
          pos.top = rect.bottom - popoverEl.offsetHeight
          break
      }
      pos.left += offsetLeft
      pos.top += offsetTop
    })
  }
  const toggleClose = () => {
    if (trigger !== 'click') return
    visible = false
    dispatch('toggle', false)
  }
  const windowClick = e => {
    if (visible && !triggerEl.contains(e.target)) {
      visible = false
      dispatch('toggle', false)
    }
  }
  export {
    className as class,
    style,
    popoverClass,
    placement,
    offsetLeft,
    offsetTop,
    trigger,
    visible,
    setPos
  }
</script>

<svelte:window on:click={windowClick} />
<div
  class={`inline-flex popover-trigger ${className}`}
  {style}
  bind:this={triggerEl}
  on:click={toggleOpen}
  on:mouseenter={() => mouseHover(true)}
  on:mouseleave={() => mouseHover(false)}
  on:keypress={() => {}}
>
  <slot name="trigger" />
</div>

{#if visible}
  <div
    class={`fixed rounded-md shadow-lg dark:shadow-zinc-900 p-2 bg-white dark:bg-black overflow-auto z-50 ${popoverClass}`}
    style:left={pos['left'] + 'px'}
    style:top={pos['top'] + 'px'}
    bind:this={popoverEl}
    transition:fade
    on:click|stopPropagation={() => toggleClose()}
    on:keypress={() => {}}
  >
    <slot />
  </div>
{/if}
