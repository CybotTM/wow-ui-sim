# Wrath F07 scroll, anchor and update fixtures

F07 covers 32 existing integration cases: 10 passing Wrath fixture rows and 22 unchanged, ungated report-only gate candidates. The REJECT repair restores the original shine timer bound and original MinimalScrollBar/EventFrame consumer. Historical 11 PASS / 21 FAIL results used weakened/substituted fixtures and are not acceptance evidence. See the [investigation](../wiki/investigations/wrath-fixture-f07.md).

## What it must do

- [x] Inspect cached Faux scrollbar buttons, thumb and textures through their `$parent` names; retain original inline offset/wheel and empty-handler assertions.
- [x] Restore the unchanged EventFrame/MinimalScrollBar anchor fixture on Wrath, ungated. Missing cached declarations are INFERRED absence; this row is a report-only gate candidate, not permission to substitute generic XML.
- [x] Load profile FrameXML for active pulse/shine worklists. Wrath uses cached free functions and named textures; Retail keeps its mixin fixture. Both require `(timer - 0.016).abs() < f64::EPSILON`.
- [x] Retain the separate actual-parent `$parentBackground` fixture and cached Wrath FrameXML corpus parser; Retail keeps its SharedXMLBase corpus.
- [x] Produce fresh Wrath/Retail counts for all 32 exact rows, Wrath stock lua-errors, both profile checks and fmt. Every selected case executes more than zero tests.

## Boundaries

No vendor/rilua changes, failure-hiding cfg gates, fallbacks, invented consumer replacements or tolerance relaxation. A failing original shine assertion requires a grounded producer diagnosis; otherwise retain the failure as a producer-bug receipt/report-only row. No native 12340 declaration is authenticated: provenance remains UNCONFIRMED, cached expectations INFERRED. No decompiled source may be read.

## Tests asserting this spec

Fresh code revision: `c968c743922ca12aabeb172f39ff5f68ed435f46`. No code changed after these executions; subsequent edits only finalize documentation. No previous-cycle executable was reused, and no optional generic anchor test was added.

| Proof scope | Fresh result |
|---|---|
| Wrath32 exact cases | 10 PASS / 22 FAIL; failures exactly the unchanged report-only originals |
| Retail32 exact cases | 32 PASS / 0 FAIL |
| Wrath stock `--no-addons --no-saved-vars lua-errors` | byte-exact `[]\n`, exit0 |
| Both profiles: `cargo check -j1 --lib --test integration --bin wow-sim` | exit0, no warnings |
| `cargo fmt --all -- --check` | exit0 |

Wrath uses `--no-default-features --features sound,gui,casc,client-wrath`; Retail uses defaults. Each case runs with `--exact`, `--nocapture`, `--test-threads 1` and a 90-second bound. All 64 matrix invocations and two restored-fixture probes execute one test each; worklist subprocess summaries do not count as additional distinct cases. The Wrath corpus parses 140 XML files.

The strict shine assertion passes on both profiles. No producer defect or producer RED/GREEN cycle is claimed. The restored MinimalScrollBar anchor row fails on Wrath with `missing track`, and passes on Retail. The entire minimal-scrollbar file matches `773fab374` byte for byte; all 22 report-only function bodies match their original specimens. These failures are retained candidates, not accepted passes or hidden exclusions.

Builds and stock execution ran inside `f07-fix.service`, `agents.slice`, MemoryMax9G/MemorySwapMax0, slot0, `CARGO_INCREMENTAL=0`, `-j1`, after literal capacity ADD verdicts. The owner marker used the service MainPID, mode0600, and was reread before the UID1000 process scan. Admission retried HOLD every120 seconds with stable `CAPACITY_ID=f07-fix`; supervisor polling was every300 seconds. Builds were separate from bounded execution. Each executable launch used a fresh child-only copy of current CVars; all 78 command receipts preserve original CVar stat/hash. Cargo.lock ordering-only drift was restored. The unit exited successfully; final process scan has zero slot references or relevant denied entries and the marker is absent. Added-line scans and manual Rust readability audit found no violations.

### Exact rows

Every name below is copied verbatim from the 32-case membership; no row is compile-gated or ignored.

