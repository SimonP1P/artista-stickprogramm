<script lang="ts">
  // Vorlagen (Handbuch Kap. 9 „Managing design templates“): „neu“ = neues Design aus einer Vorlage (Normal = Vorlage
  // von „Neu“), „speichern“ = Design als Vorlage („Save As … Templates (AMT)“), „werte“ = nur die Einzelwerte in eine
  // Vorlage („Save“ im Dialog Object Properties). Normal löschen = Werkseinstellung („Revert“). Kern: `vorlagen_liste`,
  // `vorlage_speichern`, `vorlage_neu`, `vorlage_loeschen`; Ablage im Datenordner.
  import { kern, type DesignJson, type VorlageInfo } from "$kern/kern";
  import Dialog from "../Dialog.svelte";
  import { arbeit } from "../arbeit.svelte";
  import { stoffName } from "../listen.svelte";
  import { neuAusVorlage } from "../dateien.svelte";

  let { modus, schliessen }: { modus: "neu" | "speichern" | "werte"; schliessen: () => void } = $props();

  let liste = $state.raw<VorlageInfo[]>([]);
  let gewaehlt = $state("Normal");
  // svelte-ignore state_referenced_locally
  let name = $state(modus === "werte" ? (arbeit.design.vorlage ?? "Normal") : arbeit.design.vorlage && arbeit.design.vorlage !== "Normal" ? arbeit.design.vorlage : "");
  // svelte-ignore state_referenced_locally
  let objekte = $state((arbeit.design.objekte?.length ?? 0) > 0);
  let fehler = $state("");
  let laeuft = $state(false);
  let loeschenFrage = $state<string | null>(null);

  const werteZahl = $derived.by(() => {
    const zaehlen = (v: unknown): number =>
      v && typeof v === "object" ? Object.values(v as Record<string, unknown>).reduce<number>((s, x) => s + zaehlen(x), 0) : v === undefined || v === null ? 0 : 1;
    return zaehlen(arbeit.design.einstellungen);
  });
  const vorhanden = $derived(liste.find((v) => v.name.toLowerCase() === name.trim().toLowerCase() && !v.werk));

  $effect(() => {
    kern("vorlagen_liste", {})
      .then((l) => (liste = l))
      .catch((e) => (fehler = String(e?.meldung ?? e)));
  });

  const beschreibung = (v: VorlageInfo) =>
    [
      v.werk ? "Werkseinstellung" : `${v.werte} ${v.werte === 1 ? "Einzelwert" : "Einzelwerte"}`,
      v.stoff && !v.werk ? stoffName(v.stoff) : "",
      v.objekte ? `${v.objekte} ${v.objekte === 1 ? "Objekt" : "Objekte"}` : "",
    ]
      .filter(Boolean)
      .join(" · ");

  async function speichern() {
    fehler = "";
    laeuft = true;
    try {
      liste = await kern("vorlage_speichern", {
        design: $state.snapshot(arbeit.design) as DesignJson,
        name: name.trim(),
        umfang: modus === "werte" ? "werte" : "ganz",
        objekte,
      });
      // das Design merkt sich seine Vorlage
      if (arbeit.design.vorlage !== name.trim()) arbeit.aendern((d) => (d.vorlage = name.trim()));
      arbeit.zeige(
        modus === "werte"
          ? `${werteZahl} ${werteZahl === 1 ? "Einzelwert" : "Einzelwerte"} in die Vorlage „${name.trim()}“ gespeichert – gilt für neue Designs aus dieser Vorlage`
          : `Vorlage „${name.trim()}“ gespeichert`,
        "ok",
      );
      schliessen();
    } catch (e) {
      fehler = String((e as { meldung?: string })?.meldung ?? e);
    } finally {
      laeuft = false;
    }
  }

  async function loeschen(n: string) {
    fehler = "";
    try {
      liste = await kern("vorlage_loeschen", { name: n });
      arbeit.zeige(n === "Normal" ? "Vorlage Normal auf Werkseinstellung zurückgesetzt" : `Vorlage „${n}“ gelöscht`, "info");
      if (gewaehlt === n && n !== "Normal") gewaehlt = "Normal";
    } catch (e) {
      fehler = String((e as { meldung?: string })?.meldung ?? e);
    } finally {
      loeschenFrage = null;
    }
  }

  async function neu() {
    try {
      await neuAusVorlage(gewaehlt);
      schliessen();
    } catch (e) {
      fehler = String((e as { meldung?: string })?.meldung ?? e);
    }
  }

  const titel = $derived(modus === "neu" ? "Neu aus Vorlage" : modus === "werte" ? "Werte in Vorlage speichern" : "Als Vorlage speichern");
</script>

