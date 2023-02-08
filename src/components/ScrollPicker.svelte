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
      const index = ulEl.scrollTop / selectEl.offsetHeight
      selectIndex = Math.round(index)
      value =
        typeof data[selectIndex] === 'object'
          ? data[selectIndex][valueKey]
          : data[selectIndex]
      if (selectIndex === index) dispatch('select', data[selectIndex])
    })
  }
  const click = i => {
    if (i === selectIndex) return
    selectIndex = i
    ulEl.scrollTop = selectIndex * selectEl.offsetHeight
    value =
      typeof data[selectIndex] === 'object'
        ? data[selectIndex][valueKey]
        : data[selectIndex]
    dispatch('select', data[selectIndex])
  }
  onMount(() => {
    if (!value) return
    selectIndex = data.findIndex(
      item => (typeof item === 'object' ? item[valueKey] : item) === value
    )
    ulEl.scrollTop = selectIndex * selectEl.offsetHeight
  })
  export { data, value, valueKey, labelKey, offsetIndex }
</script>

<div class="relative w-full h-full">
  <ul
    class="w-full h-full overflow-auto snap-y snap-mandatory"
    bind:this={ulEl}
    on:scroll={() => {
      scrollSelect()
    }}
  >
    <!-- 头部补足 -->
    {#each Array(offsetIndex) as _}
      <li class="h-8 snap-start" />
    {/each}
    <!-- 头部补足 -->
    {#each data as item, i}
      <li
        class="flex items-center justify-center py-1 h-8 snap-start "
        class:cursor-pointer={i !== selectIndex}
        class:text-slate-400={i !== selectIndex}
        class:text-slate-800={i === selectIndex}
        on:click={() => click(i)}
      >
        {typeof item === 'object' ? item[labelKey] : item}
      </li>
    {/each}
    <!-- 尾部补足 -->
    {#each Array(offsetIndex) as _}
      <li class="h-8 snap-start" />
    {/each}
    <!-- 尾部补足 -->
  </ul>
  <div
    bind:this={selectEl}
    class="absolute -translate-y-1/2 top-1/2 left-0 right-0 pointer-events-none border-y-slate-300 -z-10 h-8 border-y"
  />
</div>
