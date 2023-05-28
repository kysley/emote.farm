<script lang="ts">
  import { page } from "$app/stores";
  import {
    ClickableTile,
    ContentSwitcher,
    Switch,
    Tile,
  } from "carbon-components-svelte";
  import { onMount } from "svelte";
  import { createQuery, hashQueryKey } from "@tanstack/svelte-query";

  const DURATIONS = [1, 3, 8, 24];

  $: selectedIndex = 0;
  let totals = [];
  let emotes: Record<string, string>;

  $: query = createQuery<{ count: number; emoteName: string }[], Error>(
    ["since", selectedIndex],

    async () => {
      const res = await fetch(
        `http://localhost:8012/channel/moonmoon/since?since=${DURATIONS[selectedIndex]}`
      );
      const data = (await res.json()) as { count: number; emoteName: string }[];

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

<main class="flex w-full flex-col">
  <h1>{$page.params.channel}</h1>
  <div class="md:w-60 w-full">
    <ContentSwitcher bind:selectedIndex size="sm">
      <Switch text="1h" />
      <Switch text="3h" />
      <Switch text="8h" />
      <Switch text="24h" />
    </ContentSwitcher>
  </div>

  {#if !$query.isLoading && $query.data && emotes}
    <div class="grid grid-cols-8 gap-3">
      {#each $query.data as dat}
        <Tile>
          <span class="break-all">{dat.emoteName}</span>
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

  <!-- {#if $query.isLoading} -->

  <!-- {#if totals} -->
  <!-- <div class="flex flex-col"> -->
  <!-- {#each totals as doc}
        <span>
          {doc.emoteName} - {doc.usageCount}
        </span>
      {/each} -->
  <!-- </div> -->
  <!-- {/if} -->
</main>
