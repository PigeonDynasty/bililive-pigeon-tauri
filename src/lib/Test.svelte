<script lang="ts">
  import Slider from '@/components/Slider.svelte'
  import ColorSelect from '@/components/ColorSelect.svelte'
  import DatePicker from '@/components/DatePicker/DatePicker.svelte'
  import TimePicker from '@/components/TimePicker.svelte'
  import ColorPicker from '@/components/ColorPicker/ColorPicker.svelte'
  let date = ''
  let time = ''
  let datetime
  let slider = 50
  let color
  let color2 = '#f00'
  let color3 = 'rgba(255,0,0,.5)'
</script>

<h2 class="my-2">测试</h2>
<div class="flex items-center text-sm mb-2">
  <span class="w-28 mr-2">Slider</span>
  <Slider class="flex-1" bind:value={slider} />
</div>
<div class="flex items-center text-sm mb-2">
  <span class="w-28 mr-2">DatePicker</span>
  <DatePicker bind:value={date} />
</div>
<div class="flex items-center text-sm mb-2">
  <span class="w-28 mr-2">TimePicker</span>
  <TimePicker bind:value={time} />
</div>
<div class="flex items-center text-sm mb-2">
  <span class="w-28 mr-2">DateTimePicker</span>
  <DatePicker bind:value={datetime} time />
</div>
<div class="flex items-center text-sm mb-2">
  <span class="w-28 mr-2">ColorSelect</span>
  <ColorSelect bind:value={color} />
</div>
<div class="flex items-center text-sm mb-2">
  <span class="w-28 mr-2">ColorPicker</span>
  <ColorPicker bind:value={color2} />
</div>
<div class="flex items-center text-sm mb-2">
  <span class="w-28 mr-2">ColorPickerAlpha</span>
  <ColorPicker bind:value={color3} alpha />
</div>
