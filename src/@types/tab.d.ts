import type { Writable } from 'svelte/store'
namespace Tab {
  interface Context {
    register: Function
    update: Function
    current: Writable<number | string>
  }
}
