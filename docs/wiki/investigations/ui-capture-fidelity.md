# UI Capture Fidelity

A comparison of headless `wow-sim screenshot` renders against a 12.1.0 client capture (3440×1440, uiScale 0.9, so 1.6875 px per UI unit) found a set of simulator-side defects in text, atlas sampling and startup state. This page records the mechanism of each one that was fixed, and the findings that were not carried onto master.

## Content

### Screenshot command

- **`--quality`.** The standalone screenshot is lossy WebP at quality 15, which discards the small text and one-pixel borders a UI capture exists to show. `--quality 1-100` sets the WebP quality. The default stays 15, as `docs/specs/screenshot-output-format.md` states; `--png` stays the lossless path.
- **`--ui-scale <f>`.** The client renders one UI unit as `height / 768 × uiScale` pixels. The simulator ignores the `uiScale` cvars and leaves `UIParent` at scale 1. `--ui-scale` applies the scale between startup settle and `set_screen_size`. That order matters: `set_screen_size` fires `DISPLAY_SIZE_CHANGED` / `UI_SCALE_CHANGED` and replays the edit-mode anchor hooks, which re-run `ManageFramePositions` and size the managed containers for the scaled `UIParent`. A scale set from `--exec-lua` runs after that pass, so `RightManagedFrameContainer` keeps the height computed for scale 1.
- **`--delay` reaches timers.** The sleep is followed by one OnUpdate tick that reports the slept time as `elapsed`, and by the timers that came due, before `--exec-lua` runs. An addon that defers its setup with `C_Timer.After(n)` is initialised by the time the script runs. A pending `ReloadUI` is serviced after that tick, as after every other Lua phase.

### Atlas art and sampling

- **2x atlas art.** The atlas database carries a `-2x` sibling for most HUD art, and the client draws it once a UI unit spans more than one pixel. `get_render_atlas_info` now prefers the paired `-2x` entry when `prefer_hires_atlases` is set. `wow-sim` sets the flag when `--ui-scale` is above 1.0; `WOW_SIM_HIRES_ATLASES=0/1` forces either choice. The 2x art is not exactly twice the 1x size, so `AtlasLookup` carries a `logical_size` taken from the 1x sibling, and `useAtlasSize` keeps the 1x dimensions. StatusBar atlas textures resolve through the render lookup too.
- **Bilinear sampling.** The atlas sampler used `FilterMode::Nearest` since commit `048333a4b`, so every magnified texel became a hard block above 1 px per unit. The sampler is bilinear, as the glyph sampler already was. A headless test draws a 2×2 checker magnified to 16×16 and requires intermediate values.
- **Single-texel strips.** `remap_entry_uv` zeroed the half-texel inset for a 1-texel axis. Under the bilinear sampler, a three-slice centre strip then blended with the neighbouring atlas cell. A single-texel axis now samples the centre of its texel.
- **Crop edge bleed.** `extract_sub_region` bled the nearest opaque edge colour into every black texel with alpha up to 128, a guard against seams under the nearest sampler. Icons with a black drop shadow at alpha 2..128 (micro-menu icons, bag coins) got a halo in the icon colour. Only alpha-0 texels take the edge colour now, and the persistent crop cache key carries `CROP_CACHE_VERSION` so old crops are not reused.
- **Override-sized 2x entries, generator only.** Atlas members with `OverrideWidth/OverrideHeight` store the display size, whereas other `-2x` entries store the pixel rect, and the `-2x` fallback halves both. `wow-cli generate atlas` now emits `size_is_override` and accessors that skip the halving for flagged entries. The committed atlas tables are not regenerated, so the micro-menu plates (`uimicromenu2x`, no 1x sibling) still resolve to 16×20 instead of 32×41.

### Fonts and text

