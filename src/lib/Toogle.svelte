<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  const dispatch = createEventDispatcher()
  export let id = ''
  export let value = false
  const change = () => {
    dispatch('change', value)
  }
</script>

<label class="inline-flex w-14" for={id}>
  <input
    class="sr-only"
    type="checkbox"
    {id}
    bind:checked={value}
    on:change={change}
  />
  <div
    class="h-6 w-full cursor-pointer bg-zinc-200 dark:bg-zinc-700 rounded-full shadow p-0.5"
  >
    <span
      class="h-5 w-5 rounded-full bg-white dark:bg-zinc-400 shadow-sm inline-block transition-all"
    />
  </div>
</label>

<style scoped>
  input:checked + div {
    @apply bg-sky-400 dark:bg-sky-800;
  }
  input:checked + div > span {
    @apply translate-x-8;
  }
</style>
