# Crafting Cast Bar

Crafting consumed reagents and produced output, but it did not create player casting state or fire the spellcast start event. Blizzard's professions page activates `OverlayPlayerCastingBarFrame` before calling `C_TradeSkillUI.CraftRecipe`, then relies on `UNIT_SPELLCAST_START` plus `UnitCastingInfo("player")` to show the bar.

## Content

Root cause: the simulator implemented `C_TradeSkillUI.CraftRecipe` as an immediate inventory transaction only. That made the data layer work, but left `SimState.casting` empty and never notified cast-bar listeners, so the professions UI had no spellbar to render while crafting.

Fix: successful recipe crafts now start a player cast using the recipe ID/name, fire `UNIT_SPELLCAST_START` for `"player"`, and emit `UPDATE_TRADESKILL_CAST_STOPPED` when that cast completes. The default profession cast duration is 2.0 seconds; using the 1.5 second GCD-style duration made crafting finish visibly too fast. Inventory mutation remains immediate for now, preserving existing crafting tests.

Regression coverage lives in `tests/test_crafting.rs`: `craft_recipe_starts_player_cast_for_cast_bar` asserts both `UnitCastingInfo("player")` and the start event, while `craft_recipe_uses_profession_cast_duration` locks the 2.0 second cast duration.

## Sources

- [profession_crafting.rs](../../../src/lua_api/globals/missing_surface/profession_crafting.rs) — `C_TradeSkillUI.CraftRecipe` backing behavior
- [casting.rs](../../../src/iced_app/casting.rs) — cast completion event emission
- [test_crafting.rs](../../../tests/test_crafting.rs) — crafting API regression coverage

## See Also

- [[lua-api]] — Lua global/API compatibility surface
- [[event-system]] — frame event dispatch mechanics
- [[on-update-dirty]] — cast-bar rendering updates during active casts
