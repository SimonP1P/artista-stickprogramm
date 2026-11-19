<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { onMount, untrack } from "svelte";
  import Vorschau from "$lib/Vorschau.svelte";
  import { empfohleneHoehe } from "$lib/alphabet_hoehen";
  import type { AlphabetInfo, Ergebnis, RahmenInfo } from "$lib/typen";

  let rahmenListe = $state<RahmenInfo[]>([]);
  let stoffe = $state<string[]>([]);
  let jsonText = $state("");
  let ergebnis = $state<Ergebnis | null>(null);
  let fehler = $state("");
  let meldung = $state("");
  let rechnet = $state(false);
  let zeigeSpruenge = $state(true);
  let zeigeEinstiche = $state(false);
  let dateiPfad = $state<string | null>(null);
  let vorschau: Vorschau;

  // Das JSON ist die einzige Quelle: Rahmen und Stoff werden darin gesetzt.
  let design = $derived.by(() => {
    try {
      const v = JSON.parse(jsonText);
      return v && typeof v === "object" && !Array.isArray(v) ? v : null;
    } catch {
      return null;
    }
  });
  let rahmenWahl = $derived<string>(design?.rahmen ?? "artista 130 x 100");
  let stoffWahl = $derived<string>(design?.stoff ?? "0 Software Defaults");
  let rahmenInfo = $derived(rahmenListe.find((r) => r.name === rahmenWahl));

  function feldSetzen(feld: "rahmen" | "stoff", wert: string) {
    if (!design) {
      fehler = "Das Design-JSON ist ungültig – erst korrigieren.";
      return;
    }
    jsonText = JSON.stringify({ ...design, [feld]: wert }, null, 2);
  }

  // ---- Schrift mit den Original-Alphabeten (kern/stick-kern/src/schrift.rs)
  let alphabete = $state<AlphabetInfo[]>([]);
  let textWahl = $state(-1); // Index des bearbeiteten Text-Objekts, -1 = neuer Text
  let tText = $state("Anna");
  let tAlphabet = $state("Drifter");
  let tHoehe = $state(20);
  let tBreite = $state(100);
  let tFarbe = $state("#1f3a93");
  let textObjekte = $derived(
    ((design?.objekte ?? []) as any[]).map((o, i) => ({ o, i })).filter((x) => x.o?.art === "text"),
  );
  let empfohlen = $derived(empfohleneHoehe(tAlphabet));

  function textObjekt(alt?: any) {
    return {
      ...(alt ?? {}),
      art: "text",
      text: tText,
      alphabet: tAlphabet,
      hoehe: Number(tHoehe) || 20,
      breite: Number(tBreite) || 100,
      farbe: tFarbe,
      position: alt?.position ?? [0, 0],
    };
  }

  function objekteSetzen(objekte: any[]) {
    jsonText = JSON.stringify({ ...design, objekte }, null, 2);
  }

  function textEinfuegen() {
    if (!design) {
      fehler = "Das Design-JSON ist ungültig – erst korrigieren.";
      return;
    }
    const objekte = [...(design.objekte ?? [])];
    objekte.push(textObjekt());
    objekteSetzen(objekte);
    textWahl = objekte.length - 1;
    meldung = `Text „${tText}“ eingefügt.`;
  }

  function textWaehlen(i: number) {
    textWahl = i;
    const o = i >= 0 ? design?.objekte?.[i] : null;
    if (o) {
      tText = o.text ?? "";
      tAlphabet = o.alphabet ?? tAlphabet;
      tHoehe = o.hoehe ?? 20;
      tBreite = o.breite ?? 100;
      tFarbe = o.farbe ?? "#000000";
    }
  }

  function textEntfernen() {
    if (textWahl < 0 || !design) return;
    objekteSetzen((design.objekte ?? []).filter((_: any, i: number) => i !== textWahl));
    textWahl = -1;
  }

  // Bearbeiteter Text folgt den Eingaben sofort (Vorschau beim Tippen)
  $effect(() => {
    void [tText, tAlphabet, tHoehe, tBreite, tFarbe];
    untrack(() => {
      if (textWahl < 0 || !design) return;
      const objekte = [...(design.objekte ?? [])];
      if (objekte[textWahl]?.art !== "text") return;
      objekte[textWahl] = textObjekt(objekte[textWahl]);
      const neu = JSON.stringify({ ...design, objekte }, null, 2);
      if (neu !== jsonText) jsonText = neu;
    });
  });

  function neu() {
    jsonText = JSON.stringify({ name: "Neues Design", rahmen: rahmenWahl, stoff: stoffWahl, objekte: [] }, null, 2);
    dateiPfad = null;
    textWahl = -1;
    meldung = "Neues, leeres Design – zum Beispiel einen Text einfügen.";
  }

  // Neu berechnen, sobald sich das Design ändert (kurz verzögert beim Tippen)
  let zeitgeber: ReturnType<typeof setTimeout> | undefined;
  let laufNr = 0;
  $effect(() => {
    const text = jsonText;
    clearTimeout(zeitgeber);
    if (!text.trim()) return;
    zeitgeber = setTimeout(() => berechnen(text), 250);
  });

  async function berechnen(text: string) {
    const nr = ++laufNr;
    rechnet = true;
    try {
      const e = await invoke<Ergebnis>("design_berechnen", { json: text });
      if (nr === laufNr) {
        ergebnis = e;
        fehler = "";
      }
    } catch (e) {
      if (nr === laufNr) fehler = String(e);
    } finally {
      if (nr === laufNr) rechnet = false;
    }
  }

  function dateiname() {
    return (ergebnis?.name ?? "Design").replace(/\s+/g, "_");
  }

  async function beispiel() {
    jsonText = await invoke<string>("beispiel_design");
    dateiPfad = null;
    textWahl = -1;
    meldung = "Beispieldesign geladen.";
    vorschau?.einpassen();
  }

  async function oeffnen() {
    const pfad = await open({ multiple: false, directory: false, filters: [{ name: "Design (JSON)", extensions: ["json"] }] });
    if (typeof pfad !== "string") return;
    try {
      jsonText = await invoke<string>("datei_lesen", { pfad });
      dateiPfad = pfad;
      textWahl = -1;
      meldung = `Geöffnet: ${pfad}`;
      vorschau?.einpassen();
    } catch (e) {
      fehler = String(e);
    }
  }

  async function designSpeichern() {
    const pfad = await save({
      defaultPath: dateiPfad ?? `${dateiname()}.json`,
      filters: [{ name: "Design (JSON)", extensions: ["json"] }],
    });
    if (!pfad) return;
    try {
      await invoke("text_speichern", { pfad, text: jsonText });
      dateiPfad = pfad;
      meldung = `Design gespeichert: ${pfad}`;
    } catch (e) {
      fehler = String(e);
    }
  }

  async function expSpeichern() {
    const pfad = await save({ defaultPath: `${dateiname()}.exp`, filters: [{ name: "Stickdatei (EXP)", extensions: ["exp"] }] });
    if (!pfad) return;
    try {
      const bytes = await invoke<number>("exp_speichern", { json: jsonText, pfad });
      meldung = `EXP gespeichert: ${pfad} (${bytes} Byte)`;
    } catch (e) {
      fehler = String(e);
    }
  }

  onMount(async () => {
    rahmenListe = await invoke<RahmenInfo[]>("rahmen_liste");
    stoffe = await invoke<string[]>("stoffe_liste");
    alphabete = await invoke<AlphabetInfo[]>("alphabete_liste");
    if (alphabete.length && !alphabete.some((a) => a.name === tAlphabet)) tAlphabet = alphabete[0].name;
    await beispiel();
  });

  const mm = (v: number) => v.toLocaleString("de-DE", { minimumFractionDigits: 1, maximumFractionDigits: 1 });
  const mm2 = (v: number) => v.toLocaleString("de-DE", { minimumFractionDigits: 2, maximumFractionDigits: 2 });
