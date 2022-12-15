<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import plugins, { pluginAppendAll, pluginClear } from '../store/plugin'
  import Switch from '../components/Switch.svelte'
  let headEl
  let bodyEl
  let colWidth = 0
  const resizeObserver = new ResizeObserver(_entries => {
    colWidth = headEl.offsetWidth - bodyEl.offsetWidth
  })
  let path = ''
  let data = []
  const unsubscribe = plugins.subscribe(value => {
    value.forEach(item => {
      const index = data.findIndex(ele => ele.path === item.path)
      if (index > -1) data[index] = item
    })
  })

  const loadPlugins = async () => {
    data = await invoke('load_plugin_all', { load: true })
    pluginAppendAll(data.filter(item => item.plugin_type === 'Js'))
  }

  const toggleChange = row => {
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
    invoke('update_plugin_visible', { path: row.path, visible: row.visible })
  }
  const reLoad = () => {
    pluginClear()
    loadPlugins()
  }
  onMount(async () => {
    resizeObserver.observe(bodyEl)
    path = await invoke('get_plugin_dir')
    loadPlugins()
  })
  onDestroy(() => {
    resizeObserver.unobserve(bodyEl)
    unsubscribe()
  })
</script>

<div class="h-full flex flex-col">
  <div class="my-1">
    目录：{path}
    <button
      class="btn-warning float-right text-xs leading-3 py-1 px-3 rounded-full"
      on:click={reLoad}>重载</button
    >
  </div>

  <div class="table-head">
    <table class="w-full" bind:this={headEl}>
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
    <table class="w-full" bind:this={bodyEl}>
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
              <Switch
                id={'plugin_' + i}
                class="align-middle my-1"
                bind:value={row.visible}
                on:change={e => toggleChange(row)}
              />
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
