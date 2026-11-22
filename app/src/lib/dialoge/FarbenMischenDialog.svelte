<script lang="ts">
  // Farben mischen (Color Blending, Handbuch S. 220–222; Kern `farben_mischen`): das Objekt wird in zwei Lagen geteilt –
  // unten die jetzige Farbe, oben eine zweite – mit gegenläufigem Verlauf (Profil und Gegenstück); beide gruppiert.
  // Standardwerte aus dem Stil des Originals: größter Abstand unten 2,0 mm, oben 2,2 mm; dicht 0,4 mm.
  import { kern, type DesignJson, type GarnInfo, type Profil } from "$kern/kern";
  import Dialog from "../Dialog.svelte";
  import GarnDialog from "./GarnDialog.svelte";
  import { arbeit } from "../arbeit.svelte";
  import { garnText } from "../listen.svelte";
  import { GEGENSTUECK, PROFILE, reihen } from "../effekte";
  import { lesen, zahl } from "../zahl";

  let { objekt, schliessen }: { objekt: number; schliessen: () => void } = $props();

  const o = $derived(arbeit.objekte[objekt] as { name?: string; farbe?: string } | undefined);
  const unten = $derived(arbeit.garnVon(objekt));
  let garn = $state.raw<GarnInfo | null>(null);
  let garnWahl = $state(false);
  let profil = $state<Profil>("linear");
  let min = $state(0.4);
  let maxU = $state(2.0);
  let maxO = $state(2.2);
  let fehler = $state("");
  let laeuft = $state(false);

  // Vorschau: 40 mm Fläche quer zu den Reihen, unten Linien der ersten Farbe, darüber die zweite
  const L = 40;
  const linienU = $derived(reihen(profil, min, maxU, L));
  const linienO = $derived(reihen(GEGENSTUECK[profil], min, maxO, L));
  const farbeU = $derived(o?.farbe ?? unten?.rgb ?? "#000000");

  function wert(ev: Event, setzen: (v: number) => void) {
    const v = lesen((ev.currentTarget as HTMLInputElement).value);
    if (v !== null && v >= 0.1 && v <= 10) setzen(v);
  }

  async function mischen() {
    if (!garn) return;
    fehler = "";
    laeuft = true;
    try {
      const r = await kern("farben_mischen", {
        design: $state.snapshot(arbeit.design) as DesignJson,
        objekt,
        farbe: garn.rgb,
        profil,
        min,
        max_unten: maxU,
        max_oben: maxO,
      });
      const d = r.design as DesignJson;
      const oben = d.objekte?.[r.objekte[1]];
      if (oben) oben.garn = garn.nummer;
      arbeit.ersetzen(d, arbeit.pfad, true);
      arbeit.auswahl = [...r.objekte];
      arbeit.zeige(`Farben gemischt: ${garnText(garn)} liegt oben, gruppiert mit der unteren Lage`, "info");
      schliessen();
    } catch (e) {
      const x = e as { meldung?: string };
      fehler = String(x.meldung ?? e);
    } finally {
      laeuft = false;
    }
  }
</script>

<Dialog titel="Farben mischen" breite={560} {schliessen}>
  {#snippet kopf()}
    <h2 class="hd">Farben mischen</h2>
    <span class="mut small">{o?.name || `Objekt ${objekt + 1}`}</span>
  {/snippet}
  <p class="mut small" style="margin:0 0 12px">
    Das Objekt wird zweimal gestickt: unten in der jetzigen Farbe, darüber in einer zweiten. Wo die eine Lage dicht ist, ist die andere offen – so entsteht ein Übergang. Die obere Lage hat keine Unterlage.
  </p>
  <div class="zeile">
    <span>Unten</span>
    <div class="wahl"><span class="sw" style="background:{farbeU}"></span>{unten ? garnText(unten) : farbeU}</div>
  </div>
  <div class="zeile">
    <span>Oben</span>
    <button class="btn btn-secondary pick" onclick={() => (garnWahl = true)}>
      {#if garn}<span class="sw" style="background:{garn.rgb}"></span><span class="t">{garnText(garn)}</span>{:else}<span class="t">Zweites Garn wählen …</span>{/if}
    </button>
  </div>
  <label class="zeile">
    <span>Profil</span>
    <select class="input sm" bind:value={profil} aria-label="Profil der unteren Lage">
      {#each PROFILE as [w, t] (w)}<option value={w}>{t}</option>{/each}
    </select>
  </label>
  <p class="mut small" style="margin:2px 0 8px 128px">obere Lage: {PROFILE.find((x) => x[0] === GEGENSTUECK[profil])?.[1]}</p>
  <div class="zeile">
    <span>Abstand<span class="xt">X</span></span>
    <div class="werte">
      <label>dicht <input class="input sm num" value={zahl(min)} onchange={(e) => wert(e, (v) => (min = v))} aria-label="dichtester Abstand" /></label>
      <label>offen unten <input class="input sm num" value={zahl(maxU)} onchange={(e) => wert(e, (v) => (maxU = v))} aria-label="größter Abstand unten" /></label>
      <label>oben <input class="input sm num" value={zahl(maxO)} onchange={(e) => wert(e, (v) => (maxO = v))} aria-label="größter Abstand oben" /></label>
      <span class="unit">mm</span>
    </div>
  </div>
  <svg class="vorschau" viewBox="0 0 {L} 12" preserveAspectRatio="none" role="img" aria-label="Vorschau der beiden Lagen">
    <rect x="0" y="0" width={L} height="12" fill="var(--canvas, #f6f4ef)" />
    {#each linienU as y, k (k)}<line x1={y} x2={y} y1="0.5" y2="11.5" stroke={farbeU} stroke-width="0.18" />{/each}
    {#each linienO as y, k (k)}<line x1={y} x2={y} y1="0.5" y2="11.5" stroke={garn?.rgb ?? "#9aa0a6"} stroke-width="0.18" />{/each}
  </svg>
  <p class="mut small" style="margin:4px 0 0">Reihen über 40 mm quer zur Stichrichtung (Anfang links) – Vorschau, gestickt wird im Kern.</p>
  {#if fehler}<div class="errbox" style="margin-top:10px">{fehler}</div>{/if}
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
    <button class="btn btn-primary" disabled={laeuft || !garn} onclick={mischen}>Mischen</button>
  {/snippet}
</Dialog>

{#if garnWahl}
  <GarnDialog titel="Garn der oberen Lage" untertitel={o?.name ?? ""} aktuell={garn ?? undefined} waehlen={(g) => ((garn = g), (garnWahl = false))} schliessen={() => (garnWahl = false)} />
{/if}

<style>
  .zeile {
    display: grid;
    grid-template-columns: 120px 1fr;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
  }
  .wahl {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .werte {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
  }
  .werte label {
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .werte input {
    width: 56px;
  }
  .pick {
    justify-content: flex-start;
    gap: 8px;
  }
  .sw {
    width: 16px;
    height: 16px;
    border-radius: 3px;
    border: 1px solid var(--line);
    display: inline-block;
  }
  .vorschau {
    width: 100%;
    height: 60px;
    border: 1px solid var(--line);
    border-radius: 6px;
    margin-top: 6px;
  }
</style>
