<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  export let withSingle: boolean = false
  const dispatch = createEventDispatcher()
</script>

<div class="flex justify-between items-center">
  <div class="inline-flex">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="w-4 h-4 cursor-pointer hover:text-sky-400"
      on:click={() => {
        dispatch('db-left')
      }}
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M18.75 19.5l-7.5-7.5 7.5-7.5m-6 15L5.25 12l7.5-7.5"
      />
    </svg>
    {#if withSingle}
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="w-4 h-4 cursor-pointer hover:text-sky-400"
        on:click={() => {
          dispatch('left')
        }}
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M15.75 19.5L8.25 12l7.5-7.5"
        />
      </svg>
    {/if}
  </div>
  <span class="slot">
    <slot />
  </span>
  <div class="inline-flex">
    {#if withSingle}
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="w-4 h-4 cursor-pointer hover:text-sky-400"
        on:click={() => {
          dispatch('right')
        }}
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M8.25 4.5l7.5 7.5-7.5 7.5"
        />
      </svg>
    {/if}
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
      class="w-4 h-4 cursor-pointer hover:text-sky-400"
      on:click={() => {
        dispatch('db-right')
      }}
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M11.25 4.5l7.5 7.5-7.5 7.5m-6-15l7.5 7.5-7.5 7.5"
      />
    </svg>
  </div>
</div>
