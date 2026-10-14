# TextureAndTextRow visualization

Contract for supplied type-15 row inputs and `C_UIWidgetManager.GetTextureAndTextRowVisualizationInfo`. Passive DTO, empty map, normal feed and tests are implemented under `src/c_api/ui_widget_manager/`; the modeled getter/projector is not implemented yet. All behavioral requirements below remain unverified, including those with authored tests. See [widget manager state](../wiki/systems/ui-widget-manager-state.md) for surrounding system design.

## What it must do

### API shape and snapshots
- [ ] Require a numeric widgetID and return exactly one nullable row result; absent records must return one nil even with supplied membership. No invented record, enum, catalog, ID range, integer coercion, or default filler.
- [ ] Return exactly these 18 outer fields: shownState, entries, textSizeType, groupAlignment, fixedWidth, tooltipLoc, widgetSizeSetting, textureKit, frameTextureKit, hasTimer, orderIndex, widgetTag, inAnimType, outAnimType, widgetScale, layoutDirection, modelSceneLayer, scriptedAnimationEffectID. Only fixedWidth is nullable; None omits its Lua key, Some(0) remains zero, and Some(fraction) retains its value.
- [ ] Preserve ordered entries with exactly two required string fields per entry: text and tooltip. Return fresh outer, entries-array, and entry tables per query, with no cross-entry or cross-query aliases. Caller nested mutation must not alter stored inputs, retained snapshots, or later results; whole replacement must preserve retained results.

### Feed and authentic cached lifecycle
- [ ] Ordinary type-15 host feed supplies complete row state and set membership before synchronous notification; environment inputs are isolated. Membership removal retains independently queryable records.
- [ ] Authentic profile-cached Retail and PTR providers/containers must handle absent membership rendering, activation, in-place ordered-entry update, hidden release, reacquisition without stale fixedWidth, membership removal, and active unregister releasing the live row/nested entry pool while retaining supplied membership and records. Deferred layout must run through ordinary OnUpdate, not substituted events/providers/vendor overrides. The authored case bounds texture kits, animations, effects, and timers out.

### Selector and security behavior
- [ ] Follow the existing shared required-number boundary for missing/nil/string/numeric-string/bool/table/function/thread arguments. Preserve exact fractional keys; non-supplied numeric selectors return one nil. Queries must not mutate row/set inputs.
- [ ] For actual host-secret NUMBER selectors, allow untainted queries and deny tainted present/absent queries before lookup; preserve wrappers and cumulative caller taint, deny tainted unwrapping, restore the secure outer caller, and allow ordinary numeric queries and subsequent secure recovery. Returned outer/array/entry tables, keys, and values must be public and deeply fresh. This is the existing simulator sibling contract, not native parity proof.

## How it works
- [Widget manager supplied-state system](../wiki/systems/ui-widget-manager-state.md)
- [Event system](../event-system.md)

## Implementation inventory
- `src/c_api/ui_widget_manager/texture_and_text_row_record.rs`: complete outer/entry DTOs, no Default record.
- `src/c_api/ui_widget_manager.rs`: row module and DTO exports only.
- `src/c_api/ui_widget_manager/inputs.rs`: initially empty texture_and_text_row map.
- `src/c_api/ui_widget_manager/feed.rs`: row variant, type 15 and supply arm only.

## Tests asserting this spec
- `tests/ui_widget_texture_and_text_row.rs`: six authored, unexecuted API/snapshot/feed/cache cases.
- `tests/widget_row_records/fixtures.rs`: complete concrete row fixtures and recursive assertions.
- `tests/ui_widget_texture_and_text_row_security.rs`: two authored, unexecuted selector/security cases; Retail/PTR file gate, additional retail-12-1-0 gate for host secrets.
- `tests/widget_two_records/helpers.rs`: existing unchanged shared numeric/host-secret helpers. Local row observer replaces shallow snapshot observation with deep checks.

## Known gaps (current cycle)
- [ ] Freshly admit and observe genuine behavioral RED before modeled getter/projector work. Compile/discovery/cfg errors or omitted cases are not RED; callable nil-returning compatibility behavior is not populated implementation.
- [ ] Modeled getter/projector/shared registration and behavior/compilation/discovery/RED/GREEN acceptance remain pending. Private source-stage checks are not runtime proof.

## Out of scope
- Producer/getter/API-registration implementation before genuine RED; requires later main-owned work.
- Native catalog/default/enum inference, atlas/animation/effect/timer coverage and vendor changes; not part of this bounded scaffold.
