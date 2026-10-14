# TextureAndTextRow visualization

Contract for supplied type-15 row inputs and `C_UIWidgetManager.GetTextureAndTextRowVisualizationInfo`. Passive DTO, empty map, normal feed and tests are implemented under `src/c_api/ui_widget_manager/`; the modeled getter now projects complete fresh snapshots through shared namespace registration. Independent saved Retail and PTR runtime gates each confirm eight unique PASS cases, zero FAIL, exit 0; owning publication reconciliation and broader acceptance stay open. See [widget manager state](../wiki/systems/ui-widget-manager-state.md) for surrounding system design.

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
- `tests/ui_widget_texture_and_text_row.rs`: six authored API/snapshot/feed/cache cases executed in saved pre-projector Retail RED: absent-only control PASS, five FAILs. All six cases PASS in the first saved current-projector Retail runtime; independent saved-runtime gate confirms this bounded evidence.
- `tests/widget_row_records/fixtures.rs`: complete concrete row fixtures and recursive assertions.
- `tests/ui_widget_texture_and_text_row_security.rs`: two authored selector/security cases executed in saved pre-projector Retail RED, both FAIL; Retail/PTR file gate, additional retail-12-1-0 gate for host secrets. Required-number case failed at the missing argument; host-secret case failed at the initial public populated snapshot before secret wrappers were created. Those historical failure boundaries remain unchanged. Both cases PASS through later selector/security assertions in the first saved current-projector Retail runtime; independent saved-runtime gate confirms this bounded evidence.
- `tests/widget_two_records/helpers.rs`: existing unchanged shared numeric/host-secret helpers. Local row observer replaces shallow snapshot observation with deep checks.

## Known gaps (current cycle)
- [x] Saved pre-projector Retail RED at `89dca152d`: eight distinct tests, one absent-only nil-control PASS, seven FAILs, exit 101. Five direct populated cases failed the row-table assertion after the exactly-one-result check; required-number rejection failed at the missing argument. Nested one-case summaries are nonadditive. Independent saved-runtime audit confirms RED only; no new runtime execution.
- [ ] Cached provider/template checks, actual container/event registration, nonempty membership and absent-row checks were reached in saved RED. The actual-container-count flush failed after ordinary OnUpdate, but identical labels for the pre-injection count=0 and post-injection count=1 flushes leave the failing flush and count/timer predicate ambiguous. Injection and activation are not proved; explicit activation assertions and all later update/hide/reacquire/removal/unregister phases were unreached. Cached vendor bytes/provenance were not independently authenticated.
- [x] First saved current-projector Retail runtime observed on 2026-10-14: source `46f33b1c8`, producer `398c427fd`; main aggregate eight PASS, zero FAIL, exit 0. Eight nested one-case child summaries are NONADDITIVE, not eight additional tests. Independent gate read complete saved stdout/stderr/result, compile diagnostics, artifact metadata and source assertions. Receipt reuse establishes producer-digest/stat continuity, not independent executable-byte authentication.
- [x] Current saved Retail cached-consumer case completes absent-member registration/ordinary OnUpdate flush, normal-feed activation, in-place ordered-entry update, hidden row/nested-entry release, reacquisition without stale fixedWidth, membership removal retaining the record, and active unregister releasing row/entries while retaining supplied membership and record. These reached assertions do not resolve or replace the historical RED injection ambiguity above. Cached vendor bytes/provenance remain independently unauthenticated; native/render and bounded-out kit/animation/effect/timer behavior remain unproved.
- [x] Other seven current saved Retail cases complete absent-only nil, complete fresh nested snapshots/mutation isolation, whole replacement/retained snapshots, nullable-width transitions, two-environment isolation, required-number/exact-fractional selectors, and actual host-secret untainted access/tainted denial/wrapper preservation/caller-taint restoration/secure recovery assertions. This is bounded simulator behavior, not native parity.
- [ ] Compile context supplied for this observation: 741 records, six manifest warnings; peak 8,591,069,184 bytes exceeded the 8G high threshold by 1,134,592 bytes. Compilation is not behavioral acceptance.
- [x] Independent saved-Retail runtime gate PASS for eight unique cases; nested repeats remain nonadditive. Source/readability gate PASS separately covers projector and diagnostic-only follow-up.
- [x] Three unchanged Retail lifecycle sibling controls independently PASS at the same behavior source; separate from the eight row cases. Nested repeats are nonadditive.
- [ ] PTR no-run compilation at `7f44eb773` succeeded: 741 records, six manifest warnings, exit 0, 5m36s; peak 8,590,278,656 exceeded 8G high by 344,064 bytes, below 9G max. Historical fresh CPU/memory-pressure admission denial prevented sealing/runtime at that checkpoint; it is retained, not the current PTR runtime status.
- [x] Independent saved PTR first-runtime gate at `7f44eb773` confirms eight unique PASS, zero FAIL, exit 0; eight nested child summaries are nonadditive. Actual profile-cached consumer lifecycle and host-secret assertions pass, including live row/nested-entry unregister. This is saved-runtime inspection, not fresh execution, native parity, cache-byte or independent executable-byte authentication. Earlier Retail eight row cases plus three sibling controls remain separate evidence.
- [ ] Original owning 8.2.0 sweep at `c35ddb7ed` against the unchanged 52-gap fixture emitted 193 unique observed rows: 142 OK / 51 non-OK. Historical FAIL, exit 1, is retained: no new gaps; sole resolved/stale gap `wt-global-api-C_UIWidgetManager.GetTextureAndTextRowVisualizationInfo-134`, observed `raw=function; lookup=function`. `bb9ea16fe` removes only that fixture line, yielding the literal 51-entry expected-gap array. Reconciled rerun remains pending, not owning PASS. Expected-fixture reconciliation proves publication presence only, not the eight behavioral gates. See [owning sweep](patch-8-2-0-publication-sweep.md#row134-owning-observation--2026-10-14).
- [ ] Ordinary Retail CLI at `c35ddb7ed`, normal startup without addons/SavedVariables and with CASC off, has raw stdout `[]`, exit 0. Independent full diagnostic gate remains pending; this is not global startup-clean proof. Its source compile completed in 20.48s with 741 records, six manifest warnings and peak 4,762,271,744 bytes below 8G high; no causal inference from this measurement or comparison with earlier peaks.
- [ ] Format/default checks and whole-TEXT publication remain pending; native/render, cache-byte/executable-byte authentication and broader parent acceptance remain open.

## Out of scope
- Native and broader profile acceptance are not established by this bounded supplied-record model.
- Native catalog/default/enum inference, atlas/animation/effect/timer coverage and vendor changes; not part of this bounded scaffold.
