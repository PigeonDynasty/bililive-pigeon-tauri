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
    class="date-picker-trigger inline-flex relative items-center rounded-md py-1 pl-1 pr-6 shadow-md flex-1 dark:bg-black bg-white"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="w-4 h-4 mr-1 text-slate-400"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5m-9-6h.008v.008H12v-.008zM12 15h.008v.008H12V15zm0 2.25h.008v.008H12v-.008zM9.75 15h.008v.008H9.75V15zm0 2.25h.008v.008H9.75v-.008zM7.5 15h.008v.008H7.5V15zm0 2.25h.008v.008H7.5v-.008zm6.75-4.5h.008v.008h-.008v-.008zm0 2.25h.008v.008h-.008V15zm0 2.25h.008v.008h-.008v-.008zm2.25-4.5h.008v.008H16.5v-.008zm0 2.25h.008v.008H16.5V15z"
      />
    </svg>

    <input
      class="cursor-default outline-none"
      placeholder="请选择日期"
      readonly
      value={$props.date ? $props.date.format('YYYY-MM-DD') : ''}
    />
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="date-picker-clear w-4 h-4 absolute right-2 top-2 text-slate-400 cursor-pointer transition hidden"
      on:click|stopPropagation={() => {
        clear()
      }}
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M9.75 9.75l4.5 4.5m0-4.5l-4.5 4.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
      />
    </svg>
  </div>
  <svelte:component this={component} />
</Popover>

<style scoped>
  .date-picker-trigger:hover .date-picker-clear {
    @apply inline-block;
  }
</style>
