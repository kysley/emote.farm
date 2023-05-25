<script lang="ts">
  import { page } from "$app/stores";
  import { ClickableTile } from "carbon-components-svelte";
  import { onMount } from "svelte";

  let totals = [];
  let emotes = [];

  onMount(() => {
    fetch("http://localhost:8012/channel/moonmoon/totals")
      .then((r) => r.json())
      .then((d) => (totals = d.sort((a, b) => a.usageCount < b.usageCount)));
    fetch("http://localhost:8012/channel/moonmoon/emotes")
      .then((r) => r.json())
      .then((d) => (emotes = d));
  });
</script>

<main class="flex h-screen w-full flex-col">
  <h1>{$page.params.channel}</h1>
  {#if totals}
    <div class="flex flex-col">
      {#each totals as doc}
        <span>
          {doc.emoteName} - {doc.usageCount}
        </span>
      {/each}
    </div>
  {/if}
</main>
