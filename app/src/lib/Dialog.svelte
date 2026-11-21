<script lang="ts">
  // Modaler Dialog wie im Entwurf (Kopf · Inhalt · Fuß). Esc und Klick auf den Hintergrund schließen.
  import type { Snippet } from "svelte";
  import Icon from "./Icon.svelte";
  import { IC } from "./icons";

  let {
    titel,
    breite = 560,
    schliessen,
    kopf,
    fuss,
    children,
  }: {
    titel: string;
    breite?: number;
    schliessen: () => void;
    kopf?: Snippet;
    fuss?: Snippet;
    children: Snippet;
  } = $props();

  let fenster: HTMLDivElement | undefined = $state();
  $effect(() => {
    fenster?.querySelector<HTMLElement>("input, select, textarea, button:not([aria-label='Schließen'])")?.focus();
  });
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && (e.stopPropagation(), schliessen())} />

<div class="dlg-bg" role="presentation" onclick={(e) => e.target === e.currentTarget && schliessen()}>
  <div class="dlg" role="dialog" aria-modal="true" aria-label={titel} style="width:{breite}px; max-height:calc(100vh - 40px)" bind:this={fenster}>
    <div class="dlg-h">
      {#if kopf}{@render kopf()}{:else}<h2 class="hd">{titel}</h2>{/if}
      <span style="flex:1"></span>
      <button class="tb" aria-label="Schließen" onclick={schliessen}><Icon d={IC.kreuz} /></button>
    </div>
    <div class="dlg-b">{@render children()}</div>
    {#if fuss}<div class="dlg-f">{@render fuss()}</div>{/if}
  </div>
</div>
