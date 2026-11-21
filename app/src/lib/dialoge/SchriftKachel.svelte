<script lang="ts" module>
  /** Probetext der Google-Vorschau: fest, damit an Google nur der Name der Schrift geht (nicht der Text des Nutzers) */
  export const GOOGLE_PROBE = "Stickerei";
  const geladeneCss = new Set<string>();

  /** Vorschau einer Google-Schrift, die noch nicht heruntergeladen ist: Web-Schrift nur mit den Zeichen des Probetexts */
  function googleCss(familie: string) {
    if (geladeneCss.has(familie)) return;
    geladeneCss.add(familie);
    const l = document.createElement("link");
    l.rel = "stylesheet";
    l.href = `https://fonts.googleapis.com/css2?family=${encodeURIComponent(familie).replace(/%20/g, "+")}&text=${encodeURIComponent(GOOGLE_PROBE)}&display=block`;
    document.head.appendChild(l);
  }
</script>

<script lang="ts">
  // Eine Kachel der Schriftauswahl: Vorschau (erst wenn sichtbar), Name, Unterzeile oder „stickbar ab … mm“, Stern.
  // Original-Alphabet und Computer-Schrift: Umrisse aus dem Kern mit dem Text des Nutzers; Google-Schrift vor dem
  // Herunterladen: Web-Schrift von Google mit festem Probetext.
  import type { SchriftVorschau } from "$kern/kern";
  import { listen } from "../listen.svelte";
  import { zahl } from "../zahl";
  import AlphabetBild from "./AlphabetBild.svelte";
  import Umrissbild from "./Umrissbild.svelte";

  type Props = {
    quelle: "alphabet" | "ttf" | "google";
    /** Alphabet, Name der Computer-Schrift bzw. Google-Familie */
    name: string;
    unter?: string;
    text: string;
    /** Google: schon heruntergeladen */
    geladen?: boolean;
    laedt?: boolean;
    aktiv?: boolean;
    /** Höhe des Texts (mm) – Warnung, wenn kleiner als „stickbar ab“ */
    hoehe?: number;
    favorit: string;
    waehlen: () => void;
  };
  let p: Props = $props();

  let sichtbar = $state(false);
  let v = $state.raw<SchriftVorschau | null>(null);
  let fehler = $state(false);
  const vomKern = $derived(p.quelle === "ttf" || (p.quelle === "google" && !!p.geladen));
  const fav = $derived(listen.istFavorit(p.favorit));

  function beobachten(el: HTMLElement) {
    const io = new IntersectionObserver(
      (e) => {
        if (e.some((x) => x.isIntersecting)) {
          sichtbar = true;
          io.disconnect();
        }
      },
      { rootMargin: "150px" },
    );
    io.observe(el);
    return { destroy: () => io.disconnect() };
  }

  $effect(() => {
    if (!sichtbar || !vomKern) return;
    v = null;
    fehler = false;
    const [n, t] = [p.name, p.text];
    listen
      .schriftVorschau(n, t)
      .then((x) => n === p.name && t === p.text && (v = x))
      .catch(() => (fehler = true));
  });

  $effect(() => {
    if (sichtbar && p.quelle === "google" && !p.geladen) googleCss(p.name);
  });

  const stick = $derived(v?.stickbarkeit ?? null);
  const zuKlein = $derived(!!stick && p.hoehe != null && p.hoehe < stick.stickbar_ab);

  function taste(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      p.waehlen();
    }
  }
</script>

<div
  class="atile kachel"
  class:on={p.aktiv}
  role="button"
  tabindex="0"
  aria-pressed={p.aktiv}
  use:beobachten
  onclick={p.waehlen}
  onkeydown={taste}
>
  <span class="aimg" style="background:var(--panel); border-style:solid; height:58px; padding:4px 6px; font-family:inherit; color:var(--color-text)">
    {#if !sichtbar}
      <span class="mut small">…</span>
    {:else if p.quelle === "alphabet"}
      <AlphabetBild name={p.name} text={p.text} />
    {:else if vomKern}
      {#if v}<Umrissbild linien={v.linien} begrenzung={v.begrenzung} />{:else}<span class="mut small">{fehler ? "–" : "…"}</span>{/if}
    {:else}
      <span class="gprobe" style="font-family:'{p.name}', var(--font-body)">{GOOGLE_PROBE}</span>
    {/if}
  </span>
  <b class="nm" title={p.name}>{p.name}</b>
  {#if stick}
    <span class="small" class:warn={zuKlein} title="eigene Messung: ab dieser Höhe sind die dünnen Striche mindestens 0,8 mm breit (wie bei den Original-Alphabeten)">
      stickbar ab {zahl(stick.stickbar_ab)} mm{zuKlein ? ` · jetzt ${zahl(p.hoehe ?? 0)} mm` : ""}
    </span>
  {:else}
    <span class="mut small zeile">{p.unter ?? ""}</span>
  {/if}
  {#if v?.fehlende_zeichen}<span class="small warn">fehlt: {v.fehlende_zeichen}</span>{/if}
  {#if p.quelle === "google" && !p.geladen}
    <span class="laden small">{p.laedt ? "wird geladen …" : "↓ laden und verwenden"}</span>
  {/if}
  <button
    class="stern"
    class:an={fav}
    aria-pressed={fav}
    title={fav ? "Aus den Favoriten nehmen" : "Zu den Favoriten"}
    onclick={(e) => {
      e.stopPropagation();
      listen.favoritUmschalten(p.favorit);
    }}
    onkeydown={(e) => e.stopPropagation()}
  >
    <svg viewBox="0 0 24 24" width="16" height="16" aria-hidden="true">
      <path
        d="M12 2.5l2.9 5.9 6.6 1-4.8 4.6 1.1 6.5L12 17.4l-5.8 3.1 1.1-6.5-4.8-4.6 6.6-1z"
        fill={fav ? "currentColor" : "none"}
        stroke="currentColor"
        stroke-width="1.6"
        stroke-linejoin="round"
      />
    </svg>
  </button>
</div>

<style>
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
    padding-right: 18px;
  }
  .zeile {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .warn {
    color: var(--warn);
  }
  .gprobe {
    font-size: 26px;
    line-height: 1.1;
    white-space: nowrap;
    overflow: hidden;
    max-width: 100%;
  }
  .laden {
    color: var(--color-accent);
  }
  .stern {
    position: absolute;
    right: 4px;
    bottom: 4px;
    display: grid;
    place-items: center;
    width: 24px;
    height: 24px;
    padding: 0;
    border: 0;
    background: none;
    color: color-mix(in srgb, var(--color-text) 40%, transparent);
    cursor: pointer;
  }
  .stern.an {
    color: #d4a017;
  }
  .stern:hover {
    color: #d4a017;
  }
</style>
