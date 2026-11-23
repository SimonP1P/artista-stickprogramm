<script lang="ts">
  // Übertragen im Expertenmodus (Entscheidung des Nutzers 22.11.2026, `UI_Mockups/UX_ANALYSE_2026-11-22.md` S1–S8):
  // das Fenster öffnet sich beim Klick und überträgt sofort – Bild und Garnfarben immer, danach ist die Maschine
  // bereit zum Sticken (ohne weitere Bestätigung). Im einfachen Modus gibt es kein Fenster: der Stand steht im
  // Schritt „Übertragen“ (`Uebertragung.svelte`).
  import { arbeit } from "../arbeit.svelte";
  import Icon from "../Icon.svelte";
  import { IC } from "../icons";
  import { maschine } from "../maschine.svelte";
  import Uebertragung from "../Uebertragung.svelte";
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && !maschine.laeuft && (e.stopPropagation(), maschine.schliessen())} />

<div class="dlg-bg" role="presentation">
  <div class="dlg" role="dialog" aria-modal="true" aria-label="An die Maschine übertragen" style="width:520px; max-height:calc(100vh - 40px)">
    <div class="dlg-h">
      <Icon d={IC.senden} groesse="l" />
      <h2 class="hd">An die Maschine übertragen</h2>
      <span class="mut small" style="overflow:hidden; text-overflow:ellipsis; white-space:nowrap">„{arbeit.design.name ?? "Design"}“</span>
      <span style="flex:1"></span>
      {#if !maschine.laeuft}<button class="tb" aria-label="Schließen" onclick={() => maschine.schliessen()}><Icon d={IC.kreuz} /></button>{/if}
    </div>

    <div class="dlg-b">
      {#if maschine.simulation}
        <div class="simbox" style="margin-bottom:12px"><Icon d={IC.monitor} /><span><b>Simulation</b> – es wird nichts gesendet. Anschluss oben im Menü „Simulation“ wählen.</span></div>
      {/if}
      <Uebertragung details />
    </div>

    <div class="dlg-f">
      {#if maschine.laeuft}
        <button class="btn btn-secondary" onclick={() => maschine.abbrechen()}>Abbrechen</button>
      {:else if maschine.phase === "fertig" && maschine.bericht?.gestickt !== false}
        <button class="btn btn-primary" onclick={() => maschine.schliessen()}>Schließen</button>
      {:else}
        <button class="btn btn-secondary" style="margin-right:auto" onclick={() => maschine.schliessen()}>Schließen</button>
        <button class="btn btn-primary" onclick={() => maschine.erneut()}>Nochmal</button>
      {/if}
    </div>
  </div>
</div>
