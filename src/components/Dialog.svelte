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
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="fixed left-0 top-0 right-0 bottom-0 bg-zinc-950/50 flex justify-center items-center transition-all"
    transition:fade
    use:portal
    on:click|self={() => (visible = false)}
    on:keypress|self={() => (visible = false)}
  >
    <div
      class={`flex items-center justify-center flex-col shadow-md rounded-md w-1/2 px-4 py-2 bg-white dark:bg-black select-none ${className}`}
    >
      <div class="dialog-header relative clear-both w-full pb-2 text-base">
        <slot name="header">
          {title}
          <button
            class="w-6 h-6 float-right cursor-pointer rounded-full hover:bg-zinc-100 p-0.5"
            on:click={() => (visible = false)}
          >
            <XMark />
          </button>
        </slot>
      </div>
      <div class="dialog-body w-full">
        <slot />
      </div>
      {#if $$slots.footer}
        <div class="dialog-footer text-right pt-2 w-full">
          <slot name="footer" />
        </div>
      {/if}
    </div>
  </div>
{/if}
