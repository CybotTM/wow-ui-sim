# Supplied Speak For Me activity

`VoiceChatState.is_speak_for_me_active` supplies independent Speak For Me activity. False initialization is simulator policy, not a native default. State-backed query registered after genuine result-boundary RED; bounded Retail query/preservation/startup/default-check proof passes. Bounded cached consumer acceptance independently passes; initial GREEN artifact retention PASS; path hygiene FAIL and current follow-up PENDING. Existing [voice transcription contract](voice-transcribing-input.md) remains distinct.

## What it must do

- [x] Zero-argument `C_VoiceChat.IsSpeakForMeActive()` returns exactly one non-nil boolean equal to the current supplied input; default false is explicit simulator policy.
- [x] Retained query functions read live false/true/false transitions; retained values remain snapshots and environments are isolated.
- [x] Activity is independent of transcription, permission and channel state; clearing channels or disconnecting does not infer activity.

### Cached Retail edit-box consumer

- [x] The unchanged cached consumer and real ChatFrame3 edit box load through the existing full constructor; the entire runtime-error collection stays empty throughout the test.
- [x] Unmuted false activity disables activation, clears the prompt and hides a previously shown edit box. Unmuted true enables activation and sets `REMOTE_TEXT_TO_SPEECH`; no automatic Show behavior is required.
- [x] Muting preserves queried activity true while disabling activation and displaying `VOICE_TRANSCRIPTION_MUTED` in `RED_FONT_COLOR`; unmuted false restoration clears the prompt and hides again.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/sim_substates/mod.rs`: independent supplied input/default.
- `src/c_api/c_voice_chat_channels.rs`: explicit state-backed C_API query/registration.
- `tests/voice_speak_for_me_active_input.rs`: two behavioral cases under `retail-12-0-5`.
- `tests/voice_speak_for_me_consumer.rs`: one GUI/capability-gated real cached edit-box consumer case.

## Tests asserting this spec

Generated integration filter `voice_speak_for_me_active_input::` selects default/arity/type and retained-query/transition/environment/independence cases. Input-only159998966 and refinedc81f10375 each execute 0 PASS/2 assertion FAIL, exit101 after successful compilation/setup. Refined Rust type assertions pass: the existing generic namespace metatable manufactures a callable returning one nil, not a state-backed activity query. Later Lua boolean/default assertions fail; transitions/independence are unreached in RED. Source path `runtime_surface_bootstrap.lua` establishes the nil projection; no exact-value claim derives from generic assertion messages alone. Explicit C_API producer now reads only the independent input and pushes one boolean. Producera8 input2 and separate transcription-preservation3 pass, actual wait exits0/0; independent saved-evidence gate3735 PASS. Fresh normal startup asserts one false boolean and prints `SPEAK_ACTIVE_DEFAULT`/function/1/boolean/false then[]; entire14,714-byte stderr is CLEAN,219 content lines/220 LF-split entries including the final empty entry, no runtime warnings or AnimSetup failures. Default9b (docs-only overa8) library/bin check exits0 with733 JSON records/no source compiler messages; six inherited manifest deprecations remain. Source/readability/direct and Cargo formatting gate3729 PASS at a8. These are separate proof scopes, not five new cases or consumer/native/all-profile acceptance. Later consumer-test-only additions do not invalidate the default library/bin check. Initial GREEN artifact retention PASS; path hygiene FAIL and current follow-up PENDING.

### Cached consumer checkpoint — 2026-10-13

Independent saved-evidence verification PASS at compiled `978e9bcd15b47ad53dcdfd7aadb0f9ae0afb95b8`, production `a8a148e98`: exactly one real cached ChatFrame3 EditBox decision case PASS, 0 FAIL/ignored, actual wait exit0. Existing full constructor/startup and unchanged cached function reached false/unmuted (disabled, empty prompt, hidden after confirmed Show), true/unmuted (enabled, `REMOTE_TEXT_TO_SPEECH`), true/muted (activity still true, disabled, `VOICE_TRANSCRIPTION_MUTED`, RGB matches `RED_FONT_COLOR` within 0.000001), then false/unmuted restoration (disabled, empty prompt, hidden after confirmed Show). Entire runtime-error collection asserted empty after construction, prerequisites and each stage, without reset, clear or filter. Complete stderr contains no loader failure or Lua/runtime warning/error. No independent consumer RED, true-Show, event-producer, native or rendering claim. Query2/transcription3/startup/default-check PASS above remain separate scopes. [Public GREEN packet](../../data/patch-api/evidence/voice-speak-for-me-active-2026-10-13/README.md#current-green-public-packet--2026-10-13) now links copied independent source/runtime PASS reports and the 81-TEXT/594,604-byte manifest. Initial independent artifact-retention PASS does not close full-six-doc path hygiene FAIL; current follow-up gate PENDING. Six voice cases (query2/transcription3/consumer1), not six new features. Separate PERF2 is not normal-query CLEAN startup or voice acceptance.

## Documented-profile publication

Cached `VoiceChatDocumentation.lua` directly declares the zero-argument, one non-nil boolean query for Retail, PTR, Mists and WoW Forever. Manifest membership does not establish contents for unavailable Wrath/Era/Anniversary files; their query contract remains unknown. This slice publishes only the existing independent activity input, not transcription or channel selectors, and preserves their current gates.

- [ ] Expose the modeled query for the four directly documented profiles through `src/c_api/`.
- [ ] Execute Mists result-boundary RED before changing publication; `tests/voice_speak_for_me_profile_input.rs` asserts supplied false, supplied true through a retained query, environment isolation and false restoration. Tests also apply to WoW Forever; staged, not executed.
- [ ] Verify Mists/WoW Forever GREEN and Retail/PTR preservation, with independent source/runtime proof. Earlier Retail evidence remains revision-scoped.

## Known gaps (current cycle)

- [x] Execute genuine result-boundary RED; add explicit state-backed getter instead of the generic one-nil namespace result.
- [x] Bounded Retail query2/transcription-preservation3, fresh startup, default library/bin check, source/format/readability and independent proof.
- [ ] Complete the new independent retention/privacy gate for [retained GREEN text](../../data/patch-api/evidence/voice-speak-for-me-active-2026-10-13/green-current/green-retention-manifest.json) (81 payloads / 594,604 public bytes); initial artifact-retention PASS, path hygiene FAIL, current follow-up PENDING.
- [x] Execute and independently verify the real cached edit-box decision case; no independent consumer RED or event-producer/native/render acceptance is claimed.

## Out of scope

Native activity meaning/defaults/producers/transitions, transport, events, setters, permission modeling and broader-profile/native parity are unknown. No inferred channel aggregation or vendor behavior changes.