<Dialog {titel} breite={560} {schliessen}>
  {#snippet kopf()}
    <h2 class="hd">{titel}</h2>
    <span class="badge">{liste.length} {liste.length === 1 ? "Vorlage" : "Vorlagen"}</span>
  {/snippet}
  {#if modus === "neu"}
    <p class="mut small" style="margin:0 0 10px">Ein neues Design beginnt mit den Werten, dem Stoff und den Objekten der Vorlage. „Normal“ ist die Vorlage für „Datei → Neu“.</p>
    <div class="liste" role="listbox" aria-label="Vorlagen">
      {#each liste as v (v.name)}
        <div class="zeile" class:on={gewaehlt === v.name}>
          <button class="wahl" role="option" aria-selected={gewaehlt === v.name} onclick={() => (gewaehlt = v.name)} ondblclick={() => ((gewaehlt = v.name), neu())}>
            <b style="font-weight:500">{v.name}</b>{#if v.standard}<span class="tag tag-accent" style="margin-left:6px">für „Neu“</span>{/if}
            <span class="mut small" style="display:block">{beschreibung(v)}</span>
          </button>
          {#if !v.werk}
            {#if loeschenFrage === v.name}
              <span class="small">{v.standard ? "Werkseinstellung?" : "Löschen?"}</span>
              <button class="btn btn-ghost" style="font-size:12px" onclick={() => loeschen(v.name)}>Ja</button>
              <button class="btn btn-ghost" style="font-size:12px" onclick={() => (loeschenFrage = null)}>Nein</button>
            {:else}
              <button class="btn btn-ghost" style="font-size:12px" title={v.standard ? "Normal auf die Werkseinstellung zurücksetzen (Revert)" : "Vorlage löschen"} onclick={() => (loeschenFrage = v.name)}>{v.standard ? "Werkseinstellung" : "löschen"}</button>
            {/if}
          {/if}
        </div>
      {/each}
    </div>
  {:else}
    <p class="mut small" style="margin:0 0 10px">
      {#if modus === "werte"}
        Nur die Einzelwerte des Designs ({werteZahl}) kommen in die Vorlage – ihre Objekte, Rahmen und Stoff bleiben. Sie gelten für neue Designs aus dieser Vorlage; dieses Design ändert sich nicht.
      {:else}
        Das Design wird eine Vorlage: Einzelwerte ({werteZahl}), Rahmen, Stoff, Garnkarte{objekte ? " und Objekte (z. B. ein Mustertext zum Überschreiben)" : ""}. Unter dem Namen „Normal“ gilt sie für „Datei → Neu“.
      {/if}
    </p>
    <label class="feld">
      <span>Vorlage</span>
      <input class="input" list="vorlagen-namen" bind:value={name} aria-label="Name der Vorlage" placeholder="z. B. Mützen, Frottee" />
      <datalist id="vorlagen-namen">{#each liste as v (v.name)}<option value={v.name}></option>{/each}</datalist>
    </label>
    {#if modus === "speichern"}
      <label class="small" style="display:flex; gap:8px; align-items:center; margin:8px 0 0 108px"><input type="checkbox" bind:checked={objekte} />Objekte mitspeichern ({arbeit.design.objekte?.length ?? 0})</label>
    {/if}
    {#if vorhanden}<p class="small" style="margin:6px 0 0 108px; color:var(--warn, #a60)">{modus === "werte" ? `Die Einzelwerte der Vorlage „${vorhanden.name}“ werden ersetzt.` : `Die Vorlage „${vorhanden.name}“ wird ersetzt.`}</p>{/if}
  {/if}
  {#if fehler}<div class="errbox" style="margin-top:10px">{fehler}</div>{/if}
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
    {#if modus === "neu"}
      <button class="btn btn-primary" disabled={!gewaehlt} onclick={neu}>Neues Design</button>
    {:else}
      <button class="btn btn-primary" disabled={laeuft || !name.trim()} onclick={speichern}>{vorhanden ? "Ersetzen" : "Speichern"}</button>
    {/if}
  {/snippet}
</Dialog>

<style>
  .liste {
    display: flex;
    flex-direction: column;
    gap: 4px;
    max-height: 340px;
    overflow: auto;
  }
  .zeile {
    display: flex;
    align-items: center;
    gap: 6px;
    border: 1px solid var(--line);
    border-radius: 6px;
    padding: 2px 6px 2px 2px;
  }
  .zeile.on {
    border-color: var(--color-accent);
    background: var(--color-accent-100);
  }
  .wahl {
    flex: 1;
    text-align: left;
    background: none;
    border: 0;
    padding: 6px 8px;
    color: inherit;
    cursor: pointer;
    font: inherit;
  }
  .feld {
    display: grid;
    grid-template-columns: 100px 1fr;
    align-items: center;
    gap: 8px;
  }
</style>
