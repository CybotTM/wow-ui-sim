# PTR talent-award history

PTR exposes `C_SpecializationInfo.HasPlayerEarnedATalentPoint` from `src/c_api/c_spec.rs`. It projects supplied lifetime talent-award history. See the [housing consumer investigation](../wiki/systems/housing-spatial-presence.md#ptr-talent-award-startup-boundary).

## What it must do

- [x] Under PTR, return exactly one non-null boolean without arguments, as declared in pinned 70493 `SpecializationInfoDocumentation.lua:331–337`.
- [x] Project `TalentState.has_earned_talent_point` without mutation or level/rank inference. False is the empty-host-history policy, not a native default claim.
- [x] Preserve the original talent microbutton behavior on `PLAYER_ENTERING_WORLD`: false disables it and sets its unlock-level field; true enables it and clears that field; later false disables it again.
- [x] Publish `Constants.LevelConstsExposed.MIN_TALENT_LEVEL = 10` on PTR through the existing Lua constant-assignment mechanism. Pinned 12.1.7.70493 `LevelConstantsDocumentation.lua:6` names the declaration table `LevelConstsExposed_RetailAndClassic`; line12 declares `MIN_TALENT_LEVEL = 10`. The publication name is a separate, approved consumer-name decision: PTR `Blizzard_MicroMenu/Mainline/MainMenuBarMicroButtons.lua:808,1084`, `Blizzard_MicroMenu/Camelot/MainMenuBarMicroButtonsOverrides.lua:7`, and `Blizzard_Game/Mainline/EventImplementation.lua:863` read `Constants.LevelConstsExposed`. Siblings `MIN_RES_SICKNESS_LEVEL` and `MIN_ACHIEVEMENT_LEVEL` already use that name in `src/lua_api/globals/enum_data/constants_values.lua:187–188`. The value comes from the declaration, not the consumer. Retail and Forever documentation use the unsuffixed table name `LevelConstsExposed`. Forever 1.60.1.70532 declares the same value at line12 and already publishes it through `forever_finite_constants`; preserve that publication. Retail 12.1.5.70448 omits it, as do the inspected Mists/Era/Anniversary declarations; Wrath has no such documentation file. Do not expand publication to those profiles.
- [x] Keep Retail publication unchanged. Its pinned microbutton does not call this endpoint.

## How it works

- [Original world consumer and startup boundaries](../wiki/systems/housing-spatial-presence.md).

## Implementation inventory

- `src/c_api/c_spec.rs` — PTR ACTIVE gate and boolean projection.
- `src/lua_api/talent_state.rs` — supplied lifetime-history input alongside existing talent state.
- `src/lua_api/env_init/enums.rs` — PTR-only documented unlock-level constant, loaded beside `constants_values.lua`.

## Tests asserting this spec

- `tests/cf2_housing_plot.rs::ptr_talent_button_reaches_original_world_event` — original world-event RED and false/true/false button states.
- `tests/cf2_housing_plot.rs::original_world_event_branches_on_house_plot_absence_and_reentry` — complete housing path preservation.

## Known gaps (current cycle)

- [x] Original PTR false→true→false talent selector and housing eight-state selector pass after the separately committed documented constant. The missing housing producer sibling fails at pinned PTR `EventImplementation.lua:851`. Retail eight-state preservation and both zero-warning all-target checks pass.

## Out of scope

Native history acquisition, award rules, unlock eligibility, reset semantics and reconciliation with current talent ranks are unknown. The documented unlock-level constant does not establish those policies. No `C_ClassTalents` alias, Forever shim change or Retail expansion is part of this contract.
