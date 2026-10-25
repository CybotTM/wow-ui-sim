# Wrath guild-bank rank permissions

Wrath's legacy bank permission readers project explicit selected-control-rank inputs. [Model and static contradiction evidence](../wiki/investigations/wrath-guild-info.md#bank-permission-model). Authority: laptop build12340 reconstructed-source answer `guildbank-perms-12340.md`, SHA256 `f58e304a7d9c20813f23d871f9666547d0af3924d66f341eeed044b0b1588951`; no native runtime was run.

## What it must do

- [x] Only Wrath registers `GetGuildBankTabPermissions` and `GetGuildBankWithdrawLimit` through the new modeled path; other profiles retain their existing surfaces.
- [x] Tab getter returns exactly four values: numeric1/nil for masks1,2,4, then the supplied numeric stack limit. Normalize by integer conversion, subtract1, narrow to8 bits; normalized indices >=6 return nil,nil,nil,0 without rank access. Missing/non-number-convertible argument raises a usage error; extra arguments are ignored.
- [x] Money getter returns exactly one number: nonnegative supplied limit integer-divided by10000, negative unchanged. Extra arguments are ignored.
- [x] Both read the selected control rank (Lua one-based), not player rank, membership or bank-frame visibility. Neither emits events.
- [x] Host input supplies each received rank's money, six masks and six stack limits. Initial world-entry rank0 has masks/stacks -1 and money0; supplied data replaces the initial image, including rank0. No other native default is invented.
- [x] Original FriendsFrame dropdown/tab consumers update visible labels, checked states and stack/gold edit text from varying supplied ranks/tabs.

## How it works

- [Model and contradiction table](../wiki/investigations/wrath-guild-info.md#bank-permission-model).

## Implementation inventory

- `src/lua_api/state_types/character_world.rs` — explicit rank-permission input rows in existing world guild state.
- `src/lua_api/state_types/character_world_defaults.rs` — Wrath initial world-entry rank0 image.
- `src/lua_api/globals/real/wrath_guild_bank.rs` — legacy modeled projections; registration selected by existing Wrath runtime branch.

## Tests asserting this spec

- `tests/test_guild_panel.rs`: `guild_control_rank_settings_dropdown_shows_rank_rows`, `native_nonleader_rank_dropdown_completes_bank_permission_update`; both use `fixtures/wrath_f03/bank_rank_permissions.lua` after original dropdown assertions.
- `tests/wrath_guild_rank_flags.rs`: three `bank_permissions_*` cases cover exact types/arity, independent rights, selected-rank input, no events/closed bank, argument conversion, initial rank0 and replacement, signed money limits.

## Known gaps (current cycle)

- [x] Saved proof at GREEN `469f6ffed` and producer-minus RED `9f180b154`: five bank selectors fail in RED and pass in GREEN; Wrath preservation56 and Retail53 pass in GREEN. Verifier8738 accepted these receipts. See the [proof status](../wiki/investigations/wrath-guild-info.md#accepted-runtime-proof).
- [x] Current-master integration refresh at `14606d868` (merged master `08b423c5c`): Wrath/Retail all-target checks pass without warnings, five bank exact cases pass, fresh Wrath stock is `[]`. Fresh Retail master/merged stock is identical:30 keys/34 occurrences, unchanged30/removed0/added0/count-changed0. See [integration proof](../wiki/investigations/wrath-guild-info.md#current-master-integration-proof).
- [ ] Non-native simulator policy: a valid tab with no selected/supplied rank raises an explicit unavailable-input error instead of inventing a numeric limit. Native rank -1 is source-undefined; tests do not assert its result.

## Out of scope

Network packet reception, setters/staged edits/save, repeated world-entry/reset policy and native runtime certification. Host code supplies received data directly; no server values or nonleader defaults are fabricated. Existing rank-mutation APIs are not a modeled guild-bank server protocol.
