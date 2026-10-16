# XML StatusBar and Slider range/default values

## Contract

`minValue`, `maxValue` and `defaultValue` in parsed frame XML initialize StatusBar
and Slider state before lifecycle scripts. Both widgets share `FrameXml`; each
attribute resolves independently from most-derived template to instance, and an
omitted attribute preserves the existing value. Apply range before default value,
using the widget's bounded value. Direct XML, runtime template roots and template
children use the same application helper. Other widget types ignore these fields.

## Original consumer

Cached Retail `Blizzard_ObjectiveTracker/Blizzard_ObjectiveTrackerShared.xml:243`
defines the progress bar's StatusBar as 0..100/default0. The original
`ObjectiveTrackerProgressBarMixin:SetPercent` sets value and label from the supplied
percentage. A 50-percent snapshot must yield actual bar value50 and label `50%`,
not value1 caused by retaining the primitive default range0..1. No vendor setter
or fixture `SetMinMaxValues` replacement is allowed.

## Implementation and tests

`src/xml/types.rs` parses the three attributes. Shared
`globals/template/direct.rs::apply_xml_widget_values` runs in direct XML setup and
runtime template direct-property application.

`tests/xml_widget_values.rs` loads the original cached tracker template and asserts
range/default before invoking its original percent method. A second fixture covers
StatusBar independent overrides, Slider inheritance, pre-OnLoad visibility and
runtime CreateFrame inheritance for both widget types. Test commit `2ea2da694`
produced two actual Retail behavioral failures at saved `b8e9da11b`: cached range
0..1 instead of0..100; supplied initial values0/0 instead of50/15. GREEN,
StatusBar/Slider XML preservation and current Retail startup were pending at that
RED checkpoint. Saved `fe529b241` then passed both focused cases, including original
value50/label50%, independent overrides and pre-OnLoad/runtime inheritance. Saved
`9d1430bf3` preservation passed170 XML-template cases (one existing ignored tooltip
TODO),18 Slider cases and six original StatusBar UI cases (unique names; nested
child summaries do not add cases). Separate same-source Retail startup returned
`[]`, exit0. No whole-suite/native/all-profile claim.

See [XML template system](../wiki/systems/xml-template-system.md).
