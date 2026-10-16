# Era/Anniversary startup producers — A

Finite data and existing modeled inputs required by the installed Vanilla startup consumers. Implementation lives in `src/c_api/classic_startup_data.rs`, `src/c_api/class_catalog.rs`, and existing color/PvP/plain-global producers. Source-derived simulator policies, not native-client parity.

## What it must do

- [ ] R1: publish the existing Mists Classic cast-bar palette before Era/Anniversary consumers construct their type table; preserve RGB and encoded zero alpha. Leave Wrath unchanged pending a consuming source.
- [ ] R3: publish Poor=0 and nine quality values (cached ItemQualitiesDocumentation.lua:6-21), supply finite C_Item RGB from existing quality data and Enum.BagIndex.Bank=-1 from cached BagIndexConstantsDocumentation.lua:29, and allow original Constants/UIParent/UnitPopup/Bank consumers to finish; do not inject downstream bank/difficulty aliases.
- [ ] R4: expose nine compact Vanilla enumeration entries with canonical IDs 1–5,7–9,11; C_CreatureInfo uses canonical IDs, not compact indexes. Preserve Druid, exclude later classes, and return nil for unsupported Vanilla IDs. Localized class lists use the same catalog. Non-Vanilla canonical lookup behavior remains unchanged; Mists retains eleven enumeration entries.
- [ ] R6: expose existing honor state to Era/Anniversary. Yesterday projects three values including a supplied contribution (default zero); Mists retains its two-value tuple. Honor shown state follows the supplied capability, default disabled as before.
- [ ] R12: register only the existing raid-marker capability accessor on Era/Anniversary, retaining the existing enabled simulator default and supplied true/false inputs. Do not register the other Retail-only globals. Mists keeps its existing disabled compatibility default.
- [ ] R13: publish ClassicPreWrath.MAX_QUEST_WATCHES=5 from cached QuestConstantsDocumentation.lua:154-158; let Vanilla Constants derive MAX_WATCHABLE_QUESTS and the original consumer enforce capacity/replacement.

## How it works

- [Client profiles](../wiki/systems/client-profiles.md)
- [Lua API](../lua-api.md)

## Implementation inventory

- `src/c_api/classic_startup_data.rs`: finite quality API and quality/quest constants, initialized before vendor loading.
- `src/c_api/class_catalog.rs`: selected enumeration and canonical ID lookup.
- `src/lua_api/globals/font_strings_collection/colors.rs`: shared Classic palette selection.
- `src/lua_api/globals/pvp_probes.rs`, `state_types/pvp.rs`: honor registration and contribution projection.
- `src/lua_api/globals/real/publication_12_0_0.rs`: shared capability accessor; unrelated registrations remain Retail-gated.
- `src/lua_api/globals/register.rs`, `real/mod.rs`, `state.rs`, `state/sim_state.rs`: shared capability storage and narrow registration.
- `src/era/compat_bootstrap.rs`, `compat_bootstrap.lua`: finite producer initialization and retired yesterday shim.
- `src/lua_api/globals/social_probes.rs`, `missing_surface/creature_info.rs`: global enumeration and canonical C API consumer of the same model.

## Tests asserting this spec

- `tests/classic_startup_a.rs`: six original-cache consumer cases, extended with supplied false capability and nonzero yesterday contribution after their Rust storage is available. Existing `tests/mists_class_colors.rs`, `tests/p1200_plain_globals.rs` cover preservation but have not been executed here.

## Known gaps (current cycle)

- [ ] All tests are staged, unbuilt and unexecuted. Runtime RED/GREEN, startup reduction, type checking and integration acceptance remain required.
- [ ] Consumer cases load original Lua chunks and drive handlers on real widgets or presentation fixtures, not the whole startup/XML dependency graph. Full startup-order coverage remains open.
- [ ] Watch login uses the existing three static legacy watches. Five/six capacity uses supplied UI-owned watch records, not six state-backed legacy quest memberships.
- [ ] Native Vanilla class-enumeration indexing and native palette RGBA are not established. Compact enumeration is an explicit simulator choice grounded in the shipped nine-class set.
- [ ] Existing item RGB values are reused, not newly acquired native measurements. Invalid-quality behavior is outside this startup contract.

## Out of scope

Sibling B's roots and sites; Blizzard Lua modifications; native parity; baselines or asset manifests; manual unsupported LOD loads; expanding Wrath's class/honor surface.
