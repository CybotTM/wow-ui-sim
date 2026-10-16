# Supplied IconTextAndCurrencies widgets

`C_UIWidgetManager.GetIconTextAndCurrenciesWidgetVisualizationInfo` must expose explicit environment-owned supplied records to the unchanged cached Retail/PTR type-7 consumer. The existing subsystem lives in `src/c_api/ui_widget_manager/`; this slice has authored selector tests only, currently **UNEXECUTED**. No supplied-record implementation or behavioral acceptance is claimed. See [manager state and feeds](../wiki/systems/ui-widget-manager-state.md) for subsystem mechanics.

## What it must do

### Eligibility and public query

- [ ] Support Retail OR PTR builds, grounded in each profile's actual cached `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua` declaration and `Blizzard_UIWidgets/Blizzard_UIWidgetTemplateIconTextAndCurrencies.lua` registration/consumer. Both cached shared enum declarations give `Enum.UIWidgetVisualizationType.IconTextAndCurrencies = 7`. No other-profile acceptance is claimed.
- [ ] Require numeric `widgetID` using the existing manager required-number and secret policy, including contextual rejection of missing, explicit nil, string (including numeric string), boolean, table and function arguments. Preserve declared `SecretArguments = "AllowedWhenUntainted"` through that shared policy, with actual permitted/denied secret-wrapper coverage; do not introduce a new policy or integer/positive/range restrictions.
- [ ] An absent numeric ID returns exactly one nil. Numeric IDs never imply records or catalog membership.
- [ ] Project every supplied field below, all non-nil, preserving values and declared Lua types: exactly 20 outer fields and all 10 fields of every ordered `UIWidgetCurrencyInfo` row. Enum/fileID values are numeric scalars, texture kits are strings; do not clamp, synthesize defaults, or omit fields unused by Setup.

### Outer IconTextAndCurrenciesWidgetVisualizationInfo (20 fields)

| Exact field | Cached documented type |
| --- | --- |
| `shownState` | `WidgetShownState` |
| `enabledState` | `WidgetEnabledState` |
| `descriptionShownState` | `WidgetShownState` |
| `descriptionEnabledState` | `WidgetEnabledState` |
| `text` | `string` |
| `description` | `string` |
| `currencies` | `table` of `UIWidgetCurrencyInfo` |
| `tooltipLoc` | `UIWidgetTooltipLocation` |
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

### Nested UIWidgetCurrencyInfo (10 fields per row)

| Exact field | Cached documented type |
| --- | --- |
| `iconFileID` | `fileID` |
| `leadingText` | `string` |
| `text` | `string` |
| `tooltip` | `string` |
| `isCurrencyMaxed` | `bool` |
| `textFontType` | `UIWidgetFontType` |
| `textSizeType` | `UIWidgetTextSizeType` |
| `textEnabledState` | `WidgetEnabledState` |
| `iconSizeType` | `WidgetIconSizeType` |
| `updateAnimType` | `UIWidgetUpdateAnimType` |

### Snapshots and isolation

- [ ] Each query returns a fresh outer table, fresh `currencies` array and fresh row tables, preserving supplied order and empty arrays. Concrete complete multi-row records demonstrate all 20+10 fields, not merely fields read by Setup. Caller edits, row/array insertion/removal and retained snapshots cannot mutate supplied state or subsequent results.
- [ ] Replacements affect later queries only. Distinct numeric IDs, record absence/removal/restoration, and separate fresh `WowLuaEnv` instances remain isolated, using existing manager numeric identity rules without new selector restrictions.

### Normal feed and unchanged cached type-7 lifecycle

- [ ] Use existing normal `WowLuaEnv::replace_ui_widget_set`, `update_ui_widget` and `remove_ui_widget` feeds with explicit complete records and type-7 membership. State is observable before synchronous `UPDATE_UI_WIDGET`; wrong-type/absent-set updates reject atomically without mutation or notification. Membership removal/full-set replacement reconciles through `UPDATE_ALL_UI_WIDGETS` while independent visualization records remain queryable.
- [ ] Execute actual cached Retail/PTR manager and template activation through the real getter and supplied membership, without replacing provider, getter, Setup, pools or vendor code. Assert actual text, enabled/desaturated state (including cached Yellow-to-White text handling), conditional description visibility/content/enabled state, currency acquisition count/order and concrete observable supplied row content. Assert requested `Icon` texture-kit region name via cached `%s` policy only, not asset availability or pixels.
- [ ] Exercise ordinary supplied updates, two-widget supplied ordering/layout, full-set replacement, hidden release/shown reacquisition, membership removal and live unregister cleanup (pools, subscriptions and stopped updates). Allow normal cached deferred layout/removal before assertions; no vendor timing shortcuts. Ordinary updates retain acquisition `widgetTag`; reacquisition uses current supplied tag, while queries expose every replacement field. This is supplied-state lifecycle, not native animation/timing parity.

## How it works

- [Manager state and feeds](../wiki/systems/ui-widget-manager-state.md)
- [Event dispatch](../event-system.md)

## Implementation inventory

No IconTextAndCurrencies supplied-record/map/feed/getter is implemented. Existing seams, not completed feature implementations:

- `src/c_api/ui_widget_manager/queries.rs` — shared required-number/secret selector and namespace registration.
- `src/c_api/ui_widget_manager/feed.rs` — existing normal host feeds and state-before-event lifecycle.
- `src/c_api/ui_widget_manager/inputs.rs` — existing environment-owned supplied-input subsystem.
- `src/c_api/ui_widget_manager.rs` — existing public manager records and numeric identity.

## Tests asserting this spec

- `tests/ui_widget_icon_text_and_currencies_selector.rs` — Retail/PTR-eligible two-case public-selector regression, **UNEXECUTED**. Fresh environment per argument, existing `GetWidgetSetInfo` required-number control, public getter presence, absent numeric ID exactly-one-nil control and contextual required-number rejection. No supplied data, mocks, source-shape assertions or cached load. Early failure leaves later loop inputs unreached; no RED is claimed.

## Known gaps (current cycle)

- [ ] Establish the actual selector baseline and preserve its exact reached boundary before supplied-state implementation.
- [ ] Complete supplied 20+10-field records, projection, snapshots/isolation and shared secret-policy behavior remain UNIMPLEMENTED / UNEXECUTED for this slice.
- [ ] Normal-feed and unchanged cached type-7 lifecycle tests/acceptance remain unauthored and UNEXECUTED; selector-only coverage does not prove them.

## Out of scope

- Native execution/parity, reconstructed C++/wowdecomp/reconstruction reports, inferred native/default/catalog data: cached declarations and supplied state do not establish these.
- Asset existence, rendered pixels, animation execution/timing or native catalog defaults: names, numeric fileIDs and data projection are not proof.
- Other profiles, new secret/numeric identity policy and vendor overrides: outside this bounded Retail/PTR supplied-record contract.
