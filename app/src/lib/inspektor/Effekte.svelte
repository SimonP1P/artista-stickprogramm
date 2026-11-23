<script lang="ts">
  // Effekte eines Objekts (Handbuch Kap. 24, Objekteigenschaften → „Effects“; Kern `effekte`, Nachtauftrag N2):
  // Verlauf (Gradient Fill), ausgefranster Rand (Textured Edge), Welle (Wave Effect, nur Flächen) und „Farben
  // mischen …“ (Color Blending, Befehl `farben_mischen`). Werte leer = Standard des Kerns (aus dem Original).
  import { kern, type DesignJson, type EffekteJson, type ObjektJson } from "$kern/kern";
  import { arbeit } from "../arbeit.svelte";
  import { PROFILE } from "../effekte";
  import { lesen, zahl } from "../zahl";

  let { nr, o }: { nr: number; o: ObjektJson & Record<string, unknown> } = $props();

  const fx = $derived((o.effekte ?? null) as EffekteJson | null);
  const flaeche = $derived(o.art === "tatami" || o.art === "muster");
  let fehler = $state("");

  function aendern(f: (e: EffekteJson) => void) {
    arbeit.aendern((d) => {
      const x = d.objekte![nr] as Record<string, unknown>;
      const e = ((x.effekte as EffekteJson | null | undefined) ?? {}) as EffekteJson;
      f(e);
      const leer = !e.verlauf && !e.rand && !e.welle;
      if (leer) delete x.effekte;
      else x.effekte = e;
    }, `effekte:${nr}`);
  }
  function zahlFeld(ev: Event, min: number, max: number, f: (v: number | undefined) => void) {
    const t = (ev.currentTarget as HTMLInputElement).value.trim();
    if (!t) return f(undefined);
    const v = lesen(t);
    if (v !== null && v >= min && v <= max) f(v);
  }

  async function welleAn(an: boolean) {
    fehler = "";
    if (!an) return aendern((e) => delete e.welle);
    try {
      const w = await kern("welle_vorschlag", { design: $state.snapshot(arbeit.design) as DesignJson, objekt: nr });
      aendern((e) => (e.welle = w as EffekteJson["welle"]));
    } catch (x) {
      fehler = String((x as { meldung?: string })?.meldung ?? x);
    }
  }
</script>

<!-- Verlauf -->
<div class="frow">
  <label class="fl" for="fxv" title="Gradient Fill: der Abstand wechselt über die Fläche (bei Satin entlang der Säule) zwischen dicht und offen – für Schattierungen und Übergänge. Der Abstand des Objekts gilt dann nicht.">Verlauf</label>
  <div class="fc"><input id="fxv" type="checkbox" checked={!!fx?.verlauf} onchange={(ev) => aendern((e) => ((ev.currentTarget as HTMLInputElement).checked ? (e.verlauf = {}) : delete e.verlauf))} style="width:16px; height:16px" /></div>
