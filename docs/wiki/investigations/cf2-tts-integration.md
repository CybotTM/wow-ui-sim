# CF-2 TTS integration

Verified: 2026-10-25. TTS integration defers whole non-TTS test files that fail on any registered profile. The owning changes must restore complete files with their producers. Requirements in the existing specs remain in force; removing a fixture is not producer acceptance.

## Restoration inventory

The source checkpoint is `5327a3797`. Historical seven-profile module receipts are from `93c0220b6`; Forever was rebuilt at the checkpoint and each registered case was discovered and run separately against a private wowf 1.60.1.70532 cache. Earlier 70374 runtime receipts are stale input evidence, not acceptance. Passing cases below are the union across registered profiles, not an all-profile claim. All paths in the table are under `tests/`; names omit their file's module prefix.

| Deferred file | Origin commit / owner | Cases that passed within the removed file |
|---|---|---|
| `api_shape_equipment_set.rs` | `36dbfecf4` equipment-set lane | `equipment_set_original_equip_set_contract` |
| `api_shape_item_upgrade.rs` | `d95d4e478` item-upgrade lane | `item_upgrade_boolean_contract` |
| `api_shape_stable_active_list.rs` | `41df2111d` stable lane | None |
| `api_shape_store.rs` | `53e1eeb2a` store lane | None |
| `cf2_combat_aura_controls.rs` | `a46e561bc` combat/aura lane | None |
| `cf2_combataura_shape_contract.rs` | `b66ba6178` combat/aura shape lane | `original_group_buff_records_shape_blocker` |
| `cf2_forever_shape_contract.rs` | `6138f4f63` Forever shape lane | `original_camelot_equipment_predicates_shape_blocker`, `original_ping_manager_shape_blocker` |
| `cf2_gamepad_temp_cvars.rs` | `6cdd9faee` Gamepad lane | None |
| `cf2_housing_shape_contract.rs` | `d37fd73bf` housing shape lane | `original_supertracking_pair_shape_blocker` |
| `cf2_housing_supertracking.rs` | `b10530182` housing/supertracking lane | `original_supertracking_refresh_publishes_its_cache_to_listeners_preserves_boundary` |
| `cf2_pvpmap_shape_contract.rs` | `f62a6b45d` PvP/map shape lane | `original_match_enter_world_shape_blocker` |
| `cf2_social_shape_contract.rs` | `2a8a08f29` social shape lane | `original_microbutton_invitation_records_shape_blocker` |
| `cfactory_cf2_social.rs` | `e9cdb1d22` social lane | None |

`cf2_forever.rs` and `cf2_group_buff_preservation.rs` now pass their registered Forever cases after upstream producer integration and remain unchanged. Other retained files and both shared helpers remain unchanged.

## Mixed-file exception

Only `cf2_tts_shape_contract::original_gamepad_temp_cvar_shape_blocker` is removed from the mixed TTS file. Its exact source is recoverable from `5327a3797:tests/cf2_tts_shape_contract.rs`; original commit `180db46409e28ce6448c206b05d95d97eaff0f7a`. The current Forever exact run fails at the temporary-CVar boundary. The Forever startup-gaps lane owns restoration after its producer is integrated. Every TTS case remains byte-identical.

## Evidence boundaries

The five original-consumer TTS cases pass on the checkpoint's Forever build with 70532 inputs. Its exact raw stock map matches the accepted master map: 26 messages, 26 occurrences, zero additions or removals. Wrath does not register these CF-2 fixtures. The saved-marker model is user-approved, not native initial/reset evidence.

## Publication gate repair

The verifier rejected unconditional registration: the shared model was published on Wrath without a pinned declaration. All 18 function names occur as functions in `Blizzard_APIDocumentationGenerated/TTSSettingsDocumentation.lua` on Retail, PTR, Mists, Era, Anniversary and WowForever. Neither the sparse Wrath cache nor its fuller pinned source tree provides that declaration. Search absence does not establish native absence.

