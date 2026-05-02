# Adventure Guide Disabled Tabs

Adventure Guide boss/model side tabs are intentionally visible before a boss is selected, but Blizzard greys them out by desaturating their textures. The simulator rendered those disabled tabs in full color because model stub registration overwrote the real `SetDesaturated` and `SetDesaturation` texture methods on the shared frame metatable.

## Root Cause

All frame methods share one rilua `FrameRef` metatable. Texture methods registered real desaturation setters first, but the later model-method registration installed no-op `SetDesaturated` / `SetDesaturation` stubs for intentionally unsupported 3D model behavior. Since the metatable is shared, those model stubs replaced the texture implementation for every widget type.

The visible symptom was the Adventure Guide abilities tab staying saturated even though `EncounterJournal_DisplayInstance` correctly called `EncounterJournal_SetTabEnabled(..., false)`. The backing button was disabled and correctly ignored clicks, but the visual state looked active.

## Fix

Keep the generic texture desaturation methods as the shared implementation and do not register model no-op stubs with the same names. Desaturation remains a normal frame visual flag, so model widgets can still absorb the call without needing a model-specific override.

Regression coverage lives in `tests/methods_texture.rs`:

- `test_set_desaturated_updates_texture_state`
- `test_button_texture_child_desaturation_persists`

## Sources

- [Blizzard_EncounterJournal.lua](../../../../../../../../World%20of%20Warcraft/_retail_/BlizzardInterfaceCode/Interface/AddOns/Blizzard_EncounterJournal/Mainline/Blizzard_EncounterJournal.lua) — `EncounterJournal_DisplayInstance` disables boss/model tabs and `EncounterJournal_SetTabEnabled` desaturates tab textures.
- [model.rs](../../../src/lua_api/frame/methods/widgets/model.rs) — model method registration collision.
- [texture/color.rs](../../../src/lua_api/frame/methods/widgets/texture/color.rs) — generic desaturation setters.

## See Also

- [[adventure-guide-layout]] — related Adventure Guide visual/debug history.
- [[method-dispatch-refactor]] — shared frame method registration context.
