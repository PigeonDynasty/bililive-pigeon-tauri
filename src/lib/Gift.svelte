<script lang="ts">
  import gifts from '../store/gift'
  import GiftCard from './components/GiftCard.svelte'
  import Empty from '@/components/Empty.svelte'
</script>

<div class="pt-2 pb-1 relative min-h-full">
  {#each $gifts as gift ('gift-content_' + gift.id)}
    <GiftCard data={gift} />
  {/each}
  {#if $gifts.length === 0}
    <Empty
      class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2"
      text="请先连接房间"
    />
  {/if}
</div>
