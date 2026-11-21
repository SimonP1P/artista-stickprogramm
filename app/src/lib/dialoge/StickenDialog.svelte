<script lang="ts">
  // Gefahrendialog (UEBERGABE §7): Sticken nur nach drei Bestätigungen. „Sticken starten“ überträgt das bestätigte
  // Design noch einmal (zurücklesen) und gibt dann Befehl 0x08 – der Rahmen fährt sofort (am Gerät erprobt 19.11.2026).
  import Icon from "../Icon.svelte";
  import { IC } from "../icons";
  import { maschine } from "../maschine.svelte";

  let jemand = $state(false);
  let rahmen = $state(false);
  let haende = $state(false);
  const frei = $derived(jemand && rahmen && haende);

  function schliessen() {
    maschine.stickenDialog = false;
  }
  function starten() {
    if (!frei) return;
    maschine.stickenDialog = false;
    maschine.schreiben(true);
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && (e.stopPropagation(), schliessen())} />

<div class="dlg-bg" role="presentation" style="background:color-mix(in srgb,#0d0f11 62%,transparent); z-index:50">
  <div class="dlg" role="alertdialog" aria-modal="true" aria-labelledby="dSt" aria-describedby="dStD" style="width:520px; border:2px solid var(--danger)">
    <div class="hazard" aria-hidden="true"></div>
    <div class="dlg-h">
      <span style="color:var(--danger)"><Icon d={IC.warnung} groesse="l" /></span>
      <h2 id="dSt" class="hd">Jetzt sticken – der Rahmen bewegt sich sofort</h2>
    </div>
    <div class="dlg-b" id="dStD">
      <p style="margin:0 0 10px">
        Das Design wird noch einmal übertragen und geprüft; danach fährt der Rahmen ohne weitere Rückfrage los. Bitte
        bestätige jeden Punkt:
      </p>
      {#if maschine.simulation}
        <div class="simbox" style="margin-bottom:8px"><Icon d={IC.monitor} /><span><b>Simulation:</b> es fährt kein Rahmen.</span></div>
      {/if}
      <div style="display:flex; flex-direction:column; gap:6px">
        <label class="chk"><input type="checkbox" bind:checked={jemand} /><span><b style="font-weight:500">Es ist jemand an der Maschine.</b></span></label>
        <label class="chk"><input type="checkbox" bind:checked={rahmen} /><span><b style="font-weight:500">Rahmen eingesetzt, Nadel oben.</b> <span class="mut small">Nach Rahmenwechsel: «check» ist erledigt.</span></span></label>
        <label class="chk"><input type="checkbox" bind:checked={haende} /><span><b style="font-weight:500">Hände weg vom Stickbereich.</b></span></label>
      </div>
    </div>
    <div class="dlg-f">
      <button class="btn btn-secondary" style="margin-right:auto" onclick={schliessen}>Nicht sticken</button>
      <button class="btn btn-danger" disabled={!frei} onclick={starten}>Sticken starten – Rahmen fährt</button>
    </div>
  </div>
</div>
