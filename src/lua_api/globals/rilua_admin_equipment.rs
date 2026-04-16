//! Rilua A_Admin handlers — Equipment.
//!
//! Extracted from rilua_admin_extras.rs per the 750-line file cap and to keep
//! each sub-module focused on a single concern. The parent entry
//! point in rilua_admin.rs imports these as pub(super) and weaves
//! them into the A_Admin TableBuilder chain.

use crate::lua_api::rilua_methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

// ── Equipment ─────────────────────────────────────────────────────────────────

pub(super) fn equip_item(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::state_types::{CharacterStats, EquippedItem};
    let slot = i32::from_stack(state, 1)?;
    let item_id = u32::from_stack(state, 2)?;
    let mut st = borrow_state_mut(state)?;
    st.player.equipped_items.insert(
        slot,
        EquippedItem {
            item_id,
            enchant_id: 0,
            gem_ids: [0, 0, 0],
        },
    );
    st.player.stats = CharacterStats::compute(&st.player.equipped_items, st.player.class_index);
    Ok(0)
}

pub(super) fn unequip_item(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::state_types::CharacterStats;
    let slot = i32::from_stack(state, 1)?;
    let mut st = borrow_state_mut(state)?;
    st.player.equipped_items.remove(&slot);
    st.player.stats = CharacterStats::compute(&st.player.equipped_items, st.player.class_index);
    Ok(0)
}
