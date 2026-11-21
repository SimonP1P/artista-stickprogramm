<script lang="ts">
  // Statusleiste: Kennzahlen aus `Ergebnis.statistik`, Datengröße gegen die Grenze der Maschine, Hinweise mit
  // Objektnummer (Klick → Objekt wählen). Stickzeit ist eine gekennzeichnete Schätzung (keine Quelle, UEBERGABE §2).
  import { arbeit } from "./arbeit.svelte";
  import Icon from "./Icon.svelte";
  import { IC } from "./icons";
  import { ganz, zahl, zahlFest } from "./zahl";

  /** Schätzung 0,4 s je Stich (eigene Annahme des Entwurfs, keine Quelle für die artista 185) */
  const SEKUNDEN_JE_STICH = 0.4;

  let offen = $state(false);
  const e = $derived(arbeit.ergebnis);

  type Eintrag = { text: string; art: "err" | "warn" | "ok"; objekt: number | null };
  const eintraege = $derived.by((): Eintrag[] => {
    const l: Eintrag[] = [];
    if (arbeit.fehler) l.push({ text: arbeit.fehler.meldung, art: "err", objekt: arbeit.fehler.objekt ?? null });
    if (e && e.daten_bytes >= e.daten_grenze)
      l.push({
        text: `Design zu groß für die Maschine: ${ganz(e.daten_bytes / 1024)} KB von ${ganz(e.daten_grenze / 1024)} KB`,
        art: "err",
        objekt: null,
      });
    const hinweise = arbeit.fehler ? (arbeit.pruefung?.hinweise ?? []) : (e?.hinweise ?? []);
    for (const h of hinweise)
      l.push({
        text: h.meldung,
        art: h.art === "ausserhalb" ? "err" : h.art === "schrift_step" ? "ok" : "warn",
        objekt: h.objekt ?? null,
      });
    return l;
  });
  const fehlerZahl = $derived(eintraege.filter((x) => x.art === "err").length);

  function zeigen(o: number | null) {
    if (o !== null) arbeit.waehlen(o);
    offen = false;
  }
</script>

<div class="sbar tab">
  {#if e}
    <span><span class="k">Stiche</span> {ganz(e.statistik.stiche)}</span>
    <span><span class="k">Sprünge</span> {ganz(e.statistik.spruenge)}</span>
    <span><span class="k">Farbwechsel</span> {ganz(e.statistik.farbwechsel)}</span>
    <span>{zahlFest(e.statistik.breite)} × {zahlFest(e.statistik.hoehe)} mm</span>
    <span title="Stickzeit geschätzt: {zahl(SEKUNDEN_JE_STICH)} s je Stich – keine Quelle für die artista 185">
      ≈ {ganz((e.statistik.stiche * SEKUNDEN_JE_STICH) / 60)} min*
    </span>
    <span title="Datengröße für die Maschine (Stichdaten + Anhang) / Kapazität der artista 185">
      {ganz(e.daten_bytes / 1024)} / {ganz(e.daten_grenze / 1024)} KB
    </span>
    <span class="k" title="* Stickzeit geschätzt (0,4 s je Stich, keine Quelle für die artista 185)">* Schätzung</span>
  {:else}
    <span class="k">{arbeit.rechnet ? "Berechne …" : "Noch nichts berechnet"}</span>
  {/if}
  <span style="flex:1"></span>
  {#if eintraege.length}
    <button class="hchip" class:err={fehlerZahl > 0} aria-expanded={offen} onclick={() => (offen = !offen)}>
      <Icon d={IC.warnung} groesse="s" />
      {fehlerZahl ? `${fehlerZahl} Fehler` : ""}{fehlerZahl && eintraege.length > fehlerZahl ? " · " : ""}{eintraege.length > fehlerZahl
        ? `${eintraege.length - fehlerZahl} ${eintraege.length - fehlerZahl === 1 ? "Hinweis" : "Hinweise"}`
        : ""}
    </button>
  {:else}
    <span class="okchip"><Icon d={IC.haken} groesse="s" />Keine Hinweise</span>
  {/if}
</div>

{#if offen}
  <div class="shade" role="presentation" onclick={() => (offen = false)}></div>
  <div class="menu" role="dialog" aria-label="Hinweise" style="right:10px; bottom:34px; width:430px; padding:0">
    <div class="mh" style="padding:9px 12px 6px; border-bottom:1px solid var(--line)">Fehler und Hinweise</div>
    {#each eintraege as h, k (k)}
      <div style="display:flex; gap:8px; align-items:flex-start; padding:9px 12px; border-bottom:1px solid color-mix(in srgb,var(--color-text) 7%,transparent)">
        <span class="{h.art}box" style="padding:2px; border:0; background:none"><Icon d={IC.info} /></span>
        <span style="flex:1; font-size:13px">{h.text}</span>
        {#if h.objekt !== null}
          <button class="btn btn-ghost" style="font-size:12.5px" onclick={() => zeigen(h.objekt)}>Zeigen</button>
        {/if}
      </div>
    {/each}
  </div>
{/if}
