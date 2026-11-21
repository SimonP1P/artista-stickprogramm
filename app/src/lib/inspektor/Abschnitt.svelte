<script lang="ts">
  // Aufklappbarer Abschnitt des Inspektors.
  import type { Snippet } from "svelte";
  import Icon from "../Icon.svelte";
  import { IC } from "../icons";

  let { titel, offen = true, rechts = "", children }: { titel: string; offen?: boolean; rechts?: string; children: Snippet } = $props();
  let auf = $state<boolean | null>(null);
  const zu = $derived(auf === null ? !offen : !auf);
</script>

<section class="sec">
  <button class="sech" aria-expanded={!zu} onclick={() => (auf = zu)}>
    <span style="display:inline-block; transform:rotate({zu ? -90 : 0}deg)"><Icon d={IC.chevronUnten} groesse="s" /></span>
    <span class="hd" style="font-size:15px">{titel}</span>
    <span style="flex:1"></span>
    {#if rechts}<span class="mut small tab">{rechts}</span>{/if}
  </button>
  {#if !zu}<div style="padding-bottom:8px">{@render children()}</div>{/if}
</section>
