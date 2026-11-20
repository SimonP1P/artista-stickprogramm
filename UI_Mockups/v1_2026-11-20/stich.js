// Stichvorschau-Engine für die Mockups. Koordinaten in mm, Ursprung = Rahmenmitte, +y nach unten (Bildschirm).
// Geometrie ist vereinfacht (nur Darstellung) – nicht die Stichberechnung des Rust-Kerns.
const TAU = Math.PI * 2;
const rot = (p, a) => { const c = Math.cos(a), s = Math.sin(a); return [p[0] * c - p[1] * s, p[0] * s + p[1] * c]; };
const circ = (cx, cy, rx, ry, n = 56, a0 = 0) => { const o = []; for (let i = 0; i < n; i++) { const a = a0 + i / n * TAU; o.push([cx + Math.cos(a) * rx, cy + Math.sin(a) * ry]); } return o; };
const xf = (pts, { x = 0, y = 0, s = 1, r = 0 } = {}) => pts.map(p => { const q = rot([p[0] * s, p[1] * s], r * Math.PI / 180); return [q[0] + x, q[1] + y]; });
function heart() { const o = []; for (let i = 0; i < 72; i++) { const t = i / 72 * TAU; o.push([16 * Math.sin(t) ** 3 / 33, (-(13 * Math.cos(t) - 5 * Math.cos(2 * t) - 2 * Math.cos(3 * t) - Math.cos(4 * t)) + 2.5) / 33]); } return o; }
function star(n = 5, ro = 0.5, ri = 0.22) { const o = []; for (let i = 0; i < n * 2; i++) { const a = -Math.PI / 2 + i / (n * 2) * TAU, r = i % 2 ? ri : ro; o.push([Math.cos(a) * r, Math.sin(a) * r + 0.03]); } return o; }
function leaf(len = 1, wid = 0.42) { const top = [], bot = []; for (let i = 0; i <= 24; i++) { const t = i / 24, x = (t - 0.5) * len, y = Math.sin(Math.PI * t) ** 0.85 * wid / 2; top.push([x, -y]); bot.push([x, y]); } return top.concat(bot.reverse().slice(1, -1)); }
function shield() { const o = [[-0.42, -0.46], [0.42, -0.46], [0.42, 0.04]]; for (let i = 1; i <= 12; i++) { const t = i / 12, a = [0.42, 0.04], c = [0.42, 0.34], b = [0, 0.5]; o.push([(1 - t) ** 2 * a[0] + 2 * (1 - t) * t * c[0] + t * t * b[0], (1 - t) ** 2 * a[1] + 2 * (1 - t) * t * c[1] + t * t * b[1]]); } const m = o.slice(3, -1).reverse().map(p => [-p[0], p[1]]); return o.concat(m, [[-0.42, 0.04]]); }
function drop() { const o = [[0, -0.5]]; for (let i = 0; i <= 30; i++) { const a = (-50 + i / 30 * 280) * Math.PI / 180; o.push([Math.cos(a) * 0.34, 0.14 + Math.sin(a) * 0.34]); } return o; }
function band(ph = 0, th = 0.09) { const top = [], bot = []; for (let i = 0; i <= 60; i++) { const x = -0.5 + i / 60, y = Math.sin((x * 2 + ph) * TAU) * 0.22; top.push([x, y - th]); bot.push([x, y + th]); } return top.concat(bot.reverse()); }
function wave(ph = 0.25) { const o = []; for (let i = 0; i <= 60; i++) { const x = -0.5 + i / 60; o.push([x, Math.sin((x * 2 + ph) * TAU) * 0.3]); } return o; }

