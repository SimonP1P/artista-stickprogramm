<script lang="ts">
  // Stichvorschau (Herzstück, größte Fläche): zeichnet die Stiche aus `design_berechnen` und die Formen aus
  // `design_formen` – rechnet selbst keine Geometrie. Auswahl per Klick auf die Stiche: jeder Pfad trägt die
  // Objektnummer (`Ergebnis.objekt` je Stich), die Trefferprüfung macht der Browser. Darstellung wie Entwurf v3.
  import { arbeit } from "./arbeit.svelte";
  import { rahmenName } from "./listen.svelte";
  import { zahl, zahlFest } from "./zahl";
  import { knotenBild, knotenListe, knotenSetzen, knotenUmschalten, lage } from "./knoten";
  import { AUTOMATISCH_FERTIG, MINDESTENS, pfadD, ZIEHBAR, type ZeichenPunkt } from "./zeichnen";
  import type { FormWerkzeug } from "./einfuegen";

  let {
    spruenge = true,
    einstiche = false,
    raster = true,
    zoom = $bindable(1),
    werkzeug = $bindable("auswahl"),
    zeichenForm = "linie",
    gezeichnet,
    bis = null,
  }: {
    spruenge?: boolean;
    einstiche?: boolean;
    raster?: boolean;
    zoom?: number;
    /** Werkzeug: auswählen/ziehen oder Ein-/Ausstich des gewählten Objekts setzen */
    werkzeug?: "auswahl" | "einstich" | "ausstich" | "knoten" | "zeichnen";
    /** Form beim Zeichnen mit der Maus (Werkzeug „zeichnen“) */
    zeichenForm?: FormWerkzeug;
    /** Zeichnen fertig: Referenzpunkte der Form (Strg/⌘ beim letzten Klick = Quadrat) */
    gezeichnet?: (form: FormWerkzeug, punkte: ZeichenPunkt[], quadrat: boolean, gezogen: boolean) => void;
    /** Abspielen: nur die ersten `bis` Stiche zeigen (null = alle) */
    bis?: number | null;
  } = $props();

  let svg: SVGSVGElement | undefined = $state();
  let mitte = $state<[number, number]>([0, 0]);
  /** laufende Maus-Aktion: Ansicht verschieben, Objekte verschieben, skalieren, drehen */
  type Ziehen =
    | { art: "ansicht"; x: number; y: number; m: [number, number]; bewegt: boolean; nurKlick?: boolean }
    | { art: "rechteck"; start: [number, number]; mehr: boolean; bewegt: boolean }
    | { art: "objekt" | "skalieren" | "drehen"; start: [number, number]; mitte: [number, number]; bewegt: boolean };
  let ziehen: Ziehen | null = null;
  /** Auswahlrechteck während des Ziehens (mm: x0, y0, x1, y1) */
  let band = $state<[number, number, number, number] | null>(null);
  /** Leertaste gedrückt (Zeiger über der Vorschau): Ziehen verschiebt die Ansicht */
  let leertaste = $state(false);
  let zeigerDrin = false;
  /** beim Drücken erst gewählt – der Klick wählt dann keinen einzelnen Buchstaben (W9) */
  let frischGewaehlt = false;
  function istEingabe(t: EventTarget | null) {
    return !!(t as HTMLElement | null)?.closest?.("input, textarea, select, [contenteditable]");
  }
  function tasteRunter(ev: KeyboardEvent) {
    if (ev.code !== "Space" || !zeigerDrin || istEingabe(ev.target)) return;
    ev.preventDefault();
    leertaste = true;
  }
  function tasteHoch(ev: KeyboardEvent) {
    if (ev.code === "Space") leertaste = false;
  }
  /** Live-Vorschau der Griffe (SVG-Transformation der gewählten Objekte, nur Anzeige) */
  let griffTransform = $state("");
  let griffWert = $state<{ dx: number; dy: number; f: number; grad: number }>({ dx: 0, dy: 0, f: 1, grad: 0 });
  /** Stand der Berechnung beim Loslassen eines Griffs; die Live-Ansicht bleibt, bis ein neueres Ergebnis da ist */
  let griffHaltenBis = $state<number | null>(null);
  $effect(() => {
    if (griffHaltenBis === null) return;
    if (arbeit.stand !== griffHaltenBis) {
      griffTransform = "";
      griffHaltenBis = null;
      return;
    }
    const t = setTimeout(() => {
      griffTransform = "";
      griffHaltenBis = null;
    }, 4000);
    return () => clearTimeout(t);
  });

  // ---- Knoten-Werkzeug (W5): Knoten aus dem JSON des gewählten Objekts (Punktlisten, Bézier-Knoten mit Anfassern,
  // `knoten.ts`); Formen und Transformationen wandelt der Kern um (Knopf „In Knoten umwandeln“, `in_knoten_umwandeln`)
  const knotenObjekt = $derived(werkzeug === "knoten" && arbeit.auswahl.length === 1 ? arbeit.auswahl[0] : -1);
  const knotenBasis = $derived(knotenObjekt >= 0 ? knotenBild(arbeit.objekte[knotenObjekt]) : null);
  let knotenZiehen: { k: number; p: [number, number] } | null = $state(null);
  let knotenGewaehlt = $state<number | null>(null);
  const rundenKnoten = (v: number) => Math.round(v * 100) / 100;
  /** während des Ziehens: Knoten und Linien aus einer verschobenen Kopie */
  const knotenAnzeige = $derived.by(() => {
    if (!knotenBasis || !knotenZiehen) return knotenBasis;
    const k = knotenBasis.knoten[knotenZiehen.k];
    if (!k) return knotenBasis;
    const kopie = JSON.parse(JSON.stringify(arbeit.objekte[knotenObjekt]));
    knotenSetzen(kopie, k, knotenZiehen.p);
    return knotenBild(kopie);
  });

  function knotenAendern(f: (o: unknown) => void) {
    const nr = knotenObjekt;
    arbeit.aendern((d) => f(d.objekte![nr]));
  }
  /** gewählten Knoten löschen bzw. dahinter einen in der Mitte zum nächsten einfügen (Tasten Entf / Einfg) – nur in
   *  Punktlisten; Bézier-Knoten bleiben (ihre Zahl ändert der Kern beim Umwandeln) */
  export function knotenTaste(taste: string): boolean {
    const g = knotenGewaehlt;
    const k = g === null ? undefined : knotenBasis?.knoten[g];
    if (!k) return false;
    const liste = knotenListe(arbeit.objekte[knotenObjekt], k);
    if (!liste || !k.liste) return taste === "Delete" || taste === "Backspace" || taste === "Insert" || taste === "+";
    const i = k.liste.index;
    const pfad = k.liste.pfad;
    const lesen = (o: unknown) => pfad.reduce<unknown>((x, t) => (x as Record<string, unknown>)[t as string], o) as unknown[];
    if (taste === "Delete" || taste === "Backspace") {
      if (liste.length > 2) knotenAendern((o) => lesen(o).splice(i, 1));
      knotenGewaehlt = null;
      return true;
    }
    if (taste === "Insert" || taste === "+") {
      const a = lage(liste[i]), b = lage(liste[(i + 1) % liste.length]);
      knotenAendern((o) => lesen(o).splice(i + 1, 0, [rundenKnoten((a[0] + b[0]) / 2), rundenKnoten((a[1] + b[1]) / 2)]));
      knotenGewaehlt = g! + 1;
      return true;
    }
    return false;
  }

  // ---- Zeichnen mit der Maus (Nachtauftrag N0, Handbuch S. 58–62): linke Taste = Eckpunkt, rechte = Kurvenpunkt
  let zeichenPunkte = $state<ZeichenPunkt[]>([]);
  let zeichenZeiger = $state<[number, number] | null>(null);
  let zeichenQuadrat = $state(false);
  /** Kreis/Rechteck durch Ziehen: Bildschirmlage beim Drücken des ersten Punkts */
  let zeichenZug = $state<{ x: number; y: number } | null>(null);
  $effect(() => {
    // Werkzeug oder Form gewechselt: angefangene Form verwerfen
    void werkzeug;
    void zeichenForm;
    zeichenPunkte = [];
  });
  function zeichnenFertig(gezogen = false) {
    const pts = zeichenPunkte;
    if (pts.length < MINDESTENS[zeichenForm]) return false;
    zeichenPunkte = [];
    gezeichnet?.(zeichenForm, pts, zeichenQuadrat, gezogen);
    return true;
  }
  /** Tasten beim Zeichnen: Enter beendet, ⌫ nimmt den letzten Punkt zurück, Esc verwirft die angefangene Form */
  export function zeichenTaste(taste: string): boolean {
    if (werkzeug !== "zeichnen") return false;
    if (taste === "Enter") return zeichnenFertig() || true;
    if (taste === "Backspace" || taste === "Delete") {
      zeichenPunkte = zeichenPunkte.slice(0, -1);
      return true;
    }
    if (taste === "Escape" && zeichenPunkte.length) {
      zeichenPunkte = [];
      return true;
    }
    return false;
  }
  export function zeichnenAbschliessen() {
    zeichnenFertig();
  }
  export const zeichenAnzahl = () => zeichenPunkte.length;
  /** Vorschau der angefangenen Form (mit dem Zeiger als nächstem Punkt) */
  const zeichenBild = $derived.by(() => {
    if (werkzeug !== "zeichnen") return null;
    const pts = zeichenPunkte;
    const z = zeichenZeiger;
    const mitZeiger = z ? [...pts, { p: z, kurve: false }] : pts;
    if (!mitZeiger.length) return null;
    const f = zeichenForm;
    if (f === "knopfloch" && z) {
      // Umriss des Knopflochs am Zeiger (Standard 20 mm senkrecht) wie „The current buttonhole displays as an outline“
      return { d: `M${z[0]} ${-(z[1] - 10)}L${z[0]} ${-(z[1] + 10)}M${z[0] - 2} ${-(z[1] - 10)}L${z[0] + 2} ${-(z[1] - 10)}M${z[0] - 2} ${-(z[1] + 10)}L${z[0] + 2} ${-(z[1] + 10)}`, punkte: [] };
    }
    if (f === "kreis" && zeichenZug && pts.length === 1 && z) {
      // gezogen: Oval im Rechteck zwischen Anfang und Zeiger, Umschalt/Strg = Kreis
      const [p0, p1] = [pts[0].p, z];
      let [a, b] = [Math.abs(p1[0] - p0[0]) / 2, Math.abs(p1[1] - p0[1]) / 2];
      if (zeichenQuadrat) a = b = Math.max(a, b);
      const c: [number, number] = [p0[0] + Math.sign(p1[0] - p0[0] || 1) * a, p0[1] + Math.sign(p1[1] - p0[1] || 1) * b];
      return { ellipse: { c, a, b, grad: 0 }, d: "", punkte: pts };
    }
    if (f === "kreis" && mitZeiger.length >= 2) {
      const [c, r1] = [mitZeiger[0].p, mitZeiger[1].p];
      const a = Math.hypot(r1[0] - c[0], r1[1] - c[1]);
      let b = a;
      if (mitZeiger.length >= 3 && a > 0) {
        const r2 = mitZeiger[2].p;
        const [ux, uy] = [(r1[0] - c[0]) / a, (r1[1] - c[1]) / a];
        b = Math.abs((r2[0] - c[0]) * -uy + (r2[1] - c[1]) * ux);
      }
      const grad = (Math.atan2(r1[1] - c[1], r1[0] - c[0]) * 180) / Math.PI;
      return { ellipse: { c, a, b, grad }, d: `M${c[0]} ${-c[1]}L${r1[0]} ${-r1[1]}`, punkte: pts };
    }
    if (f === "rechteck" && mitZeiger.length >= 2) {
      const [p0, p1] = [mitZeiger[0].p, mitZeiger[1].p];
      let [dx, dy] = [p1[0] - p0[0], p1[1] - p0[1]];
      if (zeichenQuadrat) {
        const s_ = Math.max(Math.abs(dx), Math.abs(dy));
        [dx, dy] = [Math.sign(dx || 1) * s_, Math.sign(dy || 1) * s_];
      }
      const [x0, y0, x1, y1] = [p0[0], p0[1], p0[0] + dx, p0[1] + dy];
      return { d: `M${x0} ${-y0}L${x1} ${-y0}L${x1} ${-y1}L${x0} ${-y1}Z`, punkte: pts };
    }
    if (f === "saeule") {
      const a = mitZeiger.filter((_, i) => i % 2 === 0);
      const b = mitZeiger.filter((_, i) => i % 2 === 1);
      const paare = a
        .slice(0, b.length)
        .map((x, i) => `M${x.p[0]} ${-x.p[1]}L${b[i].p[0]} ${-b[i].p[1]}`)
        .join("");
      return { d: pfadD(a, false) + (b.length ? pfadD(b, false) : ""), paare, punkte: pts };
    }
    return { d: pfadD(mitZeiger, f === "frei" && mitZeiger.length > 2), punkte: pts };
  });

  /** Bildschirmpunkt → mm (y nach oben) über die Abbildung des SVG */
  function mm(ev: PointerEvent): [number, number] {
    const m = svg!.getScreenCTM()!.inverse();
    const p = new DOMPoint(ev.clientX, ev.clientY).matrixTransform(m);
    return [p.x, -p.y];
  }

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
    const out: { o: number; b: number; farbe: string; d: string }[] = [];
    const sprungD: string[] = [];
    const punkteD: string[] = [];
    if (!e) return { zuege: out, spruenge: "", einstiche: "", aussen: "", nadel: null as [number, number] | null };
    let block = 0;
    let aktuell: { o: number; b: number; farbe: string; d: string } | null = null;
    let vorher: [number, number] | null = null;
    let springen = false;
    let nadel: [number, number] | null = null;
    const ende = bis === null ? e.stiche.length : Math.min(bis, e.stiche.length);
    for (let i = 0; i < ende; i++) {
      const s = e.stiche[i];
      nadel = [s.x, s.y];
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
      const b = e.buchstabe?.[i] ?? 0;
      let neuerPfad = springen;
      if (!aktuell || aktuell.o !== o || aktuell.b !== b) {
        aktuell = { o, b, farbe: (e.farben[block] ?? "#000000").toLowerCase(), d: "" };
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
    return { zuege: out, spruenge: sprungD.join(""), einstiche: punkteD.join(""), aussen, nadel: bis === null ? null : nadel };
  });

  const gewaehlt = $derived(arbeit.ansicht !== "abspielen" ? arbeit.auswahl : []);
  const begrenzungen = $derived.by(() => {
    const b = gewaehlt.map((i) => arbeit.formen.find((f) => f.objekt === i)?.begrenzung).filter((b): b is number[] => !!b);
    // ganze Gruppe gewählt: ein gemeinsamer Rahmen mit Griffen (Umriss der Begrenzungen des Kerns)
    if (arbeit.auswahlGruppe && b.length > 1)
      return [[Math.min(...b.map((x) => x[0])), Math.min(...b.map((x) => x[1])), Math.max(...b.map((x) => x[2])), Math.max(...b.map((x) => x[3]))]];
    return b;
  });
  const fehlerBox = $derived.by(() => {
    const o = arbeit.fehler?.objekt;
    if (o === null || o === undefined) return null;
    return arbeit.formen.find((f) => f.objekt === o)?.begrenzung ?? null;
  });

  function objektAusZiel(t: EventTarget | null): number | null {
    const el = (t as Element | null)?.closest?.("[data-o]");
    return el ? Number(el.getAttribute("data-o")) : null;
  }

  /** Mitte der Begrenzung des gewählten Objekts (aus `design_formen`) */
  function auswahlMitte(): [number, number] | null {
    const b = begrenzungen[0];
    return b ? [(b[0] + b[2]) / 2, (b[1] + b[3]) / 2] : null;
  }

  function runter(ev: PointerEvent) {
    // Zeichnen: linke Taste Eckpunkt, rechte Kurvenpunkt (mittlere Taste/Leertaste verschieben weiter die Ansicht)
    if (werkzeug === "zeichnen" && (ev.button === 0 || ev.button === 2) && !leertaste && arbeit.ansicht !== "abspielen") {
      ev.preventDefault();
      const q = mm(ev);
      const r = (v: number) => Math.round(v * 100) / 100;
      zeichenQuadrat = ev.ctrlKey || ev.metaKey || ev.shiftKey;
      // Kreis, Rechteck und Säule: Kurvenpunkte nur bei Linie, Form und Säule
      const kurve = ev.button === 2 && (zeichenForm === "linie" || zeichenForm === "frei" || zeichenForm === "saeule");
      zeichenPunkte = [...zeichenPunkte, { p: [r(q[0]), r(q[1])], kurve }];
      zeichenZug = ev.button === 0 && zeichenPunkte.length === 1 && ZIEHBAR.includes(zeichenForm) ? { x: ev.clientX, y: ev.clientY } : null;
      const auto = AUTOMATISCH_FERTIG[zeichenForm];
      if (auto && zeichenPunkte.length >= auto) zeichnenFertig();
      return;
    }
    // Knoten-Werkzeug: Rechtsklick auf einen Knoten schaltet Ecke ↔ Kurve um (wie die Punktarten im Original)
    if (ev.button === 2 && werkzeug === "knoten") {
      const kn = (ev.target as Element | null)?.closest?.("[data-knoten]")?.getAttribute("data-knoten");
      const k = kn != null ? knotenBasis?.knoten[Number(kn)] : undefined;
      if (k) {
        ev.preventDefault();
        knotenAendern((o) => void knotenUmschalten(o, k));
        knotenGewaehlt = Number(kn);
      }
      return;
    }
    if (ev.button !== 0 && ev.button !== 1) return;
    frischGewaehlt = false;
    try {
      svg?.setPointerCapture(ev.pointerId);
    } catch {
      // nachgespielte Zeiger (Bildschirmfoto-Werkzeug) haben keine echte Zeiger-Nummer
    }
    const ziel = (ev.target as Element | null)?.closest?.("[data-griff],[data-o]");
    const griff = ziel?.getAttribute("data-griff");
    const o = ziel?.getAttribute("data-o");
    const p = mm(ev);
    // Knoten haben weder data-griff noch data-o – eigens suchen (sonst verschob der Klick das ganze Objekt)
    const knoten = (ev.target as Element | null)?.closest?.("[data-knoten]")?.getAttribute("data-knoten");
    if (ev.button === 0 && knoten && werkzeug === "knoten") {
      const k = Number(knoten);
      knotenZiehen = { k, p: knotenBasis?.knoten[k]?.p ?? p };
      knotenGewaehlt = k;
      return;
    }
    // Ansicht verschieben: mittlere Maustaste oder Leertaste + Ziehen
    if (ev.button === 1 || leertaste) {
      ziehen = { art: "ansicht", x: ev.clientX, y: ev.clientY, m: [...mitte], bewegt: false };
      return;
    }
    if (ev.button === 0 && arbeit.ansicht !== "abspielen") {
      // Ein-/Ausstich setzen
      if ((werkzeug === "einstich" || werkzeug === "ausstich") && arbeit.auswahl.length === 1) {
        const i = arbeit.auswahl[0];
        const feld = werkzeug;
        arbeit.aendern((d) => {
          (d.objekte![i] as Record<string, unknown>)[feld] = [Math.round(p[0] * 10) / 10, Math.round(p[1] * 10) / 10];
        });
        werkzeug = "auswahl";
        return;
      }
      const m = auswahlMitte();
      griffHaltenBis = null;
      griffTransform = "";
      if ((griff === "ecke" || griff === "dreh") && m) {
        ziehen = { art: griff === "ecke" ? "skalieren" : "drehen", start: p, mitte: m, bewegt: false };
        return;
      }
      const mehr = ev.shiftKey || ev.ctrlKey || ev.metaKey;
      // Knoten-Werkzeug: Objekt anklicken wählt es (seine Knoten erscheinen), Ziehen verschiebt es nicht – nur die
      // Knoten selbst werden gezogen; Ziehen auf freier Fläche oder am Objekt: Auswahlrechteck
      if (werkzeug === "knoten") {
        if (o !== null && o !== undefined && !arbeit.auswahl.includes(Number(o))) {
          arbeit.waehlen(Number(o));
          frischGewaehlt = true;
        }
        knotenGewaehlt = null;
        ziehen = { art: "rechteck", start: p, mehr, bewegt: false };
        return;
      }
      if (o !== null && o !== undefined && !mehr) {
        // nicht gewähltes Objekt anfassen: wählen und gleich verschieben können
        frischGewaehlt = !arbeit.auswahl.includes(Number(o));
        if (frischGewaehlt) arbeit.waehlen(Number(o));
        ziehen = { art: "objekt", start: p, mitte: m ?? p, bewegt: false };
        return;
      }
      // freie Fläche: Auswahlrechteck (Original: „drag a bounding box around the objects“)
      if ((o === null || o === undefined) && werkzeug === "auswahl") {
        ziehen = { art: "rechteck", start: p, mehr, bewegt: false };
        return;
      }
    }
    // linke Taste verschiebt die Ansicht nicht (Wunsch Nutzer 21.11.2026: nur mittlere Taste, Leertaste, Touchpad) –
    // nur Klick (z. B. beim Abspielen ein Objekt wählen)
    ziehen = { art: "ansicht", x: ev.clientX, y: ev.clientY, m: [...mitte], bewegt: false, nurKlick: true };
  }
  function bewegen(ev: PointerEvent) {
    if (werkzeug === "zeichnen" && svg && !ziehen) {
      zeichenZeiger = mm(ev);
      zeichenQuadrat = ev.ctrlKey || ev.metaKey || ev.shiftKey;
      return;
    }
    if (knotenZiehen && svg) {
      knotenZiehen = { ...knotenZiehen, p: mm(ev) };
      return;
    }
    if (!ziehen || !svg) return;
    if (ziehen.art === "ansicht") {
      const dx = ev.clientX - ziehen.x, dy = ev.clientY - ziehen.y;
      if (!ziehen.bewegt && Math.hypot(dx, dy) < 4) return;
      ziehen.bewegt = true;
      if (ziehen.nurKlick) return;
      const s = ausschnitt.w / svg.clientWidth;
      const k = Math.max(s, ausschnitt.h / svg.clientHeight);
      mitte = [ziehen.m[0] - dx * k, ziehen.m[1] + dy * k];
      return;
    }
    const p = mm(ev);
    const [sx, sy] = ziehen.start;
    if (ziehen.art === "rechteck") {
      if (!ziehen.bewegt && Math.hypot(p[0] - sx, p[1] - sy) < 1 / zoom) return;
      ziehen.bewegt = true;
      band = [Math.min(sx, p[0]), Math.min(sy, p[1]), Math.max(sx, p[0]), Math.max(sy, p[1])];
      return;
    }
    const [cx, cy] = ziehen.mitte;
    if (!ziehen.bewegt && Math.hypot(p[0] - sx, p[1] - sy) < 0.4 / zoom) return;
    ziehen.bewegt = true;
    if (ziehen.art === "objekt") {
      griffWert = { dx: p[0] - sx, dy: p[1] - sy, f: 1, grad: 0 };
      griffTransform = `translate(${griffWert.dx} ${-griffWert.dy})`;
    } else if (ziehen.art === "skalieren") {
      const f = Math.max(0.05, Math.hypot(p[0] - cx, p[1] - cy) / Math.max(0.01, Math.hypot(sx - cx, sy - cy)));
      griffWert = { dx: 0, dy: 0, f, grad: 0 };
      griffTransform = `translate(${cx} ${-cy}) scale(${f}) translate(${-cx} ${cy})`;
    } else {
      let grad = ((Math.atan2(p[1] - cy, p[0] - cx) - Math.atan2(sy - cy, sx - cx)) * 180) / Math.PI;
      if (ev.shiftKey) grad = Math.round(grad / 15) * 15;
      griffWert = { dx: 0, dy: 0, f: 1, grad };
      griffTransform = `rotate(${-grad} ${cx} ${-cy})`;
    }
  }
  function hoch(ev: PointerEvent) {
    // Kreis/Rechteck gezogen (drücken, ziehen, loslassen): Loslassen setzt den zweiten Punkt und schließt ab
    if (zeichenZug && werkzeug === "zeichnen") {
      const z = zeichenZug;
      zeichenZug = null;
      if (zeichenPunkte.length === 1 && Math.hypot(ev.clientX - z.x, ev.clientY - z.y) > 6) {
        const q = mm(ev);
        const r = (v: number) => Math.round(v * 100) / 100;
        zeichenQuadrat = ev.ctrlKey || ev.metaKey || ev.shiftKey;
        zeichenPunkte = [...zeichenPunkte, { p: [r(q[0]), r(q[1])], kurve: false }];
        zeichnenFertig(true);
      }
      return;
    }
    if (knotenZiehen) {
      const z = knotenZiehen;
      knotenZiehen = null;
      const k = knotenBasis?.knoten[z.k];
      if (k && (k.p[0] !== z.p[0] || k.p[1] !== z.p[1])) knotenAendern((o) => knotenSetzen(o, k, z.p, rundenKnoten));
      return;
    }
    const war = ziehen;
    ziehen = null;
    if (!war) return;
    if (war.art === "rechteck") {
      const b = band;
      band = null;
      if (war.bewegt && b) return arbeit.imRechteckWaehlen(b, war.mehr);
      if (frischGewaehlt) return void (frischGewaehlt = false);
      // Knoten-Werkzeug: Klick auf das gewählte Objekt behält die Auswahl
      if (werkzeug === "knoten" && objektAusZiel(document.elementFromPoint(ev.clientX, ev.clientY)) !== null) return;
      if (!war.mehr) arbeit.waehlen(null);
      return;
    }
    if (war.art !== "ansicht" && war.bewegt) {
      // Griffe: Wert ins JSON (Transformation des Objekts) – der Kern rechnet die Stiche neu
      const w = griffWert;
      const r = (v: number) => Math.round(v * 100) / 100;
      arbeit.auswahlTransformieren(
        (t) => {
          if (war.art === "objekt") {
            const [vx, vy] = t.verschieben ?? [0, 0];
            t.verschieben = [r(vx + w.dx), r(vy + w.dy)];
          } else if (war.art === "skalieren") {
            const alt = Array.isArray(t.skalieren) ? (t.skalieren[0] ?? 100) : (t.skalieren ?? 100);
            t.skalieren = r(alt * w.f);
          } else t.drehen = r((t.drehen ?? 0) + w.grad);
        },
        undefined,
        war.art !== "objekt",
      );
      // Live-Ansicht stehen lassen, bis der Kern die neuen Stiche liefert – sonst springt das Objekt kurz zurück
      griffHaltenBis = arbeit.stand;
      return;
    }
    griffTransform = "";
    const bewegt = war.art === "ansicht" ? war.bewegt : war.bewegt;
    if (!bewegt && ev.button === 0) {
      // Pointer-Capture lenkt das Ziel um – Element unter dem Zeiger suchen
      const ziel = document.elementFromPoint(ev.clientX, ev.clientY);
      const o = objektAusZiel(ziel);
      const b = Number(ziel?.closest?.("[data-b]")?.getAttribute("data-b") ?? 0);
      const mehr = ev.shiftKey || ev.ctrlKey || ev.metaKey;
      if (frischGewaehlt) return void (frischGewaehlt = false);
      // Klick auf einen Buchstaben des schon gewählten Texts: Buchstabe wählen (W9, nur Experte)
      if (!mehr && o !== null && b > 0 && arbeit.modus === "experte" && arbeit.auswahl.length === 1 && arbeit.auswahl[0] === o) {
        arbeit.buchstabe = arbeit.buchstabe?.objekt === o && arbeit.buchstabe.nr === b ? null : { objekt: o, nr: b };
        return;
      }
      arbeit.waehlen(o, mehr);
    }
  }
  /** Bildschirmpunkt → mm für Rad-Ereignisse */
  function mmAus(x: number, y: number): [number, number] {
    const p = new DOMPoint(x, y).matrixTransform(svg!.getScreenCTM()!.inverse());
    return [p.x, -p.y];
  }
  /** auf `neu` zoomen, der Punkt unter dem Zeiger bleibt stehen */
  function zoomUm(neu: number, x: number, y: number) {
    const z = Math.min(40, Math.max(0.25, neu));
    const [px, py] = mmAus(x, y);
    const f = zoom / z;
    mitte = [px - (px - mitte[0]) * f, py - (py - mitte[1]) * f];
    zoom = z;
  }
  /**
   * Rad und Touchpad wie in Zeichenprogrammen: Mausrad zoomt (um den Zeiger), Touchpad mit zwei Fingern verschiebt die
   * Ansicht, Zwei-Finger-Zoom (Pinch, kommt als Rad mit Strg) bzw. Strg/⌘ + Rad zoomt. Mausrad und Touchpad
   * unterscheiden: ein Mausrad liefert `wheelDeltaY` in Vielfachen von 120 (WebKit/Chromium), das Touchpad nicht.
   */
  function rad(ev: WheelEvent) {
    ev.preventDefault();
    if (!svg) return;
    if (ev.ctrlKey || ev.metaKey) return zoomUm(zoom * Math.exp(-ev.deltaY * 0.01), ev.clientX, ev.clientY);
    const wd = (ev as WheelEvent & { wheelDeltaY?: number; wheelDeltaX?: number });
    const mausrad =
      ev.deltaMode === 1 || (!ev.deltaX && !!wd.wheelDeltaY && wd.wheelDeltaY % 120 === 0 && !wd.wheelDeltaX);
    if (mausrad) return zoomUm(zoom * (ev.deltaY < 0 ? 1.15 : 1 / 1.15), ev.clientX, ev.clientY);
    const k = Math.max(ausschnitt.w / svg.clientWidth, ausschnitt.h / svg.clientHeight);
    mitte = [mitte[0] + ev.deltaX * k, mitte[1] - ev.deltaY * k];
  }
  export function einpassen() {
    zoom = 1;
    mitte = [0, 0];
  }
