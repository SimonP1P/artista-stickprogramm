<script lang="ts">
  // Schrift wählen (Wunsch Nutzer 21.11.2026): Original-Alphabete (50, von Hand digitalisiert), Computer-Schriften
  // (installiert + heruntergeladen) und Google Fonts (1 833, freie Lizenzen) – mit Suche, Stil-Filtern, Favoriten,
  // Vorschau des eigenen Texts und „stickbar ab … mm“. Listen, Vorschauen, Download und Messung kommen aus dem Kern.
  import Dialog from "../Dialog.svelte";
  import { listen } from "../listen.svelte";
  import { ganz, zahl } from "../zahl";
  import SchriftKachel from "./SchriftKachel.svelte";
  import type { SchriftWahl } from "../arbeit.svelte";
  import { untrack } from "svelte";

  let {
    start,
    aktuell,
    text,
    hoehe,
    waehlen,
    schliessen,
  }: {
    /** Reiter beim Öffnen (sonst nach der aktuellen Schrift) */
    start?: "alphabete" | "computer" | "google" | "favoriten";
    aktuell: SchriftWahl;
    /** Text des Objekts (Vorschau) */
    text?: string;
    /** Höhe des Objekts in mm (Warnung „stickbar ab“) */
    hoehe?: number;
    waehlen: (w: SchriftWahl) => void;
    schliessen: () => void;
  } = $props();

  type Reiter = "alphabete" | "computer" | "google" | "favoriten";
  type Eintrag = {
    id: string;
    quelle: "alphabet" | "ttf" | "google";
    name: string;
    unter: string;
    geladen?: boolean;
    suchtext: string;
    stile: string[];
  };

  const SEITE = 48;

  $effect(() => {
    listen.laden();
    listen.schriftenLaden().catch(() => {});
    listen.googleLaden().catch(() => {});
  });

  // Startreiter: Computer-Schrift → „Auf dem Computer“ (dort stehen auch geladene Google Fonts), sonst Favoriten, wenn
  // es welche gibt, sonst die Stick-Alphabete
  let reiter = $state<Reiter>(
    untrack(() => start ?? (aktuell.schrift ? "computer" : listen.favoriten.length ? "favoriten" : "alphabete")),
  );
  let suche = $state("");
  let stil = $state<string | null>(null);
  let anzahl = $state(SEITE);
  let laedt = $state<string | null>(null);
  let fehler = $state<string | null>(null);

  // Vorschautext: Text des Objekts (erste Zeile, ohne Farbwechsel ^), verzögert übernommen
  let probe = $state(untrack(() => (text ?? "").split("\n")[0].replaceAll("^", "").trim().slice(0, 24) || "Anna"));
  let probeAktiv = $state("");
  $effect(() => {
    const t = probe.trim() || "Anna";
    const z = setTimeout(() => (probeAktiv = t), untrack(() => probeAktiv) ? 300 : 0);
    return () => clearTimeout(z);
  });

  $effect(() => {
    void [reiter, suche, stil];
    anzahl = SEITE;
  });

  const alphabete = $derived(
    listen.alphabete.map(
      (a): Eintrag => ({
        id: `a:${a.name}`,
        quelle: "alphabet",
        name: a.name,
        unter: a.min_hoehe != null ? `empfohlen ${zahl(a.min_hoehe)}–${a.max_hoehe != null ? zahl(a.max_hoehe) : "?"} mm` : "keine Angabe",
        suchtext: a.name.toLowerCase(),
        stile: [],
      }),
    ),
  );
  const computer = $derived(
    listen.schriften.map(
      (s): Eintrag => ({
        id: `s:${s.name}`,
        quelle: "ttf",
        name: s.name,
        unter: s.heruntergeladen ? "Google Fonts, heruntergeladen" : s.familie,
        suchtext: `${s.name} ${s.familie}`.toLowerCase(),
        stile: [],
      }),
    ),
  );
  const google = $derived(
    (listen.google?.schriften ?? []).map(
      (g): Eintrag => ({
        id: `g:${g.familie}`,
        quelle: "google",
        name: g.familie,
        geladen: g.geladen,
        unter: [g.kategorie, ...g.stile.slice(0, 2)].join(" · "),
        suchtext: `${g.familie} ${g.kategorie} ${g.stile.join(" ")}`.toLowerCase(),
        stile: g.stile,
      }),
    ),
  );
  const favoriten = $derived.by(() => {
    const alle = new Map([...alphabete, ...computer, ...google].map((x) => [x.id, x]));
    return listen.favoriten.map((id) => alle.get(id)).filter((x): x is Eintrag => !!x);
  });

  const liste = $derived(
    { alphabete, computer, google, favoriten }[reiter].filter(
      (x) =>
        (!suche.trim() || suche.trim().toLowerCase().split(/\s+/).every((w) => x.suchtext.includes(w))) &&
        (!stil || reiter !== "google" || x.stile.includes(stil)),
    ),
  );
  const sichtbar = $derived(liste.slice(0, anzahl));

  function istAktiv(x: Eintrag): boolean {
    if (x.quelle === "alphabet") return x.name === aktuell.alphabet;
    return !!aktuell.schrift && x.name.toLowerCase() === aktuell.schrift.toLowerCase();
  }

  async function nehmen(x: Eintrag) {
    fehler = null;
    if (x.quelle === "alphabet") return waehlen({ alphabet: x.name });
    if (x.quelle === "ttf") return waehlen({ schrift: x.name });
    if (laedt) return;
    laedt = x.name;
    try {
      const name = x.geladen ? x.name : await listen.googleHerunterladen(x.name);
      waehlen({ schrift: name });
    } catch (e) {
      fehler = (e as { meldung?: string })?.meldung ?? String(e);
    } finally {
      laedt = null;
    }
  }

  const REITER: [Reiter, string, () => number][] = [
    ["alphabete", "Stick-Alphabete", () => alphabete.length],
    ["computer", "Auf dem Computer", () => computer.length],
    ["google", "Google Fonts", () => google.length],
    ["favoriten", "★ Favoriten", () => favoriten.length],
  ];
