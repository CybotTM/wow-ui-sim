//! `C_Heirloom` probe surface backed by `WorldState.heirlooms` and
//! `WorldState.collected_heirlooms`.
//!
//! Migrates 2 entries off `NAMESPACE_NIL_STUBS`:
//!
//! - `C_Heirloom.GetHeirloomInfo(itemID)` — returns the 10-value
//!   retail tuple (`name, itemEquipLoc, isPvP, itemTexture,
//!   upgradeLevel, source, searchFiltered, effectiveLevel, minLevel,
//!   maxLevel`). Reads `WorldState.heirlooms` for matching `itemID`.
//! - `C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(index)` — returns
//!   the itemID at a 1-based display index, or `nil` out of range.
//!
//! The stub list misspelled the index method as
//! `GetHeirloomItemIDFromDisplayedSlot` — that name is unused by the
//! Blizzard collection UI (confirmed via
//! `Blizzard_HeirloomCollection.lua:276`). Dropping the dead entry
//! and registering the real API instead.
//!
//! `itemEquipLoc` is returned as a string (`"INVTYPE_HEAD"`) even
//! though `apis.yaml` claims `number` — the collection UI feeds the
//! value to `GetHeirloomCategoryFromInvType(invType)` which string-
//! compares it against `"INVTYPE_*"` constants.

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_heirloom_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Heirloom")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetHeirloomInfo",
        c_heirloom_get_heirloom_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetHeirloomItemIDFromDisplayedIndex",
        c_heirloom_get_item_id_from_displayed_index,
    )?;
    Ok(())
}

fn c_heirloom_get_heirloom_info(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    let entry = {
        let sim = borrow_state(state)?;
        sim.world
            .heirlooms
            .iter()
            .find(|h| h.item_id == item_id)
            .cloned()
    };
    let Some(h) = entry else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let name = create_string(state, &h.name);
    let equip_loc = create_string(state, &h.equip_loc);
    let source = create_string(state, &h.source);
    state.push(name);
    state.push(equip_loc);
    state.push(Val::Bool(false));
    state.push(Val::Num(h.icon as f64));
    state.push(Val::Num(h.upgrade_level as f64));
    state.push(source);
    state.push(Val::Bool(false));
    state.push(Val::Num(h.max_level as f64));
    state.push(Val::Num(h.min_level as f64));
    state.push(Val::Num(h.max_level as f64));
    Ok(10)
}

fn c_heirloom_get_item_id_from_displayed_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let item_id = usize::try_from(index.saturating_sub(1))
        .ok()
        .and_then(|idx| borrow_state(state).ok()?.world.heirlooms.get(idx).map(|h| h.item_id));
    match item_id {
        Some(id) => state.push(Val::Num(id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}
