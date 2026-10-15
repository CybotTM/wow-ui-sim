# Forever Edit Mode enums

Forever publishes finite additions from its `EditModeManagerConstantsDocumentation.lua` through `src/c_api/forever_edit_mode_enums.rs`. Existing 1.60.1.69913 proof is historical; the pending 1.60.1.70374 gamepad extension is tracked below. See [client profiles](../wiki/systems/client-profiles.md#forever-gamepad-edit-mode--pending-2026-10-15).

## What it must do

- [x] Publish the six missing GroupFinder, MainActionBarEndCap and SwingTimer setting/index/visibility namespaces and their source metadata.
- [x] Publish documented LossOfControl and RaidWarning settings, Gamepad preset 2, systems 24–29, account settings 33–37, minimap IconScale, micro-menu DeprecatedEyeSize and unit-frame DebuffIconSize/BuffIconSize with updated metadata.
- [x] Reuse the existing RaidDispelOverlayType values and metadata.
- [x] Execute the historical 69913 Forever preset and setting-display consumers without enum lookup failures; this does not establish 70374 compatibility.
- [ ] Under `client-wowforever`, publish 70374 `EditModeSystem.GamepadMainActionBar=30` and system metadata `MinValue=0`, `MaxValue=30`, `NumValues=31`.
- [ ] Publish `EditModeGamepadMainActionBarSetting`: `Compact=0`, `ShowEmptyBars=1`, `ShowButtonPrompts=2`, `ShowHighlight=3`, `ShouldExpand=4`; metadata `MinValue=0`, `MaxValue=4`, `NumValues=5`.
- [ ] Execute actual 70374 Camelot preset constants, Mainline presets and shared setting-display consumers: `EDIT_MODE_GAMEPAD_SYSTEM_MAP[30].settings` has exactly five entries, values `0/1/0/1/1` at keys `0..4`, BOTTOM/UIParent anchor with offsetY 110, and five checkbox descriptors mapping settings `0..4`.
- [ ] Keep these additions scoped to Forever without enabling a retail epoch.

## How it works

- [Client profiles](../wiki/systems/client-profiles.md)
- [Running patch report](../wowforever-1.60.1.md)

## Implementation inventory

- `src/c_api/forever_edit_mode_enums.rs` — finite documented additions and metadata.
- `src/c_api/mod.rs` — profile-scoped compilation.
- `src/lua_api/env_init/enums.rs` — publication after shared enum initialization.
- `src/lua_api/globals/enum_data/combat_system.rs` — reused raid dispel definitions.

## Tests asserting this spec

- `tests/wowforever_edit_mode_enums.rs` — existing exact source values, metadata, actual preset and display consumers. Includes `forever_edit_mode_gamepad_consumers`; AUTHORED, NOT GREEN. Source integrated at `136748f0b`; independent source/saved-RED verification PASS, compilation/execution pending.

## Known gaps (current cycle)

### Pending 70374 gamepad candidate — 2026-10-15

Private handoff: `/home/osso/.local/state/wow-ui-sim/handoff/forever-gamepad-enum-fix-next0700/` (`candidate.patch`, `REPORT.md`, `manifest.json`). Candidate extends the existing profile-scoped C API module, not shared enums or vendor Lua. Authority: actual 70374 `EditModeManagerConstantsDocumentation.lua` lines 485–497 and 731–768; preset lines 1272–1291 and display lines 1517–1553.

| Scope | Evidence | Status |
|---|---|---|
| Sealed normal binary, source receipt `9a6ea924e`, not candidate | Saved probe: `FOREVER_GAMEPAD_RED system=nil settings=nil`; actual vendor preset line 1272 failed | Actual RED. Service exit 0 is not success: exec-Lua assertion errors do not propagate through dump-tree exit status. |
| Candidate patch | Saved `git apply --check` | Applicability PASS only at recorded inputs; no runtime proof. |
| Candidate regression | Rust test loads actual vendor consumers and asserts the contract above | AUTHORED, NOT GREEN; compilation/execution pending. |

- [ ] Run owning Forever enum/preset/default-layout/display tests on the committed source, including the new consumer regression.
- [ ] Run normal startup Lua-error gate on the resulting binary before claiming any error reduction. Main commit `d9f051156` records corrected70374 manifest and actual failing 166-record/238-occurrence baseline; neither baseline nor counts are changed here.

Independent locale-acquisition runtime verification reportedly PASS (4,434 files, hashes match); this is acquisition proof only, not a startup fix. See the [locale proof owner](casc-loading.md#locale-checkpoint--october-15-2026); no locale evidence duplicated here.

- [ ] Full Edit Mode UI loading and interactions remain parent integration work.
- [ ] Earlier-profile isolation verification remains pending.

## Out of scope

Native-game behavior, arbitrary enum generation, retail epoch activation, and vendor changes.
