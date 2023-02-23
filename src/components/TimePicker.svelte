<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import Popover from './Popover.svelte'
  import ScrollPicker from './ScrollPicker.svelte'
  import Input from './Input.svelte'
  import dayjs, { Dayjs } from 'dayjs'
  import Clock from '@/icons/Clock.svelte'
  let value: string | Dayjs | Date | number
  let className: string = ''
  const dispatch = createEventDispatcher()
  let time = []
  $: showTime = () => time.join(':')
  let visible = false
  const fixTimeNumber = num =>
    Number(num) < 10 ? '0' + Number(num) : String(num)
  // 数据源
  let timeOptions = [[], [], []]
  for (let i = 0; i < 60; i++) {
    const num = fixTimeNumber(i)
    if (i < 24) timeOptions[0].push(num)
    timeOptions[1].push(num)
    timeOptions[2].push(num)
  }
  // end 数据源
  const toggle = bol => {
    if (!bol) return
    if (value && typeof value === 'string') {
      value.split(':').forEach((num, i) => {
        time[i] = fixTimeNumber(num)
      })
    } else {
      const inTime = value ? dayjs(value) : dayjs()
      time[0] = fixTimeNumber(inTime.hour())
      time[1] = fixTimeNumber(inTime.minute())
      time[2] = fixTimeNumber(inTime.second())
    }
    select()
  }
  const clear = () => {
    time = []
    value = undefined
  }
  const select = () => {
    if (typeof value === 'string') {
      value = showTime()
    } else if (typeof value === 'number') {
      const outTime = dayjs(value)
        .hour(Number(time[0]))
        .minute(Number(time[1]))
        .second(Number(time[2]))
      value = String(value).length === 10 ? outTime.unix() : outTime.valueOf()
    } else if (value instanceof Date) {
      value = dayjs(value)
        .hour(Number(time[0]))
        .minute(Number(time[1]))
        .second(Number(time[2]))
        .toDate()
    } else if (dayjs.isDayjs(value)) {
      value = value
        .hour(Number(time[0]))
        .minute(Number(time[1]))
        .second(Number(time[2]))
    } else {
      value = dayjs(value)
    }
    dispatch('change', value)
  }
  export { value, className as class }
</script>

<Popover
  bind:visible
  trigger="manual"
  class={className}
  popoverClass="w-52"
  on:toggle={e => toggle(e.detail)}
>
  <Input
    slot="trigger"
    value={showTime()}
    placeholder="请选择时间"
    readonly
    on:click={() => (visible = !visible)}
    on:clear={() => clear()}
  >
    <Clock slot="prefixIcon" />
  </Input>
  <div class="flex -mx-2 text-xs">
    <ScrollPicker
      bind:value={time[0]}
      data={timeOptions[0]}
      on:select={select}
    />
    <ScrollPicker
      bind:value={time[1]}
      data={timeOptions[1]}
      on:select={select}
    />
    <ScrollPicker
      bind:value={time[2]}
      data={timeOptions[2]}
      on:select={select}
    />
  </div>
</Popover>
