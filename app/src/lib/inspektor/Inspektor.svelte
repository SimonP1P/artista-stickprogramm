<script lang="ts">
  // Inspektor (Expertenmodus, UEBERGABE §2/§3): Reiter Objekt · Design · Einzelwerte · JSON. Felder und Auswahlwerte aus
  // dem Schema des Kerns; welche Felder im einfachen Teil stehen, legt die Liste E_FELDER fest (UEBERGABE §3 „E“),
  // alles Weitere erscheint unter „Stichart – Feinheiten“ mit der Marke X.
  import type { ObjektJson, Transformation } from "$kern/kern";
  import { arbeit, WECHSELBAR } from "../arbeit.svelte";
  import { FUELLUNG, STICHART_NAME, UMRISS } from "../einfuegen";
  import Icon from "../Icon.svelte";
  import { ART, IC } from "../icons";
  import { garnText, listen, STOFF_TEXT } from "../listen.svelte";
  import { schema, type Feld } from "../schema.svelte";
  import { ganz, lesen, zahl, zahlFest } from "../zahl";
  import Feldzeile from "./Feldzeile.svelte";
  import Abschnitt from "./Abschnitt.svelte";
  import Einzelwerte from "./Einzelwerte.svelte";
  import JsonReiter from "./JsonReiter.svelte";
  import Buchstaben from "./Buchstaben.svelte";

  type Reiter = "objekt" | "design" | "einzelwerte" | "json";
  // Startreiter: Startoption `reiter` (Bildschirmfotos), sonst „Objekt“
  let reiter = $state<Reiter>((globalThis as { __START__?: { reiter?: Reiter } }).__START__?.reiter ?? "objekt");

  $effect(() => {
    schema.laden();
    listen.laden();
  });

  /** Felder, die im oberen Abschnitt stehen (Modus E laut UEBERGABE §3 / FUNKTIONEN.md) */
  const E_FELDER: Record<string, string[]> = {
    text: ["text", "alphabet", "hoehe", "ausrichtung", "kursiv", "grundlinie", "unterlage"],
    ttf: ["text", "schrift", "hoehe", "ausrichtung", "kursiv", "grundlinie", "unterlage"],
    monogramm: ["initialen", "hoehe", "rahmen", "rahmen_farbe", "unterlage"],
    lauf: ["geschlossen"],
    dreifach: ["geschlossen"],
    satin: ["stichart"],
    satinlinie: ["breite", "stichart"],
    blanket: ["geschlossen"],
    tatami: ["winkel", "step_muster", "aussparen"],
    satinflaeche: ["stichart"],
    muster: ["muster", "winkel", "aussparen"],
    applikation: ["abdeckung", "stil"],
    knopfloch: ["typ", "laenge", "winkel"],
    motivlinie: ["motiv", "groesse", "spiegeln", "geschlossen"],
  };
  /** an anderer Stelle bearbeitet */
  const GESONDERT = new Set(["satz", "art", "name", "farbe", "garn", "transform", "einstellungen", "einstich", "ausstich", "doppellauf", "auto_jump", "buchstaben"]);
  const SCHRIFT = new Set(["text", "ttf", "monogramm"]);
  const MIT_STEP = new Set(["tatami", "text", "ttf", "monogramm"]);

  const i = $derived(arbeit.auswahl.length === 1 ? arbeit.auswahl[0] : -1);
  const o = $derived(i >= 0 ? (arbeit.objekte[i] as ObjektJson & Record<string, unknown>) : undefined);
  const felder = $derived(o && schema.bereit ? schema.felder(o.art) : []);
  const eNamen = $derived(o ? (E_FELDER[o.art] ?? []) : []);
  const oben = $derived(eNamen.map((n) => felder.find((f) => f.name === n)).filter((f): f is Feld => !!f));
  const fein = $derived(
    felder.filter(
      (f) =>
        !eNamen.includes(f.name) &&
        !GESONDERT.has(f.name) &&
        f.typ !== "komplex" &&
        (f.name !== "step_muster" || MIT_STEP.has(o?.art ?? "")),
    ),
  );
  const formFelder = $derived(felder.filter((f) => f.typ === "komplex" && !GESONDERT.has(f.name) && !eNamen.includes(f.name)));

  const e = $derived(arbeit.ergebnis);
  const garn = $derived(e ? arbeit.garnVon(i) : undefined);

  function setzen(name: string, v: unknown, gruppe?: string) {
    const nr = i;
    arbeit.aendern((d) => {
      const x = d.objekte![nr] as Record<string, unknown>;
      if (v === undefined) delete x[name];
      else x[name] = v;
    }, gruppe ? `${gruppe}:${nr}` : undefined);
  }
  function transformSetzen(f: (t: Transformation) => void, ganzesDesign = false) {
    const nr = i;
    arbeit.aendern((d) => {
      const ziel = ganzesDesign ? d : (d.objekte![nr] as { transform?: Transformation | null });
      ziel.transform ??= {};
      f(ziel.transform!);
    });
  }
  const skal = (t: Transformation | null | undefined) =>
    Array.isArray(t?.skalieren) ? (t.skalieren[0] ?? 100) : (t?.skalieren ?? 100);

  // Grundlinie (einfach): gerade / Bogen oben / Bogen unten
  const form = $derived.by(() => {
    const g = o?.grundlinie as { art?: string; lage?: string } | undefined;
    return !g ? "gerade" : g.art === "kreis" ? (g.lage === "unten" ? "unten" : "oben") : "andere";
  });
  function formSetzen(f: string) {
    const g = o?.grundlinie as { radius?: number } | undefined;
    setzen("grundlinie", f === "gerade" ? undefined : { art: "kreis", lage: f, radius: g?.radius ?? 50 });
  }

  const andereNamen = $derived(arbeit.objekte.map((x, k) => ({ k, name: x.name ?? "" })).filter((x) => x.k !== i));
  function aussparenUmschalten(name: string, an: boolean) {
    const alt = (o?.aussparen as string[] | undefined) ?? [];
    const neu = an ? [...alt, name] : alt.filter((n) => n !== name);
    setzen("aussparen", neu.length ? neu : undefined);
  }

  $effect(() => {
    if (o?.art === "muster") listen.musterLaden();
    if (o?.art === "ttf") listen.schriftenLaden();
  });
