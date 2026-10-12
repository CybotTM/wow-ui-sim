# War Mode profile-gate source follow-up

Date: 2026-10-12. Repository: `/home/osso/Projects/wow/wow-ui-sim`.
Target: `91324b10380f9be3e8d196491e9a2d20bc597fa1`.
Mode: bounded source equivalence/profile gating and manual changed-line readability. Both requested skills read completely; verifier acted directly without delegation. No runtime/current-final acceptance claim.

## Exact scope

Full commit patch and inventory inspected: ONLY `src/c_api/c_pvp.rs` (3 additions, 2 removals) and `src/c_api/c_pvp/war_mode.rs` (0 additions, 5 removals). Five repeated attributes disappear from the shared module's imports, registration function and two getters. The sixth occurrence, at the namespace entry, becomes one `if cfg!(any(feature = "client-retail", feature = "client-ptr"))` gate. Net change: minus four lines.

`war_mode.rs` at the parent, after removing exactly those five attributes, equals target text exactly. Working-tree diff against target for both changed paths is empty. Comparison from `a5eaf4fc63e44027b1e9757b146b398c62e3a4b3` across the prior eight producer paths plus state and both test files lists only these two changed paths; their diff is exactly this gate cleanup. Supporting registration/provider/bridge/Cargo paths and `state/sim_state.rs` have empty scoped diffs. No new getter/helper test or state-field changes observed in that scope.

## Coverage matrix — source only

| Check | Literal evidence | Result / limit |
|---|---|---|
| Existing entry and order | `c_api/mod.rs:381-393` reaches `c_pvp::register_c_pvp_surface`; `c_pvp.rs:32-46` retains ensure namespace → conditional War Mode → catalog → active brawl → patch surface → training grounds | PASS source; only gate spelling/placement changed |
| Key registration and bodies | `war_mode.rs:12-32`: IsWarModeDesired registered first, then GetWarModeRewardBonus; same named functions and `?` propagation. Bodies read `war_mode.desired` and `war_mode.reward_bonus`, push Bool and Num respectively, each returns `Ok(1)` | PASS exact source equivalence after attribute removal |
| Defaults and input representation | `war_mode.rs:6-10,35-44`: bool/f64 fields, desired false, reward_bonus 10.0; `state.rs:383` initializes same default; `state/sim_state.rs:460` same owned field | PASS unchanged simulator policy; no native-default claim |
| Current bundles | `Cargo.toml:150-159`: client-retail bundles profile-retail and current epoch; client-ptr bundles PTR epoch. `c_pvp.rs:34-36` uses the SAME client-retail/client-ptr predicate as parent | PASS source: true branch publishes shared-input getters for those bundles |
| Older/historical paths | Neither bundle selected → gate false; historical profile-retail/epoch alone does not satisfy it. Wrath/Mists/Era/Anniversary/Forever do not satisfy it. `small_namespaces.rs:129-130,328-332` still registers old reward provider under the complementary predicate and pushes constant Num(10.0), Ok(1) | PASS source: no shared-input getter backport; older canned path unchanged |
| Separate Default and namespace retention | `small_namespaces.rs:125-128,323-326` retains unconditional Default registration returning Num(10.0), Ok(1). `helpers.rs:49-76` reuses existing namespace; `table_builder.rs:47-80` installs individual keys, not replacement namespace | PASS unchanged source; no dynamic namespace-state claim |
| Quest/currency/helpers/tests | Scoped a5eaf-to-target diffs exclude all changes outside the two gate files. Prior source and validator follow-up evidence remains applicable to those unchanged paths | PASS applicability only; no test execution or artifact authentication |

The only Rust source call to `war_mode::register` found under `src/` is `c_pvp.rs:35`. Compiling shared Rust implementations into all profiles is not the same as invoking their registration or publishing them to Lua. `cfg!` retains the branch body for type checking even when its constant condition is false; cross-profile compilation success was NOT checked here. No claim that every older public lookup is nil: existing canned/default/dynamic providers are outside such an absence claim.

## Rule falsification and readability

Repo `AGENTS.md:248-258` identifies repeated cfg predicates as an extraction/consolidation signal. The parent really has five identical attributes in the shared module plus its caller predicate. The cleanup removes this repetition using the existing shared module and one publication boundary. It is not literally a newly extracted module with `#[cfg(any(...))]` on its declaration, as the rule's example says; the explicit requested all-profile implementation/conditional publication design uses a `cfg!` caller gate instead. No older API expansion follows from that distinction.

New changed-line readability violations: none found. The positive two-feature condition is explicit, calls the effect-revealing `register` function, preserves fallible propagation, and adds no hidden work, mutable accumulation, unexplained literals or fallback. Manual updated metrics: `register_c_pvp_surface` has 13 body lines, maximum brace depth 2 including function body; the new ordinary if contributes one decision (local cognitive/cyclomatic approximations 1/2, not parser-derived). Shared `register` retains 7 body lines; each getter retains 3; their brace depth remains 1. Below skill limits: non-test body 30, nesting over 3 to flag, cognitive over 15, cyclomatic over 20. No metric tool or check command run. Unchanged-code prior manual evidence reused, not remeasured; user-supplied `2635+2643` reference is not independently authenticated as a separate metric artifact.

All three added lines manually inspected: zero TODO/FIXME/HACK/XXX, clippy/dead-code allowances, empty error handling, placeholder bodies or commented-out implementation. No warning suppression introduced; this is not a compiler-warning claim.

## Evidence reuse and exclusions

`source-readability-report.md` / `source-readability-proof.json` and `validator-followup-report.md` / `validator-followup-proof.json` supply unchanged-path source evidence; validator duplication was already resolved in the latter. Older a5eaf flag-query9 proof is reusable ONLY for its unchanged helper/query/test scope, not as current gate/runtime evidence; this review does not reauthenticate artifacts or claim a current pass.

User identifies queued main-owned unit `wow-quest-currency-profile-gate-20261012t102504530227z` for Quest13 plus normal Retail/PTR runs. No polling, status check, cancellation, worker inspection or adoption of their results performed.

**Outcome: PASS bounded source equivalence/profile gating; zero new changed-line readability findings.** Builds, tests, cargo/check/fmt commands, operations, binary reads/digests, delegation, questions and repository edits excluded. Only the requested report and an additive source-follow-up record in `proof.json` written. Existing proof remains historical evidence for its own source revision; this record does not upgrade its runtime/final claims to the target.
