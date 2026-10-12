# Independent final bounded verification — 2026-10-12

**PASS: current Retail/PTR ordinary quest-currency caller, four newly authenticated artifacts, fmt/default check. Full handoff OPEN.**

Source: `a5eaf4fc63e44027b1e9757b146b398c62e3a4b3`, repository `/home/osso/Projects/wow/wow-ui-sim`. Verify skill read completely. No builds, tests, replays, operations, delegation, questions, or repository edits. Only requested authentication receipts and final report/proof written under this verification root. Old sealed binaries never read or hashed. Full structured evidence and exact runtime streams: `proof.json`.

## Authentication — first action after locating current artifacts

Each current sealed binary streamed through SHA256 exactly once. Each comparison immediately persisted before moving to the next artifact. Four digest matches and four declared-byte matches; compiler records independently found in respective complete compile JSON streams.

| Epoch/profile | Artifact | Bytes | SHA256 |
|---|---|---:|---|
| `retail-final-compile-20261012T094557134417Z` | integration | 467770408 | `d2adb46168416854056170324b14385b85f21ca8be3d2ff7538df35b271b10ff` |
| Same | wow-sim | 407292424 | `6f0ab4b16b95a1dc151899e6abdaeba4cbcdd34c6621deceb64f3631f662657a` |
| `ptr-final-compile-20261012T095752975869Z` | integration | 467628448 | `9d156942dda7a19a871f8762ea6ce5813dfb85eeb79d7b5fbe32fa934e3c88ba` |
| Same | wow-sim | 414749760 | `c95f9371364b0d91ac77b51ee038412627d89b0d5dc78bc4fd7ea102cc684f02` |

Receipts in each epoch: `final-independent-auth-integration.json`, `final-independent-auth-normal.json`. No second authentication performed.

## Source applicability and compile proof

Both compilation receipts pin source `a5eaf4fc6`, observe docs HEAD `7e1364995`, and record empty `git diff a5eaf4fc6 -- src tests Cargo.toml Cargo.lock`, exit0. Current HEAD independently observed as `ce5b532c0fa593ab89f9e428b4494625b0c3a8e2`. Full drift inventory comprises 66 evidence/docs files, 7294 insertions/4 deletions. Independent expanded scoped diff including `src`, `tests`, `crates`, manifests/lock and patched iced manifest is empty, exit0. Current producers, test harness, and tests therefore unchanged by docs-only drift; untracked `.code-index.db`/`verification/` untouched.

Pinned Quest test digest in both epochs and current/check source: `ee2b5cedce9beaebc211353a01084cc6be127a7f23d1ea97450d209a851148a5`. Existing expanded tests originate at `9e3fd32551e67e06653e6224e94be8f56a6da33b`; current build source pin is not falsely described as their creation commit.

Both compile stdout streams decoded completely: 741 records each, 665 compiler artifacts, 75 build-script results, one successful build-finished, zero compiler-message diagnostics. Actual stderr read fully: compile exit0, source_unchanged=true; Retail 1m32s, PTR 1m51s. Integration and normal compiler records agree on features per profile, both nonfresh.

Retail uses default features: `client-retail`, `profile-retail`, `prefork-full-ui`, `default`, plus shared features below. PTR explicitly uses `--no-default-features --features sound,gui,casc,client-ptr`; resolved features add `client-ptr`, `forbidden-animation-aspects`, `retail-12-1-5`, `timed-signal-maps`, not Retail/default/prefork features. Shared resolved features: `aura-containers`, `aura-instance-enumeration`, `aura-xml-widgets`, `base-spell-relationships`, `casc`, `forbidden-aspects`, `gui`, `native-duration-formatting`, `numeric-rule-formatters`, `on-update-modes`, `player-cast-durations`, `retail-12-0-0`, `retail-12-0-5`, `retail-12-0-7`, `retail-12-1-0`, `rodio`, `sound`. Exact full arrays retained in proof.

Actual local toolchain receipts: `/usr/bin/cargo` 1.99.0 (5f94df478 2026-08-27), `/usr/bin/rustc` 1.99.0 (b940084d7 2026-09-28), x86_64-unknown-linux-gnu, LLVM23.1.1; not desktop pinned1.98.1 evidence.

## Exact current execution evidence

All Quest/flag stdout and stderr read fully; no ignored cases, failures, panics, or timeout outcome. Each execution invokes its epoch's authenticated integration artifact, timeout90, one test thread, nocapture. No milestone reruns.

| Current profile | Quest | Changed flags | Normal loaded executable |
|---|---|---|---|
| Retail | 13 PASS/0 FAIL, 2.31s, exit0; 10767 filtered | 9 PASS/0 FAIL, 0.75s, exit0; 10771 filtered | `[]\n`, exit0; 291 Blizzard addons; CLEAN, 0 unique/0 occurrences |
| PTR | 13 PASS/0 FAIL, 4.98s, exit0; 9945 filtered | 9 PASS/0 FAIL, 1.28s, exit0; 9949 filtered | `[]\n`, exit0; 283 Blizzard addons; CLEAN, 0 unique/0 occurrences |

**26 Quest + 18 changed-flag = 44 current-profile test executions, 22 unique test names.** Two normal executions separate. Initial/expanded RED and precursor totals excluded. Nine flag regressions justified by the changed shared validator, not unrelated scope expansion.

Full raw test names and output stored in proof; names match inspected current `tests/quest_currency_war_mode.rs` (13) and `tests/currency_war_mode_bonus.rs` (9). `tests/integration.rs` invokes the test-harness macro; `crates/test-harness/build.rs:27–68` discovers top-level test modules and emits their declarations. This is actual wiring, not an orphaned test file.

## Behavioral coverage matrix

