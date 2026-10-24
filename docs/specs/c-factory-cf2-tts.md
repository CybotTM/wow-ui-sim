# CF-2 cross-profile TTS persistence and temporary-CVar lifecycle

Slice 7 removes the original ChatFrame character-save and Forever Gamepad temporary-CVar gaps. Modeled `C_*` settings/lifecycle producers belong in `src/c_api/`; the effective CVar store remains `SimState.cvars`. This EDIT-ONLY delivery contains tests and contract, not producers or runtime proof. See [Lua API architecture](../wiki/systems/lua-api.md) and [input-style contract](forever-input-interface-style.md).

## What it must do

### Character TTS settings

- [ ] On Retail, PTR, WowForever, Mists, Era and Anniversary, unchanged cached `Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua` slider callbacks drive two distinct explicit rate values (4 and 7) and volume values (37 and 63), with `TTSUseCharacterSettings=1`. Order each preference so the first value differs from its observed initial value; no initial/default value is prescribed.
- [ ] After each pair, original `ADDON_LOADED(Blizzard_ClientSavedVariables)`, `VARIABLES_LOADED`, `PLAYER_ENTERING_WORLD` completes CheckLoad and renders that pair without a new Lua error. Both getters and controls round-trip the pair. Observe changed numeric preference values in live `SimState.cvars`, discovering keys from before/after snapshots rather than inventing native CVar names.
- [ ] Destroy and reopen the environment after each pair. Before original CheckLoad can mark again, read the saved marker and that pair, and verify the changed live CVar entries persist byte-for-byte. Load unchanged controls and confirm they render that pair. The next pair replaces the previous persisted preferences.
- [ ] With explicit absence of legacy `TEXTTOSPEECH_CONFIG`, observe the saved marker before and after original CheckLoad: an observed false must become true; an observed true remains true. Already-true preservation cannot reject a no-op Mark. Marker-producer acceptance remains PENDING-NATIVE-ORACLE until a normal unsaved input/default/reset policy is established; the preference selector's RED count does not authorize that producer.

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

Seven applicable expected-RED executions: six preference selectors plus one Gamepad selector. Marker assertions are supplemental preservation/pending, not standalone producer acceptance. Original missing boundaries: TTS MarkCharacterSettingsSaved at Retail/Forever475, PTR/Mists/Era/Anniversary472; Gamepad RemoveTempCVar at Core.lua6. Missing pre-observation GetCharacterSettingsSaved is a prerequisite blocker, not accepted original-consumer RED. Later missing SetTempCVar and saved preference methods are in-scope lifecycle gaps, not permission to bypass original code. Setup, compile, source/provenance, timeout or unrelated API failures are not accepted RED. No RED has been observed; no producer may be committed before it is.

## Known gaps (current cycle)

- [ ] Admitted compile and observed original-consumer RED, then separate backing-model producer commit and identical GREEN. No Cargo build/test/check is authorized in this edit-only phase.
- [ ] PENDING-NATIVE-ORACLE: exact account/character identity and storage selection when `TTSUseCharacterSettings` changes; initial saved flag/defaults, persistence timing before/after mark, default reset and legacy migration policy. Original TTS lines296–361 (Retail/Forever; other profiles offset−3) temporarily select character settings, migrate then restore the CVar; CheckLoad457–479/454–476 queries saved state only for nondefault legacy input and marks unconditionally. Those lines do not settle ownership/default/save-timing details. No such defaults or account-switch assertions are invented.
- [ ] PENDING-NATIVE-ORACLE: unknown/invalid/read-only/secure CVar behavior, normal writes while temporary override is active, CVAR_UPDATE ordering/counts, temporary overlay priority across multiple writers. Original Forever Gamepad Core.lua1–43 sets/removes known names by mode; it does not settle those policies. Current tests cover explicit known ordinary values, original mode overrides and removal only.

## Out of scope

Audio synthesis/host gamepad integration, general settings UI compatibility, other CF-2 rows, native default discovery, broad profile/startup health, invented compatibility shims, and producer implementation before observed RED. Voice/channel/boolean catalogs, multi-character/account switching and legacy migration require evidence beyond this bounded explicit speech-setting fixture.

## Edit-only preference producer staging — 2026-10-24

The requested producer stage does **not** authorize the older persistence oracle above.
All original CVar/reopen assertions stay intact pending ASK-USER; no public preference
CVar names or cross-environment saved-marker lifetime have been established.

### Static RED contradiction matrix (not executed)

Each of the following selectors independently loads runtime-selected ChatFrame Lua:
Retail/PTR ChatFrame TOC shared Lua/XML; Mists ChatFrame_Mists TOC permits mists and
includes the same shared consumer. Original source provenance is checked, including
slider OnValueChanged callbacks. No producer substitution or vendor edit is used.

| Exact integration selector | Contradiction to constant/no-op rate/volume producer | Marker limitation |
|---|---|---|
| `cf2_tts::original_tts_slider_settings_survive_mark_saved_and_second_environment` | Explicit zero then 4/37 and 7/63 require unequal getter results; original CheckLoad/UpdateSliders read back controls. Constant volume100 fails at zero; any constant/no-op cannot meet both pairs. | CVar-change and reopen oracles are ASK-USER, not producer acceptance. |
| `cf2_tts_shape_contract::original_tts_numeric_preferences_contract` | Original SetRate/SetVolume callbacks require rate4 then7 and volume37 then63 before UpdateSliders assertions. | No marker claim. |
| `cf2_tts_shape_contract::original_tts_saved_settings_shape_blocker` | Exact bool/void shapes first, then independent original-callback two-pair checks. | Bool shape alone accepts constant bool/no-op Mark. |
| `cf2_tts_shape_contract::original_tts_saved_settings_consumer_blocker` | Original CheckLoad first with supplied legacy settings, loaded flag and saved=true; then independent two-pair checks. | Constant-false fails after Mark; constant-true + no-op Mark is observationally equivalent without an unsaved/reset oracle. |

The unchanged base shim returns100/0 and ignores setters. Each new two-pair
prerequisite therefore contradicts it at the original callback read-back boundary.
This is static proof, **not observed runtime RED**. Rejecting constant-true saved
markers in every selector is impossible under the currently documented contract;
no initial=false assertion, reset oracle, or native-parity claim is introduced.

`cf2_tts_shape_contract::original_tts_documented_preference_pairs_contract` adds
supplemental explicit-input voice ID/name, boolean setting, chat type and supplied
channel-record round trips. Original voice/chat/channel setters are provenance-checked;
its numerical prerequisite independently rejects constant/no-op rate/volume producers.
It does not claim native voice-name defaults, cross-name/ID mapping, channel-key encoding,
marker defaults or persistence. Saved-shape and saved-consumer selectors retain their
respective shape/CheckLoad failures **before** numerical checks to avoid masking them.
