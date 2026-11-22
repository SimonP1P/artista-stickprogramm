<script lang="ts">
  // Eine Zeile des Inspektors nach dem Schema des Kerns (UEBERGABE §2 „Feldzeile“): Beschriftung links (118 px), Wert
  // rechts, Einheit, Hilfe darunter. Leere Eingabe = Feld entfernen (Standard des Stils/Kerns gilt).
  import type { Feld } from "../schema.svelte";
  import type { WirksamerWert } from "$kern/kern";
  import { lesen, zahl } from "../zahl";
  import { unterlageAusHaken, unterlageHaken } from "../einfuegen";

  let {
    feld,
    wert,
    setzen,
    beschriftung,
    x = false,
    haken = false,
    wirksam,
    geladen = false,
  }: {
    feld: Feld;
    wert: unknown;
    setzen: (v: unknown) => void;
    beschriftung?: string;
    x?: boolean;
    /** Unterlage als zwei Häkchen Mittellauf/Zickzack (Texte, Monogramme) statt Auswahlliste */
    haken?: boolean;
    /** Wert, mit dem der Kern bei leerem Feld rechnet (`objekt_werte`) – steht grau im Feld */
    wirksam?: WirksamerWert;
    /** wirksame Werte des Objekts sind da: ein Feld ohne Wert gilt dann gerade nicht (z. B. Stichwinkel ohne Muster) */
    geladen?: boolean;
  } = $props();
  const HERKUNFT: Record<string, string> = {
    stil: "Stoff/Stil",
    alphabet: "Alphabet",
    muster: "Füllmuster",
    motivsatz: "Motivsatz",
    vorlage: "Vorlage des Originals",
    schrift: "Schrift",
  };
  /** Anzeige des wirksamen Werts (Zahl, an/aus oder Text des Kerns) */
  const wirkText = $derived.by(() => {
    if (!wirksam) return null;
    if (wirksam.text) return wirksam.text;
    const w = wirksam.wert;
    return typeof w === "number" ? zahl(w, 3) : typeof w === "boolean" ? (w ? "an" : "aus") : w == null ? null : String(w);
  });

  const NAMEN: Record<string, string> = {
    abstand: "Abstand",
    breite: "Breite",
    hoehe: "Höhe",
    laenge: "Länge",
    winkel: "Stichwinkel",
    stichart: "Stichart",
    unterlage: "Unterlage",
    versatz: "Lage",
    geschlossen: "Geschlossen",
    kurzstiche: "Kurzstiche",
    faktor: "Faktor",
    knickwinkel: "Am Knick teilen",
    ueberlappung: "Überlappung",
    groesse: "Mustergröße",
    abdeckung: "Abdeckung",
    stil: "Stoff zuschneiden",
    heft_breite: "Heftstich-Breite",
    heft_abstand: "Heftstich-Abstand",
    platz_laenge: "Platzierung: Stichlänge",
    typ: "Typ",
    kursiv: "Kursiv",
    ausrichtung: "Ausrichtung",
    wortabstand: "Wortabstand",
    zeilenabstand: "Zeilenabstand",
    rahmen: "Rahmen",
    rahmen_farbe: "Rahmenfarbe",
    rahmen_groesse: "Größe Rahmen",
    vorlage: "Rahmen",
    rahmen_farben: "Garn Rahmen",
    initialen: "Initialen",
    spaltenabstand: "Spaltenabstand",
    reihenabstand: "Reihenabstand",
    dreifach: "Dreifach",
    text: "Text",
    alphabet: "Alphabet",
    muster: "Füllmuster",
    step_muster: "Step-Muster",
    schrift: "Schrift",
    motiv: "Motiv",
    spiegeln: "Spiegeln",
    aussparen: "Aussparen",
    knickwinkel_: "Knickwinkel",
    stich: "Stichart",
    teilabdeckung: "Teil-Abdeckung",
    staerke: "Stärke",
    striche: "Striche",
  };
  /** Vorlagewerte, die nicht im Schema stehen (Kern: schrift.rs WORTABSTAND_ANTEIL 0,25, ZEILENABSTAND_ANTEIL 0,6;
   *  Mustergröße 100 %) – Anzeige statt „Stil“. Werte nach Stil/Stoff liefert der Kern noch nicht (offen). */
  const VORGABE: Record<string, number> = { wortabstand: 25, zeilenabstand: 60, groesse: 100 };
  /** Erklärung für das „?“: Beschreibung aus dem Schema ohne JSON-Schreibweise */
  const hilfe = $derived(
    [
      (feld.beschreibung ?? "").replace(/`([^`]*)`/g, "„$1“").replace(/\\n/g, "Zeilenumbruch").trim(),
      wirksam && wirkText ? `Leer = ${wirkText} (${HERKUNFT[wirksam.herkunft] ?? wirksam.herkunft}).` : "",
    ]
      .filter(Boolean)
      .join(" "),
  );
  const titel = $derived(beschriftung ?? NAMEN[feld.name] ?? feld.name);
  const id = $derived(`f-${feld.name}`);

  const unterlageWert = $derived(
    Array.isArray(wert) ? `[${wert[0] ?? ""}]` : typeof wert === "string" ? wert : "",
  );
  const haken_ = $derived(unterlageHaken(wert));
  function hakenSetzen(art: "mittellauf" | "zickzack", an: boolean) {
    setzen(unterlageAusHaken({ ...haken_, [art]: an }));
  }
  function unterlageSetzen(v: string) {
    if (v === "") setzen(undefined);
    else if (v.startsWith("[")) setzen([v.slice(1, -1)]);
    else setzen(v);
  }
</script>

<div class="frow">
  <label class="fl" for={id}>{titel}{#if hilfe}<span class="frage" title={hilfe} aria-label={hilfe}>?</span>{/if}</label>
  <div class="fc">
    {#if feld.typ === "zahl" || feld.typ === "ganz"}
      <input
        {id}
        class="input sm num"
        class:wirk={!!wirkText}
        style="width:{wirkText && wirkText.length > 7 ? 130 : 90}px"
        inputmode="decimal"
        value={typeof wert === "number" ? zahl(wert, 3) : ""}
        placeholder={wirkText ?? (typeof feld.standard === "number" ? zahl(feld.standard, 3) : feld.name in VORGABE ? zahl(VORGABE[feld.name], 3) : geladen ? "–" : "Stil")}
        onchange={(e) => {
          const t = (e.currentTarget as HTMLInputElement).value.trim();
          if (t === "") return setzen(undefined);
          const v = lesen(t);
          if (v !== null) setzen(feld.typ === "ganz" ? Math.round(v) : v);
        }}
      />
      <span class="unit">{feld.einheit}</span>
    {:else if feld.typ === "bool"}
      {#if feld.leer}
        <select {id} class="input sm" value={wert === true ? "an" : wert === false ? "aus" : ""} onchange={(e) => {
          const v = (e.currentTarget as HTMLSelectElement).value;
          setzen(v === "" ? undefined : v === "an");
        }}>
          <option value="">wie Stil{wirkText ? ` (${wirkText})` : ""}</option><option value="an">an</option><option value="aus">aus</option>
        </select>
      {:else}
        <input {id} type="checkbox" checked={wert === true || (wert === undefined && feld.standard === true)} onchange={(e) => setzen((e.currentTarget as HTMLInputElement).checked)} style="width:16px; height:16px" />
      {/if}
    {:else if feld.typ === "auswahl"}
      <select {id} class="input sm" value={typeof wert === "string" ? wert : ""} onchange={(e) => {
        const v = (e.currentTarget as HTMLSelectElement).value;
        setzen(v === "" ? undefined : v);
      }}>
        <option value="">{feld.standard ? `Standard (${feld.standard})` : "Standard"}</option>
        {#each feld.werte as w (w.wert)}<option value={w.wert}>{w.text}</option>{/each}
      </select>
    {:else if feld.typ === "unterlage" && haken}
      <div class="haken" title="Unterlage der Buchstaben: Mittellauf in der Mitte jeder Säule, Zickzack quer darüber. Ohne Angabe wie Stil „Schrift“ des Originals (nur Zickzack).">
        <label class="chk"><input type="checkbox" checked={haken_.mittellauf} onchange={(e) => hakenSetzen("mittellauf", (e.currentTarget as HTMLInputElement).checked)} /><span>Mittellauf</span></label>
        <label class="chk"><input type="checkbox" checked={haken_.zickzack} onchange={(e) => hakenSetzen("zickzack", (e.currentTarget as HTMLInputElement).checked)} /><span>Zickzack</span></label>
        {#if haken_.andere.length}<span class="mut small">+ {haken_.andere.join(", ")}</span>{/if}
      </div>
    {:else if feld.typ === "unterlage"}
      <select {id} class="input sm" value={unterlageWert} onchange={(e) => unterlageSetzen((e.currentTarget as HTMLSelectElement).value)}>
        <option value="">wie Stil (auto)</option>
        {#each feld.werte as w (w.wert)}<option value={w.wert}>{w.text}</option>{/each}
        {#if Array.isArray(wert) && wert.length > 1}<option value={unterlageWert}>{wert.join(" + ")} (JSON)</option>{/if}
      </select>
    {:else if feld.typ === "text"}
      <input {id} class="input sm" value={typeof wert === "string" ? wert : ""} onchange={(e) => {
        const v = (e.currentTarget as HTMLInputElement).value;
        setzen(v === "" ? undefined : v);
      }} />
    {:else}
      <span class="mut small">im Reiter JSON bzw. in der Formansicht</span>
    {/if}
  </div>
</div>

<style>
  /* kleines „?“ hinter der Beschriftung: Erklärung beim Darüberfahren (schnelle Tooltips, Tooltip.svelte) */
  .frage {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 15px;
    height: 15px;
    margin-left: 5px;
    border: 1px solid currentColor;
    border-radius: 50%;
    font-size: 10px;
    font-weight: 600;
    line-height: 1;
    opacity: 0.5;
    cursor: help;
    vertical-align: 1px;
  }
  .frage:hover {
    opacity: 1;
  }
  /* wirksamer Wert im leeren Feld: deutlich lesbar, aber heller als ein eingetragener Wert */
  .wirk::placeholder {
    color: color-mix(in srgb, var(--color-text) 62%, transparent);
    opacity: 1;
  }
  .haken {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 6px 16px;
  }
</style>