</div>
{#if fx?.verlauf}
  {@const v = fx.verlauf}
  <div class="frow">
    <label class="fl" for="fxvp">Profil</label>
    <div class="fc">
      <select id="fxvp" class="input sm" value={v.profil ?? "linear"} onchange={(ev) => aendern((e) => (e.verlauf!.profil = (ev.currentTarget as HTMLSelectElement).value as never))}>
        {#each PROFILE as [w, t] (w)}<option value={w}>{t}</option>{/each}
      </select>
    </div>
    <div class="fhelp">Anfang = Seite, an der die Reihen beginnen (quer zur Stichrichtung) bzw. Anfang der Satinsäule</div>
  </div>
  <div class="frow">
    <label class="fl" for="fxvmin">Abstand dicht</label>
    <div class="fc"><input id="fxvmin" class="input sm num" style="width:90px" placeholder="0,4" value={v.min != null ? zahl(v.min) : ""} onchange={(ev) => zahlFeld(ev, 0.1, 10, (x) => aendern((e) => (x == null ? delete e.verlauf!.min : (e.verlauf!.min = x))))} /><span class="unit">mm</span></div>
  </div>
  <div class="frow">
    <label class="fl" for="fxvmax">Abstand offen</label>
    <div class="fc"><input id="fxvmax" class="input sm num" style="width:90px" placeholder="2,0" value={v.max != null ? zahl(v.max) : ""} onchange={(ev) => zahlFeld(ev, 0.1, 10, (x) => aendern((e) => (x == null ? delete e.verlauf!.max : (e.verlauf!.max = x))))} /><span class="unit">mm</span></div>
    <div class="fhelp">0,1 … 10 mm („Maximum Spacing“); Standard 0,4 / 2,0 mm wie das Original</div>
  </div>
{/if}

<!-- Ausgefranster Rand -->
<div class="frow">
  <label class="fl" for="fxr" title="Textured Edge: die Reihen- bzw. Stichenden streuen um den Rand – für Fell, Gras, weiche Kanten.">Ausgefranster Rand</label>
  <div class="fc"><input id="fxr" type="checkbox" checked={!!fx?.rand} onchange={(ev) => aendern((e) => ((ev.currentTarget as HTMLInputElement).checked ? (e.rand = {}) : delete e.rand))} style="width:16px; height:16px" /></div>
</div>
{#if fx?.rand}
  {@const r = fx.rand}
  <div class="frow">
    <span class="fl">Seite</span>
    <div class="fc">
      <div class="seg seg-sm" role="radiogroup" aria-label="Seite des ausgefransten Rands" style="display:flex; width:100%">
        {#each [["links", "Links"], ["beide", "Beide"], ["rechts", "Rechts"]] as [w, t] (w)}
          <label class="seg-opt" style="flex:1; justify-content:center; padding-inline:4px"><input type="radio" name="fxrs" checked={(r.seite ?? "rechts") === w} onchange={() => aendern((e) => (e.rand!.seite = w as never))} />{t}</label>
        {/each}
      </div>
    </div>
    <div class="fhelp">in Stichrichtung gesehen; bei Satin links = Seite A</div>
  </div>
  <div class="frow">
    <label class="fl" for="fxrsp">Spanne</label>
    <div class="fc"><input id="fxrsp" class="input sm num" style="width:90px" placeholder="1,5" value={r.spanne != null ? zahl(r.spanne) : ""} onchange={(ev) => zahlFeld(ev, 0.1, 10, (x) => aendern((e) => (x == null ? delete e.rand!.spanne : (e.rand!.spanne = x))))} /><span class="unit">mm</span></div>
    <div class="fhelp">so weit streuen die Enden nach innen und außen („Span“)</div>
  </div>
  <div class="frow">
    <label class="fl" for="fxrg">Glätte</label>
    <div class="fc">
      <input id="fxrg" type="range" min="0" max="100" step="5" value={Math.round((r.glaette ?? 0.1) * 100)} oninput={(ev) => aendern((e) => (e.rand!.glaette = Number((ev.currentTarget as HTMLInputElement).value) / 100))} style="flex:1" />
      <span class="unit" style="width:40px; text-align:right">{Math.round((r.glaette ?? 0.1) * 100)} %</span>
    </div>
    <div class="fhelp">links rau (jede Reihe frei), rechts glatt („Texture“)</div>
  </div>
{/if}

<!-- Welle -->
{#if flaeche}
  <div class="frow">
    <label class="fl" for="fxw" title="Wave Effect: die Reihen folgen einer Leitlinie; Dichte und Einstichmuster bleiben gleich. Stichlänge ≤ 4 mm sieht am besten aus.">Welle</label>
    <div class="fc"><input id="fxw" type="checkbox" checked={!!fx?.welle} onchange={(ev) => welleAn((ev.currentTarget as HTMLInputElement).checked)} style="width:16px; height:16px" /></div>
    {#if fx?.welle}<div class="fhelp">Leitlinie mit „Knoten bearbeiten“ formen (Punkte ziehen, Rechtsklick = Ecke ↔ Kurve)</div>{/if}
  </div>
{/if}
{#if fehler}<div class="errbox" style="margin:6px 0">{fehler}</div>{/if}

<div class="frow">
  <span class="fl" title="Color Blending: das Objekt wird in zwei Lagen geteilt – unten die jetzige Farbe, oben eine zweite – mit gegenläufigem Verlauf; beide werden gruppiert.">Farben mischen</span>
  <div class="fc"><button class="btn btn-secondary" style="width:100%" onclick={() => (arbeit.dialog = { art: "farben_mischen", objekt: nr })}>Zweite Farbe dazu …</button></div>
</div>