export const MOTIVE = [
  { id: 'bluete', name: 'Blüte', kat: 'Natur', w: 70, h: 70, farben: ['#d94f70', '#f2c14e'], parts: () => [...[0, 1, 2, 3, 4, 5].map(i => ({ pts: xf(circ(0, 0, 0.2, 0.115, 36), { x: Math.cos(i / 6 * TAU) * 0.28, y: Math.sin(i / 6 * TAU) * 0.28, r: i * 60 }), c: 0, ang: i * 60 + 90 })), { pts: circ(0, 0, 0.14, 0.14, 40), c: 1 }] },
  { id: 'herz', name: 'Herz', kat: 'Formen', w: 64, h: 58, farben: ['#c8102e', '#d94f70'], parts: () => [{ pts: heart(), c: 0 }, { pts: heart(), c: 1, kind: 'outline' }] },
  { id: 'stern', name: 'Stern', kat: 'Formen', w: 66, h: 66, farben: ['#f2c14e', '#c8a24a'], parts: () => [{ pts: star(), c: 0 }, { pts: star(), c: 1, kind: 'outline' }] },
  { id: 'blatt', name: 'Blatt', kat: 'Natur', w: 74, h: 74, farben: ['#4f8a3a', '#2d5a22'], parts: () => [{ pts: xf(leaf(), { r: -40 }), c: 0, ang: 50 }, { pts: xf([[-0.5, 0], [0.4, 0]], { r: -40 }), c: 1, kind: 'line' }] },
  { id: 'wappen', name: 'Wappen', kat: 'Embleme', w: 60, h: 70, farben: ['#1f3a6b', '#c8a24a', '#f7f6f2'], parts: () => [{ pts: shield(), c: 0 }, { pts: shield(), c: 1, kind: 'outline', w: 2.6 }, { pts: xf(star(), { s: 0.5, y: -0.04 }), c: 2 }] },
  { id: 'abzeichen', name: 'Rundes Abzeichen', kat: 'Embleme', w: 70, h: 70, farben: ['#2d5a22', '#e8e1c8', '#c8a24a'], parts: () => [{ pts: circ(0, 0, 0.47, 0.47, 72), c: 0 }, { pts: circ(0, 0, 0.27, 0.27, 56), c: 1, ang: 135 }, { pts: circ(0, 0, 0.47, 0.47, 72), c: 2, kind: 'outline', w: 2.8 }, { pts: xf(star(5, 0.5, 0.24), { s: 0.34 }), c: 2 }] },
  { id: 'krone', name: 'Krone', kat: 'Embleme', w: 72, h: 50, farben: ['#c8a24a', '#5a3e2b'], parts: () => [{ pts: [[-0.5, 0.36], [-0.5, -0.36], [-0.25, -0.02], [0, -0.48], [0.25, -0.02], [0.5, -0.36], [0.5, 0.36]], c: 0 }, { pts: [[-0.5, 0.36], [-0.5, -0.36], [-0.25, -0.02], [0, -0.48], [0.25, -0.02], [0.5, -0.36], [0.5, 0.36]], c: 1, kind: 'outline' }] },
  { id: 'tropfen', name: 'Tropfen', kat: 'Formen', w: 46, h: 66, farben: ['#3a86c8', '#1f3a6b'], parts: () => [{ pts: drop(), c: 0, ang: 0 }, { pts: drop(), c: 1, kind: 'outline' }] },
  { id: 'wabe', name: 'Wabe', kat: 'Formen', w: 66, h: 60, farben: ['#f2c14e', '#5a3e2b'], parts: () => [{ pts: circ(0, 0, 0.5, 0.5, 6), c: 0 }, { pts: circ(0, 0, 0.5, 0.5, 6), c: 1, kind: 'outline' }, { pts: circ(0, 0, 0.26, 0.26, 6), c: 1, kind: 'outline', w: 1.6 }] },
  { id: 'kranz', name: 'Kranz', kat: 'Natur', w: 76, h: 76, farben: ['#4f8a3a', '#c8102e'], parts: () => [...Array.from({ length: 12 }, (_, i) => { const a = i / 12 * TAU; return { pts: xf(leaf(1, 0.45), { s: 0.2, x: Math.cos(a) * 0.38, y: Math.sin(a) * 0.38, r: a * 180 / Math.PI + 90 + 25 }), c: 0, ang: a * 180 / Math.PI }; }), ...[0.2, 1.75, 3.3].map(a => ({ pts: circ(Math.cos(a) * 0.29, Math.sin(a) * 0.29, 0.05, 0.05, 20), c: 1 }))] },
  { id: 'pfote', name: 'Pfote', kat: 'Tiere', w: 56, h: 56, farben: ['#5a3e2b'], parts: () => [{ pts: circ(0, 0.17, 0.25, 0.21, 48), c: 0 }, ...[[-0.33, -0.12], [-0.12, -0.33], [0.12, -0.33], [0.33, -0.12]].map(([x, y]) => ({ pts: circ(x, y, 0.1, 0.13, 32), c: 0 }))] },
  { id: 'bordure', name: 'Wellen-Bordüre', kat: 'Bordüren', w: 130, h: 34, farben: ['#1f8b8b', '#3a86c8'], parts: () => [{ pts: band(), c: 0, ang: 90 }, { pts: wave(0.5), c: 1, kind: 'line' }] },
];

