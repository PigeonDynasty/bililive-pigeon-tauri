<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import XCircle from '@/icons/XCircle.svelte'
  let inputEl
  let value
  let type = 'text'
  let placeholder = ''
  let readonly = false
  let disabled = false
  let clearable = true
  let className = ''
  $: inputClass = [
    'input inline-flex relative items-center flex-1',
    className
  ].join(' ')
  $: inputEl && (inputEl.type = type)
  const dispatch = createEventDispatcher()
  const clear = () => {
    value = null
    dispatch('clear')
  }
  export {
    value,
    type,
    placeholder,
    readonly,
    disabled,
    clearable,
    className as class
  }
</script>

<div
  class={inputClass}
  class:pl-2={$$slots.prefixIcon}
  class:pr-6={clearable}
  on:click={() => dispatch('click', value)}
  on:keypress={() => {}}
>
  {#if $$slots.prefixIcon}
    <i class="w-4 h-4 mr-1 text-slate-400">
      <slot name="prefixIcon" />
    </i>
  {/if}
  {#if type === 'textarea'}
    <textarea
      class="outline-none dark:bg-black bg-white w-full"
      {placeholder}
      {readonly}
      {disabled}
      bind:value
      on:input={() => dispatch('input', value)}
      on:focus={() => dispatch('focus', value)}
      on:blur={() => dispatch('blur', value)}
      on:change={() => dispatch('change', value)}
    />
  {:else}
    <input
      class="outline-none dark:bg-black bg-white w-full h-6"
      {placeholder}
      {readonly}
      {disabled}
      bind:this={inputEl}
      bind:value
      on:input={() => dispatch('input', value)}
      on:focus={() => dispatch('focus', value)}
      on:blur={() => dispatch('blur', value)}
      on:change={() => dispatch('change', value)}
    />
  {/if}
  {#if !disabled && !readonly && clearable && value}
    <i
      class="input-clear w-4 h-4 absolute right-2 top-2 text-slate-400 cursor-pointer transition hidden"
      on:click|stopPropagation={() => {
        clear()
      }}
      on:keypress={() => {}}
    >
      <XCircle />
    </i>
  {/if}
</div>

<style lang="postcss">
  .input:hover .input-clear {
    @apply inline-block;
  }
</style>
