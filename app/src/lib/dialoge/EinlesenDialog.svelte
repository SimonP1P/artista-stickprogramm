<script lang="ts">
  // Bild umwandeln (`bild_umwandeln`, eigene Festlegung – Auto Digitizer des Originals nicht nachgebaut) und SVG
  // einlesen (`svg_lesen`, Stichart nach Breite wie EasyStitch des Originals). Ergebnis: neues Design.
  import { open } from "@tauri-apps/plugin-dialog";
  import { kern, type DesignJson } from "$kern/kern";
  import { arbeit, alsFehler } from "../arbeit.svelte";
  import { dateiname } from "../dateien.svelte";
  import Dialog from "../Dialog.svelte";
  import { lesen } from "../zahl";

  let { art, schliessen }: { art: "bild" | "svg"; schliessen: () => void } = $props();

  const experte = $derived(arbeit.modus === "experte");
  let pfad = $state<string | null>(null);
  // Standardbreite: Bild 80 mm (Kern), SVG wie die Zeichnung
  let breite = $state("");
  $effect.pre(() => {
    if (art === "bild" && breite === "") breite = "80";
  });
  let farben = $state("6");
  let minFlaeche = $state("4");
  let glaettung = $state("0,2");
  let hintergrund = $state(false);
  let aussparen = $state(true);
  let kuerzeste = $state(true);
  let laeuft = $state(false);
  let fehler = $state<string | null>(null);
  let hinweise = $state<string[]>([]);

  async function dateiWaehlen() {
    const p = await open({
      multiple: false,
      directory: false,
      filters:
        art === "bild"
          ? [{ name: "Bild", extensions: ["png", "jpg", "jpeg", "bmp", "gif"] }]
          : [{ name: "SVG (z. B. aus Inkscape)", extensions: ["svg"] }],
    });
    if (typeof p === "string") pfad = p;
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
          pfad,
          breite: lesen(breite) ?? undefined,
          farben: Math.round(lesen(farben) ?? 6),
          min_flaeche: lesen(minFlaeche) ?? undefined,
          glaettung: lesen(glaettung) ?? undefined,
          mit_hintergrund: hintergrund,
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
      design.stoff ??= arbeit.design.stoff;
      design.garnkarte ??= arbeit.design.garnkarte;
      arbeit.ersetzen(design, null, true);
      arbeit.auswahl = [];
      if (hinweise.length) arbeit.zeige(`Eingelesen mit ${hinweise.length} Hinweis(en): ${hinweise.join(" · ")}`, "info");
      schliessen();
    } catch (x) {
      fehler = alsFehler(x).meldung;
    } finally {
      laeuft = false;
    }
  }
</script>

<Dialog titel={art === "bild" ? "Bild umwandeln" : "SVG einlesen"} breite={480} {schliessen}>
  <div style="display:flex; flex-direction:column; gap:10px">
    <p class="mut small" style="margin:0">
      {#if art === "bild"}
        Ein Bild (PNG, JPG …) wird in Stickflächen nach Farben umgewandelt – ein neues Design. Verfahren: eigene Festlegung
        (der Auto-Digitizer des Originals ist nicht nachgebaut).
      {:else}
        Eine SVG-Zeichnung (z. B. aus Inkscape; Text vorher in Pfade umwandeln) wird zum neuen Design. Die Stichart je
        Form wählt das Programm nach der Breite wie EasyStitch des Originals (Linie bis 2 mm Laufstich, Fläche bis 8 mm
        Satin, darüber Tatami).
      {/if}
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
        <input class="input sm num" inputmode="decimal" bind:value={breite} placeholder={art === "svg" ? "wie SVG" : ""} style="width:90px" />
        <span class="unit">mm</span>
      </div>
    </div>
    {#if art === "bild"}
      <div class="frow" style="padding:0">
        <span class="fl">Farben</span>
        <div class="fc"><input class="input sm num" inputmode="numeric" bind:value={farben} style="width:90px" /></div>
      </div>
      {#if experte}
        <div class="frow" style="padding:0">
          <span class="fl">Kleinste Fläche <span class="xt">X</span></span>
          <div class="fc"><input class="input sm num" inputmode="decimal" bind:value={minFlaeche} style="width:90px" /><span class="unit">mm²</span></div>
        </div>
        <div class="frow" style="padding:0">
          <span class="fl">Glättung <span class="xt">X</span></span>
          <div class="fc"><input class="input sm num" inputmode="decimal" bind:value={glaettung} style="width:90px" /><span class="unit">mm</span></div>
        </div>
        <label class="frow" style="padding:0"><span class="fl">Hintergrund <span class="xt">X</span></span><span class="fc"><input type="checkbox" bind:checked={hintergrund} /> mitsticken</span></label>
      {/if}
    {:else if experte}
      <label class="frow" style="padding:0"><span class="fl">Überdeckung <span class="xt">X</span></span><span class="fc"><input type="checkbox" bind:checked={aussparen} /> aussparen (wie die SVG aussieht)</span></label>
      <label class="frow" style="padding:0"><span class="fl">Verbindung <span class="xt">X</span></span><span class="fc"><input type="checkbox" bind:checked={kuerzeste} /> kürzeste (Closest Join)</span></label>
    {/if}
    <div class="note small">Rahmen des neuen Designs: {arbeit.design.rahmen}. Das bisherige Design wird ersetzt (Rückgängig möglich).</div>
    {#if fehler}<div class="errbox">{fehler}</div>{/if}
  </div>
  {#snippet fuss()}
    <button class="btn btn-secondary" onclick={schliessen}>Abbrechen</button>
    <button class="btn btn-primary" disabled={!pfad || laeuft} onclick={umwandeln}>{laeuft ? "Wandle um …" : art === "bild" ? "Umwandeln" : "Einlesen"}</button>
  {/snippet}
</Dialog>
