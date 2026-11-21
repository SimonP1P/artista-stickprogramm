<script lang="ts">
  // „Einzelne Buchstaben“ (Expertenmodus, UEBERGABE §3 X, W9): Text/Monogramm wählen, dann in der Vorschau auf einen
  // Buchstaben klicken – der Kern liefert je Stich die Buchstabennummer (`Ergebnis.buchstabe`). Bearbeitet wird der
  // Eintrag `buchstaben[] {nr, farbe, transform, winkellinien}` des Objekts (Handbuch Kap. 20: einzelne Buchstaben
  // verschieben, drehen, skalieren, umfärben).
  import type { BuchstabeJson, Transformation } from "$kern/kern";
  import { arbeit } from "../arbeit.svelte";
  import Icon from "../Icon.svelte";
  import { IC } from "../icons";
  import { lesen, zahl } from "../zahl";

  let { nr: objektNr }: { nr: number } = $props();

  const o = $derived(arbeit.objekte[objektNr] as Record<string, unknown> | undefined);
  const liste = $derived(((o?.buchstaben as BuchstabeJson[] | undefined) ?? []).slice().sort((a, b) => a.nr - b.nr));
  const gewaehlt = $derived(arbeit.buchstabe?.objekt === objektNr ? arbeit.buchstabe.nr : null);
  const eintrag = $derived(gewaehlt === null ? undefined : liste.find((b) => b.nr === gewaehlt));
  const t = $derived((eintrag?.transform ?? undefined) as Transformation | undefined);

  /** Zeichen Nr. n (1 = erstes, ohne Leerraum und `^`) des Texts bzw. der Initialen */
  function zeichen(n: number): string {
    const text = String(o?.text ?? o?.initialen ?? "");
    return [...text].filter((c) => !/\s/.test(c) && c !== "^")[n - 1] ?? "?";
  }

  function aendern(f: (b: BuchstabeJson) => void, feld: string) {
    const n = gewaehlt;
    if (n === null) return;
    arbeit.aendern((d) => {
      const x = d.objekte![objektNr] as Record<string, unknown>;
      const l = ((x.buchstaben as BuchstabeJson[] | undefined) ??= []);
      let b = l.find((y) => y.nr === n);
      if (!b) l.push((b = { nr: n }));
      f(b);
      if (b.transform && Object.keys(b.transform).length === 0) delete b.transform;
      if (b.farbe === null) delete b.farbe;
      if (Object.keys(b).length === 1) l.splice(l.indexOf(b), 1);
      if (!l.length) delete x.buchstaben;
    }, `bu:${objektNr}:${n}:${feld}`);
  }
  function transformAendern(f: (x: Transformation) => void, feld: string) {
    aendern((b) => {
      const x: Transformation = { ...(b.transform ?? {}) };
      f(x);
      b.transform = x;
    }, feld);
  }
  function zuruecksetzen() {
    const n = gewaehlt;
    arbeit.aendern((d) => {
      const x = d.objekte![objektNr] as Record<string, unknown>;
      const l = (x.buchstaben as BuchstabeJson[] | undefined) ?? [];
      x.buchstaben = l.filter((b) => b.nr !== n);
      if (!(x.buchstaben as BuchstabeJson[]).length) delete x.buchstaben;
    });
  }
  const skal = (x: Transformation | undefined) =>
    Array.isArray(x?.skalieren) ? (x.skalieren[0] ?? 100) : ((x?.skalieren as number | undefined) ?? 100);
</script>

