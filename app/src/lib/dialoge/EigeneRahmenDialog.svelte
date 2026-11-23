<script lang="ts">
  // Eigene Rahmen (Handbuch Kap. 6 S. 49–51 „Creating custom hoops“): Rechteck oder Oval nach eigenen Maßen anlegen,
  // ändern, löschen. Zum Gestalten frei; beim Senden nennt das Programm den Rahmen der artista 185, in den die Stiche
  // passen (Entscheidung des Nutzers 22.11.2026). Ablage im Datenordner (Kern `rahmen_speichern`/`rahmen_loeschen`).
  import { kern, type EigenerRahmenJson, type RahmenInfo } from "$kern/kern";
  import Dialog from "../Dialog.svelte";
  import { arbeit } from "../arbeit.svelte";
  import { zahl } from "../zahl";

  let { schliessen }: { schliessen: () => void } = $props();

  // svelte-ignore state_referenced_locally
  const start = arbeit.design.eigener_rahmen;
  let name = $state(start?.name ?? "Mein Rahmen");
  let form = $state<"rechteck" | "oval">(start?.form ?? "rechteck");
  let breite = $state(start?.breite ?? 200);
  let hoehe = $state(start?.hoehe ?? 200);
  let gerade = $state(start?.gerade ?? 0);
  let fehler = $state("");
  let laeuft = $state(false);

  const eigene = $derived(arbeit.rahmenListe.filter((r) => r.eigen));
  const ersetzt = $derived(eigene.some((r) => r.name.toLowerCase() === name.trim().toLowerCase()));
  const angabe = $derived<EigenerRahmenJson>({ name: name.trim(), form, breite, hoehe, gerade: form === "oval" ? gerade : 0 });
  const gueltig = $derived(!!name.trim() && breite >= 10 && breite <= 1000 && hoehe >= 10 && hoehe <= 1000 && (form === "rechteck" || (gerade >= 0 && gerade < hoehe)));

  // Vorschau wie im Kern (Rahmen::eigener): Oval = gerader Teil + Halbellipsen oben/unten
  const pfad = $derived.by(() => {
    if (!gueltig) return "";
    if (form === "rechteck") return `M${-breite / 2} ${-hoehe / 2}h${breite}v${hoehe}h${-breite}Z`;
    const a = breite / 2, g = gerade / 2, c = (hoehe - gerade) / 2;
    return `M${a} ${-g} A${a} ${c} 0 0 0 ${-a} ${-g} L${-a} ${g} A${a} ${c} 0 0 0 ${a} ${g} Z`;
  });
  // größter Rahmen der 185 als Vergleich
  const mega = $derived(arbeit.rahmenListe.filter((r) => !r.eigen).reduce<RahmenInfo | null>((m, r) => (!m || r.breite * r.hoehe > m.breite * m.hoehe ? r : m), null));
  const sicht = $derived(Math.max(breite, hoehe, mega?.breite ?? 0, mega?.hoehe ?? 0) / 2 + 10);

  function laden(r: RahmenInfo) {
    if (!r.eigen) return;
    name = r.eigen.name;
    form = r.eigen.form ?? "rechteck";
    breite = r.eigen.breite;
    hoehe = r.eigen.hoehe;
    gerade = r.eigen.gerade ?? 0;
  }

  async function speichern(nutzen: boolean) {
    fehler = "";
    laeuft = true;
    try {
      arbeit.rahmenListe = await kern("rahmen_speichern", { rahmen: angabe });
      if (nutzen) arbeit.rahmenWaehlen(angabe.name);
      else if (arbeit.design.eigener_rahmen?.name === angabe.name) arbeit.rahmenWaehlen(angabe.name);
      arbeit.zeige(`Rahmen „${angabe.name}“ gespeichert${nutzen ? " und gewählt" : ""}`, "ok");
      schliessen();
    } catch (e) {
      fehler = String((e as { meldung?: string })?.meldung ?? e);
    } finally {
      laeuft = false;
    }
  }

  async function loeschen(r: RahmenInfo) {
    fehler = "";
    try {
      arbeit.rahmenListe = await kern("rahmen_loeschen", { name: r.name });
      arbeit.zeige(`Rahmen „${r.name}“ gelöscht${arbeit.design.eigener_rahmen?.name === r.name ? " – das Design behält ihn, bis du einen anderen wählst" : ""}`, "info");
    } catch (e) {
      fehler = String((e as { meldung?: string })?.meldung ?? e);
    }
  }
</script>

