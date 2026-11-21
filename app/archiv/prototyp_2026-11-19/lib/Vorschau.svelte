<script lang="ts">
  // Stichvorschau: Rahmen + Stiche je Farbe, Sprünge gestrichelt. Koordinaten in mm, +y nach oben.
  import type { Ergebnis } from "./typen";

  let {
    ergebnis,
    spruenge = true,
    einstiche = false,
  }: { ergebnis: Ergebnis | null; spruenge?: boolean; einstiche?: boolean } = $props();

  let huelle: HTMLDivElement;
  let canvas: HTMLCanvasElement;
  let groesse = $state({ b: 0, h: 0 });
  let zoom = $state(1); // Faktor auf „einpassen“
  let versatz = $state({ x: 0, y: 0 }); // Verschiebung in Bildpunkten
  let ziehen: { x: number; y: number } | null = null;

  $effect(() => {
    const ro = new ResizeObserver(([e]) => {
      groesse = { b: e.contentRect.width, h: e.contentRect.height };
    });
    ro.observe(huelle);
    return () => ro.disconnect();
  });

  // Abbildung mm -> Bildpunkte (eingepasst auf Rahmen + Stiche)
  let abbildung = $derived.by(() => {
    const { b, h } = groesse;
    let x0 = -50, x1 = 50, y0 = -50, y1 = 50;
    if (ergebnis) {
      [x0, x1, y0, y1] = [Infinity, -Infinity, Infinity, -Infinity];
      const punkte: [number, number][] = ergebnis.rahmen.form.concat(ergebnis.stiche.map((s) => [s.x, s.y]));
      for (const [x, y] of punkte) {
        x0 = Math.min(x0, x);
        x1 = Math.max(x1, x);
        y0 = Math.min(y0, y);
        y1 = Math.max(y1, y);
      }
    }
    const rand = 8;
    const oben = 48; // Platz für die Werkzeugleiste
    const s = Math.min(b / (x1 - x0 + 2 * rand), (h - oben) / (y1 - y0 + 2 * rand)) * zoom;
    const cx = (x0 + x1) / 2, cy = (y0 + y1) / 2;
    return {
      s,
      x: (x: number) => b / 2 + versatz.x + (x - cx) * s,
      y: (y: number) => oben + (h - oben) / 2 + versatz.y - (y - cy) * s,
    };
  });

  $effect(() => {
    const { b, h } = groesse;
    if (!canvas || b === 0) return;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.round(b * dpr);
    canvas.height = Math.round(h * dpr);
    const ctx = canvas.getContext("2d")!;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.fillStyle = "#e9ecef";
    ctx.fillRect(0, 0, b, h);
    if (!ergebnis) return;
    const { s, x: tx, y: ty } = abbildung;

    // Rahmen (Stickfeld) und Mittelkreuz
    ctx.beginPath();
    ergebnis.rahmen.form.forEach(([x, y], i) => (i ? ctx.lineTo(tx(x), ty(y)) : ctx.moveTo(tx(x), ty(y))));
    ctx.closePath();
    ctx.fillStyle = "#ffffff";
    ctx.fill();
    ctx.setLineDash([6, 4]);
    ctx.strokeStyle = "#8a96a3";
    ctx.lineWidth = 1;
    ctx.stroke();
    ctx.setLineDash([]);
    ctx.beginPath();
    ctx.moveTo(tx(-3), ty(0));
    ctx.lineTo(tx(3), ty(0));
    ctx.moveTo(tx(0), ty(-3));
    ctx.lineTo(tx(0), ty(3));
    ctx.stroke();

    // Stiche je Farbe; Sprünge getrennt sammeln
    const faden = Math.max(0.35 * s, 0.6);
    const sprungLinien: [number, number, number, number][] = [];
    let farbe = 0;
    let vorher: { x: number; y: number } | null = null;
    ctx.lineJoin = "round";
    ctx.lineCap = "round";
    ctx.lineWidth = faden;
    ctx.strokeStyle = ergebnis.farben[0] ?? "#000";
    ctx.beginPath();
    for (const st of ergebnis.stiche) {
      if (st.art === "farbwechsel") {
        ctx.stroke();
        farbe += 1;
        ctx.strokeStyle = ergebnis.farben[farbe] ?? "#000";
        ctx.beginPath();
        continue;
      }
      if (st.art === "sprung") {
        if (vorher) sprungLinien.push([tx(vorher.x), ty(vorher.y), tx(st.x), ty(st.y)]);
        ctx.moveTo(tx(st.x), ty(st.y));
      } else if (!vorher) {
        ctx.moveTo(tx(st.x), ty(st.y));
      } else {
        ctx.lineTo(tx(st.x), ty(st.y));
      }
      vorher = st;
    }
    ctx.stroke();

    if (spruenge && sprungLinien.length) {
      ctx.setLineDash([4, 4]);
      ctx.lineWidth = 1;
      ctx.strokeStyle = "rgba(90, 100, 110, 0.6)";
      ctx.beginPath();
      for (const [a, b2, c, d] of sprungLinien) {
        ctx.moveTo(a, b2);
        ctx.lineTo(c, d);
      }
      ctx.stroke();
      ctx.setLineDash([]);
    }
    if (einstiche) {
      ctx.fillStyle = "rgba(0, 0, 0, 0.75)";
      const r = Math.max(Math.min(0.12 * s, 2.5), 0.8);
      for (const st of ergebnis.stiche) {
        if (st.art !== "stich") continue;
        ctx.beginPath();
        ctx.arc(tx(st.x), ty(st.y), r, 0, 2 * Math.PI);
        ctx.fill();
      }
    }
  });

  function rad(e: WheelEvent) {
    e.preventDefault();
    const f = e.deltaY < 0 ? 1.2 : 1 / 1.2;
    const neu = Math.min(Math.max(zoom * f, 0.2), 60);
    const k = neu / zoom;
    const rect = canvas.getBoundingClientRect();
    const mx = e.clientX - rect.left - groesse.b / 2;
    const my = e.clientY - rect.top - (48 + (groesse.h - 48) / 2);
    versatz = { x: mx - (mx - versatz.x) * k, y: my - (my - versatz.y) * k };
    zoom = neu;
  }

  function runter(e: PointerEvent) {
    ziehen = { x: e.clientX - versatz.x, y: e.clientY - versatz.y };
    canvas.setPointerCapture(e.pointerId);
  }

  function bewegen(e: PointerEvent) {
    if (ziehen) versatz = { x: e.clientX - ziehen.x, y: e.clientY - ziehen.y };
  }

  export function einpassen() {
    zoom = 1;
    versatz = { x: 0, y: 0 };
  }
</script>

<div class="huelle" bind:this={huelle}>
  <canvas
    bind:this={canvas}
    onwheel={rad}
    onpointerdown={runter}
    onpointermove={bewegen}
    onpointerup={() => (ziehen = null)}
    ondblclick={einpassen}
  ></canvas>
</div>

<style>
  .huelle {
    position: absolute;
    inset: 0;
  }
  canvas {
    width: 100%;
    height: 100%;
    display: block;
    cursor: grab;
    touch-action: none;
  }
  canvas:active {
    cursor: grabbing;
  }
</style>
