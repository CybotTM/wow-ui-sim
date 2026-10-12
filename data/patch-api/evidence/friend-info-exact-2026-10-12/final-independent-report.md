# Final independent additive proof — exact Friend slice

**PASS — bounded simulator slice. Not native, full-suite, performance or full-handoff acceptance.**

2026-10-12. Producer `be9c734a4`; security fixture `6ec0c1cb8`; observed HEAD `05fddbf399640fdeac345e25a1dcae9a0541cacd` is documentation-only relative to the fixture's relevant source. Read verify skill. Only read-only evidence inspection; no Cargo/test/runtime launch, delegation, deployment, repository edits or seal rehashing. Earlier reports remain historical and untouched.

## Precise execution coverage

| Evidence epoch | Actual output / result | Credited proof |
|---|---|---|
| Retail122455 | Original exact group **4PASS/1FAIL**, exit101 | Reuse only four unchanged PASS cases: alias, live/isolation, types/miss, whole-addon popup. Original secret failure retained. |
| Retail122910 | Secret1, legacy2, seeded1; all exit0, **4PASS/0FAIL** | Newer fixture/runtime proof. Not eight cases in this invocation. |
| Retail normal123242 | exit0; **291** Blizzard addons; stdout `[]`; CLEAN,0 unique/0 occurrences | Loaded raw-publication/default-state/@file probe. |
| PTR runtime124813520393 | Exact5+legacy2+seeded1; all exit0, **8PASS/0FAIL**; normal exit0, **283** addons, `[]`, CLEAN0/0 | Newer PTR execution of all eight names using previously authenticated123249 seals. |
| Check124825577210 | fmt exit0, default cargo check exit0;9/9 source hashes unchanged | Saved formatting/default-profile check evidence; six inherited manifest warnings. |

**12 newer executions (Retail4+PTR8) +4 older reused PASS executions =16 credited scope executions across8 unique test names.** No16-new claim; no same-epoch Retail8 claim. This verifier itself ran zero tests. Exact names, summaries, commands, full stdout/stderr and result receipts retained in `final-independent-proof.json`.

## Behavioral matrix and limits

- **Secret contract:** authentic secret identity; untainted wrapped-name exact fields/table/one-result; wrapped numeric rejection; tainted secret rejection; ordinary tainted-name acceptance; caller taint preservation and restoration. Retail122910 and PTR124813 secret test PASS. The6ec delta removed only two `canaccessvalue` helper predicates, not getter assertions. No producer/helper fix claimed. Helper/native parity remains separate.
- **State/input/alias:** supplied exact-name lookup, mutable notes, independent environments; missing/nil/boolean/table/function rejection; unknown-name one-nil policy; raw/ordinary/raw alias absence. Retail four unchanged older passes reused; PTR current exact group PASS. Plain numeric/userdata and malformed UTF-8 input coverage not established; simulator miss arity/UTF-8 policy not native parity.
- **Whole vendor popup:** actual `tests/friend_info_exact.rs:128-238` loads complete Blizzard_StaticPopup_Game/FriendsFrame addon closure from profile cache, validates root load and Mainline handler provenance, opens public StaticPopup_Show, checks real edit-box prefill/hide, mutates note and reopens, asserts empty recorded errors. Retail older popup PASS and PTR new popup PASS execute these unchanged assertions. No monkey patch or fake consumer. PTR stderr retains guarded nil-probe missing-requirement observations; they are not discarded or represented as absent. Test requires actual loader warnings empty and recorded Lua errors empty.
- **Compatibility:** `friend_list_who` two cases and `system_api_seeded` one case PASS on Retail/PTR. Earlier-profile/indexed-provider and C_Social preservation remains source-equivalence evidence, not runtime proof for all profiles.
- **Normal loaded probes:** both successful submissions use `--exec-lua @.../normal-probe.lua`, prior authenticated normal seals, `--no-addons --no-saved-vars`,90s timeout and sound disabled. Probe checks raw exact publication, raw/ordinary alternate absence, seeded Arthax empty note, Durotan `old guildie`, one table/nil result and unknown-name nil. Full stderr ends CLEAN0/0, with no exec-lua read/probe error. Counts291/283 were read from actual stderr, not inferred.

## Source applicability and reusable proof

