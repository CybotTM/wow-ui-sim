# Speak For Me supplied-input staging verification

Revision: `159998966ae5eb9a768b5ddc3658368961c1d276` (2026-10-13).

**SOURCE STAGING VERIFIED; behavioral RED pending. No runtime PASS or feature-completion claim.** Verification was direct, bounded and non-delegated. No Cargo, build, runtime, old binary, authentication or operational commands executed. Main compilation was not touched.

## Artifact checks

- [EXIST] PASS: `src/lua_api/sim_substates/mod.rs` (754 lines), `tests/voice_speak_for_me_active_input.rs` (82), `docs/specs/voice-speak-for-me-active-input.md` (33).
- [SUBSTANTIVE] PASS for input/test-only scope: public boolean input at mod.rs:345-347; explicit false default at :375. No aggregation or inferred producer. No `IsSpeakForMeActive` implementation/registration or other input references found in `src/`. Query implementation remains intentionally absent; not a runtime-wiring PASS.
- [WIRED] PASS at source-discovery level only: `crates/test-harness/build.rs:29-65` generates module declarations and exported `integration_tests!`; :441-475 discovers top-level `.rs` files and does not exclude this test; :478-482 uses its filename stem. `crates/test-harness/src/lib.rs:7` includes generated macro source; `tests/integration.rs:1` invokes macro. `Cargo.toml:178-180` defines integration target despite autotests=false. Test file :3 gates both tests on `retail-12-0-5`. No compiled discovery listing executed or generated build output inspected.
- [ANTI-PATTERN] PASS: changed Rust lines contain 0 TODO, FIXME, HACK, XXX markers; no empty bodies, suppressions, commented-out implementation or fallback paths introduced.

## Exact assertion coverage (source only)

| Case | Concrete assertions/state | Proof level |
|---|---|---|
| `voice_speak_for_me_active_input::speak_for_me_active_is_one_false_boolean_by_default` | :12 missing-query guard; :13 exactly one return; :14 boolean; :15 false, zero query arguments | Authored, not executed |
| `voice_speak_for_me_active_input::retained_speak_for_me_query_reads_live_independent_environment_input` | :25-34 two environments retain query and false snapshot; :37 sets first input true; :41-47 retained and direct true, old snapshot false, one boolean result, transcription and permission false; :51 second remains false | Authored, not executed |
| Same second case | :54-60 sets second true; :62-68 sets first false, transcription true, clears both channel stores, active ID None, connection status 0; :73-76 first retained query false, transcription true, empty channels, nil active ID; :81 second retained query true | Authored, not executed |

Both tests first explicitly assert query existence. RED execution must establish actual failure boundary; subsequent assertions have not been reached. Combined mutations in the second case do not independently prove that clearing channels/disconnecting while Speak For Me stays true preserves activity. Permission is observed false, not independently varied. These are coverage limits, not evidence of runtime failure. Retained-value immutability is explicitly checked after first true transition; false/true/false live-query transitions and cross-environment isolation are authored.

## Primary cached signature

Read-only local primary source: `<HOME>/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/Blizzard_APIDocumentationGenerated/VoiceChatDocumentation.lua:567-575`.

`Name = "IsSpeakForMeActive"`, `Type = "Function"`; no Arguments section; Returns contains exactly `{ Name = "isActive", Type = "bool", Nilable = false }`. Supports zero-argument, one non-nil boolean contract only. Does not establish native false default, activity semantics, transitions or consumer execution. Cache SHA256: `a5720ca95a63083caaba58b623dbe8a12e5ef19aac1ef88817e8b5b25faab9df`.

## Preservation and readability

Commit changes exactly three files, 119 additions, no deletions. Parent/current `VoiceChatState` Default blocks compared exactly after removing only new `is_speak_for_me_active: false` initializer: equality true. Existing voice defaults and seeded channel implementation unchanged in commit. No production voice API file changed.

Changed-lines-only manual rust-readability audit: mod.rs:345-347,375 and test :1-82. No readability violations found. New field/default self-descriptive; test mutations explicitly target state; no new non-test functions, complex boolean conditions, magic domain constants, deep nesting, suppressions or duplicated production logic. Test 2 stays below 200-line test threshold. Existing module functions not audited.

Direct formatting command (only changed mod/test, child traversal disabled):

```
rustfmt --check --edition 2024 --config skip_children=true src/lua_api/sim_substates/mod.rs tests/voice_speak_for_me_active_input.rs
```

Full command output: empty. Exit code: 0. Formatting PASS, not compilation or behavior PASS.

## Proof ledger

- Git commit/diff inspection: revision above, exact three-file stage confirmed. Current tracked diff empty at initial inspection.
- Source discovery/query scan/default comparison/assertion audit: current files at recorded hashes in proof.json; source evidence only.
- rustfmt command above: exit 0; covers only changed Rust files at recorded hashes.
- Behavioral RED/GREEN, compilation, runtime preservation, startup and cached consumer acceptance: PENDING/NOT RUN by explicit scope.

Initial command setup issue: Pyrun default command cwd was not repository; initial git inspection failed. Retried with explicit `.cwd(repo)`. A subsequent evaluation-local variable was unavailable; resolved by using explicit path. Neither produced source/runtime evidence nor changed repository state.
