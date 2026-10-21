# Wrath autocomplete results

Wrath's native FrameXML autocomplete consumes global `GetAutoCompleteResults` as separate name strings, unlike the modern `C_AutoComplete` array of name/priority records. The native consumer is loaded through the active profile's committed FrameXML order. The modeled surface belongs to C API; its existing social-state implementation is `src/c_api/c_auto_complete.rs`.

Authority: laptop relay `autocomplete-results-12340.md`, 2026-10-21, static reconstruction of build 12340, not native execution. Other behavior remains INFERRED unless explicitly identified as cached FrameXML behavior.

## What it must do

- [ ] Wrath global query accepts `(text, include, exclude, numReturns, cursorPosition, allowFullMatch)` and supplies separate plain strings to the original `AutoComplete_Update` consumer.
- [ ] No match returns zero values and the original consumer hides its box.
- [ ] Positive limits cap results; zero and negative limits do not cap. Six matching names make the original five-button consumer display its disabled continuation sentinel.
- [ ] Native editbox callbacks preserve their original user-input, suffix-completion, cursor, selection and dismissal behavior without modern library replacements.
- [ ] Generic result/selection/clear behavior uses native result buttons; modern-only callback/mixin/pool/ScrollBox assertions remain intact for their existing consumers.
- [ ] Modern `C_AutoComplete` behavior and non-Wrath global publication remain unchanged.

Cached native FrameXML behavior differs from modern tests: selection starts at zero; focus loss hides without clearing the stored parent or selected index; an empty result update also retains the selected index until fresh results are shown; Tab cycles selection; no arrow-dispatch, guild-roster-event, priority-color, realm-appending or custom-click-callback handler exists in that consumer. Missing authenticated native GlueXML is an evidence gap, not proof that game autocomplete cannot run on glue screens.

## How it works

- [Lua API and environment](../lua-api.md)
- [Native addon loading](../addon-loading-pipeline.md)

## Implementation inventory

- `src/c_api/c_auto_complete.rs` — existing modern completion model; native query adapter pending original-consumer RED.
- `src/lua_api/workarounds/temporary/auto_complete_defaults.rs` — existing legacy forwarder and unmodeled realm defaults.
- `tests/common/wrath_autocomplete.rs` — loads native FrameXML and supplies six explicit social names without replacing vendor functions.

## Tests asserting this spec

- `tests/wrath_autocomplete_results.rs` — original-consumer query/callback and vararg/count boundaries.
- `tests/blizzard_auto_complete_*.rs` — F09 native result/editbox/button fixtures alongside unchanged modern assertions.
- `tests/c_autocomplete.rs` — existing modern namespace preservation.

## Known gaps (current cycle)

- [ ] Execute source-bound native consumer RED, then correct only the demonstrated Wrath query incompatibility.
- [ ] Complete targeted Wrath proof, stock empty Lua errors, Retail before/after preservation and warning-free checks.

## Out of scope

- Native execution parity: authority is static reconstruction only.
- Whisper history, native Battle.net presence nodes and recency timestamps: no suitable native backing state established for this batch.
- UTF-8/case-mapping and center-editing edge parity: native helper caveats remain; do not claim modern Rust character slicing reproduces them.
- New modern-library shims, independent native popup instances, vendor or rilua modifications, and unrelated legacy global cleanup.
