<script context="module">
  export const TABS_KEY = Symbol()
</script>

<script lang="ts">
  import { setContext, onDestroy } from 'svelte'
  import { writable } from 'svelte/store'
  let className = ''
  export { className as class }
  let tabs = []
  const current = writable<number | string>(-1)
  setContext(TABS_KEY, {
    register: tab => {
      let key = tab.key || tabs.length
      tabs = [
        ...tabs,
        {
          header: tab.header,
          key: key
        }
      ]
      if (tabs.length === 1) selectTab(key)
      onDestroy(() => {
        const i = tabs.findIndex(item => item.key === key)
        tabs.splice(i, 1)
        tabs = tabs
      })
      return Promise.resolve(key)
    },
    current
  })
  const selectTab = key => {
    current.set(key)
  }
</script>

<div class:tabs class={className}>
  <div
    class="tabs-header flex space-x-1 rounded-lg bg-slate-100 dark:bg-slate-800 p-1"
  >
    {#each tabs as { header, key } ('tab-header_' + key)}
      <button
        class="tab-header cursor-pointer py-1 px-3 rounded-md text-sm font-semibold"
        class:text-slate-600={$current !== key}
        class:dark:text-slate-300={$current !== key}
        class:text-slate-800={$current === key}
        class:dark:text-slate-100={$current === key}
        class:shadow={$current === key}
        class:bg-white={$current === key}
        class:dark:bg-black={$current === key}
        on:click={() => selectTab(key)}
      >
        {header}
      </button>
    {/each}
  </div>
  <ul class="tabs-content">
    <slot />
  </ul>
</div>
