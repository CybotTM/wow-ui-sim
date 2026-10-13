# Supplied BulletTextList visualization

BulletTextList type10 belongs to the `src/c_api/ui_widget_manager/` supplied-state subsystem; the [widget manager wiki](../wiki/systems/ui-widget-manager-state.md) owns how records, cached consumers and events work. Contract sources are the active Retail/PTR caches under `~/.cache/wow-ui-sim/blizzard-ui/<profile>/AddOns/`: `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua` declares the getter and fifteen fields; `Blizzard_UIWidgets/Blizzard_UIWidgetTemplateBulletTextList.lua` registers type10 and consumes pooled lines. These are cached declarations/consumer source, not native runtime proof. This stage adds tests/spec only: no BulletTextList DTO, map, feed or implemented getter. Empty host backing is planned simulator policy, not production state yet.

## What it must do

### Public query and supplied-state policy

- [ ] Retail/PTR public `C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo(widgetID)` rejects supplied nil, string, bool and table selectors with contextual `C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo: widgetID must be a number` error text. A failure merely caused by an absent function is not successful validation.
- [ ] Follow existing numeric `WidgetId` lookup policy without truncation, coercion or a new native ID-range rule. Empty-by-default supplied backing and unknown numeric IDs return exactly one nil. **Simulator arity/default policy**, not native proof; cached declarations establish a nullable result, not observed native unknown-ID behavior.
- [ ] Preserve existing numeric `AllowedWhenUntainted` policy: real host-secret NUMBER selectors are accepted when untainted and denied before present/absent lookup when tainted. Both cached getter declarations explicitly say `SecretArguments = "AllowedWhenUntainted"`; actual BulletTextList execution remains unproved. Do not invent a new secrecy policy.

### Complete fifteen-field snapshots

Both cached structures declare every field non-nil. Preserve supplied values and declared Lua types without synthesizing defaults or consumer clamping. Enum declarations establish field types, not native admissibility of arbitrary enum values.

- [ ] `shownState`: preserve supplied `WidgetShownState` numeric value.
- [ ] `enabledState`: preserve supplied `WidgetEnabledState` numeric value.
- [ ] `lines`: return a fresh ordered, dense one-based array of supplied strings, preserving order, empty strings and an empty list.
- [ ] `widgetSizeSetting`: preserve supplied number.
- [ ] `textureKit`: preserve supplied texture-kit string.
- [ ] `frameTextureKit`: preserve supplied texture-kit string.
- [ ] `hasTimer`: preserve supplied bool.
- [ ] `orderIndex`: preserve supplied number.
- [ ] `widgetTag`: preserve supplied string.
- [ ] `inAnimType`: preserve supplied `WidgetAnimationType` numeric value.
- [ ] `outAnimType`: preserve supplied `WidgetAnimationType` numeric value.
- [ ] `widgetScale`: preserve supplied `UIWidgetScale` numeric value.
- [ ] `layoutDirection`: preserve supplied `UIWidgetLayoutDirection` numeric value.
- [ ] `modelSceneLayer`: preserve supplied `UIWidgetModelSceneLayer` numeric value.
- [ ] `scriptedAnimationEffectID`: preserve supplied number.

### Freshness, isolation and feed ordering

- [ ] Every present-record query returns a fresh row and fresh `lines` table. Caller mutations to either cannot change host input, later queries or retained snapshots.
- [ ] Clone supplied records so replacement/deletion cannot alter retained rows or lines. Identical IDs in separate `WowLuaEnv` instances remain independent; no normal demo seeds are added.
- [ ] Extend the existing typed host feed for type10 only after genuine public-API RED. Validate supplied-set existence and entry/visualization type before mutation or notification; rejection leaves state/events unchanged.
- [ ] Store complete visualization and membership before synchronous `UPDATE_UI_WIDGET`; real event callbacks must observe updated data through the public getter. Membership removal reconciles through existing `UPDATE_ALL_UI_WIDGETS` ordering while independent visualization remains queryable, following [existing supplied-state policy](ui-widget-manager-state.md).

### Real cached pooled-line consumer

These requirements are driven by unchanged Retail/PTR cached source, not evidence that simulator execution already works.

