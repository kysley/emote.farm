<script lang="ts">
  import {
    DataTable,
    DataTableSkeleton,
    Link,
    Toolbar,
    ToolbarContent,
    ToolbarSearch,
  } from "carbon-components-svelte";

  export let data;
  export let loading = false;
  export let onSearch: (search: string) => void = () => {};

  let filteredRowIds: number[] = [];
</script>

{#if loading}
  <DataTableSkeleton />
{:else}
  <DataTable
    expandable
    sortable
    headers={[
      { key: "emoteName", value: "Emote" },
      { key: "count", value: "# uses" },
    ]}
    rows={data}
  >
    <Toolbar>
      <ToolbarContent>
        <ToolbarSearch persistent shouldFilterRows bind:filteredRowIds />
      </ToolbarContent>
    </Toolbar>
    <svelte:fragment slot="cell" let:row let:cell>
      {#if cell.key === "emoteName"}
        <Link
          href={`https://betterttv.com/emotes/${row.emoteId}`}
          target="_blank">{cell.value}</Link
        >
        <!-- <img
          src={`https://cdn.betterttv.net/emote/${row.emoteId}/3x.webp`}
          alt={cell.value.emoteName}
        /> -->
      {:else}
        {cell.value}
      {/if}
    </svelte:fragment>
    <svelte:fragment slot="expanded-row" let:row>
      <img
        src={`https://cdn.betterttv.net/emote/${row.emoteId}/3x.webp`}
        alt={row.value}
      />
    </svelte:fragment>
  </DataTable>
{/if}
