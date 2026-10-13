# Retail SpellDisplay removal — saved runtime audit

## Verdict

PASS for the bounded saved simulator runtime scopes: **28/28 tests**, zero failed/ignored/measured, plus the normal CLI startup probe. Nine finished scopes have actual saved launcher `code=exited, status=0/SUCCESS`, `Finished with result: success`, and result `launcher_exit: 0`. Not a native/render/full-parent acceptance verdict.

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-removal-green-20261013T193523870575Z`.
Saved source claim: `621649e56d8418d21c4539b26d787ad40c73431f` (source621). No independent source hash performed. Followed `<HOME>/AgentConfig/skills/verify/SKILL.md` as the assigned verifier; artifact mode adapted to existing finished runtime proof. No build, runtime launch, retest, hash, delegation, wait, or tracked edit. All writes confined to this report directory.

## Finished scope matrix

Each stdout/stderr was read completely, including normal startup stderr in three contiguous reads. Byte lengths independently match result metadata. Full exact streams, argv, admission, launcher, and result records are copied under `proof/<scope>/`. `proof.json` contains exact passing test names and independent audit facts.

| Scope | Tests passed | Filtered | Test time | stdout/stderr bytes | Actual service runtime / memory peak |
|---|---:|---:|---:|---:|---|
| runtime-spell-display-post-removal | 9/9 | 10985 | 6.33s | 1209 / 6394 | 6.436s / 1.7G |
| runtime-removal-policy | 1/1 | 10993 | 0.20s | 228 / 683 | 248ms / 97.9M |
| runtime-registration-aspect | 5/5 | 10989 | 0.56s | 672 / 3195 | 592ms / 65.4M |
| runtime-unit-filter | 2/2 | 10992 | 0.29s | 318 / 1372 | 336ms / 65.6M |
| runtime-normal-startup-post-removal | CLI probe, no Rust cases | — | — | 33 / 14705 | 5.991s / 1.5G |
| runtime-text-state-preservation | 7/7 | 10987 | 4.05s | 942 / 5133 | 4.129s / 1G |
| runtime-speak-consumer-preservation | 1/1 | 10993 | 5.97s | 224 / 6955 | 6.110s / 1.3G |
| runtime-speak-input-preservation | 2/2 | 10992 | 0.63s | 328 / 1926 | 681ms / 86M |
| runtime-callback-lifecycle | 1/1 | 10993 | 0.55s | 201 / 642 | 683ms / 86.9M |

## Reached assertions: source-to-test evidence

References below are relative to the epoch's `source/`; copies preserved under `proof/source/`. Passing exact case names plus unconditionally executed assertion code establish these reached boundaries, not assertions printed individually by the runner. No skipped cases or source conditional early-return path in these cases was found. `tests/widget_discrete/fixtures.rs:40` executes Lua, panics on failure, and checks accumulated Lua errors after every `assert_lua` phase. `src/lua_api/env.rs:123` propagates `exec_rilua` errors.

### SpellDisplay: all nine cases

| Source test | Reached boundary |
|---|---|
| tests/widget_spell_display/queries.rs:7 | Complete exact 14-field outer/16-field nested snapshot; fresh nested tables; caller mutation isolation; old snapshot survives replacement and deletion; absent getter returns exactly one nil. |
| queries.rs:73 | Negative/fractional/zero/large numeric keys retained without truncation; -0 equals 0; missing/nil/string/bool/table/function arguments rejected with numeric-ID diagnostic. |
| queries.rs:103 | Same ID in two environments remains isolated. |
| feed.rs:46 | Committed record and membership visible inside real event observer; two immutable nested update snapshots; one full update on membership removal; second removal false; visualization survives removal. |
| feed.rs:114 | Unknown set/type14 mismatch rejected without mutation, registration, or event notification. |
| security.rs:9 | Actual VM secret-number IDs accepted untainted, rejected tainted for present and absent IDs; no tainted declassification; public returned fields; retained snapshot and untainted recovery. |
| consumer.rs:106 | Real cached constructor/type13 registration, event-driven activation, same-frame incremental text/tooltip/stack/tint/earned/width changes, Hidden release, pool restoration, full reconciliation after removal, retained independent visualization and snapshot; actual OnUpdate clears dirty layout and verifies pool/showing counts 0→1→1→0→1→0, no timers. |
| consumer.rs:191 | Container/frame attachedUnit player propagates as consumer context, not an added visualization getter field. |
| consumer.rs:209 | Zero spell initially lacks hadSpellID; host record replacement emits no feed event; actual XML OnUpdate polls to Crusader Strike 35395 on the same consumer frame. |

`fixtures.rs:61` establishes the supplied getter table before cached construction/security helpers. `consumer.rs:11` loads the profile cache addon closure, asserts Blizzard_UIWidgets loaded, checks grounded enums and catalog spell, constructs the actual UIWidgetContainerTemplate and registers UPDATE_UI_WIDGET/UPDATE_ALL_UI_WIDGETS. World-loot pip branch is deliberately excluded. `src/c_api/ui_widget_manager/spell_display.rs:11,25` registers Retail/PTR getter, looks up state, and projects fresh nested tables.

### Removal / registration / filter controls

- `tests/frame_unregister_unknown.rs:9` executes the entire included `tests/fixtures/frame_unregister_unknown.lua`. Cached `PLAYER_EQUIPED_SPELLS_CHANGED` and invented unknown removal return exactly one false without throwing/registering; known PLAYER_LOGIN still delivers before/after cleanup; unknown RegisterEvent/RegisterUnitEvent still fail; known removal returns exactly one true and stops delivery. Final `FRAME_UNREGISTER_UNKNOWN_POLICY_DONE` appears in saved stderr.
- `src/lua_api/frame/methods/text_attribute_event/events.rs:81` gates UnregisterEvent by EventRegistrations before removing existing state; only previously registered events trigger hlist removal. RegisterEvent/RegisterUnitEvent retain checked admission at lines 24/44/99. This is simulator policy, not established native behavior.
- `tests/forbidden_aspect_creation.rs:157,196,252,299,345`: eight operations deny tainted callers while permitting secure callers; rejected mutations preserve individual/unit/all listeners and filters; callback replacements do not become active; original callbacks survive; zero-mask delivery/unregistration and callback controls still work. All five exact cases passed.
- `tests/unit_event_second_unit.rs:4,44`: modeled player heal 50000→70000 and target damage 50000→35000 deliver only matching first/second units; focus suppressed; single-unit re-registration replaces second filter; UnregisterEvent/UnregisterAllEvents stop both-unit delivery.
- `tests/global_event_callbacks.rs:75` calls lifecycle helper at line48, loading actual cached Blizzard_SharedXMLBase/Event.lua at line210. Lua FireEvent preserves three payload values after stripping nil owner; player/target callback counts progress independently; each handle unregister stops its matching callback. Exact-token simulator policy only; no native alias or full callback suite claim.

### Existing Text/Speak regression

- `tests/widget_text_state/queries.rs:7,80,115`, `feed.rs:46,103`, `security.rs:9`: exact 21-field record, fresh snapshots, one nil on absence/deletion, replacement without query clamping, strict numeric IDs, environment isolation, event-before-query visibility/retention, rejected feeds without side effects, actual secret-number taint/recovery controls.
- `tests/widget_text_state/consumer.rs:145`: actual cached type8 constructor/event/pool/layout activation, update, Hidden, restore, membership removal. Checks text/tooltip/width, measured height and bottom-padding clamp, font/justify/color/order, retained record and snapshot. Simulator geometry assertions are not render/pixel proof.
- `tests/voice_speak_for_me_consumer.rs:28`: cached ChatFrame3/editBox constructor and live direct consumer decision across inactive/unmuted → active/unmuted → active/muted → restored inactive/unmuted. Checks disableActivate, prompt text/color, hiding, independent activity; accumulated Lua errors checked at every step. No voice transport/native/event acceptance.
- `tests/voice_speak_for_me_active_input.rs:8,27`: one false boolean default; retained query sees live independent environment input; saved boolean unchanged; transcribing/channel/connection state does not overwrite supplied speak activity.

## Full stderr classification and normal CLI probe

SpellDisplay, registration-aspect, Text-state, Speak-input, and callback-lifecycle stderr contain startup timing records only. Removal adds its final assertion marker. Unit-filter adds modeled INSTANT spellcast logs for 85673 and 35395. No printed failure/panic/budget diagnostics in these streams.

Speak-consumer stderr contains startup, post-load workaround/restoration and EditMode timing; not silent stderr, and not evidence of unmodified whole-runtime vendor behavior. Source assertions separately require empty accumulated Lua errors.

Normal startup stderr contains the entire startup: font fallback with CASC disabled; sound disabled; SavedVariables/addons disabled; **read-only WTF EditMode cache still loaded despite --no-saved-vars**; 291 Blizzard addons; bytecode cache 1569/1569 hits; post-load workarounds twice; GC restart. Final exact diagnostic:

```text
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```

Exact stdout:

```text
SPELL_DISPLAY_NORMAL_PROBE_OK
[]
```

Saved argv places the marker after assertions that SpellDisplay getter exists and unknown ID761999 returns exactly one nil, TextWithState unknown ID returns exactly one nil, and SpeakForMe query is a false boolean. This proves those normal-startup query assertions and reported clean Lua errors, not supplied-record consumer/native/render acceptance. `--no-addons` excludes third-party addons, not Blizzard UI.

## Launcher admission and reused receipts

Every saved admission has Retail profile, allowed=true, worker MemoryMax=4294967296 and MemoryHigh=3221225472 bytes. Independently recomputed `memory.current - half_clean_file_and_slab_reclaim + worker_max` equals projected for all nine scopes; projected remains below saved parent memory.high/max (42949672960/45097156608). Host available is a separate quantity; not incorrectly compared against parent projected usage. Raw memory/CPU/I/O pressure and memory.stat retained verbatim in each admission copy; historical admission, not current resource health.

Actual argv uses systemd-run --user --wait, agents.slice, MemoryMax4G/MemoryHigh3G, CPUQuota200%, KillMode control-group; timeout86 seconds; read-only bwrap source overlay/target, isolated network, tmpfs /tmp, WOW_SIM_CASC=0 and WOW_SIM_NO_SOUND=1. Integration scopes run the sealed integration executable directly, exact filters, --nocapture, --test-threads=1. Normal startup runs sealed wow-sim directly, --no-addons --no-saved-vars --exec-lua … lua-errors. No builds in audited runtime commands. Every saved launcher reports zero swap peak and success, not timeout termination.

Independent stat-only read at audit time matches saved receipt before/after, seal, and runtime receipt_stat_reused in every scope:

| Artifact | st_dev | st_ino | st_size | st_mtime_ns |
|---|---:|---:|---:|---:|
| artifacts/deps/integration-9f07e4a55f2bbbf7 | 66306 | 142479842 | 470982048 | 1791920372989653997 |
| artifacts/wow-sim | 66306 | 142479845 | 408320640 | 1791920284944505512 |

Saved receipts claim PASS, one historical hash read, matching seal digest; digest strings agree. **No new hash read.** Stat equality is reuse evidence, not cryptographic content verification. Seal explicitly excludes independent authentication of external libraries/cache/settings and source archive. Source-to-binary association remains inherited saved evidence, not independently rebuilt/rehashed here.

## Unproved / excluded

Native WoW parity, render/pixel acceptance, full-parent acceptance, PTR/other profiles, native token aliases, world-loot branch, broad/full test suite, Rust-fired callback sibling and other callback cases remain unproved by this audit. Default-check separate pending epoch not inspected or waited on. No newly appearing unfinished scope launched or waited on.

## Delivered proof and readback

`proof.json` records nine audited scopes, 28 exact passing case names, stat/admission checks, stream classifications and limitations. `proof/` preserves 81 complete evidence/source files. Every copied file was immediately read back byte-for-byte; proof.json immediately read back equal. This report is immediately read back after writing. No source/executable hashing performed, no tracked edits.
