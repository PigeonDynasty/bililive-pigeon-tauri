<script context="module">
  export const TABS_KEY = Symbol()
</script>

<script lang="ts">
  import { setContext, onDestroy } from 'svelte'
  import { writable } from 'svelte/store'
  import { fade } from 'svelte/transition'
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
    console.log(current, key)
  }
</script>

<div class:tabs class={className}>
  <div class="tabs-header flex flex-nowrap">
    {#each tabs as { header, key } ('tab-header_' + key)}
      <span
        class="tab-header relative cursor-pointer py-1 px-2 border-b-2 border-transparent"
        on:click={() => selectTab(key)}
        >{header}
        {#if $current === key}
          <p
            transition:fade
            class="h-[2px] absolute bottom-0 left-0 right-0 bg-sky-400 dark:bg-sky-800"
          />
        {/if}
      </span>
    {/each}
  </div>
  <ul class="tabs-content">
    <slot />
  </ul>
</div>
