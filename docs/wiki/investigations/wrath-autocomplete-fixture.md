# Wrath autocomplete fixture and legacy query boundary

F09's modern addon/mixin prerequisites prevented native autocomplete consumers from running. Load the active profile's committed FrameXML before testing the original singleton box, result buttons and editbox callbacks. [Contract](../../specs/wrath-autocomplete-results.md) owns requirements, test scope and gaps.

## Root cause and correction

Native `AutoComplete_Update` passes `(text, include, exclude, count, cursor)` and consumes separate returned strings. The existing global forwarded modern `(text, count, cursor, allowFullMatch, include, exclude)` arguments and returned a table. Source-bound fixture-only RED at `c4664700f` reached the original consumer: the ALL include mask failed signed-limit conversion at argument two. Direct string-result updating passed. An initial small-mask probe used a missing list alias and never queried; it is excluded from producer RED evidence. Its corrected template-based case awaits separate RED/GREEN proof.

The C API model now has a Wrath-specific query projection over existing friend GUIDs and active party identities. A real legacy-global boundary performs Lua 5.1 scalar coercion and pushes separate strings, including zero returns on misses. The modern namespace and non-Wrath global forwarder are separate; Wrath has no alternate modern-table result path. All 33 adapted original cases and five valid added cases pass at `db793e416`; the corrected extra probe and preservation gates remain pending.

## Coverage boundaries

Native fixtures retain shared query, result, selection, click, clear, template and dismissal assertions. Literal modern callback setters, popup mixins/pools, scroll widgets, tooltip/icon rows and independent popup instances remain original modern assertions, not synthesized native libraries. Native GlueXML availability remains unverified.

Build 12340 authority is the 2026-10-21 laptop relay `autocomplete-results-12340.md`, static reconstruction only. Cached Lua/XML expectations are INFERRED, not native-client execution. Guild/native-presence identity links, incomplete friend identities, whisper history, recency and UTF-8/case-helper edge parity remain outside this bounded model.

## Sources

- [Contract and test inventory](../../specs/wrath-autocomplete-results.md)
- [C API query projection](../../../src/c_api/c_auto_complete/wrath.rs)
- [Real legacy global](../../../src/lua_api/globals/real/wrath_autocomplete.rs)
- [Native fixture](../../../tests/common/wrath_autocomplete.rs)

## See Also

- [Addon loading](../../addon-loading-pipeline.md) — native core order before dependent consumers
- [Lua API](../../lua-api.md) — environment and binding boundary
