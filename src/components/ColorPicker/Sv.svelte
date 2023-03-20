<script lang="ts">
  import { getContext, onDestroy } from 'svelte'

  import Grab from '../Template/Grab.svelte'
  import { COLOR_PICKER_KEY } from './ColorPicker.svelte'
  const { hsv }: Color.Context = getContext(COLOR_PICKER_KEY)
  const move = ({ left, top }: Grab.Pos) => {
    hsv.update(value => {
      value.s = left
      value.v = 100 - top
      return value
    })
  }
</script>

<Grab
  class="mr-2"
  percent
  left={$hsv.s}
  top={100 - $hsv.v}
  on:move={e => move(e.detail)}
>
  <div
    class="relative w-72 h-48 "
    style:background={`hsl(${$hsv.h * 3.6}, 100%, 50%)`}
  >
    <div
      class="absolute left-0 top-0 right-0 bottom-0 bg-gradient-to-r from-white"
    />
    <div
      class="absolute left-0 top-0 right-0 bottom-0 bg-gradient-to-t from-black"
    />
  </div>
  <span slot="grab" class="inline-block w-2 h-2 shadow border rounded" />
</Grab>
