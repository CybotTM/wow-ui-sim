# Speak For Me producer source gate

Verified 2026-10-13. Target: `a8a148e98d8dc32cf81e789fc8926830c9ed0161`.

**SOURCE PASS; runtime GREEN PENDING.** Direct, bounded verification following verify/SKILL.md and rust-readability/SKILL.md. No delegation, tracked edits, compilation, test/runtime execution, binary hashing/authentication, deployment or operational changes.

HEAD advanced concurrently to `9b027baaf8931e1e3780a33b0a7fd098e9476f87` (wiki-only changes). All four inspected source/test/spec files compare byte-for-byte to target commit; proof.json records both revisions and SHA256 comparisons. This does not establish immutable whole-checkout identity during formatting.

## Artifact gate

- [EXIST] PASS: `src/lua_api/sim_substates/mod.rs` 754 lines; `src/c_api/c_voice_chat_channels.rs` 147; `tests/voice_speak_for_me_active_input.rs` 95; `docs/specs/voice-speak-for-me-active-input.md` 33.
- [SUBSTANTIVE] PASS: mod.rs:345–347 declares separate public `bool is_speak_for_me_active`; :375 initializes false as explicit simulator policy. API :56–60 reads `borrow_state(state)?.voice_chat.is_speak_for_me_active`, pushes `Val::Bool(active)`, returns `Ok(1)`. No argument extraction: zero-argument use requires no argument. This does not specify rejection of extra arguments. No channel/permission/transcription aggregation, mutations, events, setters or native inference added.
- [WIRED] PASS, source only: API :37–46 ensures existing `C_VoiceChat` namespace and explicitly registers `IsSpeakForMeActive`. Both module declaration `src/c_api/mod.rs:210–211` and existing caller `src/lua_api/globals/missing_surface.rs:278–279` are gated by `retail-12-0-5`. Existing path: `env_init/mod.rs:67` → `globals/register.rs:34–44,222–234` → `missing_surface::register_all` :75–81 → `register_social_namespace_surfaces` :266–279 → channel registrar. No new alternate registration path. `lua_bridge/table_builder.rs:47–55,58–80` installs a Rust closure directly using table.raw_set; `methods.rs:85–90` borrows environment app_data's SimState immutably. The query copies its boolean before pushing; no state snapshot captured in the registered function.
- [ANTI-PATTERN] PASS: zero TODO/FIXME/HACK/XXX markers in additions from bounded commits `159998966`, `c81f10375`, `a8a148e98`; no newly introduced empty implementation, suppression or fallback. Diffs add only supplied input/default, behavioral tests/spec, and explicit producer/registration. No vendor or static-vendor override introduced. Source-wide symbol scan finds only field, initializer, registration and producer references.

## Concrete coverage matrix

All current assertions below are authored source, **not GREEN execution evidence**.

| Case | Exact assertions/state | Proof level |
|---|---|---|
| Default test :8–24 | Rust function-type assertion; zero-argument call yields exactly one result, boolean, false (:18–20) | Source verified |
| Retained query :28–64 | Two independent environments retain function and false value; first input true yields retained/direct true, prior value stays false, one boolean result; transcription/permission remain false; second retained query stays false | Source verified |
| Independence :66–85 | Second input set true. First activity remains TRUE while transcription becomes true, both channel stores clear, active ID becomes None, connection status becomes 0. Assertions require retained query TRUE, transcription true, empty channels, nil active ID | Source verified; fixes pre-c81 confounded false-before-clear test |
| Final transition :87–94 | Only first activity changes false; first retained query false, second retained query still true | Source verified |
| Discovery | `crates/test-harness/build.rs:29–65,441–482` includes top-level test file, not excluded; `tests/integration.rs:1` invokes generated macro; test :3 feature-gates both cases | Source only; compiled discovery not run |

Limits: permission is observed false, not independently varied. Channel clearing/disconnection is a combined mutation, not separately isolated experiments. No explicit connection-status getter assertion; Rust fixture writes disconnected status. Retained value immutability is asserted after first true transition, not again at final false. Source producer has no dependencies on those other fields. No native default/semantics, events, setters, transport, security parity or cached consumer runtime acceptance established.

## RED root cause: callable is not a valid result

Fresh read of saved refined RED `runtime/stdout`, `stderr`, `result.json` under `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-red-refined-20261013T143003506802Z`: **0 passed, 2 failed, 0 ignored/measured, 10974 filtered; exit101**, saved revision `c81f10375`.

Both failures occur at later Lua exec unwraps (test :23 and :44), after reached Rust function-type checks. Retained case fails on first environment's `retained_speak_value == false`; second loop iteration and all input transitions are unreached. Default case's generic assertion message alone cannot distinguish arity/type/default failures or reveal actual value.

Source explains the result boundary: `runtime_surface_bootstrap.lua:65–77` generic namespace `__index` manufactures and caches a function containing `return nil` (one nil, not zero results). `:112–125,209–214,1584` attaches that metatable to C_* namespace tables after existing global registration (`env_init/mod.rs:67–70`). Thus function-type checks can pass without an activity producer; one nil fails boolean/default assertions. **One-nil explanation is source-derived, not a value printed by RED logs.** Current explicit raw registration supplies the state-backed boolean instead; runtime acceptance remains pending. Existing generic fallback was neither modified nor added by this change.

Primary local signature: cached retail `Blizzard_APIDocumentationGenerated/VoiceChatDocumentation.lua:567–575`, `IsSpeakForMeActive`, no Arguments section, one `bool` return with `Nilable = false`. Signature only; not native behavior/default or observed consumer execution.

## Formatting, readability and proof ledger

1. `rustfmt --check --edition 2024 --config skip_children=true src/lua_api/sim_substates/mod.rs src/c_api/c_voice_chat_channels.rs tests/voice_speak_for_me_active_input.rs`: **exit0**, full stdout/stderr empty. Changed mod/API/test only; child traversal disabled.
2. `cargo fmt --check`: run exactly once, **exit0**, full stdout/stderr empty. Whole formatting-discovered checkout scope at invocation, not compilation and not immutable target-tree proof. No broad check rerun.
3. Manual changed-lines-only Rust readability: **zero violations**. Producer is three straightforward statements with visible immutable borrow/stack output; field/default and mutations explicit; no new branching, deep nesting, opaque conditions, warning suppression or duplicated production decision. Test bodies below 200-line threshold. Existing unrelated module code not audited.
4. Target file hashes and byte comparisons persisted in proof.json; tracked `git diff HEAD --exit-code`: **exit0**, empty output. Formatting evidence remains valid for unchanged target files. Historical RED remains historical; current GREEN, startup diagnostics, preservation and compile/type checks are NOT RUN by explicit bounds.

Initial git probes failed outside repository (exit128); explicit command cwd resolved it. One later evaluation-local variable was unavailable; literal cwd resolved it. Neither error changed files or furnished proof. Only untracked verification artifacts written. report.md and proof.json read back after persistence.
