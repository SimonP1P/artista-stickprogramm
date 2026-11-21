<script lang="ts">
  // Schriften eingebettet (offline, UEBERGABE §5): Barlow 400/500/700, Barlow Condensed 400/600
  import "@fontsource/barlow/400.css";
  import "@fontsource/barlow/500.css";
  import "@fontsource/barlow/700.css";
  import "@fontsource/barlow-condensed/400.css";
  import "@fontsource/barlow-condensed/600.css";
  import "$lib/stil/industry.css";
  import "$lib/stil/app.css";
  import Tooltip from "$lib/Tooltip.svelte";

  let { children } = $props();
</script>

{@render children()}
<Tooltip />
