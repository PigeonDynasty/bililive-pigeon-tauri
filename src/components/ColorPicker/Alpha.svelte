<script lang="ts">
  import { getContext } from 'svelte'
  import Grab, { GrabType } from '../Grab.svelte'
  import { COLOR_PICKER_KEY } from './ColorPicker.svelte'
  const { a }: Color.Context = getContext(COLOR_PICKER_KEY)
  import { rgb2rgbString } from './color'
  let color: Color.RGB
  $: bg = `linear-gradient(to right, ${rgb2rgbString({ ...color, a: 0 })},
  ${rgb2rgbString({ ...color, a: 1 })})`
  const move = ({ left }: Grab.Pos) => {
    a.update(_value => {
      return Math.round(left) / 100
    })
  }
  export { color }
</script>

<Grab
  class="mt-2 alpha-bg w-72"
  type={GrabType.X}
  percent
  left={$a * 100}
  on:move={e => move(e.detail)}
>
  <div class="h-3" style:background={bg} />
  <div slot="grab" class="h-3 w-1.5 shadow-sm rounded-sm border bg-white" />
</Grab>
