<script lang="ts">
  import { slide } from 'svelte/transition'
  import Table from '../../components/Table.svelte'
  import { delGiftContent } from '../../store/gift'
  import rooms from '../../store/room'
  import { fade } from 'svelte/transition'

  $: cardHeader = data.id + (data.uname ? `-${data.uname}` : '')
  let open = true
  let data = {
    id: '',
    uname: '',
    data: []
  }
  export { data }
</script>

<div class="shadow-md rounded bg-white dark:bg-black mb-2">
  <h5
    class="flex justify-between cursor-pointer p-2"
    on:click={() => (open = !open)}
  >
    {cardHeader}
    <div class="inline-flex items-center">
      {#if $rooms.every(item => item.room_id !== data.id)}
        <button
          transition:fade
          class="btn-rose text-xs px-2 py-1 rounded-full mr-1"
          on:click|stopPropagation={() => {
            delGiftContent(data.id)
          }}
        >
          移除
        </button>
      {/if}
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="w-4 h-4 transition duration-300"
        class:-rotate-90={!open}
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M19.5 8.25l-7.5 7.5-7.5-7.5"
        />
      </svg>
    </div>
  </h5>
  {#if open}
    <div transition:slide>
      <Table>
        <colgroup slot="colgroup">
          <col width="30" />
          <col />
          <col width="100" />
          <col width="50" />
          <col width="45" />
        </colgroup>
        <thead slot="head">
          <tr>
            <th />
            <th>昵称</th>
            <th>礼物</th>
            <th>类型</th>
            <th>数量</th>
          </tr>
        </thead>
        <tbody slot="body">
          {#each data.data as row, i}
            <tr
              class:bg-neutral-100={i % 2 === 0}
              class:dark:bg-neutral-800={i % 2 === 0}
            >
              <td>{i + 1}</td>
              <td>{row.uname}</td>
              <td>{row.giftName}</td>
              <td>{row.coin_type}</td>
              <td>{row.num}</td>
            </tr>
          {/each}
        </tbody>
      </Table>
    </div>
  {/if}
</div>
