# Supplied TextWithSubtext widgets

The contract requires `C_UIWidgetManager.GetTextWithSubtextWidgetVisualizationInfo(widgetID)` to project explicit host-supplied type25 records from `src/c_api/ui_widget_manager/`. Contract grounded in the profile-scoped Retail/PTR cached `UIWidgetManagerDocumentation.lua` and unchanged `Blizzard_UIWidgetTemplateTextWithSubtext.lua`/`.xml`; records are synthetic inputs, not native catalog observations. See [widget manager state](../wiki/systems/ui-widget-manager-state.md) for architecture.

## What it must do

- [ ] Unknown numeric ID returns exactly one nil; missing/non-number selectors reject through the shared required-number policy. Fractional supplied IDs remain exact numeric keys.
- [ ] Populated getter returns exactly one fresh table with precisely the 27 documented non-nil keys, types and supplied values. Caller mutation cannot alter input or retained snapshots; replacement changes all fields without mutating old snapshots; environments remain isolated.
- [ ] Normal type25 feeds publish complete records and membership before synchronous notifications. Wrong visualization type or absent set rejects without changing stored records/membership or notifying subscribers. Membership removal retains the independent visualization record.
- [ ] Actual cached type25 registration and `UIWidgetContainerTemplate` activate both text lines through normal feeds, apply independent fonts/colors/justification, tooltip metadata, spacing, width, vertical line order, order index, tag and scale; updates preserve frame identity. Hidden releases, Shown reacquires; removal and active unregister release frames and subscriptions.
- [ ] With `retail-12-1-0`, real present/absent secret-number wrappers permit untainted queries, reject tainted queries before lookup, preserve wrappers/caller taint and leave supplied state unchanged. Public output stays non-secret.

## How it works

- [Widget manager state](../wiki/systems/ui-widget-manager-state.md)
- [Event dispatch](../event-system.md)

## Implementation inventory

- `src/c_api/ui_widget_manager/text_with_subtext_record.rs`: complete `TextWithSubtextRecord` (enum `u32`, numeric `f64`, strings `String`, timer `bool`).
- `src/c_api/ui_widget_manager/inputs.rs`: `UiWidgetInputs.text_with_subtext` keyed by `WidgetId`.
- `src/c_api/ui_widget_manager/feed.rs`: `WidgetVisualization::TextWithSubtext` normal type25 update feed.
- `src/c_api/ui_widget_manager.rs`: record exports and namespace subsystem; getter projection/registration remains parent integration work.

## Tests asserting this spec

- `tests/ui_widget_text_with_subtext.rs`: Retail/PTR suite entry.
- `tests/widget_subtext/queries.rs`: unknown control, selector rejection, exact27 snapshots/replacement, two-environment isolation.
- `tests/widget_subtext/feed.rs`: state-before-event and atomic feed rejection.
- `tests/widget_subtext/consumer.rs`: unchanged cached activation and lifecycle; Lua assertion files contain observations only.
- `tests/widget_subtext/security.rs`: actual host-secret boundary using shared helper.

## Known gaps (current cycle)

- [ ] Populated tests are integrated before getter implementation; execution and RED/GREEN evidence remain pending. Getter intentionally absent at scaffold revision `39a454d09` and test checkpoint `0a3a5ac20`.

## Out of scope

Native catalog invention, vendor patches, animation/asset execution and visual pixel parity. `widgetWidth` is projected independently; cached two-line consumer chooses width from positive `widgetSizeSetting`, not `widgetWidth`.
