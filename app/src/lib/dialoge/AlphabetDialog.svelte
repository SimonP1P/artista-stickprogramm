<script lang="ts">
  // Alphabet wählen: 50 Original-Alphabete mit Vorschau aus den ESB-Umrissen (`alphabet_vorschau`) und empfohlener
  // Höhe aus dem Handbuch (`alphabete_liste`).
  import Dialog from "../Dialog.svelte";
  import { listen } from "../listen.svelte";
  import { zahl } from "../zahl";
  import AlphabetBild from "./AlphabetBild.svelte";

  let { aktuell, waehlen, schliessen }: { aktuell?: string; waehlen: (name: string) => void; schliessen: () => void } =
    $props();

  let suche = $state("");
  const treffer = $derived(
    listen.alphabete.filter((a) => a.name.toLowerCase().includes(suche.trim().toLowerCase())),
  );
</script>

<Dialog titel="Alphabet wählen" breite={820} {schliessen}>
  {#snippet kopf()}
    <h2 class="hd">Alphabet wählen</h2>
    <span class="badge">{treffer.length} von {listen.alphabete.length}</span>
    <span style="flex:1"></span>
    <input class="input sm" placeholder="Suchen …" aria-label="Alphabet suchen" bind:value={suche} style="width:200px" />
  {/snippet}
  <p class="mut small" style="margin:0 0 10px">
    Vorschau aus den Umrissen der Original-Alphabete. Unter jedem Alphabet die empfohlene Höhe (Handbuch).
  </p>
  <div style="display:grid; grid-template-columns:repeat(5,minmax(0,1fr)); gap:8px">
    {#each treffer as a (a.name)}
      <button class="atile" class:on={a.name === aktuell} aria-pressed={a.name === aktuell} onclick={() => waehlen(a.name)}>
        <span class="aimg" style="background:var(--panel); border-style:solid; height:52px"><AlphabetBild name={a.name} /></span>
        <b style="font-weight:500; font-size:13px">{a.name}</b>
        <span class="mut small">
          {#if a.min_hoehe != null}{zahl(a.min_hoehe)}–{a.max_hoehe != null ? zahl(a.max_hoehe) : "?"} mm{:else}keine Angabe{/if}
        </span>
      </button>
    {/each}
  </div>
  {#snippet fuss()}
    <span class="mut small" style="margin-right:auto">Computer-Schriften (TrueType/OpenType): Expertenmodus → Einfügen</span>
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
  {/snippet}
</Dialog>
