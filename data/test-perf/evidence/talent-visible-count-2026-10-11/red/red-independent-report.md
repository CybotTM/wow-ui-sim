# Independent bounded RED gate

**AUTHENTIC_TARGET_RED — intended collector assertion failed.** No test, build, check, rerun, delegation, network request, operation, or source mutation performed by this verifier.

## Artifact / source authentication

- Authenticated only new `integration-sealed`, exactly once: 467440944 bytes; SHA-256 `43d9ffe4dcd9e7c1b5e635db07dee249f1627c8f5154a65b71a0f6c27b674cd6`. Both match `artifact.json`. No old executable/library bytes read or hashed.
- Saved `compile.stdout` contains one selected integration compiler-artifact record, identical to `artifact.json.cargo_record`: package `/home/osso/Projects/wow/wow-ui-sim`, target `tests/integration.rs`, test=true, fresh=false, opt_level=1, line-tables-only debuginfo, debug assertions/overflow checks enabled. Features include default, client-retail, profile-retail, retail-12-1-0, gui, casc, sound, prefork-full-ui. Saved build-finished success=true; `compile-result.json`: exit=0, source_unchanged=true.
- Saved actual tools: `/usr/bin/cargo` 1.99.0 (5f94df478, 2026-08-27); `/usr/bin/rustc` 1.99.0 (b940084d7, 2026-09-28), x86_64-unknown-linux-gnu, LLVM 23.1.1. No toolchain rerun.
- Read-only Git HEAD equals `c7f37def1180d9544ef68a439c1ce938648e07f3`. Current test source equals committed source; its SHA-256 `a6d6e1782b37b77805fc50141c1da5be971ed1190c19e3dcacbcb7eb64747053` matches `revision.json`. Git status has only untracked `.code-index.db` and `verification/`; no tracked edits.
- Commit adds exactly 99 test lines in one file. Removing the new regression from c7 reproduces its parent source byte-for-byte: collector and both original performance tests/assertions are unchanged. No production/vendor change.
- Linkage: `tests/integration.rs:1` invokes `integration_tests!`; `crates/test-harness/src/lib.rs:7` includes its generated macro. `crates/test-harness/build.rs:29-66` discovers top-level test modules and emits path-based modules; lines 441-449 select Rust files and lines 462-483 do not exclude this file. Saved runtime prints the exact linked new selector.

## Actual failure boundary

Single submitted selector: `talent_panel_onupdate_perf::visible_onupdate_collector_tracks_child_parent_visibility`, `--exact --nocapture --test-threads=1`, unit `wow-talent-counter-red-case-20261012t000938z.service`.

Full runtime stdout:

```text
running 1 test
test talent_panel_onupdate_perf::visible_onupdate_collector_tracks_child_parent_visibility ... FAILED

failures:

failures:
    talent_panel_onupdate_perf::visible_onupdate_collector_tracks_child_parent_visibility

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10735 filtered out; finished in 0.27s
```

Failure at `tests/talent_panel_onupdate_perf.rs:135:5`:

```text
assertion `left == right` failed: Collector should count the visible child that received OnUpdate: []
  left: 0
 right: 1
```

Sequential source evidence: fixture registration, child `IsVisible()`, callback `GetScript` type, actual `GetName`, successful public `fire_on_update`, readable callback counter, and `ticks == 1` assertion at line 133 precede line 135 and therefore passed before this panic. Baseline+1=1 establishes baseline=0. The old collector returned 0/names=[] despite a visible child actually receiving its callback. This is the intended counterexample, not an early API/visibility/callback precondition failure and not zero selected tests.

Later name-membership, ancestor-hide exclusion, and re-show restoration assertions were NOT reached. They specify behavioral transitions through public calls without forcing the visibility cache, but RED execution does not prove those transitions.

## Counts / terminal receipts

- Physical Rust rows: 1 executed failure, 0 passes, 10735 filtered; 10736 discoverable rows in this executable. Source file has 3 `#[test]` rows (new regression plus two unchanged performance controls). Logical new regression cases: 1. Original performance controls executed: 0.
- Saved terminal receipt: ActiveState=failed, SubState=failed, Result=exit-code, ExecMainStatus=101, ControlGroup empty, RuntimeMaxUSec=1min 30s, KillMode=control-group. `process-observation.json` at 2026-10-12T00:10:22.171670+00:00 reports no remaining sealed-executable processes. These are saved main-thread receipts, not newly probed process state.

## Manual readability / source behavior

Scope: 99 new lines plus unchanged collector. New test is a single linear behavioral sequence sharing environment, child identity, and cumulative callback counter; no branching/nesting, parameter overload, suppression, or TODO/FIXME/HACK/XXX markers. Phase-specific expect/assert messages expose the failing transition. Repeated visible/hidden/restored assertions represent distinct observable states, not redundant cases. No blocking changed-code readability finding. Manual assessment only; no readability CLI, lint, formatter, or check executed.

Unchanged helper `capture_visible_onupdate` reads `visible_on_update_cache.clone().unwrap_or_default()` and maps widget names/parent keys to strings. Its empty result does not distinguish absent cache from empty cache. The demonstrated mismatch is collector output versus actual callback, not proof of a specific cache lifecycle mechanism or production OnUpdate failure.

## Warning / acceptance limits

Saved compile stderr retains six inherited iced-wgpu-patched manifest deprecations: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity (hyphenated lint names deprecated in favor of underscore names). Cargo JSON compiler-message rows: 0. Compilation succeeded, but this is NOT a blanket warning-clean claim.

Broad BA remains FAIL; full handoff remains OPEN. No authentication of old broad proof. No GREEN/fix claim, native/addon acceptance, loaded talent-panel performance acceptance, or loaded-latency causal claim.
