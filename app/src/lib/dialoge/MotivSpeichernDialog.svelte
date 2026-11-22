<script lang="ts">
  // Ausgewählte Objekte als Motiv speichern (Handbuch Kap. 25 „Create Pattern“, Settings-Menü des Originals): eigener
  // Motivsatz (neu oder vorhanden) und Name des Motivs; der Kern (`motiv_speichern`) wandelt die Objekte in Knoten um
  // und legt den Satz im Datenordner ab. Bezugspunkte: linke und rechte Kante auf halber Höhe (eigene Festlegung –
  // im Original klickt man sie an); sie bestimmen den Wiederholabstand der Motivlinie.
  import { kern, type DesignJson, type MotivInfo, type MotivsatzInfo } from "$kern/kern";
  import Dialog from "../Dialog.svelte";
  import { arbeit } from "../arbeit.svelte";

  let { schliessen }: { schliessen: () => void } = $props();

  let saetze = $state.raw<MotivsatzInfo[]>([]);
  let satz = $state("Meine Motive");
  let name = $state("");
  let vorhanden = $state.raw<MotivInfo[]>([]);
  let fehler = $state("");
  let laeuft = $state(false);
  const eigene = $derived(saetze.filter((s) => s.eigen));
  const ersetzt = $derived(vorhanden.some((m) => m.name === name.trim()));
  // svelte-ignore state_referenced_locally
  const auswahl = [...arbeit.auswahl].sort((a, b) => a - b);

  $effect(() => {
    kern("motive_liste", {})
      .then((l) => {
        saetze = l;
        const e = l.filter((s) => s.eigen);
        if (e.length) satz = e[0].name;
      })
      .catch((e) => (fehler = String(e?.meldung ?? e)));
  });
  $effect(() => {
    const s = satz.trim();
    vorhanden = [];
    if (!eigene.some((x) => x.name === s)) return;
    kern("motivsatz_motive", { satz: s })
      .then((m) => {
        vorhanden = m;
        if (!name) name = `Motiv ${m.length + 1}`;
      })
      .catch(() => {});
  });
  $effect(() => {
    if (!name && !eigene.some((x) => x.name === satz.trim())) name = "Motiv 1";
  });

  async function speichern() {
    fehler = "";
    laeuft = true;
    try {
      const i = await kern("motiv_speichern", {
        design: $state.snapshot(arbeit.design) as DesignJson,
        objekte: auswahl,
        satz: satz.trim(),
        name: name.trim(),
      });
      arbeit.zeige(`Motiv „${name.trim()}“ in „${i.name}“ gespeichert – wählbar bei Motivlinie und Motivfüllung`, "info");
      schliessen();
    } catch (e) {
      const x = e as { meldung?: string; objekt?: number | null };
      fehler = (x.objekt != null ? `Objekt ${x.objekt + 1}: ` : "") + String(x.meldung ?? e);
    } finally {
      laeuft = false;
    }
  }
</script>

<Dialog titel="Als Motiv speichern" breite={520} {schliessen}>
  {#snippet kopf()}
    <h2 class="hd">Als Motiv speichern</h2>
    <span class="badge">{auswahl.length} {auswahl.length === 1 ? "Objekt" : "Objekte"}</span>
  {/snippet}
  <p class="mut small" style="margin:0 0 12px">
    Die gewählten Objekte (Läufe, Satinsäulen, Flächen) werden ein Motiv für <b>Motivlinie</b> und <b>Motivfüllung</b>. Wie im Original: das Motiv von links nach rechts digitalisieren, damit sich die Wiederholungen nicht kreuzen. Linke und rechte Kante bestimmen den Abstand der Wiederholungen.
  </p>
  <label class="zeile">
    <span>Motivsatz</span>
    <input class="input" list="eigene-saetze" bind:value={satz} aria-label="Motivsatz" />
    <datalist id="eigene-saetze">{#each eigene as s (s.datei)}<option value={s.name}></option>{/each}</datalist>
  </label>
  <p class="mut small" style="margin:2px 0 10px 128px">{eigene.some((x) => x.name === satz.trim()) ? `vorhandener Satz · ${vorhanden.length} Motive` : "neuer Satz"}</p>
  <label class="zeile">
    <span>Name des Motivs</span>
    <input class="input" bind:value={name} aria-label="Name des Motivs" />
  </label>
  {#if ersetzt}<p class="small" style="margin:6px 0 0 128px; color:var(--warn, #a60)">Ein Motiv mit diesem Namen gibt es schon – es wird ersetzt.</p>{/if}
  {#if fehler}<div class="errbox" style="margin-top:10px">{fehler}</div>{/if}
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
    <button class="btn btn-primary" disabled={laeuft || !satz.trim() || !name.trim() || !auswahl.length} onclick={speichern}>{ersetzt ? "Ersetzen" : "Speichern"}</button>
  {/snippet}
</Dialog>

<style>
  .zeile {
    display: grid;
    grid-template-columns: 120px 1fr;
    align-items: center;
    gap: 8px;
  }
</style>
