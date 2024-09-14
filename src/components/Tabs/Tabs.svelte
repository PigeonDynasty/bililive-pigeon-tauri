<script lang="ts" context="module">
  export const TABS_KEY = Symbol()
  export enum HEADER_TYPES {
    button,
    link
  }
</script>

<script lang="ts">
  import XMark from '@/icons/XMark.svelte'
  import { setContext, onDestroy, createEventDispatcher } from 'svelte'
  import { writable } from 'svelte/store'
  const dispatch = createEventDispatcher()
  let className = ''
  let closeable = false
  let headerType: HEADER_TYPES = HEADER_TYPES.button
  $: tabsClass = ['tabs flex flex-col', className].join(' ')
  interface Tab {
    header: string
    key: string | number
  }
  let tabs: Tab[] = []
  const current = writable<number | string>(-1)
  setContext(TABS_KEY, {
    register: (tab: Tab) => {
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
    update: (tab: Tab) => {
      const index = tabs.findIndex(item => item.key === tab.key)
      if (index > -1) tabs[index].header = tab.header
    },
    current
  })
  const selectTab = (key: string | number) => {
    current.set(key)
    dispatch('change', key)
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

  export { className as class, closeable, headerType, selectTab }
</script>

<div class={tabsClass}>
  <div
    class="tabs-header flex space-x-1 rounded-lg overflow-x-auto"
    class:p-1={tabs.length > 0}
    class:bg-slate-100={headerType === HEADER_TYPES.button}
    class:dark:bg-slate-800={headerType === HEADER_TYPES.button}
    class:dark:shadow-md={headerType === HEADER_TYPES.button}
    class:divide-x={headerType === HEADER_TYPES.link}
    class:divide-zinc-200={headerType === HEADER_TYPES.link}
  >
    {#each tabs as { header, key }, index ('tab-header_' + key)}
      <button
        class="tab-header cursor-pointer py-1 px-3 text-sm font-semibold whitespace-nowrap flex items-center flex-shrink-0"
        class:rounded-md={headerType === HEADER_TYPES.button}
        class:text-slate-600={$current !== key}
        class:dark:text-slate-300={$current !== key}
        class:text-slate-800={headerType === HEADER_TYPES.button &&
          $current === key}
        class:dark:text-slate-100={headerType === HEADER_TYPES.button &&
          $current === key}
        class:shadow={headerType === HEADER_TYPES.button && $current === key}
        class:bg-white={headerType === HEADER_TYPES.button && $current === key}
        class:dark:bg-black={headerType === HEADER_TYPES.button &&
          $current === key}
        class:text-sky-400={headerType === HEADER_TYPES.link &&
          $current === key}
        class:dark:text-sky-600={headerType === HEADER_TYPES.link &&
          $current === key}
        on:click|stopPropagation={() => selectTab(key)}
      >
        {header}
        {#if closeable}
          <button
            class="w-3 h-3 ml-1 rounded-full hover:bg-slate-200 dark:hover:bg-slate-700"
            on:click|stopPropagation={() => onClose(key, index)}
          >
            <XMark />
          </button>
        {/if}
      </button>
    {/each}
  </div>
  <ul class="tabs-content flex-1 overflow-auto">
    <slot />
  </ul>
</div>