</script>

<div class="programm">
  <header>
    <div class="titel">
      <strong>Artista Stickprogramm</strong>
      <span>BERNINA artista 185</span>
    </div>
    <div class="knoepfe">
      <button onclick={neu}>Neu</button>
      <button onclick={beispiel}>Beispiel</button>
      <button onclick={oeffnen}>Öffnen…</button>
      <button onclick={designSpeichern}>Design speichern…</button>
      <button class="haupt" onclick={expSpeichern} disabled={!ergebnis || !!fehler}>EXP speichern…</button>
    </div>
  </header>

  <aside class="links">
    <section>
      <h2><span class="nr">1</span> Rahmen</h2>
      <select value={rahmenWahl} onchange={(e) => feldSetzen("rahmen", e.currentTarget.value)}>
        {#each rahmenListe as r (r.name)}
          <option value={r.name}>{r.name}</option>
        {/each}
      </select>
      {#if rahmenInfo}
        <p class="klein">Stickfeld {mm(rahmenInfo.breite)} × {mm(rahmenInfo.hoehe)} mm{rahmenInfo.hinweis ? ` · ${rahmenInfo.hinweis}` : ""}</p>
      {/if}
    </section>
    <section>
      <h2><span class="nr">2</span> Stoff</h2>
      <select value={stoffWahl} onchange={(e) => feldSetzen("stoff", e.currentTarget.value)}>
        {#each stoffe as s (s)}
          <option value={s}>{s}</option>
        {/each}
      </select>
      <p class="klein">Stichabstände und Zugausgleich wie im Original (Fabric Settings).</p>
    </section>
    <section class="schrift">
      <h2><span class="nr">3</span> Design anpassen</h2>
      <h3>Schrift</h3>
      <select value={textWahl} onchange={(e) => textWaehlen(Number(e.currentTarget.value))}>
        <option value={-1}>Neuer Text …</option>
        {#each textObjekte as t (t.i)}
          <option value={t.i}>„{t.o.text}“ – {t.o.alphabet}, {t.o.hoehe ?? 20} mm</option>
        {/each}
      </select>
      <input class="textfeld" type="text" bind:value={tText} placeholder="Text eingeben" />
      <select bind:value={tAlphabet}>
        {#each alphabete as a (a.datei)}
          <option value={a.name}>{a.name}</option>
        {/each}
      </select>
      <div class="zeile">
        <label>Höhe <input type="number" min="1" max="200" step="1" bind:value={tHoehe} /> mm</label>
        <label>Breite <input type="number" min="30" max="300" step="5" bind:value={tBreite} /> %</label>
        <input type="color" bind:value={tFarbe} title="Garnfarbe (Vorschau)" />
      </div>
      {#if empfohlen}
        <p class="klein" class:warnung-klein={tHoehe < empfohlen[0] || tHoehe > empfohlen[1]}>
          Empfohlen für {tAlphabet}: {empfohlen[0]}–{empfohlen[1]} mm (Handbuch, Alphabet Samples)
        </p>
      {/if}
      <div class="zeile">
        {#if textWahl < 0}
          <button onclick={textEinfuegen} disabled={!tText.trim()}>Text einfügen</button>
        {:else}
          <button onclick={textEinfuegen} disabled={!tText.trim()}>Als neuen Text einfügen</button>
          <button onclick={textEntfernen}>Entfernen</button>
        {/if}
      </div>
    </section>
    <section class="editor">
      <h3>Alle Objekte (JSON)</h3>
      <textarea bind:value={jsonText} spellcheck="false" autocomplete="off"></textarea>
      <p class="klein">Koordinaten in mm, Ursprung = Rahmenmitte, +y nach oben. Format: <code>stickprogramm/README.md</code></p>
    </section>
  </aside>

  <main>
    <Vorschau bind:this={vorschau} {ergebnis} spruenge={zeigeSpruenge} einstiche={zeigeEinstiche} />
    <div class="werkzeug">
      <label><input type="checkbox" bind:checked={zeigeSpruenge} /> Sprünge</label>
      <label><input type="checkbox" bind:checked={zeigeEinstiche} /> Einstiche</label>
      <button onclick={() => vorschau?.einpassen()}>Einpassen</button>
      <span class="hinweis">Mausrad: Zoom · Ziehen: verschieben · Doppelklick: einpassen</span>
    </div>
    {#if rechnet}<div class="rechnet">rechnet …</div>{/if}
  </main>

  <aside class="rechts">
    <section>
      <h2>Design</h2>
      {#if ergebnis}
        <dl>
          <dt>Name</dt><dd>{ergebnis.name}</dd>
          <dt>Stiche</dt><dd>{ergebnis.statistik.stiche.toLocaleString("de-DE")}</dd>
          <dt>Sprünge</dt><dd>{ergebnis.statistik.spruenge}</dd>
          <dt>Farbwechsel</dt><dd>{ergebnis.statistik.farbwechsel}</dd>
          <dt>Größe</dt><dd>{mm(ergebnis.statistik.breite)} × {mm(ergebnis.statistik.hoehe)} mm</dd>
          <dt>Stichlänge</dt><dd>{mm2(ergebnis.statistik.min_stich)} – {mm2(ergebnis.statistik.max_stich)} mm</dd>
        </dl>
        <h3>Farben</h3>
        <ol class="farben">
          {#each ergebnis.farben as f, i (i)}
            <li><span class="feld" style:background={f}></span>{i + 1}. {f}</li>
          {/each}
        </ol>
        {#if ergebnis.fehlende_zeichen}
          <p class="warnung">Nicht im Alphabet: „{ergebnis.fehlende_zeichen}“</p>
        {/if}
        {#if ergebnis.ausserhalb > 0}
          <p class="warnung">{ergebnis.ausserhalb} Einstiche liegen außerhalb des Stickfelds!</p>
        {:else}
          <p class="ok">Passt in den Rahmen.</p>
        {/if}
      {:else}
        <p class="klein">Noch kein Design berechnet.</p>
      {/if}
    </section>
    <section>
      <h2><span class="nr">4</span> An die Maschine</h2>
      <button disabled>Senden</button>
      <p class="klein">Kommt mit Phase 5 – die Übertragung zur artista 185 wird gerade erforscht.</p>
    </section>
  </aside>

  <footer>
    {#if fehler}<span class="fehler">{fehler}</span>{:else}<span>{meldung}</span>{/if}
  </footer>
</div>

<style>
  :global(html, body) {
    margin: 0;
    height: 100%;
    font-family: "Segoe UI", system-ui, -apple-system, sans-serif;
    font-size: 14px;
    color: #1d2329;
    background: #f4f5f7;
  }
  .programm {
    display: grid;
    grid-template-columns: 340px 1fr 270px;
    grid-template-rows: auto 1fr auto;
    grid-template-areas: "kopf kopf kopf" "links mitte rechts" "fuss fuss fuss";
    height: 100vh;
  }
  header {
    grid-area: kopf;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 14px;
    background: #22303c;
    color: #fff;
  }
  .titel strong {
    font-size: 16px;
    margin-right: 10px;
  }
  .titel span {
    opacity: 0.7;
  }
  .knoepfe {
    display: flex;
    gap: 6px;
  }
  button {
    font: inherit;
    padding: 5px 12px;
    border: 1px solid #b8c0c8;
    border-radius: 5px;
    background: #fff;
    color: #1d2329;
    cursor: pointer;
  }
  button:hover:not(:disabled) {
    background: #eef2f6;
  }
  button:disabled {
    opacity: 0.5;
    cursor: default;
  }
  button.haupt {
    background: #2e7d32;
    border-color: #2e7d32;
    color: #fff;
  }
  aside {
    overflow: auto;
    background: #fff;
    padding: 6px 14px;
  }
  .links {
    grid-area: links;
    border-right: 1px solid #dde1e5;
    display: flex;
    flex-direction: column;
  }
  .rechts {
    grid-area: rechts;
    border-left: 1px solid #dde1e5;
  }
  section {
    margin-bottom: 14px;
  }
  .editor {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 200px;
  }
  .schrift {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .schrift h3 {
    margin: 2px 0 0;
  }
  .textfeld {
    font: inherit;
    font-size: 15px;
    padding: 5px 6px;
    border: 1px solid #c9d0d6;
    border-radius: 4px;
  }
  .zeile {
    display: flex;
    gap: 10px;
    align-items: center;
    flex-wrap: wrap;
  }
  .zeile input[type="number"] {
    width: 56px;
    font: inherit;
    padding: 2px 4px;
  }
  .zeile input[type="color"] {
    width: 34px;
    height: 26px;
    padding: 0;
    border: 1px solid #c9d0d6;
  }
  .warnung-klein {
    color: #b71c1c;
  }
  h2 {
    font-size: 13px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: #4a5662;
    margin: 10px 0 6px;
  }
  h3 {
    font-size: 13px;
    margin: 12px 0 4px;
  }
  .nr {
    display: inline-block;
    width: 18px;
    height: 18px;
    line-height: 18px;
    text-align: center;
    border-radius: 50%;
    background: #22303c;
    color: #fff;
    font-size: 11px;
    margin-right: 4px;
  }
  select {
    width: 100%;
    font: inherit;
    padding: 4px;
  }
  textarea {
    flex: 1;
    width: 100%;
    box-sizing: border-box;
    font-family: Consolas, "Cascadia Mono", monospace;
    font-size: 12px;
    resize: none;
    padding: 6px;
    border: 1px solid #c9d0d6;
    border-radius: 4px;
  }
  .klein {
    font-size: 12px;
    color: #5d6b78;
    margin: 4px 0;
  }
  main {
    grid-area: mitte;
    position: relative;
    overflow: hidden;
  }
  .werkzeug {
    position: absolute;
    left: 10px;
    top: 10px;
    display: flex;
    gap: 12px;
    align-items: center;
    background: rgba(255, 255, 255, 0.92);
    padding: 4px 10px;
    border-radius: 6px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
  }
  .hinweis {
    font-size: 12px;
    color: #6b7783;
  }
  .rechnet {
    position: absolute;
    right: 12px;
    top: 12px;
    font-size: 12px;
    color: #6b7783;
  }
  dl {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 3px 10px;
    margin: 0;
  }
  dt {
    color: #5d6b78;
  }
  dd {
    margin: 0;
    text-align: right;
  }
  .farben {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  .farben li {
    display: flex;
    align-items: center;
    gap: 6px;
    margin: 2px 0;
    font-family: Consolas, monospace;
    font-size: 12px;
  }
  .feld {
    width: 16px;
    height: 16px;
    border-radius: 3px;
    border: 1px solid rgba(0, 0, 0, 0.2);
  }
  .warnung {
    color: #b71c1c;
    font-weight: 600;
  }
  .ok {
    color: #2e7d32;
  }
  footer {
    grid-area: fuss;
    padding: 4px 14px;
    font-size: 12px;
    background: #e9ecef;
    border-top: 1px solid #dde1e5;
    min-height: 18px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .fehler {
    color: #b71c1c;
  }
</style>
