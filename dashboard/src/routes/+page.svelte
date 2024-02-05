<script lang="ts">
  import type { Sensors } from "$lib/piApi";
  import { sensorState } from "$lib/piApi";
  import { repeat } from "$lib/repeat";
  let status = $state<Sensors | undefined>();

  $effect(() => {
    repeat(async () => {
      status = await sensorState();
    }, 1000);
  });
</script>

<h1>Dashboard</h1>
<div>{status?.temperature}°C</div>
<div>{status?.humidity}%</div>
