<script lang="ts">
  // Eine Zeile des Inspektors nach dem Schema des Kerns (UEBERGABE §2 „Feldzeile“): Beschriftung links (118 px), Wert
  // rechts, Einheit, Hilfe darunter. Leere Eingabe = Feld entfernen (Standard des Stils/Kerns gilt).
  import type { Feld } from "../schema.svelte";
  import { lesen, zahl } from "../zahl";
  import { unterlageAusHaken, unterlageHaken } from "../einfuegen";

  let {
    feld,
    wert,
    setzen,
    beschriftung,
    x = false,
    haken = false,
  }: {
    feld: Feld;
    wert: unknown;
    setzen: (v: unknown) => void;
    beschriftung?: string;
    x?: boolean;
    /** Unterlage als zwei Häkchen Mittellauf/Zickzack (Texte, Monogramme) statt Auswahlliste */
    haken?: boolean;
  } = $props();

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
    dreifach: "Dreifach",
    text: "Text",
  };
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
  <label class="fl" for={id} title={feld.beschreibung}>{titel}{#if x}<span class="xt" title="nur im Expertenmodus">X</span>{/if}</label>
  <div class="fc">
    {#if feld.typ === "zahl" || feld.typ === "ganz"}
      <input
        {id}
        class="input sm num"
        style="width:90px"
        inputmode="decimal"
        value={typeof wert === "number" ? zahl(wert, 3) : ""}
        placeholder={typeof feld.standard === "number" ? zahl(feld.standard, 3) : "Stil"}
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
          <option value="">wie Stil</option><option value="an">an</option><option value="aus">aus</option>
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
  {#if feld.typ !== "komplex" && !(haken && feld.typ === "unterlage") && feld.beschreibung && feld.beschreibung.length <= 110}<div class="fhelp">{feld.beschreibung}</div>{/if}
</div>

<style>
  .haken {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 6px 16px;
  }
</style>