<Dialog titel="Eigene Rahmen" breite={720} {schliessen}>
  {#snippet kopf()}
    <h2 class="hd">Eigene Rahmen</h2>
    <span class="badge">{eigene.length} gespeichert</span>
  {/snippet}
  <p class="mut small" style="margin:0 0 12px">
    Für Rahmen, die nicht in der Liste stehen (wie im Original „Create custom hoop“). Gestalten geht in jeder Größe. Die artista 185 kennt eigene Rahmen nicht: beim Übertragen nennt das Programm den Rahmen der 185, in den die Stiche passen – passt keiner, wird nicht übertragen.
  </p>
  <div class="raster">
    <div>
      <label class="zeile"><span>Name</span><input class="input" bind:value={name} aria-label="Name des Rahmens" /></label>
      <div class="zeile">
        <span>Form</span>
        <div class="seg seg-sm" role="radiogroup" aria-label="Form" style="display:flex">
          {#each [["rechteck", "Rechteck"], ["oval", "Oval"]] as [w, l] (w)}
            <label class="seg-opt" style="flex:1; justify-content:center"><input type="radio" name="rform" checked={form === w} onchange={() => (form = w as typeof form)} />{l}</label>
          {/each}
        </div>
      </div>
      <label class="zeile"><span>Breite</span><span class="ein"><input class="input" type="number" min="10" max="1000" step="1" bind:value={breite} aria-label="Breite in mm" /> mm</span></label>
      <label class="zeile"><span>{form === "oval" ? "Gesamthöhe" : "Höhe"}</span><span class="ein"><input class="input" type="number" min="10" max="1000" step="1" bind:value={hoehe} aria-label="Höhe in mm" /> mm</span></label>
      {#if form === "oval"}
        <label class="zeile" title="Höhe des geraden Mittelteils (im Original „Rectangle Height“); 0 = Ellipse"><span>gerader Teil</span><span class="ein"><input class="input" type="number" min="0" max={hoehe - 1} step="1" bind:value={gerade} aria-label="gerader Teil in mm" /> mm</span></label>
      {/if}
      {#if ersetzt}<p class="small" style="margin:6px 0 0 108px; color:var(--warn, #a60)">Einen Rahmen mit diesem Namen gibt es schon – er wird ersetzt.</p>{/if}
      {#if eigene.length}
        <div class="small mut" style="margin:14px 0 4px">Gespeichert</div>
        <ul class="liste">
          {#each eigene as r (r.name)}
            <li>
              <button class="btn btn-ghost l" onclick={() => laden(r)} title="zum Ändern laden">{r.name}<span class="mut small"> · {r.eigen?.form === "oval" ? "Oval" : "Rechteck"} {zahl(r.breite)} × {zahl(r.hoehe)} mm</span></button>
              <button class="btn btn-ghost" style="font-size:12px" onclick={() => loeschen(r)}>löschen</button>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
    <svg class="vor" viewBox="{-sicht} {-sicht} {2 * sicht} {2 * sicht}" aria-label="Vorschau des Rahmens">
      {#if mega}
        <path d={mega.form.map(([x, y], j) => `${j ? "L" : "M"}${x} ${-y}`).join("") + "Z"} fill="none" stroke="var(--color-text-muted, #888)" stroke-width={sicht / 150} stroke-dasharray="{sicht / 40} {sicht / 60}" />
        <text x={-mega.breite / 2} y={-mega.hoehe / 2 - sicht / 50} font-size={sicht / 16} fill="var(--color-text-muted, #888)">{mega.name.replace(/^artista /, "")}</text>
      {/if}
      {#if pfad}<path d={pfad} fill="var(--color-accent-100)" stroke="var(--color-accent)" stroke-width={sicht / 90} />{/if}
    </svg>
  </div>
  {#if fehler}<div class="errbox" style="margin-top:10px">{fehler}</div>{/if}
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Schließen</button>
    <button class="btn btn-secondary" disabled={laeuft || !gueltig} onclick={() => speichern(false)}>{ersetzt ? "Ersetzen" : "Speichern"}</button>
    <button class="btn btn-primary" disabled={laeuft || !gueltig} onclick={() => speichern(true)}>Speichern und wählen</button>
  {/snippet}
</Dialog>

<style>
  .raster {
    display: grid;
    grid-template-columns: 1fr 260px;
    gap: 16px;
    align-items: start;
  }
  .zeile {
    display: grid;
    grid-template-columns: 100px 1fr;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
  }
  .ein {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .ein input {
    width: 100px;
  }
  .liste {
    list-style: none;
    margin: 0;
    padding: 0;
    max-height: 160px;
    overflow: auto;
  }
  .liste li {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .liste .l {
    text-align: left;
    font-size: 13px;
  }
  .vor {
    width: 260px;
    height: 260px;
    background: var(--surface-2, rgba(0, 0, 0, 0.03));
    border-radius: 8px;
  }
</style>