- [ ] Activate type10 through the real cached manager and `UIWidgetContainerTemplate`, not a fake consumer or replacement global. Actual template `OnLoad` creates its line pool; `Setup` displays supplied strings in order and propagates enabled state to Bullet/Text.
- [ ] Incremental supplied updates run actual cached `Setup`: old active lines are released, new ordered lines acquired, stale lines removed, and visible text/enabled state updated. Observe real pool/count/frame behavior after actual OnUpdate flushing where required.
- [ ] Hidden visualization is suppressed by the cached type10 query adapter; visible restoration reactivates it through real manager reconciliation.
- [ ] Membership removal releases the active widget and pooled lines through real cached reset/removal behavior; retained query snapshots remain unchanged. Assert committed-state event ordering before downstream consumer outcomes.

## How it works

- [Widget manager architecture](../wiki/systems/ui-widget-manager-state.md)
- [Existing supplied-state/feed contract](ui-widget-manager-state.md)
- [Event dispatch](../event-system.md)
- [Cached addon loading](../addon-loading-pipeline.md)

## Implementation inventory

- `tests/ui_widget_bullet_text_list_api_red.rs`: two Retail/PTR-only public-API cases using real `WowLuaEnv::new`/`eval`; no host producer, stubs, mocks or replaced globals.
- `src/c_api/ui_widget_manager/queries.rs`: existing numeric validation/query owner; inspected source has no BulletTextList getter registration.
- `src/c_api/ui_widget_manager/inputs.rs`: existing environment-owned backing; inspected source has no BulletTextList map/DTO.
- `src/c_api/ui_widget_manager/feed.rs`: existing typed producer owner; BulletTextList extension is planned, not implemented in this stage.
- Cached Retail/PTR API documentation and BulletTextList Lua/XML templates: external declaration/consumer owners, unchanged. No new production file exists for this feature.

## Tests asserting this spec

`tests/ui_widget_bullet_text_list_api_red.rs` contains exactly two cases, under `cfg(any(feature = "client-retail", feature = "client-ptr"))`:

1. `bullet_text_list_public_api_rejects_non_numeric_widget_id_with_context`: supplied nil/string/bool/table calls must reject at the contextual widgetID-number boundary. First observable assertion is public validation, before cache loading, host input or consumer prerequisites. Missing-function errors cannot satisfy the required message.
2. `bullet_text_list_empty_default_unknown_numeric_id_returns_one_nil_simulator_policy`: unknown numeric IDs must return exactly one nil in an untouched environment. Explicit simulator arity/default policy, not native proof.

Helper follows existing TextWithState/Discrete phase-tagged Lua error capture, using `eval::<()>` as required here. No source-substring or raw-registration assertions. Compilation and execution were unrun at the original test/spec checkpoint `3bd83b432`. Initial API RED is now independently classified for Retail at that source and PTR at `13a818ba5`: each executed two cases, 1 PASS / 1 FAIL, exit101. Empty unknown numeric one-nil policy passed; supplied nil was accepted, failing `nil: expected widgetID validation error`. String/bool/table and subsequent message checks were unreached. This proves the initial validation/default boundary only; fields, freshness, real-secret policy, feed and cached consumer have no new behavioral proof.

## Known gaps (current cycle)

- [x] Initial Retail/PTR API RED executed before production edits: each 1 PASS / 1 FAIL, exit101, independently classified; exact boundary above.
- [ ] Add empty host DTO/backing, complete getter projection and type10 feed only after that RED boundary is observed.
- [ ] Add supplied-record field/freshness/isolation/security/feed tests and real cached pooled-line lifecycle proof; the current two cases do not cover these requirements.
- [ ] Establish bounded GREEN and integration proof separately; no native/render/full-widget acceptance follows from cached declarations or this test-stage source.

## Out of scope

Other widget types, vendor/cache/Wowless modifications, new normal/demo seeds and new transport are excluded from this slice. Native catalogs, native unknown-ID arity, native enum admissibility, native event/allocation/lifetime parity, rendered/pixel parity and full widget-system acceptance remain unproved. No production DTO/state/feed/getter edits, build/integration harness changes or runtime execution belong to this test/spec-only stage.
