# T6 explicit startup service inputs

T6 restores named original-consumer service inputs after [factory removal](c-namespace-nil-members.md), without attributing the unidentified Forever timer family. [Implementation notes](../wiki/systems/c-factory-startup-followup.md#t6-service-inputs) distinguish real existing state from unavailable service capabilities.

## What it must do

- [ ] Retail/PTR `C_Log.LogErrorMessage(message)` uses the existing simulator console sink.
- [ ] Era/Anniversary `C_VoiceChat.IsSpeakForMeActive()` exposes the existing independent supplied activity flag.
- [ ] All six non-Wrath profiles' chat-type TTS preference queries return false until per-chat-type preference storage exists.
- [ ] Retail/PTR/Forever voice-login and collectable-source tracking capability queries are named temporary false defaults; simulated channels/connection status are not authentication evidence.
- [ ] Retail/PTR secure combat-filter reset is a named temporary void operation while no secure filter backend exists, independent of ordinary combat-log filters.
- [ ] Era/Anniversary corpse-map position remains absent until map-keyed corpse coordinates exist; player coordinates must not substitute.
- [ ] Forever preferred-locale ID/name lists are empty, Discord voice capability is false, and active provider uses the documented Legacy enum as a temporary presentation default. Each names its missing backing system.
- [ ] Unknown members remain nil and Wrath's runtime surface remains unchanged.

## How it works

- [Service capability gaps and existing activity/sink adapters](../wiki/systems/c-factory-startup-followup.md#t6-service-inputs)

## Implementation inventory

- `src/c_api/registration.rs` — existing log sink publication for actual Retail/PTR consumers.
- `src/c_api/c_voice_chat_activity.rs` — existing supplied activity also published for documented Era/Anniversary consumers.
- `src/lua_api/workarounds/temporary/startup_service_capability_defaults.rs` — individually named and profile-scoped missing service systems.
- `src/lua_api/workarounds/{mod.rs,temporary/mod.rs}` — non-Wrath bootstrap registration.

## Tests asserting this spec

- `tests/cfactory_t6_services.rs` — generated documentation contracts, concrete console/activity state and complete original preferred-locale dropdown consumer.

## Known gaps (current cycle)

- [ ] New test-only revision RED and targeted GREEN; combined stock startup matrix.
- [ ] Forever anonymous timer family remains unresolved: original callback `Blizzard_SharedXMLBase/FunctionUtil.lua:87`, unknown C_* root. Preserve its null attribution and normal/wrapped error evidence.

## Out of scope

Wrath changes, invented timer attribution, native authentication/provider selection, service entitlement policy, corpse coordinates, preferred-locale catalogs and per-chat-type preference storage. No generic function fabrication or vendor changes.
