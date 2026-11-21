// Tauri hat keinen Node-Server: adapter-static mit Rückfall auf index.html (SPA).
// Siehe https://v2.tauri.app/start/frontend/sveltekit/
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      fallback: "index.html",
    }),
    // Hash-Routing: die gebaute Seite läuft auch als Datei (Bildschirmfotos mit werkzeuge/bildschirmfoto.py)
    router: { type: "hash" },
    alias: {
      // Schnittstelle zum Rechenkern – aus dem Kern erzeugt (`stick api exportieren`), NICHT kopieren,
      // damit die Typen immer zum Kern passen (CLAUDE.md §4 Regel 7).
      $kern: "../kern/schnittstelle",
      // kern.ts liegt außerhalb von app/ und importiert @tauri-apps/api – aus app/node_modules auflösen
      "@tauri-apps/api": "node_modules/@tauri-apps/api",
    },
  },
};

export default config;
