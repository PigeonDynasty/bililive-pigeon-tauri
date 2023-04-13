<script lang="ts">
  import Swatch from '@/icons/Swatch.svelte'
  import { createEventDispatcher } from 'svelte'
  import Popover from './Popover.svelte'
  const dispatch = createEventDispatcher()
  let colorEl
  let value: string = ''
  let className: string = ''
  let colors: string[] = [
    'slate',
    'gray',
    'zinc',
    'neutral',
    'stone',
    'red',
    'orange',
    'amber',
    'yellow',
    'lime',
    'green',
    'emerald',
    'teal',
    'cyan',
    'sky',
    'blue',
    'indigo',
    'violet',
    'purple',
    'fuchsia',
    'pink',
    'rose'
  ]
  let variants: number[] = [
    50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950
  ]
  $: textColor = () => {
    let arr = value.split('-')
    if (arr.length > 1) {
      return Number(arr[1]) < 500 ? 'text-slate-800' : 'text-slate-100'
    }
    return ''
  }
  const selectColor = (color, variant) => {
    value = color + '-' + variant
    requestAnimationFrame(() => {
      dispatch('change', value, getColor())
    })
  }
  const clear = () => {
    value = ''
    requestAnimationFrame(() => {
      dispatch('change', value, getColor())
    })
  }
  const getColor = () => {
    return colorEl ? getComputedStyle(colorEl)['background-color'] : ''
  }
  export { className as class, value, getColor }
</script>

<Popover>
  <div
    slot="trigger"
    class={`inline-flex items-center cursor-pointer py-1 px-2 h-8 shadow-md rounded-md text-sm whitespace-nowrap bg-${value} ${textColor()} ${className}`}
    bind:this={colorEl}
  >
    <i class="w-4 h-4" class:mr-1={!!value}>
      <Swatch />
    </i>
    {value}
  </div>

  <div class="color-select_panel overflow-auto h-48">
    {#each colors as color}
      <div class="flex">
        {#each variants as variant}
          <div
            class={`cursor-pointer w-6 h-6 rounded-full m-1 bg-${color}-${variant}`}
            title={`${color}-${variant}`}
            on:click={() => selectColor(color, variant)}
            on:keypress={() => {}}
          />
        {/each}
      </div>
    {/each}
  </div>
  <div class="text-right mt-2">
    <button
      class="hover:bg-zinc-100 dark:hover:bg-zinc-800 py-1 px-2 rounded-md text-xs"
      on:click={() => clear()}>清除</button
    >
  </div>
</Popover>
