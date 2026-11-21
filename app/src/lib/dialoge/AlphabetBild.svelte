<script lang="ts">
  // Vorschau eines Original-Alphabets: Umrisse aus dem Kern (`alphabet_vorschau`), gefüllt (Umrissbild).
  import type { AlphabetVorschau } from "$kern/kern";
  import { listen } from "../listen.svelte";
  import Umrissbild from "./Umrissbild.svelte";

  let { name, text = "Aa Bb" }: { name: string; text?: string } = $props();
  let v = $state.raw<AlphabetVorschau | null>(null);
  let fehler = $state(false);

  $effect(() => {
    v = null;
    fehler = false;
    const [n, t] = [name, text];
    listen
      .alphabetVorschau(n, t)
      .then((x) => n === name && t === text && (v = x))
      .catch(() => (fehler = true));
  });
</script>

{#if v}
  <Umrissbild linien={v.linien} begrenzung={v.begrenzung} />
{:else}
  <span class="mut small">{fehler ? "–" : "…"}</span>
{/if}
