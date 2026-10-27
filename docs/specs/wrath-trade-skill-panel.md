# Wrath trade-skill panel

This contract covers the legacy Wrath trade-skill panel and the existing simulator recipe catalogue.

## What it must do

- [ ] The selected recipe survives changes in display order; the displayed index follows the selected recipe.
- [ ] Name filtering is case-insensitive, changes the visible count and display order, and emits `TRADE_SKILL_FILTER_UPDATE` on change.
- [ ] The original panel opens Blacksmithing and can select and filter recipes.

The pinned Wrath panel calls `CloseTradeSkill` on hide at `Blizzard_TradeSkillUI/Blizzard_TradeSkillUI.xml:983`. The pinned Wrath consumer uses the selection at `Blizzard_TradeSkillUI/Blizzard_TradeSkillUI.lua:31-34,247-272`, the list and recipe queries at `:96-235,275-404`, and the name and level setters at `:423-449`. The 3.3.5a comparison (TradeSkillFrame_Scripts.cpp:159-183,766-771; TradeSkillFrame.cpp:755-775,1265-1294,1369-1389) describes selection and filtering, not a capture of the pinned client. The `SetTradeSkillItemNameFilter` historical listing is `data/patch-api/sources/2.1.0-api-changes.txt:77`.

The existing Blacksmithing catalogue is a simulator scenario, not a native character inventory. Its empty cooldown/tool maps, unlinked state, zero repeat count, empty name/level filters, and disabled makeable filter are best-guess scenario defaults; native defaults are unknown. The catalogue supplies no category header rows. Header selection and expansion are scenario defaults with no rows; the original consumer's header branches are not used for this catalogue. Full inventory-dependent craftability is not modeled here.

For a valid recipe, `GetTradeSkillInfo` returns `true, nil, 0, 0, false, 0, 0, 0` after its first three values as best-guess scenario defaults. An invalid `GetTradeSkillIcon` index returns file ID 134400 as a best-guess scenario default. Native values for these outputs are unknown; these defaults are not native measurements.

## How it works

- [Profession API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/globals/missing_surface/professions.rs`: profile publication.
- `src/lua_api/globals/missing_surface/professions_legacy_trade_skill.rs`: legacy queries and actions.
- `src/lua_api/state_types/crafting.rs`: scenario state.

## Tests asserting this spec

- `tests/test_showuipanel_toggles.rs`: original panel, selection and name-filter interactions.

## Known gaps (current cycle)

- [ ] Native scenario defaults and complete catalogue semantics require independent evidence.

## Out of scope

- Full tradeskill lifecycle, populated category headers, and native inventory synchronization.
