<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { fade } from 'svelte/transition'
  import portal from '@/utils/portal'
  import XMark from '@/icons/XMark.svelte'
  const dispatch = createEventDispatcher()
  let visible: boolean = false
  let className: string = ''
  let title: string = ''
  $: if (visible) {
    dispatch('open')
  } else {
    dispatch('close')
  }
  export { visible, className as class, title }
</script>

{#if visible}
  <div
    class="fixed left-0 top-0 right-0 bottom-0 bg-zinc-950/50 flex justify-center items-center transition-all z-"
    transition:fade
    use:portal
    on:click={() => (visible = false)}
    on:keypress={() => (visible = false)}
  >
    <div
      class={`flex items-center justify-center flex-col shadow-md rounded-md w-1/2 px-4 py-2 bg-white dark:bg-black select-none ${className}`}
    >
      <div class="dialog-header relative clear-both w-full pb-2">
        <slot name="header">
          {title}
          <span
            class="w-6 h-6 float-right cursor-pointer rounded-full hover:bg-zinc-100 p-0.5"
            on:click={() => (visible = false)}
            on:keypress={() => {}}
          >
            <XMark />
          </span>
        </slot>
      </div>
      <slot />
      {#if !$$slots.footer}
        <div class="dialog-footer">
          <slot name="footer" />
        </div>
      {/if}
    </div>
  </div>
{/if}