| Exact case (verbatim) | Wrath | Retail |
|---|---|---|
| `scroll_widgets::test_legacy_scrollbar_supports_scrollutil_percentage_api` | FAIL | PASS |
| `scroll_widgets::test_scroll_button_has_textures` | PASS | PASS |
| `scroll_widgets::test_scrollbar_has_buttons` | PASS | PASS |
| `scroll_widgets::test_scrollbar_has_thumb_texture` | PASS | PASS |
| `scroll_widgets::test_scrollbar_track_textures` | FAIL | PASS |
| `scroll_widgets::test_scrollboxlist_callbacks_and_foreach_frame_are_stateful` | FAIL | PASS |
| `scroll_widgets::test_scrollboxlist_creates_child_frames` | FAIL | PASS |
| `scroll_widgets::test_scrollboxlist_interpolate_scroll_round_trip` | FAIL | PASS |
| `scroll_widgets::test_scrollboxlist_keyvalues` | FAIL | PASS |
| `scroll_widgets::test_scrollboxlist_mixin_methods` | FAIL | PASS |
| `scroll_widgets::test_scrollboxlist_rust_children_keys` | FAIL | PASS |
| `scroll_widgets::test_scrollboxlist_shadows_have_textures` | FAIL | PASS |
| `scroll_widgets::test_scrollframe_template_creates_scrollbar` | PASS | PASS |
| `scroll_widgets::xml_arguments::cached_faux_scrollframe_inline_offset_and_wheel_delta_sync_scrollbar` | PASS | PASS |
| `scroll_widgets_minimal::test_inherited_minimal_scrollbar_child_keeps_inline_outer_anchors` | FAIL | PASS |
| `scroll_widgets_minimal::test_minimal_scrollbar_child_structure` | FAIL | PASS |
| `scroll_widgets_minimal::test_minimal_scrollbar_keyvalues` | FAIL | PASS |
| `scroll_widgets_minimal::test_minimal_scrollbar_mixin_accessors` | FAIL | PASS |
| `scroll_widgets_minimal::test_minimal_scrollbar_rust_children_keys` | FAIL | PASS |
| `scroll_widgets_minimal::test_minimal_scrollbar_stepper_sizes` | FAIL | PASS |
| `scroll_widgets_minimal::test_minimal_scrollbar_thumb_textures` | FAIL | PASS |
| `scroll_widgets_minimal::test_minimal_scrollbar_track_has_thumb` | FAIL | PASS |
| `scroll_widgets_minimal::test_minimal_scrollbar_track_textures` | FAIL | PASS |
| `scroll_widgets_minimal::test_reapplied_inline_child_anchor_uses_actual_parent_for_dollar_parent_lookup` | PASS | PASS |
| `scroll_widgets_minimal::test_scrollframe_template_creates_scrollbar_with_thumb` | FAIL | PASS |
| `scroll_widgets_minimal::test_unnamed_scrollbox_creates_child_frames` | FAIL | PASS |
| `uiparent_onupdate_worklists::animated_shine_onupdate_still_updates_active_shines` | PASS | PASS |
| `uiparent_onupdate_worklists::button_pulse_onupdate_still_updates_active_buttons` | PASS | PASS |
| `xml_parsing::test_parse_all_xml_in_shared_xml_base` | PASS | PASS |
| `xml_parsing::test_parse_callback_registrant_xml` | FAIL | PASS |
| `xml_parsing::test_parse_color_swatch_xml` | FAIL | PASS |
| `xml_templates::inline_advanced::empty_script_overrides::cached_faux_scrollframe_empty_function_disables_inherited_auto_range` | PASS | PASS |

## Related docs

- [Fixture investigation](../wiki/investigations/wrath-fixture-f07.md)
- [Inline XML arguments](xml-scroll-arguments.md)
- [Empty script clearing](xml-empty-script-clearing.md)
- [Anchor reapplication](../wiki/investigations/chatframe-scrollbar-anchor-reapply.md)

## Out of scope

Modern ScrollUtil/ScrollBox/MinimalScrollBar publication on Wrath; physical wheel input; rendering/performance changes; unrelated producer defects; full native-client parity.
