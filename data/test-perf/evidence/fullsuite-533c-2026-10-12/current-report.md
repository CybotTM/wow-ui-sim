# Saved current full-suite — FAIL; parent OPEN

Read/followed `/home/osso/AgentConfig/skills/verify/SKILL.md`. Assigned verifier, saved-artifact-only inspection. Entire saved JSON/log parsed; no builds, tests, replays, operations, delegation, binary access, new seals/authentication or repo edits. Only two exclusive new requested outputs written.

## Binding and full text

Saved revision `533c086e046324528d92d581a241021bf455ebdb`; JSON ref/sha agree. Started **2026-10-12T11:05:49-0500**, finished **2026-10-12T11:25:49-0500**. Saved submission names `full-suite-1791821148.service`, invocation `0d21aac24c2c402e966f5880c0501dd1`, submission exit0; not live-polled and not suite success.

Log: `/home/osso/Projects/wow/full-suite-results/533c086e046324528d92d581a241021bf455ebdb.log` — **2,086,638 bytes / 15,677 lines**, SHA256 `121787457524b11309e92f915f5307e1568ab9952cdf73d8273408885859aa1a`. Complete combined saved stream, exact JSON, all named outcomes, warnings, phase streams and source-inspection output embedded in `current-proof.json`. Separate historical stdout/stderr are not supplied by this log; no reconstructed stream split or event chronology claimed.

## Exact phase counts

| Phase | PASS | FAIL | Skipped | Executed | Exit |
|---|---:|---:|---:|---:|---:|
| Integration | 10,777 | 0 | 19 | 10,777 | 0 |
| Prefork main | 2,320 | 1 | none recorded | 2,321 | phase1 |
| Prefork auxiliary | 4 | 0 | none recorded | 4 | included in phase1 |
| Lib | 2,001 | 0 | 0 | 2,001 | 0 |
| Total executions | **15,102** | **1** | **19 integration skips** | **15,103** | **FAIL** |

Integration summary10817: `10777 tests run: 10777 passed (14 slow), 19 skipped`; lib summary15676: `2001 tests run: 2001 passed, 0 skipped`. Continuous terminal ordinals1–10777 and1–2001, unique phase-scoped binary/selector keys; **zero repeated nextest failure-summary rows in current log**. Prefork main summary13152, auxiliary summaries13157/13161/13165: **2321 + 2 + 1 + 1**. Exactly2325 distinct named terminal rows. Repeated panic/left/right payload after the failed named row is diagnostic output, not a second failure. Current log has only these four `running`/`test result` pairs; **no extra inner-child execution blocks**. BA report separately excluded2 integration and1 lib failure-summary duplicates plus embedded child/parent diagnostic recaps. Cross-phase repetitions are executions, not globally distinct tests.19 skips are aggregate evidence, not invented named rows; omitted selectors receive no credit.

## Earliest observed failure boundary

`patch_8_1_0_publication_sweep::patch_8_1_0_publication_sweep`: one prefork FAIL, named row13076; stderr13079 onward; earliest observed assertion at log13081, **`tests/common/publication_sweep.rs:315:5`**. Exact message:

```text
assertion `left == right` failed: new gaps: []; resolved/stale gaps: ["wt-global-api-C_FriendList.GetFriendInfo-44", "wt-global-api-C_MountJournal.GetMountFromItem-78"]
```

Left observed `non_ok` has52 IDs, right retained `known` has54. Left-only IDs empty; right-only exactly the two above. Source308–317 constructs these sets and asserts exact equality. Classification: retained-known-gap/publication-observation mismatch with two resolved/stale entries, **not a newly missing API assertion**, not signature/behavior/native proof. No causal claim. `new_failures` repeats this selector but does not establish regression causality. Full exact left/right/stderr retained, including long payloads truncated by display tools.

## Original four main prefork selectors

| Exact selector | Result | Current log line |
|---|---|---:|
| `patch_8_0_1_cached_semantics::patch_8_0_1_cached_event_and_aura_input_changes` | PASS | 13072 |
| `patch_8_1_0_cached_surfaces::patch_8_1_0_cached_retirement` | PASS | 13074 |
| `patch_8_1_0_calendar_compare::patch_8_1_0_cached_calendar_ordering` | PASS | 13075 |
| `patch_8_1_0_publication_sweep::patch_8_1_0_publication_sweep` | FAIL | 13076 |