</script>

<svelte:window onkeydown={tasteRunter} onkeyup={tasteHoch} onblur={() => (leertaste = false)} />

<!-- svelte-ignore a11y_no_noninteractive_element_interactions (Zeichenfläche; mittlere Maustaste ohne Auto-Scroll) -->
<svg
  bind:this={svg}
  role="img"
  aria-label="Stichvorschau {rahmen?.name ?? ''}"
  viewBox="{ausschnitt.x} {ausschnitt.y} {ausschnitt.w} {ausschnitt.h}"
  preserveAspectRatio="xMidYMid meet"
  onpointerdown={runter}
  onpointermove={bewegen}
  onpointerup={hoch}
  onmousedown={(e) => e.button === 1 && e.preventDefault()}
  onpointerenter={() => (zeigerDrin = true)}
  onpointerleave={() => ((zeigerDrin = false), (zeichenZeiger = null))}
  oncontextmenu={(e) => e.preventDefault()}
  onwheel={rad}
  style="cursor:{leertaste ? 'grab' : werkzeug !== 'auswahl' ? 'crosshair' : 'default'}"
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
      {rahmenName(rahmen.name)} · Stickfeld {zahl(rahmen.breite)} × {zahl(rahmen.hoehe)} mm
    </text>
  {/if}

  {#if arbeit.ansicht === "form"}
    {#each arbeit.formen as f (f.objekt)}
      {@const an = gewaehlt.includes(f.objekt)}
      {@const farbe = (f.farbe || "#000000").toLowerCase()}
      {@const strich =
        dk && hell(farbe) < 0.25 ? schattieren(farbe, 0.55) : hell(farbe) > 0.8 ? schattieren(farbe, -0.5) : farbe}
      <g data-o={f.objekt} opacity={gewaehlt.length && !an ? 0.45 : 1} style="cursor:{an ? 'move' : 'pointer'}" transform={an && griffTransform ? griffTransform : undefined}>
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
      {#if arbeit.buchstabe && arbeit.buchstabe.objekt === z.o && arbeit.buchstabe.nr === z.b && gewaehlt.includes(z.o)}
        <path d={z.d} fill="none" stroke={farben.akzent} stroke-width="1.4" stroke-opacity=".55" stroke-linejoin="round" stroke-linecap="round" pointer-events="none" transform={griffTransform || undefined} />
      {/if}
      <g data-o={z.o} data-b={z.b || undefined} opacity={gewaehlt.length && !gewaehlt.includes(z.o) ? 0.28 : 1} style="cursor:{gewaehlt.includes(z.o) ? 'move' : 'pointer'}" transform={gewaehlt.includes(z.o) && griffTransform ? griffTransform : undefined}>
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

  {#if knotenAnzeige && knotenAnzeige.knoten.length}
    {@const g = 1 / Math.sqrt(zoom)}
    {#each knotenAnzeige.linien as d, i (i)}
      <path {d} fill="none" stroke={farben.akzent} stroke-width=".3" stroke-dasharray="1 .6" pointer-events="none" />
    {/each}
    {#each knotenAnzeige.knoten as k, i (i)}
      {@const [x, y] = k.p}
      {@const an = knotenGewaehlt === i}
      {#if k.anfasser && k.anker}
        <line x1={k.anker[0]} y1={-k.anker[1]} x2={x} y2={-y} stroke={farben.akzent} stroke-width=".2" pointer-events="none" />
        <circle cx={x} cy={-y} r={0.8 * g} fill={an ? farben.akzent : farben.griff} stroke={farben.akzent} stroke-width=".3" data-knoten={i} style="cursor:move" />
      {:else if k.kurve}
        <!-- Kurvenpunkt eines Pfads: rund und größer als ein Anfasser (Rechtsklick = Eckpunkt) -->
        <circle cx={x} cy={-y} r={1.1 * g} fill={an ? farben.akzent : farben.griff} stroke={farben.akzent} stroke-width=".4" data-knoten={i} style="cursor:move" />
      {:else}
        <rect x={x - g} y={-y - g} width={2 * g} height={2 * g} fill={an ? farben.akzent : farben.griff} stroke={farben.akzent} stroke-width=".35" data-knoten={i} style="cursor:move" />
      {/if}
    {/each}
  {/if}
  {#if zeichenBild}
    <!-- Zeichnen: angefangene Form mit dem Zeiger als nächstem Punkt (nur Anzeige) -->
    {@const g = 1 / Math.sqrt(zoom)}
    <g pointer-events="none">
      {#if zeichenBild.ellipse}
        {@const el = zeichenBild.ellipse}
        <ellipse cx={el.c[0]} cy={-el.c[1]} rx={el.a} ry={el.b} transform="rotate({-el.grad} {el.c[0]} {-el.c[1]})" fill="none" stroke={farben.akzent} stroke-width=".35" />
      {/if}
      <path d={zeichenBild.d} fill="none" stroke={farben.akzent} stroke-width=".35" stroke-dasharray={zeichenBild.ellipse ? "1 .6" : undefined} />
      {#if zeichenBild.paare}<path d={zeichenBild.paare} fill="none" stroke={farben.akzent} stroke-width=".2" stroke-dasharray="1 .6" />{/if}
      {#each zeichenBild.punkte as z, i (i)}
        {#if z.kurve}
          <circle cx={z.p[0]} cy={-z.p[1]} r={1.1 * g} fill={farben.griff} stroke={farben.akzent} stroke-width=".4" />
        {:else}
          <rect x={z.p[0] - g} y={-z.p[1] - g} width={2 * g} height={2 * g} fill={farben.griff} stroke={farben.akzent} stroke-width=".35" />
        {/if}
      {/each}
    </g>
  {/if}
  {#if zuege.nadel}
    {@const [nx, ny] = zuege.nadel}
    <g stroke={dk ? "#fff" : "#111"} stroke-width=".3" pointer-events="none">
      <circle cx={nx} cy={-ny} r="2.2" fill="none" /><path d="M{nx - 4} {-ny}h8M{nx} {-ny - 4}v8" />
    </g>
  {/if}
  {#if arbeit.ansicht === "stiche" && gewaehlt.length === 1}
    {@const o = arbeit.objekte[gewaehlt[0]]}
    {#if o?.einstich}
      <g pointer-events="none"><circle cx={o.einstich[0]} cy={-o.einstich[1]} r="2.4" fill="#2f6f3e" stroke="#fff" stroke-width=".4" /><text x={o.einstich[0]} y={-o.einstich[1] + 1.1} font-size="2.8" text-anchor="middle" fill="#fff" font-weight="700" font-family="Barlow, sans-serif">E</text></g>
    {/if}
    {#if o?.ausstich}
      <g pointer-events="none"><rect x={o.ausstich[0] - 2.3} y={-o.ausstich[1] - 2.3} width="4.6" height="4.6" fill="#9b1c1c" stroke="#fff" stroke-width=".4" /><text x={o.ausstich[0]} y={-o.ausstich[1] + 1.1} font-size="2.8" text-anchor="middle" fill="#fff" font-weight="700" font-family="Barlow, sans-serif">A</text></g>
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
    <g transform={griffTransform || undefined}>
      <rect x={X0} y={Y0} width={X1 - X0} height={Y1 - Y0} fill="none" stroke={farben.akzent} stroke-width=".35" stroke-dasharray="1.6 1" pointer-events="none" />
      {#if begrenzungen.length === 1 && arbeit.ansicht !== "abspielen" && werkzeug !== "knoten"}
        {#each [[X0, Y0], [mx, Y0], [X1, Y0], [X1, my], [X1, Y1], [mx, Y1], [X0, Y1], [X0, my]] as [x, y], j (j)}
          <rect
            x={x - 1.1 / Math.sqrt(zoom)}
            y={y - 1.1 / Math.sqrt(zoom)}
            width={2.2 / Math.sqrt(zoom)}
            height={2.2 / Math.sqrt(zoom)}
            fill={farben.griff}
            stroke={farben.akzent}
            stroke-width=".4"
            data-griff={j % 2 === 0 ? "ecke" : undefined}
            style={j % 2 === 0 ? "cursor:nwse-resize" : "pointer-events:none"}
          />
        {/each}
        <path d="M{mx} {Y0}V{Y0 - 5}" stroke={farben.akzent} stroke-width=".35" pointer-events="none" />
        <circle cx={mx} cy={Y0 - 6.2} r={1.3 / Math.sqrt(zoom)} fill={farben.griff} stroke={farben.akzent} stroke-width=".4" data-griff="dreh" style="cursor:grab" />
        <rect x={mx - 13} y={Y1 + 2} width="26" height="5" fill={farben.akzent} />
        <text x={mx} y={Y1 + 5.6} font-size="3" text-anchor="middle" fill="#fff" font-family="Barlow, sans-serif" pointer-events="none">
          {#if griffTransform && griffWert.f !== 1}{zahl(griffWert.f * 100, 0)} %{:else if griffTransform && griffWert.grad}{zahl(griffWert.grad, 0)}°{:else}{zahlFest(x1 - x0)} × {zahlFest(y1 - y0)} mm{/if}
        </text>
      {/if}
    </g>
  {/each}

  {#if band}
    {@const [x0, y0, x1, y1] = band}
    <rect x={x0} y={-y1} width={x1 - x0} height={y1 - y0} fill={farben.akzent} fill-opacity=".08" stroke={farben.akzent} stroke-width={0.3 / Math.sqrt(zoom)} stroke-dasharray="1.2 .8" pointer-events="none" />
  {/if}

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
