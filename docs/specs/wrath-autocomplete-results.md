# Wrath autocomplete results

Wrath's native FrameXML autocomplete consumes global `GetAutoCompleteResults` as separate name strings, unlike the modern `C_AutoComplete` array of name/priority records. The native consumer is loaded through the active profile's committed FrameXML order. The modeled surface belongs to C API; its existing social-state implementation is `src/c_api/c_auto_complete.rs`.

Authority: laptop relay `autocomplete-results-12340.md`, 2026-10-21, static reconstruction of build 12340, not native execution. Other behavior remains INFERRED unless explicitly identified as cached FrameXML behavior.

## What it must do

- [x] Wrath global query accepts `(text, include, exclude, numReturns, cursorPosition, allowFullMatch)` and supplies separate plain strings to the original `AutoComplete_Update` consumer.
- [x] No match returns zero values and the original consumer hides its box.
- [x] Positive limits cap results; zero and negative limits do not cap. Six matching names make the original five-button consumer display its disabled continuation sentinel.
- [x] Native editbox callbacks preserve their original user-input, suffix-completion, cursor, selection and dismissal behavior without modern library replacements.
- [x] Generic result/selection/clear behavior uses native result buttons; modern-only callback/mixin/pool/ScrollBox assertions remain intact for their existing consumers.
- [x] Modern `C_AutoComplete` behavior and Retail original consumers remain unchanged in the bounded 67-case before/after selection. Other non-Wrath publication paths are source-preserved, not separately runtime-verified.
- [ ] Native projection uses existing friend GUIDs and active, cached party identities; merges flags by GUID, excludes self, and never deduplicates different identities by equal display text.
- [ ] Context filtering honors the context CVar (ANY included bit, exclusion veto); numeric cursor defaults, full-match truthiness and ASCII center-edit matching follow the relay's bounded contract.

Cached native FrameXML behavior differs from modern tests: selection starts at zero; focus loss hides without clearing the stored parent or selected index; an empty result update also retains the selected index until fresh results are shown; Tab cycles selection; no arrow-dispatch, guild-roster-event, priority-color, realm-appending or custom-click-callback handler exists in that consumer. Missing authenticated native GlueXML is an evidence gap, not proof that game autocomplete cannot run on glue screens.

## How it works

- [Native autocomplete diagnosis](../wiki/investigations/wrath-autocomplete-fixture.md)
- [Lua API and environment](../lua-api.md)
- [Native addon loading](../addon-loading-pipeline.md)

## Implementation inventory

- `src/c_api/c_auto_complete.rs` — unchanged modern completion model and Wrath model module declaration.
- `src/c_api/c_auto_complete/wrath.rs` — GUID-backed candidate projection and native matching/filter/limit behavior.
- `src/lua_api/globals/real/wrath_autocomplete.rs` — legacy scalar coercion and vararg string boundary.
- `src/lua_api/globals/real/wrath_startup_queries.rs` — Wrath-only registration.
- `src/lua_api/workarounds/temporary/auto_complete_defaults.rs` — non-Wrath result forwarder and existing unmodeled realm defaults; no alternate modern result path under Wrath.
- `tests/common/wrath_autocomplete.rs` — loads native FrameXML and supplies six explicit social names without replacing vendor functions.

## Tests asserting this spec

- `tests/wrath_autocomplete_results.rs` — original-consumer query/callback and vararg/count boundaries.
- `tests/blizzard_auto_complete_*.rs` — F09 native result/editbox/button fixtures alongside unchanged modern assertions.
- `tests/c_autocomplete.rs` — existing modern namespace preservation.

Fixture-only source `c4664700f`: eleven positive-count Wrath cases executed. Nine valid query/callback RED failures report the unsigned include mask being interpreted as an `i32` limit at argument two; direct native result updating passes. One additional failure is excluded from producer RED: its missing `AUTOCOMPLETE_LIST.FRIEND` alias prevented the query. The corrected case uses `AUTOCOMPLETE_LIST_TEMPLATES.FRIEND`: genuine original-consumer RED at `14f3c5d8b` (plain friend name was nil), GREEN at `c522c9d87`, with byte-identical corrected test source.

Producer source `db793e416`: all 33 adapted original cases, five other added native cases and seven existing modern namespace cases pass. Their source is unchanged at `c522c9d87`; the corrected sixth native case passes there after merging master `342acab31`. All original 60 rows executed: 33 PASS, 26 report-only modern-target gates and one native-glue evidence gap. No runtime cfg gate was added.

Retail preservation: all 67 bounded cases pass both before (`14f3c5d8b`, pre-producer source) and after (`c522c9d87`), with zero retained or new failures. Both current stock `lua-errors` outputs are byte-exact empty arrays. Full-feature Wrath/Retail integration checks have zero warnings; formatting passes. Every sealed ELF is source-bound, and original persisted CVar stat/hash remains unchanged. A Wrath check postprocessing regex failed after Cargo exited successfully; saved logs were recovered without rerunning Cargo.

These are bounded receipts, not full-suite or native-runtime parity. The identity/context/cursor requirements above remain unchecked where no dedicated behavioral proof was added.

## Known gaps (current cycle)

- [ ] Native GlueXML availability remains unverified; modern-only original targets remain report-only, not runtime passes.

## Out of scope

- Native execution parity: authority is static reconstruction only.
- Guild candidates and friend rows without GUIDs: native identity links are unavailable; no synthetic identities or name-based merges are introduced.
- Whisper history, native Battle.net presence nodes and recency timestamps: no suitable native backing state established for this batch. Alphabetical ordering models only native recency ties.
- UTF-8/case-mapping and center-editing edge parity: native helper caveats remain; do not claim modern Rust character slicing reproduces them.
- New modern-library shims, independent native popup instances, vendor or rilua modifications, and unrelated legacy global cleanup.
