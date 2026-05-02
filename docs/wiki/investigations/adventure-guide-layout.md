# Adventure Guide Layout

Adventure Guide Suggested Content cards can overlap text and buttons when runtime geometry queries resolve a resized frame without also resolving frames anchored to it.

## Content

Blizzard's `EJSuggestFrame_RefreshDisplay` resizes the title and description frames, then immediately reads `GetTop()` and `GetBottom()` to recenter each card's `centerDisplay`. The secondary cards chain their layout through anchors:

- `title`
- `description`, anchored to `title:BOTTOMLEFT`
- `button`, anchored to `description:BOTTOMLEFT`

The simulator's synchronous dirty-rect fast path resolved only the queried frame when Lua called geometry methods such as `GetBottom()`. If `title:SetHeight()` made the title dirty, `title:GetBottom()` returned the new title rect, but dependent siblings like `description` still kept their old cached rects. That made Adventure Guide text and buttons overlap even though the Blizzard XML anchors were correct.

The fix is for `SimState::resolve_rect_if_dirty` to use the anchor-dependent layout path for directly dirty frames, and for dirty ancestor resolution to recompute anchor dependents before clearing dirty roots. The hit-grid incremental queue must include those same transitive anchor dependents, because moved sibling buttons are not descendants of the resized anchor target. Regression coverage lives in `tests/layout_resolve.rs` as `querying_resized_anchor_target_updates_dependent_siblings`.

## Sources

- [state_render.rs](../../../src/lua_api/state_render.rs) - synchronous dirty rect resolution and anchor-dependent recomputation
- [layout_resolve.rs](../../../tests/layout_resolve.rs) - regression coverage for resized anchor targets updating sibling dependents
- [Blizzard_EncounterJournal.xml](</syncthing/World of Warcraft/_retail_/BlizzardInterfaceCode/Interface/AddOns/Blizzard_EncounterJournal/Mainline/Blizzard_EncounterJournal.xml>) - Adventure Journal Suggested Content card anchors
- [Blizzard_EncounterJournal.lua](</syncthing/World of Warcraft/_retail_/BlizzardInterfaceCode/Interface/AddOns/Blizzard_EncounterJournal/Mainline/Blizzard_EncounterJournal.lua>) - runtime text sizing and geometry queries

## See Also

- [[layout-system]] - anchor resolution and dirty layout behavior
- [[adventure-guide-boss-icons]] - separate Encounter Journal image fallback issue
