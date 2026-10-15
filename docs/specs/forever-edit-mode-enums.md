# Forever Edit Mode enums

Forever publishes finite additions from its `EditModeManagerConstantsDocumentation.lua` through `src/c_api/forever_edit_mode_enums.rs`. Existing 1.60.1.69913 proof is historical; the bounded 1.60.1.70374 gamepad proof is tracked below. See [client profiles](../wiki/systems/client-profiles.md#forever-gamepad-edit-mode--2026-10-15).

## What it must do

- [x] Publish the six missing GroupFinder, MainActionBarEndCap and SwingTimer setting/index/visibility namespaces and their source metadata.
- [x] Publish documented LossOfControl and RaidWarning settings, Gamepad preset 2, systems 24–29, account settings 33–37, minimap IconScale, micro-menu DeprecatedEyeSize and unit-frame DebuffIconSize/BuffIconSize with updated metadata.
- [x] Reuse the existing RaidDispelOverlayType values and metadata.
- [x] Execute the historical 69913 Forever preset and setting-display consumers without enum lookup failures; this does not establish 70374 compatibility.
- [x] Under `client-wowforever`, publish 70374 `EditModeSystem.GamepadMainActionBar=30` and system metadata `MinValue=0`, `MaxValue=30`, `NumValues=31`.
- [x] Publish `EditModeGamepadMainActionBarSetting`: `Compact=0`, `ShowEmptyBars=1`, `ShowButtonPrompts=2`, `ShowHighlight=3`, `ShouldExpand=4`; metadata `MinValue=0`, `MaxValue=4`, `NumValues=5`.
- [x] Execute actual 70374 Camelot preset constants, Mainline presets and shared setting-display consumers: `EDIT_MODE_GAMEPAD_SYSTEM_MAP[30].settings` has exactly five entries, values `0/1/0/1/1` at keys `0..4`, BOTTOM/UIParent anchor with offsetY 110, and five checkbox descriptors mapping settings `0..4`.
- [ ] Preserve other-profile runtime publication. Source gating is Forever-only and executed bundle assertions exclude retail epochs; other-profile runtime absence remains untested.

## How it works

- [Client profiles](../wiki/systems/client-profiles.md)
- [Running patch report](../wowforever-1.60.1.md)

## Implementation inventory

- `src/c_api/forever_edit_mode_enums.rs` — finite documented additions and metadata.
- `src/c_api/mod.rs` — profile-scoped compilation.
- `src/lua_api/env_init/enums.rs` — publication after shared enum initialization.
- `src/lua_api/globals/enum_data/combat_system.rs` — reused raid dispel definitions.

## Tests asserting this spec

- `tests/wowforever_edit_mode_enums.rs` — existing exact source values, metadata, actual preset and display consumers. Includes executed `forever_edit_mode_gamepad_consumers`; bounded PASS on runtime source136, not the new import revision.

## Known gaps (current cycle)

### Verified 70374 gamepad scope — 2026-10-15

[Central receipts](../baselines/README.md#verified-scoped-receipts--2026-10-15) establish actual compiled enum, preset, default-layout, display and gamepad-consumer PASS at runtime source136. The gamepad regression loads real cached TableUtil/preset/display Lua and asserts the concrete values above; external vendor cache inputs were not independently resealed.

Historical source9a sealed-binary probe `FOREVER_GAMEPAD_RED system=nil settings=nil` and preset line1272 failure remain RED history, not current results. Candidate applicability/source review alone was not runtime proof; the saved executed tests now close that bounded model gap.

- [x] Owning enum/preset/default-layout/display/gamepad model gate executed.
- [x] Actual post-enum startup captured; [baseline owner](../baselines/README.md#forever-lua-errors) records removed preset/display cascade and remaining FAIL, not clean startup.
- [ ] Full Edit Mode UI interactions, native parity, earlier-profile isolation and parent acceptance remain open.

Locale acquisition remains separate; see [locale owner](casc-loading.md#locale-checkpoint--october-15-2026).

## Out of scope

Native-game behavior, arbitrary enum generation, retail epoch activation, and vendor changes.
