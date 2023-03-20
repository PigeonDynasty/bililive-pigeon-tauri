<script lang="ts">
  import { getContext } from 'svelte'
  import ChevronHeader from './ChevronHeader.svelte'
  import { DATEPICKER_KEY, DatePickerSelectType } from './DatePicker.svelte'
  import dayjs from 'dayjs'
  import type { DatePicker } from '@/@types/date-picker'
  const { props }: DatePicker.Context = getContext(DATEPICKER_KEY)
  const currentYear: number = dayjs().year()
  $: activeYear = $props.date ? $props.date.year() : -1
  $: startYear = Math.floor($props.showDate.year() / 10) * 10
  const selectYear = (i: number) => {
    props.update(value => {
      if (!value.date) value.date = dayjs()
      value.date = value.date.year(startYear + i)
      value.showDate = value.showDate.year(startYear + i)
      value.view = DatePickerSelectType.MONTH
      return value
    })
  }
  const updateShowYear = (type: 'add' | 'subtract') => {
    props.update(value => {
      value.showDate =
        type === 'add'
          ? value.showDate.add(10, 'year')
          : value.showDate.subtract(10, 'year')
      return value
    })
  }
</script>

<ChevronHeader
  on:db-left={() => updateShowYear('subtract')}
  on:db-right={() => updateShowYear('add')}
>
  {startYear} - {startYear + 9}
</ChevronHeader>
<div class="flex flex-wrap">
  {#each Array(10) as _, i}
    <span
      class="w-1/4 px-1 mt-2 text-center link"
      class:font-bold={startYear + i === currentYear}
      class:text-sky-400={startYear + i === currentYear ||
        startYear + i === activeYear}
      on:click={() => selectYear(i)}
      on:keypress={() => {}}
    >
      {startYear + i}
    </span>
  {/each}
</div>
