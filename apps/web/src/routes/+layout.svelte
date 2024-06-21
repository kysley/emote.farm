<script>
  import "../app.css";
  import "carbon-components-svelte/css/all.css";
  import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query";

  const queryClient = new QueryClient();
</script>

<link rel="stylesheet" href="https://unpkg.com/@carbon/charts/styles.css" />

<QueryClientProvider client={queryClient}>
  <slot />
</QueryClientProvider>
