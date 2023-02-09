<script lang="ts">
  import { getContext } from 'svelte'
  import ChevronHeader from './ChevronHeader.svelte'
  import { DATEPICKER_KEY, DatePickerSelectType } from './DatePicker.svelte'
  import dayjs from 'dayjs'
  const { props }: any = getContext(DATEPICKER_KEY)
  const currentDate = dayjs()
  const currentYear: number = currentDate.year()
  const currentMonth: number = currentDate.month()

  $: activeYear = $props.date ? $props.date.year() : -1
  $: activeMonth = $props.date ? $props.date.month() : -1
  $: showYear = $props.showDate.year()
  const selectMonth = (i: number) => {
    props.update(value => {
      value.view = DatePickerSelectType.DATE
      if (!value.date) {
        value.date = dayjs()
        value.date = value.date.date(1)
      }
      value.date = value.date.year(showYear).month(i)
      value.showDate = value.showDate.year(showYear).month(i)
      return value
    })
  }
  const toSelectYear = () => {
    props.update(value => {
      value.view = DatePickerSelectType.YEAR
      return value
    })
  }
  const updateShowYear = (type: 'add' | 'subtract') => {
    props.update(value => {
      value.showDate =
        type === 'add'
          ? value.showDate.add(1, 'year')
          : value.showDate.subtract(1, 'year')
      return value
    })
  }
</script>

<ChevronHeader
  on:db-left={() => updateShowYear('subtract')}
  on:db-right={() => updateShowYear('add')}
>
  <span class="link" on:click={() => toSelectYear()}>
    {showYear}
  </span>
</ChevronHeader>
<div class="flex flex-wrap">
  {#each Array(12) as _, i}
    <span
      class="w-1/4 px-1 mt-2 text-center link"
      class:font-bold={showYear === currentYear && i === currentMonth}
      class:text-sky-400={(showYear === currentYear && i === currentMonth) ||
        (showYear === activeYear && i === activeMonth)}
      on:click={() => selectMonth(i)}
    >
      {i + 1}月
    </span>
  {/each}
</div>
