import { writable } from 'svelte/store'
const account = writable({
  cookie: ''
})

const updateCookie = (val: string) => {
  account.update(value => {
    value.cookie = val
    return value
  })
}
export default account
export { updateCookie }