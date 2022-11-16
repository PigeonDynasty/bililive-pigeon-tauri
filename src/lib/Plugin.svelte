<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import plugins from '../utils/plugin'
  import Toogle from './Toogle.svelte'
  let headRef
  let bodyRef
  let colWidth = 0
  const resizeObserver = new ResizeObserver(entries => {
    colWidth = headRef.offsetWidth - bodyRef.offsetWidth
  })

  let data = []
  onMount(async () => {
    resizeObserver.observe(bodyRef)

    data = await invoke('load_plugin_all', { load: true })
    plugins.appendAll(data.filter(item => item.plugin_type === 'Js'))
    plugins.data.subscribe(value => {
      value.forEach(item => {
        const index = data.findIndex(ele => ele.path === item.path)
        if (index > -1) data[index] = item
      })
    })
  })
  onDestroy(() => {
    resizeObserver.unobserve(bodyRef)
  })
  const toogleChange = row => {
    switch (row.plugin_type) {
      case 'Js':
        row.visible ? row.load() : row.unload()
        break
      case 'Dylib':
        row.visible
          ? invoke('load_plugin', { name: row.name })
          : invoke('unload_plugin', { name: row.name })
        break
    }
  }
</script>

<div class="h-full flex flex-col">
  <div class="table-head">
    <table class="w-full" bind:this={headRef}>
      <colgroup>
        <col name="col_1" width="120" />
        <col name="col_2" width="120" />
        <col name="col_3" />
        <col name="col_4" width="140" />
        <col name="col_5" width={60 + colWidth} />
      </colgroup>
      <thead>
        <tr>
          <th>名称</th>
          <th>作者</th>
          <th>说明</th>
          <th>联系方式</th>
          <th>启用</th>
        </tr>
      </thead>
    </table>
  </div>
  <div class="table-body overflow-auto">
    <table class="w-full" bind:this={bodyRef}>
      <colgroup>
        <col name="col_1" width="120" />
        <col name="col_2" width="120" />
        <col name="col_3" />
        <col name="col_4" width="140" />
        <col name="col_5" width="60" />
      </colgroup>
      <tbody>
        {#each data as row, i}
          <tr
            class:bg-neutral-100={i % 2 === 0}
            class:dark:bg-neutral-800={i % 2 === 0}
          >
            <td>{row.name}</td>
            <td>{row.author}</td>
            <td>{row.description}</td>
            <td>{row.contact}</td>
            <td>
              <Toogle
                id={'plugin_' + i}
                class="align-middle my-1"
                bind:value={row.visible}
                on:change={e => toogleChange(row)}
              />
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
