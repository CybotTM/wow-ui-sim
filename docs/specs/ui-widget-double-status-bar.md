# UI Widget DoubleStatusBar

Draft owning contract for `C_UIWidgetManager.GetDoubleStatusBarWidgetVisualizationInfo` and its actual cached `UIWidgetTemplateDoubleStatusBar` consumer. Existing supplied-set state/feed lives in `src/c_api/ui_widget_manager/`; earlier inspection at `2c843ad2c45fae07d3098a7fb39eacbde85c770d` conflated missing modeled backing with an absent runtime getter. First retail execution passed callable absent-record getters and safe provider skipping as existing behavior; populated records remain unproved by this evidence. See [manager architecture](../wiki/systems/ui-widget-manager-state.md).

## What it must do

### Initial absent-record boundary

- [x] Publish the exact getter as a callable function. For a numeric ID with no supplied visualization, return exactly one nil result (not zero results, a placeholder table, or extra values).
- [x] With the actual profile-cached provider registered for visualization type 3, a normal supplied set containing that type and no visualization record must register through `UIWidgetContainerTemplate:RegisterForWidgetSet` without errors or acquiring a widget frame.
- [x] Replacing that supplied set through the normal host feed must preserve membership and safely reconcile without acquiring a frame or recording Lua errors.

### Documented record frontier (not covered by the initial tests)

Retail and PTR cached `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua` name `DoubleStatusBarWidgetVisualizationInfo` as the getter's single nilable result and `widgetID` as a required number. The following **26 fields**, each non-nilable in those generated docs, define the grounded future full-record frontier; no default values or numeric enum policies are inferred. Secret argument metadata is `AllowedWhenUntainted`; native parity and security behavior are not tested here.

| Field | Generated API type |
|---|---|
| `shownState` | `WidgetShownState` |
| `leftBarMin` | `number` |
| `leftBarMax` | `number` |
| `leftBarValue` | `number` |
| `leftBarTooltip` | `string` |
| `rightBarMin` | `number` |
| `rightBarMax` | `number` |
| `rightBarValue` | `number` |
| `rightBarTooltip` | `string` |
| `barValueTextType` | `StatusBarValueTextType` |
| `text` | `string` |
| `leftBarTooltipLoc` | `UIWidgetTooltipLocation` |
| `rightBarTooltipLoc` | `UIWidgetTooltipLocation` |
| `fillMotionType` | `UIWidgetMotionType` |
| `widgetSizeSetting` | `number` |
| `textureKit` | `textureKit` |
| `frameTextureKit` | `textureKit` |
| `hasTimer` | `bool` |
| `orderIndex` | `number` |
| `widgetTag` | `string` |
| `inAnimType` | `WidgetAnimationType` |
| `outAnimType` | `WidgetAnimationType` |
| `widgetScale` | `UIWidgetScale` |
| `layoutDirection` | `UIWidgetLayoutDirection` |
| `modelSceneLayer` | `UIWidgetModelSceneLayer` |
| `scriptedAnimationEffectID` | `number` |

Scaffolds `2b13a7c57` and `bd48d6ce8` add explicit 26-field `DoubleStatusBarRecord`, an initially empty environment-local keyed map and type3 typed host-feed storage. Those scaffold revisions did not add projection or modeled registration. `8645aaf0a` adds the complete 26-field fresh snapshot projection; `a62c81768` wires its module and registration. Selectors use shared `read_numeric_id` policy; no new validation, defaults, inferred records or demo seeds. Implemented but behaviorally unverified.

## How it works

- [UI widget manager state](../wiki/systems/ui-widget-manager-state.md)
- [Addon loading pipeline](../addon-loading-pipeline.md)
- [Event dispatch](../event-system.md)

## Implementation inventory

