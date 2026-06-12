# ScaleEventProbe

Captures when the real retail client fires `DISPLAY_SIZE_CHANGED` and
`UI_SCALE_CHANGED`, with the screen size, `uiScale`/`useUiScale` CVars, and
UIParent effective scale at the moment each event fired. Also logs
`CVAR_UPDATE` for the two scale CVars.

## Why

Community evidence (ElvUI commit `2934c29c` "add option to ignore the UI Scale
changed popup when changing the window size", tukui issue #1066) shows the
live client fires `UI_SCALE_CHANGED` on window resize/maximize, at least in
some configurations — so "resize never implies UI_SCALE_CHANGED" is not safe
retail truth. This probe pins down the exact conditions.

## Scenarios to capture

Install under the live retail AddOns directory, enable, log in. Run each
scenario with a marker first, then `/reload` or logout to flush
`ScaleEventProbeDB`:

"The window" below means the WoW client's OS window: set Display Mode to
**Windowed** (not Fullscreen / Fullscreen Windowed, which have no resizable
border), then drag the window edge/corner like any application window.

1. `/scaleprobe mark fixed-scale resize` — with **Use UI Scale enabled** and a
   manual slider value, drag-resize the window. Expectation to test: does
   `UI_SCALE_CHANGED` fire, or only `DISPLAY_SIZE_CHANGED`?
2. `/scaleprobe mark auto-scale resize` — with **Use UI Scale disabled**
   (default/auto scale), drag-resize the window so the height changes.
3. `/scaleprobe mark maximize-restore` — windowed mode, click
   maximize/restore.
4. `/scaleprobe mark scale-slider` — change the UI Scale slider in System
   settings (no resize). Baseline: `UI_SCALE_CHANGED` should fire.
5. `/scaleprobe mark resolution-change` — change resolution / fullscreen
   toggle in System settings.

Each event also prints to chat live, so you can watch whether a continuous
drag fires once or repeatedly. `/scaleprobe counts` prints per-event totals.
