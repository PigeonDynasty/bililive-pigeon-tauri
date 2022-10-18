<script lang="ts">
  import { getContext } from 'svelte'
  import { TABS_KEY } from './Tabs.svelte'
  const { register, current }: any = getContext(TABS_KEY)
  import { fade } from 'svelte/transition'
  export let header = ''
  export let key = ''
  register({
    header,
    key
  }).then(i => {
    if (i > -1) key = i
  })
</script>

{#if $current === key}
  <li class="tab-content transition" in:fade>
    <slot />
  </li>
{/if}
