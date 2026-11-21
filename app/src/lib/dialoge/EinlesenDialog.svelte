<script lang="ts">
  // Bild umwandeln wie der „Auto Digitizer“ des Originals (`bild_vorschau` + `bild_umwandeln`; Schritte nach Handbuch
  // Kap. 12/13, Verfahren eigene Festlegung) und SVG einlesen (`svg_lesen`, Stichart nach Breite wie EasyStitch).
  // Bild: links das vereinfachte Bild, rechts Einstellungen und die Farbliste – je Farbe sticken oder weglassen
  // („Omitted Colors“). Weggelassene Farben blendet nur die Anzeige aus (Leinwand), gerechnet wird im Kern.
  import { untrack } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { kern, type BildVorschau, type DesignJson } from "$kern/kern";
  import { arbeit, alsFehler, leeresDesign } from "../arbeit.svelte";
  import { dateiname } from "../dateien.svelte";
  import Dialog from "../Dialog.svelte";
  import { garnText } from "../listen.svelte";
  import { lesen, zahl } from "../zahl";

  let { art, schliessen, startPfad }: { art: "bild" | "svg"; schliessen: () => void; startPfad?: string } = $props();

  const experte = $derived(arbeit.modus === "experte");
  // svelte-ignore state_referenced_locally
  let pfad = $state<string | null>(startPfad ?? null);
  // Standardbreite: Bild 80 mm (Kern), SVG wie die Zeichnung
  let breite = $state("");
  $effect.pre(() => {
    if (art === "bild" && breite === "") breite = "80";
  });
  let farben = $state(8);
  let vereinfachen = $state(2);
  /** Farben auf die Garne der Garnkarte des Designs ziehen – nicht bei „Default“ (nur 19 Grundfarben: aus einem Foto
   *  würden Bäume schwarz und khaki) */
  let garnfarben = $state((arbeit.design.garnkarte ?? "Default") !== "Default");
  let fuellung = $state<"step" | "auto">("step");
  let minFlaeche = $state("4");
  let glaettung = $state("0,2");
  let aussparen = $state(true);
  let kuerzeste = $state(true);
  /** bisheriges Design ersetzen statt als Element hinzufügen (Standard: hinzufügen, wie „Insert Design“) */
  let ersetzen = $state(false);
  let laeuft = $state(false);
  let fehler = $state<string | null>(null);
  let hinweise = $state<string[]>([]);

  const karte = $derived(arbeit.design.garnkarte ?? "Default");
  const mitKarte = $derived(garnfarben && karte !== "keine");

  // ---- Vorschau (Bild) ----
  let vorschau = $state<BildVorschau | null>(null);
  let laedt = $state(false);
  let anfrage = 0;
  /** weggelassene Farben („#rrggbb“) der aktuellen Vorschau */
  let weg = $state<string[]>([]);
  /** Wahl des Anwenders (null = Vorschlag des Programms übernehmen) */
  let wahl: string[] | null = null;

  function eingabe() {
    return {
      pfad: pfad!,
      breite: lesen(breite) ?? undefined,
      farben: Math.max(2, Math.min(24, Math.round(farben))),
      vereinfachen,
      min_flaeche: lesen(minFlaeche) ?? undefined,
      glaettung: lesen(glaettung) ?? undefined,
      garnkarte: karte !== "keine" ? karte : undefined,
      garnfarben: mitKarte,
    };
  }

  const rgb = (h: string) => [1, 3, 5].map((i) => parseInt(h.slice(i, i + 2), 16));
  const abstand = (a: string, b: string) => {
    const [x, y] = [rgb(a), rgb(b)];
    return Math.hypot(x[0] - y[0], x[1] - y[1], x[2] - y[2]);
  };

  async function vorschauRechnen(e: ReturnType<typeof eingabe>, nr: number) {
    laedt = true;
    try {
      const v = await kern("bild_vorschau", e);
      if (nr !== anfrage) return;
      vorschau = v;
      fehler = null;
      // Wahl des Anwenders auf die neuen Farben übertragen (nächste Farbe, wenn ähnlich), sonst der Vorschlag
      weg =
        wahl === null
          ? v.farben.filter((f) => f.weglassen).map((f) => f.farbe)
          : v.farben.filter((f) => wahl!.some((w) => abstand(w, f.farbe) < 40)).map((f) => f.farbe);
    } catch (x) {
      if (nr === anfrage) fehler = alsFehler(x).meldung;
    } finally {
      if (nr === anfrage) laedt = false;
    }
  }

  // neu rechnen, wenn sich etwas ändert, das das Bild verändert: das erste Mal sofort, danach kurz gebündelt (Schieber);
  // ältere Antworten verfallen
  $effect(() => {
    if (art !== "bild" || !pfad) return;
    const e = eingabe();
    const nr = ++anfrage;
    if (!untrack(() => vorschau)) {
      vorschauRechnen(e, nr);
      return;
    }
    const t = setTimeout(() => vorschauRechnen(e, nr), 250);
    return () => clearTimeout(t);
  });

  function umschalten(farbe: string) {
    weg = weg.includes(farbe) ? weg.filter((f) => f !== farbe) : [...weg, farbe];
    wahl = [...weg];
  }

  // Anzeige: vereinfachtes Bild, weggelassene Farben durchsichtig (Stoff sichtbar)
  let leinwand: HTMLCanvasElement | undefined = $state();
  $effect(() => {
    const v = vorschau;
    const aus = new Set(weg);
    const c = leinwand;
    if (!v || !c) return;
    const img = new Image();
    img.onload = () => {
      c.width = img.width;
      c.height = img.height;
      const g = c.getContext("2d", { willReadFrequently: true });
      if (!g) return;
      g.clearRect(0, 0, c.width, c.height);
      g.drawImage(img, 0, 0);
      if (!aus.size) return;
      const d = g.getImageData(0, 0, c.width, c.height);
      const hex = (i: number) => "#" + [0, 1, 2].map((k) => d.data[i + k].toString(16).padStart(2, "0")).join("");
      for (let i = 0; i < d.data.length; i += 4) if (d.data[i + 3] && aus.has(hex(i))) d.data[i + 3] = 0;
      g.putImageData(d, 0, 0);
    };
    img.src = v.bild;
  });

  const hoeheMm = $derived(vorschau && lesen(breite) ? ((lesen(breite) ?? 0) * vorschau.hoehe_px) / vorschau.breite_px : null);

  async function dateiWaehlen() {
    const p = await open({
      multiple: false,
      directory: false,
      filters:
        art === "bild"
          ? [{ name: "Bild", extensions: ["png", "jpg", "jpeg", "bmp", "gif"] }]
          : [{ name: "SVG (z. B. aus Inkscape)", extensions: ["svg"] }],
    });
    if (typeof p === "string") {
      pfad = p;
      vorschau = null;
      wahl = null;
    }
  }

  function rahmenStoff() {
    const d = arbeit.design;
    return { name: d.name ?? "Design", rahmen: d.rahmen, stoff: d.stoff, garnkarte: d.garnkarte };
  }

  async function umwandeln() {
    if (!pfad) return;
    laeuft = true;
    fehler = null;
    hinweise = [];
    const rahmen = arbeit.design.rahmen;
    try {
      let design: DesignJson;
      if (art === "bild") {
        const e = await kern("bild_umwandeln", {
          ...eingabe(),
          weglassen: vorschau ? [...weg] : undefined,
          fuellung,
          rahmen,
        });
        design = e.design;
      } else {
        const e = await kern("svg_lesen", {
          pfad,
          breite: lesen(breite) ?? undefined,
          rahmen,
          aussparen,
          kuerzeste_verbindung: kuerzeste,
        });
        design = e.design;
        hinweise = e.hinweise;
      }
      // als **ein Element** einfügen: alle Objekte in einer Gruppe (Größe/Drehen/Lage gelten für das ganze Bild)
      const ziel = ersetzen || !arbeit.objekte.length ? { ...leeresDesign(), ...rahmenStoff() } : $state.snapshot(arbeit.design);
      const r = await kern("design_einfuegen", {
        design: ziel as DesignJson,
        einfuegen: design,
        gruppe: { name: pfad ? dateiname(pfad).replace(/\.[^.]+$/, "") : art, herkunft: art },
      });
      arbeit.ersetzen(r.design, ersetzen || !arbeit.objekte.length ? null : arbeit.pfad, true);
      arbeit.auswahl = r.objekte;
      if (hinweise.length) arbeit.zeige(`Eingelesen mit ${hinweise.length} Hinweis(en): ${hinweise.join(" · ")}`, "info");
      schliessen();
    } catch (x) {
      fehler = alsFehler(x).meldung;
    } finally {
      laeuft = false;
    }
  }

  const STUFEN = ["aus", "sehr fein", "fein", "mittel", "grob", "sehr grob"];
