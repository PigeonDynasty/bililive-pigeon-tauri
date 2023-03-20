<script lang="ts" context="module">
  const COLOR_PICKER_KEY = Symbol()
  export { COLOR_PICKER_KEY }
</script>

<script lang="ts">
  import { setContext, onDestroy } from 'svelte'
  import { writable } from 'svelte/store'
  import Popover, { PopoverPlacement } from '../Popover.svelte'
  import EyeDropper from '@/icons/EyeDropper.svelte'
  import Hue from './Hue.svelte'
  import Sv from './Sv.svelte'
  import Alpha from './Alpha.svelte'
  import Input from '../Input.svelte'
  import { rgb2rgbString, hsv2rgb, rgb2hex, toRgb, rgb2hsv } from './color'
  let value: string = ''
  let alpha: boolean = false

  let visible: boolean = false
  let color: string = ''
  let rgb: Color.RGB = toRgb(value)
  let hsv = writable<Color.HSV>(rgb2hsv(rgb))
  let a = writable<number>(rgb.a)
  $: colorHsv = rgb2hsv(toRgb(value))
  setContext(COLOR_PICKER_KEY, {
    hsv,
    a
  })
  const unsubscribe_hsv = hsv.subscribe(value => {
    rgb = hsv2rgb(value)
    rgb.a = $a
    color = alpha ? rgb2rgbString(rgb) : rgb2hex(rgb)
  })
  const unsubscribe_a = a.subscribe(value => {
    rgb.a = value
    color = alpha ? rgb2rgbString(rgb) : rgb2hex(rgb)
  })
  const toggle = (bol: Boolean) => {
    if (!bol) return
    rgb = toRgb(value)
    a.set(rgb.a)
    hsv.set(rgb2hsv(rgb))
  }
  const clear = () => {
    value = ''
    visible = false
  }
  const confirm = () => {
    value = color
    visible = false
  }
  onDestroy(() => {
    unsubscribe_hsv()
    unsubscribe_a()
  })
  export { value, alpha }
</script>

<Popover
  placement={PopoverPlacement.RIGHT_BOTTOM}
  trigger="manual"
  bind:visible
  on:toggle={e => toggle(e.detail)}
>
  <div
    slot="trigger"
    class="shadow-md rounded-md alpha-bg overflow-hidden"
    class:text-slate-800={colorHsv.v >= 50}
    class:text-slate-100={colorHsv.v < 50}
    on:click={() => {
      visible = true
    }}
    on:keypress={() => {}}
  >
    <span
      class="inline-flex items-center cursor-pointer py-1 px-2 h-8"
      style:background-color={value}
    >
      <i class="w-4 h-4">
        <EyeDropper />
      </i>
    </span>
  </div>
  <div class="flex">
    <Sv />
    <Hue />
  </div>
  {#if alpha}
    <Alpha color={rgb} />
  {/if}
  <div class="flex justify-between items-center mt-2">
    <span class="inline-flex alpha-bg mr-2">
      <span class="inline-block h-6 w-6" style:background-color={color} />
    </span>
    <Input
      class="h-6 mr-2"
      bind:value={color}
      placeholder=""
      clearable={false}
    />
    <button
      class="hover:bg-zinc-100 dark:hover:bg-zinc-800 py-1 px-2 rounded-md text-xs mr-2"
      on:click={() => clear()}
    >
      清除
    </button>
    <button
      class="btn-sky py-1 px-2 rounded-md text-xs"
      on:click={() => confirm()}
    >
      确认
    </button>
  </div>
</Popover>

<style lang="postcss">
  :global(.alpha-bg) {
    background-image: linear-gradient(45deg, #d4d4d8 25%, transparent 25%),
      linear-gradient(135deg, #d4d4d8 25%, transparent 25%),
      linear-gradient(45deg, transparent 75%, #d4d4d8 75%),
      linear-gradient(135deg, transparent 75%, #d4d4d8 75%);
    background-size: 12px 12px;
    background-position: 0 0, 6px 0, 6px -6px, 0 6px;
  }
</style>
