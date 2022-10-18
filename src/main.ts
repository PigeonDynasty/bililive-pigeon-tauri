import './style.css'
import App from './App.svelte'
import ripple from './utils/ripple'
import { plugins } from './utils/plugin'
const app = new App({
  target: document.getElementById('app')
})
ripple()
plugins.appendAll([
  {
    path: '/Users/takenokos/Documents/BililivePigeon/plugins/test_plugin.js',
    visible: true
  }
])
export default app