Source inspection: `src/lua_api/env_init/runtime_surface_bootstrap.lua:64–79` defines a generic namespace `__index` that logs missing keys, creates a function returning nil and caches it with `rawset`; lines112–129 attach that namespace metatable. That historical inspection predates modeled registration at `a62c81768`; `double_status_bar.rs` now supplies the producer. Callable absent-ID behavior therefore does not establish a modeled getter or native API availability.

- `src/c_api/ui_widget_manager/inputs.rs` — existing `WidgetSetEntry` and `WidgetSetRecord`, independent supplied membership.
- `src/c_api/ui_widget_manager/feed.rs` — existing `replace_ui_widget_set` publishes normal full reconciliation after storing membership.
- `src/c_api/ui_widget_manager/queries.rs` — existing set queries; missing modeled backing does not imply an absent runtime getter. Initial retail execution proves callable absent-record behavior only.
- Profile cache `Blizzard_UIWidgets/Blizzard_UIWidgetTemplateDoubleStatusBar.lua` — actual provider calls the exact getter and filters missing/hidden records; not repo-owned code.
- Profile cache `Blizzard_UIWidgets/Blizzard_UIWidgetManager.lua` — actual container registration, provider dispatch and frame acquisition.

## Tests asserting this spec

`tests/ui_widget_double_status_bar.rs`:

- `double_status_bar_getter_is_callable_and_absent_id_returns_exactly_one_nil` — exact callable getter and result count/value.
- `cached_double_status_bar_consumer_skips_member_without_visualization` — real closure, nonempty membership/provider guards, actual registration and ordinary host-feed reconciliation, zero acquisition/errors.

First retail execution on 2026-10-14 passed both tests for this model: **4 primary PASS across both models**, zero failures. Nested timeout-harness repeats are nonadditive. Callability, exactly one nil for absent records and safe cached-provider skipping passed as existing behavior on first execution. No genuine RED was observed, despite the artifact directory name.

Independent saved-outcome verification: `/home/osso/.local/state/wow-ui-sim/verification/widget-two-api-initial-outcome-current/report.md` and `proof.json`, source `758dfdf5d8c49a1ca8d49fa518c01c7da9e65a1a`. It confirms four distinct Retail PASS, no genuine RED, and no populated-record coverage; timeout child summaries are not additional cases. This is bounded saved-evidence verification, not broad acceptance.

Current authored revision `fb7a1c243` contains twelve cases across both modules (six each): the two initial cases per module plus complete fresh/replaced snapshots, environment isolation, numeric/secret selector policy and actual cached populated feed lifecycle. Genuine observed Retail RED at `fb7a1c24370fca440a68709e0a70b5a0e9c428f8`: 12 primary outcomes, **4 PASS / 8 FAIL**. Supplied queries return nil, missing-selector acceptance fails rejection expectations, and cached consumers acquire no populated widget; later lifecycle assertions are unreached. Receipt: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/widget-two-populated-retail-red-20261014T161619034690Z/runtime-two-widget-populated-red`. Independent agent4706 classification remains pending. Subsequent complete projection and registration are source implementation only: no behavioral GREEN yet; producer compile/GREEN work is running, not accepted evidence.

## Known gaps (current cycle)

- [x] Execute both initial retail boundaries and preserve the PASS artifact above; no genuine RED observed.
- [x] Independent saved-outcome report confirms initial four PASS only.
- [x] Observe genuine populated Retail RED: four primary PASS / eight FAIL; independent classification pending.
- [x] Implement complete modeled getter projection/registration at `8645aaf0a` + `a62c81768`; source-only, not behavioral GREEN.
- [ ] Verify implemented producer and populated cached lifecycle; native and earlier-profile availability remain UNKNOWN.
- [ ] Populated full-record projection and visible activation/update/removal remain unproved by this initial evidence; the documented field frontier is unchanged.

## Out of scope

- Inferred catalog/default records and vendor behavior replacement are not part of the supplied-state contract.
- These initial boundaries do not prove native parity, rendered fidelity, other-profile behavior, or broader parent acceptance; those remain separate evidence gaps.
