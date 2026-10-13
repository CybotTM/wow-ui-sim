# Supplied Speak For Me activity

`VoiceChatState.is_speak_for_me_active` supplies independent Speak For Me activity. False initialization is simulator policy, not a native default. State-backed query registered after genuine result-boundary RED; bounded Retail query/preservation/startup/default-check proof passes. Cached consumer acceptance remains pending. Existing [voice transcription contract](voice-transcribing-input.md) remains distinct.

## What it must do

- [x] Zero-argument `C_VoiceChat.IsSpeakForMeActive()` returns exactly one non-nil boolean equal to the current supplied input; default false is explicit simulator policy.
- [x] Retained query functions read live false/true/false transitions; retained values remain snapshots and environments are isolated.
- [x] Activity is independent of transcription, permission and channel state; clearing channels or disconnecting does not infer activity.

### Cached Retail edit-box consumer

- [ ] The unchanged cached consumer and real ChatFrame3 edit box load through the existing full constructor; the entire runtime-error collection stays empty throughout the test.
- [ ] Unmuted false activity disables activation, clears the prompt and hides a previously shown edit box. Unmuted true enables activation and sets `REMOTE_TEXT_TO_SPEECH`; no automatic Show behavior is required.
- [ ] Muting preserves queried activity true while disabling activation and displaying `VOICE_TRANSCRIPTION_MUTED` in `RED_FONT_COLOR`; unmuted false restoration clears the prompt and hides again.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/sim_substates/mod.rs`: independent supplied input/default.
- `src/c_api/c_voice_chat_channels.rs`: explicit state-backed C_API query/registration.
- `tests/voice_speak_for_me_active_input.rs`: two behavioral cases under `retail-12-0-5`.
- `tests/voice_speak_for_me_consumer.rs`: one GUI/capability-gated real cached edit-box consumer case.

## Tests asserting this spec

Generated integration filter `voice_speak_for_me_active_input::` selects default/arity/type and retained-query/transition/environment/independence cases. Input-only159998966 and refinedc81f10375 each execute 0 PASS/2 assertion FAIL, exit101 after successful compilation/setup. Refined Rust type assertions pass: the existing generic namespace metatable manufactures a callable returning one nil, not a state-backed activity query. Later Lua boolean/default assertions fail; transitions/independence are unreached in RED. Source path `runtime_surface_bootstrap.lua` establishes the nil projection; no exact-value claim derives from generic assertion messages alone. Explicit C_API producer now reads only the independent input and pushes one boolean. Producera8 input2 and separate transcription-preservation3 pass, actual wait exits0/0; independent saved-evidence gate3735 PASS. Fresh normal startup asserts one false boolean and prints `SPEAK_ACTIVE_DEFAULT`/function/1/boolean/false then[]; entire14,714-byte stderr is CLEAN,219 content lines/220 LF-split entries including the final empty entry, no runtime warnings or AnimSetup failures. Default9b (docs-only overa8) library/bin check exits0 with733 JSON records/no source compiler messages; six inherited manifest deprecations remain. Source/readability/direct and Cargo formatting gate3729 PASS at a8. These are separate proof scopes, not five new cases or consumer/native/all-profile acceptance. Later consumer-test-only additions do not invalidate the default library/bin check. Public GREEN retention remains pending.

## Known gaps (current cycle)

- [x] Execute genuine result-boundary RED; add explicit state-backed getter instead of the generic one-nil namespace result.
- [x] Bounded Retail query2/transcription-preservation3, fresh startup, default library/bin check, source/format/readability and independent proof.
- [ ] Retain current GREEN text and complete its independent retention/privacy gate.
- [ ] Execute the staged real cached edit-box consumer case; no independent consumer RED or event-producer/native/render acceptance is claimed. Source query references do not establish runtime acceptance.

## Out of scope

Native activity meaning/defaults/producers/transitions, transport, events, setters, permission modeling and broader-profile/native parity are unknown. No inferred channel aggregation or vendor behavior changes.
