<script lang="ts">
  // Einfacher Modus: Assistent in 4 Schritten (UEBERGABE §2/§3 „Assistent“). Jede Eingabe ändert das Design-JSON;
  // Stiche, Größe, Garne und Prüfungen kommen aus dem Kern (`design_berechnen`).
  import type { ObjektJson, TextJson, MonogrammJson, Transformation } from "$kern/kern";
  import { arbeit } from "./arbeit.svelte";
  import { expExportieren, oeffnen, speichern } from "./dateien.svelte";
  import Icon from "./Icon.svelte";
  import { ART, IC } from "./icons";
  import { garnText, listen, STOFF_TEXT } from "./listen.svelte";
  import { ganz, lesen, zahl, zahlFest } from "./zahl";
  import { ANLEITUNG_RAHMEN, maschine, SIMULATION } from "./maschine.svelte";

  let { istMac }: { istMac: boolean } = $props();

  type Text = TextJson & { art: "text" };
  type Mono = MonogrammJson & { art: "monogramm" };

  const SCHRITTE = $derived(["Rahmen & Stoff", "Gestalten", "Prüfen & Garn", istMac ? "Speichern" : "Senden"]);
  const STANDARD_ALPHABET = "Drifter";
  const STANDARD_FARBE = "#004780";

  $effect(() => {
    listen.laden();
  });

  const e = $derived(arbeit.ergebnis);
  const rahmen = $derived(arbeit.rahmenListe.find((r) => r.name === arbeit.design.rahmen));
  const quer = $derived(/ Rotated$/.test(arbeit.design.rahmen ?? ""));
  const rahmenBasis = $derived(arbeit.rahmenListe.filter((r) => !/ Rotated$/.test(r.name)));

  /** Objekt, das der Assistent bearbeitet: gewählter Text/Monogramm, sonst der erste */
  const haupt = $derived.by(() => {
    const o = arbeit.objekte;
    const g = arbeit.gewaehlt;
    if (g >= 0 && (o[g]?.art === "text" || o[g]?.art === "monogramm")) return g;
    return o.findIndex((x) => x.art === "text" || x.art === "monogramm");
  });
  const objekt = $derived(haupt >= 0 ? arbeit.objekte[haupt] : undefined);
  const text = $derived(objekt?.art === "text" ? (objekt as Text) : undefined);
  const mono = $derived(objekt?.art === "monogramm" ? (objekt as Mono) : undefined);
  const alphabet = $derived(listen.alphabet(text?.alphabet));
  const fehlend = $derived(
    (e?.hinweise ?? []).filter((h) => h.art === "fehlende_zeichen" && h.objekt === haupt).map((h) => h.meldung),
  );
  const hauptGarn = $derived.by(() => {
    if (!e || haupt < 0) return undefined;
    const k = e.bloecke.findIndex((b) => b.objekt === haupt || b.objekte.includes(haupt));
    return k >= 0 ? e.garne[k] : undefined;
  });

  const transform = $derived(arbeit.design.transform ?? {});
  const skalierung = $derived(
    Array.isArray(transform.skalieren) ? (transform.skalieren[0] ?? 100) : (transform.skalieren ?? 100),
  );

  function objektAendern(f: (o: ObjektJson) => void, gruppe?: string) {
    const i = haupt;
    if (i < 0) return;
    arbeit.aendern((d) => f(d.objekte![i]), gruppe ? `${gruppe}:${i}` : undefined);
  }
  function transformAendern(f: (t: Transformation) => void, gruppe?: string) {
    arbeit.aendern((d) => {
      d.transform ??= {};
      f(d.transform);
    }, gruppe);
  }

  function rahmenWaehlen(basis: string) {
    arbeit.aendern((d) => {
      d.rahmen = (quer ? `${basis} Rotated` : basis) as typeof d.rahmen;
    });
  }
  function querUmschalten(an: boolean) {
    const basis = (arbeit.design.rahmen ?? "artista 130 x 100").replace(/ Rotated$/, "");
    arbeit.aendern((d) => (d.rahmen = (an ? `${basis} Rotated` : basis) as typeof d.rahmen));
  }

  function textEinfuegen() {
    arbeit.aendern((d) => {
      d.objekte ??= [];
      d.objekte.push({ art: "text", name: "Anna", text: "Anna", alphabet: STANDARD_ALPHABET, hoehe: 20, farbe: STANDARD_FARBE });
    });
    arbeit.auswahl = [arbeit.objekte.length - 1];
  }
  function monogrammEinfuegen() {
    arbeit.aendern((d) => {
      d.objekte ??= [];
      d.objekte.push({ art: "monogramm", name: "Monogramm", initialen: "AB", hoehe: 30, rahmen: "raute", farbe: STANDARD_FARBE });
    });
    arbeit.auswahl = [arbeit.objekte.length - 1];
  }

  const form = $derived(
    !text?.grundlinie ? "gerade" : text.grundlinie.art === "kreis" ? (text.grundlinie.lage === "unten" ? "unten" : "oben") : "andere",
  );
  function formSetzen(f: "gerade" | "oben" | "unten") {
    objektAendern((o) => {
      const t = o as Text;
      if (f === "gerade") delete t.grundlinie;
      // Kreis-Grundlinie mit Radius der Vorlage (50 mm, Dialog „Layout“ des Originals)
      else t.grundlinie = { art: "kreis", lage: f, radius: t.grundlinie?.radius ?? 50 };
    });
  }

  const hoeheWarnung = $derived.by(() => {
    const h = text?.hoehe ?? 18;
    if (!alphabet || alphabet.min_hoehe == null) return null;
    if (h < alphabet.min_hoehe) return `Kleiner als empfohlen (ab ${zahl(alphabet.min_hoehe)} mm) – Details können zulaufen.`;
    if (alphabet.max_hoehe != null && h > alphabet.max_hoehe)
      return `Größer als empfohlen (bis ${zahl(alphabet.max_hoehe)} mm) – lange Satinstiche können locker werden.`;
    return null;
  });
  const BAND_MIN = 3, BAND_MAX = 120;
  const band = $derived.by(() => {
    if (!alphabet || alphabet.min_hoehe == null) return null;
    const a = (alphabet.min_hoehe - BAND_MIN) / (BAND_MAX - BAND_MIN);
    const b = ((alphabet.max_hoehe ?? alphabet.min_hoehe) - BAND_MIN) / (BAND_MAX - BAND_MIN);
    return { l: `${a * 100}%`, w: `${Math.max(1, (b - a) * 100)}%` };
  });

  /** Design in die Mitte: Verschiebung um die Mitte der Stiche (Begrenzung aus dem Kern) */
  function mittig() {
    const b = e?.begrenzung;
    if (!b) return;
    const [mx, my] = [(b[0] + b[2]) / 2, (b[1] + b[3]) / 2];
    transformAendern((t) => {
      const [vx, vy] = t.verschieben ?? [0, 0];
      t.verschieben = [round(vx - mx), round(vy - my)];
    });
  }
  /** Einpassen: so skalieren, dass die Stiche mit 5 % Rand ins Stickfeld passen, und mittig setzen */
  function einpassen() {
    const b = e?.begrenzung;
    if (!b || !rahmen) return;
    const w = b[2] - b[0], h = b[3] - b[1];
    if (w <= 0 || h <= 0) return;
    const f = Math.min(rahmen.breite / w, rahmen.hoehe / h) * 0.95;
    const [mx, my] = [(b[0] + b[2]) / 2, (b[1] + b[3]) / 2];
    transformAendern((t) => {
      t.skalieren = Math.floor(skalierung * f);
      const [vx, vy] = t.verschieben ?? [0, 0];
      t.verschieben = [round(vx - mx), round(vy - my)];
    });
  }
  const round = (v: number) => Math.round(v * 10) / 10;

  const passt = $derived(!!e && e.ausserhalb === 0 && !arbeit.fehler);
  const groesseText = $derived(
    e && rahmen
      ? `${zahlFest(e.statistik.breite)} × ${zahlFest(e.statistik.hoehe)} mm im Rahmen ${zahl(rahmen.breite)} × ${zahl(rahmen.hoehe)} mm`
      : "",
  );

  const LAGE_TEXT: Record<string, string> = {
    platzierung: "danach Applikationsstoff auflegen",
    schnittlinie: "danach Stoff entlang der Linie zuschneiden",
    heften: "danach überstehenden Stoff zuschneiden",
    abdecken: "Abdeckstich",
  };
  const bloecke = $derived(
    (e?.farben ?? []).map((farbe, k) => {
      const b = e!.bloecke[k];
      const namen = (b?.objekte ?? []).map((i) => {
        const o = arbeit.objekte[i];
        return o?.name || ("text" in (o ?? {}) ? (o as Text).text : "") || ART[o?.art ?? ""]?.[0] || `Nr. ${i + 1}`;
      });
      return { k, farbe, garn: e!.garne[k], b, namen, lagen: (b?.applikation ?? []).map((l) => LAGE_TEXT[l] ?? l) };
    }),
  );

  const weiterText = $derived(["Weiter: Gestalten", "Weiter: Prüfen", istMac ? "Weiter: Speichern" : "Weiter: Senden"]);
