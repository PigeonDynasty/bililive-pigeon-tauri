<script lang="ts">
  import ChevronDoubleLeft from '@/icons/ChevronDoubleLeft.svelte'
  import ChevronDoubleRight from '@/icons/ChevronDoubleRight.svelte'
  import ChevronLeft from '@/icons/ChevronLeft.svelte'
  import ChevronRight from '@/icons/ChevronRight.svelte'
  import { createEventDispatcher, getContext } from 'svelte'
  import TimePicker from '../TimePicker.svelte'
  import { DATEPICKER_KEY } from './DatePicker.svelte'
  export let withSingle: boolean = false
  const dispatch = createEventDispatcher()
  const { props, time, select }: any = getContext(DATEPICKER_KEY)
  const selectTime = date => {
    props.update(value => {
      value.date = date
      return value
    })
    select(date)
  }
</script>

{#if time}
  <div class="pb-1">
    <TimePicker
      class="w-full"
      value={$props.date}
      on:change={e => selectTime(e.detail)}
    />
  </div>
{/if}
<div class="flex justify-between items-center">
  <div class="inline-flex">
    <i
      class="w-4 h-4 cursor-pointer hover:text-sky-400"
      on:click={() => {
        dispatch('db-left')
      }}
    >
      <ChevronDoubleLeft />
    </i>
    {#if withSingle}
      <i
        class="w-4 h-4 cursor-pointer hover:text-sky-400"
        on:click={() => {
          dispatch('left')
        }}
      >
        <ChevronLeft />
      </i>
    {/if}
  </div>
  <span class="slot">
    <slot />
  </span>
  <div class="inline-flex">
    {#if withSingle}
      <i
        class="w-4 h-4 cursor-pointer hover:text-sky-400"
        on:click={() => {
          dispatch('right')
        }}
      >
        <ChevronRight />
      </i>
    {/if}
    <i
      class="w-4 h-4 cursor-pointer hover:text-sky-400"
      on:click={() => {
        dispatch('db-right')
      }}
    >
      <ChevronDoubleRight />
    </i>
  </div>
</div>
