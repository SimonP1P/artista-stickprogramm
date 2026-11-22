<script lang="ts">
  // Schriften eingebettet (offline, UEBERGABE §5): Barlow 400/500/700, Barlow Condensed 400/600
  import "@fontsource/barlow/400.css";
  import "@fontsource/barlow/500.css";
  import "@fontsource/barlow/700.css";
  import "@fontsource/barlow-condensed/400.css";
  import "@fontsource/barlow-condensed/600.css";
  import "$lib/stil/industry.css";
  import "$lib/stil/app.css";
  import Tooltip from "$lib/Tooltip.svelte";

  let { children } = $props();

  // Programm statt Webseite (Nutzer 21.11.2026): kein Rechtsklick-Menü des Browsers (außer in Eingabefeldern zum
  // Kopieren/Einfügen) und keine Browser-Tasten (Neu laden, Suchen, Drucken, Quelltext, Zurück, Zoom). WebView2 unter
  // Windows führt diese sonst aus; die Seite kann sie mit preventDefault abfangen. Die Tasten des Programms
  // (Strg+S/O/N/E/Z/Y/D/G/A) sind nicht dabei.
  const eingabe = (t: EventTarget | null) => !!(t as HTMLElement | null)?.closest?.("input, textarea, select, [contenteditable]");
  function kontextmenue(ev: MouseEvent) {
    if (!eingabe(ev.target)) ev.preventDefault();
  }
  const BROWSER_STRG = new Set(["r", "f", "p", "u", "j", "h", "l", "k", "+", "-", "=", "0"]);
  const BROWSER_TASTEN = new Set(["F1", "F3", "F5", "F6", "F7", "F12", "BrowserBack", "BrowserForward", "BrowserRefresh", "BrowserSearch"]);
  function browserTasten(ev: KeyboardEvent) {
    const strg = ev.ctrlKey || ev.metaKey;
    const k = ev.key.toLowerCase();
    if (
      BROWSER_TASTEN.has(ev.key) ||
      (strg && BROWSER_STRG.has(k)) ||
      (strg && ev.shiftKey && ["i", "c"].includes(k)) ||
      (ev.altKey && (ev.key === "ArrowLeft" || ev.key === "ArrowRight") && !eingabe(ev.target))
    )
      ev.preventDefault();
  }
</script>

<svelte:window oncontextmenu={kontextmenue} onkeydowncapture={browserTasten} />

{@render children()}
<Tooltip />
