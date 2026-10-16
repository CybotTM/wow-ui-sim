# HorizontalCurrencies supplied state

Private supplied-state contract for `src/c_api/ui_widget_manager/`; architecture
owner: [manager state](../wiki/systems/ui-widget-manager-state.md).
2026-10-16. Staged contract; no compiler, test, RED/GREEN or startup proof.
This expands the selector-only preparation without rewriting historical evidence.

## What it must do

- [ ] Accept explicit complete supplied records only; fresh environments start
  with an empty map. Reuse existing `UiWidgetCurrencyRecord`, not a second row type.
- [ ] Public `GetHorizontalCurrenciesWidgetVisualizationInfo(widgetID)` requires
  a number and returns exactly one nullable record. A populated record returns a
  table containing all 15 outer fields and every ordered 10-field currency row.
  Missing required input and invalid types are separate controls. Diagnostic
  wording follows existing simulator required-number behavior, not native proof.
- [ ] Each query returns fresh outer/array/row tables. Caller edits, row insertion,
  deletion, whole replacement, later host updates and record removal do not mutate
  retained snapshots. Fractional keys remain distinct; environments stay isolated.
- [ ] Empty currencies are a fresh empty table, not omitted/nil. Restore ordered
  rows without filling an older empty snapshot. No invented widget-ID range policy.
- [ ] `SecretArguments = "AllowedWhenUntainted"`: test actual wrapped present/absent
  numbers securely, deny both under tainted callers before lookup, preserve wrappers,
  caller taint and host state, then recover securely. Security case is compiled only
  when the repository's `retail-12-1-0` feature exposes the existing test helpers.
- [ ] Type9 feed publishes complete membership and record before synchronous
  `UPDATE_UI_WIDGET`; invalid visualization type or missing set changes neither
  records/membership nor events. Removing membership retains independent records.
- [ ] Actual cached Retail/PTR HorizontalCurrencies Lua/XML, loaded through the
  existing addon closure and unmodified manager provider, consumes normal feeds.
  Cover activation, in-place reordered row update, empty rows, both cached width
  branches, sorting, full-set replacement, Hidden release, Shown reacquisition,
  membership removal, unregister while live, nested pool/subscription cleanup and
  non-reactivation after unregister. Assert fields/anchors/requested texture IDs,
  not rendered pixels or asset availability.

`HorizontalCurrenciesWidgetVisualizationInfo`:

| Requirement | Field | Declared type | Nilable |
| --- | --- | --- | --- |
| [ ] | `shownState` | `WidgetShownState` | false |
| [ ] | `currencies` | `table`, InnerType `UIWidgetCurrencyInfo` | false |
| [ ] | `tooltipLoc` | `UIWidgetTooltipLocation` | false |
| [ ] | `widgetSizeSetting` | `number` | false |
| [ ] | `textureKit` | `textureKit` | false |
| [ ] | `frameTextureKit` | `textureKit` | false |
| [ ] | `hasTimer` | `bool` | false |
| [ ] | `orderIndex` | `number` | false |
| [ ] | `widgetTag` | `string` | false |
| [ ] | `inAnimType` | `WidgetAnimationType` | false |
| [ ] | `outAnimType` | `WidgetAnimationType` | false |
| [ ] | `widgetScale` | `UIWidgetScale` | false |
| [ ] | `layoutDirection` | `UIWidgetLayoutDirection` | false |
| [ ] | `modelSceneLayer` | `UIWidgetModelSceneLayer` | false |
| [ ] | `scriptedAnimationEffectID` | `number` | false |

Ordered `UIWidgetCurrencyInfo` rows:

| Requirement | Field | Declared type | Nilable |
| --- | --- | --- | --- |
| [ ] | `iconFileID` | `fileID` | false |
| [ ] | `leadingText` | `string` | false |
| [ ] | `text` | `string` | false |
| [ ] | `tooltip` | `string` | false |
| [ ] | `isCurrencyMaxed` | `bool` | false |
| [ ] | `textFontType` | `UIWidgetFontType` | false |
| [ ] | `textSizeType` | `UIWidgetTextSizeType` | false |
| [ ] | `textEnabledState` | `WidgetEnabledState` | false |
| [ ] | `iconSizeType` | `WidgetIconSizeType` | false |
| [ ] | `updateAnimType` | `UIWidgetUpdateAnimType` | false |

## How it works

- [Manager state](../wiki/systems/ui-widget-manager-state.md).
- Public cached sources, independently inspected in both
  `~/.cache/wow-ui-sim/blizzard-ui/{retail,ptr}/AddOns/`:
  `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua:170–181`
  (selector/security/return), `:1488–1507` (15 outer fields), `:2044–2057`
  (10 row fields); `UIWidgetManagerSharedDocumentation.lua:144` (type9).
- `Blizzard_UIWidgets/Blizzard_UIWidgetTemplateHorizontalCurrencies.lua:1–8`
  calls the public getter, filters Hidden and registers the real template;
  `:12–44` acquires ordered pooled rows, uses Yellow text and Red/White tooltips,
  anchors the first row to the widget and later rows with a 10-unit gap;
  `:47–53` determines dimensions, `:56–63` creates/resets the nested pool.
  The actual XML `Blizzard_UIWidgetTemplateHorizontalCurrencies.xml:3–5`
  declares the virtual template. Shared `Blizzard_UIWidgetTemplateBase.lua:325–403`
  consumes all row text, tooltip, icon, font/size and animation inputs.
- Fresh snapshot, exact supplied identity, feed atomicity and isolation are
  simulator contracts, not behavior inferred from native execution. Cached
  consumers ground the public integration assertions; metadata grounds fields.

## Implementation inventory

Private passive proposals, not canonical implementation:

- `src/c_api/ui_widget_manager/horizontal_currencies_record.rs`: complete 15-field
  input, reusing existing `UiWidgetCurrencyRecord`.
- `src/c_api/ui_widget_manager.rs`: passive module declaration and public export.
- `src/c_api/ui_widget_manager/inputs.rs`: empty environment-owned record map.
- `src/c_api/ui_widget_manager/feed.rs`: variant, type9 validation and supply arm.

Patch `00-tests.patch` contains only this passive wiring and tests. Existing
missing-member namespace behavior returns nil, so populated tests must fail at
the table boundary. Patch `10-producer.patch` replaces that missing behavior with
explicit getter registration and complete fresh projection; adopt it only after
recording the populated behavioral RED.

## Tests asserting this spec

`tests/ui_widget_horizontal_currencies_passive.rs`: host-only storage/feed controls.
`tests/ui_widget_horizontal_currencies_selector.rs`: missing/type selector controls.
`tests/ui_widget_horizontal_currencies.rs` and `tests/widget_horizontal_currencies/`:
full query, feed, gated security and cached lifecycle assertions.

13 intended Rust test functions with security feature enabled (12 without). Two passive, two selector, five query, two feed,
one feature-gated security, one composite cached lifecycle. Helpers/loops/profiles
are not extra independent cases. Every execution boundary is currently UNREACHED.

## Known gaps (current cycle)

- [ ] Main adoption, formatting, compiler checks and populated behavioral RED.
- [ ] Producer patch adoption after RED, subsequent bounded Retail/PTR behavior,
  startup and other required gates. Producer is staged separately, not executed.

## Out of scope

Native/catalog/default/domain inference; reconstructed sources; vendor mutation;
new currency schema; async/timer/fade/effect activation or pixel/asset acceptance;
all-profile/full-suite/parent-goal completion. Timer/fade/effect/update-animation
values occur only in projection/feed tests without a cached consumer.