function scan(pts, ang, sp) {
  const r = pts.map(p => rot(p, -ang)); let y0 = Infinity, y1 = -Infinity;
  for (const p of r) { y0 = Math.min(y0, p[1]); y1 = Math.max(y1, p[1]); }
  const rows = [];
  for (let y = y0 + sp / 2; y < y1; y += sp) {
    const xs = [];
    for (let i = 0, n = r.length; i < n; i++) { const A = r[i], B = r[(i + 1) % n]; if ((A[1] <= y && B[1] > y) || (B[1] <= y && A[1] > y)) xs.push(A[0] + (y - A[1]) / (B[1] - A[1]) * (B[0] - A[0])); }
    xs.sort((p, q) => p - q); rows.push({ y, xs });
  }
  return rows;
}
function tatami(pts, ang, sp = 0.45, len = 3.2) {
  const strands = [];
  scan(pts, ang, sp).forEach((row, i) => {
    for (let k = 0; k + 1 < row.xs.length; k += 2) {
      const s = strands[k / 2] || (strands[k / 2] = []);
      const a = row.xs[k], b = row.xs[k + 1], off = ((i % 3) / 3) * len, xs = [a];
      for (let x = a + off; x < b - 0.3; x += len) if (x > a + 0.3) xs.push(x);
      xs.push(b); if (i % 2) xs.reverse();
      for (const x of xs) s.push(rot([x, row.y], ang));
    }
  });
  const out = [];
  for (const st of strands) { if (!st) continue; let cur = [st[0]]; for (let i = 1; i < st.length; i++) { const a = st[i - 1], b = st[i]; if (Math.hypot(b[0] - a[0], b[1] - a[1]) > len * 1.6) { out.push(cur); cur = []; } cur.push(b); } out.push(cur); }
  return out.filter(x => x.length > 1);
}
function satinFill(pts, ang, sp = 0.4) {
  const strands = [];
  scan(pts, ang, sp).forEach(row => {
    for (let k = 0; k + 1 < row.xs.length; k += 2) { const s = strands[k / 2] || (strands[k / 2] = []); s.push(rot([row.xs[k], row.y], ang), rot([row.xs[k + 1], row.y + sp / 2], ang)); }
  });
  return strands;
}
function walk(pts, closed, step, fn) {
  const n = pts.length, seg = []; let tot = 0;
  for (let i = 0; i < (closed ? n : n - 1); i++) { const A = pts[i], B = pts[(i + 1) % n], l = Math.hypot(B[0] - A[0], B[1] - A[1]); if (l > 1e-6) { seg.push({ A, B, l, s0: tot }); tot += l; } }
  let k = 0, j = 0;
  for (let s = 0; s <= tot; s += step, j++) {
    while (k < seg.length - 1 && seg[k].s0 + seg[k].l < s) k++;
    const g = seg[k], t = Math.min(1, (s - g.s0) / g.l), tx = (g.B[0] - g.A[0]) / g.l, ty = (g.B[1] - g.A[1]) / g.l;
    fn([g.A[0] + (g.B[0] - g.A[0]) * t, g.A[1] + (g.B[1] - g.A[1]) * t], [-ty, tx], j);
  }
}
function satinOutline(pts, w = 2.2, sp = 0.42) { const o = []; walk(pts, true, sp / 2, (p, n, j) => { const d = (j % 2 ? 1 : -1) * w / 2; o.push([p[0] + n[0] * d, p[1] + n[1] * d]); }); return [o]; }
function running(pts, closed, len = 2.4) { const o = []; walk(pts, closed, len, p => o.push(p)); if (closed && o.length) o.push(o[0]); return [o]; }
function echo(pts, k) { let cx = 0, cy = 0; for (const p of pts) { cx += p[0]; cy += p[1]; } cx /= pts.length; cy /= pts.length; return pts.map(p => [cx + (p[0] - cx) * k, cy + (p[1] - cy) * k]); }

