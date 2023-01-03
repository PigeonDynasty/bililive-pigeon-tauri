<script lang="ts">
  import gifts from '../store/gift'
  import GiftCard from './components/GiftCard.svelte'
</script>

<div class="pt-2">
  {#each $gifts as gift ('gift-content_' + gift.id)}
    <GiftCard data={gift} />
  {/each}
</div>