- **`<Shadow>` in font definitions.** `FontXml` had no field for the element, so every font object from `Fonts.xml` had no drop shadow. The loader emits `SetShadowOffset` / `SetShadowColor` on the font-object paths; `GameFontNormal` reports offset (1, −1) and colour (0, 0, 0, 1).
- **`<Color>` in font definitions.** `FontXml` had no field for this element either, so `GameFontNormal` and its relatives answered (1, 1, 1) and inheriting FontStrings rendered white where the client draws gold. The loader parses the element and emits `SetTextColor` on the font object after the inheritance copy, so a child's own colour wins. A FontString's own `<Color color="NAME"/>` resolves through the named colour global the same way.
- **Shadow direction and scale.** `SetShadowOffset(1, -1)` means one unit right and one unit down, because WoW's y axis points up. The quad builders passed the pair through in screen space (y down) and unscaled, so the shadow sat above the glyphs. The offset is flipped and scaled with the UI now; a render test with a (2, −2) shadow pins the direction. The edit-box text path in `quad_builders_button.rs` still passes the raw offset.
- **Cooldown-count font copy.** A font object carries loader fields (`__font`, `__height`, with `__height = 12.0` as a placeholder for an inherited size) and live fields (`__fontPath`, `__fontHeight`, `__fontFlags`) that `GetFont` reads. Master's `read_font_object_fields` already prefers the live set; the cooldown-count copy in `cooldown.rs` now does too.

### Startup state

- **Permanent auras.** `seed_buff_durations` computed `expirationTime - GetTime()` for auras with `duration == 0` and `expirationTime == 0`, and displayed "-2 s". Blizzard gates the computation on both fields being positive (`BuffFrame.lua`).
- **Pet-battle queue status.** `C_PetBattles.GetPVPMatchmakingInfo` returned the numeric `None`; the client returns nothing while not queued. `QueueStatusFrame.lua` lists a pet-battle entry for any non-nil status, so the LFG eye showed "Pet Battle / Unknown" in every render.
- **Chat window names.** `FCF_SetWindowName` names an unnamed window itself and stores the name through `SetChatWindowName`, which the chat-window workaround lacked. The tabs read "General" and "Combat Log".
- **Chat channel button.** The voice workaround drew a second icon over Blizzard's headset icon, and the default voice state reported an active channel. The workaround keeps Blizzard's icon, and the default state is outside a voice channel.
- **Status bar selection highlight.** Startup called the edit-mode-only `EditModeAccountSettingsMixin:RefreshStatusTrackingBar2`, which sets `isInEditMode` and calls `HighlightSystem()`. The status bars then showed the Edit Mode selection brackets. The call is gone, and a full-startup test guards it.

### Findings not carried onto master

These were part of the original branch and are left out, because current master pins the opposite behaviour or already has the change:

- **Glyph snapping to whole pixels.** `smooth_scaling_render_measure_and_cache_flips` and `smooth_scaling_wrapped_render_and_cached_measurement_agree` pin sub-pixel glyph positions (line separation 15.84; snapping gives 16).
- **`systemInfo` tracker sizing and its test adjustments.** The `objective_tracker` fixture in `tests/frame_positions.rs` fails with the change.
- **`FontString:SetVertexColor` / `GetVertexColor`.** `vertex_boolean_selects_both_rgba_branches` and `vertex_boolean_isolates_region_instances` fail with it, so `PlayerLevelText` and the buff durations still render white.
- **Objective tracker height clamp.** Master removed the forced `SetHeight(836.5)` in `c6a452dfc`.
- **Regenerated 12.1.0 atlas tables** and the `src/atlas.rs` consumer of `size_is_override`. The consumer reads a field that the committed tables do not have, so both go in together with a regeneration.

## Sources

- [main.rs](../../../src/bin/wow_sim/main.rs) and [gui_commands.rs](../../../src/bin/wow_sim/gui_commands.rs) — screenshot options `--quality`, `--ui-scale`, `--delay`
- [atlas.rs](../../../src/atlas.rs) — `prefer_hires_atlases`, render lookup
- [gen_atlas.rs](../../../src/bin/wow_cli/gen_atlas.rs) — `size_is_override` emission
- [quad_builders.rs](../../../src/iced_app/quad_builders.rs) — text shadow offset
- [xml_file.rs](../../../src/loader/xml_file.rs) — font `<Shadow>` and `<Color>`
- [screenshot-output-format.md](../../specs/screenshot-output-format.md) — screenshot encoding contract

## See Also

- [[texture-atlas]] — atlas lookup and nine-slice kits
- [[rendering-pipeline]] — samplers, quad batches, glyph atlas
- [[cli-commands]] — screenshot and `--delay` options
