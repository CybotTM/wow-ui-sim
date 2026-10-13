# Frame event removal — source and retained RED verification

Date: 2026-10-13. Canonical checkout: `<REPO>`.

**Bounded source/retained-RED audit PASS. Post-fix runtime GREEN NOT ESTABLISHED. Native parity UNPROVED.**

## Scope and revision

Read/followed verify and rust-readability skills. Acting as assigned verifier; no delegation. Inspected production commit `621649e56d8418d21c4539b26d787ad40c73431f` over scaffold `3b716b165f640c576ace9e6e46b80e6a175cb5e3`. HEAD is production commit; scoped working files have no diff against it. Scaffold adds spec, Lua fixture and Rust test; production commit changes only events.rs and the two owning specs. No tracked writes, commits, builds, Cargo, runtime replay, binary/archive/source-tree hashing or binary inspection performed. Pre-existing untracked `.code-index.db` and `verification/` left untouched.

## Coverage matrix

| Boundary | Concrete evidence | Proof level |
|---|---|---|
| Absent/unknown removal returns one false | events.rs:80–98 guards aspect before argument conversion, calls direct registry removal, pushes Bool(was_registered), returns Ok(1). Registry mod.rs:329–342 removes only the named membership/unit filter and returns false if absent. | Source only |
| Preserve other registrations | Registry removal addresses only supplied event; dispatch index removal occurs only when membership existed. Fixture checks PLAYER_LOGIN delivery before/after unknown cleanup. | Source only; fixture controls unreached in saved RED |
| Strict RegisterEvent/RegisterUnitEvent | events.rs:38–51 retains ensure_registerable_event for insertion and unit registration. No change to validator or callback gates. | Source only |
| Aspect guard and VM hlist | events.rs:82 guard precedes stack_val(state,2); lines 89–91 unregister VM hlist only if was_registered. Same ordering/condition as scaffold. | Source comparison; runtime preservation unexecuted |
| Unit filters / callback / Classic registration | Production diff does not change unit filter or callback functions. Existing Classic validator delegates to wrath under Wrath/Mists/Era/Anniversary cfg in src/event/valid_events.rs:20–27. | Source comparison only, not all-profile acceptance |

## Artifact checks

[EXIST] PASS — events.rs (782 lines), Rust test (13 lines), shared fixture (59 lines) and both owning specs present.

[SUBSTANTIVE] PASS — two direct registry-mutation helpers replace the operation enum/shared validating mutation. Rust test executes include_str!("fixtures/frame_unregister_unknown.lua") through WowLuaEnv::exec. Lua fixture contains actual registration, delivery, rejection, removal and exact return-arity assertions, not mocks.

[WIRED] PASS — text_attribute_event/mod.rs:571,578–579 binds RegisterEvent/UnregisterEvent to inspected functions. tests/integration.rs:1 invokes test-harness macro. crates/test-harness/build.rs:29–60 discovers top-level tests and emits modules; lines 441–449,462–476 include this .rs filename. Source wiring established, not a generated or compiled-test claim. admin.rs:427 registers A_Admin.FireEvent; admin_events.rs:16–25 calls dispatch_event_now. Fixture calls that real simulator input three times.

[ANTI-PATTERN] PASS — no new TODO/FIXME/HACK/XXX, warning suppressions, empty handlers or commented-out implementation in introduced Rust/fixture code. Existing unrelated TODO in events.rs is outside changed lines.

Rust readability: manual changed-line review; no introduced violations. Insertion/removal helpers have three parameters, four body lines and no nested branches; removal wrapper retains shallow early-return/conditional control flow; Rust test has four body lines. No metrics/audit executable run under requested command restriction.

## Fixture identity and saved RED

Current shared Lua fixture is byte-for-byte equal to `<PRIVATE_VERIFICATION_ROOT>/unregister-policy-red-current/probe.lua` by direct byte comparison, not hash. No replay.

Read complete saved packet at:
`<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-green-20261013T184746921697Z/runtime-unregister-policy-red/`

