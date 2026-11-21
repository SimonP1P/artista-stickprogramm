<script lang="ts">
  // Reiter „Einzelwerte“ (UEBERGABE §3): Werte des Parameterkatalogs (`parameter_katalog`) unter `einstellungen` – für das
  // ganze Design oder je Objekt (4b.16). Herkunft: Standard · Stoff · Design · geändert; Zurücksetzen löscht den Schlüssel.
  import { kern, type Parameter } from "$kern/kern";
  import { arbeit } from "../arbeit.svelte";
  import Icon from "../Icon.svelte";
  import { IC } from "../icons";
  import { lesen, zahl } from "../zahl";

  let katalog = $state.raw<Parameter[]>([]);
  let ebene = $state<"design" | "objekt">("design");
  let alle = $state(false);

  $effect(() => {
    kern("parameter_katalog", {}).then((k) => (katalog = k)).catch(() => {});
  });

  const i = $derived(arbeit.auswahl.length === 1 ? arbeit.auswahl[0] : -1);
  const o = $derived(i >= 0 ? arbeit.objekte[i] : undefined);
  $effect(() => {
    if (ebene === "objekt" && !o) ebene = "design";
  });

  type Baum = Record<string, unknown>;
  function holen(b: unknown, pfad: string[]): unknown {
    let x = b;
    for (const p of pfad) x = x && typeof x === "object" ? (x as Baum)[p] : undefined;
    return x;
  }
  const teile = (p: Parameter) => (p.json_design ?? "").split(".").slice(1);

  const liste = $derived(
    katalog.filter(
      (p) =>
        p.json_design?.startsWith("einstellungen.") &&
        p.json_design !== "einstellungen.stoff" &&
        (alle || p.modus !== "automatisch") &&
        (ebene === "design" ? p.ebene === "beide" || p.ebene === "design" : p.ebene === "beide") &&
        (ebene === "design" || !o || p.gilt_fuer.length === 0 || p.gilt_fuer.includes(o.art)),
    ),
  );
  const gruppen = $derived([...new Set(liste.map((p) => p.gruppe))]);

  function wertUndHerkunft(p: Parameter) {
    const pfad = teile(p);
    const d = holen(arbeit.design.einstellungen, pfad);
    if (ebene === "objekt" && o) {
      const w = holen(o.einstellungen, pfad);
      if (w !== undefined) return { wert: w, herkunft: "geändert", erbt: d ?? p.standard };
      if (d !== undefined) return { wert: undefined, herkunft: "Design", erbt: d };
    } else if (d !== undefined) return { wert: d, herkunft: "geändert", erbt: p.standard };
    return { wert: undefined, herkunft: p.stoff ? "Stoff" : "Standard", erbt: p.standard };
  }

  function setzen(p: Parameter, v: unknown) {
    const pfad = teile(p);
    const nr = i;
    const obj = ebene === "objekt";
    arbeit.aendern((d) => {
      const wurzel = (obj ? d.objekte![nr] : d) as Baum;
      wurzel.einstellungen ??= {};
      let x = wurzel.einstellungen as Baum;
      const kette: Baum[] = [x];
      for (const t of pfad.slice(0, -1)) {
        x[t] ??= {};
        x = x[t] as Baum;
        kette.push(x);
      }
      const letzt = pfad[pfad.length - 1];
      if (v === undefined) delete x[letzt];
      else x[letzt] = v;
      // leere Zwischenstufen entfernen
      for (let k = pfad.length - 2; k >= 0; k--) {
        const eltern = k === 0 ? (wurzel.einstellungen as Baum) : kette[k];
        if (Object.keys(kette[k + 1] ?? {}).length === 0) delete eltern[pfad[k]];
      }
      if (Object.keys(wurzel.einstellungen as Baum).length === 0) delete wurzel.einstellungen;
    }, `ew:${p.id}`);
  }
  const anzeigen = (v: unknown) =>
    typeof v === "number" ? zahl(v, 3) : typeof v === "boolean" ? (v ? "an" : "aus") : v == null ? "–" : String(v);
