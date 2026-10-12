# Expanded Quest RED — independent artifact verification

Scope: only this positive-red epoch. No tests, builds, replays, delegation or old-artifact hashing. No parent-scope completion claim.

## Artifact authentication

Single streaming SHA256 pass authenticated `integration-sealed`: **467982416 bytes**, digest **4e6d1f18bec3b6c186d9380e128c8a1881fe45165034d9bdb5d22f4e70dd0dbf**. Both match `artifact.json`. Comparison persisted immediately in `positive-red-independent-auth.json`, before runtime/source inspection; no rehash. An initial evaluation failed on an undefined import before opening/hashing the binary; the subsequent self-contained evaluation performed the sole streaming pass.

This authenticates bytes against the supplied manifest, not a signature or an independently reconstructed source-to-binary provenance chain.

## Compile, execution and applicability

- `compile-result.json`: exit **0**, source_unchanged **true**. Full compile stdout parsed: **741 JSON records**, one matching integration executable record exactly equals artifact cargo_record, build-finished success=true. Compile stderr: six deprecated clippy manifest-key warnings; finished test profile in 2m08s. Cargo/rustc recorded as Arch 1.99.0; default retail includes retail-12-1-0, so both secret cases ran.
- Full `runtime.stdout`: **13 executed, 1 passed, 12 failed, 0 ignored, 0 measured, 10767 filtered**, 2.15s. Full stderr contains 12 matching panics and loaded QuestUtils closure provenance. Main epoch `runtime-terminal.json`: ActiveState=failed, SubState=failed, Result=exit-code, **ExecMainStatus=101**, MainPID=0. Its outer exit=0 means the status query succeeded, not the tests. Parent-directory runtime-terminal.json is unavailable.
- Runtime argv explicitly selects the sealed artifact, filter `quest_currency_war_mode::`, `--nocapture --test-threads=1`, with CASC/sound disabled and 90-second service runtime limit. Evidence shows completed assertion failures, not a compile failure or timeout.
- Pinned test commit **9e3fd32551e67e06653e6224e94be8f56a6da33b**; metadata/current HEAD **8a10302064ed91b8f0944eef475871902b311bf0**. source-applicability diff_exit=0 and empty diff. Independently read git diff for src/tests/Cargo.toml/Cargo.lock: empty; working status for those paths clean. Pinned test source obtained by git show is byte-equal to inspected working source (no source hashing). HEAD commit changes only two documentation files. Recorded test_source_sha256 is metadata, not independently reauthenticated here.

The observed RED is pinned-build evidence with corroborated source applicability; it does not prove a newly rebuilt current implementation or a GREEN producer implementation.

## Exact execution/failure boundaries

Names below omit `quest_currency_war_mode::quest_currency_war_mode_`. Rust helper panic line 13 wraps failed Lua assertions; their actual source boundaries are separately identified.

