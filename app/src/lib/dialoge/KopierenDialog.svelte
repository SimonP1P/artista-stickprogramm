<script lang="ts">
  // Mirror-Merge und Kranz (Wreath) mit Kaleidoskop – Menü Anordnen (Handbuch Kap. 15 S. 140–142). Das Original setzt
  // den Ankerpunkt mit der Maus an einer Bezugslinie; hier: Zahlen oder Klick in die Vorschau (eigene Festlegung).
  // Die Rechnung macht der Kern (`spiegeln_kopieren`, `kranz`); die Vorschau zeigt die Formen des Ergebnisses
  // (`design_formen`): Original kräftig, Kopien heller, Stickfeld gestrichelt.
  import { arbeit } from "../arbeit.svelte";
  import Dialog from "../Dialog.svelte";
  import { formPfade, pfad } from "../formpfade";
  import { kern, type DesignJson, type KopienErgebnis, type ObjektForm, type Spiegeln } from "$kern/kern";
  import { lesen, zahl } from "../zahl";

  let { weg, schliessen }: { weg: "spiegeln" | "kranz"; schliessen: () => void } = $props();

  // svelte-ignore state_referenced_locally
  let art = $state<"spiegeln" | "kranz">(weg);
  let richtung = $state<Spiegeln>("links_rechts");
  let punkte = $state(6);
  let kaleidoskop = $state(false);
  let zusammen = $state(false);
  let ax = $state("0");
  let ay = $state("0");
  let vonHand = $state(false);
  let ergebnis = $state.raw<KopienErgebnis | null>(null);
  // bis das Ergebnis da ist: die Formen des Designs
  let formen = $state.raw<ObjektForm[]>(arbeit.formen);
  let fehler = $state("");
  let laeuft = $state(false);

  const auswahl = [...arbeit.auswahl];
  // Rechteck der Auswahl (Anzeige-Koordinaten)
  const box = (() => {
    const b = arbeit.formen.filter((f) => auswahl.includes(f.objekt) && f.begrenzung).map((f) => f.begrenzung!);
    if (!b.length) return [0, 0, 0, 0];
    return [Math.min(...b.map((x) => x[0])), Math.min(...b.map((x) => x[1])), Math.max(...b.map((x) => x[2])), Math.max(...b.map((x) => x[3]))];
  })();
  const bw = box[2] - box[0];
  const bh = box[3] - box[1];
  const mitteX = (box[0] + box[2]) / 2;
  const mitteY = (box[1] + box[3]) / 2;

  // Vorschlag des Ankerpunkts (eigene Festlegung): Spiegeln an der rechten bzw. unteren Kante (Kopie liegt an),
  // Kranz: Mittelpunkt unter der Auswahl, so weit weg, dass benachbarte Kopien sich nicht berühren
  function vorschlag() {
    if (vonHand) return;
    if (art === "spiegeln") {
      ax = zahl(box[2], 2);
      ay = zahl(box[1], 2);
    } else {
      const s = Math.max(bw, bh);
      const r = Math.max(bh / 2 + 2, (s * 1.1) / (2 * Math.sin(Math.PI / Math.max(2, punkte))));
      ax = zahl(mitteX, 2);
      ay = zahl(mitteY - r, 2);
    }
  }
  $effect(() => {
    void art;
    void punkte;
    vorschlag();
  });

  const anker = $derived({ x: lesen(ax) ?? 0, y: lesen(ay) ?? 0 });
  const kaleidoGeht = $derived(punkte % 2 === 0);

  // Ergebnis neu rechnen, sobald sich etwas ändert (das erste Mal sofort, danach kurz entprellt)
  let zeitgeber: ReturnType<typeof setTimeout> | undefined;
  let erstes = true;
  $effect(() => {
    const eingabe = {
      design: $state.snapshot(arbeit.design) as DesignJson,
      objekte: auswahl,
      zusammenfuehren: zusammen,
    };
    const a = art;
    const k = { punkte, kaleidoskop: kaleidoskop && kaleidoGeht, mitte: [anker.x, anker.y] as [number, number] };
    const s = { art: richtung, achse: [anker.x, anker.y] as [number, number] };
    clearTimeout(zeitgeber);
    const rechnen = async () => {
      laeuft = true;
      try {
        const r = a === "kranz" ? await kern("kranz", { ...eingabe, ...k }) : await kern("spiegeln_kopieren", { ...eingabe, ...s });
        ergebnis = r;
        formen = await kern("design_formen", { design: r.design });
        fehler = "";
      } catch (e) {
        const x = e as { meldung?: string };
        fehler = String(x.meldung ?? e);
        ergebnis = null;
      } finally {
        laeuft = false;
      }
    };
    if (erstes) {
      erstes = false;
      rechnen();
    } else zeitgeber = setTimeout(rechnen, 150);
  });

  // Vorschau: Rahmen, Formen, Anker
  const rahmen = $derived(arbeit.rahmenListe.find((r) => r.name === arbeit.design.rahmen));
  const ansicht = $derived.by(() => {
    const xs: number[] = [anker.x];
    const ys: number[] = [anker.y];
    for (const f of formen)
      if (f.begrenzung) {
        xs.push(f.begrenzung[0], f.begrenzung[2]);
        ys.push(f.begrenzung[1], f.begrenzung[3]);
      }
    for (const q of rahmen?.form ?? []) {
      xs.push(q[0]);
      ys.push(q[1]);
    }
    const x0 = Math.min(...xs);
    const x1 = Math.max(...xs);
    const y0 = Math.min(...ys);
    const y1 = Math.max(...ys);
    const r = Math.max(x1 - x0, y1 - y0, 10) * 0.05;
    return { x: x0 - r, y: -y1 - r, w: x1 - x0 + 2 * r, h: y1 - y0 + 2 * r, s: Math.max(x1 - x0, y1 - y0, 10) };
  });
  const neu = $derived(new Set(ergebnis?.neu ?? []));
  const strich = $derived(ansicht.s / 300);

  let svg: SVGSVGElement | undefined = $state();
  function klick(ev: MouseEvent) {
    const m = svg?.getScreenCTM();
    if (!svg || !m) return;
    const q = new DOMPoint(ev.clientX, ev.clientY).matrixTransform(m.inverse());
    vonHand = true;
    ax = zahl(q.x, 2);
    ay = zahl(-q.y, 2);
  }

  function uebernehmen() {
    if (!ergebnis) return;
    arbeit.ersetzen(ergebnis.design, arbeit.pfad, true);
    arbeit.auswahl = [...auswahl, ...ergebnis.neu];
    arbeit.zeige(
      (art === "kranz" ? "Kranz: " : "Gespiegelt: ") + anzahl(ergebnis.neu.length) +
        (ergebnis.zusammengefuehrt ? ` · ${ergebnis.zusammengefuehrt} Fläche(n) zusammengeführt` : "") +
        " – Strg+G gruppiert alles",
      "ok",
    );
    schliessen();
  }

  const anzahl = (n: number) => (n === 1 ? "1 neues Objekt" : `${n} neue Objekte`);

  const RICHTUNGEN: [Spiegeln, string, string][] = [
    ["links_rechts", "Waagrecht", "Kopie gespiegelt an einer senkrechten Linie (links/rechts)"],
    ["oben_unten", "Senkrecht", "Kopie gespiegelt an einer waagrechten Linie (oben/unten)"],
    ["beide", "Beides", "drei Kopien: links/rechts, oben/unten und über Kreuz"],
  ];
