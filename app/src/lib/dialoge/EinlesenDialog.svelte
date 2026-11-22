<script lang="ts">
  // Bild umwandeln wie der „Auto Digitizer“ des Originals (`bild_vorschau` + `bild_umwandeln`; Schritte nach Handbuch
  // Kap. 12/13, Verfahren eigene Festlegung), **Zauberstab** (Magic Wand: Flächen im vorbereiteten Bild anklicken,
  // `zauberstab`), **PhotoSnap** (Foto als Zickzackreihen, `photosnap`) und SVG einlesen (`svg_lesen`, Stichart nach
  // Breite wie EasyStitch).
  // Bild: links das vereinfachte Bild, rechts Einstellungen und die Farbliste – je Farbe sticken oder weglassen
  // („Omitted Colors“). Weggelassene Farben blendet nur die Anzeige aus (Leinwand), gerechnet wird im Kern.
  import { untrack } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { kern, type BildVorschau, type DesignJson, type ZauberStich } from "$kern/kern";
  import { arbeit, alsFehler, leeresDesign } from "../arbeit.svelte";
  import { dateiname } from "../dateien.svelte";
  import Dialog from "../Dialog.svelte";
  import { garnText } from "../listen.svelte";
  import { lesen, zahl } from "../zahl";

  let {
    art,
    schliessen,
    startPfad,
    startModus,
  }: { art: "bild" | "svg"; schliessen: () => void; startPfad?: string; startModus?: "auto" | "zauber" | "photo" } = $props();

  /** Weg beim Bild: alles umwandeln (Auto Digitizer), Flächen anklicken (Zauberstab) oder PhotoSnap */
  // svelte-ignore state_referenced_locally
  let modus = $state<"auto" | "zauber" | "photo">(startModus ?? "auto");
  const MODI = [
    ["auto", "Alles umwandeln"],
    ["zauber", "Zauberstab"],
    ["photo", "PhotoSnap"],
  ] as const;

  // ---- Zauberstab: Klicks (Lage 0 … 1 im Vorschaubild) mit Stichart; jede getroffene Fläche wird ein Objekt ----
  const ZAUBER_STICH: [ZauberStich, string][] = [
    ["step", "Fläche – Step"],
    ["satin", "Fläche – Satin"],
    ["auto", "Fläche – Auto (bis 8 mm Satin)"],
    ["lauf", "Umriss – Laufstich"],
    ["dreifach", "Umriss – Dreifachstich"],
    ["satinlinie", "Umriss – Satinlinie"],
  ];
  let zauberStich = $state<ZauberStich>("step");
  let klicks = $state<{ x: number; y: number; stichart: ZauberStich; farbe: string }[]>([]);

  // ---- PhotoSnap ----
  let farbig = $state(false);
  let hintergrund = $state<"hell" | "dunkel">("hell");
  let aufloesung = $state<"grob" | "mittel" | "fein">("mittel");
  let photoWinkel = $state("0");

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
      farben: modus === "photo" ? 16 : Math.max(2, Math.min(24, Math.round(farben))),
      vereinfachen: modus === "photo" ? 0 : vereinfachen,
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
  /** Bildpunkte des vorbereiteten Bildes (ohne Ausblenden) – für die Farbe am Klickpunkt und das Markieren */
  let basis: ImageData | null = null;
  $effect(() => {
    const v = vorschau;
    // weggelassene Farben nur beim Umwandeln ausblenden; Zauberstab: angeklickte Flächen markieren
    const aus = new Set(modus === "auto" ? weg : []);
    const markiert = modus === "zauber" ? klicks.map((k) => ({ ...k })) : [];
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
      basis = g.getImageData(0, 0, c.width, c.height);
      if (!aus.size && !markiert.length) return;
      const d = g.getImageData(0, 0, c.width, c.height);
      const hex = (i: number) => "#" + [0, 1, 2].map((k) => d.data[i + k].toString(16).padStart(2, "0")).join("");
      if (aus.size) for (let i = 0; i < d.data.length; i += 4) if (d.data[i + 3] && aus.has(hex(i))) d.data[i + 3] = 0;
      for (const k of markiert) {
        const m = flaeche(basis, Math.floor(k.x * c.width), Math.floor(k.y * c.height));
        // schraffiert markieren: jede zweite Diagonale abdunkeln
        for (let i = 0; i < m.length; i++)
          if (m[i] && ((i % c.width) + Math.floor(i / c.width)) % 6 < 3) {
            d.data[i * 4] = 255;
            d.data[i * 4 + 1] = 255;
            d.data[i * 4 + 2] = 255;
          }
      }
      g.putImageData(d, 0, 0);
    };
    img.src = v.bild;
  });

  /** zusammenhängende Fläche gleicher Farbe (4er-Nachbarschaft) wie der Zauberstab im Kern */
  function flaeche(b: ImageData, x0: number, y0: number): Uint8Array {
    const { width: w, height: h, data } = b;
    const m = new Uint8Array(w * h);
    if (x0 < 0 || y0 < 0 || x0 >= w || y0 >= h) return m;
    const s0 = (y0 * w + x0) * 4;
    if (!data[s0 + 3]) return m;
    const gleich = (i: number) => data[i * 4 + 3] > 0 && data[i * 4] === data[s0] && data[i * 4 + 1] === data[s0 + 1] && data[i * 4 + 2] === data[s0 + 2];
    const stapel = [y0 * w + x0];
    m[y0 * w + x0] = 1;
    while (stapel.length) {
      const i = stapel.pop()!;
      const x = i % w;
      for (const n of [x > 0 ? i - 1 : -1, x < w - 1 ? i + 1 : -1, i - w, i + w])
        if (n >= 0 && n < w * h && !m[n] && gleich(n)) {
          m[n] = 1;
          stapel.push(n);
        }
    }
    return m;
  }

  function klick(ev: MouseEvent) {
    if (modus !== "zauber" || !leinwand || !basis) return;
    const r = leinwand.getBoundingClientRect();
    const x = (ev.clientX - r.left) / r.width;
    const y = (ev.clientY - r.top) / r.height;
    const px = Math.floor(x * leinwand.width);
    const py = Math.floor(y * leinwand.height);
    const i = (py * leinwand.width + px) * 4;
    if (!basis.data[i + 3]) return;
    const farbe = "#" + [0, 1, 2].map((k) => basis!.data[i + k].toString(16).padStart(2, "0")).join("");
    // dieselbe Fläche noch einmal: Stichart ersetzen statt doppelt
    const m = flaeche(basis, px, py);
    const schon = klicks.findIndex((k) => m[Math.floor(k.y * leinwand!.height) * leinwand!.width + Math.floor(k.x * leinwand!.width)]);
    if (schon >= 0) klicks[schon].stichart = zauberStich;
    else klicks.push({ x, y, stichart: zauberStich, farbe });
  }

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
      klicks = [];
    }
  }

  function rahmenStoff() {
    const d = arbeit.design;
    return { name: d.name ?? "Design", rahmen: d.rahmen, eigener_rahmen: d.eigener_rahmen ? { ...d.eigener_rahmen } : undefined, stoff: d.stoff, garnkarte: d.garnkarte };
  }

  async function umwandeln() {
    if (!pfad) return;
    laeuft = true;
    fehler = null;
    hinweise = [];
    const rahmen = arbeit.design.rahmen;
    try {
      let design: DesignJson;
      if (art === "bild" && modus === "zauber") {
        const e = await kern("zauberstab", { bild: { ...eingabe(), rahmen }, klicks: klicks.map(({ x, y, stichart }) => ({ x, y, stichart })) });
        design = e.design;
        hinweise = e.hinweise;
      } else if (art === "bild" && modus === "photo") {
        const e = await kern("photosnap", {
          pfad,
          breite: lesen(breite) ?? undefined,
          farbig,
          garnkarte: mitKarte ? karte : undefined,
          aufloesung,
          hintergrund,
          winkel: lesen(photoWinkel) ?? 0,
          rahmen,
        });
        design = e.design;
      } else if (art === "bild") {
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
      // PhotoSnap ist ein Objekt – keine Gruppe nötig
      const r = await kern("design_einfuegen", {
        design: ziel as DesignJson,
        einfuegen: design,
        gruppe: modus === "photo" && art === "bild" ? undefined : { name: pfad ? dateiname(pfad).replace(/\.[^.]+$/, "") : art, herkunft: art },
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

<Dialog titel={art === "svg" ? "SVG einlesen" : modus === "zauber" ? "Zauberstab" : modus === "photo" ? "PhotoSnap" : "Bild umwandeln"} breite={art === "bild" ? 880 : 480} {schliessen}>
  {#if art === "bild"}
    <div class="bild-dlg">
      <!-- links: vereinfachtes Bild -->
      <div class="bild-links">
        <div class="bild-feld" class:leer={!vorschau}>
          {#if vorschau}
            <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
            <canvas
              bind:this={leinwand}
              class:zauber={modus === "zauber"}
              class:grau={modus === "photo" && !farbig}
              aria-label="Vorschau des vereinfachten Bildes"
              onclick={klick}
            ></canvas>
          {:else if pfad}
            <span class="mut small">{laedt ? "Bild wird vorbereitet …" : ""}</span>
          {:else}
            <button class="btn btn-secondary" onclick={dateiWaehlen}>Bild wählen …</button>
          {/if}
          {#if laedt && vorschau}<span class="bild-laedt small">rechnet …</span>{/if}
        </div>
        <p class="mut small" style="margin:6px 0 0">
          {#if modus === "zauber"}
            Fläche anklicken – jede angeklickte Fläche wird ein Objekt mit der gewählten Stichart, in der Reihenfolge der
            Klicks (wie der Zauberstab im Original). Schraffiert = gewählt; noch einmal klicken ändert die Stichart.
          {:else if modus === "photo"}
            PhotoSnap stickt das Bild als Reihen von Zickzackstichen: dunkle Stellen dicht, helle locker – wie ein
            Zeilendrucker. {farbig ? "Vier Farblagen (Cyan, Magenta, Gelb, Schwarz)." : "Eine Farbe (Graustufen)."}
          {:else}
            So wird das Bild gestickt: jede Fläche eine Farbe. Kariert = Stoff bleibt frei (Farbe weggelassen).
          {/if}
          {#if vorschau && hoeheMm}Größe {zahl(lesen(breite) ?? 0, 1)} × {zahl(hoeheMm, 1)} mm.{/if}
        </p>
      </div>
      <!-- rechts: Einstellungen -->
      <div class="bild-rechts">
        <div class="seg seg-sm" role="radiogroup" aria-label="Weg" style="display:flex">
          {#each MODI as [w, l] (w)}
            <label class="seg-opt" style="flex:1; justify-content:center"><input type="radio" name="bmodus" checked={modus === w} onchange={() => (modus = w)} />{l}</label>
          {/each}
        </div>
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
        {#if modus !== "photo"}
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
        {/if}
        {#if modus === "photo"}
          <div class="frow" style="padding:0">
            <span class="fl">Farbe</span>
            <div class="fc">
              <div class="seg seg-sm" role="radiogroup" aria-label="Farbe" style="display:flex; width:100%">
                <label class="seg-opt" style="flex:1; justify-content:center"><input type="radio" name="pfarbe" checked={!farbig} onchange={() => (farbig = false)} />einfarbig</label>
                <label class="seg-opt" style="flex:1; justify-content:center"><input type="radio" name="pfarbe" checked={farbig} onchange={() => (farbig = true)} />4 Farblagen</label>
              </div>
            </div>
          </div>
          <div class="frow" style="padding:0">
            <span class="fl" title="Heller Stoff: dunkle Stellen bekommen viel Faden; dunkler Stoff: helle Stellen">Stoff</span>
            <div class="fc">
              <select class="input sm" bind:value={hintergrund}>
                <option value="hell">hell</option>
                <option value="dunkel">dunkel</option>
              </select>
            </div>
          </div>
          <div class="frow" style="padding:0">
            <span class="fl" title="Breite der Stichreihen: grob 3 mm, mittel 2 mm, fein 1 mm">Auflösung</span>
            <div class="fc">
              <select class="input sm" bind:value={aufloesung}>
                <option value="grob">grob (3 mm)</option>
                <option value="mittel">mittel (2 mm)</option>
                <option value="fein">fein (1 mm)</option>
              </select>
            </div>
          </div>
          <div class="frow" style="padding:0">
            <span class="fl">Winkel</span>
            <div class="fc"><input class="input sm num" inputmode="decimal" bind:value={photoWinkel} style="width:80px" /><span class="unit">°</span></div>
          </div>
        {/if}
        <label class="frow" style="padding:0" title="Jede Farbe wird das nächste Garn der Garnkarte – gleiche Garne werden eine Farbe">
          <span class="fl">Garnfarben</span>
          <span class="fc"><input type="checkbox" bind:checked={garnfarben} disabled={karte === "keine"} /> aus der Karte „{karte}“</span>
        </label>
        {#if garnfarben && karte === "Default"}
          <p class="small mut" style="margin:-4px 0 0">„Default“ hat nur 19 Grundfarben – für Fotos besser eine echte Garnkarte (Isacord, Madeira …) im Schritt „Prüfen &amp; Garn“ wählen.</p>
        {/if}
        {#if modus === "zauber"}
          <div class="frow" style="padding:0">
            <span class="fl">Stichart</span>
            <div class="fc">
              <select class="input sm" bind:value={zauberStich} title="Gilt für den nächsten Klick (wie die Stichart vor dem Zauberstab im Original)">
                {#each ZAUBER_STICH as [w, l] (w)}<option value={w}>{l}</option>{/each}
              </select>
            </div>
          </div>
          <div class="mh" style="padding:6px 0 2px">Angeklickt – Reihenfolge = Stickreihenfolge</div>
          <div class="bild-farben">
            {#each klicks as k, n (n)}
              <div class="bild-farbe" style="cursor:default">
                <span class="small tab" style="width:16px">{n + 1}</span>
                <span class="sw" style="background:{k.farbe}"></span>
                <select class="input sm" style="flex:1; min-width:0" bind:value={k.stichart}>
                  {#each ZAUBER_STICH as [w, l] (w)}<option value={w}>{l}</option>{/each}
                </select>
                <button class="btn btn-ghost btn-sm" title="Entfernen" aria-label="Entfernen" onclick={() => klicks.splice(n, 1)}>×</button>
              </div>
            {:else}
              <p class="small mut" style="margin:6px 8px">Noch keine Fläche – links ins Bild klicken.</p>
            {/each}
          </div>
        {/if}
        {#if experte && modus !== "photo"}
          {#if modus === "auto"}
            <div class="frow" style="padding:0">
              <span class="fl">Stichart <span class="xt">X</span></span>
              <div class="fc">
                <select class="input sm" bind:value={fuellung}>
                  <option value="step">Step – sauber bei Fotos</option>
                  <option value="auto">Auto – schmale Flächen Satin (Logos)</option>
                </select>
              </div>
            </div>
          {/if}
          <div class="frow" style="padding:0">
            <span class="fl">Kleinste Fläche <span class="xt">X</span></span>
            <div class="fc"><input class="input sm num" inputmode="decimal" bind:value={minFlaeche} style="width:80px" /><span class="unit">mm²</span></div>
          </div>
          <div class="frow" style="padding:0">
            <span class="fl">Glättung <span class="xt">X</span></span>
            <div class="fc"><input class="input sm num" inputmode="decimal" bind:value={glaettung} style="width:80px" /><span class="unit">mm</span></div>
          </div>
        {/if}

        {#if vorschau && modus === "auto"}
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
      {#if modus === "photo"}
        PhotoSnap wird {ersetzen || !arbeit.objekte.length ? "das neue Design" : "als neues Objekt hinzugefügt"}; Auflösung, Stoff, Winkel und Farblagen lassen sich danach im Inspektor ändern. Rückgängig möglich.
      {:else}
        Das Bild wird {ersetzen || !arbeit.objekte.length ? "das neue Design" : "als neues Element hinzugefügt"} – als Gruppe: Größe, Drehen und Lage gelten für alles zusammen. Zugausgleich 0,4 mm und Unterlage wie vom Handbuch empfohlen; Reihenfolge nach kürzesten Wegen (Closest Join wie der Auto Digitizer). Rückgängig möglich.
      {/if}
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
    <button class="btn btn-primary" disabled={!pfad || laeuft || (art === "bild" && !vorschau) || (art === "bild" && modus === "zauber" && !klicks.length)} onclick={umwandeln}
      >{laeuft ? "Wandle um …" : art === "svg" ? "Einlesen" : modus === "zauber" ? `Übernehmen (${klicks.length})` : modus === "photo" ? "PhotoSnap anlegen" : "Umwandeln"}</button
    >
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
  .bild-feld canvas.zauber {
    cursor: crosshair;
  }
  .bild-feld canvas.grau {
    filter: grayscale(1);
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
