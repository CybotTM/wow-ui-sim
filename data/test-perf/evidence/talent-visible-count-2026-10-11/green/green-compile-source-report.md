# PARTIAL — GREEN compile/source gate only

Revision `5d2e4ece1c08ec967cc8a7fd8f8bbe60d5a02ba4`; epoch `green-compile-20261012T002653774260Z`. **Compilation/source checks supported; runtime blocked. No GREEN behavior or repair acceptance.**

## First NEW artifact authentication — reuse later

Authenticated only this epoch's `integration-sealed`, once: **467445040 bytes**, SHA-256 `aa801f09926eb845115af4fe88a6ab341b9e6b85b3739c3923bee8daf368555d`. Size/hash match `artifact.json`; its Cargo record exactly matches the single selected integration record in saved `compile.stdout`. No RED/old executable or library bytes read or hashed. Reuse this authentication for final behavior verification; do not rehash this artifact merely for another milestone.

Saved compile command: `/usr/bin/cargo test --test integration --no-run --message-format=json --offline --locked -j 12`. Exit **0**, `source_unchanged=true`, Cargo `build-finished.success=true`. Actual saved tools: Cargo 1.99.0 (`5f94df478`, 2026-08-27); rustc 1.99.0 (`b940084d7`, 2026-09-28), x86_64-unknown-linux-gnu, LLVM 23.1.1.

Cargo target is `tests/integration.rs`, integration test binary, fresh=false; optimized test profile opt_level=1, line-tables-only debuginfo, assertions/overflow checks enabled. Exact feature list retained in proof JSON: default Retail/profile-retail through retail-12-1-0, gui/casc/sound, prefork-full-ui and capability features. Current HEAD/source match committed 5d; test-source SHA-256 `3e822fbe379a9245ba854a69b0e9ca07492ad89b8d0f507714b8675914e6e5fa` matches compile revision receipt. This is recorded compile linkage, not hermetic provenance.

Saved stderr has six inherited iced-wgpu-patched manifest deprecations; Cargo JSON compiler-message rows=0. No warning-clean claim. Existing `final-fmt.json` at 5d reports exit 0; reused, not rerun.

## Source gate

| Scope | Evidence | Limit |
|---|---|---|
| Collector | `tests/talent_panel_onupdate_perf.rs:53-73`: 7 added/1 removed lines; reads registered `on_update_frames`, existing `widgets.is_ancestor_visible`, sorts numeric IDs; name/parent-key/id mapping unchanged | Visible registered frames, not mode-eligible or actually fired callbacks |
| Registration/predicate | `src/lua_api/script_helpers/on_update_cache.rs:8-27,31-60` tracks OnUpdate OR OnPostUpdate registrations; `src/widget/registry/mod.rs:593-609` rejects missing/non-displayed frames and ancestors | Source evidence only, not runtime proof |
| New regression | Entire 99-line addition byte-identical to c7 | Visible membership, hidden-parent exclusion and restored visibility still need GREEN execution |
| Original controls | Removing c7 regression reconstructs parent file; replacing only new collector reconstructs c7. Both original test bodies, timing helpers/constants unchanged: ceiling 192; avg/max/delta 35/75/20ms; baseline/steady/settle ticks 20/24/10 | Controls unexecuted in this gate |
| Production/API/vendor | Git range c7-parent→5d has no changed paths under src, crates, Interface, Cargo.toml or Cargo.lock; no tracked worktree edits | No production compatibility acceptance |

Manual changed-helper readability audit: no violations. 21-line helper, four simple iterator combinators, single existing predicate, explicit local sort; no new branching, suppression, fallback or external mutation. No readability CLI or lint run.

**Collector predicate source audit is not an implementation-shape substitute for behavioral verification.**

## Accepted RED reused, not reopened

Accepted verifier2469 report: **0 passes/1 failure**, collector0 vs expected1 after callback count1 and public visibility/name/script preconditions. Later membership/hide/re-show assertions were unreached; original performance controls executed0. No RED reauthentication or rerun. Retained 16 copied text/metadata files total19054 bytes; stat sizes match retention manifest. Derived `red-source.rs` equals Git c7 bytes. These checks do not reauthenticate the RED binary or broaden accepted RED scope.

## Runtime blocked / pending

Saved admission **2026-10-12T00:29:35.744121Z**: load1=36.12, available RAM=24.63176727294922GiB, relevant Cargo processes=[], admitted=false. Saved result **RESOURCE_BLOCKED**, executed_cases=0, pending_jobs=0. No live polling/operations performed.

Pending: GREEN regression runtime; both original loaded talent-panel performance controls; default `cargo check`. No body-stage attribution, runtime hide/show proof, repair/adder acceptance, loaded/native/addon acceptance, or full handoff claim. Broad BA remains FAIL; full handoff OPEN.

No builds/tests/checks/reruns/polling/operations/network/delegation/source edits performed. Private report/proof only. Inspection failures (nonpersistent variables/ctx and initial non-repo Git cwd) are recorded in proof JSON; no build/test/check was repeated.