| Boundary | Concrete assertions / evidence | Proof |
|---|---|---|
| Real ordinary vendor caller | Harness loads `Blizzard_FrameXMLUtil` with empty overrides; actual `QuestUtils_AddQuestCurrencyRewardsToTooltip` reports cached Mainline source six times/profile. Calls `(901001, GameTooltip, nil, nil)`; return `(1,false,nil)`, exact icon/count/name line `7 Fixture Marks` | PASS both profiles |
| Every bonus gate | Desired disabled; eligibility removed; world-quest classification false; `(nil,nil)`, `(false,true)`, `(nil,false)` flags suppress; `(true,false)` restores; faction47 suppresses, removal restores | PASS both profiles; current vendor `ShouldShowWarModeBonus` matches these gates |
| Percentage and duplicates | Exact green `+25%` then live `+40%`; duplicate rewards7/5/11 retained; bonus counts in order **1,1,2,3,2,1**, including repeat render and independent second currency | PASS both profiles |
| Owned state / numeric identity | Generic PvP independent; live desired/percentage changes; 25.5→0 with Default10; eligibility independent of classification; faction update/removal and original47 retained; two environments switch bonus ownership independently; i32 endpoints and fractional/nonfinite/out-of-range nonaliasing | PASS both profiles |
| Secrets / existing flags | Valid and invalid tainted secrets denied before type/lookup; wrappers stay secret, no unwrap/declassification, caller taint retained/restored; untainted supplied IDs/type failures preserve wrapper/value/context. All9 independently nullable flag combinations, updates/removal/currency isolation, two-result arity | PASS both profiles with actual `retail-12-1-0` enabled |

No mocked getter or vendor rewrite introduced by the producer source. Existing source/spec/wiring receipts reused from2635; duplicate-validator finding resolved by2643 source-equivalence/manual-readability follow-up. Current source bodies independently inspected: shared helper, WarMode state getters, eligibility and currency queries. Substantive live state reads and numeric validation; earlier registration-route proof reused only within unchanged source scope. Prior added-line anti-pattern scans: zero TODO/FIXME/HACK/XXX/suppressions; follow-up retains one validator and unchanged secret-first/two-return contract.

## Normal publication and hidden-failure check

Actual normal argv/script inspected, not just status/summary. Five raw public queries: desired=false, percentage10, eligibility=false for901001, faction=nil for90001, flags=(nil,nil) for90001. First four exactly one result; flags exactly two. All raw entries asserted functions; relation/flag string inputs rejected; separate Default10 unchanged. Default getter itself is not asserted raw/arity—do not extend that proof.

Normal stdout/stderr both read fully: exactly `[]\n`/exit0, zero unique/occurrence errors, zero `[exec-lua]` failure markers. `src/lua_errors.rs:47–72` executes the supplied script and routes failures through `call_error_handler`; `script_helpers.rs:526–550` sinks error before invoking user handler. Thus startup JSON alone was not used to excuse a swallowed assertion failure.

Runtime recipe explicitly sets CASC0/sound1. Normal recipe explicitly disables sound and uses `--no-addons --no-saved-vars`; logs confirm addons/SavedVariables/sound disabled and casc=false. Normal startup still reads edit-mode layout cache. Inherited environment not comprehensively snapshotted; no claim of a pristine host or full environment isolation. Environment ownership is established by the concrete separate-VM test.

## Checks, warnings, controller truth

Epoch `check-20261012T100335665643Z`: actual `cargo fmt --check` exit0, empty stdout/stderr; actual default `cargo check --offline --locked -j12` exit0, empty stdout, finished36.84s. Complete check stderr inspected. All14 source/test/manifest hashes independently recomputed from current tree equal recorded before/after; no relevant source change invalidates these proofs.

Exactly six inherited `iced-wgpu-patched/Cargo.toml` manifest deprecations per compile/check stream: `large-enum-variant`→`large_enum_variant`, `map-entry`→`map_entry`, `match-wildcard-for-single-variants`→`match_wildcard_for_single_variants`, `redundant-closure-for-method-calls`→`redundant_closure_for_method_calls`, `trivially-copy-pass-by-ref`→`trivially_copy_pass_by_ref`, `type-complexity`→`type_complexity`. Summary says generated6; not seven warnings, not warning-free, not new Rust diagnostics.

Final/check controllers decoded completely: worker JSON `type=completed`, no error, expected epoch consoles, empty controller stderr. Terminal receipts: `ActiveState=active`, `SubState=exited`, `MainPID=0`, `Result=success`, `ExecMainStatus=0`. Both independent worker and service evidence agree. Historical precursor controller instead has `type=error`, `runtime admission denied` despite service success; its zero PTR executions remain historical and are not promoted. Per-epoch `outcome.json` is compile-only stale `COMPILED_NOT_EXECUTED/0`; actual runtime/flags/normal outcomes and `green-outcome.json` prove later execution.

## Reused evidence and remaining boundaries

Prior source/readability2635 plus resolved2643 reused; manual metrics only (prior maximum body27/depth2, refactored query18/depth2). Automated readability/cognitive/cyclomatic metrics unavailable; no invented metric score. Existing RED and retention receipts reused without old seal access: initial5=1PASS/4FAIL; expanded13=1PASS/12FAIL; ordinary baseline solePASS, downstream gate assertions initially unreached. Retention previously108/108 copies, 2,889,750 bytes—**through precursor only**, not final stream retention.

Native quest/currency/faction catalogs, other reward/container/XP/money paths, active-mode/phase/toggle/setter lifecycle, earlier profiles, external verification and full handoff remain **OPEN**. Frozen historical full-suite3FAIL not rerun, cleared, or absorbed into bounded PASS. Public docs still say final queued; this report does not edit or reconcile them. **Main's final-stream retention/reconciliation is pending next; public packet is not claimed complete.**