</script>

<div style="padding:10px 14px; display:flex; flex-direction:column; gap:8px; border-bottom:1px solid var(--line)">
  <div class="seg seg-sm" role="radiogroup" aria-label="Ebene" style="display:flex">
    <label class="seg-opt" style="flex:1; justify-content:center"><input type="radio" name="ewe" checked={ebene === "design"} onchange={() => (ebene = "design")} />Ganzes Design</label>
    <label class="seg-opt" style="flex:1; justify-content:center" class:aus={!o}><input type="radio" name="ewe" disabled={!o} checked={ebene === "objekt"} onchange={() => (ebene = "objekt")} />{o ? `Objekt Nr. ${i + 1}` : "Objekt (keins gewählt)"}</label>
  </div>
  <span class="mut small">Reihenfolge: Stoff → Design → Objekt → Felder direkt am Objekt. Leeres Feld = geerbter Wert.</span>
  <label class="small" style="display:flex; gap:6px; align-items:center"><input type="checkbox" bind:checked={alle} />auch automatische Werte zeigen</label>
</div>

{#each gruppen as g (g)}
  <div class="mh" style="padding:10px 14px 4px">{g}</div>
  {#each liste.filter((p) => p.gruppe === g) as p (p.id)}
    {@const w = wertUndHerkunft(p)}
    <div class="frow">
      <span class="fl" title="{p.wirkung} · Quelle: {p.quelle}">{p.name}{#if p.modus === "experte"}<span class="xt">X</span>{/if}</span>
      <div class="fc">
        {#if typeof p.standard === "boolean" || typeof w.wert === "boolean"}
          <select class="input sm" value={w.wert === undefined ? "" : w.wert ? "an" : "aus"} onchange={(ev) => {
            const v = (ev.currentTarget as HTMLSelectElement).value;
            setzen(p, v === "" ? undefined : v === "an");
          }}>
            <option value="">geerbt ({anzeigen(w.erbt)})</option><option value="an">an</option><option value="aus">aus</option>
          </select>
        {:else if p.werte.length}
          <select class="input sm" value={typeof w.wert === "string" ? w.wert : ""} onchange={(ev) => {
            const v = (ev.currentTarget as HTMLSelectElement).value;
            setzen(p, v === "" ? undefined : v);
          }}>
            <option value="">geerbt ({anzeigen(w.erbt)})</option>
            {#each p.werte as x (x)}<option value={x}>{x}</option>{/each}
          </select>
        {:else}
          <input
            class="input sm num"
            style="width:80px"
            inputmode="decimal"
            value={typeof w.wert === "number" ? zahl(w.wert, 3) : ""}
            placeholder={p.stoff && w.herkunft === "Stoff" ? "je Stoff" : anzeigen(w.erbt)}
            onchange={(ev) => {
              const t = (ev.currentTarget as HTMLInputElement).value.trim();
              if (t === "") return setzen(p, undefined);
              const v = lesen(t);
              if (v !== null) setzen(p, v);
            }}
          />
          <span class="unit">{p.einheit}</span>
        {/if}
        <span class="src" class:src-mod={w.herkunft === "geändert"} class:src-stoff={w.herkunft === "Stoff"} class:src-erbt={w.herkunft === "Design"}>{w.herkunft}</span>
        {#if w.herkunft === "geändert"}
          <button class="tb sm" aria-label="Zurücksetzen" title="Zurücksetzen" onclick={() => setzen(p, undefined)}><Icon d={IC.zuruecksetzen} groesse="s" /></button>
        {/if}
      </div>
      {#if p.bereich}<div class="fhelp">Bereich {zahl(p.bereich[0], 3)} – {zahl(p.bereich[1], 3)} {p.einheit}</div>{/if}
    </div>
  {/each}
{:else}
  <p class="mut small" style="padding:14px">{katalog.length ? "Keine Einzelwerte für diese Ebene." : "Lade Parameterkatalog …"}</p>
{/each}
