# CF-2 cross-profile TTS persistence and temporary-CVar lifecycle

Slice 7 covers original ChatFrame character-save and Forever Gamepad temporary-CVar gaps. Modeled `C_*` producers belong in `src/c_api/`; ordinary CVars remain `SimState.cvars`. Delivery is edit-only: authored tests and staged producers, no runtime RED/GREEN. See [Lua API architecture](../wiki/systems/lua-api.md).

## What it must do

### Character TTS settings

- [ ] Retail, PTR, WowForever, Mists, Era and Anniversary load unchanged cached ChatFrame consumers. Original slider callbacks drive rate0/volume0, then rate4/volume37 and rate7/volume63 with `TTSUseCharacterSettings=1`. Each numerical selector independently rejects constant/no-op producers.
- [ ] After each pair, original ADDON_LOADED(Blizzard_ClientSavedVariables), VARIABLES_LOADED and PLAYER_ENTERING_WORLD complete CheckLoad without new Lua errors. Getters and original UpdateSliders controls agree on the pair.
- [ ] Destroy and recreate the environment after each pair. Before original CheckLoad can mark again, C_TTSSettings getters observe saved=true and the persisted rate/volume, voice ID/name, boolean setting, chat type and supplied channel record. Both pairs replace earlier values, including false/true preferences. Original controls then render the persisted pair.
- [ ] A fresh isolated store starts with saved=false; MarkCharacterSettingsSaved returns no values and makes it true. Original CheckLoad exercises this transition. The marker persists through environment recreation. This is a **user-approved non-native model**, not a native default/lifetime finding.
- [ ] Persist preferences and marker using existing simulator storage infrastructure, scoped to the active client profile. Do not expose preference storage as public CVars. TTSUseCharacterSettings remains the ordinary account/character selector CVar.

### Forever temporary CVars

- [ ] Seed ordinary SoftTargetEnemy=2 and unrelated cameraDistanceMaxZoomFactor=1.9 through the live CVar store. Original Blizzard_Gamepad/Core.lua Mkb callback removes temporary values even without an overlay, preserving both ordinary values.
- [ ] Original InputUtil transitions Gamepad then Mkb. Gamepad exposes SoftTargetEnemy=1 temporarily; removal restores2. Lua and Rust CVar reads agree, unrelated value remains1.9, repeated removal preserves2.
- [ ] A second environment during the override reads persisted2, not temporary1. Reopening after removal reads2/1.9. Temporary values never overwrite ordinary persistence.

### Fixture safety and applicability

- [ ] Load original functions/templates through the profile-cache addon closure without overrides; assert source provenance and retain load errors.
- [ ] Persistence cases reexecute an exact selector in a case-owned child process outside deps. Explicit empty CVar fixture and child-only XDG_DATA_HOME prevent ambient/user preference reads. Environments in one case share the same fixture; unrelated cases do not. External verification launchers separately preserve current host CVar bytes and stat/hash receipts before/after, including launcher failures; never reset/restore/write the original.
- [ ] File-level cfg registers TTS on the six non-Wrath profiles and Gamepad only on Forever. No runtime N/A shortcuts, ignored tests, suppressions, vendor edits or authored replacement consumers.

## User decisions — 2026-10-24

The earlier live-CVar-key oracle is withdrawn: original sources do not establish rate/volume preference keys. Remove its nonempty changed-key assertions and dependent reopened-key comparisons; preserve all getter, control, provenance, error-count and selector-CVar assertions. Getter-based cross-environment persistence remains required and extends to other preferences.

Fresh saved=false, Mark=>true and persistence are approved simulator behavior, not native evidence. Existing SetDefaultSettings behavior resets only the selected preference table and preserves the marker; native reset semantics remain unknown. These decisions supersede earlier ASK-USER text and the earlier prohibition on edit-only producer staging. No builds, slots or runtime claims are authorized by this delivery.

## How it works

