<script lang="ts">
  import { onMount, getContext } from 'svelte'
  import ChevronHeader from './ChevronHeader.svelte'
  import { DATEPICKER_KEY, DatePickerSelectType } from './DatePicker.svelte'
  import dayjs, { Dayjs } from 'dayjs'
  const { props, time, select }: any = getContext(DATEPICKER_KEY)
  const currentDate = dayjs()
  let dateArray = [] // 渲染的日期列表
  const toSelect = (view: DatePickerSelectType) => {
    props.update(value => {
      value.view = view
      return value
    })
  }
  const selectDate = (date: Dayjs) => {
    props.update(value => {
      if (time) {
        value.date = value.date
          ? value.date.year(date.year()).month(date.month()).date(date.date())
          : date
      } else {
        value.date = date
      }
      if (!date.isSame(value.showDate, 'month')) {
        value.showDate = date
        initDateArray()
      }
      return value
    })
    select(date)
  }
  const updateShowDate = (op: 'add' | 'subtract', type: 'year' | 'month') => {
    props.update(value => {
      value.showDate =
        op === 'add'
          ? value.showDate.add(1, type)
          : value.showDate.subtract(1, type)
      return value
    })
    initDateArray()
  }
  const initDateArray = () => {
    const showYear = $props.showDate.year()
    const showMonth = $props.showDate.month()
    // 当前月第一天
    const startDate = dayjs(`${showYear}-${showMonth + 1}-01`).startOf('M')
    // 当月第一天对应那一周的第一天
    const startDay = startDate.startOf('w')
    // 当前月最后一天
    const endDate = dayjs(`${showYear}-${showMonth + 1}-01`).endOf('M')
    // 当前月最后一天对应那一周的最后一天
    const endDay = endDate.endOf('w')
    let inDate = startDay.clone()
    const array = []
    while (!inDate.isAfter(endDay, 'date')) {
      array.push(inDate)
      inDate = inDate.add(1, 'day')
    }
    dateArray = array
  }
  onMount(() => {
    initDateArray()
  })
</script>

<ChevronHeader
  withSingle
  on:db-left={() => updateShowDate('subtract', 'year')}
  on:db-right={() => updateShowDate('add', 'year')}
  on:left={() => updateShowDate('subtract', 'month')}
  on:right={() => updateShowDate('add', 'month')}
>
  <span
    class="link"
    on:click={() => toSelect(DatePickerSelectType.YEAR)}
    on:keypress={() => {}}
  >
    {$props.showDate.year()}年
  </span>
  <span
    class="link ml-2"
    on:click={() => toSelect(DatePickerSelectType.MONTH)}
    on:keypress={() => {}}
  >
    {$props.showDate.month() + 1}月
  </span>
</ChevronHeader>
<div class="border-b text-slate-400 flex justify-around text-sm my-1 pb-1">
  <span>日</span>
  <span>一</span>
  <span>二</span>
  <span>三</span>
  <span>四</span>
  <span>五</span>
  <span>六</span>
</div>
<div class="flex flex-wrap text-center">
  {#each dateArray as date}
    <span
      class="link inline-block w-6 h-6 leading-6 text-xs m-1 rounded-full"
      class:cursor-default={!date.isSame($props.showDate, 'month')}
      class:text-slate-400={!date.isSame($props.showDate, 'month')}
      class:font-bold={date.isSame(currentDate, 'day')}
      class:text-sky-400={date.isSame(currentDate, 'day')}
      class:dark:text-sky-800={date.isSame(currentDate, 'day')}
      class:text-white={$props.date && date.isSame($props.date, 'day')}
      class:hover:text-white={$props.date && date.isSame($props.date, 'day')}
      class:bg-sky-400={$props.date && date.isSame($props.date, 'day')}
      class:dark:bg-sky-800={$props.date && date.isSame($props.date, 'day')}
      on:click={() => selectDate(date)}
      on:keypress={() => {}}
    >
      {date.date()}
    </span>
  {/each}
</div>
