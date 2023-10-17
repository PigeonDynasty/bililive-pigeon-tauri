import { writable } from 'svelte/store'
const account = writable({
  cookie: ''
})
export default account