- [Lua API architecture](../wiki/systems/lua-api.md#tts-preference-model-edit-only-staging)
- [Forever input-style ownership](forever-input-interface-style.md)
- [C namespace boundary](c-namespace-nil-members.md)

### Staged producer contract and evidence limits

C_TTSSettings uses private Lua preference tables for account and character selection. Existing SavedVariables serialization/storage persists those tables and the marker in a profile-scoped simulator model file, isolated with the owning CVar store. No public preference CVar, alternate producer or fallback shim is added. Mutating setters and Mark save immediately; environment initialization restores before any original consumer executes. Storage errors are surfaced, not silently ignored.

Original Shared/TextToSpeechFrame.lua DefaultLegacySettings (lines5–39 in all six cached TTS profiles) supplies legacy UI defaults: rate0, volume100, Standard/Alternate IDs0/1, four true boolean options, NarrateMyMessages=false, the 18 enabled chat types and empty channels. Initialization/reset uses that source set; these are not proven modern native C++ defaults. Original TextToSpeechCommands.lua lines1–5 supplies UI ranges [-10,10] and [0,100]. Unsupported direct setter values reject explicitly; native invalid-input policy is unknown.

TTS string arguments accept strings and numbers; numbers use Lua-5.1 VM number-to-string formatting (0 becomes "0"). Other types retain the existing string error. This is Lua-5.1 C-API semantics, pending native confirmation for SetVoiceOptionName. The original saved-settings consumer rejects a number-rejecting implementation, but does not distinguish a silent voice-name no-op from storing "0": it never reads that migrated name.

Documented omitted bool newVal defaults false. Supplied voice IDs/names persist; the opposite representation resolves only through existing C_VoiceChat.GetTtsVoices rows. No default voice name is invented; absent catalogue matches produce explicit UNKNOWN errors. Supplied ChatChannelInfo records round-trip by all six documented fields. Legacy channel keys persist verbatim; key-to-record encoding and identity across changed metadata remain unknown.

All seven available profile caches were searched for preference CVar mappings. No rate/volume/voice/chat/channel or marker mapping was found; Wrath's sparse cache has no TTS consumer. Settings definitions for textToSpeech, remoteTextToSpeech, remoteTextToSpeechVoice and accessibility screen narration do not establish C_TTSSettings preference keys. Search absence is not proof of native internals. No decompiled source was read.

## Tests asserting this spec

Integration target selectors (authored, unexecuted):

| Selector | Contradiction / proof boundary |
|---|---|
| cf2_tts::original_tts_slider_settings_survive_mark_saved_and_second_environment | Original sliders/CheckLoad/UpdateSliders plus two getter-based reopen pairs, other preferences and approved fresh=false marker. |
| cf2_tts_shape_contract::original_tts_numeric_preferences_contract | Original numerical callbacks independently require4/37 then7/63 and matching controls. |
| cf2_tts_shape_contract::original_tts_saved_settings_shape_blocker | Bool/void shape preflight first, then independent numerical contradiction. Bool shape alone accepts constant bool. |
| cf2_tts_shape_contract::original_tts_saved_settings_consumer_blocker | Original CheckLoad first with nondefault legacy input, then independent numerical contradiction. Constant-true/no-op Mark remains indistinguishable within this selector alone. |
| cf2_tts_shape_contract::original_tts_documented_preference_pairs_contract | Independent numerical contradiction plus explicit voice, bool, chat and channel round trips. No default/reset/native persistence claim. |

Forever additionally registers cf2_gamepad_temp_cvars::original_gamepad_temp_cvars_restore_persisted_values_without_touching_ordinary_cvar; its producer is outside this TTS change.

Original missing boundaries at the pre-producer base: MarkCharacterSettingsSaved at Retail/Forever475 and PTR/Mists/Era/Anniversary472; Gamepad RemoveTempCVar at Core.lua6. Missing pre-observation getter is a prerequisite blocker, not accepted original-consumer RED. Setup, compilation, provenance, timeout and unrelated API failures are not RED.

## Known gaps

- [ ] Run admitted original-consumer RED and identical final GREEN; no build/test/check has run in this edit-only delivery. Keep test changes separate from producer commits for later source-bound replay.
- [ ] Native account/character ownership, fresh/reset marker policy, save timing, legacy migration and multi-character/account switching remain unproven. Approved persistence semantics do not establish native parity.
- [ ] Channel-key-to-record migration and changed-metadata identity remain incomplete; no speculative aliases.
- [ ] Gamepad invalid/read-only/secure CVar behavior, ordinary writes during overlays, event ordering and multiple-writer priority remain outside the bounded fixture.

## Out of scope

Audio synthesis, host gamepad integration, general settings UI compatibility, other CF-2 rows, native default discovery, broad profile/startup health, invented compatibility shims, and full account/character migration.
