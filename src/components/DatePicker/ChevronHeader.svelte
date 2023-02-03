<script lang="ts">
  import ChevronDoubleLeft from '@/icons/ChevronDoubleLeft.svelte'
  import ChevronDoubleRight from '@/icons/ChevronDoubleRight.svelte'
  import ChevronLeft from '@/icons/ChevronLeft.svelte'
  import ChevronRight from '@/icons/ChevronRight.svelte'
  import { createEventDispatcher } from 'svelte'
  export let withSingle: boolean = false
  const dispatch = createEventDispatcher()
</script>

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
