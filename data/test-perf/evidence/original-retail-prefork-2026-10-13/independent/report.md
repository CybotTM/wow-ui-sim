# Original four prefork cases — saved-evidence verification

**Test verdict: PASS — 4/4 original cases, 0 test failures, three actual `systemd-run --wait` launcher exits 0. Diagnostic verdict: runtime logs contain no observed warning/error/failure diagnostics; compiler stderr is NOT warning-free (six manifest deprecations).**

Verified 2026-10-13 by reading persisted evidence, not executing tests. Evidence root:
`<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/original-prefork-20261013T135420006338Z`.
Runtime evidence: `runtime-original/`. Recorded compiled/snapshot source: `a6723200be40145ec352666b538998bf4035a608`; recorded production: `50f6f0ea8`. These are persisted provenance labels, not newly authenticated revision identities.

## Exact original selectors and cases reached

Each saved argv sets `PREFORK_EXACT_FIXTURE_GROUP` to the exact group below and invokes the saved `artifacts/deps/prefork_full_ui-17a690b4e27ba81b` with `--nocapture`; no substring test filter or publication/owning suite was used. Saved source `source/tests/prefork_full_ui.rs:227-247` declares these original registries; `:317-318` selects the exact-group path; `:370-387` dispatches to their original constructors. Stdout names match those registries exactly, without duplicates or controller fixture names.

| Group selector | Original test reached (`... ok` in stdout) | Result |
|---|---|---|
| `chat` | `chat_frame::test_chat_editbox_click_type_and_submit` | PASS |
| `chat` | `chat_frame::test_chat_editbox_text_color_after_activation` | PASS |
| `cast-bar` | `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix` | PASS |
| `spellbook` | `blizzard_player_spells_loads::mainline_spellbook_keybind_opens_and_closes_without_runtime_errors` | PASS |

Exact stdout summaries:
- chat: `test result: ok. 2 passed; 0 failed; 2 total`
- cast-bar: `test result: ok. 1 passed; 0 failed; 1 total`
- spellbook: `test result: ok. 1 passed; 0 failed; 1 total`

## Actual wait exits, not inferred from PASS

Saved `run-original.py` records `result.returncode` from the captured launcher into each result JSON. All three complete argv arrays include `systemd-run --user --wait`. Each launcher independently states `Finished with result: success` and `Main processes terminated with: code=exited, status=0/SUCCESS`; each result JSON records `launcher_exit: 0`. Aggregate `runtime-original/results.json` agrees with all individual results.

| Group | Recorded service unit | Wait exit | Stdout/stderr bytes |
|---|---|---:|---:|
| chat | `wow-original-prefork-chat-20261013t141805978605z.service` | 0 | 196 / 6950 |
| cast-bar | `wow-original-prefork-cast-bar-20261013t141818008227z.service` | 0 | 148 / 6951 |
| spellbook | `wow-original-prefork-spellbook-20261013t141827872300z.service` | 0 | 171 / 9050 |

Saved argv wraps each run in `timeout --signal=TERM --kill-after=2 90` and read-only bwrap mounts with isolated PID/network namespaces. The recorded exits are success, not timeout exits. No launcher was rerun or service queried.

## Diagnostics — checked separately from test status

Full stdout, stderr, argv, launcher, and individual/aggregate result files were read. Runtime stderr contains startup, workaround, and EditMode informational traces. No observed warning, error, panic, or failure diagnostic in these three saved runtime stderr files. Chat/cast-bar traces reach `reapply_player_frame_anchor`; spellbook additionally reaches startup sequences and two `S → TOGGLESPELLBOOK → PlayerSpellsUtil.ToggleSpellBookFrame()` dispatches. The logged `FIRST_FRAME_RENDERED` event does not prove a native rendered frame. Names such as `seterrorhandler restored` are informational messages, not failures.

Compiler proof is reused: `compile.stdout:741` is `{"reason":"build-finished","success":true}`. `compile-terminal.txt` records `SubState=exited`, `Result=success`, `ExecMainCode=1`, `ExecMainStatus=0`; `ExecMainCode=1` denotes the recorded exited process classification, not exit status 1. `ActiveState=active` is also preserved, not mistaken for a still-running compilation. `compile-result.json` agrees, with `build_finished: true` and `compiler_messages: []`.

**Compiler stderr still has six real warnings**, despite the empty JSON compiler-message list: deprecated manifest Clippy lint names in `iced-wgpu-patched/Cargo.toml`: `large-enum-variant`, `map-entry`, `match-wildcard-for-single-variants`, `redundant-closure-for-method-calls`, `trivially-copy-pass-by-ref`, and `type-complexity` (underscore spellings advised). The additional `iced_wgpu (manifest) generated 6 warnings` line summarizes them; it is not a seventh distinct warning. No observed compiler error; stderr ends with `Finished test profile [optimized + debuginfo] target(s) in 31.78s` (backticks omitted here). Test PASS must not be reported as warning-free compilation.

## Receipt reuse and proof limits

Only existing receipt stat fields are reused; no executable stat/authentication/hash performed. `receipt-prefork_full_ui.json` has equal saved before/after stats: device 66306, inode 141428134, size 514726256, mtime_ns 1791899692248073938. Receipt authentication/digest fields are not used as fresh proof.

- This verifies the saved original Retail epoch's four named prefork results, not current checkout/build/runtime state. Retail/live active cache/read-only provenance comes from saved metadata and argv, not a new runtime/cache inspection or immutable cache archive.
- No assertion-body audit, broad suite, owning/publication repeat, conformance run, or new execution. Success is limited to the exact cases reached and persisted diagnostics; silence is not proof of all runtime errors being absent outside captured channels.
- No native GUI, rendering, visual parity, performance, or resource-comparison claim. Startup/keybind timings and launcher resource figures are not benchmark evidence.
- No build, authentication, executable/library/archive/runtime hashing or inspection, operational action, delegation, or tracked-source edits performed. Only requested verification artifacts were written.

`proof.json` preserves full runtime evidence texts/argv/results, exact case and summary comparisons, diagnostic classifications, persisted compiler proof, receipt stats, and limits. Both output artifacts are read back after persistence.
