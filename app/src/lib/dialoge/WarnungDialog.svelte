<script lang="ts">
  // Warnung nach dem Umordnen (`ReihenfolgeErgebnis.vertauscht` [oben, unten], bisherige Nummern): ein Objekt liegt jetzt
  // unter einer Fläche, über der es lag. „Rückgängig“ stellt die alte Reihenfolge her.
  import { arbeit } from "../arbeit.svelte";
  import Dialog from "../Dialog.svelte";

  let { vertauscht, namen, vorher, schliessen }: { vertauscht: number[][]; namen: string[]; vorher: string; schliessen: () => void } = $props();
</script>

<Dialog titel="Achtung: Überdeckung geändert" breite={520} {schliessen}>
  {#snippet kopf()}<h2 class="hd" style="color:var(--warn)">⚠ Achtung: Überdeckung geändert</h2>{/snippet}
  <p style="margin:0 0 8px">Nach dem Umordnen liegen Objekte unter einer Fläche, über der sie vorher lagen:</p>
  <ul style="margin:0 0 10px; padding-left:20px">
    {#each vertauscht as [oben, unten], k (k)}
      <li>„{namen[oben] ?? `Nr. ${oben + 1}`}“ lag über „{namen[unten] ?? `Nr. ${unten + 1}`}“ und liegt jetzt darunter.</li>
    {/each}
  </ul>
  <p class="mut small" style="margin:0">Beispiel: die Kontur wird von der Füllung verdeckt. Rückgängig stellt die alte Reihenfolge her.</p>
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Trotzdem behalten</button>
    <button class="btn btn-primary" onclick={() => (arbeit.wiederherstellen(vorher), schliessen())}>Rückgängig</button>
  {/snippet}
</Dialog>
