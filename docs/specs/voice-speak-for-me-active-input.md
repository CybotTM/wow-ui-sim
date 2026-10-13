# Supplied Speak For Me activity

`VoiceChatState.is_speak_for_me_active` supplies independent Speak For Me activity. False initialization is simulator policy, not a native default. Input/test-only stage; query producer and behavioral execution pending. Existing [voice transcription contract](voice-transcribing-input.md) remains distinct.

## What it must do

- [ ] Zero-argument `C_VoiceChat.IsSpeakForMeActive()` returns exactly one non-nil boolean equal to the current supplied input; default false is explicit simulator policy.
- [ ] Retained query functions read live false/true/false transitions; retained values remain snapshots and environments are isolated.
- [ ] Activity is independent of transcription, permission and channel state; clearing channels or disconnecting does not infer activity.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/sim_substates/mod.rs`: independent supplied input/default.
- `src/c_api/c_voice_chat_channels.rs`: intended C_API query/registration; not implemented in input-only stage.
- `tests/voice_speak_for_me_active_input.rs`: two behavioral cases under `retail-12-0-5`.

## Tests asserting this spec

Generated integration filter `voice_speak_for_me_active_input::` selects default/arity/type and retained-query/transition/environment/independence cases. Both begin with an explicit missing-query type assertion; later assertions are not proven until reached. Input-only159998966 executed 0 PASS/2 assertion FAIL, exit101 after successful compilation/setup; Lua errors omitted assertion text. The refined Rust-side type assertions expose the actual missing value before producer implementation; that execution remains pending. No GREEN yet.

## Known gaps (current cycle)

- [ ] Execute genuine query-boundary RED, then implement minimal state-backed getter.
- [ ] Targeted GREEN/preservation, applicable startup diagnostics/check/format/readability and independent proof.
- [ ] Actual cached consumer behavior remains unverified; source query references do not establish runtime acceptance.

## Out of scope

Native activity meaning/defaults/producers/transitions, transport, events, setters, permission modeling and broader-profile/native parity are unknown. No inferred channel aggregation or vendor behavior changes.