</script>

<Dialog titel={art === "bild" ? "Bild umwandeln" : "SVG einlesen"} breite={art === "bild" ? 880 : 480} {schliessen}>
  {#if art === "bild"}
    <div class="bild-dlg">
      <!-- links: vereinfachtes Bild -->
      <div class="bild-links">
        <div class="bild-feld" class:leer={!vorschau}>
          {#if vorschau}
            <canvas bind:this={leinwand} aria-label="Vorschau des vereinfachten Bildes"></canvas>
          {:else if pfad}
            <span class="mut small">{laedt ? "Bild wird vorbereitet …" : ""}</span>
          {:else}
            <button class="btn btn-secondary" onclick={dateiWaehlen}>Bild wählen …</button>
          {/if}
          {#if laedt && vorschau}<span class="bild-laedt small">rechnet …</span>{/if}
        </div>
        <p class="mut small" style="margin:6px 0 0">
          So wird das Bild gestickt: jede Fläche eine Farbe. Kariert = Stoff bleibt frei (Farbe weggelassen).
          {#if vorschau && hoeheMm}Größe {zahl(lesen(breite) ?? 0, 1)} × {zahl(hoeheMm, 1)} mm.{/if}
        </p>
      </div>
      <!-- rechts: Einstellungen -->
      <div class="bild-rechts">
        <div class="frow" style="padding:0">
          <span class="fl">Datei</span>
          <div class="fc">
            <button class="btn btn-secondary pick" onclick={dateiWaehlen}><span class="t">{pfad ? dateiname(pfad) : "Datei wählen …"}</span></button>
          </div>
        </div>
        <div class="frow" style="padding:0">
          <span class="fl">Breite</span>
          <div class="fc"><input class="input sm num" inputmode="decimal" bind:value={breite} style="width:80px" /><span class="unit">mm</span></div>
        </div>
        <div class="frow" style="padding:0">
          <label class="fl" for="bfarben">Farben</label>
          <div class="fc" style="gap:8px">
            <input id="bfarben" type="range" min="2" max="16" step="1" bind:value={farben} style="flex:1" />
            <span class="tab" style="width:22px; text-align:right">{farben}</span>
          </div>
        </div>
        <div class="frow" style="padding:0">
          <label class="fl" for="bvereinf" title="Glättet Fotorauschen und feine Einzelheiten, Kanten bleiben scharf – wie „Artwork Preparation“ im Original">Vereinfachen</label>
          <div class="fc" style="gap:8px">
            <input id="bvereinf" type="range" min="0" max="5" step="1" bind:value={vereinfachen} style="flex:1" />
            <span class="small" style="width:62px">{STUFEN[vereinfachen]}</span>
          </div>
        </div>
        <label class="frow" style="padding:0" title="Jede Farbe wird das nächste Garn der Garnkarte – gleiche Garne werden eine Farbe">
          <span class="fl">Garnfarben</span>
          <span class="fc"><input type="checkbox" bind:checked={garnfarben} disabled={karte === "keine"} /> aus der Karte „{karte}“</span>
        </label>
        {#if garnfarben && karte === "Default"}
          <p class="small mut" style="margin:-4px 0 0">„Default“ hat nur 19 Grundfarben – für Fotos besser eine echte Garnkarte (Isacord, Madeira …) im Schritt „Prüfen &amp; Garn“ wählen.</p>
        {/if}
        {#if experte}
          <div class="frow" style="padding:0">
            <span class="fl">Stichart <span class="xt">X</span></span>
            <div class="fc">
              <select class="input sm" bind:value={fuellung}>
                <option value="step">Step – sauber bei Fotos</option>
                <option value="auto">Auto – schmale Flächen Satin (Logos)</option>
              </select>
            </div>
          </div>
          <div class="frow" style="padding:0">
            <span class="fl">Kleinste Fläche <span class="xt">X</span></span>
            <div class="fc"><input class="input sm num" inputmode="decimal" bind:value={minFlaeche} style="width:80px" /><span class="unit">mm²</span></div>
          </div>
          <div class="frow" style="padding:0">
            <span class="fl">Glättung <span class="xt">X</span></span>
            <div class="fc"><input class="input sm num" inputmode="decimal" bind:value={glaettung} style="width:80px" /><span class="unit">mm</span></div>
          </div>
        {/if}

        {#if vorschau}
          <div class="mh" style="padding:6px 0 2px">Farben – Haken = wird gestickt</div>
          <div class="bild-farben">
            {#each vorschau.farben as f (f.farbe)}
              {@const an = !weg.includes(f.farbe)}
              <label class="bild-farbe" class:aus={!an}>
                <input type="checkbox" checked={an} onchange={() => umschalten(f.farbe)} />
                <span class="sw" style="background:{f.farbe}"></span>
                <span class="small" style="flex:1; min-width:0; overflow:hidden; text-overflow:ellipsis; white-space:nowrap">{f.garn ? (mitKarte ? "" : "≈ ") + garnText(f.garn) : f.farbe}</span>
                <span class="small mut tab">{zahl(f.anteil, 1)} %</span>
              </label>
            {/each}
          </div>
          {#if vorschau.farben.some((f) => f.weglassen) && weg.length}
            <p class="small mut" style="margin:2px 0 0">Weißer Hintergrund ist zum Weglassen vorgeschlagen (wie im Original) – Haken setzen, dann wird er gestickt.</p>
          {/if}
        {/if}
        {#if arbeit.objekte.length}
          <label class="frow" style="padding:0"><span class="fl">Bisheriges</span><span class="fc"><input type="checkbox" bind:checked={ersetzen} /> Design ersetzen</span></label>
        {/if}
      </div>
    </div>
    <div class="note small" style="margin-top:10px">
      Das Bild wird {ersetzen || !arbeit.objekte.length ? "das neue Design" : "als neues Element hinzugefügt"} – als Gruppe: Größe, Drehen und Lage gelten für alles zusammen. Zugausgleich 0,4 mm und Unterlage wie vom Handbuch empfohlen. Rückgängig möglich.
    </div>
    {#if fehler}<div class="errbox" style="margin-top:8px">{fehler}</div>{/if}
  {:else}
    <div style="display:flex; flex-direction:column; gap:10px">
      <p class="mut small" style="margin:0">
        Eine SVG-Zeichnung (z. B. aus Inkscape; Text vorher in Pfade umwandeln) wird zum neuen Design. Die Stichart je
        Form wählt das Programm nach der Breite wie EasyStitch des Originals (Linie bis 2 mm Laufstich, Fläche bis 8 mm
        Satin, darüber Tatami).
      </p>
      <div class="frow" style="padding:0">
        <span class="fl">Datei</span>
        <div class="fc">
          <button class="btn btn-secondary pick" onclick={dateiWaehlen}><span class="t">{pfad ? dateiname(pfad) : "Datei wählen …"}</span></button>
        </div>
      </div>
      <div class="frow" style="padding:0">
        <span class="fl">Breite</span>
        <div class="fc">
          <input class="input sm num" inputmode="decimal" bind:value={breite} placeholder="wie SVG" style="width:90px" />
          <span class="unit">mm</span>
        </div>
      </div>
      {#if experte}
        <label class="frow" style="padding:0"><span class="fl">Überdeckung <span class="xt">X</span></span><span class="fc"><input type="checkbox" bind:checked={aussparen} /> aussparen (wie die SVG aussieht)</span></label>
        <label class="frow" style="padding:0"><span class="fl">Verbindung <span class="xt">X</span></span><span class="fc"><input type="checkbox" bind:checked={kuerzeste} /> kürzeste (Closest Join)</span></label>
      {/if}
      {#if arbeit.objekte.length}
        <label class="frow" style="padding:0"><span class="fl">Bisheriges</span><span class="fc"><input type="checkbox" bind:checked={ersetzen} /> Design ersetzen (sonst als Element hinzufügen)</span></label>
      {/if}
      <div class="note small">
        Die Zeichnung wird {ersetzen || !arbeit.objekte.length ? "das neue Design" : "als neues Element hinzugefügt"} – als Gruppe: Größe, Drehen und Lage gelten für alles zusammen. Rückgängig möglich.
      </div>
      {#if fehler}<div class="errbox">{fehler}</div>{/if}
    </div>
  {/if}
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
    <button class="btn btn-primary" disabled={!pfad || laeuft || (art === "bild" && !vorschau)} onclick={umwandeln}>{laeuft ? "Wandle um …" : art === "bild" ? "Umwandeln" : "Einlesen"}</button>
  {/snippet}
</Dialog>

<style>
  .bild-dlg {
    display: grid;
    grid-template-columns: minmax(0, 1fr) 340px;
    gap: 16px;
    align-items: start;
  }
  .bild-feld {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 300px;
    border: 1px solid var(--line);
    /* Karomuster = Stoff bleibt frei */
    background: repeating-conic-gradient(#e7e4dd 0% 25%, #f7f5f0 0% 50%) 0 0 / 14px 14px;
  }
  .bild-feld.leer {
    background: var(--panel);
  }
  .bild-feld canvas {
    display: block;
    max-width: 100%;
    max-height: 420px;
    image-rendering: pixelated;
  }
  .bild-laedt {
    position: absolute;
    right: 6px;
    top: 6px;
    padding: 2px 7px;
    background: rgb(0 0 0 / 0.6);
    color: #fff;
  }
  .bild-rechts {
    display: flex;
    flex-direction: column;
    gap: 9px;
  }
  .bild-farben {
    display: flex;
    flex-direction: column;
    max-height: 230px;
    overflow: auto;
    border: 1px solid var(--line);
  }
  .bild-farbe {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 8px;
    cursor: pointer;
    border-bottom: 1px solid var(--line);
  }
  .bild-farbe:last-child {
    border-bottom: 0;
  }
  .bild-farbe.aus {
    opacity: 0.55;
  }
  .bild-farbe .sw {
    width: 18px;
    height: 18px;
    flex: none;
    border: 1px solid rgb(0 0 0 / 0.2);
  }
</style>
