# Supplied DiscreteProgressSteps visualization

Host-supplied DiscreteProgressSteps records belong to `src/c_api/ui_widget_manager/`; [widget manager architecture](../wiki/systems/ui-widget-manager-state.md) describes existing supplied-record behavior. Complete snapshot producer and typed host-feed variant are implemented after genuine five-case RED at `42f206f66`: 0 PASS / 5 missing-supplied-record FAIL, before cache/atlas prerequisites. Current GREEN/startup/profile acceptance is pending. Cached Retail/PTR schema identifies visualization19 and all19 nonnil fields; native runtime/catalog/render parity remains unknown.

## What it must do

- [ ] `C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo(number)` returns nil without supplied data, otherwise a fresh, complete 19-field cached-schema snapshot preserving supplied numbers/enums/strings/bool without inferred defaults or consumer clamping; caller mutation cannot alter retained snapshots or input state.
- [ ] Supplied replacement/removal and independent environments are observable; numeric IDs and non-number rejection match existing bars. Actual host-secret numbers are allowed when untainted, denied before supplied/absent lookup when tainted, with public recovery and public snapshot fields.
- [ ] Existing typed host update feed accepts visualization19, validates entry/visualization type and supplied-set existence before mutation, stores the supplied record before synchronous notification, and preserves independent record lookup after membership removal.
- [ ] Unchanged active Retail/PTR cached manager/template consumes the supplied record and observes actual progress/numSteps geometry, incremental updates, hidden/restore, record/membership removal, pool cleanup and unregister/no reacquisition. Atlas application is established through real catalog-backed texture state, never injected data.

Complete snapshot fields: `shownState`, `tooltip`, `progressMin`, `progressMax`, `progressVal`, `numSteps`, `tooltipLoc`, `widgetSizeSetting`, `textureKit`, `frameTextureKit`, `hasTimer`, `orderIndex`, `widgetTag`, `inAnimType`, `outAnimType`, `widgetScale`, `layoutDirection`, `modelSceneLayer`, `scriptedAnimationEffectID`. Numeric key/snapshot policies follow the [manager contract](ui-widget-manager-state.md); no native acceptance range is inferred.

## How it works

- [Widget manager state](../wiki/systems/ui-widget-manager-state.md)

## Implementation inventory

- `src/c_api/ui_widget_manager/discrete_record.rs`: complete explicit input DTO.
- `src/c_api/ui_widget_manager/inputs.rs`: environment-owned keyed map, empty by default.
- `src/c_api/ui_widget_manager/discrete.rs`: authenticated numeric lookup and fresh complete19-field snapshot projection.
- `src/c_api/ui_widget_manager/queries.rs`: registers the query in the existing namespace.
- `src/c_api/ui_widget_manager/feed.rs`: typed visualization19 validation/storage before existing synchronous notification.
- `src/c_api/ui_widget_manager.rs`: module wiring and public DTO export.

## Tests asserting this spec

`tests/ui_widget_manager_discrete.rs` is automatically included by existing integration macro; explicit child paths cover `tests/widget_discrete/{queries,consumer,security,feed,fixtures}.rs`. Seven intended cases: query3, actual cached consumer1, actual-secret1 gated by `retail-12-1-0`, typed-feed2. At `42f206f66`, original five cases physically fail on missing supplied record; one timeout-child summary is duplicate, not a sixth case. Later fields/numeric/security/cache assertions were unreached. Original RED does not establish an independent feed RED. Query/security tests directly seed inputs; current consumer activation/update/hidden/restore/membership removal uses typed feeds, with explicit direct record deletion plus actual event for the separate record-removal boundary. Feed cases assert state visible inside real event callbacks and rejection without mutation/notification. Current seven-case GREEN remains unrun.

## Known gaps (current cycle)

- [ ] Complete independent TEXT-retention gate for [genuine42 RED packet](../../data/patch-api/evidence/ui-widget-discrete-progress-2026-10-13/README.md); compile exit0 with six inherited manifest deprecations and no source compiler messages, runtime exit101 is behavioral failure, not setup/timeout.
- [ ] Execute current query/feed/security and actual cached lifecycle GREEN, inspect full diagnostics, then applicable startup/check/PTR scopes. Current cached lifecycle/atlas/primitive assertions remain unexecuted after producer changes.

## Out of scope

Native/server catalog discovery and rendered/native parity are not established by host-supplied fixture records. No vendor behavior changes or fabricated texture catalogs.