- stdout: 903 bytes, 10 lines; stderr: 15,537 bytes, 228 lines (read in two complete chunks).
- result.json: 672 bytes; argv.json: 1,740; admission.json: 2,410; launcher.txt: 309. All read completely.
- Saved source attribution: `38ecbd8ee5dfae4315ca98001b7e794e324b1066`, Retail, filter `frame-unregister-policy-shared-fixture`. Receipt is saved stat-only reuse; executable identity was not freshly inspected/revalidated.
- argv records systemd-run --user --wait, timeout 86, bwrap read-only source bind, network unshare, no addons/saved vars, CASC=0, sound disabled, private probe via --exec-lua followed by lua-errors. Admission allowed=true, worker maximum 4 GiB/high 3 GiB. This describes saved invocation, not a newly run command.
- launcher reports status=1/FAILURE, service runtime 7.128s; result launcher_exit=1. No timeout outcome.
- First pcall catches unknown-event registration error from UnregisterEvent. The next assertion fails at fixture line 6: `cached Base.lua:697 UnregisterEvent('PLAYER_EQUIPED_SPELLS_CHANGED') must not throw`.
- stdout has two count=1 error entries: original exec-lua assertion and its ScriptErrors wrapper. stderr ends FAILED, 2 unique/2 occurrences. These are one observed fixture failure plus wrapper, not two independent cases. No completion marker. All later arity/registration/delivery/removal controls unreached. This is saved genuine pre-fix RED, not proof of post-fix behavior.

## Documentation and links

Both owning specs updated in production commit with source policy and retained RED; neither claims fresh GREEN/native parity. frame-event-removal.md retains unchecked requirements/preservation and bounded Retail/PTR proof gap. forbidden-aspect-event-input.md changes unrestricted removal wording to link selected state-based policy, without widening security claims.

Verified all 11 local link occurrences in the two owning specs resolve to existing files (none have anchors). Includes five scaffold links (event-system twice, frame-unit-event-filter, forbidden-aspect-event-input, admin-api/events) and production link frame-event-removal. No broken new links.

## Executed proof ledger

Revision/scope: current scoped files equal 621649e56; no later relevant edits.

1. `rustfmt --check --edition 2024 --config skip_children=true src/lua_api/frame/methods/text_attribute_event/events.rs tests/frame_unregister_unknown.rs`
   Exit 0; stdout empty; stderr empty. Only these two files, child traversal disabled.
2. `git diff --check 3b716b165^ 621649e56`
   Exit 0; stdout empty; stderr empty. Range includes scaffold plus production commit.

No Cargo/build/runtime/other audit/check suite executed. Source inspection git commands are not behavioral proofs.

## Recommended existing preservation filters — source only, NOT RUN

Use exact integration test names; profile feature gates must enable cases rather than yielding zero tests.

### Now: direct affected boundary

- `forbidden_aspect_creation::event_registration_aspect_rejects_addon_callers_and_permits_secure_callers` — all eight event operations, tainted rejection/secure access (line 157).
- `forbidden_aspect_creation::event_registration_aspect_preserves_existing_listener_delivery` — rejected individual/unit/all cleanup and actual dispatch preserve prior memberships/filters (line 196).
- `forbidden_aspect_creation::event_registration_aspect_preserves_callbacks_when_replacement_is_rejected` — callbacks and unit-filter delivery survive rejected removal/replacement (line 252).
- `unit_event_second_unit::reregistering_and_unregistering_clear_the_second_unit_filter` — re-registration, UnregisterEvent and UnregisterAllEvents suppress both-unit delivery (line 44).
- `global_event_callbacks::unit_event_callback_actual_wrapper_lua_lifecycle` — actual pinned Event.lua wrapper, public Lua producer, exact callback removal (line 75; retail-12-0-0 gate).

### Later: complementary existing controls

- `forbidden_aspect_creation::event_registration_aspect_leaves_zero_mask_mutations_and_delivery_unchanged` (line 299).
- `forbidden_aspect_creation::event_registration_aspect_leaves_zero_mask_callback_delivery_unchanged` (line 345).
- `unit_event_second_unit::second_unit_filter_receives_modeled_player_and_target_health_events` (line 4).
- `global_event_callbacks::unit_event_callback_actual_wrapper_rust_lifecycle` (line 70; retail-12-0-0 gate).
- `global_event_callbacks::global_event_callback_event_lua_synchronous_lifecycle` (line 251; actual Event.lua, payload/arity/GC/removal; retail-12-0-0 gate).

These recommendations do not establish preservation or cover Classic permissiveness at runtime. Post-fix new-fixture Retail/PTR execution, profile-specific preservation and native removal parity remain open.
