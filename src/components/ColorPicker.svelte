<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import Popover from './Popover.svelte'
  const dispatch = createEventDispatcher()
  let colorEl
  let value: string = ''
  let className: string = ''
  // let colors: string[] = [
  //   'slate',
  //   'gray',
  //   'zinc',
  //   'neutral',
  //   'stone',
  //   'red',
  //   'orange',
  //   'amber',
  //   'yellow',
  //   'lime',
  //   'green',
  //   'emerald',
  //   'teal',
  //   'cyan',
  //   'sky',
  //   'blue',
  //   'indigo',
  //   'violet',
  //   'purple',
  //   'fuchsia',
  //   'pink',
  //   'rose'
  // ]
  // let variants: number[] = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900]
  $: textColor = () => {
    let arr = value.split('-')
    if (arr.length > 1) {
      return Number(arr[1]) < 500 ? 'text-slate-800' : 'text-slate-100'
    }
    return ''
  }
  const selectColor = (color, variant) => {
    value = color + '-' + variant
    requestAnimationFrame(() => {
      dispatch('change', value, getColor())
    })
  }
  const getColor = () => {
    return colorEl ? getComputedStyle(colorEl)['background-color'] : ''
  }
  export { className as class, value, getColor }
</script>

<Popover popoverClass="h-48">
  <div
    slot="trigger"
    class={`inline-flex cursor-pointer py-1 px-2 shadow-md bg-${value} ${textColor()} ${className}`}
    bind:this={colorEl}
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-6 w-6"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"
      />
    </svg>
    {value}
  </div>
  <!-- FIXME 为什么循环不行 fk -->
  <!-- {#each colors as color}
        <div class="flex">
          {#each variants as variant}
            <div
              class={`cursor-pointer w-6 h-6 rounded-full m-1 bg-${color}-${variant}`}
              title={`${color}-${variant}`}
              on:click={() => selectColor(color, variant)}
            />
          {/each}
        </div>
      {/each} -->
  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-slate-50"
      title="slate-50"
      on:click={() => selectColor('slate', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-slate-100"
      title="slate-100"
      on:click={() => selectColor('slate', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-slate-200"
      title="slate-200"
      on:click={() => selectColor('slate', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-slate-300"
      title="slate-300"
      on:click={() => selectColor('slate', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-slate-400"
      title="slate-400"
      on:click={() => selectColor('slate', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-slate-500"
      title="slate-500"
      on:click={() => selectColor('slate', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-slate-600"
      title="slate-600"
      on:click={() => selectColor('slate', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-slate-700"
      title="slate-700"
      on:click={() => selectColor('slate', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-slate-800"
      title="slate-800"
      on:click={() => selectColor('slate', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-slate-900"
      title="slate-900"
      on:click={() => selectColor('slate', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-gray-50"
      title="gray-50"
      on:click={() => selectColor('gray', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-gray-100"
      title="gray-100"
      on:click={() => selectColor('gray', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-gray-200"
      title="gray-200"
      on:click={() => selectColor('gray', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-gray-300"
      title="gray-300"
      on:click={() => selectColor('gray', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-gray-400"
      title="gray-400"
      on:click={() => selectColor('gray', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-gray-500"
      title="gray-500"
      on:click={() => selectColor('gray', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-gray-600"
      title="gray-600"
      on:click={() => selectColor('gray', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-gray-700"
      title="gray-700"
      on:click={() => selectColor('gray', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-gray-800"
      title="gray-800"
      on:click={() => selectColor('gray', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-gray-900"
      title="gray-900"
      on:click={() => selectColor('gray', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-zinc-50"
      title="zinc-50"
      on:click={() => selectColor('zinc', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-zinc-100"
      title="zinc-100"
      on:click={() => selectColor('zinc', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-zinc-200"
      title="zinc-200"
      on:click={() => selectColor('zinc', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-zinc-300"
      title="zinc-300"
      on:click={() => selectColor('zinc', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-zinc-400"
      title="zinc-400"
      on:click={() => selectColor('zinc', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-zinc-500"
      title="zinc-500"
      on:click={() => selectColor('zinc', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-zinc-600"
      title="zinc-600"
      on:click={() => selectColor('zinc', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-zinc-700"
      title="zinc-700"
      on:click={() => selectColor('zinc', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-zinc-800"
      title="zinc-800"
      on:click={() => selectColor('zinc', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-zinc-900"
      title="zinc-900"
      on:click={() => selectColor('zinc', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-neutral-50"
      title="neutral-50"
      on:click={() => selectColor('neutral', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-neutral-100"
      title="neutral-100"
      on:click={() => selectColor('neutral', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-neutral-200"
      title="neutral-200"
      on:click={() => selectColor('neutral', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-neutral-300"
      title="neutral-300"
      on:click={() => selectColor('neutral', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-neutral-400"
      title="neutral-400"
      on:click={() => selectColor('neutral', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-neutral-500"
      title="neutral-500"
      on:click={() => selectColor('neutral', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-neutral-600"
      title="neutral-600"
      on:click={() => selectColor('neutral', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-neutral-700"
      title="neutral-700"
      on:click={() => selectColor('neutral', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-neutral-800"
      title="neutral-800"
      on:click={() => selectColor('neutral', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-neutral-900"
      title="neutral-900"
      on:click={() => selectColor('neutral', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-stone-50"
      title="stone-50"
      on:click={() => selectColor('stone', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-stone-100"
      title="stone-100"
      on:click={() => selectColor('stone', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-stone-200"
      title="stone-200"
      on:click={() => selectColor('stone', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-stone-300"
      title="stone-300"
      on:click={() => selectColor('stone', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-stone-400"
      title="stone-400"
      on:click={() => selectColor('stone', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-stone-500"
      title="stone-500"
      on:click={() => selectColor('stone', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-stone-600"
      title="stone-600"
      on:click={() => selectColor('stone', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-stone-700"
      title="stone-700"
      on:click={() => selectColor('stone', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-stone-800"
      title="stone-800"
      on:click={() => selectColor('stone', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-stone-900"
      title="stone-900"
      on:click={() => selectColor('stone', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-red-50"
      title="red-50"
      on:click={() => selectColor('red', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-red-100"
      title="red-100"
      on:click={() => selectColor('red', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-red-200"
      title="red-200"
      on:click={() => selectColor('red', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-red-300"
      title="red-300"
      on:click={() => selectColor('red', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-red-400"
      title="red-400"
      on:click={() => selectColor('red', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-red-500"
      title="red-500"
      on:click={() => selectColor('red', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-red-600"
      title="red-600"
      on:click={() => selectColor('red', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-red-700"
      title="red-700"
      on:click={() => selectColor('red', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-red-800"
      title="red-800"
      on:click={() => selectColor('red', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-red-900"
      title="red-900"
      on:click={() => selectColor('red', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-orange-50"
      title="orange-50"
      on:click={() => selectColor('orange', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-orange-100"
      title="orange-100"
      on:click={() => selectColor('orange', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-orange-200"
      title="orange-200"
      on:click={() => selectColor('orange', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-orange-300"
      title="orange-300"
      on:click={() => selectColor('orange', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-orange-400"
      title="orange-400"
      on:click={() => selectColor('orange', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-orange-500"
      title="orange-500"
      on:click={() => selectColor('orange', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-orange-600"
      title="orange-600"
      on:click={() => selectColor('orange', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-orange-700"
      title="orange-700"
      on:click={() => selectColor('orange', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-orange-800"
      title="orange-800"
      on:click={() => selectColor('orange', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-orange-900"
      title="orange-900"
      on:click={() => selectColor('orange', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-amber-50"
      title="amber-50"
      on:click={() => selectColor('amber', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-amber-100"
      title="amber-100"
      on:click={() => selectColor('amber', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-amber-200"
      title="amber-200"
      on:click={() => selectColor('amber', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-amber-300"
      title="amber-300"
      on:click={() => selectColor('amber', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-amber-400"
      title="amber-400"
      on:click={() => selectColor('amber', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-amber-500"
      title="amber-500"
      on:click={() => selectColor('amber', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-amber-600"
      title="amber-600"
      on:click={() => selectColor('amber', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-amber-700"
      title="amber-700"
      on:click={() => selectColor('amber', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-amber-800"
      title="amber-800"
      on:click={() => selectColor('amber', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-amber-900"
      title="amber-900"
      on:click={() => selectColor('amber', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-yellow-50"
      title="yellow-50"
      on:click={() => selectColor('yellow', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-yellow-100"
      title="yellow-100"
      on:click={() => selectColor('yellow', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-yellow-200"
      title="yellow-200"
      on:click={() => selectColor('yellow', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-yellow-300"
      title="yellow-300"
      on:click={() => selectColor('yellow', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-yellow-400"
      title="yellow-400"
      on:click={() => selectColor('yellow', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-yellow-500"
      title="yellow-500"
      on:click={() => selectColor('yellow', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-yellow-600"
      title="yellow-600"
      on:click={() => selectColor('yellow', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-yellow-700"
      title="yellow-700"
      on:click={() => selectColor('yellow', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-yellow-800"
      title="yellow-800"
      on:click={() => selectColor('yellow', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-yellow-900"
      title="yellow-900"
      on:click={() => selectColor('yellow', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-lime-50"
      title="lime-50"
      on:click={() => selectColor('lime', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-lime-100"
      title="lime-100"
      on:click={() => selectColor('lime', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-lime-200"
      title="lime-200"
      on:click={() => selectColor('lime', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-lime-300"
      title="lime-300"
      on:click={() => selectColor('lime', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-lime-400"
      title="lime-400"
      on:click={() => selectColor('lime', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-lime-500"
      title="lime-500"
      on:click={() => selectColor('lime', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-lime-600"
      title="lime-600"
      on:click={() => selectColor('lime', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-lime-700"
      title="lime-700"
      on:click={() => selectColor('lime', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-lime-800"
      title="lime-800"
      on:click={() => selectColor('lime', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-lime-900"
      title="lime-900"
      on:click={() => selectColor('lime', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-green-50"
      title="green-50"
      on:click={() => selectColor('green', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-green-100"
      title="green-100"
      on:click={() => selectColor('green', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-green-200"
      title="green-200"
      on:click={() => selectColor('green', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-green-300"
      title="green-300"
      on:click={() => selectColor('green', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-green-400"
      title="green-400"
      on:click={() => selectColor('green', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-green-500"
      title="green-500"
      on:click={() => selectColor('green', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-green-600"
      title="green-600"
      on:click={() => selectColor('green', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-green-700"
      title="green-700"
      on:click={() => selectColor('green', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-green-800"
      title="green-800"
      on:click={() => selectColor('green', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-green-900"
      title="green-900"
      on:click={() => selectColor('green', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-emerald-50"
      title="emerald-50"
      on:click={() => selectColor('emerald', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-emerald-100"
      title="emerald-100"
      on:click={() => selectColor('emerald', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-emerald-200"
      title="emerald-200"
      on:click={() => selectColor('emerald', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-emerald-300"
      title="emerald-300"
      on:click={() => selectColor('emerald', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-emerald-400"
      title="emerald-400"
      on:click={() => selectColor('emerald', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-emerald-500"
      title="emerald-500"
      on:click={() => selectColor('emerald', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-emerald-600"
      title="emerald-600"
      on:click={() => selectColor('emerald', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-emerald-700"
      title="emerald-700"
      on:click={() => selectColor('emerald', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-emerald-800"
      title="emerald-800"
      on:click={() => selectColor('emerald', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-emerald-900"
      title="emerald-900"
      on:click={() => selectColor('emerald', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-teal-50"
      title="teal-50"
      on:click={() => selectColor('teal', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-teal-100"
      title="teal-100"
      on:click={() => selectColor('teal', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-teal-200"
      title="teal-200"
      on:click={() => selectColor('teal', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-teal-300"
      title="teal-300"
      on:click={() => selectColor('teal', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-teal-400"
      title="teal-400"
      on:click={() => selectColor('teal', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-teal-500"
      title="teal-500"
      on:click={() => selectColor('teal', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-teal-600"
      title="teal-600"
      on:click={() => selectColor('teal', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-teal-700"
      title="teal-700"
      on:click={() => selectColor('teal', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-teal-800"
      title="teal-800"
      on:click={() => selectColor('teal', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-teal-900"
      title="teal-900"
      on:click={() => selectColor('teal', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-cyan-50"
      title="cyan-50"
      on:click={() => selectColor('cyan', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-cyan-100"
      title="cyan-100"
      on:click={() => selectColor('cyan', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-cyan-200"
      title="cyan-200"
      on:click={() => selectColor('cyan', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-cyan-300"
      title="cyan-300"
      on:click={() => selectColor('cyan', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-cyan-400"
      title="cyan-400"
      on:click={() => selectColor('cyan', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-cyan-500"
      title="cyan-500"
      on:click={() => selectColor('cyan', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-cyan-600"
      title="cyan-600"
      on:click={() => selectColor('cyan', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-cyan-700"
      title="cyan-700"
      on:click={() => selectColor('cyan', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-cyan-800"
      title="cyan-800"
      on:click={() => selectColor('cyan', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-cyan-900"
      title="cyan-900"
      on:click={() => selectColor('cyan', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-sky-50"
      title="sky-50"
      on:click={() => selectColor('sky', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-sky-100"
      title="sky-100"
      on:click={() => selectColor('sky', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-sky-200"
      title="sky-200"
      on:click={() => selectColor('sky', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-sky-300"
      title="sky-300"
      on:click={() => selectColor('sky', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-sky-400"
      title="sky-400"
      on:click={() => selectColor('sky', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-sky-500"
      title="sky-500"
      on:click={() => selectColor('sky', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-sky-600"
      title="sky-600"
      on:click={() => selectColor('sky', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-sky-700"
      title="sky-700"
      on:click={() => selectColor('sky', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-sky-800"
      title="sky-800"
      on:click={() => selectColor('sky', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-sky-900"
      title="sky-900"
      on:click={() => selectColor('sky', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-blue-50"
      title="blue-50"
      on:click={() => selectColor('blue', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-blue-100"
      title="blue-100"
      on:click={() => selectColor('blue', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-blue-200"
      title="blue-200"
      on:click={() => selectColor('blue', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-blue-300"
      title="blue-300"
      on:click={() => selectColor('blue', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-blue-400"
      title="blue-400"
      on:click={() => selectColor('blue', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-blue-500"
      title="blue-500"
      on:click={() => selectColor('blue', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-blue-600"
      title="blue-600"
      on:click={() => selectColor('blue', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-blue-700"
      title="blue-700"
      on:click={() => selectColor('blue', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-blue-800"
      title="blue-800"
      on:click={() => selectColor('blue', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-blue-900"
      title="blue-900"
      on:click={() => selectColor('blue', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-indigo-50"
      title="indigo-50"
      on:click={() => selectColor('indigo', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-indigo-100"
      title="indigo-100"
      on:click={() => selectColor('indigo', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-indigo-200"
      title="indigo-200"
      on:click={() => selectColor('indigo', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-indigo-300"
      title="indigo-300"
      on:click={() => selectColor('indigo', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-indigo-400"
      title="indigo-400"
      on:click={() => selectColor('indigo', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-indigo-500"
      title="indigo-500"
      on:click={() => selectColor('indigo', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-indigo-600"
      title="indigo-600"
      on:click={() => selectColor('indigo', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-indigo-700"
      title="indigo-700"
      on:click={() => selectColor('indigo', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-indigo-800"
      title="indigo-800"
      on:click={() => selectColor('indigo', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-indigo-900"
      title="indigo-900"
      on:click={() => selectColor('indigo', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-violet-50"
      title="violet-50"
      on:click={() => selectColor('violet', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-violet-100"
      title="violet-100"
      on:click={() => selectColor('violet', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-violet-200"
      title="violet-200"
      on:click={() => selectColor('violet', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-violet-300"
      title="violet-300"
      on:click={() => selectColor('violet', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-violet-400"
      title="violet-400"
      on:click={() => selectColor('violet', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-violet-500"
      title="violet-500"
      on:click={() => selectColor('violet', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-violet-600"
      title="violet-600"
      on:click={() => selectColor('violet', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-violet-700"
      title="violet-700"
      on:click={() => selectColor('violet', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-violet-800"
      title="violet-800"
      on:click={() => selectColor('violet', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-violet-900"
      title="violet-900"
      on:click={() => selectColor('violet', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-purple-50"
      title="purple-50"
      on:click={() => selectColor('purple', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-purple-100"
      title="purple-100"
      on:click={() => selectColor('purple', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-purple-200"
      title="purple-200"
      on:click={() => selectColor('purple', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-purple-300"
      title="purple-300"
      on:click={() => selectColor('purple', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-purple-400"
      title="purple-400"
      on:click={() => selectColor('purple', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-purple-500"
      title="purple-500"
      on:click={() => selectColor('purple', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-purple-600"
      title="purple-600"
      on:click={() => selectColor('purple', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-purple-700"
      title="purple-700"
      on:click={() => selectColor('purple', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-purple-800"
      title="purple-800"
      on:click={() => selectColor('purple', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-purple-900"
      title="purple-900"
      on:click={() => selectColor('purple', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-fuchsia-50"
      title="fuchsia-50"
      on:click={() => selectColor('fuchsia', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-fuchsia-100"
      title="fuchsia-100"
      on:click={() => selectColor('fuchsia', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-fuchsia-200"
      title="fuchsia-200"
      on:click={() => selectColor('fuchsia', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-fuchsia-300"
      title="fuchsia-300"
      on:click={() => selectColor('fuchsia', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-fuchsia-400"
      title="fuchsia-400"
      on:click={() => selectColor('fuchsia', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-fuchsia-500"
      title="fuchsia-500"
      on:click={() => selectColor('fuchsia', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-fuchsia-600"
      title="fuchsia-600"
      on:click={() => selectColor('fuchsia', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-fuchsia-700"
      title="fuchsia-700"
      on:click={() => selectColor('fuchsia', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-fuchsia-800"
      title="fuchsia-800"
      on:click={() => selectColor('fuchsia', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-fuchsia-900"
      title="fuchsia-900"
      on:click={() => selectColor('fuchsia', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-pink-50"
      title="pink-50"
      on:click={() => selectColor('pink', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-pink-100"
      title="pink-100"
      on:click={() => selectColor('pink', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-pink-200"
      title="pink-200"
      on:click={() => selectColor('pink', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-pink-300"
      title="pink-300"
      on:click={() => selectColor('pink', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-pink-400"
      title="pink-400"
      on:click={() => selectColor('pink', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-pink-500"
      title="pink-500"
      on:click={() => selectColor('pink', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-pink-600"
      title="pink-600"
      on:click={() => selectColor('pink', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-pink-700"
      title="pink-700"
      on:click={() => selectColor('pink', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-pink-800"
      title="pink-800"
      on:click={() => selectColor('pink', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-pink-900"
      title="pink-900"
      on:click={() => selectColor('pink', 900)}
    />
  </div>

  <div class="flex">
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-rose-50"
      title="rose-50"
      on:click={() => selectColor('rose', 50)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-rose-100"
      title="rose-100"
      on:click={() => selectColor('rose', 100)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-rose-200"
      title="rose-200"
      on:click={() => selectColor('rose', 200)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-rose-300"
      title="rose-300"
      on:click={() => selectColor('rose', 300)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-rose-400"
      title="rose-400"
      on:click={() => selectColor('rose', 400)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-rose-500"
      title="rose-500"
      on:click={() => selectColor('rose', 500)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-rose-600"
      title="rose-600"
      on:click={() => selectColor('rose', 600)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-rose-700"
      title="rose-700"
      on:click={() => selectColor('rose', 700)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-rose-800"
      title="rose-800"
      on:click={() => selectColor('rose', 800)}
    />
    <div
      class="cursor-pointer w-6 h-6 rounded-full m-1 bg-rose-900"
      title="rose-900"
      on:click={() => selectColor('rose', 900)}
    />
  </div>
</Popover>
