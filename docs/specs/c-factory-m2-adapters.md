# M2 equipment, spellbook and action startup inputs

M2 replaces individually required factory gaps with state-backed C API bindings or named temporary capabilities. [Follow-up notes](../wiki/systems/c-factory-startup-followup.md#m2-state-adapters) describe ownership; [nil-member publication](c-namespace-nil-members.md) remains unchanged.

## What it must do

- [ ] Outside Wrath, `C_PaperDollInfo.OffhandHasWeapon()` reads equipped slot 17 and the item catalog's weapon inventory types; absent/uncatalogued items and shields are not weapons.
- [ ] Retail/PTR/Forever `C_SpellBook.IsSpellKnownOrInSpellBook` projects existing known-or-book membership (including off-spec book presence), with existing pet spell membership for the pet bank. The current override model remains identity-only.
- [ ] Forever `C_ActionBar.UnregisterActionUIButton` removes the selected registered frame without removing other buttons.
- [ ] Forever `C_CooldownViewer.IsCooldownViewerAvailable` exposes the existing supplied availability and failure reason, not an inferred class/spec policy.
- [ ] Forever keyring capability, lifetime talent-point award history and targeting item-condition queries are named temporary false defaults until their backing systems exist.

## How it works

- [State adapters and missing capability systems](../wiki/systems/c-factory-startup-followup.md#m2-state-adapters)

## Implementation inventory

- `src/c_api/{c_paper_doll_info,c_spell_book,c_action_bar,c_cooldown_viewer}.rs` — state/catalog queries and action mutation.
- `src/lua_api/globals/action_bar_api/registration.rs` — existing C API binding registration.
- `src/lua_api/{state.rs,state/sim_state.rs}` — existing availability snapshot also available to Forever.
- `src/lua_api/workarounds/temporary/forever_startup_capabilities.rs` — three explicitly missing systems; profile-only bootstrap wiring in workaround modules.

## Tests asserting this spec

- `tests/cfactory_m2_adapters.rs` — generated-documentation contracts, concrete weapon/shield/absence, spellbook membership and button-removal behavior.

## Known gaps (current cycle)

- [ ] New test-only revision RED and targeted GREEN; combined stock startup matrix.

## Out of scope

Wrath behavior, inferred keyring visibility from key inventory, talent award inference from level/spent points, native targeting conditions and nonidentity spell overrides. No arbitrary C member factory.
