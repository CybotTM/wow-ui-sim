# Global voice transcription activity

`VoiceChatState.is_transcribing: bool` supplies global transcription activity independently of permission and channel snapshots. False default is simulator policy, not native default or aggregation. Query producer remains absent; three prospective tests are uncompiled/unexecuted. Architecture: [Lua API](../lua-api.md).

## What it must do

- [ ] Default explicit global input to false without changing other voice defaults.
- [ ] Zero-argument `C_VoiceChat.IsTranscribing()` returns exactly one non-nil boolean equal to current input.
- [ ] Retained query function reads live false/true/false transitions; earlier values remain snapshots. Environments are isolated.
- [ ] Permission neither gates nor changes activity. Existing constant-false `IsTranscriptionAllowed()` remains unchanged; no permission-true input invented.
- [ ] Channel host/returned snapshots do not determine or mutate global input. Global true may coexist with empty channels, no active channel and disconnected status; global false may coexist with a transcribing channel snapshot.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

| Path | Role |
|---|---|
| `src/lua_api/sim_substates/mod.rs` | Explicit global input/default; no getter. |
| `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs` | Existing environment-owned voice state and separate channel collection. |
| `src/c_api/c_voice_chat_channels.rs` | Existing channel selectors and fresh DTO snapshots, unchanged. |
| `src/lua_api/globals/missing_surface/voice_chat.rs` | Existing voice/permission queries, unchanged. |
| `tests/voice_transcribing_input.rs` | Three prospective tests under shared `retail-12-0-5` capability. |

## Tests asserting this spec

Generated `--test integration` filter `voice_transcribing_input::` (expected three): default/publication/arity/type; live retained query and environment isolation; permission/channel snapshot independence, conflicting inputs and disconnected/empty state. Existing `GetChannelForChannelType` and `GetChannelForCommunityStream` providers supply full snapshots including `isTranscribing`. Missing new query is an intended behavioral boundary, not a setup workaround. All tests UNCOMPILED/UNRUN.

## Known gaps (current cycle)

- [ ] Compile and execute tests against unchanged query surface; fixture/compile failures are not behavioral RED.
- [ ] Add minimal C_API getter after genuine RED, then bounded GREEN and applicable startup/check proof.
- [ ] Actual cached caller test withheld until `IsSpeakForMeActive()` prerequisite is source-established.

## Out of scope

Transport, capture, recognition, permission modeling, events, public/admin setters, aggregation and active-channel inference. No native activity definition/default, extra-argument rejection/coercion or broader-profile/native acceptance claimed.