Current read-only Git: `rev-parse HEAD`, `status --porcelain`, `diff 6ec0c1cb8 -- src tests Cargo.toml Cargo.lock`, `diff be9c734a4 6ec0c1cb8 -- src Cargo.toml Cargo.lock`, and `diff 6ec0c1cb8 -- docs/specs/friend-info-name-query.md`; all exit0. Relevant source/test/Cargo diffs empty; spec differs only introductory progress narrative. Existing untracked `.code-index.db` and `verification/` preserved. Current hashes match saved PTR runtime/check before/after **9/9**: four producer/module paths, three tests, Cargo.toml and Cargo.lock. This is bounded identity, not attestation of every build input or transient state.

Reuse original `source-proof.json`/readability report only for unchanged be9 producer source conformance. Reuse `secret-fixture-delta-proof.json`/report for6ec contract retention/readability. Historical HEAD/test identities remain historical. Current runtime receipts now prove the listed executed assertions; do not relabel earlier source-only reports as execution receipts. No fresh readability metric certification.

Reuse PTR123249 `ptr-compile-independent-proof.json`, report and first-auth integration/normal receipts. Matching manifests and current PTR inode/size/mtime/ctime confirmed without opening/hashing seals. Integration467661672 bytes; normal414744392 bytes. Existing Retail auth receipts reused, manifest digest/size and current size match; their older schemas lack PTR-style complete stat snapshots, so no invented stat equivalence. Saved compile streams fully parsed:741 records each, build-finished success=true, exits0. PTR compile selected client-ptr, not retail. Saved toolchain Cargo/rustc1.99.0. Final check is default profile, **not a separate PTR cargo check**. Compile/check stderr retain six deprecated iced-wgpu-patched manifest lint-key warnings; not warning-free.

## Admission and terminal controller

Original PTR compile epoch's runtime predicate required6GiB/load<24/no relevant Cargo. Its six observations exhausted12:34:40–12:44:18 UTC; terminal RESOURCE_BLOCKED,0 runtime cases. Preserve that record, not reinterpret it as executed.

First runtime-only retry124636 was **declined**, safe=false, available16.01324462890625GiB, no execution; controller records `Runtime-only safety admission denied`. It was not admitted merely because available memory approximated the16GiB ceiling.

Retry2 changed only runtime admission for nonperformance evidence: prior controller must be terminal MainPID0/exited; available>=20GiB, no relevant Cargo, memory full PSI avg10<5; CPUQuota200% (2CPU), MemoryMax16G and per-command90s bounds. Arbitrary load<24 gate omitted for this runtime-only scope. No producer, fixture/assertion, seal or profile change; no performance claim. At12:48:13.541603 UTC actual available30.315799713134766GiB, load24.28, no relevant Cargo, memory full avg10=0.00: old capacity admitted=false but new safe=true. One admitted observation; six bounded retries configured, not six successful attempts. Final default check separately admitted under original check predicate at12:48:25.590208 UTC (30.5914GiB/load23.56).

Actual retry2 submission sets CPUQuota200%, MemoryMax16G, KillModecontrol-group, Typeoneshot/RemainAfterExityes and agents.slice. Request matches executed worker byte-for-byte; controller stdout type=completed names exact runtime/check epochs; stderr empty. Fresh read-only `systemctl --user show wow-friend-info-exact-runtime-retry2-20261012t124813411684z --property=MainPID,SubState,Result,ExecMainStatus,CPUQuotaPerSecUSec,MemoryMax` exit0: **MainPID0, SubStateexited, Resultsuccess, ExecMainStatus0, CPUQuotaPerSecUSec2s, MemoryMax17179869184**. No active controller inferred from submission success alone.

## Failure history and exclusions

Initial RED120059:0PASS/4FAIL exit101. Corrected-readiness RED121320:0PASS/4FAIL exit101; absent exact getter and real Mainline nil-info/prefill boundary; reopen unreached. First GREEN122455 remains4PASS/1FAIL exit101. Separate helper diagnostic failed after successful untainted secret getter at canaccessvalue; not retroactive full-secret proof. Retry1 inline normal argument failed CLI parsing exit2 before loaded UI/probe; successful @file retry adds evidence, does not erase failure. Original PTR six load/resource blocks and unsafe16GiB admission refusal remain explicit.

No native client, full suite, all-profile runtime, performance or full-handoff acceptance. Spec still describes runtime acceptance pending; this verifier does not update repository progress or perform handoff. Only these two new final proof artifacts written, exclusively, outside repository. Internal exploration/schema/nonpersistent-variable errors produced no repository writes or launches; corrected evidence composition is self-contained.
