<script lang="ts">
  import Toggle from './Toogle.svelte'
  import { WebviewWindow, currentMonitor } from '@tauri-apps/api/window'
  let toogleVal = false
  let web = null
  $: {
    if (toogleVal && !web) {
      currentMonitor().then(monitor => {
        web = new WebviewWindow('sidewin', {
          url: '/side.html',
          // alwaysOnTop: true,
          // decorations: false,
          // resizable: false,
          title: '',
          // transparent: true,
          x: monitor.size.width,
          y: monitor.size.height,
          width: 300
        })
        web.once('tauri://created', function () {
          setTimeout(() => {
            web.emit('add', '<p>test</p>')
          }, 10000)
        })
      })
    } else if (!toogleVal && web) {
      console.log('before', web)
      web.close()
      console.log('after', web)
      web = null
    }
  }
</script>

<Toggle id="toggle" bind:value={toogleVal} />
