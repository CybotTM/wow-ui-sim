# Supplied Spacer visualization

`C_UIWidgetManager.GetSpacerVisualizationInfo` is the type-22 query declared by the cached Retail/PTR API sources and used by their Spacer consumer. [Manager architecture](../wiki/systems/ui-widget-manager-state.md) owns the surrounding state/feed system.

## Required behavior

- [ ] Follow the shared simulator required-number selector boundary: reject missing, nil, string, numeric-string, boolean, table, function and thread arguments with a number-required error. Preserve supplied fractional keys without integer coercion or invented ID ranges.
- [ ] Return one nil for absent records and fresh complete snapshots for explicitly supplied records. No inferred catalogs, default records or fabricated data.
- [ ] Keep inputs environment-local; normal feeds publish record and membership before synchronous notifications. Preserve independent records on membership removal.
- [ ] Validate actual host-secret access and cached consumer activation, updates, hidden release, reacquisition, removal and applicable live unregister cleanup at their real profile scopes.

## Current implementation and proof

- `tests/ui_widget_spacer_selector_red.rs` authors only the first required-number case, using the actual runtime query without new production records or fake providers.
- Passive `SpacerRecord`, initially empty environment-local `spacers` map and normal typed type-22 feed are now implemented. Every declared field must be supplied; no record defaults or catalog seeding. No modeled getter/projector/query registration exists yet. Full populated/lifecycle cases are being adopted separately.
- First Retail selector RED at `1ffab60b4` independently confirms one unique FAIL, exit 101: callable getter accepted the missing selector. Seven later invalid forms and error-text assertion were unreached; nested summaries are nonadditive. Prior zero-launch capacity denials remain historical. Its no-run build: 741 records, six manifest warnings, 33.25s, terminal exit 0, peak 6,010,707,968 bytes below 8G high/9G max. This selector RED predates passive production inputs; populated RED remains required before a modeled getter, and no populated/lifecycle/native proof is claimed.
- Required acceptance remains scoped Retail/PTR behavior, source/readability, format/default check, ordinary startup diagnostics and applicable publication evidence. Source/compilation/callability alone does not establish populated behavior.

## Limits

Strict selector rejection is the existing simulator sibling policy, not native parity proof. Native catalogs, animation/effect/timer/atlas behavior, nonfinite dimension policy, rendering and broader parent acceptance are not established by this slice. No vendor/cache/Wowless modification is authorized by it.
