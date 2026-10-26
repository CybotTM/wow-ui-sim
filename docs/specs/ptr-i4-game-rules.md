# PTR I4 GameRules supplied inputs

PTR 12.1.7.70493 exposes seven GameRules queries/mutations from supplied state, plus finite source-published enum tables. This bounded simulator contract does not establish native capability policy. See the [I4 investigation](../wiki/investigations/patch-12-1-7-api-audit.md).

## What it must do

- [ ] Publish `IsHardcoreActive`, `IsSelfFoundAllowed`, `IsSDHDToggleEnabled` and `AccountHasSDEnabled` only on PTR in this packet. Each returns one boolean from its distinct supplied capability field. Initial false values mean no positive input was supplied, not that native accounts lack the capability.
- [ ] PTR `IsWoWHack` returns exactly one non-nil boolean: true only when the existing supplied `active_game_mode` equals `Enum.GameMode.WoWHack`. Original MicroMenu OnLoad must initialize `addedButtons` with mode-specific spell/talent buttons; original UIErrorsFrame OnLoad must enable energy errors and use its WoWHack anchor only in that mode. Independent environments and Retail publication remain unchanged.
- [ ] `GetForeverExperiencePreset` returns the supplied nullable enum integer. No preset is inferred from the active profile or game mode. The pinned documentation declares Classic=0 and Modern=1.
- [ ] `SetSDHDToggleValue(setToSD, usageLocation)` updates supplied account SD selection and records the supplied documented origin. It returns no values. This contract does not simulate native telemetry or reload behavior.
- [ ] Original selected Blizzard consumers react to changed supplied inputs. Other profiles and independent environments retain their state and surfaces.
- [ ] Publish RecentAlliesInteractionCategoryFilter and its metadata exactly as the pinned documentation declares. Original RecentAllies labels remain reachable. Publish GameRules argument/result enums required by the selected consumers from the same pinned declarations, without extrapolating values.

## How it works

- [I4 source and runtime investigation](../wiki/investigations/patch-12-1-7-api-audit.md)

## Implementation inventory

- `src/lua_api/sim_substates/mod.rs`: supplied GameRules capability fields.
- `src/c_api/ptr_i4_game_rules.rs`: PTR finite declarations, capability queries and supplied SD mutation. Preset query is pending.
- `src/c_api/mod.rs`: module declaration.
- `src/lua_api/globals/game_rules.rs`: existing registration calls the C API packet.

## Tests asserting this spec

- `tests/ptr_i4_batch2.rs`: original selected game/glue consumer behavior, post-consumer return type/arity checks and Retail publication preservation. Mutation is checked before the additional setter arity call.

## Known gaps (current cycle)

- [ ] Capability queries and SD mutation are implemented after original-line RED; their GREEN and the preset query remain pending. Enum-only replay proves setter RED at Barbershop line273 and hardcore RED at popup line3512. The RecentAllies labels selector passes with the documented enum.
- [ ] Runtime GREEN, stock error comparison and profile warning/format gates remain pending.
- [ ] `IsWoWHack` original-consumer REDs reach GameRulesUtil207 through MicroMenu OnLoad and UIErrorsFrame12. Its producer compares the existing supplied mode with the existing GameMode enum declaration; GREEN remains pending. The pinned declaration is `GameRulesDocumentation.lua:349–354`.

## Out of scope

Native realm/account capability derivation, HD/SD eligibility, telemetry effects, persistent settings, unknown argument restrictions, secret inputs and native quota policies are not established. No SetForeverExperiencePreset, SmartNavigation import or additional RecentAllies API is added. The wider I4 gap inventory remains incomplete.