</script>

<Dialog titel={art === "kranz" ? "Kranz (Wreath)" : "Spiegeln und kopieren (Mirror-Merge)"} breite={760} {schliessen}>
  <div class="kp">
    <div class="kp-bild">
      <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
      <svg bind:this={svg} viewBox="{ansicht.x} {ansicht.y} {ansicht.w} {ansicht.h}" preserveAspectRatio="xMidYMid meet" role="img" aria-label="Vorschau – klicken setzt den Ankerpunkt" onclick={klick}>
        {#if rahmen}<path d={pfad(rahmen.form, true)} fill="var(--bg-1, #fff)" stroke="var(--ink-3, #999)" stroke-width={strich} stroke-dasharray="{strich * 4} {strich * 3}" />{/if}
        {#each formen as f (f.objekt)}
          {@const t = formPfade(f.linien)}
          {@const deck = neu.has(f.objekt) ? 0.45 : auswahl.includes(f.objekt) ? 0.95 : 0.25}
          <g fill={f.farbe} stroke={f.farbe} opacity={deck}>
            {#each t.saeulen as d, k (k)}<path {d} stroke-width={strich * 0.5} />{/each}
            {#if t.flaechen}<path d={t.flaechen} fill-rule="nonzero" stroke="none" />{/if}
            {#each t.offen as d, k (k)}<path {d} fill="none" stroke-width={strich * 2} stroke-linecap="round" />{/each}
          </g>
        {/each}
        <!-- Anker: Linie(n) beim Spiegeln, Kreuz beim Kranz -->
        {#if art === "spiegeln"}
          {#if richtung !== "oben_unten"}<line x1={anker.x} x2={anker.x} y1={ansicht.y} y2={ansicht.y + ansicht.h} stroke="var(--accent, #2b6cb0)" stroke-width={strich * 1.5} stroke-dasharray="{strich * 6} {strich * 3}" />{/if}
          {#if richtung !== "links_rechts"}<line y1={-anker.y} y2={-anker.y} x1={ansicht.x} x2={ansicht.x + ansicht.w} stroke="var(--accent, #2b6cb0)" stroke-width={strich * 1.5} stroke-dasharray="{strich * 6} {strich * 3}" />{/if}
        {:else}
          <line x1={anker.x} y1={-anker.y} x2={mitteX} y2={-mitteY} stroke="var(--accent, #2b6cb0)" stroke-width={strich} stroke-dasharray="{strich * 4} {strich * 3}" />
        {/if}
        <circle cx={anker.x} cy={-anker.y} r={strich * 5} fill="none" stroke="var(--accent, #2b6cb0)" stroke-width={strich * 1.5} />
        <circle cx={anker.x} cy={-anker.y} r={strich * 1.2} fill="var(--accent, #2b6cb0)" />
      </svg>
    </div>
    <div class="kp-rechts">
      <div class="seg seg-sm" role="radiogroup" aria-label="Werkzeug" style="display:flex">
        <label class="seg-opt" style="flex:1; justify-content:center"><input type="radio" name="kweg" checked={art === "spiegeln"} onchange={() => (art = "spiegeln")} />Spiegeln</label>
        <label class="seg-opt" style="flex:1; justify-content:center"><input type="radio" name="kweg" checked={art === "kranz"} onchange={() => (art = "kranz")} />Kranz</label>
      </div>
      {#if art === "spiegeln"}
        <div class="frow" style="padding:0">
          <span class="fl">Richtung</span>
          <div class="fc">
            <select class="input sm" bind:value={richtung}>
              {#each RICHTUNGEN as [w, l, h] (w)}<option value={w} title={h}>{l}</option>{/each}
            </select>
          </div>
        </div>
      {:else}
        <div class="frow" style="padding:0">
          <label class="fl" for="kpunkte" title="Original + Kopien rundherum (Wreath Points)">Punkte</label>
          <div class="fc"><input id="kpunkte" class="input sm num" type="number" min="2" max="36" bind:value={punkte} style="width:80px" /></div>
        </div>
        <label class="frow" style="padding:0" title="Jede zweite Kopie gespiegelt – nur bei gerader Zahl (wie im Original)">
          <span class="fl">Kaleidoskop</span>
          <span class="fc"><input type="checkbox" bind:checked={kaleidoskop} disabled={!kaleidoGeht} />{#if !kaleidoGeht}<span class="small mut" style="margin-left:6px">nur gerade Zahl</span>{/if}</span>
        </label>
      {/if}
      <div class="frow" style="padding:0">
        <span class="fl" title={art === "kranz" ? "Mittelpunkt des Kranzes" : "Punkt, durch den die Spiegellinie geht"}>{art === "kranz" ? "Mitte" : "Achse"}</span>
        <div class="fc" style="gap:4px">
          <span class="small mut">x</span><input class="input sm num" inputmode="decimal" bind:value={ax} oninput={() => (vonHand = true)} style="width:62px" />
          <span class="small mut">y</span><input class="input sm num" inputmode="decimal" bind:value={ay} oninput={() => (vonHand = true)} style="width:62px" />
          <span class="unit">mm</span>
        </div>
      </div>
      {#if vonHand}
        <button class="btn btn-ghost btn-sm" style="align-self:flex-start" onclick={() => ((vonHand = false), vorschlag())}>Vorschlag wiederherstellen</button>
      {/if}
      {#if ergebnis?.ueberlappt || zusammen}
        <label class="note small" style="display:flex; gap:8px; align-items:flex-start">
          <input type="checkbox" bind:checked={zusammen} />
          <span><b>Überlappende Objekte zusammenführen?</b> (wie das Original fragt) – Step-, Muster-, Stippling- und Satinflächen, die sich mit ihrer Kopie überlappen, werden ein Objekt.</span>
        </label>
      {/if}
      {#if fehler}<p class="small" style="color:var(--err); margin:0">{fehler}</p>{/if}
      <p class="small mut" style="margin:0">
        In die Vorschau klicken setzt {art === "kranz" ? "den Mittelpunkt" : "die Spiegellinie"}. Die Kopien kommen in der Stickreihenfolge direkt hinter die Auswahl{art === "kranz" ? ", im Uhrzeigersinn" : ""}; Stiche werden neu berechnet.
      </p>
    </div>
  </div>
  {#snippet fuss()}
    <span class="small mut" style="margin-right:auto">{ergebnis ? anzahl(ergebnis.neu.length) : laeuft ? "rechne …" : ""}</span>
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
    <button class="btn btn-primary" disabled={!ergebnis || laeuft} onclick={uebernehmen}>Übernehmen</button>
  {/snippet}
</Dialog>

<style>
  .kp {
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 16px;
  }
  .kp-bild {
    border: 1px solid var(--line, #ddd);
    border-radius: 6px;
    background: var(--bg-2, #f4f4f2);
    height: 380px;
  }
  .kp-bild svg {
    width: 100%;
    height: 100%;
    cursor: crosshair;
    display: block;
  }
  .kp-rechts {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
</style>
