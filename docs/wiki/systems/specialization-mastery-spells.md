# Specialization Mastery Spells

`C_SpecializationInfo.GetSpecializationMasterySpells(specIndex, isInspect, isPet)`
returns the mastery spell ID array for a player spec, modeled from
ChrSpecialization.db2 data. This retired the temporary empty-table shim that
previously backed the Character sheet Mastery tooltip.

## Content

### Data

`data/specializations.rs` (`SpecInfo.mastery_spell_ids`) carries the
`MasterySpellID_0`/`MasterySpellID_1` columns from ChrSpecialization.db2
(source: https://wago.tools/db2/ChrSpecialization/csv, zero entries dropped).
Most specs have exactly one mastery spell; initial/Adventurer specs have none.
Examples: Holy Paladin 183997 (Lightbringer), Protection Paladin 76671 (Divine
Bulwark), Retribution 267316 (Highlord's Judgment).

### API surface

- `c_spec_get_specialization_mastery_spells` in `src/c_api/c_spec.rs` maps the
  1-based spec index to the player's class specs via
  `crate::specializations::specs_for_class` and returns a Lua array of spell
  IDs (`push_number_array`).
- Consumers: `PaperDollFrame.lua` `Mastery_OnEnter` (Character sheet Mastery
  tooltip appends each spell via `GameTooltip:AppendInfo("GetSpellByID", ...)`),
  the Cata `Blizzard_TalentUI`, and the `Blizzard_DeprecatedSpecialization`
  wrapper that returns `masterySpells[1], masterySpells[2]` as the legacy
  two-value global.

### Retired shim

`src/lua_api/workarounds/temporary/specialization_mastery_defaults.rs`
(returned `{}`) was deleted along with its wiring in `workarounds/mod.rs` and
`workarounds/temporary/mod.rs`.

## Sources

- `data/specializations.rs` — generated spec data with mastery spell IDs
- `src/c_api/c_spec.rs` — namespace method
- `tests/blizzard_deprecated_specialization_loads.rs` — deprecated-wrapper coverage

## See Also

- [[deprecated-specialization-alias-identity]] — alias identity fix made in the same change
