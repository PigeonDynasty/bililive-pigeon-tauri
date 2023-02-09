<script lang="ts">
  import ArchiveBoxXMark from '@/icons/ArchiveBoxXMark.svelte'
  let text = '暂无数据'
  let className = ''
  export { text, className as class }
</script>

<div class={`text-zinc-300 dark:text-zinc-600 text-center ${className}`}>
  <i class="inline-block w-16 h-16">
    <ArchiveBoxXMark />
  </i>
  <p>{text}</p>
</div>
