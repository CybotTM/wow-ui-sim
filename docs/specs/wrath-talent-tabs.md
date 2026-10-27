# Wrath talent tabs and rows

The Wrath talent panel reads scenario-supplied tabs, rows and prerequisites. The [F10 investigation](../wiki/investigations/wrath-spell-panels-f10.md) records the original consumer.

## What it must do

- [ ] Render supplied tab backgrounds and talent rows through the original show path.
- [ ] Reflect supplied tab counts and preview point totals, including two distinct values.
- [ ] Derive prerequisite satisfaction from the supplied prerequisite talent rank and preview rank.
- [ ] Return the comparison's missing-tab and missing-row tuples, and allow a supplied icon to be nil.
- [ ] Preserve the existing temporary defaults on non-Wrath profiles.

## How it works

- [Wrath F10 consumer investigation](../wiki/investigations/wrath-spell-panels-f10.md)

## Implementation inventory

- `src/lua_api/state/support_types.rs`: supplied tab, row and prerequisite records.
- `src/lua_api/globals/real/wrath_talent_groups.rs`: Wrath legacy query producers.
- `src/lua_api/workarounds/temporary/legacy_talent_skill_defaults.rs`: non-Wrath defaults.

## Tests asserting this spec

- `tests/common/wrath_spell_panels_f10.rs`: original show and supplied state.

## Known gaps (current cycle)

- [ ] Verify the aso 3.3.5a comparison boundaries; inspected and pet catalogs remain unsupplied scenario state.
- [ ] Verify the original render path and profile checks.

## Out of scope

- Talent learning, preview mutations, inspected and pet talent catalogs.

## Sources

Pinned Wrath `FrameXML/TalentFrameBase.lua:118,139,163,195,526,568,573` and `Blizzard_TalentUI/Blizzard_TalentUI.lua:596,604` are original consumer calls. Historical signatures are in `data/patch-api/sources/3.1.0-api-changes.txt:25–27,32` and `2.3.0-api-changes.txt:86,88`; the preview-points signature differs from the pinned consumer. The aso 3.3.5a comparison (`TalentInfo_Scripts.cpp:28–164,271–457`, `TalentInfo.cpp:220–226,306–354,719–742`) supports the tuple and prerequisite semantics. Its preview-points arguments are `(pet, group)`, matching the consumer and superseding the older signature.