| Case | Result / reached boundary | Downstream proof boundary |
|---|---|---|
| `actual_vendor_bonus_and_each_independent_gate` | **FAIL** — Initial positive render returns ordinary reward only; expected additional +25% bonus (helper line 211; call line 392). | Desired-off, eligibility removal, world-quest-off, currency flag permutations, faction suppression/removal, +40% update and final lua_errors assertion all unexecuted. |
| `actual_vendor_duplicate_first_instances_are_per_currency` | **FAIL** — First render successfully asserts added=3, container=false, info=nil; bonus count 0, expected 1 (line 471; first call line 491). | Ordinary-row equality after bonus-count assertion, repeat-render reset, counts 2/3/2, faction suppression and final lua_errors assertion unexecuted. |
| `actual_vendor_inputs_are_environment_local` | **FAIL** — Both vendor environments created; first environment positive render has ordinary row but no +25% bonus (line 211; call line 521). | Second environment baseline render, later first/second toggles and independence assertions unexecuted. |
| `actual_vendor_ordinary_reward_baseline` | **PASS** — PASS: real loaded vendor public closure, added=1/container=false/info=nil, exact ordinary 7 Fixture Marks row, no recorded Lua errors (lines 74-89, 185-193). | No positive War Mode inference from this pass. |
| `desired_has_one_boolean_result` | **FAIL** — One-value assertion reached and passed; boolean type assertion fails (Lua source line 103; Rust panic line 13). | Initial false value, existing percentage=10 and separate default=10 assertions unexecuted. |
| `eligibility_requires_numeric_identity` | **FAIL** — First nonnumeric input string 901001 fails rejection assertion (line 123; Rust panic line 13). | Other nonnumeric inputs, missing/nil argument rejection and absent eligibility=false assertions unexecuted. |
| `faction_query_requires_numeric_identity` | **FAIL** — First nonnumeric input string 90001 fails rejection assertion (line 142; Rust panic line 13). | Other nonnumeric inputs, missing/nil argument rejection and one nullable absent relation assertions unexecuted. |
| `relation_queries_authenticate_tainted_secrets_first` | **FAIL** — First getter QuestCanHaveWarModeBonus with secretwrap(901001) in tainted addon fails denial assertion (line 169; Rust panic line 13). | Error substring/authentication precedence, wrapper preservation, declassification denial, taint preservation/restoration, invalid secret and currency getter controls unexecuted. Compound assertion does not identify which denial predicate failed. |
| `supplied_desired_is_live_independent_and_environment_owned` | **FAIL** — Both environments created and first desired=true supplied; first public desired=true assertion fails (line 232; Rust panic line 13). | Generic PvP separation, live false update, second desired=true and environment independence unexecuted. |
| `supplied_eligibility_is_not_world_quest_classification` | **FAIL** — Owned set populated with daily ID and i32 limits; first getter cardinality=1 passes, expected true for 901002 fails (lines 287-290; Rust panic line 13). | World-quest separation, i32 limit reads, nonaliasing numeric probes and eligibility remove/insert updates unexecuted. |
| `supplied_faction_projection_updates_removes_and_preserves_identity` | **FAIL** — Owned map populated; first getter cardinality=1 passes, expected faction 47 for 90001 fails (lines 328-331; Rust panic line 13). | Second currency/limits, original scalar capture, nonaliasing probes, update and removal assertions unexecuted. |
| `supplied_percentage_is_live_without_changing_default_query` | **FAIL** — Owned percentage=25.5 supplied; getter cardinality=1 passes, expected 25.5 fails (line 258; Rust panic line 13). | Default=10 assertion and live zero/default preservation assertions unexecuted. |
| `supplied_untainted_secrets_preserve_identity_and_caller` | **FAIL** — Owned eligibility and faction supplied; first wrapped quest expected true fails (line 375; Rust panic line 13). | Wrapped currency expected 47, invalid wrapped type rejection, secret identity preservation and untainted caller assertions unexecuted. |

## Real vendor consumer versus owned scaffold

Test loader (`tests/quest_currency_war_mode.rs:22-35`) loads the current-profile cached Blizzard_FrameXMLUtil addon closure, obtains `debug.getinfo(...,'S').source`, and logs **@Interface/AddOns/Blizzard_FrameXMLUtil/Mainline/QuestUtils.lua**. Baseline PASS proves that public closure renders the supplied ordinary reward and reports no recorded Lua errors. Tests do not replace the public QuestUtils consumer with a handwritten function.

Read cached retail vendor `QuestUtils.lua`: lines 683-693 independently gate desired mode, currency flags/first instance, world-quest classification, quest eligibility, and absence of faction relation. Lines 898-914 build per-currency first-instance state; lines 933 and 975-976 obtain percentage and append bonus. This is inspected cache source, not independently hashed runtime vendor provenance. Positive tests call this real consumer after direct Rust-owned fixture injection; all three positive consumer cases execute but fail at their **first positive check**. Later negative controls do not independently pass merely because the test name promises them.

`src/c_api/c_pvp/war_mode.rs` defines real owned fields `desired: bool` and `reward_bonus: f64`, default false/10; `SimState` holds initialized eligibility set and faction map. This is **input scaffold**, not working getters. `src/lua_api/globals/quest_surface/register.rs:51` still registers eligibility to `return_false`; `small_namespaces.rs:322-330` retains fixed 10-percent getters. Source search finds no dedicated desired/faction producer. Failed assertions do not establish the exact returned desired/faction value. Separate existing `DoesWarModeBonusApply` reads its own nullable two-flag map (`src/c_api/item_spell/c_currency.rs:79-95`); it does not wire the other producers or prove this expanded surface GREEN.

## Metadata discrepancies / conclusion

`outcome.json` is stale: “COMPILED_NOT_EXECUTED”, executed_cases=0 contradicts completed 13-case runtime and terminal exit 101. `runtime-submission.json.scope` also understates coverage (“no positive supplied caller/model ... run”); its actual argv and logs execute the expanded positive tests.

**Independent result: sealed artifact authenticated; expanded RED observed. Ordinary real-vendor baseline PASS; 12 failures with exact boundaries above. Getter implementation, downstream controls, parent scope and final acceptance remain unproven.** Machine-readable evidence: `positive-red-independent-proof.json`.
