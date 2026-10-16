# FontString alpha gradient (T4, approved simulator policy)

Integration checkpoint (2026-10-16): user approved the scalar/taper/completion policy; Retail/Wrath API and renderer RED → GREEN recorded in [bounded proof](../baselines/approved-batch-2026-10-16.md). No native-parity claim.

FontString text reveal and its completion boolean. Producer: `src/lua_api/frame/methods/misc/alpha_gradient.rs`; rendering: `src/render/glyph.rs`. The contract was approved and its tested API/render boundaries reached behavioral RED before producer integration; it does not establish native parity. See [rendering](../rendering-pipeline.md).

## What it must do

- [ ] Interpret start/length in Unicode scalar positions after stripping WoW markup. Spaces and explicit line breaks count; automatic wrapping adds no positions. A shaped cluster uses its first scalar's alpha.
- [ ] Multiply ordinary text, shadow and outline alpha by `clamp((start + length - position) / length, 0, 1)`. For zero length use a hard boundary: positions strictly below start visible, others hidden. Preserve geometry, RGB and layout/cache behavior; colored segments continue the same position sequence.
- [ ] Replace slot-zero gradient on each setter call, retaining current storage/getter/clear behavior. Completion retains the gradient; clearing restores ordinary text rendering.
- [ ] Return true while start is less than stripped scalar count, false at/after that count (including empty text). Length controls the taper, not the completion threshold. Completion ignores viewport clipping, font size and wrapping.
- [ ] Drive the original cached Wrath callback through gradual progress and the consumer's 1024 instant-progress convention; clear fading, reveal its dependent frame and enable accept on false. Characterize the cached Mists Classic callback explicitly, without claiming automatic profile selection. Immediate dependent reveal branch only; scheduled dependent-frame fade is separate.

## How it works

- [Rendering pipeline](../rendering-pipeline.md)
- [Widget system](../widget-system.md)

## Implementation inventory

- `src/widget/frame_types.rs`: stored gradient and scalar alpha calculation.
- `src/lua_api/frame/methods/misc/alpha_gradient.rs`: setter state and completion result.
- `src/render/glyph.rs`, `src/render/glyph/text_emit.rs`: shaped cluster positions and alpha paint.
- `src/iced_app/quad_builders.rs`: FontString paint and colored-segment offsets.

## Tests asserting this spec

- `tests/fontstring_alpha_gradient.rs`: scalar/markup completion, replacement, clearing and empty text.
- `src/iced_app/quad_builders_alpha_gradient_tests.rs`: concrete emitted glyph alpha and unchanged geometry; original cached consumer callbacks are separate, currently blocked proof.

Public consumer citations (cache root `~/.cache/wow-ui-sim/blizzard-ui`): Wrath `FrameXML/QuestInfo.lua:3-16,479-492`, constants `FrameXML/QuestFrame.lua:4-6`; Mists `AddOns/Blizzard_UIPanels_Game/Classic/QuestFrame.lua:4-6,799-814`, dependent helpers `AddOns/Blizzard_UIPanels_Game/Wrath/QuestInfo.lua:919-945`. Wrath's cache places the consumer beside AddOns, not inside it; tests use that literal location, without an alternate-source fallback.

## Known gaps (current cycle)

- [x] User separately approved units/taper/completion; API and three renderer cases per Retail/Wrath profile reached RED then GREEN.
- [ ] Original Wrath quest callbacks remain blocked by legacy PlaySound string input. Main requires rerun only after `wrath-fixes` and this branch merge.
- [ ] Mists fixture, automatic profile selection, native and broader acceptance remain unexecuted. Startup observations are separate and Wrath remains a failing compatibility inventory.

## Out of scope

Native thresholds, malformed/nonfinite input redesign, ScrollBox edge fades, vendor edits, quest acceptance effects, automatic Mists branch-selection proof and other-profile acceptance.
