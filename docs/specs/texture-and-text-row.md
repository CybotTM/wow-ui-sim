# TextureAndTextRow visualization

Contract for supplied type-15 row inputs and `C_UIWidgetManager.GetTextureAndTextRowVisualizationInfo`. Passive DTO, empty map, normal feed and tests are implemented under `src/c_api/ui_widget_manager/`; the modeled getter now projects complete fresh snapshots through shared namespace registration. GREEN behavioral acceptance remains unverified. See [widget manager state](../wiki/systems/ui-widget-manager-state.md) for surrounding system design.

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
- `src/c_api/ui_widget_manager.rs`: row module declaration and DTO exports.
- `src/c_api/ui_widget_manager/inputs.rs`: initially empty texture_and_text_row map.
- `src/c_api/ui_widget_manager/feed.rs`: row variant, type 15 and normal supply arm.
- `src/c_api/ui_widget_manager/texture_and_text_row.rs`: complete fresh outer/array/entry projection; nullable width omitted only when absent.
- `src/c_api/ui_widget_manager/queries.rs`: shared required-number/secret selector policy and namespace registration.

## Tests asserting this spec
- `tests/ui_widget_texture_and_text_row.rs`: six authored API/snapshot/feed/cache cases executed in saved pre-projector Retail RED: absent-only control PASS, five FAILs. Current-projector GREEN remains unexecuted.
- `tests/widget_row_records/fixtures.rs`: complete concrete row fixtures and recursive assertions.
- `tests/ui_widget_texture_and_text_row_security.rs`: two authored selector/security cases executed in saved pre-projector Retail RED, both FAIL; Retail/PTR file gate, additional retail-12-1-0 gate for host secrets. Required-number case failed at the missing argument; host-secret case failed at the initial public populated snapshot before secret wrappers were created. Later selector/security assertions and current-projector GREEN remain unexecuted.
- `tests/widget_two_records/helpers.rs`: existing unchanged shared numeric/host-secret helpers. Local row observer replaces shallow snapshot observation with deep checks.

## Known gaps (current cycle)
- [x] Saved pre-projector Retail RED at `89dca152d`: eight distinct tests, one absent-only nil-control PASS, seven FAILs, exit 101. Five direct populated cases failed the row-table assertion after the exactly-one-result check; required-number rejection failed at the missing argument. Nested one-case summaries are nonadditive. Independent saved-runtime audit confirms RED only; no new runtime execution.
- [ ] Cached provider/template checks, actual container/event registration, nonempty membership and absent-row checks were reached in saved RED. The actual-container-count flush failed after ordinary OnUpdate, but identical labels for the pre-injection count=0 and post-injection count=1 flushes leave the failing flush and count/timer predicate ambiguous. Injection and activation are not proved; explicit activation assertions and all later update/hide/reacquire/removal/unregister phases were unreached. Cached vendor bytes/provenance were not independently authenticated.
- [ ] Current-projector GREEN is unexecuted; cached activation/lifecycle, PTR, format/default check, normal diagnostics and owning publication acceptance remain pending. Saved pre-projector compilation and the absent-only nil control do not prove current populated behavior or runtime acceptance.

## Out of scope
- Native and broader profile acceptance are not established by this bounded supplied-record model.
- Native catalog/default/enum inference, atlas/animation/effect/timer coverage and vendor changes; not part of this bounded scaffold.
