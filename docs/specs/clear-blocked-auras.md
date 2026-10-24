# Selected-unit aura cleanup

## Scope

Retail/PTR original `ArenaUnitFrameDebuffMixin.Update` and the original CompactRaidFrameManager initialization interrupted by that cleanup. Excludes combat filters, group-buff configuration, other profiles, layout/XML changes and vendor patches.

## Required behavior

- `C_UnitAuras.ClearBlockedAuras(unit)` removes display blocks for the selected unit from the same effective block set used by `AddBlockedAura` and visible enumeration. Existing backing is `C_UnitAuras._blockedAuras`; do not clear the unused `SimState.blocked_auras_by_unit` field or introduce a second store.
- Clearing party1 restores supplied aura instance IDs201/202; player101 remains blocked. Repeated clears and re-block/re-clear preserve aura DTOs and other units.
- Original raid-manager OnLoad must continue through both divider-pool assignments. Original GROUP_ROSTER_UPDATE then PLAYER_ENTERING_WORLD, options-flow updates, pool acquire/release/reacquire must complete. Tests must retain initial OnLoad errors; no manually supplied pools.
- Place the endpoint under `src/c_api/`, with only the narrow existing-state sharing changes necessary. No invented native defaults or ambient inputs.

## Acceptance and status

**Tests staged only; no compilation, observed RED, producer change or GREEN yet.** Each original-consumer selector must independently reject constant/no-op/wrong-unit implementations through concrete visibility observations. Missing-member failure alone does not establish that proof.

| Selector in integration target | Profiles | Required observations | Current proof |
|---|---|---|---|
| `cf2_combat_aura_controls::original_arena_cleanup_restores_selected_unit_aura_enumeration` | Retail/PTR | Original selected arena cleanup; party201/202 restored, player101 blocked, repeat and re-block passes, unchanged DTOs | Authored; execution pending |
| `stock_residual_roots::original_raid_manager_initializes_divider_pools_after_arena_cleanup` | Retail/PTR | Initial selected XML OnLoad retained; real arena child; assigned-unit transitions; original roster/world order; real divider pools acquire/release/reacquire | Authored; execution pending |
| `cf2_combat_aura_controls::original_arena_unassigned_unit_rejects_nil_best_guess` | Retail/PTR | Assigned clear first; original SetUnit(nil) rejects; omitted input rejects; blocks survive rejection and reassignment clears correctly | Authored best guess; execution pending |

Existing B-AURA and U-DIVIDER test ownership was adopted rather than reimplemented. The U-DIVIDER case retains the normal initial load, not a repaired/recreated manager after failed initialization.

## Best-guess nil policy — non-native

User authorized a reasoned best guess on2026-10-24, pending user-supplied Retail native findings. **Guess: reject nil/omitted unit using the existing required-string argument conversion, before mutating any blocks; do not silently no-op or clear globally.** Implement only after observed RED. The earlier producer-preparation patch's inferred nil no-op policy is not adopted.

Basis: generated UnitAuraDocumentation declares ClearBlockedAuras.unit as UnitTokenRestrictedForAddOns with Nilable=false and no returns. Existing `src/c_api/c_unit_auras.rs::is_private_aura_filtered_out_by_instance_id` uses required `String::from_stack`; that conversion rejects nil and nonstrings. `c_unit_aura_spell_queries.rs::read_public_unit` instead explicitly accepts nil for a query with no modeled unit. Siblings are not uniform; required mutation input and documented non-nilability favor strict conversion here. Legacy AddBlockedAura accepts missing unit as an empty-string key, but that legacy permissiveness is not native evidence and does not justify an empty-token mutation for this new endpoint. Unknown string tokens have no matching block entries unless previously blocked; do not invent unit validation or player substitution.

The separate nil selector tests the original widget transition and omitted direct call, with assigned-unit cleanup before and after to reject constant/no-op/wrong-unit implementations. This policy must be labeled BEST GUESS, non-native in the future producer's code comment and replaced if authorized Retail evidence contradicts it. No decompiled source is read by this lane.

Source-selected `CompactArenaFrameMixin.RefreshMembers` assigns `"arena"..index` before `DebuffFrame:SetUnit`; the historical stock ClearBlockedAuras stack includes that path. Stock records do not record argument values. Therefore none of the historical eight keys/ten occurrences per profile is established to depend on nil handling. Whether other initial ordering reaches nil requires runtime evidence; initial diagnostics must not be cleared to avoid it.

## Runtime selection

For Retail/PTR, normal game-screen closure starts at `Blizzard_CompactRaidFrames`, depends on `Blizzard_RaidFrame` and `Blizzard_UnitFrame`, and selects the Mainline manager via `[Family]`. `Blizzard_UnitFrame_Mainline.toc` has `AllowLoad: Game`, `AllowLoadGameType: mainline` and selects Mainline CompactArenaFrame Lua/XML. Tests load this closure without overrides and assert original debug source provenance. Source selection is not observed runtime acceptance until the tests execute.

## Full-stock gate

Capture full exact message→count maps on the own-base stock revision and final producer revision for each profile. Compare removed keys, added keys and changed surviving counts, preserving ScriptErrors wrappers. Compare the own-base revision with current master, not the older SuperTrack base alone: club replaces invitation errors with unread errors and house removes two keys. Historical aura-family opportunity is at most eight keys/ten occurrences per profile, not a promised result or prefork acceptance.
