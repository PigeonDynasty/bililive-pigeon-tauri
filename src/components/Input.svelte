<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import XCircle from '@/icons/XCircle.svelte'
  let value
  let placeholder = ''
  let readonly = false
  let clearable = true
  const dispatch = createEventDispatcher()
  const clear = () => {
    value = null
    dispatch('clear')
  }
  export { value, placeholder, readonly, clearable }
</script>

<div
  class="input inline-flex relative items-center flex-1"
  class:pl-2={$$slots.prefixIcon}
  class:pr-6={clearable}
>
  {#if $$slots.prefixIcon}
    <i class="w-4 h-4 mr-1 text-slate-400">
      <slot name="prefixIcon" />
    </i>
  {/if}
  <input class="outline-none" {placeholder} {readonly} bind:value />
  {#if clearable && value}
    <i
      class="input-clear w-4 h-4 absolute right-2 top-2 text-slate-400 cursor-pointer transition hidden"
      on:click|stopPropagation={() => {
        clear()
      }}
    >
      <XCircle />
    </i>
  {/if}
</div>

<style scoped>
  .input:hover .input-clear {
    @apply inline-block;
  }
</style>
