# Inline XML scroll saved-proof handoff

**Read-only assessment:** 2026-10-11. Repository `/home/osso/Projects/wow/wow-ui-sim`; observed HEAD `21e1b183e0a5d985d5431e012ec0e9a8bd4c9e3b`. No commands executed tests/builds/runtime or hashed broad sources/binaries. No repository files changed. Existing unrelated untracked `.code-index.db` and `verification/` were present.

## Evidence ledger

| Layer | Evidence | Result / limit |
|---|---|---|
| Historical RED | `docs/specs/xml-scroll-arguments.md` records tests-only `6b63be464`; test source is `tests/scroll_widgets/xml_arguments.rs`. | Doc claims three RED cases: ordinary XML and runtime-template arguments resolve sentinels rather than passed values; cached FauxScrollFrame scrollbar fails to synchronize at offset 37. This is historical evidence, not current behavior. |
| Producer source | Commit `0c913d2ed17039e73fa6ba8941dfc122d644d676` (“Bind inline XML scroll argument names without consuming varargs”) is an ancestor of HEAD. It changes `src/loader/helpers.rs` to emit inline argument aliases. The focused tests were later added in commit `66c3296be` and diagnostic assertions in `6b63be464`. | Source implementation is present at HEAD by ancestry. It is not runtime proof. |
| Saved behavioral proof | `/home/osso/Projects/wow/full-suite-results/1dcc59c0788d0e09ee3d02516123df965ba8dc63.log`, matching JSON. | All three exact test selectors in `xml_arguments.rs` have PASS rows: runtime template (10703 integration run, position 8275), ordinary XML (position 8276), cached FauxScrollFrame offset/wheel sync (position 8279). Thus **3/3 scoped cases passed in that saved suite execution**. |
| Saved suite outcome | JSON timestamp `2026-10-11T02:47:33-0500` to `03:15:28-0500`; integration exit 0, prefork exit 1, lib exit 0. Integration failures `[]`; prefork failure `blizzard_garrison_ui_loads::blizzard_garrison_ui_loads_explicitly_via_load_addon_without_errors`; lib failures `[]`. | Overall saved suite was **not fully green**. The three scoped integration cases passed; do not report overall completion from this suite. |
| Current scoped equivalence | HEAD is after producer commit; `src/loader/helpers.rs` diff from producer commit to HEAD only makes two helper functions `pub(crate)`. The saved suite’s exact Rust test selectors match current `tests/scroll_widgets/xml_arguments.rs` selectors. | Strong scoped equivalence: the producer behavior and test bodies/selectors are unchanged between producer commit and current HEAD. However suite JSON names SHA `1dcc59c...`, while checkout HEAD is `21e1b...`; this proof is historical saved execution, not a fresh run on HEAD. |

## Exact covered assertions

- `ordinary_xml_inline_scroll_arguments_shadow_globals_and_preserve_varargs`: range `80:120` with two callback args, horizontal `35` and vertical `37` with one each, wheel `-1` with one; sentinel globals stay unchanged.
- `runtime_template_inline_scroll_arguments_shadow_globals_and_preserve_varargs`: same contract through a runtime-created template instance.
- `cached_faux_scrollframe_inline_offset_and_wheel_delta_sync_scrollbar`: unchanged cached `FauxScrollFrameTemplateLight`; setting vertical scroll to 37 updates scrollbar, directly invoking actual XML wheel handler advances frame and bar to 48 using step 11; sentinel globals stay unchanged.

These are simulator integration tests, including direct handler invocation; not physical wheel-input or native-client proof.

## Conclusion

Existing saved proof covers the exact three inline-XML scroll cases: **3/3 PASS in the saved integration run**. Producer source is present at current HEAD and its behavior is unchanged since `0c913d2...`; saved tests have the same selectors as the current test file. Keep historical tests-only RED, producer source, saved PASS, and current scoped source/test equivalence as distinct claims. The suite has a separate prefork failure, and no fresh current-HEAD execution was performed, so do not claim a freshly verified clean suite or completion solely from source.
