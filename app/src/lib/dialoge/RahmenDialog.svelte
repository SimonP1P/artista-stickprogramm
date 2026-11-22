<script lang="ts" module>
  /** Wahl im Dialog: Rahmen aus dem Katalog oder Rahmen des Originals (Diamond) */
  export type RahmenWahl = { vorlage: string } | { rahmen: "raute" | "zier" | "keine" };
  const geladeneCss = new Set<string>();
  /** Vorschau der vorgeschlagenen Schrift: Web-Schrift von Google nur mit den festen Probebuchstaben (an Google geht nur
   *  der Name der Schrift, nicht die Initialen des Anwenders) – ohne Internet steht eine Ersatzschrift da */
  function googleCss(familie: string) {
    if (geladeneCss.has(familie)) return;
    geladeneCss.add(familie);
    const l = document.createElement("link");
    l.rel = "stylesheet";
    l.href = `https://fonts.googleapis.com/css2?family=${encodeURIComponent(familie).replace(/%20/g, "+")}&text=ABC&display=block`;
    document.head.appendChild(l);
  }
</script>

<script lang="ts">
  // Rahmen eines Monogramms wählen (Wunsch Nutzer 21.11.2026, „ähnlich wie die Schriften“): die Rahmen des Originals
  // (Diamond2/3 mit Raute oder Zierwinkel, Handbuch Kap. 21) und der Katalog mit 47 Rahmen in 8 Kategorien (eigene
  // Erweiterung, Kern `monogramm_rahmen`). Vorschau: Zeichnung der Vorlage mit Probebuchstaben in der vorgeschlagenen
  // Schrift; gestickt setzt der Kern die Initialen in den Rahmen.
  import Dialog from "../Dialog.svelte";
  import { listen } from "../listen.svelte";
  import { ganz, zahl } from "../zahl";
  import AlphabetBild from "./AlphabetBild.svelte";
  import type { MonogrammRahmenInfo } from "$kern/kern";

  let {
    aktuell,
    waehlen,
    schliessen,
  }: {
    /** gewählter Rahmen: Name aus dem Katalog oder Rahmen des Originals */
    aktuell: { vorlage?: string; rahmen?: string };
    waehlen: (w: RahmenWahl) => void;
    schliessen: () => void;
  } = $props();

  const ORIGINAL = "Original BERNINA";
  const ORIGINALE: { rahmen: "raute" | "zier" | "keine"; name: string; text: string; unter: string }[] = [
    { rahmen: "raute", name: "Raute", text: "Ab]", unter: "Diamond · 2–3 Initialen" },
    { rahmen: "zier", name: "Zierwinkel", text: "Ab[", unter: "Diamond · 2–3 Initialen" },
    { rahmen: "keine", name: "ohne Rahmen", text: "Ab", unter: "Diamond · 2–3 Initialen" },
  ];

  $effect(() => {
    listen.monogrammeLaden().catch(() => (fehler = "Katalog der Rahmen nicht lesbar"));
  });

  let suche = $state("");
  let kategorie = $state<string | null>(null);
  let fehler = $state<string | null>(null);

  const katalog = $derived(listen.monogramme);
  const gesucht = $derived.by(() => {
    const w = suche.trim().toLowerCase().split(/\s+/).filter(Boolean);
    return (katalog?.rahmen ?? []).filter((r) =>
      w.every((v) => `${r.name} ${r.kategorie} ${r.schrift} ${r.beschreibung}`.toLowerCase().includes(v)),
    );
  });
  const originaleSichtbar = $derived(
    (!kategorie || kategorie === ORIGINAL) &&
      suche
        .trim()
        .toLowerCase()
        .split(/\s+/)
        .filter(Boolean)
        .every((v) => `original bernina diamond raute zierwinkel`.includes(v)),
  );
  const filter = $derived([
    { name: ORIGINAL, anzahl: originaleSichtbar || kategorie === ORIGINAL ? ORIGINALE.length : 0 },
    ...(katalog?.kategorien ?? []).map((k) => ({ name: k, anzahl: gesucht.filter((r) => r.kategorie === k).length })),
  ]);
  /** Abschnitte: je Kategorie die gefundenen Rahmen */
  const abschnitte = $derived(
    (katalog?.kategorien ?? [])
      .filter((k) => !kategorie || kategorie === k)
      .map((k) => ({ name: k, rahmen: gesucht.filter((r) => r.kategorie === k) }))
      .filter((a) => a.rahmen.length),
  );

  const svgBild = (svg: string) => `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svg)}`;

  function beobachten(el: HTMLElement, familie: string) {
    const io = new IntersectionObserver(
      (e) => {
        if (e.some((x) => x.isIntersecting)) {
          googleCss(familie);
          io.disconnect();
        }
      },
      { rootMargin: "150px" },
    );
    io.observe(el);
    return { destroy: () => io.disconnect() };
  }

  /** Probebuchstaben mit den Höhen der Vorlage: Schriftgröße so, dass die Versalhöhe (≈ 0,7 em) passt */
  function probe(r: MonogrammRahmenInfo) {
    const z = r.buchstaben === 3 ? ["A", "B", "C"] : ["A", "B"];
    return z.map((c, k) => ({ c, em: ((r.hoehen[k] ?? r.hoehen[0]) / 90 / 0.7) * 100 }));
  }

  function taste(e: KeyboardEvent, w: RahmenWahl) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      waehlen(w);
    }
  }
</script>

