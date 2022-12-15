<script lang="ts" context="module">
  export const TABS_KEY = Symbol()
</script>

<script lang="ts">
  import { setContext, onDestroy, createEventDispatcher } from 'svelte'
  import { writable } from 'svelte/store'
  const dispatch = createEventDispatcher()
  let className = ''
  let closeable = false
  $: tabsClass = ['tabs flex flex-col', className].join(' ')
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
    update: tab => {
      const index = tabs.findIndex(item => item.key === tab.key)
      if (index > -1) tabs[index].header = tab.header
    },
    current
  })
  const selectTab = (key: string | number) => {
    current.set(key)
  }
  const onClose = (key: string | number, index: number) => {
    if ($current === key) {
      selectTab(
        tabs[index + 1]
          ? tabs[index + 1].key
          : tabs[index - 1]
          ? tabs[index - 1].key
          : -1
      )
    }
    dispatch('close', { key, index })
  }

  export { className as class, closeable, selectTab }
</script>

<div class={tabsClass}>
  <div
    class="tabs-header flex space-x-1 rounded-lg bg-slate-100 dark:bg-slate-800 shadow-md"
    class:p-1={tabs.length > 0}
  >
    {#each tabs as { header, key }, index ('tab-header_' + key)}
      <button
        class="tab-header cursor-pointer py-1 px-3 rounded-md text-sm font-semibold whitespace-nowrap flex items-center"
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
        {#if closeable}
          <button
            class="w-3 h-3 ml-1 rounded-full hover:bg-slate-200 dark:hover:bg-slate-700"
            on:click={() => onClose(key, index)}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="2"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        {/if}
      </button>
    {/each}
  </div>
  <ul class="tabs-content flex-1 overflow-auto">
    <slot />
  </ul>
</div>
