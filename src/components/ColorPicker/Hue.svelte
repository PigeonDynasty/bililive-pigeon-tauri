<script lang="ts">
  import { getContext } from 'svelte'
  import Grab, { GrabType } from '../Template/Grab.svelte'
  import { COLOR_PICKER_KEY } from './ColorPicker.svelte'
  const { hsv }: Color.Context = getContext(COLOR_PICKER_KEY)

  const move = ({ top }: Grab.Pos) => {
    hsv.update(value => {
      value.h = top
      return value
    })
  }
</script>

<Grab type={GrabType.Y} percent top={$hsv.h} on:move={e => move(e.detail)}>
  <div class="hue w-3 h-full" />
  <div slot="grab" class="w-3 h-1.5 shadow-sm rounded-sm border bg-white" />
</Grab>

<style lang="postcss">
  .hue {
    background: linear-gradient(
      to bottom,
      #f00 0%,
      #ff0 17%,
      #0f0 33%,
      #0ff 50%,
      #00f 67%,
      #f0f 83%,
      #f00 100%
    );
  }
</style>
