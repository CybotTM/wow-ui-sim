# CF-2 cross-profile TTS persistence and temporary-CVar lifecycle

Slice 7 removes the original ChatFrame character-save and Forever Gamepad temporary-CVar gaps. Modeled `C_*` settings/lifecycle producers belong in `src/c_api/`; the effective CVar store remains `SimState.cvars`. This EDIT-ONLY delivery contains tests and contract, not producers or runtime proof. See [Lua API architecture](../wiki/systems/lua-api.md) and [input-style contract](forever-input-interface-style.md).

## What it must do

### Character TTS settings

- [ ] On Retail, PTR, WowForever, Mists, Era and Anniversary, unchanged cached `Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua` slider consumers accept explicit rate4/volume37 with `TTSUseCharacterSettings=1`. After original `ADDON_LOADED(Blizzard_ClientSavedVariables)`, `VARIABLES_LOADED`, `PLAYER_ENTERING_WORLD`, original CheckLoad completes its mark-saved call and updates the real slider controls to4/37 without a new Lua error.
- [ ] With explicit absence of legacy `TEXTTOSPEECH_CONFIG`, successful original mark-saved yields `GetCharacterSettingsSaved()==true`. A second environment opened against the same isolated persistent storage reads true and4/37 **before** it can mark anything again; its unchanged original controls render4/37 after loading.
- [ ] Values are actual saved preferences, not unchanged temporary constant getters, no-op setters or a test-only Lua table. The first environment is destroyed before the second opens. These requirements are the user-requested mark/save lifecycle; they do not establish whether setters persist before marking.

### Forever temporary CVars

- [ ] Seed ordinary `SoftTargetEnemy=2` and unrelated `cameraDistanceMaxZoomFactor=1.9` through the existing live CVar store. Original `Blizzard_Gamepad/Core.lua` Mkb callback removes temporary values even at initial/no-overlay entry without changing either persisted value.
- [ ] Drive the unchanged original InputUtil transition listener into Gamepad then Mkb. Original Gamepad initialization temporarily exposes `SoftTargetEnemy=1`; original removal restores2. Lua global/namespace reads and Rust CVarStorage agree, unrelated ordinary value remains1.9, and repeated Mkb removal leaves2.
- [ ] A second environment opened while the temporary override is active reads persisted2, not temporary1. Reopening after removal also reads2/1.9. Temporary state must not overwrite persisted ordinary settings.

### Fixture safety and applicability

- [ ] Load original functions/templates through the active profile-cache addon closure with no addon overrides. Preserve load errors; direct consumer invocations expose errors and compare recorded-error counts. Function/listener source provenance must identify the original files.
- [ ] Run persistence fixtures only in an isolated child process with a sealed executable outside `deps`. A fresh child-only `XDG_DATA_HOME` contains byte-exact current persisted CVars; original stat/hash receipts before/after include launcher failures. Never reset, restore or write the host original. Cargo's automatic per-environment CVar temp files cannot establish persistence across environments.
- [ ] File-level profile cfg registers TTS only on Retail, PTR, Forever, Mists, Era and Anniversary, and Gamepad only on Forever. Non-applicable profiles register no such tests, without runtime N/A returns. No hiding gates beyond registration-by-applicability, vendor edits, API substitutions, authored consumers, ignores, suppressions or weakened assertions.

## How it works

- [Lua API architecture](../wiki/systems/lua-api.md)
- [Existing Forever input-style ownership](forever-input-interface-style.md)
- [Original C namespace boundary](c-namespace-nil-members.md)

## Implementation inventory

- `src/cvars.rs`: live effective defaults, registered defaults and persisted ordinary override store; no temporary override layer/removal exists at this base. `src/cvar_storage.rs` does not exist.
- `src/lua_api/state/sim_state.rs`: owns `cvars`; constructor in `src/lua_api/state.rs` opens storage.
- `src/lua_api/globals/set_cvar_verb.rs`: existing modeled ordinary CVar reads/writes; namespace has no SetTempCVar/RemoveTempCVar registration.
- `src/lua_api/workarounds/temporary/tts_settings_defaults.rs`: explicitly temporary constant speech/voice getters and no-op setters; no saved preferences backend.
- `src/lua_api/globals/channel_verbs.rs`: TTS channel getter constant true/setter no-op; not saved character preference storage.
- `src/c_api/c_input_interface_style.rs`: existing Forever-only current-style state. This fixture drives original listener event payloads, not host device changes; that separate state-before-event contract is not re-proven here.
- `tests/cf2_tts.rs`: authored original TTS persistence assertions on six non-Wrath profiles; uncompiled/unrun.
- `tests/cf2_gamepad_temp_cvars.rs`: authored original Gamepad temporary-CVar lifecycle on Forever only; uncompiled/unrun.

## Tests asserting this spec

Integration target exact selectors:

- `cf2_tts::original_tts_slider_settings_survive_mark_saved_and_second_environment` — six non-Wrath profiles above.
- `cf2_gamepad_temp_cvars::original_gamepad_temp_cvars_restore_persisted_values_without_touching_ordinary_cvar` — WowForever only.

Seven applicable executions. Expected first missing boundaries: original TTS MarkCharacterSettingsSaved at Retail/Forever475, PTR/Mists/Era/Anniversary472; original Gamepad RemoveTempCVar at Core.lua6. Later missing SetTempCVar and saved preference methods are in-scope lifecycle gaps, not permission to bypass original code. Setup, compile, source/provenance, timeout or unrelated API failures are not accepted RED. No RED has been observed; no producer may be committed before it is.

## Known gaps (current cycle)

- [ ] Admitted compile and observed original-consumer RED, then separate backing-model producer commit and identical GREEN. No Cargo build/test/check is authorized in this edit-only phase.
- [ ] PENDING-NATIVE-ORACLE: exact account/character identity and storage selection when `TTSUseCharacterSettings` changes; initial saved flag/defaults, persistence timing before/after mark, default reset and legacy migration policy. Original TTS lines296–361 (Retail/Forever; other profiles offset−3) temporarily select character settings, migrate then restore the CVar; CheckLoad457–479/454–476 queries saved state only for nondefault legacy input and marks unconditionally. Those lines do not settle ownership/default/save-timing details. No such defaults or account-switch assertions are invented.
- [ ] PENDING-NATIVE-ORACLE: unknown/invalid/read-only/secure CVar behavior, normal writes while temporary override is active, CVAR_UPDATE ordering/counts, temporary overlay priority across multiple writers. Original Forever Gamepad Core.lua1–43 sets/removes known names by mode; it does not settle those policies. Current tests cover explicit known ordinary values, original mode overrides and removal only.

## Out of scope

Audio synthesis/host gamepad integration, general settings UI compatibility, other CF-2 rows, native default discovery, broad profile/startup health, invented compatibility shims, and producer implementation before observed RED. Voice/channel/boolean catalogs, multi-character/account switching and legacy migration require evidence beyond this bounded explicit speech-setting fixture.
