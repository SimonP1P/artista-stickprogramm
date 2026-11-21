<script lang="ts">
  // Stichvorschau (Herzstück, größte Fläche): zeichnet die Stiche aus `design_berechnen` und die Formen aus
  // `design_formen` – rechnet selbst keine Geometrie. Auswahl per Klick auf die Stiche: jeder Pfad trägt die
  // Objektnummer (`Ergebnis.objekt` je Stich), die Trefferprüfung macht der Browser. Darstellung wie Entwurf v3.
  import { arbeit } from "./arbeit.svelte";
  import { zahl, zahlFest } from "./zahl";

  let {
    spruenge = true,
    einstiche = false,
    raster = true,
    zoom = $bindable(1),
  }: { spruenge?: boolean; einstiche?: boolean; raster?: boolean; zoom?: number } = $props();

  let svg: SVGSVGElement | undefined = $state();
  let mitte = $state<[number, number]>([0, 0]);
  let ziehen: { x: number; y: number; m: [number, number]; bewegt: boolean } | null = null;

  const e = $derived(arbeit.ergebnis);
  const rahmen = $derived(e?.rahmen ?? arbeit.rahmenListe.find((r) => r.name === arbeit.design.rahmen) ?? null);
  const dk = $derived(arbeit.dunkel);
  const farben = $derived({
    grund: dk ? "#0f1113" : "#dcdde0",
    reif: dk ? "#30343a" : "#c9cbd0",
    reifRand: dk ? "#555b63" : "#a9acb2",
    stoff: dk ? "#2a2d31" : "#fbfaf7",
    raster: dk ? "#3b4046" : "#e6e3dc",
    akzent: dk ? "#94bce3" : "#416180",
    tinte: dk ? "#e6e8ea" : "#1d1f20",
    beschriftung: dk ? "#c9ccd0" : "#4a4d50",
    sprung: dk ? "#9aa0a6" : "#6b6e72",
    fehler: dk ? "#ff8a80" : "#b42318",
    griff: dk ? "#16191c" : "#fff",
  });

  // Ausschnitt in mm (y nach unten gespiegelt: SVG-y = −y)
  const rand = 16;
  const ausschnitt = $derived.by(() => {
    const b = (rahmen?.breite ?? 100) + 2 * rand;
    const h = (rahmen?.hoehe ?? 100) + 2 * rand;
    return { x: mitte[0] - b / zoom / 2, y: -mitte[1] - h / zoom / 2, w: b / zoom, h: h / zoom };
  });

  const rahmenPfad = $derived(
    rahmen ? rahmen.form.map(([x, y], i) => `${i ? "L" : "M"}${x.toFixed(2)} ${(-y).toFixed(2)}`).join("") + "Z" : "",
  );

  const rasterPfad = $derived.by(() => {
    if (!rahmen) return "";
    let g = "";
    const bx = rahmen.breite / 2, by = rahmen.hoehe / 2;
    for (let x = -Math.floor(bx / 10) * 10; x <= bx; x += 10) g += `M${x} ${-by}V${by}`;
    for (let y = -Math.floor(by / 10) * 10; y <= by; y += 10) g += `M${-bx} ${y}H${bx}`;
    return g;
  });

  /** Garnfarbe etwas dunkler als Rand (Fadenwirkung wie im Entwurf) */
  function schattieren(hex: string, k: number) {
    const n = parseInt(hex.slice(1), 16);
    const c = [n >> 16, (n >> 8) & 255, n & 255].map((v) => Math.round(k < 0 ? v * (1 + k) : v + (255 - v) * k));
    return "#" + c.map((v) => v.toString(16).padStart(2, "0")).join("");
  }
  function hell(hex: string) {
    const n = parseInt(hex.slice(1), 16);
    return (0.299 * (n >> 16) + 0.587 * ((n >> 8) & 255) + 0.114 * (n & 255)) / 255;
  }

  // Stiche je (Objekt, Farbblock) als Pfad; Sprünge und Farbwechsel beginnen einen neuen Teilpfad
  const zuege = $derived.by(() => {
    const out: { o: number; farbe: string; d: string }[] = [];
    const sprungD: string[] = [];
    const punkteD: string[] = [];
    if (!e) return { zuege: out, spruenge: "", einstiche: "", aussen: "" };
    let block = 0;
    let aktuell: { o: number; farbe: string; d: string } | null = null;
    let vorher: [number, number] | null = null;
    let springen = false;
    for (let i = 0; i < e.stiche.length; i++) {
      const s = e.stiche[i];
      const x = s.x.toFixed(2), y = (-s.y).toFixed(2);
      if (s.art === "farbwechsel") {
        block++;
        aktuell = null;
        springen = true;
        continue;
      }
      if (s.art === "sprung") {
        if (vorher) sprungD.push(`M${vorher[0].toFixed(2)} ${(-vorher[1]).toFixed(2)}L${x} ${y}`);
        vorher = [s.x, s.y];
        springen = true;
        continue;
      }
      const o = e.objekt[i] ?? 0;
      let neuerPfad = springen;
      if (!aktuell || aktuell.o !== o) {
        aktuell = { o, farbe: (e.farben[block] ?? "#000000").toLowerCase(), d: "" };
        out.push(aktuell);
        neuerPfad = true;
      }
      if (neuerPfad) {
        // ohne Sprung dazwischen vom vorigen Einstich aus weiterzeichnen (Verbindungsstich)
        aktuell.d += vorher && !springen ? `M${vorher[0].toFixed(2)} ${(-vorher[1]).toFixed(2)}L${x} ${y}` : `M${x} ${y}`;
        springen = false;
      } else aktuell.d += `L${x} ${y}`;
      vorher = [s.x, s.y];
      if (einstiche) punkteD.push(`M${x} ${y}h.01`);
    }
    const aussen = (e.ausserhalb_stiche ?? [])
      .map((i) => e.stiche[i])
      .map((s) => `M${(s.x - 0.5).toFixed(1)} ${(-s.y - 0.5).toFixed(1)}l1 1m0 -1l-1 1`)
      .join("");
    return { zuege: out, spruenge: sprungD.join(""), einstiche: punkteD.join(""), aussen };
  });

  const gewaehlt = $derived(arbeit.ansicht !== "abspielen" ? arbeit.auswahl : []);
  const begrenzungen = $derived(
    gewaehlt.map((i) => arbeit.formen.find((f) => f.objekt === i)?.begrenzung).filter((b): b is number[] => !!b),
  );
  const fehlerBox = $derived.by(() => {
    const o = arbeit.fehler?.objekt;
    if (o === null || o === undefined) return null;
    return arbeit.formen.find((f) => f.objekt === o)?.begrenzung ?? null;
  });

  function objektAusZiel(t: EventTarget | null): number | null {
    const el = (t as Element | null)?.closest?.("[data-o]");
    return el ? Number(el.getAttribute("data-o")) : null;
  }

  function runter(ev: PointerEvent) {
    if (ev.button !== 0 && ev.button !== 1) return;
    ziehen = { x: ev.clientX, y: ev.clientY, m: [...mitte], bewegt: false };
    svg?.setPointerCapture(ev.pointerId);
  }
  function bewegen(ev: PointerEvent) {
    if (!ziehen || !svg) return;
    const dx = ev.clientX - ziehen.x, dy = ev.clientY - ziehen.y;
    if (!ziehen.bewegt && Math.hypot(dx, dy) < 4) return;
    ziehen.bewegt = true;
    const s = ausschnitt.w / svg.clientWidth;
    const k = Math.max(s, ausschnitt.h / svg.clientHeight);
    mitte = [ziehen.m[0] - dx * k, ziehen.m[1] + dy * k];
  }
  function hoch(ev: PointerEvent) {
    const war = ziehen;
    ziehen = null;
    if (war && !war.bewegt && ev.button === 0) {
      // Pointer-Capture lenkt das Ziel um – Element unter dem Zeiger suchen
      const ziel = document.elementFromPoint(ev.clientX, ev.clientY);
      arbeit.waehlen(objektAusZiel(ziel), ev.shiftKey || ev.ctrlKey || ev.metaKey);
    }
  }
  function rad(ev: WheelEvent) {
    ev.preventDefault();
    zoom = Math.min(40, Math.max(0.25, zoom * (ev.deltaY < 0 ? 1.15 : 1 / 1.15)));
  }
  export function einpassen() {
    zoom = 1;
    mitte = [0, 0];
  }