</script>

<aside class="pane" aria-label="Inspektor" style="width:340px; border-left:1px solid var(--line)">
  <div class="itabs" role="tablist">
    {#each [["objekt", "Objekt"], ["design", "Design"], ["einzelwerte", "Einzelwerte"], ["json", "JSON"]] as [w, l] (w)}
      <button class="itab" role="tab" aria-selected={reiter === w} onclick={() => (reiter = w as Reiter)}>{l}</button>
    {/each}
  </div>
  <div class="scroll">
    {#if reiter === "objekt"}
      {#if !o}
        <div style="padding:24px 18px; text-align:center" class="mut">
          <Icon d={IC.zeiger} groesse="l" />
          <p style="margin:6px 0 0">
            {arbeit.auswahl.length > 1 ? `${arbeit.auswahl.length} Objekte gewählt – Duplizieren, Löschen und Anordnen gelten für alle.` : "Kein Objekt gewählt. Objekt in der Vorschau oder in der Liste anklicken."}
          </p>
          {#if arbeit.auswahlGruppe}
            {@const g = arbeit.auswahlGruppe}
            <p style="margin:10px 0 0; color:var(--color-text)">
              Gruppe „{arbeit.design.gruppen?.[g]?.name || g}“: Verschieben, Größe und Drehen gelten für die ganze Gruppe.
            </p>
            <p class="small" style="margin:4px 0 8px">Einzelnes Objekt bearbeiten: in der Objektliste anklicken.</p>
            <button class="btn btn-secondary" onclick={() => arbeit.gruppeAufloesen()}>Gruppe auflösen</button>
          {:else if arbeit.auswahl.length > 1}
            <button class="btn btn-secondary" style="margin-top:10px" onclick={() => arbeit.gruppieren()}>Gruppieren</button>
          {/if}
          <p class="small" style="margin:10px 0 0">Rahmen, Stoff, Garnkarte und das ganze Design: Reiter „Design“.</p>
        </div>
      {:else}
        <div class="frow" style="padding-top:10px">
          <span class="fl tab">Nr. {i + 1}</span>
          <span class="fc"><Icon d={ART[o.art]?.[1] ?? ""} />{ART[o.art]?.[0] ?? o.art}</span>
        </div>
        {#if WECHSELBAR.includes(o.art)}
          <!-- Stichart wechseln wie „Outline/Fill“ + Liste im Original (Handbuch S. 63/64); Form bleibt (Kern) -->
          <div class="frow">
            <label class="fl" for="istichart">Stichart</label>
            <div class="fc">
              <select id="istichart" class="input sm" value={o.art} onchange={(ev) => arbeit.stichartAendern([i], (ev.currentTarget as HTMLSelectElement).value)}>
                <optgroup label="Füllung (Fläche)">
                  {#each FUELLUNG as a (a)}<option value={a}>{STICHART_NAME[a]} – Fläche</option>{/each}
                </optgroup>
                <optgroup label="Umriss (Linie)">
                  {#each UMRISS as a (a)}<option value={a}>{STICHART_NAME[a]} – Linie</option>{/each}
                </optgroup>
              </select>
            </div>
          </div>
          {#if (FUELLUNG as readonly string[]).includes(o.art)}
            <div class="frow">
              <span class="fl"></span>
              <div class="fc">
                <button class="btn btn-secondary" style="min-height:28px" title="Legt eine Linie mit derselben Form dahinter an ({STICHART_NAME[arbeit.vorwahl.umriss]}, wie die Umriss-Vorwahl) und gruppiert beide – im Original zwei Objekte von Hand" onclick={() => arbeit.umrandungDazu(i)}>
                  <Icon d={IC.plus} />Umrandung dazu
                </button>
              </div>
            </div>
          {/if}
        {/if}
        <div class="frow">
          <label class="fl" for="iname">Name</label>
          <div class="fc"><input id="iname" class="input sm" value={o.name ?? ""} onchange={(ev) => setzen("name", (ev.currentTarget as HTMLInputElement).value || undefined)} /></div>
        </div>

        <Abschnitt titel={ART[o.art]?.[0] ?? o.art}>
          <div class="frow">
            <span class="fl">Garn</span>
            <div class="fc">
              <button class="btn btn-secondary pick" onclick={() => (arbeit.dialog = { art: "garn", objekt: i })}>
                <span class="sw" style="background:{garn?.rgb ?? o.farbe ?? '#000'}"></span><span class="t">{garnText(garn)}</span><Icon d={IC.chevronRechts} />
              </button>
            </div>
            <div class="fhelp">{o.garn ? "von Hand gewählt" : `nächstes Garn zu ${o.farbe ?? "#000000"}`}</div>
          </div>
          {#each oben as f (f.name)}
            {#if f.name === "alphabet"}
              <div class="frow">
                <span class="fl">Schrift</span>
                <div class="fc"><button class="btn btn-secondary pick" onclick={() => (arbeit.dialog = { art: "schrift", objekt: i })}><span class="t">{String(o.alphabet ?? "")}</span><Icon d={IC.chevronRechts} /></button></div>
              </div>
            {:else if f.name === "text"}
              <div class="frow">
                <label class="fl" for="itext">Text</label>
                <div class="fc"><textarea id="itext" class="input sm" rows="2" style="width:100%" value={String(o.text ?? "")} oninput={(ev) => setzen("text", (ev.currentTarget as HTMLTextAreaElement).value, "text")}></textarea></div>
                <div class="fhelp">neue Zeile mit Eingabe, Farbwechsel mit ^</div>
              </div>
            {:else if f.name === "grundlinie"}
              <div class="frow">
                <span class="fl">Grundlinie</span>
                <div class="fc">
                  <div class="seg seg-sm" role="radiogroup" aria-label="Grundlinie" style="display:flex; width:100%">
                    {#each [["gerade", "gerade"], ["oben", "Bogen ↑"], ["unten", "Bogen ↓"]] as [w, l] (w)}
                      <label class="seg-opt" style="flex:1; justify-content:center; padding-inline:4px"><input type="radio" name="igl" checked={form === w} onchange={() => formSetzen(w)} />{l}</label>
                    {/each}
                  </div>
                </div>
                {#if form === "andere"}<div class="fhelp">eigene Grundlinie – Feinwerte im Reiter JSON</div>{/if}
              </div>
            {:else if f.name === "schrift"}
              <div class="frow">
                <span class="fl">Schrift</span>
                <div class="fc"><button class="btn btn-secondary pick" onclick={() => (arbeit.dialog = { art: "schrift", objekt: i })}><span class="t">{String(o.schrift ?? "")}</span><Icon d={IC.chevronRechts} /></button></div>
              </div>
            {:else if f.name === "muster"}
              <div class="frow">
                <span class="fl">Füllmuster</span>
                <div class="fc"><button class="btn btn-secondary pick" onclick={() => (arbeit.dialog = { art: "muster", objekt: i })}><span class="t">{String(o.muster ?? "")}</span><Icon d={IC.chevronRechts} /></button></div>
              </div>
            {:else if f.name === "motiv"}
              <div class="frow">
                <span class="fl">Motiv</span>
                <div class="fc"><button class="btn btn-secondary pick" onclick={() => (arbeit.dialog = { art: "motiv", objekt: i })}><span class="t">{String(o.satz ?? "Heirloom").replace(/^_/, "")} · {String(o.motiv ?? "705")}</span><Icon d={IC.chevronRechts} /></button></div>
              </div>
            {:else if f.name === "step_muster"}
              <div class="frow">
                <span class="fl">Step-Muster</span>
                <div class="fc"><button class="btn btn-secondary pick" onclick={() => (arbeit.dialog = { art: "step", objekt: i })}><span class="t">{o.step_muster == null ? "wie Stoff/Stil" : `Nr. ${o.step_muster}`}</span><Icon d={IC.chevronRechts} /></button></div>
                <div class="fhelp">1 = gleichmäßig</div>
              </div>
            {:else if f.name === "aussparen"}
              <div class="frow" style="align-items:start">
                <span class="fl">Aussparen</span>
                <div class="fc" style="flex-direction:column; align-items:flex-start; gap:2px">
                  {#each andereNamen.filter((x) => x.name) as x (x.k)}
                    <label class="small" style="display:flex; gap:6px; align-items:center"><input type="checkbox" checked={((o.aussparen as string[] | undefined) ?? []).includes(x.name)} onchange={(ev) => aussparenUmschalten(x.name, (ev.currentTarget as HTMLInputElement).checked)} />Nr. {x.k + 1} {x.name}</label>
                  {:else}
                    <span class="mut small">– keine Objekte mit Namen –</span>
                  {/each}
                </div>
                <div class="fhelp">Fläche spart die gewählten Objekte aus (Knockout)</div>
              </div>
            {:else}
              <Feldzeile feld={f} wert={o[f.name]} beschriftung={o.art === "motivlinie" && f.name === "groesse" ? "Motivgröße" : undefined} haken={SCHRIFT.has(o.art)} setzen={(v) => setzen(f.name, v)} />
            {/if}
          {/each}
        </Abschnitt>

        {#if fein.length}
          <Abschnitt titel="Stichart – Feinheiten" offen={false}>
            {#each fein as f (f.name)}<Feldzeile feld={f} wert={o[f.name]} haken={SCHRIFT.has(o.art)} setzen={(v) => setzen(f.name, v)} x />{/each}
          </Abschnitt>
        {/if}

        {#if SCHRIFT.has(o.art)}
          <Abschnitt titel="Einzelne Buchstaben" rechts={(o.buchstaben as unknown[] | undefined)?.length ? `${(o.buchstaben as unknown[]).length} geändert` : ""} offen={arbeit.buchstabe?.objekt === i}>
            <Buchstaben nr={i} />
          </Abschnitt>
        {/if}

        <Abschnitt titel="Größe und Lage">
          {@render transformZeilen(o.transform as Transformation | undefined, false)}
        </Abschnitt>

        <Abschnitt titel="Bearbeiten" offen={false}>
          <div class="frow">
            <label class="fl" for="idopp">Verstärken<span class="xt">X</span></label>
            <div class="fc"><input id="idopp" type="checkbox" checked={o.doppellauf === true} onchange={(ev) => setzen("doppellauf", (ev.currentTarget as HTMLInputElement).checked || undefined)} style="width:16px; height:16px" /> Double Run</div>
          </div>
          <div class="frow">
            <label class="fl" for="iaj">Auto Jump<span class="xt">X</span></label>
            <div class="fc">
              <select id="iaj" class="input sm" value={o.auto_jump === true ? "an" : o.auto_jump === false ? "aus" : typeof o.auto_jump === "object" && o.auto_jump ? "werte" : ""} onchange={(ev) => {
                const v = (ev.currentTarget as HTMLSelectElement).value;
                setzen("auto_jump", v === "" ? undefined : v === "werte" ? { max: 12.7, sprung: 7 } : v === "an");
              }}>
                <option value="">wie Stil</option><option value="an">an (12,7 / 7 mm)</option><option value="aus">aus</option><option value="werte">eigene Werte</option>
              </select>
            </div>
            {#if typeof o.auto_jump === "object" && o.auto_jump}
              {@const aj = o.auto_jump as { max?: number; sprung?: number }}
              <div class="fhelp" style="display:flex; gap:6px; align-items:center">
                max <input class="input sm num" style="width:60px" value={zahl(aj.max ?? 12.7)} onchange={(ev) => setzen("auto_jump", { ...aj, max: lesen((ev.currentTarget as HTMLInputElement).value) ?? 12.7 })} />
                Sprung <input class="input sm num" style="width:60px" value={zahl(aj.sprung ?? 7)} onchange={(ev) => setzen("auto_jump", { ...aj, sprung: lesen((ev.currentTarget as HTMLInputElement).value) ?? 7 })} /> mm
              </div>
            {/if}
          </div>
          {#each [["einstich", "Einstich"], ["ausstich", "Ausstich"]] as [k, l] (k)}
            {@const p = o[k] as number[] | null | undefined}
            <div class="frow">
              <span class="fl">{l}<span class="xt">X</span></span>
              <div class="fc">
                {#if p}<span class="tab">{zahlFest(p[0])} / {zahlFest(p[1])} mm</span><button class="btn btn-ghost" style="font-size:12px" onclick={() => setzen(k, undefined)}>entfernen</button>
                {:else}<span class="mut small">automatisch (Griff in der Formansicht: E5)</span>{/if}
              </div>
            </div>
          {/each}
        </Abschnitt>

        {#if formFelder.length}
          <div class="small mut" style="padding:10px 14px">
            Form: {formFelder.map((f) => f.name).join(", ")} – in der Formansicht bzw. im Reiter JSON.
          </div>
        {/if}
      {/if}
    {:else if reiter === "design"}
      <Abschnitt titel="Rahmen, Stoff, Garn">
        <div class="frow">
          <label class="fl" for="drahmen">Rahmen</label>
          <div class="fc"><select id="drahmen" class="input sm" value={arbeit.design.rahmen ?? ""} onchange={(ev) => arbeit.aendern((d) => (d.rahmen = (ev.currentTarget as HTMLSelectElement).value as typeof d.rahmen))}>
            {#each arbeit.rahmenListe as r (r.name)}<option value={r.name}>{r.name}</option>{/each}
          </select></div>
          <div class="fhelp">{arbeit.rahmenListe.find((r) => r.name === arbeit.design.rahmen)?.hinweis ?? ""}</div>
        </div>
        <div class="frow">
          <label class="fl" for="dstoff">Stoff</label>
          <div class="fc"><select id="dstoff" class="input sm" value={arbeit.design.stoff ?? "0 Software Defaults"} onchange={(ev) => arbeit.aendern((d) => (d.stoff = (ev.currentTarget as HTMLSelectElement).value as typeof d.stoff))}>
            {#each listen.stoffe as s (s.name)}<option value={s.name}>{s.name} – {STOFF_TEXT[s.name]?.de ?? ""}</option>{/each}
          </select></div>
          <div class="fhelp">ändert Dichte, Unterlage, Zugausgleich</div>
        </div>
        <div class="frow">
          <label class="fl" for="dkarte">Garnkarte</label>
          <div class="fc"><select id="dkarte" class="input sm" value={arbeit.design.garnkarte ?? "Default"} onchange={(ev) => arbeit.aendern((d) => (d.garnkarte = (ev.currentTarget as HTMLSelectElement).value))}>
            {#each listen.garnkarten as k (k)}<option value={k}>{k}</option>{/each}
            <option value="keine">keine</option>
          </select></div>
          <div class="fhelp">jede Farbe → nächstes Garn der Karte</div>
        </div>
      </Abschnitt>
      <Abschnitt titel="Ganzes Design" rechts={e ? `${zahlFest(e.statistik.breite)} × ${zahlFest(e.statistik.hoehe)} mm` : ""}>
        {@render transformZeilen(arbeit.design.transform ?? undefined, true)}
        {#each ["start", "ende"] as n (n)}
          {@const f = schema.designFeld(n)}
          {#if f}
            <Feldzeile feld={f} wert={(arbeit.design as Record<string, unknown>)[n]} beschriftung={n === "start" ? "Startpunkt" : "Endpunkt"} x setzen={(v) => arbeit.aendern((d) => {
              const x = d as Record<string, unknown>;
              if (v === undefined) delete x[n];
              else x[n] = v;
            })} />
          {/if}
        {/each}
        <div class="frow">
          <label class="fl" for="dkv">Kürzeste Verbindung<span class="xt">X</span></label>
          <div class="fc"><input id="dkv" type="checkbox" checked={arbeit.design.kuerzeste_verbindung === true} onchange={(ev) => arbeit.aendern((d) => (d.kuerzeste_verbindung = (ev.currentTarget as HTMLInputElement).checked))} style="width:16px; height:16px" /> Closest Join</div>
        </div>
      </Abschnitt>
      {#if e}
        <Abschnitt titel="Kennzahlen">
          {#each [["Stiche", ganz(e.statistik.stiche)], ["Sprünge", ganz(e.statistik.spruenge)], ["Farbwechsel", ganz(e.statistik.farbwechsel)], ["Stichlänge", `${zahl(e.statistik.min_stich)} – ${zahl(e.statistik.max_stich)} mm`], ["Einstiche außerhalb", ganz(e.ausserhalb)], ["Stickzeit", `≈ ${ganz((e.statistik.stiche * 0.4) / 60)} min (Schätzung, 0,4 s je Stich)`], ["Datengröße", `≈ ${ganz(e.daten_bytes / 1024)} KB von ≈ ${ganz(e.daten_grenze / 1024)} KB`]] as [l, w] (l)}
            <div class="frow"><span class="fl">{l}</span><span class="fc tab">{w}</span></div>
          {/each}
        </Abschnitt>
        <Abschnitt titel="Farbstopps und Garne">
          {#each e.farben as f, k (k)}
            {@const g = e.garne[k]}
            {@const b = e.bloecke[k]}
            <div class="frow">
              <span class="fl tab">{k + 1}</span>
              <div class="fc">
                <span class="sw" style="background:{g?.rgb ?? f}"></span><span class="small" style="flex:1">{garnText(g)}{b?.von_hand ? " · von Hand" : ""}</span>
                {#if b}<button class="btn btn-ghost" style="font-size:12px" onclick={() => (arbeit.dialog = { art: "garn", objekt: b.objekt })}>ändern</button>{/if}
              </div>
            </div>
          {/each}
        </Abschnitt>
      {/if}
    {:else if reiter === "einzelwerte"}
      <Einzelwerte />
    {:else}
      <JsonReiter />
    {/if}
  </div>
</aside>

{#snippet transformZeilen(t: Transformation | undefined, ganzesDesign: boolean)}
  <div class="frow">
    <label class="fl" for="tg-{ganzesDesign}">Größe</label>
    <div class="fc"><input id="tg-{ganzesDesign}" class="input sm num" style="width:90px" value={zahl(skal(t))} onchange={(ev) => {
      const v = lesen((ev.currentTarget as HTMLInputElement).value);
      if (v && v > 0) transformSetzen((x) => (x.skalieren = v), ganzesDesign);
    }} /><span class="unit">%</span></div>
    <div class="fhelp">Stiche werden neu berechnet, nie skaliert</div>
  </div>
  <div class="frow">
    <label class="fl" for="td-{ganzesDesign}">Drehen</label>
    <div class="fc"><input id="td-{ganzesDesign}" class="input sm num" style="width:90px" value={zahl(t?.drehen ?? 0)} onchange={(ev) => {
      const v = lesen((ev.currentTarget as HTMLInputElement).value);
      if (v !== null) transformSetzen((x) => (x.drehen = v), ganzesDesign);
    }} /><span class="unit">°</span></div>
  </div>
  <div class="frow">
    <span class="fl">Spiegeln</span>
    <div class="fc">
      <div class="seg seg-sm" role="radiogroup" aria-label="Spiegeln">
        {#each [["", "nein"], ["links_rechts", "↔"], ["oben_unten", "↕"], ["beide", "beide"]] as [w, l] (w)}
          <label class="seg-opt"><input type="radio" name="sp-{ganzesDesign}" checked={(t?.spiegeln ?? "") === w} onchange={() => transformSetzen((x) => {
            if (w) x.spiegeln = w as Transformation["spiegeln"];
            else delete x.spiegeln;
          }, ganzesDesign)} />{l}</label>
        {/each}
      </div>
    </div>
  </div>
  <div class="frow">
    <label class="fl" for="ts-{ganzesDesign}">Schrägstellen<span class="xt">X</span></label>
    <div class="fc"><input id="ts-{ganzesDesign}" class="input sm num" style="width:90px" value={zahl(t?.schraeg ?? 0)} onchange={(ev) => {
      const v = lesen((ev.currentTarget as HTMLInputElement).value);
      if (v !== null) transformSetzen((x) => (x.schraeg = v), ganzesDesign);
    }} /><span class="unit">°</span></div>
  </div>
  <div class="frow">
    <span class="fl">Verschieben</span>
    <div class="fc">
      {#each [0, 1] as achse (achse)}
        <input class="input sm num" style="width:64px" aria-label={achse ? "y" : "x"} value={zahl(t?.verschieben?.[achse] ?? 0)} onchange={(ev) => {
          const v = lesen((ev.currentTarget as HTMLInputElement).value);
          if (v !== null) transformSetzen((x) => {
            const p = [...(x.verschieben ?? [0, 0])] as [number, number];
            p[achse] = v;
            x.verschieben = p;
          }, ganzesDesign);
        }} />
      {/each}
      <span class="unit">mm</span>
    </div>
  </div>
{/snippet}
