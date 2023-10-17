<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  const dispatch = createEventDispatcher()
  let id = ''
  let value = false
  let className = ''
  let disabled = false
  $: labelClass = ['inline-flex w-14', className].join(' ')
  const change = () => {
    dispatch('change', value)
  }
  export { className as class, id, value, disabled }
</script>

<label class={labelClass} for={id}>
  <input
    class="sr-only"
    type="checkbox"
    {id}
    {disabled}
    bind:checked={value}
    on:change={change}
  />
  <div
    class="h-6 w-full bg-zinc-200 dark:bg-zinc-700 rounded-full shadow p-0.5"
    class:cursor-pointer={!disabled}
    class:cursor-not-allowed={disabled}
  >
    <span
      class="h-5 w-5 rounded-full bg-white dark:bg-zinc-400 shadow-sm inline-block"
    />
  </div>
</label>

<style lang="postcss">
  input:checked + div {
    @apply bg-sky-400 dark:bg-sky-800;
  }
  input:checked + div > span {
    @apply translate-x-8;
  }
</style>
