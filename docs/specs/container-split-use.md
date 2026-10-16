# Supplied container split and use requests

Status: corrected tests/passive scaffold staged; RED/GREEN unexecuted. Wrath legacy dispatch is cfg-gated and required, never runtime-skipped; its baseline is inert.

Container splitting and accepted item-use requests share state across `C_Container` and legacy entry points. Backing behavior belongs in `src/c_api/container_inventory.rs`; passive eligibility and request records belong in `src/c_api/container_actions.rs`. See [inventory context](../wiki/systems/lua-api.md).

This is a **simulator-authored supplied-state policy**, not native parity. Public BankFrame callbacks establish split dispatch and HUDInventoryButton/ChatFrame establish use dispatch. They do not establish native validation, consumption, casting, cooldown, or event effects.

## What it must do

### Splitting

- [ ] A supplied five-item stack split by two retains three in its source and puts the same item ID/count two on the cursor, with source bag/slot origin. Unrelated stacks stay unchanged; dropping through existing pickup conserves the split quantity.
- [ ] `GetCursorInfo()` for an item returns `("item", itemID, stackCount)`. **The third count result is simulator-authored**, not a claim about native return positions. Other cursor kinds keep existing results.
- [ ] Zero, negative, fractional, non-number (including numeric-string), or amounts at least the source count are inert. Missing source and occupied cursor are inert. Positive controls prove that valid operations still mutate state.
- [ ] Namespaced and evidenced Wrath legacy calls use the same state transition. Finite integer bag, slot, and amount inputs are required; existing supplied bag keys are used without inventing native bank/capacity rules.

### Bounded use dispatch

- [ ] An occupied positive-count bag slot whose item ID is explicitly supplied in `container_actions.usable_items` appends one `{bag, slot, item_id}` to environment-local `use_requests` on each namespaced or legacy call.
- [ ] Unsupplied eligibility and empty slots append nothing. Requests neither consume/move the stack nor replace an existing cursor. A second environment starts without eligibility or requests.
- [ ] Eligibility is supplied simulator state, not an inference from item ID, quality, a native catalog, or callability. Supplied five-stack Hearthstone is deliberately synthetic; native stackability is not asserted.

## How it works

- [Lua API context](../wiki/systems/lua-api.md).
- [Existing cursor transfer contract](cursor-item-transfer.md).

## Implementation inventory

- `src/c_api/container_actions.rs` — passive eligibility/request types, installed with the tests-stage scaffold.
- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs` — per-environment passive state and initialization.
- `src/c_api/mod.rs` — public fixture/model module.
- `src/c_api/container_inventory.rs` — shared split/use producers and namespaced publication.
- `src/lua_api/globals/inventory_verbs.rs` — shared legacy publication and item cursor-count projection.
- `src/lua_api/globals/stubs/global_stubs.rs` — move legacy UseContainerItem no-op into the explicit passive scaffold registration; producer replaces that passive body.
- `src/lua_api/workarounds/temporary/container_default_shapes.rs` — remove only split/use no-op entries; retain unrelated defaults.

## Tests asserting this spec

`tests/container_split_use.rs` (auto-discovered grouped integration module):

- `b1_split_moves_supplied_quantity_and_reports_cursor_count`
- `b1_invalid_amounts_are_inert_then_valid_split_succeeds`
- `b1_occupied_cursor_and_missing_source_are_inert`
- `b3_namespaced_and_legacy_use_record_the_same_supplied_request_model`
- `b3_unsupplied_and_empty_use_are_inert_then_eligible_use_records`

## Known gaps (current cycle)

Wrath public legacy calls: `~/.cache/wow-ui-sim/blizzard-ui/wrath/FrameXML/BankFrame.lua:15` (`SplitContainerItem`) and `ChatFrame.lua:1005` (`UseContainerItem`). Retail/PTR/Mists/Forever cached consumers use `C_Container`; Era/Anniversary caches are absent, so no legacy-consumer conclusion is claimed for them. Wrath tests try legacy first so its inert baseline fails on state values; other profiles compile only namespaced cases. `UseContainerItem` moves from HEAD's generic no-op list to an explicit passive publication with identical no-return/no-op behavior; the old list entry is removed, not duplicated. Missing Wrath legacy split receives the same passive body.

- [ ] Staged only: execute behavioral RED with passive scaffold before applying producer; then execute focused GREEN/profile preservation. No test/build/type/format acceptance is claimed here.
- [ ] Use dispatch is observable only through supplied model state. No gameplay effect or public event is established by the inspected call sites; B3 gameplay compatibility remains unproven.

## Out of scope

Native effects, eligibility discovery, target argument behavior (including Wrath's optional target), bank transfer modes, combat/security gates, BAG_UPDATE/CURSOR_CHANGED timing, cooldowns, casting, healing, consumption, stack maxima, merging, richer cursor metadata, automatic ClearCursor return, and runtime reachability of cached public consumers. No vendor edits or reconstructed implementation evidence.
