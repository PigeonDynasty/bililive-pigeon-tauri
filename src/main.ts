import './style.css'
import App from './App.svelte'
import ripple from './utils/ripple'
const app = new App({
  target: document.getElementById('app')
})
ripple()
export default app
