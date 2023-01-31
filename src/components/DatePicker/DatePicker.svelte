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
  <div slot="trigger">
    <input
      class="input cursor-default"
      placeholder="请选择日期"
      readonly
      value={$props.date ? $props.date.format('YYYY-MM-DD') : ''}
    />
  </div>
  <svelte:component this={component} />
</Popover>