function partStrands(p, st, i) {
  const ang = (p.ang ?? (i % 2 ? 135 : 45)) * Math.PI / 180;
  if (p.kind === 'line') return running(p.pts, false, 2.0).concat(running(p.pts.slice().reverse(), false, 2.0));
  if (p.kind === 'outline') return st === 'stepp' ? running(p.pts, true, 2.2) : satinOutline(p.pts, p.w || 2.2);
  if (st === 'satin') return satinFill(p.pts, ((p.ang ?? 0) + 90) * Math.PI / 180, 0.4);
  if (st === 'stepp') return [1, 0.72, 0.44].flatMap(k => running(echo(p.pts, k), true, 2.2));
  return tatami(p.pts, ang);
}

let FONTS = {};
export function setFonts(f) { FONTS = f; }
function textStrands(o) {
  const res = 5, F = FONTS[o.font] || { css: px => `700 ${px}px sans-serif`, cap: 0.72 };
  const px = Math.max(8, Math.round(o.h * res / (F.cap || 0.72))), sx = (o.breite || 100) / 100;
  const c = document.createElement('canvas'), m = c.getContext('2d'); m.font = F.css(px);
  const tw = m.measureText(o.text || ' ').width, W = Math.ceil(tw * sx + px * 0.6), H = Math.ceil(px * 1.5);
  c.width = W; c.height = H; const x = c.getContext('2d', { willReadFrequently: true });
  x.font = F.css(px); x.textBaseline = 'middle'; x.setTransform(sx, 0, 0, 1, px * 0.3, H * 0.52); x.fillText(o.text || '', 0, 0);
  const d = x.getImageData(0, 0, W, H).data, step = 0.42 * res, strands = [];
  let prev = [];
  for (let fx = 0; fx < W; fx += step) {
    const xi = Math.round(fx), runs = []; let top = -1;
    for (let y = 0; y < H; y++) { const on = d[(y * W + xi) * 4 + 3] > 110; if (on && top < 0) top = y; if (!on && top >= 0) { runs.push([top, y - 1]); top = -1; } }
    if (top >= 0) runs.push([top, H - 1]);
    const next = [];
    for (const r of runs) {
      const hit = prev.find(p => !p.used && r[0] <= p.r[1] + 1 && r[1] >= p.r[0] - 1);
      const s = hit ? (hit.used = true, hit.s) : (strands.push({ pts: [], f: 0 }), strands[strands.length - 1]);
      s.f ^= 1; s.pts.push([(xi - W / 2) / res, ((s.f ? r[0] : r[1]) - H / 2) / res]); next.push({ r, s, used: false });
    }
    prev = next;
  }
  return strands.map(s => s.pts).filter(p => p.length > 1);
}

const cache = new Map();
function localBlocks(o) {
  const key = JSON.stringify([o.t, o.m, o.w, o.h, o.st, o.text, o.font, o.breite]);
  if (cache.has(key)) return cache.get(key);
  let blocks = [];
  if (o.t === 'text') blocks = [{ c: 0, strands: textStrands(o) }];
  else {
    const M = MOTIVE.find(m => m.id === o.m) || MOTIVE[0], byC = new Map();
    M.parts().forEach((p, i) => { const sp = { ...p, pts: p.pts.map(q => [q[0] * o.w, q[1] * o.h]) }; const arr = byC.get(p.c) || []; arr.push(...partStrands(sp, o.st || 'fuell', i)); byC.set(p.c, arr); });
    blocks = [...byC.entries()].sort((a, b) => a[0] - b[0]).map(([c, strands]) => ({ c, strands }));
  }
  let x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity;
  for (const b of blocks) { b.n = 0; for (const s of b.strands) { b.n += s.length - 1; for (const p of s) { x0 = Math.min(x0, p[0]); y0 = Math.min(y0, p[1]); x1 = Math.max(x1, p[0]); y1 = Math.max(y1, p[1]); } } }
  if (!isFinite(x0)) { x0 = y0 = -1; x1 = y1 = 1; }
  const r = { blocks, lb: [x0, y0, x1, y1] };
  if (cache.size > 300) cache.clear();
  cache.set(key, r); return r;
}

