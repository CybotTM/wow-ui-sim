//! `C_AzeriteEmpoweredItem` — respec-flow surface read by
//! `Blizzard_AzeriteRespecUI`. Backed by `state.azerite_empowered`.
//!
//! Methods (4 total — scoped to what `Blizzard_AzeriteRespecUI` actually
//! consumes; the full namespace has more entries that no UI addon reaches):
//! - `CloseAzeriteEmpoweredItemRespec(itemLocation?)` — fires
//!   `AZERITE_EMPOWERED_ITEM_RESPEC_CLOSE` so the panel can hide via
//!   `UIPanelWindows.showFailedFunc`.
//! - `GetAzeriteEmpoweredItemRespecCost() → number` — returns
//!   `state.azerite_empowered.respec_cost` (copper). Drives
//!   `AzeriteRespecMixin:RefreshCostFrame`.
//! - `IsAzeriteEmpoweredItem(itemLocation) → bool` — true when the
//!   resolved item id is in `state.azerite_empowered.empowered_items`.
//!   Accepts both the test-shape `{itemID = N}` and the canonical
//!   `ItemLocationMixin` shapes (`{bagID, slotIndex}` and
//!   `{equipmentSlotIndex}`).
//! - `ConfirmAzeriteEmpoweredItemRespec(itemLocation)` — records the
//!   location on `last_confirmed_respec`. No-op when the location is nil
//!   or not a table (mirrors the addon's nil-cursor case).

use crate::c_api::ensure_namespace;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_api::state::ItemLocationData;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

type AzeriteEmpoweredMethod = fn(&mut LuaState) -> LuaResult<u32>;

const AZERITE_EMPOWERED_METHODS: &[(&str, AzeriteEmpoweredMethod)] = &[
    (
        "CloseAzeriteEmpoweredItemRespec",
        close_azerite_empowered_item_respec,
    ),
    (
        "GetAzeriteEmpoweredItemRespecCost",
        get_azerite_empowered_item_respec_cost,
    ),
    ("IsAzeriteEmpoweredItem", is_azerite_empowered_item),
    (
        "ConfirmAzeriteEmpoweredItemRespec",
        confirm_azerite_empowered_item_respec,
    ),
];

pub(crate) fn register_c_azerite_empowered_item_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_AzeriteEmpoweredItem")?;
    for &(name, func) in AZERITE_EMPOWERED_METHODS {
        table_set_rust_fn_static(state, ns, name, func)?;
    }
    Ok(())
}

fn close_azerite_empowered_item_respec(state: &mut LuaState) -> LuaResult<u32> {
    let location = read_optional_location(state, 1);
    {
        let mut sim = borrow_state_mut(state)?;
        sim.azerite_empowered.last_close_request = location;
    }
    dispatch_event_now(state, "AZERITE_EMPOWERED_ITEM_RESPEC_CLOSE", &[])?;
    Ok(0)
}

fn get_azerite_empowered_item_respec_cost(state: &mut LuaState) -> LuaResult<u32> {
    let cost = borrow_state(state)?.azerite_empowered.respec_cost;
    state.push(Val::Num(cost as f64));
    Ok(1)
}

fn is_azerite_empowered_item(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = resolve_location_item_id(state, 1);
    let empowered = match item_id {
        Some(id) => borrow_state(state)?
            .azerite_empowered
            .empowered_items
            .contains(&id),
        None => false,
    };
    state.push(Val::Bool(empowered));
    Ok(1)
}

fn confirm_azerite_empowered_item_respec(state: &mut LuaState) -> LuaResult<u32> {
    let Some(location) = read_optional_location(state, 1) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    sim.azerite_empowered.last_confirmed_respec = Some(location);
    Ok(0)
}

/// Read the `{bagID, slotIndex, equipmentSlotIndex}` shape produced by
/// `ItemLocationMixin` (or its test-friendly cousin `{itemID}`) into the
/// simulator's `ItemLocationData`. Returns `None` when the slot is
/// missing/nil — `CloseAzeriteEmpoweredItemRespec` calls in via
/// `UIPanelWindows.showFailedFunc` with no argument.
fn read_optional_location(state: &mut LuaState, slot: i32) -> Option<ItemLocationData> {
    let Ok(Val::Table(table_ref)) = Val::from_stack(state, slot) else {
        return None;
    };
    let bag_id_key = state.gc.intern_string_static(b"bagID");
    let slot_index_key = state.gc.intern_string_static(b"slotIndex");
    let equipment_slot_key = state.gc.intern_string_static(b"equipmentSlotIndex");
    let table = state.gc.tables.get(table_ref)?;
    let arena = &state.gc.string_arena;
    Some(ItemLocationData {
        bag_id: read_optional_int(table, bag_id_key, arena),
        slot_index: read_optional_int(table, slot_index_key, arena),
        equipment_slot_index: read_optional_int(table, equipment_slot_key, arena),
    })
}

/// Three shapes the respec UI can hand us — the test-friendly
/// `{itemID}` short form, plus the canonical `ItemLocationMixin` bag and
/// equipment-slot variants.
struct LocationFields {
    item_id: Option<i32>,
    bag_id: Option<i32>,
    slot_index: Option<i32>,
    equipment_slot: Option<i32>,
}

/// Resolve an `itemLocation` arg to a concrete item id. Tries
/// `itemID` (test convenience shape) first, then `bagID`/`slotIndex`
/// against `state.bag_items`, then `equipmentSlotIndex` against
/// `player.equipped_items`. Returns `None` when no shape matches —
/// `IsAzeriteEmpoweredItem` then reports false, which mirrors how the
/// addon treats an unbound cursor.
fn resolve_location_item_id(state: &mut LuaState, slot: i32) -> Option<i32> {
    let fields = read_location_fields(state, slot)?;
    if let Some(id) = fields.item_id {
        return Some(id);
    }
    let sim = borrow_state(state).ok()?;
    if let (Some(bag), Some(idx)) = (fields.bag_id, fields.slot_index) {
        if let Some((item_id, _)) = sim.get_bag_item(bag, idx) {
            return Some(item_id as i32);
        }
    }
    let slot_idx = fields.equipment_slot?;
    sim.player
        .equipped_items
        .get(&slot_idx)
        .map(|item| item.item_id as i32)
}

fn read_location_fields(state: &mut LuaState, slot: i32) -> Option<LocationFields> {
    let Ok(Val::Table(table_ref)) = Val::from_stack(state, slot) else {
        return None;
    };
    let item_id_key = state.gc.intern_string_static(b"itemID");
    let bag_id_key = state.gc.intern_string_static(b"bagID");
    let slot_index_key = state.gc.intern_string_static(b"slotIndex");
    let equipment_slot_key = state.gc.intern_string_static(b"equipmentSlotIndex");
    let table = state.gc.tables.get(table_ref)?;
    let arena = &state.gc.string_arena;
    let item_id = match table.get_str(item_id_key, arena) {
        Val::Num(n) if n > 0.0 => Some(n as i32),
        _ => None,
    };
    Some(LocationFields {
        item_id,
        bag_id: read_optional_int(table, bag_id_key, arena),
        slot_index: read_optional_int(table, slot_index_key, arena),
        equipment_slot: read_optional_int(table, equipment_slot_key, arena),
    })
}

fn read_optional_int(
    table: &Table,
    key: rilua::vm::gc::arena::GcRef<rilua::vm::string::LuaString>,
    arena: &rilua::vm::gc::arena::Arena<rilua::vm::string::LuaString>,
) -> Option<i32> {
    match table.get_str(key, arena) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}
