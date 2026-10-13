# Saved Mists cast-bar runtime verification

## Verdict
PASS, narrowly: one selected, current-source, phase-adjusted Mists cast-bar body completed through the normal `prefork_full_ui` cast-bar fixture group. Not an unchanged historical immediate-hidden body, normal `wow-sim` invocation, setup-only success, admission-only success, or conformance-test count. Setup is NOT clean.

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/mists-original-cast-prefork-current-20261013T195457944782Z`.
Declared source: `844a63adb95c7111e1286fa668e632f7cd237806`; declared production delta: `621649e56`. Source files below are saved snapshot reads, not independently authenticated source-to-binary provenance.

## Saved runtime evidence
- `runtime-original-cast/stdout`: fully read, 148 bytes, 4 lines. Exact selector `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix`; `running 1 tests`, selected test `... ok`, `1 passed; 0 failed; 1 total`.
- `runtime-original-cast/stderr`: fully read, 13,170 bytes, 118 lines (read ranges 1–65 and 66–118). Includes constructor/startup probes and the body-only `cast-case-before-original-assertions` probe.
- `result.json`: launcher exit 0; `launcher.txt`: main process exited status 0/SUCCESS; service result success, runtime 7.497s. These saved results do not prove general process containment/cleanup.
- `argv.json`: `PREFORK_EXACT_FIXTURE_GROUP=cast-bar`, exact selector, `--exact --nocapture --test-threads=1`; saved read-only source overlay, CASC disabled, sound disabled, product `wow_classic`, timeout 86s. Admission receipt alone is not behavioral proof.

## Actual execution and source boundaries
All paths in this section are relative to epoch `source/`.

1. `tests/prefork_full_ui.rs:301–327`: normal main path configures child bytecode read-only setup and dispatches exact fixture group; separate conformance driver modes precede it. `:238–241` registers the exact name directly to `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix::run`. `:370–392` chooses CAST_BAR_FIXTURE_CASES and `spell_casting::env_with_full_blizzard_ui`, not controller substitution. `:455–485` runs conformance as setup prerequisite, enters parent bytecode bypass, creates the original constructor fixture, releases parent bytecode cache memory, then returns it to the prefork runner. Conformance output is not the reported one-test case count.
2. `tests/spell_casting.rs:138–163`: original constructor obtains profile Blizzard UI cache, creates WowLuaEnv, sets 1024×768, discovers/loads addons, applies post-load workarounds, fires startup events, then returns. Loader/event results are intentionally ignored here; successful construction does not establish error-free loading. `:216–234` fires startup/edit-mode/binding/display/scale events with before/after probes.
3. `tests/common/mod.rs:115–131`: fixture macro emits the callable body without replacing it. `tests/common/prefork.rs:113–156` separates setup from selected-case execution; `:447–475` invokes `(selected.case.test)(state)` after child setup inside catch_unwind and emits CHILD_PASS only after return.
4. `tests/spell_casting.rs:446–566`: ACTUAL current body. Mists first advances `env.fire_on_update(0.55)` at :451–452 before the :454 body probe and all assertions. Saved stderr shows `after-startup-events` IsShown=true and membership present; `cast-case-before-original-assertions` IsShown=false and membership absent. Hidden-state proof is AFTER that phase advance, not immediately at constructor return. Timing comment describes 0.2s delay + 0.3s fade; runtime does not independently authenticate native fade timing or establish first writer.

### Exact Mists assertion coverage
The selected body has eleven applicable Rust assertions; return without panic supports these source-defined assertions together, not separate per-assertion log records:

| Scope | Source assertions | Strongest proof |
|---|---|---|
| Unlocked layout | :470 lock value = 0; :471–474 not attached | Current selected-body PASS |
| Post-0.55s idle state | :494 hidden; :495–498 parent identity equals UIParentBottomManagedFrameContainer; :499–502 layoutParent same container; :503–506 absent from showingFrames | Body PASS plus matching saved phase probe |
| Player anchor respects unlocked setting | :526 player ApplySystemAnchor pcall succeeds; :527–530 stays unattached after detach + player anchor | Current selected-body PASS |
| Locked cast anchor | :557–560 cast ApplySystemAnchor pcall succeeds after lock setting=1 and UpdateSettingMap(true); :561–564 attached; :565 parent name PlayerFrame | Current selected-body PASS |

The non-Mists BottomManagedFrameContainer branch at :508–512 did not run. Other spell/cast visibility/healing tests did not run. Current non-retail normal-test wrapper :569–575 uses this constructor/body, but this receipt proves prefork execution, not a separate normal integration invocation.

## Residual diagnostics and build identity
Two setup self-anchor Lua-error emissions remain: original frame-create error and its wrapped ScriptErrors emission, both for `__Blizzard_UIPanels_Game_20279`, `Action[SetPoint] ... [Cannot anchor to itself]`. They are two emissions of the same reported setup failure, not evidence of two independent root causes. Pre-initialization probes also record three `systemInfo.isInDefaultPosition=READ_ERROR` observations while systemInfo=nil. No clean-startup claim.

Saved compile JSONL parsed: 741 records (665 compiler-artifact, 75 build-script-executed, 1 successful build-finished), zero compiler-message records. Full compile stderr read: six manifest deprecations plus ONE source dead-code warning, `src/lua_api/globals/real/unit_roles.rs:17:5`, unused Tank/Healer/Damage variants. Zero JSON diagnostics does NOT mean zero source warnings. Terminal ExecMainStatus=0.

Cargo target is test `prefork_full_ui`, required feature `prefork-full-ui`, executable metadata `target/debug/deps/prefork_full_ui-cd41c58227789dc0`, fresh=false, test profile opt_level=1 with debug assertions/overflow checks. Effective features: casc, client-mists, gui, prefork-full-ui, rodio, sound. Copied name `artifacts/wow-sim` is misleading and does not make this a normal wow-sim binary.

Saved `receipt-prefork_full_ui.json` reports PASS, SHA256 matching seal, hash_reads=1, 344540168 bytes, matching pre/post descriptor/path stats. Receipt read only; no executable open/hash or reauthentication performed. Seal/source-scope explicitly exclude independent authentication of external libraries/cache/settings and source archive.

## Cache and excluded claims
Saved cache-comparison.json: 2,977 files before and after; equal content/stat snapshots for `<HOME>/.cache/wow-ui-sim/blizzard-ui/mists/AddOns`. Existing main-owned snapshots reused; no cache read/hash, repair, sync, or reauthentication.

Historical chatpair and Retail original4 retain separate validity; no repeat or extension from this receipt. Historical immediate-hidden FAIL is not superseded by an unchanged-body claim: current selected body advances phase before checking hidden state. Parent message reports accepted coherent fade-policy disposition; that message is context, not independent file verification here.

UNPROVED: native parity, rendering, general cache semantics, broad process guarantees, full parent/full suite, first-writer identity, immediate-startup hidden state, unchanged historical body equality, and independently authenticated source-to-binary/library/archive provenance.

## Verification activity and readback
Saved-proof inspection only. No launch/build/binary/library/archive/source authentication, delegation, fixes, tracked edits, or cache rehash. report.md and proof.json are the only written outputs, under the requested untracked state directory. Latest readback: both complete files read after writing; see accompanying final readback operation. Existing evidence is historical saved runtime evidence, not a fresh run.