</script>

<aside class="pane" aria-label="Assistent" style="width:360px; border-right:1px solid var(--line)">
  <nav aria-label="Schritte" style="flex:none; display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); border-bottom:1px solid var(--line); background:var(--color-bg)">
    {#each SCHRITTE as s, k (k)}
      <button
        class="stp"
        class:cur={arbeit.schritt === k + 1}
        class:done={arbeit.schritt > k + 1}
        aria-current={arbeit.schritt === k + 1 ? "step" : undefined}
        onclick={() => (arbeit.schritt = k + 1)}
        style="flex-direction:column; align-items:flex-start; gap:3px; font-size:12px"
      >
        <span class="n">{k + 1}</span><span style="white-space:nowrap">{s}</span>
      </button>
    {/each}
  </nav>

  <div class="scroll" style="padding:16px 16px 24px">
    {#if arbeit.schritt === 1}
      <h2 style="font-size:25px; margin:0 0 2px">Rahmen und Stoff</h2>
      <p class="mut" style="margin:0 0 12px">Welcher Rahmen ist an der Maschine eingesetzt?</p>
      <div style="display:flex; flex-direction:column; gap:6px">
        {#each rahmenBasis as r (r.name)}
          {@const an = (arbeit.design.rahmen ?? "").replace(/ Rotated$/, "") === r.name}
          <button class="rk" class:on={an} aria-pressed={an} onclick={() => rahmenWaehlen(r.name)}>
            <span class="pic" aria-hidden="true">
              <svg viewBox="{-r.breite / 2 - 2} {-r.hoehe / 2 - 2} {r.breite + 4} {r.hoehe + 4}" style="width:46px; height:46px">
                <path
                  d={r.form.map(([x, y], j) => `${j ? "L" : "M"}${x} ${-y}`).join("") + "Z"}
                  fill="none"
                  stroke="currentColor"
                  stroke-width={Math.max(r.breite, r.hoehe) / 30}
                />
              </svg>
            </span>
            <span style="display:flex; flex-direction:column; min-width:0">
              <span class="hd" style="font-size:16px">
                {r.name.replace(/^artista /, "").replace(/ x /, " × ")}
                {#if r.name === "artista 130 x 100"}<span class="tag tag-accent" style="margin-left:8px; font-family:var(--font-body); font-weight:400">Standard</span>{/if}
              </span>
              <span class="mut small">Stickfeld {zahl(r.breite)} × {zahl(r.hoehe)} mm{r.hinweis ? ` · ${r.hinweis}` : ""}</span>
            </span>
          </button>
        {/each}
      </div>
      <label style="display:flex; gap:8px; align-items:center; margin:10px 0 18px; font-size:13px">
        <input type="checkbox" checked={quer} onchange={(ev) => querUmschalten((ev.currentTarget as HTMLInputElement).checked)} style="width:16px; height:16px; accent-color:var(--color-accent)" />
        Rahmen quer einspannen (um 90° gedreht)
      </label>
      <h3 style="font-size:19px; margin:0 0 6px">Stoff</h3>
      <p class="mut small" style="margin:0 0 8px">Der Stoff ändert Dichte, Unterlage und Zugausgleich – die Stiche werden sofort neu berechnet.</p>
      <div role="radiogroup" aria-label="Stoff" style="display:flex; flex-direction:column; gap:5px">
        {#each listen.stoffe as s (s.name)}
          {@const t = STOFF_TEXT[s.name] ?? { de: s.name, bsp: "" }}
          <label class="opt">
            <input type="radio" name="stoff" checked={(arbeit.design.stoff ?? "0 Software Defaults") === s.name} onchange={() => arbeit.aendern((d) => (d.stoff = s.name as typeof d.stoff))} />
            <span style="display:flex; flex-direction:column; min-width:0">
              <span><b style="font-weight:500">{t.de}</b> <span class="mut small">– z. B. {t.bsp}</span></span>
              <span class="mut small">{s.name} · Satin {zahl(s.satin_abstand_fest, 2)} · Tatami {zahl(s.tatami_abstand, 2)} · Zugausgleich {zahl(s.zugausgleich, 2)} mm</span>
            </span>
          </label>
        {/each}
      </div>
      <div class="note" style="margin-top:12px">
        <b style="font-weight:500">Material-Tipp</b> (eigener Hinweis, nicht aus dem Original): fester Stoff oder passendes Vlies –
        Abreißvlies, bei dehnbarem Stoff Schneide- oder Bügelvlies; bei Frottee zusätzlich wasserlösliche Folie oben.
      </div>
    {:else if arbeit.schritt === 2}
      {#if !objekt}
        <h2 style="font-size:25px; margin:0 0 2px">Was möchtest du sticken?</h2>
        <p class="mut" style="margin:0 0 12px">Am häufigsten: ein Name oder Monogramm.</p>
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:8px">
          <button class="big" onclick={textEinfuegen}><Icon d={ART.text[1]} groesse="l" /><b style="font-weight:500">Text</b><span class="mut small">Name, Spruch – 50 Alphabete</span></button>
          <button class="big" onclick={monogrammEinfuegen}><Icon d={ART.monogramm[1]} groesse="l" /><b style="font-weight:500">Monogramm</b><span class="mut small">2–3 Initialen, mit Rahmen</span></button>
          <button class="big" onclick={() => (arbeit.dialog = { art: "bild" })}><Icon d={IC.bild} groesse="l" /><b style="font-weight:500">Bild umwandeln</b><span class="mut small">PNG / JPG → Stickflächen</span></button>
          <button class="big" onclick={() => (arbeit.dialog = { art: "svg" })}><Icon d={IC.bezier} groesse="l" /><b style="font-weight:500">SVG einlesen</b><span class="mut small">Zeichnung aus Inkscape</span></button>
          <button class="big" onclick={oeffnen}><Icon d={IC.ordner} groesse="l" /><b style="font-weight:500">Design öffnen</b><span class="mut small">gespeichertes Design (JSON)</span></button>
          <button class="big" disabled><Icon d={IC.kommtNoch} groesse="l" /><span><b style="font-weight:500">Motiv aus Sammlung</b> <span class="soon">kommt noch</span></span><span class="small">Formen selbst zeichnen: Expertenmodus</span></button>
        </div>
        {#if arbeit.objekte.length}
          <div class="note" style="margin-top:12px">Das Design enthält {arbeit.objekte.length} Objekt(e) ohne Text – bearbeiten im Expertenmodus.</div>
        {/if}
      {:else if text}
        <h2 style="font-size:25px; margin:0 0 10px">Text</h2>
        <div class="field" style="margin-bottom:12px">
          <label for="atxt">Text <span class="mut">(neue Zeile mit Eingabe)</span></label>
          <textarea
            id="atxt"
            class="input"
            rows="2"
            style="min-height:58px; font-size:16px"
            value={text.text}
            oninput={(ev) => {
              const v = (ev.currentTarget as HTMLTextAreaElement).value;
              objektAendern((o) => {
                (o as Text).text = v;
                o.name = v.split("\n")[0];
              }, "text");
            }}
          ></textarea>
        </div>
        {#each fehlend as f (f)}
          <div class="warnbox" style="margin:-4px 0 12px"><Icon d={IC.warnung} /><span>{f}</span></div>
        {/each}
        <div class="field" style="margin-bottom:12px">
          <label for="aalpha">Alphabet</label>
          <button id="aalpha" class="btn btn-secondary pick" onclick={() => (arbeit.dialog = { art: "alphabet", objekt: haupt })} style="min-height:40px">
            <span class="t"><b style="font-weight:500">{text.alphabet}</b>
              {#if alphabet?.min_hoehe != null}<span class="mut small"> · empfohlen {zahl(alphabet.min_hoehe)}–{alphabet.max_hoehe != null ? zahl(alphabet.max_hoehe) : "?"} mm</span>{/if}</span>
            <Icon d={IC.chevronRechts} />
          </button>
        </div>
        <div class="field" style="margin-bottom:12px">
          <label for="ahoehe">Höhe der Buchstaben</label>
          <div style="display:flex; align-items:center; gap:10px">
            <div class="range" style="flex:1">
              {#if band}<span class="band" style="left:{band.l}; width:{band.w}" title="empfohlen"></span>{/if}
              <input
                type="range"
                min={BAND_MIN}
                max={BAND_MAX}
                step="1"
                value={text.hoehe ?? 18}
                aria-label="Höhe (Schieberegler)"
                oninput={(ev) => {
                  const v = Number((ev.currentTarget as HTMLInputElement).value);
                  objektAendern((o) => ((o as Text).hoehe = v), "hoehe");
                }}
              />
            </div>
            <input
              id="ahoehe"
              class="input sm num"
              style="width:62px"
              inputmode="decimal"
              value={zahl(text.hoehe ?? 18)}
              onchange={(ev) => {
                const v = lesen((ev.currentTarget as HTMLInputElement).value);
                if (v && v > 0) objektAendern((o) => ((o as Text).hoehe = v));
              }}
            /><span class="unit">mm</span>
          </div>
          <div class="small mut" style="margin-top:2px">grünes Band = empfohlene Höhe des Alphabets (Handbuch)</div>
          {#if hoeheWarnung}<div class="warnbox" style="margin-top:6px"><Icon d={IC.warnung} /><span>{hoeheWarnung}</span></div>{/if}
        </div>
        <div style="display:grid; grid-template-columns:1fr 1fr; gap:10px; margin-bottom:12px">
          <div class="field">
            <span class="fl">Ausrichtung</span>
            <div class="seg seg-sm" role="radiogroup" aria-label="Ausrichtung" style="display:flex">
              {#each [["links", "links"], ["mitte", "mittig"], ["rechts", "rechts"]] as [w, l] (w)}
                <label class="seg-opt" style="flex:1; justify-content:center"><input type="radio" name="ausr" checked={(text.ausrichtung ?? "mitte") === w} onchange={() => objektAendern((o) => ((o as Text).ausrichtung = w as Text["ausrichtung"]))} />{l}</label>
              {/each}
            </div>
          </div>
          <div class="field">
            <span class="fl">Form</span>
            <div class="seg seg-sm" role="radiogroup" aria-label="Grundlinie" style="display:flex">
              {#each [["gerade", "gerade"], ["oben", "Bogen oben"], ["unten", "Bogen unten"]] as [w, l] (w)}
                <label class="seg-opt" style="flex:1; justify-content:center; padding-inline:4px; line-height:1.15; text-align:center"><input type="radio" name="form" checked={form === w} onchange={() => formSetzen(w as "gerade")} />{l}</label>
              {/each}
            </div>
          </div>
        </div>
        {@render garnWahl()}
        {@render groesseDrehen()}
        {@render passung()}
      {:else if mono}
        <h2 style="font-size:25px; margin:0 0 10px">Monogramm</h2>
        <div class="field" style="margin-bottom:12px">
          <label for="ainit">Initialen <span class="mut">(2 oder 3 Buchstaben A–Z)</span></label>
          <input
            id="ainit"
            class="input"
            style="font-size:16px"
            maxlength="3"
            value={mono.initialen}
            oninput={(ev) => {
              const v = (ev.currentTarget as HTMLInputElement).value.toUpperCase();
              objektAendern((o) => {
                (o as Mono).initialen = v;
                o.name = v;
              }, "init");
            }}
          />
        </div>
        <div class="field" style="margin-bottom:12px">
          <span class="fl">Rahmen</span>
          <div class="seg seg-sm" role="radiogroup" aria-label="Rahmen des Monogramms" style="display:flex">
            {#each [["raute", "Raute"], ["zier", "Zier"], ["keine", "keiner"]] as [w, l] (w)}
              <label class="seg-opt" style="flex:1; justify-content:center"><input type="radio" name="mrahmen" checked={(mono.rahmen ?? "keine") === w} onchange={() => objektAendern((o) => ((o as Mono).rahmen = w as Mono["rahmen"]))} />{l}</label>
            {/each}
          </div>
        </div>
        <div class="field" style="margin-bottom:12px">
          <label for="mhoehe">Höhe</label>
          <div class="fc">
            <input
              id="mhoehe"
              class="input sm num"
              style="width:80px"
              inputmode="decimal"
              value={zahl(mono.hoehe ?? 18)}
              onchange={(ev) => {
                const v = lesen((ev.currentTarget as HTMLInputElement).value);
                if (v && v > 0) objektAendern((o) => ((o as Mono).hoehe = v));
              }}
            /><span class="unit">mm</span>
          </div>
        </div>
        {@render garnWahl()}
        {@render groesseDrehen()}
        {@render passung()}
      {/if}
    {:else if arbeit.schritt === 3}
      <h2 style="font-size:25px; margin:0 0 2px">Prüfen und Garn</h2>
      <p class="mut" style="margin:0 0 12px">Jede Farbe ist ein Farbstopp: die Maschine hält an, du wechselst das Garn.</p>
      <ol class="ol" aria-label="Farbstopps" style="border:1px solid var(--line); background:var(--color-bg); margin-bottom:14px">
        {#each bloecke as b (b.k)}
          <li style="display:grid; grid-template-columns:22px 22px minmax(0,1fr) auto; gap:8px; align-items:center; padding:8px 10px; border-bottom:1px solid color-mix(in srgb,var(--color-text) 7%,transparent)">
            <span class="mut tab">{b.k + 1}</span><span class="sw l" style="background:{b.garn?.rgb ?? b.farbe}"></span>
            <span style="min-width:0">
              <span style="display:block">{garnText(b.garn)}</span>
              <span class="mut small">{b.namen.join(", ")}{b.b?.von_hand ? " · von Hand gewählt" : ""}</span>
              {#each b.lagen as l (l)}<span class="small" style="display:block; color:var(--warn)">Applikation: {l}</span>{/each}
            </span>
            <button class="btn btn-secondary" style="min-height:28px; padding:2px 10px" disabled={!b.b} onclick={() => b.b && (arbeit.dialog = { art: "garn", objekt: b.b.objekt, titel: `Farbe ${b.k + 1} · ${b.namen[0] ?? ""}` })}>Garn ändern</button>
          </li>
        {:else}
          <li class="mut" style="padding:10px">Noch keine Stiche.</li>
        {/each}
      </ol>
      {#if e}
        <div class="kz" style="margin-bottom:14px">
          <div><span class="mut small">Stiche</span><b>{ganz(e.statistik.stiche)}</b></div>
          <div><span class="mut small">Farben</span><b>{e.farben.length}</b></div>
          <div><span class="mut small">Größe</span><b>{zahlFest(e.statistik.breite)} × {zahlFest(e.statistik.hoehe)} mm</b></div>
          <div title="geschätzt: 0,4 s je Stich (keine Quelle)"><span class="mut small">Stickzeit*</span><b>≈ {ganz((e.statistik.stiche * 0.4) / 60)} min</b></div>
        </div>
      {/if}
      <div style="display:flex; flex-direction:column; gap:6px; margin-bottom:14px">
        {@render passung()}
        {#each (e?.hinweise ?? []).filter((h) => h.art === "fehlende_zeichen") as h (h.meldung)}
          <div class="warnbox"><Icon d={IC.warnung} /><span>{h.meldung}</span></div>
        {/each}
      </div>
      <div class="note">
        <b style="font-weight:500">Für die Person an der Maschine</b>
        <ul style="margin:6px 0 0; padding-left:18px">
          <li>Stickgarn 40 oben, Stick-Unterfaden unten.</li>
          <li>Die artista 185 schneidet Verbindungsfäden nicht – nach dem Sticken von Hand schneiden.</li>
          <li>Bei jedem Farbstopp das Garn laut Liste oben wechseln.</li>
        </ul>
      </div>
    {:else}
      {#if istMac}
        <h2 style="font-size:25px; margin:0 0 2px">Speichern und exportieren</h2>
        <p class="mut" style="margin:0 0 12px">Auf diesem Mac ist keine Maschine angeschlossen. Das Design zum Sticken am Windows-PC speichern.</p>
      {:else}
        <h2 style="font-size:25px; margin:0 0 2px">An die Maschine senden</h2>
        <p class="mut" style="margin:0 0 12px">Erst wird nur gelesen und ein Plan gezeigt. Geschrieben wird erst nach deiner Bestätigung.</p>
        <div class="field" style="margin-bottom:10px">
          <label for="aport">Anschluss</label>
          <select id="aport" class="input" value={maschine.port} onchange={(e) => maschine.portSetzen((e.currentTarget as HTMLSelectElement).value)}>
            {#each maschine.ports as p (p)}<option value={p}>{p}</option>{/each}
            <option value={SIMULATION}>Simulation (ohne Maschine)</option>
          </select>
        </div>
        {#if maschine.simulation}
          <div class="simbox" style="margin-bottom:10px"><Icon d={IC.monitor} /><span><b>Simulation:</b> zum Ausprobieren ohne Maschine. Es wird nichts gesendet.</span></div>
        {/if}
        <button class="btn btn-secondary" style="width:100%; margin-bottom:8px" disabled={maschine.laeuft} onclick={() => maschine.pruefen()}>
          {maschine.laeuft ? "Prüfe …" : "Maschine prüfen (nur lesen, ≈ 4 s)"}
        </button>
        {#if maschine.info}
          <div class="okbox" style="margin-bottom:12px"><Icon d={IC.haken} /><span>Firmware {maschine.info.sitzung.naehmaschine} · Stickmodul erreicht · {maschine.info.farbbereich.text}</span></div>
        {:else if maschine.pruefFehler}
          <div class="errbox" style="margin-bottom:12px"><Icon d={IC.fehler} /><span>{maschine.pruefFehler.meldung}</span></div>
        {/if}
        <div class="note" style="margin-bottom:12px">
          <b style="font-weight:500">Nach einem Rahmenwechsel an der Maschine:</b>
          <ol style="margin:6px 0 0; padding-left:20px">{#each ANLEITUNG_RAHMEN as a (a)}<li style="margin-bottom:2px">{a}</li>{/each}</ol>
        </div>
        <button class="btn btn-primary blueprint btn-block" style="min-height:42px; font-size:16px; width:100%" disabled={!!maschine.sperrgrund} onclick={() => maschine.sendenOeffnen()}>
          <i class="corner tl"></i><i class="corner tr"></i><i class="corner bl"></i><i class="corner br"></i>Plan anzeigen …
        </button>
        {#if maschine.sperrgrund}<p class="small" style="color:var(--err); margin:6px 0 0">{maschine.sperrgrund}</p>{/if}
        <p class="mut small" style="margin:8px 0 14px">Das Design bleibt nur bis zum Ausschalten in der Maschine – dauerhaft speichern ist noch nicht möglich.</p>
      {/if}
      <div style="display:flex; flex-direction:column; gap:8px">
        <button class="big" onclick={() => speichern()}><b style="font-weight:500">Design speichern</b><span class="mut small">eigenes Format (JSON) – zum Weiterbearbeiten</span></button>
        <button class="big" onclick={expExportieren}><b style="font-weight:500">Stickdatei exportieren (EXP)</b><span class="mut small">Bernina-Format – Stiche ohne Garnnamen</span></button>
      </div>
      {#if istMac}<div class="note" style="margin-top:12px">Senden an die Maschine gibt es nur unter Windows mit Kabel (COM-Port).</div>{/if}
    {/if}
  </div>

  <div style="flex:none; display:flex; gap:8px; padding:10px 16px; border-top:1px solid var(--line); background:var(--color-bg)">
    {#if arbeit.schritt > 1}<button class="btn btn-secondary" onclick={() => arbeit.schritt--}>Zurück</button>{/if}
    <span style="flex:1"></span>
    {#if arbeit.schritt < 4}
      <button class="btn btn-primary" onclick={() => arbeit.schritt++}>{weiterText[arbeit.schritt - 1]}<Icon d={IC.chevronRechts} /></button>
    {/if}
  </div>
</aside>

{#snippet garnWahl()}
  <div class="field" style="margin-bottom:12px">
    <label for="agarn">Garnfarbe</label>
    <button id="agarn" class="btn btn-secondary pick" onclick={() => (arbeit.dialog = { art: "garn", objekt: haupt })} style="min-height:40px">
      <span class="sw l" style="background:{hauptGarn?.rgb ?? objekt?.farbe ?? '#000000'}"></span>
      <span class="t">{hauptGarn ? garnText(hauptGarn) : (objekt?.farbe ?? "")}</span>
      <Icon d={IC.chevronRechts} />
    </button>
  </div>
{/snippet}

{#snippet groesseDrehen()}
  <div style="display:grid; grid-template-columns:1fr 1fr auto; gap:8px; align-items:end; margin-bottom:12px">
    <div class="field">
      <label for="agr">Größe des Designs</label>
      <div class="fc">
        <input
          id="agr"
          class="input sm num"
          inputmode="decimal"
          value={zahl(skalierung)}
          onchange={(ev) => {
            const v = lesen((ev.currentTarget as HTMLInputElement).value);
            if (v && v > 0) transformAendern((t) => (t.skalieren = v));
          }}
        /><span class="unit">%</span>
      </div>
    </div>
    <div class="field">
      <label for="adr">Drehen</label>
      <div class="fc">
        <input
          id="adr"
          class="input sm num"
          inputmode="decimal"
          value={zahl(transform.drehen ?? 0)}
          onchange={(ev) => {
            const v = lesen((ev.currentTarget as HTMLInputElement).value);
            if (v !== null) transformAendern((t) => (t.drehen = v));
          }}
        /><span class="unit">°</span>
      </div>
    </div>
    <button class="btn btn-secondary" style="min-height:30px" onclick={mittig} disabled={!e?.begrenzung}>Mittig</button>
  </div>
{/snippet}

{#snippet passung()}
  {#if e && e.ausserhalb > 0}
    <div class="errbox" style="flex-direction:column">
      <span style="display:flex; gap:8px">
        <Icon d={IC.fehler} />
        <span><b style="font-weight:600">Passt nicht in den Rahmen.</b> {ganz(e.ausserhalb)} Einstiche liegen außerhalb des Stickfelds ({rahmen ? `${zahl(rahmen.breite)} × ${zahl(rahmen.hoehe)} mm` : ""}). So kann nicht gesendet werden.</span>
      </span>
      <span style="display:flex; gap:8px; margin-left:24px"><button class="btn btn-primary" onclick={einpassen}>Einpassen</button></span>
    </div>
  {:else if passt}
    <div class="okbox"><Icon d={IC.haken} /><span>Passt: {groesseText}</span></div>
  {/if}
{/snippet}
