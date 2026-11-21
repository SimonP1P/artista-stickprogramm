<script lang="ts">
  // Eine Zeile des Inspektors nach dem Schema des Kerns (UEBERGABE §2 „Feldzeile“): Beschriftung links (118 px), Wert
  // rechts, Einheit, Hilfe darunter. Leere Eingabe = Feld entfernen (Standard des Stils/Kerns gilt).
  import type { Feld } from "../schema.svelte";
  import { lesen, zahl } from "../zahl";

  let {
    feld,
    wert,
    setzen,
    beschriftung,
    x = false,
  }: {
    feld: Feld;
    wert: unknown;
    setzen: (v: unknown) => void;
    beschriftung?: string;
    x?: boolean;
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
    initialen: "Initialen",
    dreifach: "Dreifach",
    text: "Text",
  };
  const titel = $derived(beschriftung ?? NAMEN[feld.name] ?? feld.name);
  const id = $derived(`f-${feld.name}`);

  const unterlageWert = $derived(
    Array.isArray(wert) ? `[${wert[0] ?? ""}]` : typeof wert === "string" ? wert : "",
  );
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
  {#if feld.typ !== "komplex" && feld.beschreibung && feld.beschreibung.length <= 110}<div class="fhelp">{feld.beschreibung}</div>{/if}
</div>