## Original four auxiliary selectors — all executed, not skipped

| Exact selector | Result | Current log line |
|---|---|---:|
| `chat_frame::test_chat_editbox_click_type_and_submit` | PASS | 13154 |
| `chat_frame::test_chat_editbox_text_color_after_activation` | PASS | 13155 |
| `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix` | PASS | 13159 |
| `blizzard_player_spells_loads::mainline_spellbook_keybind_opens_and_closes_without_runtime_errors` | PASS | 13163 |

These are2+1+1 and already included in prefork totals; not four extra tests beyond2325. Original Garrison load also PASS at11464; all2325 exact prefork results retained in proof, without selection inference.

## Original failed BA controls — now exact-selector PASS

| Exact selector | Current result | Current log line |
|---|---|---:|
| `startup_perf::full_game_startup_stays_under_budget` | PASS | 9729 |
| `talent_panel_onupdate_perf::talent_panel_open_onupdate_tick_latency_stays_under_budget` | PASS | 9792 |
| `asset_resolver_config::tests::gui_resolution_cache_preparation` | PASS | 14393 |

BA remains historical FAIL: startup96.24s against20.00s, talent displayed-average delta24.47ms against20.00ms (baseline1.18ms, talent25.65ms, max45.69ms), CASC cold preparation/content lookup error for encoding key `53dcc8beeb4c9e75b69c5f2a8a67c7e4`. Current nextest PASS rows do **not** retain successful body startup/talent timing or CASC cache details; row wall durations155.254s/112.067s/24.528s are whole-test timings, not budget measurements. No inferred current timings, cache repair or cause. Previous isolated BA performance controls are separate evidence, not replacements for BA failures or additions to this suite.

Thresholds preserved: startup `<20s`; talent mean `<=35ms`, maximum `<=75ms`, mean `<=baseline+20ms` (printed wording uses `<`); baseline20 ticks, steady24, settle10. Current source also retains visible-handler added ceiling192. BA-to533 source diff retains numerical thresholds/assertions but includes startup observation instrumentation and talent logging/visible-collector changes; **not identical historical harness/source**. No unchanged-environment or native equivalence inference.

## BA report comparison

Reused `/home/osso/.local/state/wow-ui-sim/verification/ba-fullsuite-current/report.md` and its authenticated `countproof.json`; no BA seal/authentication access or rerun. BA integration10,713PASS/2FAIL/19skips/10,715executed; prefork2,325PASS/0FAIL including four auxiliaries; lib2,000PASS/1FAIL/2,001executed. BA total15,038PASS/3FAIL/15,041executed. Current changes: integration **+62 executions, +64PASS, -2FAIL**; prefork unchanged2325executed but **-1PASS/+1FAIL**; lib unchanged2001executed but **+1PASS/-1FAIL**. Total **+62 executions, +64PASS, -2FAIL**; skips unchanged19. Count differences do not establish causality or all-case selection equivalence.

## Production-scope applicability and warnings

Observed canonical HEAD `f68ab4ed7e93a7de2b52c532fc7f5d686c69c9a0`. Complete committed changed-path list533→f68 contains **93 paths**, all `docs/` or `data/test-perf/evidence/`. Explicit pinned production-scope `git diff 533… f68… -- src tests crates Cargo.toml Cargo.lock build.rs .cargo scripts build`: empty stdout/stderr, exit0. Original canonical working diff against HEAD for same tracked scope is empty, exit0. Thus saved533 results apply to inspected **tracked production source through f68**; not historical cleanliness, untracked inputs, external dependency content, runtime cache, toolchain or executable identity. No source/binary hashes needed or performed.

**Not warning-free.** Six distinct iced-wgpu-patched manifest deprecated hyphenated Clippy keys (`large-enum-variant`, `map-entry`, `match-wildcard-for-single-variants`, `redundant-closure-for-method-calls`, `trivially-copy-pass-by-ref`, `type-complexity`) each recur in integration/prefork/lib compile output, plus three six-warning summaries:21 `warning:` lines, not21 distinct warnings. All raw warnings and other runtime diagnostics retained verbatim in full stream; no suppression or threshold/skip changes.

**Saved full-suite FAIL. Parent OPEN.** No overall PASS, native/catalog/dressability parity, cache-content provenance, binary-to-source identity, or causal repair claim. Other parent acceptance gates remain outside this saved-suite inspection.
