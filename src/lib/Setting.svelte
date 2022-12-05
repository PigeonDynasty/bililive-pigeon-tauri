<script lang="ts">
  import Toogle from './Toogle.svelte'
  import { WebviewWindow, currentMonitor } from '@tauri-apps/api/window'
  import ColorPicker from './ColorPicker.svelte'
  let web: WebviewWindow = null
  const toogleChange = async e => {
    const val = e.detail
    if (val && !web) {
      currentMonitor().then(monitor => {
        web = new WebviewWindow('sidewin', {
          url: '/side.html',
          // alwaysOnTop: true,
          // decorations: false,
          // resizable: false,
          title: '',
          // transparent: true,
          // x: monitor.size.width ,
          x: 0,
          y: monitor.size.height,
          height: monitor.size.height,
          width: 300
        })
        web.once('tauri://created', () => {
          setTimeout(() => {
            web && web.emit('add', '<p>test</p>')
          }, 5000)
        })
      })
    } else if (!val && web) {
      await web.close()
      web = null
    }
  }
  let color = ''
</script>

<Toogle id="toogle" on:change={toogleChange} />
<ColorPicker bind:value={color} />
