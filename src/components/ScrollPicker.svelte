<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte'
  let data = []
  let value
  let valueKey = 'value'
  let labelKey = 'label'
  let offsetIndex = 2
  let selectIndex = 0
  let ulEl, selectEl
  const dispatch = createEventDispatcher()
  const scrollSelect = () => {
    if (!ulEl) return
    requestAnimationFrame(() => {
      selectIndex = ulEl.scrollTop / selectEl.offsetHeight + offsetIndex - 1
      value =
        typeof data[selectIndex] === 'object'
          ? data[selectIndex][valueKey]
          : data[selectIndex]
      dispatch('select', data[selectIndex])
    })
  }
  onMount(() => {
    if (value)
      selectIndex = data.findIndex(
        item => (typeof item === 'object' ? item[valueKey] : item) === value
      )
  })
  export { value, valueKey, labelKey, offsetIndex }
</script>

<div class="relative w-full h-full">
  <div class="w-full h-full overflow-hidden">
    <ul
      class="snap-y snap-mandatory"
      bind:this={ulEl}
      on:scroll={() => {
        scrollSelect()
      }}
    >
      {#each data as item, i}
        <li
          class="flex items-center justify-center py-1 h-8 snap-start"
          class:text-slate-600={i === selectIndex}
          class:mt-16={i === 0}
          class:mb-16={i === data.length - 1}
        >
          {typeof item === 'object' ? item[labelKey] : item}
        </li>
      {/each}
    </ul>
  </div>
  <div
    bind:this={selectEl}
    class="absolute -translate-y-1/2 top-1/2 left-0 right-0 pointer-events-none border-y-slate-400 -z-10 h-8"
  />
</div>