// design: {objs:[...], order:[keys]|null}; hoop: {w,h,oval}
export function build(design, hoop) {
  const objs = [], blocks = [];
  design.objs.forEach((o, oi) => {
    const L = localBlocks(o), a = (o.rot || 0) * Math.PI / 180, T = p => { const q = rot(p, a); return [q[0] + o.x, q[1] + o.y]; };
    const [x0, y0, x1, y1] = L.lb, corners = [[x0, y0], [x1, y0], [x1, y1], [x0, y1]].map(T);
    objs.push({ oi, x: o.x, y: o.y, rot: o.rot || 0, lb: L.lb, corners, w: x1 - x0, h: y1 - y0 });
    L.blocks.forEach(b => blocks.push({ key: oi + ':' + b.c, oi, c: b.c, hex: (o.colors || [])[b.c] || '#444', n: b.n, strands: b.strands.map(s => s.map(T)) }));
  });
  if (design.order) { const idx = k => { const i = design.order.indexOf(k); return i < 0 ? 999 : i; }; blocks.sort((a, b) => idx(a.key) - idx(b.key)); }
  let total = 0, over = 0, X0 = Infinity, Y0 = Infinity, X1 = -Infinity, Y1 = -Infinity;
  for (const b of blocks) total += b.n;
  for (const ob of objs) for (const [x, y] of ob.corners) {
    X0 = Math.min(X0, x); Y0 = Math.min(Y0, y); X1 = Math.max(X1, x); Y1 = Math.max(Y1, y);
    over = Math.max(over, Math.abs(x) - hoop.w / 2, Math.abs(y) - hoop.h / 2);
    if (hoop.oval) { const r = Math.min(hoop.w, hoop.h) / 2, lx = Math.max(0, Math.abs(x) - (hoop.w / 2 - r)), ly = Math.max(0, Math.abs(y) - (hoop.h / 2 - r)); over = Math.max(over, Math.hypot(lx, ly) - r); }
  }
  if (!objs.length) { X0 = Y0 = X1 = Y1 = 0; }
  return { objs, blocks, total, over: Math.max(0, over), bw: X1 - X0, bh: Y1 - Y0, bcx: (X0 + X1) / 2, bcy: (Y0 + Y1) / 2 };
}

export function hit(built, x, y) {
  for (let i = built.objs.length - 1; i >= 0; i--) {
    const o = built.objs[i], q = rot([x - o.x, y - o.y], -o.rot * Math.PI / 180), [x0, y0, x1, y1] = o.lb;
    if (q[0] >= x0 - 2 && q[0] <= x1 + 2 && q[1] >= y0 - 2 && q[1] <= y1 + 2) return o.oi;
  }
  return -1;
}

function shade(hex, k) {
  const n = parseInt(hex.slice(1), 16); let r = n >> 16, g = (n >> 8) & 255, b = n & 255;
  const f = v => Math.round(k < 0 ? v * (1 + k) : v + (255 - v) * k);
  return `rgb(${f(r)},${f(g)},${f(b)})`;
}
function rrect(c, x, y, w, h, r) { r = Math.min(r, w / 2, h / 2); c.beginPath(); c.moveTo(x + r, y); c.arcTo(x + w, y, x + w, y + h, r); c.arcTo(x + w, y + h, x, y + h, r); c.arcTo(x, y + h, x, y, r); c.arcTo(x, y, x + w, y, r); c.closePath(); }

function drawStrands(c, blocks, s, limit, opt) {
  const lw = Math.max(0.42, 0.9 / s); let count = 0, last = null, needle = null; const jumps = [];
  for (const b of blocks) {
    if (limit != null && count >= limit) break;
    const passes = opt.sheen ? [[shade(b.hex, -0.35), lw * 1.3, 0.1], [b.hex, lw, 0], [shade(b.hex, 0.35), lw * 0.38, -0.08]] : [[b.hex, lw, 0]];
    const paths = passes.map(() => new Path2D()); let cnt = count;
    for (const st of b.strands) {
      if (limit != null && cnt >= limit) break;
      if (last && Math.hypot(st[0][0] - last[0], st[0][1] - last[1]) > 1.5) jumps.push([last, st[0]]);
      for (const P of paths) P.moveTo(st[0][0], st[0][1]);
      let i = 1;
      for (; i < st.length; i++) { if (limit != null && cnt >= limit) break; for (const P of paths) P.lineTo(st[i][0], st[i][1]); cnt++; }
      last = st[i - 1]; needle = last;
    }
    passes.forEach(([col, w, off], k) => { c.save(); c.translate(off, off); c.strokeStyle = col; c.lineWidth = w; c.lineJoin = 'round'; c.lineCap = 'round'; c.stroke(paths[k]); c.restore(); });
    count = cnt;
  }
  return { needle, jumps };
}