</script>

<svg
  bind:this={svg}
  role="img"
  aria-label="Stichvorschau {rahmen?.name ?? ''}"
  viewBox="{ausschnitt.x} {ausschnitt.y} {ausschnitt.w} {ausschnitt.h}"
  preserveAspectRatio="xMidYMid meet"
  onpointerdown={runter}
  onpointermove={bewegen}
  onpointerup={hoch}
  onwheel={rad}
>
  <rect x={ausschnitt.x - 500} y={ausschnitt.y - 500} width={ausschnitt.w + 1000} height={ausschnitt.h + 1000} fill={farben.grund} />
  {#if rahmen}
    <!-- Rahmen: Reif und Stoff als breite Striche um das Stickfeld (reine Darstellung) -->
    <path d={rahmenPfad} fill={farben.reif} stroke={farben.reif} stroke-width="18" stroke-linejoin="round" />
    <path d={rahmenPfad} fill={farben.stoff} stroke={farben.stoff} stroke-width="13" stroke-linejoin="round" />
    {#if raster && arbeit.ansicht !== "abspielen"}
      <clipPath id="stickfeld"><path d={rahmenPfad} /></clipPath>
      <path d={rasterPfad} stroke={farben.raster} stroke-width=".25" fill="none" clip-path="url(#stickfeld)" />
    {/if}
    <path d={rahmenPfad} fill="none" stroke={farben.akzent} stroke-width=".35" stroke-dasharray="2 1.2" />
    <path d="M-3 0H3M0 -3V3" stroke={farben.akzent} stroke-width=".3" />
    <text x={-rahmen.breite / 2} y={-rahmen.hoehe / 2 - 10.5} font-size="3.4" fill={farben.beschriftung} font-family="Barlow, system-ui, sans-serif">
      {rahmen.name} · Stickfeld {zahl(rahmen.breite)} × {zahl(rahmen.hoehe)} mm
    </text>
  {/if}

  {#if arbeit.ansicht === "form"}
    {#each arbeit.formen as f (f.objekt)}
      {@const an = gewaehlt.includes(f.objekt)}
      {@const farbe = (f.farbe || "#000000").toLowerCase()}
      {@const strich =
        dk && hell(farbe) < 0.25 ? schattieren(farbe, 0.55) : hell(farbe) > 0.8 ? schattieren(farbe, -0.5) : farbe}
      <g data-o={f.objekt} opacity={gewaehlt.length && !an ? 0.45 : 1} style="cursor:pointer">
        {#each f.linien as l, k (k)}
          <path
            d={l.punkte.map(([x, y], j) => `${j ? "L" : "M"}${x.toFixed(2)} ${(-y).toFixed(2)}`).join("") + (l.geschlossen ? "Z" : "")}
            fill={l.art === "umriss" ? farbe : "none"}
            fill-opacity={l.art === "umriss" ? 0.22 : 0}
            stroke={strich}
            stroke-width={an ? 0.55 : 0.35}
            stroke-dasharray={l.art === "winkellinie" ? "1.4 .8" : undefined}
            pointer-events="visible"
          />
        {/each}
      </g>
      {#if an && f.einstich}
        <g pointer-events="none">
          <circle cx={f.einstich[0]} cy={-f.einstich[1]} r="2.4" fill="#2f6f3e" stroke="#fff" stroke-width=".4" />
          <text x={f.einstich[0]} y={-f.einstich[1] + 1.1} font-size="2.8" text-anchor="middle" fill="#fff" font-weight="700" font-family="Barlow, sans-serif">E</text>
        </g>
      {/if}
      {#if an && f.ausstich}
        <g pointer-events="none">
          <rect x={f.ausstich[0] - 2.3} y={-f.ausstich[1] - 2.3} width="4.6" height="4.6" fill="#9b1c1c" stroke="#fff" stroke-width=".4" />
          <text x={f.ausstich[0]} y={-f.ausstich[1] + 1.1} font-size="2.8" text-anchor="middle" fill="#fff" font-weight="700" font-family="Barlow, sans-serif">A</text>
        </g>
      {/if}
    {/each}
  {:else}
    {#each zuege.zuege as z, k (k)}
      {@const rand =
        dk && hell(z.farbe) < 0.25
          ? schattieren(z.farbe, 0.45)
          : hell(z.farbe) > 0.8
            ? schattieren(z.farbe, -0.45)
            : schattieren(z.farbe, -0.38)}
      <g data-o={z.o} opacity={gewaehlt.length && !gewaehlt.includes(z.o) ? 0.28 : 1} style="cursor:pointer">
        <path d={z.d} fill="none" stroke="transparent" stroke-width="1.6" pointer-events="stroke" />
        <path d={z.d} fill="none" stroke={rand} stroke-width=".34" stroke-linejoin="round" stroke-linecap="round" pointer-events="none" />
        <path d={z.d} fill="none" stroke={z.farbe} stroke-width=".2" stroke-linejoin="round" stroke-linecap="round" pointer-events="none" />
      </g>
    {/each}
    {#if spruenge && zuege.spruenge}
      <path d={zuege.spruenge} stroke={farben.sprung} stroke-width=".18" stroke-dasharray=".9 .7" fill="none" pointer-events="none" />
    {/if}
    {#if einstiche && zuege.einstiche}
      <path d={zuege.einstiche} stroke={farben.tinte} stroke-width=".28" stroke-linecap="round" pointer-events="none" />
    {/if}
    {#if zuege.aussen}
      <path d={zuege.aussen} stroke={farben.fehler} stroke-width=".3" pointer-events="none" />
    {/if}
  {/if}

  {#each begrenzungen as [x0, y0, x1, y1], k (k)}
    {@const p = 1.5}
    {@const X0 = x0 - p}
    {@const X1 = x1 + p}
    {@const Y0 = -y1 - p}
    {@const Y1 = -y0 + p}
    {@const mx = (X0 + X1) / 2}
    {@const my = (Y0 + Y1) / 2}
    <g pointer-events="none">
      <rect x={X0} y={Y0} width={X1 - X0} height={Y1 - Y0} fill="none" stroke={farben.akzent} stroke-width=".35" stroke-dasharray="1.6 1" />
      {#if begrenzungen.length === 1}
        {#each [[X0, Y0], [mx, Y0], [X1, Y0], [X1, my], [X1, Y1], [mx, Y1], [X0, Y1], [X0, my]] as [x, y], j (j)}
          <rect x={x - 1.1} y={y - 1.1} width="2.2" height="2.2" fill={farben.griff} stroke={farben.akzent} stroke-width=".4" />
        {/each}
        <path d="M{mx} {Y0}V{Y0 - 5}" stroke={farben.akzent} stroke-width=".35" />
        <circle cx={mx} cy={Y0 - 6.2} r="1.3" fill={farben.griff} stroke={farben.akzent} stroke-width=".4" />
        <rect x={mx - 13} y={Y1 + 2} width="26" height="5" fill={farben.akzent} />
        <text x={mx} y={Y1 + 5.6} font-size="3" text-anchor="middle" fill="#fff" font-family="Barlow, sans-serif">
          {zahlFest(x1 - x0)} × {zahlFest(y1 - y0)} mm
        </text>
      {/if}
    </g>
  {/each}

  {#if fehlerBox && arbeit.fehler}
    {@const [x0, y0, x1, y1] = fehlerBox}
    <g pointer-events="none">
      <rect x={x0 - 3} y={-y1 - 3} width={x1 - x0 + 6} height={y1 - y0 + 6} fill="none" stroke={farben.fehler} stroke-width=".55" stroke-dasharray="2.2 1.2" />
      <rect x={x0 - 4} y={-y1 - 9.4} width="21" height="5.4" fill={farben.fehler} />
      <text x={x0 + 6.5} y={-y1 - 5.5} font-size="3.2" text-anchor="middle" fill="#fff" font-weight="700" font-family="Barlow, sans-serif">
        Fehler · Nr. {(arbeit.fehler.objekt ?? 0) + 1}
      </text>
    </g>
  {/if}
</svg>

<style>
  svg {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    display: block;
    touch-action: none;
  }
</style>
