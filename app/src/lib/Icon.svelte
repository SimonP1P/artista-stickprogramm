<script lang="ts">
  // Icon im Stil des Entwurfs (Strich 1,5, currentColor). Größe: "s" 14 px, "" 16 px, "l" 20 px.
  let { d, groesse = "" }: { d: string; groesse?: "" | "s" | "l" } = $props();
</script>

<svg class="ico {groesse}" viewBox="0 0 24 24" aria-hidden="true"><path {d}></path></svg>