</script>

<Dialog titel="Schrift wählen" breite={1000} {schliessen}>
  {#snippet kopf()}
    <h2 class="hd">Schrift wählen</h2>
    <span style="flex:1"></span>
    <input class="input sm" placeholder="Suchen … (Name, Stil)" aria-label="Schrift suchen" bind:value={suche} style="width:230px" />
  {/snippet}

  <div style="display:flex; gap:10px; align-items:center; flex-wrap:wrap; margin-bottom:10px">
    <div class="seg seg-sm" role="radiogroup" aria-label="Quelle der Schriften" style="display:flex">
      {#each REITER as [r, l, n] (r)}
        <label class="seg-opt"><input type="radio" name="sreiter" checked={reiter === r} onchange={() => (reiter = r)} />{l}&nbsp;<span class="tab" style="opacity:.75">{ganz(n())}</span></label>
      {/each}
    </div>
    <span style="flex:1"></span>
    <label class="small mut" for="sprobe">Vorschautext</label>
    <input id="sprobe" class="input sm" bind:value={probe} style="width:180px" />
  </div>

  {#if reiter === "google" && listen.google}
    <div class="stile" role="group" aria-label="Stil">
      <button class="stil" class:on={!stil} aria-pressed={!stil} onclick={() => (stil = null)}>Alle</button>
      {#each listen.google.stile as s (s)}
        <button class="stil" class:on={stil === s} aria-pressed={stil === s} onclick={() => (stil = stil === s ? null : s)}>{s}</button>
      {/each}
    </div>
  {/if}

  <p class="mut small" style="margin:0 0 10px">
    {#if reiter === "alphabete"}
      Die 50 Alphabete der BERNINA-Software – von Hand digitalisiert, sticken am saubersten. Darunter die empfohlene Höhe (Handbuch).
    {:else if reiter === "computer"}
      Schriften auf diesem Rechner. Sie werden automatisch in Stiche umgewandelt. „Stickbar ab“ = ab dieser Höhe sind die dünnen Striche breit genug (eigene Messung).
    {:else if reiter === "google"}
      {ganz(google.length)} freie Schriften (auch für Gesticktes zum Verkaufen). Vorschau mit Beispielwort aus dem Internet – an Google geht nur der Name der Schrift. Ein Klick lädt die Schrift einmal herunter; danach geht sie auch ohne Internet.
    {:else}
      Mit dem Stern markierte Schriften (nur auf diesem Rechner gespeichert).
    {/if}
  </p>

  {#if fehler}<div class="errbox" style="margin-bottom:10px">{fehler}</div>{/if}

  {#if reiter === "computer" && !listen.schriften.length}
    <p class="mut">Schriften werden gelesen … (das erste Mal kann es einige Sekunden dauern)</p>
  {:else if reiter === "google" && !listen.google}
    <p class="mut">Katalog wird geladen …</p>
  {:else if !liste.length}
    <p class="mut">{reiter === "favoriten" && !suche ? "Noch keine Favoriten – Stern an einer Schrift anklicken." : "Keine Schrift gefunden."}</p>
  {:else}
    <div style="display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); gap:8px">
      {#each sichtbar as x (x.id)}
        <SchriftKachel
          quelle={x.quelle}
          name={x.name}
          unter={x.unter}
          text={probeAktiv || "Anna"}
          geladen={x.geladen}
          laedt={laedt === x.name}
          aktiv={istAktiv(x)}
          {hoehe}
          favorit={x.id}
          waehlen={() => nehmen(x)}
        />
      {/each}
    </div>
    {#if liste.length > anzahl}
      <div style="display:flex; justify-content:center; margin-top:10px">
        <button class="btn btn-secondary" onclick={() => (anzahl += SEITE * 2)}>Mehr anzeigen ({ganz(liste.length - anzahl)} weitere)</button>
      </div>
    {/if}
  {/if}

  {#snippet fuss()}
    <span class="mut small" style="margin-right:auto">
      {#if reiter === "computer"}<button class="btn btn-ghost" style="font-size:12.5px" onclick={() => listen.schriftenLaden(true)}>Neu einlesen</button>{/if}
      {#if listen.google && reiter === "google"}Katalog: Google Fonts, Stand {listen.google.stand}{/if}
    </span>
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
  .stil:hover {
    border-color: var(--color-accent);
  }
  .stil.on {
    background: var(--color-accent);
    border-color: var(--color-accent);
    color: var(--color-bg);
  }
</style>
