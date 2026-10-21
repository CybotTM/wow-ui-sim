# Wrath guild rank flags

Wrath's legacy `GuildControlGetRankFlags` projects selected guild rights as positional results, not a table. Source: `src/lua_api/globals/guild_control.rs`; [model notes](../wiki/systems/lua-api.md#wrath-guild-rank-rights). Authority is the laptop's build 12340 static reconstruction received 2026-10-21, not native execution.

## What it must do

- [x] Under `client-wrath`, the global ignores every argument and returns exactly 17 positions from the selected rank's supplied rights: number `1` for enabled rights, nil otherwise, including trailing nils.
- [x] The original `GuildControlCheckboxUpdate(GuildControlGetRankFlags())` consumer applies the selected rights to native checkbox children. Position 14 remains an ordinary API result even though native UI omits that checkbox.
- [x] With a valid selected rank but no guild, return 17 nils.
- [ ] Other profiles' global and the existing `C_GuildInfo.GuildControlGetRankFlags` table projection retain explicit-index lookup and boolean entries.

Native mask order for the positional input is `0x1, 0x2, 0x4, 0x8, 0x80, 0x100, 0x10, 0x20, 0x1000, 0x2000, 0x4000, 0x8000, 0x10000, 0x20000, 0x40000, 0x80000, 0x100000`. The simulator's supplied `flags` vector is positional; it does not store a native bitmask.

## How it works

- [Legacy guild state and projection](../wiki/systems/lua-api.md#wrath-guild-rank-rights).

## Implementation inventory

- `src/lua_api/globals/guild_control.rs` — legacy positional projection and preserved table projection.
- `src/lua_api/state_types/character_world.rs` — existing selected rank and supplied rights.
- `src/lua_api/globals/admin_zone_economy.rs` — existing `A_Admin.SetGuildRanks` fixture input.

## Tests asserting this spec

- `tests/wrath_guild_rank_flags.rs` — original native checkbox consumer, exact result arity/types, ignored arguments, position 14 and no-guild results.
- `tests/guild_control.rs` — selected-rank data and preserved namespace/table behavior.
- `tests/test_guild_panel.rs` — native rank menu consumers; unrelated bank-permission gaps remain distinct.

## Known gaps (current cycle)

Wrath proof: all three native contract cases and all 14 existing guild-model cases pass. Stock Wrath startup reports an empty Lua-error array. This does not establish native-client execution or the excluded mutation lifecycle.

- [ ] Retail profile preservation proof is pending.

## Out of scope

- Unselected ranks and ranks beyond the native ten-entry array: undefined or unsupported native reads; no parity is asserted for the simulator sentinel.
- Rank mutation lifecycle and validation: native edit flags are separate from stored rights, while the current `GuildControlSetRankFlag` changes the supplied vector immediately. Staged edits/save semantics are not modeled by this return-shape correction.
