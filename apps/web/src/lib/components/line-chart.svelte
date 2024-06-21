<script lang="ts">
  import { LineChart } from "@carbon/charts-svelte";

  export let data: unknown[] = [];
</script>

<LineChart
  data={[
    {
      time_slot: "2023-06-01 22:25:00",
      emote_name: "DOCING",
      occurrences: 8,
    },
    {
      time_slot: "2023-06-01 22:25:00",
      emote_name: "GIGA",
      occurrences: 9,
    },
    {
      time_slot: "2023-06-01 22:25:00",
      emote_name: "Looking",
      occurrences: 4,
    },
    {
      time_slot: "2023-06-01 22:25:00",
      emote_name: "MAAAAN",
      occurrences: 7,
    },
    {
      time_slot: "2023-06-01 22:20:00",
      emote_name: "GIGA",
      occurrences: 3,
    },
    {
      time_slot: "2023-06-01 22:20:00",
      emote_name: "Looking",
      occurrences: 2,
    },
    {
      time_slot: "2023-06-01 22:20:00",
      emote_name: "MAAAAN",
      occurrences: 18,
    },
    {
      time_slot: "2023-06-01 22:15:00",
      emote_name: "Glizzy",
      occurrences: 1,
    },
    {
      time_slot: "2023-06-01 22:15:00",
      emote_name: "Looking",
      occurrences: 5,
    },
    {
      time_slot: "2023-06-01 22:15:00",
      emote_name: "MAAAAN",
      occurrences: 43,
    },
    {
      time_slot: "2023-06-01 22:10:00",
      emote_name: "DOCING",
      occurrences: 195,
    },
    {
      time_slot: "2023-06-01 22:10:00",
      emote_name: "GIGA",
      occurrences: 3,
    },
    {
      time_slot: "2023-06-01 22:10:00",
      emote_name: "Glizzy",
      occurrences: 594,
    },
    {
      time_slot: "2023-06-01 22:10:00",
      emote_name: "Looking",
      occurrences: 19,
    },
    {
      time_slot: "2023-06-01 22:10:00",
      emote_name: "MAAAAN",
      occurrences: 4,
    },
    {
      time_slot: "2023-06-01 22:05:00",
      emote_name: "GIGA",
      occurrences: 8,
    },
    {
      time_slot: "2023-06-01 22:05:00",
      emote_name: "Glizzy",
      occurrences: 19,
    },
    {
      time_slot: "2023-06-01 22:05:00",
      emote_name: "Looking",
      occurrences: 22,
    },
    {
      time_slot: "2023-06-01 22:05:00",
      emote_name: "MAAAAN",
      occurrences: 9,
    },
    {
      time_slot: "2023-06-01 22:00:00",
      emote_name: "Looking",
      occurrences: 5,
    },
    {
      time_slot: "2023-06-01 22:00:00",
      emote_name: "MAAAAN",
      occurrences: 6,
    },
    {
      time_slot: "2023-06-01 21:55:00",
      emote_name: "GIGA",
      occurrences: 29,
    },
    {
      time_slot: "2023-06-01 21:55:00",
      emote_name: "Looking",
      occurrences: 13,
    },
    {
      time_slot: "2023-06-01 21:55:00",
      emote_name: "MAAAAN",
      occurrences: 3,
    },
    {
      time_slot: "2023-06-01 21:50:00",
      emote_name: "GIGA",
      occurrences: 7,
    },
    {
      time_slot: "2023-06-01 21:50:00",
      emote_name: "Glizzy",
      occurrences: 1,
    },
    {
      time_slot: "2023-06-01 21:50:00",
      emote_name: "Looking",
      occurrences: 3,
    },
    {
      time_slot: "2023-06-01 21:50:00",
      emote_name: "MAAAAN",
      occurrences: 12,
    },
    {
      time_slot: "2023-06-01 21:45:00",
      emote_name: "GIGA",
      occurrences: 5,
    },
    {
      time_slot: "2023-06-01 21:45:00",
      emote_name: "Looking",
      occurrences: 10,
    },
    {
      time_slot: "2023-06-01 21:45:00",
      emote_name: "MAAAAN",
      occurrences: 9,
    },
    {
      time_slot: "2023-06-01 21:40:00",
      emote_name: "DOCING",
      occurrences: 1,
    },
    {
      time_slot: "2023-06-01 21:40:00",
      emote_name: "GIGA",
      occurrences: 6,
    },
    {
      time_slot: "2023-06-01 21:40:00",
      emote_name: "Looking",
      occurrences: 6,
    },
    {
      time_slot: "2023-06-01 21:40:00",
      emote_name: "MAAAAN",
      occurrences: 7,
    },
    {
      time_slot: "2023-06-01 21:35:00",
      emote_name: "DOCING",
      occurrences: 5,
    },
    {
      time_slot: "2023-06-01 21:35:00",
      emote_name: "GIGA",
      occurrences: 21,
    },
    {
      time_slot: "2023-06-01 21:35:00",
      emote_name: "Glizzy",
      occurrences: 118,
    },
    {
      time_slot: "2023-06-01 21:35:00",
      emote_name: "Looking",
      occurrences: 14,
    },
    {
      time_slot: "2023-06-01 21:35:00",
      emote_name: "MAAAAN",
      occurrences: 8,
    },
    {
      time_slot: "2023-06-01 21:30:00",
      emote_name: "GIGA",
      occurrences: 29,
    },
    {
      time_slot: "2023-06-01 21:30:00",
      emote_name: "Looking",
      occurrences: 5,
    },
    {
      time_slot: "2023-06-01 21:30:00",
      emote_name: "MAAAAN",
      occurrences: 7,
    },
  ]}
  options={{
    toolbar: {
      enabled: false,
    },
    data: {
      groupMapsTo: "emote_name",
    },
    zoomBar: {
      top: {
        enabled: true,
      },
    },
    tooltip: {
      valueFormatter: (value) => {
        if (value instanceof Date) {
          const format = new Intl.DateTimeFormat("en-US", {
            minute: "2-digit",
            hour: "numeric",
          }).format(value);
          return format;
        }
        return value;
      },
    },
    axes: {
      bottom: {
        title: "Emote used at",
        mapsTo: "time_slot",
        scaleType: "time",
      },
      left: {
        mapsTo: "occurrences",
        title: "Emote uses",
        scaleType: "linear",
      },
    },
    curve: "curveMonotoneX",
    height: "400px",
  }}
/>
