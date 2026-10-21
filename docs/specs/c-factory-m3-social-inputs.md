# M3 social startup inputs

M3 replaces observed social/group/chat/guild gaps exposed by [factory removal](c-namespace-nil-members.md). Real C namespace queries and backing types belong in `src/c_api/`; explicit temporary defaults belong in `src/lua_api/workarounds/temporary/`. No arbitrary member factory is permitted. [Lua API notes](../wiki/systems/lua-api.md) describe the shared simulator state.

## What it must do

- [ ] On the six observed non-Wrath profiles, `C_GuildInfo.GetGuildTabardInfo(unit?)` returns nil when local-player numeric tabard metadata is unavailable, or for units without a supplied snapshot. Wrath keeps this unregistered member nil.
- [ ] Supplied tabard metadata projects existing guild-logo RGB values into fresh ColorMixin objects, with numeric `emblemFileID` and `emblemStyle`, matching the cached `GuildInfoSharedDocumentation.lua` DTO.
- [ ] `A_Admin.SetGuildTabardMetadata(fileID?, style?)` accepts a complete pair of nonnegative u32 values, clears metadata when both are absent, and rejects invalid/partial updates without changing prior state. This is a simulator input contract, not a native setter.
- [ ] Replacing the legacy logo through `SetGuildEmblem` invalidates numeric metadata that cannot be derived from its filename. Legacy RGB/filename behavior remains intact.
- [ ] Other observed M3 members use existing backing state where available; otherwise only explicitly named temporary inactive-service contracts, each naming its missing backing system and retirement condition. No native policy is inferred from a method name.
- [ ] Non-Wrath documented social contracts use explicit simulator defaults until their backing services exist: no cross-faction roster, no silence restriction, chat reception allowed, LFD/GroupFinder and LFG authentication unavailable. Mists preserves its existing guild-bank storage capability; other profiles report that unmodeled capability unavailable.
- [ ] Retail/PTR/Forever mentorship reports documented `None=0`, Recruit-a-Friend availability and legacy friend mode are disabled, and language capability follows the existing explicit legacy language list. Forever friend-tag game-type capability is disabled. These are named temporary policies, not native defaults.
- [ ] Existing explicit methods, strict/retired surfaces and arbitrary-member nil behavior remain intact. Original startup assertions remain unchanged.

## How it works

- [C namespace lookup and diagnostics](../wiki/systems/addon-loading.md#missing-c-members-f0-2026-10-20)

## Implementation inventory

- `src/lua_api/state_types/character_world.rs` — existing guild-logo RGB and filename state; numeric metadata integration point.
- `src/lua_api/globals/real/guild_logo.rs` — existing legacy projection, retained.
- `src/c_api/c_guild_info.rs` — typed tabard metadata and C query.
- `src/lua_api/workarounds/temporary/social_service_defaults.rs` — named missing-service policies, scoped by cached declarations.
- `src/lua_api/workarounds/temporary/startup_namespace_defaults.rs` — existing named empty-state defaults; do not replace real state with a factory.

## Tests asserting this spec

- `tests/guild_tabard_info.rs` — documented DTO, supplied metadata, fresh mutable colors, validation/clearing and legacy state preservation.
- `tests/c_factory_m3_social.rs` — per-profile generated-documentation return contracts and explicitly chosen temporary policies.
- `tests/c_namespace_nil_contract.rs`, `tests/nil_symbol_access.rs` — removal contract retained.
- `tests/startup_warnings.rs` — unchanged final startup health gate.

## Known gaps (current cycle)

- [ ] Implement and prove the tabard snapshot behavior against the original GuildMicroButton consumer (`MainMenuBarMicroButtons.lua:1254` on Retail).
- [ ] Resolve remaining observed M3 roots: mentorship status, cross-faction party status, guild-bank availability, friend-tag/game-type support, social mute/receive policy, language capability, legacy-friend mode, LFD/GroupFinder availability, LFG authentication and Recruit-a-Friend availability.
- [ ] Re-capture the post-M3 frontier without hiding failures belonging to other batches.

## Out of scope

- Guessing modern native conditions or reconstructing client code.
- M1/M2/M4/M5/T6 producer work, except preserving their recorded failures and necessary existing state.
- Changes to Blizzard/vendor Lua, rilua, original persisted CVars, or arbitrary member fabrication.
- Newly exposed members absent from the frozen gap-list: retain them as residuals with profile, original consumer location, stderr receipt and causal root fix; do not expand this batch.