<Dialog titel="Rahmen wählen" breite={1040} {schliessen}>
  {#snippet kopf()}
    <h2 class="hd">Rahmen wählen</h2>
    <span style="flex:1"></span>
    <input class="input sm" placeholder="Suchen … (Name, Schrift)" aria-label="Rahmen suchen" bind:value={suche} style="width:230px" />
  {/snippet}

  <div class="stile" role="group" aria-label="Kategorie">
    <button class="stil" class:on={!kategorie} aria-pressed={!kategorie} onclick={() => (kategorie = null)}>Alle</button>
    {#each filter as f (f.name)}
      <button class="stil" class:on={kategorie === f.name} aria-pressed={kategorie === f.name} disabled={!f.anzahl && kategorie !== f.name} onclick={() => (kategorie = kategorie === f.name ? null : f.name)}
        >{f.name}&nbsp;<span class="tab" style="opacity:.7">{f.anzahl}</span></button
      >
    {/each}
  </div>

  <p class="mut small" style="margin:0 0 10px">
    Rahmen aus dem Katalog: Die Initialen stehen in der vorgeschlagenen Google-Schrift (beim ersten Mal wird sie heruntergeladen, danach geht es ohne Internet) und werden so groß, wie der Rahmen Platz lässt. Schrift, Größe und Farben kannst du danach ändern. Die Probebuchstaben hier zeigen nur den Stil.
  </p>

  {#if fehler}<div class="errbox" style="margin-bottom:10px">{fehler}</div>{/if}

  {#if originaleSichtbar}
    <h3 class="kat">{ORIGINAL} <span class="mut small" style="font-weight:400">– Monogramm-Alphabete Diamond2/3 wie in der BERNINA-Software</span></h3>
    <div class="raster">
      {#each ORIGINALE as x (x.rahmen)}
        {@const an = !aktuell.vorlage && (aktuell.rahmen ?? "keine") === x.rahmen}
        <div class="atile kachel" class:on={an} role="button" tabindex="0" aria-pressed={an} onclick={() => waehlen({ rahmen: x.rahmen })} onkeydown={(e) => taste(e, { rahmen: x.rahmen })}>
          <span class="bild"><span style="width:70%; height:70%; display:flex"><AlphabetBild name="Diamond2" text={x.text} /></span></span>
          <b class="nm">{x.name}</b>
          <span class="mut small zeile">{x.unter}</span>
        </div>
      {/each}
    </div>
  {/if}

  {#if !katalog}
    <p class="mut">Katalog wird geladen …</p>
  {:else}
    {#each abschnitte as a (a.name)}
      <h3 class="kat">{a.name}</h3>
      <div class="raster">
        {#each a.rahmen as r (r.name)}
          {@const an = aktuell.vorlage?.toLowerCase() === r.name.toLowerCase()}
          <div
            class="atile kachel"
            class:on={an}
            role="button"
            tabindex="0"
            aria-pressed={an}
            title={r.beschreibung}
            use:beobachten={r.schrift}
            onclick={() => waehlen({ vorlage: r.name })}
            onkeydown={(e) => taste(e, { vorlage: r.name })}
          >
            <span class="bild">
              <img src={svgBild(r.svg)} alt="" draggable="false" />
              <span class="probe" style="font-family:'{r.schrift}', Georgia, serif; color:{r.farben[0] ?? '#333'}">
                {#each probe(r) as z (z.c)}<span style="font-size:{z.em}cqw">{z.c}</span>{/each}
              </span>
            </span>
            <b class="nm">{r.name}</b>
            <span class="mut small zeile">{r.schrift} · {r.buchstaben} Initialen · {zahl(Math.max(...r.hoehen))} mm</span>
          </div>
        {/each}
      </div>
    {:else}
      {#if !originaleSichtbar}<p class="mut">Kein Rahmen gefunden.</p>{/if}
    {/each}
  {/if}

  {#snippet fuss()}
    <span class="mut small" style="margin-right:auto">{katalog ? `${ganz(katalog.rahmen.length)} Rahmen im Katalog · ${ORIGINALE.length} des Originals` : ""}</span>
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
  {/snippet}
</Dialog>

<style>
  .stile {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-bottom: 10px;
  }
  .stil {
    font: inherit;
    font-size: 12.5px;
    padding: 3px 10px;
    border: 1px solid var(--line);
    background: var(--color-bg);
    color: var(--color-text);
    cursor: pointer;
  }
  .stil:hover:not(:disabled) {
    border-color: var(--color-accent);
  }
  .stil:disabled {
    opacity: 0.45;
    cursor: default;
  }
  .stil.on {
    background: var(--color-accent);
    border-color: var(--color-accent);
    color: var(--color-bg);
  }
  .kachel {
    position: relative;
    min-width: 0;
  }
  .nm {
    font-weight: 500;
    font-size: 13px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .zeile {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .kat {
    font-size: 14px;
    font-weight: 600;
    margin: 14px 0 8px;
  }
  .raster {
    display: grid;
    grid-template-columns: repeat(6, minmax(0, 1fr));
    gap: 8px;
  }
  .bild {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    aspect-ratio: 1;
    width: 100%;
    background: #fff;
    border: 1px solid var(--line);
    container-type: inline-size;
    overflow: hidden;
  }
  .bild img {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }
  .probe {
    position: relative;
    display: flex;
    align-items: center;
    gap: 2cqw;
    line-height: 1;
    pointer-events: none;
  }
</style>
