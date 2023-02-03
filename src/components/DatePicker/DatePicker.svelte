<script lang="ts" context="module">
  import { writable } from 'svelte/store'
  import dayjs, { Dayjs } from 'dayjs'
  const DATEPICKER_KEY = Symbol()
  enum DatePickerSelectType {
    DATE,
    MONTH,
    YEAR
  }
  export { DATEPICKER_KEY, DatePickerSelectType }
</script>

<script lang="ts">
  import { setContext, onDestroy, createEventDispatcher } from 'svelte'
  import Popover from '../Popover.svelte'
  import TableDate from './TableDate.svelte'
  import TableMonth from './TableMonth.svelte'
  import TableYear from './TableYear.svelte'
  import XCircle from '@/icons/XCircle.svelte'
  import CalendarDays from '@/icons/CalendarDays.svelte'
  export let value: undefined | string | Dayjs | Date
  let visible: boolean = false
  const props = writable({
    view: DatePickerSelectType.DATE,
    date: value ? dayjs(value) : undefined,
    showDate: dayjs()
  })
  const dispatch = createEventDispatcher()
  setContext(DATEPICKER_KEY, {
    props,
    select: (date: Dayjs) => {
      visible = false
      value = date
      dispatch('select', date)
    }
  })
  const clear = () => {
    props.update(value => {
      value.date = undefined
      return value
    })
    value = undefined
  }
  let component = null
  const unsubscribe = props.subscribe(value => {
    switch (value.view) {
      case DatePickerSelectType.DATE:
        component = TableDate
        break
      case DatePickerSelectType.MONTH:
        component = TableMonth
        break
      case DatePickerSelectType.YEAR:
        component = TableYear
        break
    }
  })
  onDestroy(() => {
    unsubscribe()
  })
</script>

<Popover bind:visible trigger="manual" popoverClass="w-60">
  <div
    slot="trigger"
    class="date-picker-trigger inline-flex relative items-center rounded-md py-1 pl-2 pr-6 shadow-md flex-1 dark:bg-black bg-white"
  >
    <i class="w-4 h-4 mr-1 text-slate-400">
      <CalendarDays />
    </i>
    <input
      class="cursor-default outline-none"
      placeholder="请选择日期"
      readonly
      value={$props.date ? $props.date.format('YYYY-MM-DD') : ''}
    />
    <i
      class="date-picker-clear w-4 h-4 absolute right-2 top-2 text-slate-400 cursor-pointer transition hidden"
      on:click|stopPropagation={() => {
        clear()
      }}
    >
      <XCircle />
    </i>
  </div>
  <svelte:component this={component} />
</Popover>

<style scoped>
  .date-picker-trigger:hover .date-picker-clear {
    @apply inline-block;
  }
</style>
