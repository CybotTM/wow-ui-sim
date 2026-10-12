# Independent gate — FAIL

Original compiled revision: `59f3ff1367ce5f003ee584a8be307b62d3f5e539`. Final independent read includes main-owned follow-up; not covered by original compilation.
Owned epoch: `diagnostic-20261012T140542064588Z`.

## Exact blocking boundary

Existing targeted integration compilation exited **101**, build-finished success=false, zero integration executable artifacts. Compiler error **E0583**, `tests/startup_perf.rs:8:1`, `mod observation;`: file not found for module `observation`. Compiler searched `tests/observation.rs` or `tests/observation/mod.rs`; submitted helper is `tests/startup_perf/observation.rs`. Suggested bounded correction: explicit module path for the existing helper. No source changes performed.

Six manifest deprecation warnings retained verbatim below. Unit ActiveState=active/SubState=exited/Result=success/ExecMainStatus=0 does NOT prove compilation or tests passed: controller JSON reports Compile failed or source changed; actual compile exit is 101. Capacity was admitted, not blocked.

## Coverage / proof levels

| Contract | Finding | Proof level |
|---|---|---|
| Canonical setup/discovery/load order/post-load/startup events | Preserved relative to parent; same new_game_env and Game discovery, same loop and error conversion | Source only, tests/startup_perf.rs:30–77 and tests/perf/base_game.rs:6–19 |
| Readiness and strict 20-second assertions | Original query and both assertions preserved; added emission before assertions; budget remains elapsed < Duration::from_secs(20) | Source only, tests/startup_perf.rs:16,109–133 |
| Same-call elapsed before unwrap/record | Clock starts before load_addon(&env.loader_env(), toc_path), stops immediately afterward; includes loader_env argument evaluation, not pure-body measurement | Source only, tests/startup_perf.rs:66–75 |
| Full clock and recording overhead | Vec allocation/row cloning/push inside addon/full clock; full clock ends after startup events; JSON encoding/emission after full clock, no subtraction | Source only, tests/startup_perf.rs:30–46,64–95 |
| Typed serializer concrete behavior | Distinct names, durations, counts, six cache counters, counts-only diagnostics; total 47 agrees with LoadTiming total=2+3+5+30+7; JSON roundtrip and exact full expected object | Source only, tests/startup_perf/observation.rs:62–150; src/loader/mod.rs:372–381. Execution blocked |
| Wiring | Existing startup test attempts to import helper but module file resolution fails | FAIL compilation, tests/startup_perf.rs:8 |
| JSON rows/order/count/summary; same-run returned field coherence | No runtime outputs exist | BLOCKED, not inferred |
| Exact raw stdout selector/body credit | Both selectors unexecuted; zero body credit, no parent/child summaries to double-count | BLOCKED |
| Original threshold outcome | No startup execution; no measured elapsed or threshold result | NOT EXECUTED, not PASS or observed threshold FAIL |
| Artifact authentication | artifact.json never created; no new or old seal hashed | Not applicable, authentication count 0 |
| Source before/after | Five source file digests before/after compile equal; independent final read differs because main applied explicit module-path follow-up while report was being saved | PASS original compile stability; final follow-up drift retained in proof.json |
| Production check reuse | Existing Friend check124825 exit 0; unchanged src/Cargo/library scope relative to recorded check HEAD 05fddbf399; six warnings remain | Applicable only to unchanged production scope; does not cover new integration code |

## Rust readability audit

[LENGTH] tests/startup_perf/observation.rs:15 — encode_addon_observation()
Flat JSON projection exceeds the skill's 30-line non-test body guideline (32 body lines). Declarative shape, no deep nesting or branching; no runtime side effects hidden. If enforcing length mechanically, split timing/cache JSON projections into named pure serializers. Advisory only; no adjacent refactor authorized.

Concrete test remains under 200 lines. Changed code has no TODO/FIXME/HACK/XXX, warning suppressions, empty error handlers, speculative fallback, or commented-out code blocks. Retained observation push loop pairs effectful ordered loads with returned results; map conversion would not reduce comprehension. No new cognitive complexity >15 or nesting >3 found by manual inspection. No metric/check command launched.

## Retained warnings

warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim-test-harness v0.1.0 (/home/osso/Projects/wow/wow-ui-sim/crates/test-harness)
   Compiling wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
error: could not compile `wow-ui-sim` (test "integration") due to 1 previous error

## Constraints

No compile/test/check/runtime replay, no unit mutation, no delegation, no source edits, no cache tree or old binary hashing. No performance repair, exclusive timing buckets, native-client parity, historical equivalence, or causal attribution claim. No independent receipt written because no artifact existed. Separate report/proof directory; existing epochs untouched.