{#if gewaehlt === null}
  <p class="small mut" style="padding:4px 14px 8px; margin:0">
    Auf einen Buchstaben in der Vorschau klicken (Objekt ist gewählt) – dann hier verschieben, drehen, skalieren oder
    umfärben. Stiche werden neu berechnet.
  </p>
{:else}
  <div class="frow">
    <span class="fl">Buchstabe</span>
    <div class="fc">
      <b class="hd" style="font-size:18px; min-width:22px">{zeichen(gewaehlt)}</b>
      <span class="mut small">Nr. {gewaehlt}</span>
      <span style="flex:1"></span>
      {#if eintrag}<button class="tb sm" aria-label="Buchstaben zurücksetzen" title="Zurücksetzen" onclick={zuruecksetzen}><Icon d={IC.zuruecksetzen} groesse="s" /></button>{/if}
      <button class="tb sm" aria-label="Buchstabenauswahl aufheben" title="Auswahl aufheben" onclick={() => (arbeit.buchstabe = null)}><Icon d={IC.kreuz} groesse="s" /></button>
    </div>
  </div>
  <div class="frow">
    <label class="fl" for="bu-farbe">Farbe</label>
    <div class="fc">
      <input id="bu-farbe" type="color" value={eintrag?.farbe ?? (o?.farbe as string) ?? "#000000"} oninput={(ev) => aendern((b) => (b.farbe = (ev.currentTarget as HTMLInputElement).value), "farbe")} style="width:44px; height:28px; padding:0; border:1px solid var(--line); background:none" />
      <span class="src" class:src-mod={!!eintrag?.farbe}>{eintrag?.farbe ? "eigene" : "wie Text"}</span>
      {#if eintrag?.farbe}<button class="tb sm" aria-label="Farbe wie Text" title="wie Text" onclick={() => aendern((b) => delete b.farbe, "farbe")}><Icon d={IC.zuruecksetzen} groesse="s" /></button>{/if}
    </div>
    <div class="fhelp">Eigene Farbe = eigener Farbstopp</div>
  </div>
  <div class="frow">
    <label class="fl" for="bu-g">Größe</label>
    <div class="fc"><input id="bu-g" class="input sm num" style="width:80px" value={zahl(skal(t))} onchange={(ev) => {
      const v = lesen((ev.currentTarget as HTMLInputElement).value);
      if (v && v > 0) transformAendern((x) => (v === 100 ? delete x.skalieren : (x.skalieren = v)), "g");
    }} /><span class="unit">%</span></div>
  </div>
  <div class="frow">
    <label class="fl" for="bu-d">Drehen</label>
    <div class="fc"><input id="bu-d" class="input sm num" style="width:80px" value={zahl(t?.drehen ?? 0)} onchange={(ev) => {
      const v = lesen((ev.currentTarget as HTMLInputElement).value);
      if (v !== null) transformAendern((x) => (v === 0 ? delete x.drehen : (x.drehen = v)), "d");
    }} /><span class="unit">°</span></div>
  </div>
  <div class="frow">
    <span class="fl">Verschieben</span>
    <div class="fc">
      {#each [0, 1] as achse (achse)}
        <input class="input sm num" style="width:64px" aria-label={achse ? "y" : "x"} value={zahl(t?.verschieben?.[achse] ?? 0)} onchange={(ev) => {
          const v = lesen((ev.currentTarget as HTMLInputElement).value);
          if (v !== null) transformAendern((x) => {
            const p = [...(x.verschieben ?? [0, 0])] as [number, number];
            p[achse] = v;
            if (p[0] === 0 && p[1] === 0) delete x.verschieben;
            else x.verschieben = p;
          }, "v");
        }} />
      {/each}
      <span class="unit">mm</span>
    </div>
  </div>
  {#if eintrag?.winkellinien?.length}
    <div class="small mut" style="padding:2px 14px 6px">{eintrag.winkellinien.length} Stichwinkel-Linie(n) – im Reiter JSON</div>
  {/if}
{/if}
{#if liste.length}
  <div style="display:flex; flex-wrap:wrap; gap:6px; padding:4px 14px 8px">
    <span class="small mut">Geändert:</span>
    {#each liste as b (b.nr)}
      <button class="btn btn-secondary" style="min-height:24px; padding:0 8px; {b.nr === gewaehlt ? 'border-color:var(--accent); color:var(--accent)' : ''}" onclick={() => (arbeit.buchstabe = { objekt: objektNr, nr: b.nr })} title="Buchstabe Nr. {b.nr}">
        {zeichen(b.nr)}<span class="mut" style="font-size:10px">&nbsp;{b.nr}</span>
      </button>
    {/each}
  </div>
{/if}
