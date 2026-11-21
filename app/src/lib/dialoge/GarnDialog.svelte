<script lang="ts">
  // Garn von Hand wählen (`garnkarte_garne`): setzt am Objekt `garn` = Nummer und `farbe` = Farbe des Garns.
  // Ohne Wahl gilt das nächste Garn der Karte zur Bildschirmfarbe. Die Garnkarte gilt für das ganze Design.
  import type { GarnInfo } from "$kern/kern";
  import { arbeit } from "../arbeit.svelte";
  import Dialog from "../Dialog.svelte";
  import { garnText, listen } from "../listen.svelte";

  let {
    titel = "Garn von Hand wählen",
    untertitel = "",
    aktuell,
    waehlen,
    schliessen,
  }: {
    titel?: string;
    untertitel?: string;
    aktuell?: GarnInfo;
    waehlen: (g: GarnInfo) => void;
    schliessen: () => void;
  } = $props();

  const karte = $derived(arbeit.design.garnkarte ?? "Default");
  let garne = $state.raw<GarnInfo[]>([]);
  let suche = $state("");

  $effect(() => {
    const k = karte;
    garne = [];
    if (k !== "keine") listen.garne(k).then((g) => k === karte && (garne = g));
  });

  const treffer = $derived.by(() => {
    const s = suche.trim().toLowerCase();
    return s ? garne.filter((g) => g.nummer.toLowerCase().includes(s) || g.name.toLowerCase().includes(s)) : garne;
  });

  function karteWechseln(k: string) {
    arbeit.aendern((d) => (d.garnkarte = k));
  }
</script>

<Dialog {titel} breite={540} {schliessen}>
  {#snippet kopf()}
    {#if aktuell}<span class="sw l" style="background:{aktuell.rgb}"></span>{/if}
    <div style="display:flex; flex-direction:column; min-width:0">
      <h2 class="hd">{titel}</h2>
      <span class="mut small">{untertitel}{aktuell ? ` · jetzt: ${garnText(aktuell)}` : ""}</span>
    </div>
  {/snippet}
  <div style="display:flex; gap:8px; margin:-14px -16px 0; padding:10px 16px; border-bottom:1px solid var(--line)">
    <input class="input sm" style="flex:1" placeholder="Nummer oder Name, z. B. 3622 oder marine" aria-label="Garn suchen" bind:value={suche} />
    <select class="input sm" aria-label="Garnkarte" value={karte} onchange={(e) => karteWechseln((e.currentTarget as HTMLSelectElement).value)} style="width:210px">
      {#each listen.garnkarten as k (k)}<option value={k}>{k}</option>{/each}
      <option value="keine">keine</option>
    </select>
  </div>
  <div style="margin:0 -16px -14px">
    {#if karte === "keine"}
      <p class="mut" style="padding:14px 16px">Ohne Garnkarte gibt es keine Garnnamen – zuerst eine Karte wählen.</p>
    {/if}
    {#each treffer as g (g.nummer + g.name)}
      <button class="grow" class:on={aktuell?.nummer === g.nummer} onclick={() => waehlen(g)}>
        <span class="sw l" style="background:{g.rgb}"></span>
        <span class="tab">{g.nummer}</span>
        <span>{g.name}</span>
        <span class="mut small">{g.marke}</span>
      </button>
    {/each}
  </div>
  {#snippet fuss()}
    <span class="mut small" style="margin-right:auto">Ohne Wahl gilt: nächstes Garn der Karte zur Bildschirmfarbe.</span>
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
  {/snippet}
</Dialog>