Registration now checks `client_profile::ACTIVE` before namespace creation or persistence initialization and returns on Wrath. The six legacy `c_system_api` TTS tests use the same visible profile selection: each positively asserts namespace absence on Wrath; other profiles retain their original assertions. This applicability correction was explicitly approved. A separate publication regression checks Wrath absence or all 18 raw function members. The Wrath regression failed against the ungated producer after exact discovery; post-fix runtime and all-target proof are pending.

The merge from `origin/master` at `05f95ab1c` preserves both parents' wiki log entries. No master branch update or rebase was performed.

## Bounded verification

Historical code checkpoint `297b198b6` passes 37 exact cases: five original-consumer TTS cases on each of six registered profiles, plus the corrected legacy Alternate-voice test on all seven. Each case has exact-list discovery followed by one exact execution. All seven all-target checks pass. Retail/Wrath have zero warnings; PTR/Mists/Era/Anniversary/Forever have only the three inherited `fast_handler_late_bind` dead-code warnings. The parent explicitly accepted the Era/Anniversary allowance; the `i1-test-warnings` lane owns cleanup.

Stock results retain raw message keys and counts, without headline normalization:

| Profile | Master messages / occurrences | Branch messages / occurrences | Added / removed raw keys |
|---|---:|---:|---:|
| Retail | 16 / 18 | 14 / 16 | 0 / 2 |
| PTR | 428 / 608 | 426 / 606 | 0 / 2 |
| Mists | 14 / 20 | 12 / 18 | 2 / 4 |
| Wrath | 0 / 0 | 0 / 0 | 0 / 0 |
| Era | 4 / 10 | 2 / 8 | 0 / 2 |
| Anniversary | 4 / 10 | 2 / 8 | 0 / 2 |
| Forever | 26 / 26 | 26 / 26 | 0 / 0 |

Saved-marker failures disappear on Retail/PTR/Mists/Era/Anniversary. Mists additionally changes the first failing WorldMap provider from GroupMembers' missing `GetMapDisplayInfo` to Vehicle's missing `GetBattlefieldVehicles`, including the Blizzard re-report. Both API gaps are unchanged; `Blizzard_MapCanvas.lua:451–454` traverses providers with `pairs`. This is a source-backed first-failure ordering explanation, not a deterministic ordering guarantee or a claim of zero raw additions. Wrath's paired zero result covers its selected sparse cache: five Blizzard addons and 2,396 frames, not broader native startup health.

Retail/Wrath master maps were built at `cad1e2902`, whose runtime inputs match `5d867ef38`; the other five master maps were reused with source custody. Forever uses the verified private wowf 70532 cache. Library freshness was audited separately: fresh Retail reuse has 1,372 identical dep-info inputs, and fresh Forever reuse has 1,208; source/library-input diffs are empty across the respective source checkpoints. Other current library records were rebuilt, not blindly accepted as fresh. Native parity and full-suite acceptance remain outside this proof.

## Alternate voice oracle correction

Master's temporary `tts_settings_defaults.rs` returns zero for every voice type. The new model uses the pinned Blizzard legacy UI defaults instead: `Blizzard_ChatFrame/Shared/TextToSpeechFrame.lua:11` sets `ttsVoiceOptionAlternateDefault = 1`; its migration at line 332 also selects 1 when no Alternate setting is supplied. All six available TTS consumer caches contain this default. `TTSSettingsDocumentation.lua` documents the enum argument and numeric return, but no native default. The legacy `c_system_api::test_c_tts_settings_get_voice_option_id` oracle is therefore corrected from 0 to 1, with the source cited in the test. This is UI-default evidence, not native default parity; Wrath has no original TTS consumer cache.

## Sources

- [TTS contract](../../specs/c-factory-cf2-tts.md) — persistence and marker decisions.
- [Shape contracts](../../specs/cf2-shape-contracts.md) — deferred requirements and profile selection.
- [API shape contracts](../../specs/api-shape-contracts.md) — equipment, upgrade, stable and store requirements.

## See Also

- [[lua-api]] — TTS model mechanics.
- [[consumer-selection-and-red-proof]] — consumer applicability and RED limits.
