// Felder je Objektart aus dem Schema des Kerns (`design_schema` = kern/schnittstelle/design.schema.json): Typ,
// Standard, Auswahlwerte, Beschreibung. Der Inspektor baut daraus seine Zeilen – neue Felder des Kerns erscheinen ohne
// Änderung an der Oberfläche (im Expertenmodus).

import { kern } from "$kern/kern";

type S = Record<string, unknown> & {
  $ref?: string;
  anyOf?: S[];
  oneOf?: S[];
  type?: string | string[];
  const?: string;
  description?: string;
  default?: unknown;
  properties?: Record<string, S>;
  items?: S;
  enum?: string[];
};

export type FeldTyp = "zahl" | "ganz" | "bool" | "text" | "auswahl" | "unterlage" | "komplex";
export type Auswahlwert = { wert: string; text: string };
export type Feld = {
  name: string;
  typ: FeldTyp;
  beschreibung: string;
  standard: unknown;
  werte: Auswahlwert[];
  einheit: string;
  /** darf leer sein (null = Standard des Stils) */
  leer: boolean;
};

/** Text einer Auswahl: Beschreibung; „Standard“ allein sagt nichts → „Wert (Standard)“ */
function auswahlText(wert: string, beschreibung?: string): string {
  if (!beschreibung) return wert;
  return beschreibung === "Standard" ? `${wert} (Standard)` : beschreibung;
}

class Schema {
  #s = $state.raw<S | null>(null);
  #laden: Promise<void> | null = null;

  laden() {
    this.#laden ??= kern("design_schema", {}).then((s) => {
      this.#s = s as S;
    });
    return this.#laden;
  }

  get bereit() {
    return this.#s !== null;
  }

  #def(ref: string): S {
    const name = ref.split("/").pop() ?? "";
    return ((this.#s?.$defs as Record<string, S>) ?? {})[name] ?? {};
  }

  #aufloesen(s: S): { s: S; leer: boolean } {
    let leer = false;
    let x = s;
    for (let i = 0; i < 4; i++) {
      if (x.$ref) {
        x = { ...this.#def(x.$ref), description: x.description ?? this.#def(x.$ref).description, default: x.default };
        continue;
      }
      if (x.anyOf && x.anyOf.length === 2 && x.anyOf.some((a) => a.type === "null")) {
        leer = true;
        const a = x.anyOf.find((a) => a.type !== "null")!;
        x = { ...a, description: x.description ?? a.description, default: x.default };
        continue;
      }
      break;
    }
    if (Array.isArray(x.type) && x.type.includes("null")) leer = true;
    return { s: x, leer };
  }

  /** Objektart → Definition (`TatamiJson` …) */
  #artDef(art: string): S | null {
    const oneOf = this.#def("#/$defs/ObjektJson").oneOf ?? [];
    const v = oneOf.find((o) => (o.properties?.art as S | undefined)?.const === art);
    return v?.$ref ? this.#def(v.$ref) : null;
  }

  /** Beschreibung der Objektart */
  artBeschreibung(art: string): string {
    return this.#artDef(art)?.description ?? "";
  }

  felder(art: string): Feld[] {
    const d = this.#artDef(art);
    if (!d?.properties) return [];
    return Object.entries(d.properties).map(([name, roh]) => this.#feld(name, roh));
  }

  feld(art: string, name: string): Feld | undefined {
    return this.felder(art).find((f) => f.name === name);
  }

  /** Feld der obersten Ebene des Designs (rahmen, stoff, start …) */
  designFeld(name: string): Feld | undefined {
    const p = (this.#s?.properties as Record<string, S> | undefined)?.[name];
    return p ? this.#feld(name, p) : undefined;
  }

  #feld(name: string, roh: S): Feld {
    const { s, leer } = this.#aufloesen(roh);
    const beschreibung = (roh.description ?? s.description ?? "").replace(/\n/g, " ");
    const einheit = /\bin mm\b|mm\)/.test(beschreibung)
      ? "mm"
      : /Grad|°/.test(beschreibung)
        ? "°"
        : /in %|%/.test(beschreibung)
          ? "%"
          : "";
    const basis = { name, beschreibung, standard: roh.default ?? s.default, einheit, leer, werte: [] as Auswahlwert[] };
    // Unterlage: "auto" | "keine" | [Arten]
    if (name === "unterlage" && s.anyOf) return { ...basis, typ: "unterlage", werte: this.#unterlageWerte() };
    // Auswahl: `oneOf` aus `const` (mit Beschreibung) und `enum` (Werte ohne Beschreibung, z. B. Ausrichtung links/rechts)
    const oneOf = s.oneOf ?? [];
    if (oneOf.length && oneOf.every((o) => o.const !== undefined || o.enum))
      return {
        ...basis,
        typ: "auswahl",
        werte: oneOf.flatMap((o) =>
          o.enum ? o.enum.map((e) => ({ wert: e, text: e })) : [{ wert: o.const!, text: auswahlText(o.const!, o.description) }],
        ),
      };
    if (s.enum) return { ...basis, typ: "auswahl", werte: s.enum.map((e) => ({ wert: e, text: e })) };
    const t = (Array.isArray(s.type) ? s.type.filter((x) => x !== "null") : [s.type])[0];
    if (t === "boolean") return { ...basis, typ: "bool" };
    if (t === "integer") return { ...basis, typ: "ganz" };
    if (t === "number") return { ...basis, typ: "zahl" };
    if (t === "string") return { ...basis, typ: "text" };
    return { ...basis, typ: "komplex" };
  }

  #unterlageWerte(): Auswahlwert[] {
    const w = (this.#def("#/$defs/UnterlageWahl").oneOf ?? []).map((o) => ({ wert: o.const!, text: o.description ?? o.const! }));
    const a = (this.#def("#/$defs/UnterlageArt").oneOf ?? []).map((o) => ({ wert: `[${o.const}]`, text: `nur ${o.description ?? o.const}` }));
    return [...w, ...a];
  }
}

export const schema = new Schema();
