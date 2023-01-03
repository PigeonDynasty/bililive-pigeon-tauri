<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  let className = ''
  let emptyText = ''
  let headEl
  let bodyEl
  let dataLen = 0
  const resizeObserver = new ResizeObserver(_entries => {
    const cols = headEl.querySelectorAll('col')
    if (cols.length > 0) {
      const bodyCols = bodyEl.querySelectorAll('col')
      cols[cols.length - 1].setAttribute(
        'width',
        bodyCols[bodyCols.length - 1].offsetWidth +
          headEl.offsetWidth -
          bodyEl.offsetWidth
      )
    }
    dataLen = bodyEl.querySelectorAll('tr').length
  })
  $: tableClass = ['data-table flex flex-col overflow-hidden', className].join(
    ' '
  )
  $: showEmpty = !$$slots.body || dataLen === 0
  onMount(async () => {
    resizeObserver.observe(bodyEl)
  })
  onDestroy(() => {
    resizeObserver.unobserve(bodyEl)
  })
  export { className as class }
</script>

<div class={tableClass}>
  <div class="table-head">
    <table class="w-full" bind:this={headEl}>
      <slot name="colgroup" />
      <slot name="head" />
    </table>
  </div>
  <div class="table-body overflow-auto">
    <table class="w-full" bind:this={bodyEl}>
      <slot name="colgroup" />
      <slot name="body" />
    </table>
    {#if showEmpty}
      <div class="empty-text text-center p-1">
        <slot name="emptyText" />
        {#if !$$slots.emptyText}
          {emptyText || '暂无数据'}
        {/if}
      </div>
    {/if}
  </div>
</div>
