# Expanded positive RED — independent retained-artifact verification

Overall: **PENDING**. Compilation and artifact authentication pass; expanded RED execution is not proven.

## First authentication

Authenticated only `positive-red-compile-20261012T075738427394Z/integration-sealed`, exactly one SHA-256 pass. Persisted `positive-red-independent-auth.json` immediately after computing digest/size, before reporting or inspecting compile/source state. That receipt remains independent and unchanged.

- SHA-256: `b09f48ba38bdf471119f3b13e899efc1941bf92b1735020f4530d3b6f0a5acf1` — exact `artifact.json` match.
- Bytes: 467618248 — exact `artifact.json` match.
- No initial RED or old artifact was rehashed.

## Compile/source evidence

Retained `compile-result.json`: exit 0, source_unchanged true. `outcome.json`: COMPILED_NOT_EXECUTED, executed_cases 0. Parsed every line of retained compile.stdout: 665 compiler-artifact records, 75 build-script-executed records, 1 build-finished record (success true), 0 compiler-message records. Full compile.stderr read: six deprecated iced-wgpu-patched manifest lint-key warnings, one summary, successful test-profile completion in 2m 33s. Not warning-free.

Retained toolchain: Cargo 1.99.0 (5f94df478); rustc 1.99.0 (b940084d7), x86_64-unknown-linux-gnu, LLVM 23.1.1. No toolchain/build command rerun.

Built revision and inspected HEAD: `a1de57b4ea99f485bd77c430d9fea4301ed3764a`. Read-only git comparisons confirmed exact current/committed text equality for test file, both state files, C currency registration file and currency spec. Worktree contains unrelated untracked `.code-index.db` and `verification/`; no tracked modifications observed. Retained source digest is provenance only; source was not rehashed.

## Concrete coverage matrix

| Contract / feature | Source evidence | Proof level |
|---|---|---|
| Absence, strict number input, unrepresented numbers | tests/currency_war_mode_bonus.rs:32,43,57 | Compiled assertions only; runtime PENDING |
| Untainted wrapper/caller preservation; tainted denial before validation | tests/currency_war_mode_bonus.rs:71,89; retail-12-1-0 enabled in manifest | Compiled assertions only; runtime PENDING |
| All nine independently nullable flag pairs | tests/currency_war_mode_bonus.rs:125; IDs 90001–90009, plus absent 90010 | Compiled assertions only; runtime PENDING |
| Owned update/removal, scalar snapshot and currency isolation | tests/currency_war_mode_bonus.rs:156 | Compiled assertions only; runtime PENDING |
| Positive identity and numeric non-aliasing | tests/currency_war_mode_bonus.rs:197; i32 min/max, fractions, overflow, NaN/infinities | Compiled assertions only; runtime PENDING |
| Supplied secret identity and invalid secret type | tests/currency_war_mode_bonus.rs:223 | Compiled assertions only; runtime PENDING |
| Owned map scaffold | src/lua_api/state/sim_state.rs:573: HashMap<i32, (Option<bool>, Option<bool>)>; state.rs:492: HashMap::new() | Source inspected; no normal membership seed |
| Getter producer/public registration | c_currency.rs method list has no DoesWarModeBonusApply; src search returned only map declaration/init | Missing in inspected source; no getter proof |
| QuestUtils consumption/tooltip behavior | Not executed or inspected as an integration path | Not proven; separate caller dependencies |

Nine test functions total: five prior controls plus four supplied-state controls. Nine nullable combinations are assertions within one test, not nine separately executed tests. Test wiring: tests/integration.rs:1 invokes integration_tests!; crates/test-harness/src/lib.rs:7 includes generated declarations; crates/test-harness/build.rs:30–66 discovers top-level modules and emits declarations. This is source wiring evidence, not a test-list/run claim. TODO/FIXME/HACK/XXX counts in expanded test source: each 0.

## Runtime observation — one snapshot, no polling

Epoch directory inspected once for runtime outputs. No runtime.stdout, runtime.stderr, runtime-submission.json or runtime-result artifact present. Retained admission at 2026-10-12T08:00:13.018676+00:00: admitted false, available 26.84356689453125 GiB, load1 34.33, relevant_cargo empty. Inspected admission code requires load <24; retained load exceeds that threshold. runtime-blocked record: RESOURCE_BLOCKED, submitted false, executed_cases 0.

Exact observed execution count: 0. Passed/failed test counts: unknown, not 0/9 or 9/9. No test failure boundary, runtime exit code or RED execution can be asserted. Actual retained boundary is runtime admission before submission. Expanded positive RED is genuinely PENDING.

No builds, tests, binary runs, delegation, network, operational changes, waits or polls performed. Read-only local metadata/source inspection and requested proof-file writes only. Two inspection calls failed (Pyrun variables unavailable in the next evaluation; git cwd not inherited); corrected using explicit paths, without any artifact rehash or runtime reinspection.
