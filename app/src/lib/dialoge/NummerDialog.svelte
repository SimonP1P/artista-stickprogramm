<script lang="ts">
  // „Vor Objekt Nr. …“ / „Nach Objekt Nr. …“ (Menü Anordnen, Resequence by Number im Original): Nummer eingeben,
  // die Auswahl kommt davor bzw. dahinter (Kern `reihenfolge_aendern`). Ersetzt die Browser-Abfrage.
  import { arbeit } from "../arbeit.svelte";
  import Dialog from "../Dialog.svelte";

  let { vor, schliessen }: { vor: boolean; schliessen: () => void } = $props();

  const n = arbeit.objekte.length;
  const auswahl = [...arbeit.auswahl];
  // svelte-ignore state_referenced_locally
  let nr = $state<number>(vor ? Math.max(1, Math.min(...auswahl.map((i) => i + 1)) - 1) : Math.min(n, Math.max(...auswahl.map((i) => i + 1)) + 1));
  const gueltig = $derived(Number.isInteger(nr) && nr >= 1 && nr <= n);

  function ok() {
    if (!gueltig) return;
    const z = nr - 1;
    arbeit.umordnen(vor ? { art: "vor", auswahl, ziel: z } : { art: "nach", auswahl, ziel: z });
    schliessen();
  }
</script>

<Dialog titel={vor ? "Vor Objekt Nr. …" : "Nach Objekt Nr. …"} breite={380} {schliessen}>
  <form onsubmit={(e) => (e.preventDefault(), ok())}>
    <div class="frow" style="padding:0">
      <label class="fl" for="onr">{vor ? "Vor" : "Nach"} Objekt Nr.</label>
      <div class="fc"><input id="onr" class="input sm num" type="number" min="1" max={n} step="1" bind:value={nr} style="width:90px" /><span class="unit">von {n}</span></div>
    </div>
    <p class="small mut" style="margin:8px 0 0">
      {auswahl.length === 1 ? "Das gewählte Objekt kommt" : `Die ${auswahl.length} gewählten Objekte kommen`} in der Stickreihenfolge {vor ? "vor" : "hinter"} Objekt {gueltig ? nr : "…"}{gueltig && arbeit.objekte[nr - 1]?.name ? ` („${arbeit.objekte[nr - 1].name}“)` : ""}.
    </p>
  </form>
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
    <button class="btn btn-primary" disabled={!gueltig} onclick={ok}>Umordnen</button>
  {/snippet}
</Dialog>
