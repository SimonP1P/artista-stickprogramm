<script lang="ts">
  // Schnelle Tooltips (Wunsch Nutzer 21.11.2026: die eingebauten des Browsers erscheinen am Mac erst nach 1–2 s und
  // lassen sich nicht einstellen). Übernimmt jedes `title` der Seite beim Überfahren nach `data-tip` und zeigt den Text
  // nach VERZOEGERUNG_MS. Elemente in einem Bereich mit `data-tip-rechts` (Werkzeugleiste) zeigen ihn rechts daneben.
  const VERZOEGERUNG_MS = 180;

  let tip = $state<{ text: string; x: number; y: number; rechts: boolean; oben: boolean } | null>(null);
  let ziel: HTMLElement | null = null;
  let zeitgeber: ReturnType<typeof setTimeout> | undefined;

  function weg() {
    clearTimeout(zeitgeber);
    tip = null;
    ziel = null;
  }

  function ueber(ev: PointerEvent) {
    if (ev.pointerType === "touch") return;
    const el = (ev.target as Element | null)?.closest?.("[title], [data-tip]") as HTMLElement | null;
    if (el === ziel) return;
    weg();
    if (!el || el instanceof SVGElement) return;
    const t = el.getAttribute("title");
    if (t) {
      el.dataset.tip = t;
      el.removeAttribute("title");
    }
    const text = el.dataset.tip;
    if (!text) return;
    ziel = el;
    zeitgeber = setTimeout(() => {
      if (ziel !== el || !el.isConnected) return;
      const r = el.getBoundingClientRect();
      const rechts = !!el.closest("[data-tip-rechts]");
      const oben = !rechts && r.bottom + 40 > window.innerHeight;
      tip = rechts
        ? { text, x: r.right + 8, y: r.top + r.height / 2, rechts, oben: false }
        : { text, x: Math.min(Math.max(r.left + r.width / 2, 100), window.innerWidth - 100), y: oben ? r.top - 6 : r.bottom + 6, rechts, oben };
    }, VERZOEGERUNG_MS);
  }

  function raus(ev: PointerEvent) {
    if (!ev.relatedTarget) weg();
  }
</script>

<svelte:document onpointerover={ueber} onpointerout={raus} onpointerdown={weg} onkeydown={weg} onwheel={weg} />

{#if tip}
  <div
    class="tip"
    role="tooltip"
    style="left:{tip.x}px; top:{tip.y}px; transform:{tip.rechts ? 'translateY(-50%)' : tip.oben ? 'translate(-50%,-100%)' : 'translateX(-50%)'}"
  >
    {tip.text}
  </div>
{/if}

<style>
  .tip {
    position: fixed;
    z-index: 10000;
    pointer-events: none;
    max-width: min(320px, calc(100vw - 16px));
    padding: 4px 8px;
    font-size: 12px;
    line-height: 1.35;
    color: #fff;
    background: #1f2328;
    border-radius: 3px;
    box-shadow: 0 2px 8px rgb(0 0 0 / 0.25);
    white-space: normal;
  }
</style>