// theme: {ring, ringEdge, accent, ink, bg}; o.hoop = null -> freie Arbeitsfläche (o.view = {w,h} mm)
export function draw(canvas, built, o) {
  const c = canvas.getContext('2d'), cw = canvas.width, ch = canvas.height, th = o.theme;
  c.setTransform(1, 0, 0, 1, 0, 0); c.clearRect(0, 0, cw, ch);
  let s, cx = cw / 2, cy = ch / 2, dark = false;
  if (o.hoop) {
    const W = o.hoop.w, H = o.hoop.h, m = 22; s = Math.min(cw / (W + 2 * m), ch / (H + 2 * m + 10)) * 0.94; cy = ch / 2 + 5 * s;
    c.setTransform(s, 0, 0, s, cx, cy);
    const R = o.hoop.oval ? Math.min(W, H) / 2 : 7;
    c.fillStyle = th.ringEdge; rrect(c, -10, -H / 2 - 26, 20, 14, 2); c.fill();
    c.fillStyle = th.ring; rrect(c, -W / 2 - 15, -H / 2 - 15, W + 30, H + 30, R + 15); c.fill();
    c.strokeStyle = th.ringEdge; c.lineWidth = 1.4 / s; c.stroke();
    rrect(c, -W / 2 - 7, -H / 2 - 7, W + 14, H + 14, R + 7); c.fillStyle = o.fabric; c.fill();
    c.save(); c.clip();
    dark = parseInt(o.fabric.slice(1, 3), 16) < 128; c.strokeStyle = dark ? 'rgba(255,255,255,.06)' : 'rgba(0,0,0,.05)'; c.lineWidth = 0.3;
    const step = s > 3 ? 1.2 : 2.4; c.beginPath();
    for (let x = -W / 2 - 8; x < W / 2 + 8; x += step) { c.moveTo(x, -H / 2 - 8); c.lineTo(x, H / 2 + 8); }
    for (let y = -H / 2 - 8; y < H / 2 + 8; y += step) { c.moveTo(-W / 2 - 8, y); c.lineTo(W / 2 + 8, y); }
    c.stroke();
    if (o.grid) {
      c.strokeStyle = dark ? 'rgba(255,255,255,.16)' : 'rgba(0,0,0,.12)'; c.lineWidth = 0.7 / s; c.beginPath();
      for (let x = 0; x <= W / 2; x += 10) { c.moveTo(x, -H / 2); c.lineTo(x, H / 2); c.moveTo(-x, -H / 2); c.lineTo(-x, H / 2); }
      for (let y = 0; y <= H / 2; y += 10) { c.moveTo(-W / 2, y); c.lineTo(W / 2, y); c.moveTo(-W / 2, -y); c.lineTo(W / 2, -y); }
      c.stroke(); c.strokeStyle = dark ? 'rgba(255,255,255,.35)' : 'rgba(0,0,0,.28)'; c.beginPath(); c.moveTo(-W / 2, 0); c.lineTo(W / 2, 0); c.moveTo(0, -H / 2); c.lineTo(0, H / 2); c.stroke();
    }
    c.restore();
    c.save(); c.setLineDash([5 / s, 4 / s]); c.strokeStyle = th.accent; c.lineWidth = 1.2 / s; rrect(c, -W / 2, -H / 2, W, H, o.hoop.oval ? R : 1); c.stroke(); c.restore();
    if (o.rulers) {
      const u = (o.dpr || 1) / s, inkA = dark ? 'rgba(255,255,255,.75)' : 'rgba(0,0,0,.6)', lblEvery = s / (o.dpr || 1) > 3 ? 10 : 20;
      c.save(); c.strokeStyle = inkA; c.fillStyle = inkA; c.lineWidth = 1 * u; c.font = 10 * u + 'px ' + (o.font || 'sans-serif'); c.beginPath();
      for (let x = Math.ceil(-W / 2 / 5) * 5; x <= W / 2; x += 5) { const L = x % 10 === 0 ? 7 * u : 4 * u; c.moveTo(x, H / 2); c.lineTo(x, H / 2 - L); }
      for (let y = Math.ceil(-H / 2 / 5) * 5; y <= H / 2; y += 5) { const L = y % 10 === 0 ? 7 * u : 4 * u; c.moveTo(-W / 2, y); c.lineTo(-W / 2 + L, y); }
      c.stroke(); c.textAlign = 'center'; c.textBaseline = 'bottom';
      for (let x = Math.ceil(-W / 2 / lblEvery) * lblEvery; x <= W / 2; x += lblEvery) if (Math.abs(x) < W / 2 - 3) c.fillText(String(x), x, H / 2 - 8 * u);
      c.textAlign = 'left'; c.textBaseline = 'middle';
      for (let y = Math.ceil(-H / 2 / lblEvery) * lblEvery; y <= H / 2; y += lblEvery) if (Math.abs(y) < H / 2 - 3) c.fillText(String(-y), -W / 2 + 9 * u, y);
      c.restore();
    }
  } else {
    const V = o.view || { w: 160, h: 160 }; s = Math.min(cw / V.w, ch / V.h);
    c.setTransform(s, 0, 0, s, cx, cy);
    const hw = cw / 2 / s, hh = ch / 2 / s;
    c.lineWidth = 0.6 / s;
    for (const [stp, al] of [[10, 0.07], [50, 0.13]]) {
      c.strokeStyle = 'rgba(0,0,0,' + al + ')'; c.beginPath();
      for (let x = 0; x <= hw; x += stp) { c.moveTo(x, -hh); c.lineTo(x, hh); c.moveTo(-x, -hh); c.lineTo(-x, hh); }
      for (let y = 0; y <= hh; y += stp) { c.moveTo(-hw, y); c.lineTo(hw, y); c.moveTo(-hw, -y); c.lineTo(hw, -y); }
      c.stroke();
    }
  }
  canvas._view = { s, cx, cy };
  const { needle, jumps } = drawStrands(c, built.blocks, s, o.limit, o);
  if (o.jumps && jumps.length) { c.save(); c.setLineDash([2 / s, 2 / s]); c.strokeStyle = dark ? 'rgba(255,255,255,.7)' : 'rgba(0,0,0,.55)'; c.lineWidth = 0.8 / s; c.beginPath(); for (const [p, q] of jumps) { c.moveTo(p[0], p[1]); c.lineTo(q[0], q[1]); } c.stroke(); c.restore(); }
  if (o.limit != null && needle && o.limit < built.total) {
    c.save(); c.strokeStyle = th.accent; c.lineWidth = 1.5 / s; c.beginPath(); c.arc(needle[0], needle[1], 5 / s, 0, TAU); c.moveTo(needle[0] - 9 / s, needle[1]); c.lineTo(needle[0] + 9 / s, needle[1]); c.moveTo(needle[0], needle[1] - 9 / s); c.lineTo(needle[0], needle[1] + 9 / s); c.stroke(); c.restore();
  }
  canvas._handles = null; const u = (o.dpr || 1) / s;
  const ob = o.sel >= 0 && built.objs[o.sel];
  if (ob) {
    const k = ob.corners, ctr = [(k[0][0] + k[2][0]) / 2, (k[0][1] + k[2][1]) / 2], a = ob.rot * Math.PI / 180, up = [Math.sin(a), -Math.cos(a)];
    const tm = [(k[0][0] + k[1][0]) / 2, (k[0][1] + k[1][1]) / 2], rh = [tm[0] + up[0] * 22 * u, tm[1] + up[1] * 22 * u];
    c.save(); c.setLineDash([4 * u, 3 * u]); c.strokeStyle = th.accent; c.lineWidth = 1.2 * u; c.beginPath(); k.forEach((p, i) => i ? c.lineTo(p[0], p[1]) : c.moveTo(p[0], p[1])); c.closePath(); c.stroke(); c.setLineDash([]);
    if (o.handles !== false) {
      c.beginPath(); c.moveTo(tm[0], tm[1]); c.lineTo(rh[0], rh[1]); c.stroke();
      c.fillStyle = th.bg; c.beginPath(); c.arc(rh[0], rh[1], 5.5 * u, 0, TAU); c.fill(); c.stroke();
      const hs = 9 * u; for (const p of k) { c.fillStyle = th.bg; c.fillRect(p[0] - hs / 2, p[1] - hs / 2, hs, hs); c.strokeRect(p[0] - hs / 2, p[1] - hs / 2, hs, hs); }
    }
    if (o.label) {
      const by = Math.max(k[0][1], k[1][1], k[2][1], k[3][1]) + 16 * u, fs = 12 * u; c.font = '600 ' + fs + 'px ' + (o.font || 'sans-serif'); c.textAlign = 'center'; c.textBaseline = 'middle';
      const tw = c.measureText(o.label).width; c.fillStyle = th.accent; rrect(c, ctr[0] - tw / 2 - 6 * u, by - 9 * u, tw + 12 * u, 18 * u, 2 * u); c.fill(); c.fillStyle = th.bg; c.fillText(o.label, ctr[0], by + 0.5 * u);
    }
    c.restore();
    canvas._handles = { corners: k, rot: rh, center: ctr };
  }
  if (o.group && built.objs.length) {
    const x0 = built.bcx - built.bw / 2, x1 = built.bcx + built.bw / 2, y0 = built.bcy - built.bh / 2, y1 = built.bcy + built.bh / 2, k = [[x0, y0], [x1, y0], [x1, y1], [x0, y1]];
    const f = v => (Math.round(v * 10) / 10).toLocaleString('de-DE') + ' mm';
    c.save(); c.strokeStyle = th.accent; c.lineWidth = 1.2 * u; c.setLineDash([4 * u, 3 * u]); c.strokeRect(x0, y0, x1 - x0, y1 - y0); c.setLineDash([]);
    const hs = 10 * u; for (const p of k) { c.fillStyle = th.bg; c.fillRect(p[0] - hs / 2, p[1] - hs / 2, hs, hs); c.strokeRect(p[0] - hs / 2, p[1] - hs / 2, hs, hs); }
    c.lineWidth = 1 * u; c.font = '600 ' + 11 * u + 'px ' + (o.font || 'sans-serif'); c.textAlign = 'center'; c.textBaseline = 'middle';
    const tag = (txt, x, y, rotd) => { c.save(); c.translate(x, y); if (rotd) c.rotate(-Math.PI / 2); const tw = c.measureText(txt).width + 10 * u; c.fillStyle = th.accent; c.fillRect(-tw / 2, -8 * u, tw, 16 * u); c.fillStyle = th.bg; c.fillText(txt, 0, 0.5 * u); c.restore(); };
    const tick = (x, y) => { c.moveTo(x - 4 * u, y + 4 * u); c.lineTo(x + 4 * u, y - 4 * u); };
    const gy = y1 + 22 * u, gx = x0 - 22 * u; c.beginPath();
    c.moveTo(x0, y1 + 6 * u); c.lineTo(x0, gy + 5 * u); c.moveTo(x1, y1 + 6 * u); c.lineTo(x1, gy + 5 * u); c.moveTo(x0, gy); c.lineTo(x1, gy); tick(x0, gy); tick(x1, gy);
    c.moveTo(x0 - 6 * u, y0); c.lineTo(gx - 5 * u, y0); c.moveTo(x0 - 6 * u, y1); c.lineTo(gx - 5 * u, y1); c.moveTo(gx, y0); c.lineTo(gx, y1); tick(gx, y0); tick(gx, y1);
    c.stroke(); tag(f(built.bw), (x0 + x1) / 2, gy, false); tag(f(built.bh), gx, (y0 + y1) / 2, true);
    if (o.label) tag(o.label, (x0 + x1) / 2, y0 - 16 * u, false);
    c.restore();
    canvas._handles = { corners: k, rot: null, center: [built.bcx, built.bcy], group: true };
  }
}

export function hitBlock(built, oi, x, y) {
  let best = -1, bd = 3;
  for (const b of built.blocks) { if (b.oi !== oi) continue; for (const st of b.strands) for (let i = 0; i < st.length; i += 2) { const d = Math.hypot(st[i][0] - x, st[i][1] - y); if (d < bd) { bd = d; best = b.c; } } }
  return best;
}

export function thumb(canvas, design, fabric) {
  const c = canvas.getContext('2d'), cw = canvas.width, ch = canvas.height;
  const b = build(design, { w: 999, h: 999 });
  c.setTransform(1, 0, 0, 1, 0, 0); c.fillStyle = fabric; c.fillRect(0, 0, cw, ch);
  const s = Math.min(cw / (b.bw + 10), ch / (b.bh + 10)); c.setTransform(s, 0, 0, s, cw / 2 - b.bcx * s, ch / 2 - b.bcy * s);
  drawStrands(c, b.blocks, s, null, { sheen: true });
}
