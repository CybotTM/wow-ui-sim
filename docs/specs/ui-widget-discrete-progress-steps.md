# Supplied DiscreteProgressSteps visualization

Host-supplied DiscreteProgressSteps records belong to `src/c_api/ui_widget_manager/`; [widget manager architecture](../wiki/systems/ui-widget-manager-state.md) describes existing supplied-record behavior. Only DTO/map/test scaffolding is staged; no getter/registration/feed producer or executed RED yet. Cached Retail/PTR schema identifies visualization19 and all19 nonnil fields; native runtime/catalog/render parity remains unknown.

## What it must do

- [ ] `C_UIWidgetManager.GetDiscreteProgressStepsVisualizationInfo(number)` returns nil without supplied data, otherwise a fresh, complete 19-field cached-schema snapshot preserving supplied numbers/enums/strings/bool without inferred defaults or consumer clamping; caller mutation cannot alter retained snapshots or input state.
- [ ] Supplied replacement/removal and independent environments are observable; numeric IDs and non-number rejection match existing bars. Actual host-secret numbers are allowed when untainted, denied before supplied/absent lookup when tainted, with public recovery and public snapshot fields.
- [ ] Existing typed host update feed accepts visualization19, validates matching membership before mutation, stores the supplied record before synchronous notification, and preserves independent record lookup after membership removal.
- [ ] Unchanged active Retail/PTR cached manager/template consumes the supplied record and observes actual progress/numSteps geometry, incremental updates, hidden/restore, record/membership removal, pool cleanup and unregister/no reacquisition. Atlas application is established through real catalog-backed texture state, never injected data.

Complete snapshot fields: `shownState`, `tooltip`, `progressMin`, `progressMax`, `progressVal`, `numSteps`, `tooltipLoc`, `widgetSizeSetting`, `textureKit`, `frameTextureKit`, `hasTimer`, `orderIndex`, `widgetTag`, `inAnimType`, `outAnimType`, `widgetScale`, `layoutDirection`, `modelSceneLayer`, `scriptedAnimationEffectID`. Numeric key/snapshot policies follow the [manager contract](ui-widget-manager-state.md); no native acceptance range is inferred.

## How it works

- [Widget manager state](../wiki/systems/ui-widget-manager-state.md)

## Implementation inventory

- `src/c_api/ui_widget_manager/discrete_record.rs`: staged complete input DTO only.
- `src/c_api/ui_widget_manager/inputs.rs`: staged keyed input map only.
- `src/c_api/ui_widget_manager.rs`: staged public DTO export only.

## Tests asserting this spec

`tests/ui_widget_manager_discrete.rs` is automatically included by existing integration macro; its explicit child paths cover `tests/widget_discrete/{queries,consumer,security,fixtures}.rs`. Five cases: three query cases, one cached consumer lifecycle, one actual-secret case gated by `retail-12-1-0`. All are unrun; initial getter/supplied-record assertions precede cache/prerequisite work. Direct input seeding is not host-feed proof.

## Known gaps (current cycle)

- [ ] Compile candidates and retain genuine missing-getter/supplied-record RED before implementing getter/registration or host-feed variant/storage.
- [ ] Implement/verify producer only after RED; adapt host-feed tests separately. All cached lifecycle/atlas/primitive assertions currently unreached.

## Out of scope

Native/server catalog discovery and rendered/native parity are not established by host-supplied fixture records. No vendor behavior changes or fabricated texture catalogs.
