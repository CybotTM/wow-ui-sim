# CF-2 TTS integration

Verified: 2026-10-25. TTS integration defers whole non-TTS test files that fail on any registered profile. The owning changes must restore complete files with their producers. Requirements in the existing specs remain in force; removing a fixture is not producer acceptance.

## Restoration inventory

The source checkpoint is `5327a3797`. Historical seven-profile module receipts are from `93c0220b6`; Forever was rebuilt at the checkpoint and each registered case was discovered and run separately. Passing cases below are the union across registered profiles, not an all-profile claim. All paths in the table are under `tests/`; names omit their file's module prefix.

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

The five original-consumer TTS cases pass on the checkpoint's Forever build. Final seven-profile exact-selector, stock-map, warning and formatting gates are separate from this deferral decision. Wrath does not register these CF-2 fixtures. The saved-marker model is user-approved, not native initial/reset evidence.

## Sources

- [TTS contract](../../specs/c-factory-cf2-tts.md) — persistence and marker decisions.
- [Shape contracts](../../specs/cf2-shape-contracts.md) — deferred requirements and profile selection.
- [API shape contracts](../../specs/api-shape-contracts.md) — equipment, upgrade, stable and store requirements.

## See Also

- [[lua-api]] — TTS model mechanics.
- [[consumer-selection-and-red-proof]] — consumer applicability and RED limits.
