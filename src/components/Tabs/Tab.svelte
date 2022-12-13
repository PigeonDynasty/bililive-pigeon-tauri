<script lang="ts">
  import { getContext } from 'svelte'
  import { TABS_KEY } from './Tabs.svelte'
  const { register, current }: any = getContext(TABS_KEY)
  import { fade } from 'svelte/transition'
  export let header: string | number = ''
  export let key = ''
  register({
    header,
    key
  }).then(k => {
    key = k
  })
</script>

<li
  data-key={key}
  class="tab-content transition h-full"
  class:hidden={$current !== key}
  in:fade
>
  <slot />
</li>
