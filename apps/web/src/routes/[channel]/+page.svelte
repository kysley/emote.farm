<script lang="ts">
  import { page } from "$app/stores";
  import {
    ContentSwitcher,
    Switch,
    Tile,
    Search,
    Pagination,
  } from "carbon-components-svelte";
  import { onMount } from "svelte";
  import { createQuery } from "@tanstack/svelte-query";
  import { debounce } from "../../lib/debounce";

  const DURATIONS = [1, 3, 8, 24];

  $: selectedIndex = 0;
  $: searchFilterDebounced = "";
  $: searchFilter = "";
  let totals = [];
  let emotes: Record<string, string>;

  const updateDebouncedValue = debounce((value: string) => {
    searchFilterDebounced = value;
    console.log("Debounced value:", searchFilterDebounced);
    // Perform any additional logic with the debounced value
  }, 300); // Adjust the delay time according to your requirements

  $: query = createQuery<{ count: number; emoteName: string }[], Error>(
    ["since", selectedIndex, searchFilterDebounced],

    async () => {
      const res = await fetch(
        `http://localhost:8012/channel/moonmoon/since?since=${DURATIONS[selectedIndex]}`
      );
      const data = (await res.json()) as { count: number; emoteName: string }[];

      if (searchFilterDebounced) {
        return data.filter((e) =>
          e.emoteName
            .toLowerCase()
            .includes(searchFilterDebounced.toLowerCase())
        );
      }

      return data;
    },

    {
      refetchInterval: 5000,
      select(data) {
        return data.sort((a, b) => a.count < b.count);
      },
    }
  );

  onMount(() => {
    // fetch("http://localhost:8012/channel/moonmoon/totals")
    //   .then((r) => r.json())
    //   .then((d) => (totals = d.sort((a, b) => a.usageCount < b.usageCount)));

    fetch("http://localhost:8012/channel/moonmoon/emotes")
      .then((r) => r.json())
      .then((d) => (emotes = d));
  });
</script>

<main class="flex w-full flex-col p-5">
  <h1>{$page.params.channel}</h1>
  <div class="w-full flex flex-row gap-3 items-center justify-between">
    <ContentSwitcher bind:selectedIndex size="sm" class="w-52">
      <Switch text="1h" />
      <Switch text="3h" />
      <Switch text="8h" />
      <Switch text="24h" />
      <Switch text="all time" disabled />
    </ContentSwitcher>
    <Search
      on:input={(e) => (searchFilter = e.target.value)}
      on:input={(e) => updateDebouncedValue(e.target.value)}
    />
  </div>

  {#if !$query.isLoading && $query.data && emotes}
    <div class="grid grid-cols-8 gap-3">
      {#each $query.data as dat}
        <Tile>
          <a
            href={`https://betterttv.com/emotes/${emotes[dat.emoteName]}`}
            target="_blank"
          >
            <span class="break-all">{dat.emoteName}</span>
          </a>
          <img
            src={`https://cdn.betterttv.net/emote/${
              emotes[dat.emoteName]
            }/3x.webp`}
            alt={dat.emoteName}
          />
          <span class="font-bold">{dat.count}</span>
        </Tile>
      {/each}
    </div>
  {/if}
</main>
<Pagination
  style="position:sticky; bottom: 0; left: 0;"
  pageSize={$query.data?.length}
  totalItems={$query.data?.length}
  pageSizeInputDisabled
  pageInputDisabled
/>
