# Empty color override query

`C_ColorOverrides.GetColorOverrideInfo(overrideType)` reports no override until
an override registry is modeled. This temporary simulator policy preserves the
original ColorManager default-color path. See [Lua API architecture](../lua-api.md).

## What it must do

- [ ] Return exactly one nil for an override type when no override registry exists.
- [ ] Original Retail `ColorManager.GetColorDataForBagItemQuality` returns its
  default bag color for a quality with an entry in `ITEM_QUALITY_OVERRIDES`.
- [ ] Original CharacterFrame opening retains equipment icons and player title.

Cached `Blizzard_APIDocumentationGenerated/ColorOverridesDocumentation.lua`
declares the required `ColorOverride` argument and nullable `ColorOverrideInfo`
result. Nil is this simulator's empty-state policy, not native default proof.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/workarounds/temporary/color_defaults.rs`: temporary empty query;
  retire when a color override registry supplies the query through `c_api`.

## Tests asserting this spec

- `tests/blizzard_ui_unit.rs::original_color_manager_bag_quality_uses_default_without_override`
- Existing `character_panel_equipment_slots_match_inventory_or_background_textures`
  and `character_panel_title_text_matches_player_name` preservation cases.

## Known gaps (current cycle)

- [ ] Behavioral RED/GREEN and separate Retail startup observation.

## Out of scope

Override mutation, populated override records, general color registry fidelity,
native parity, and the separately reported rilua error-unwind defect.
