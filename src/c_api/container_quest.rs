//! Explicit item-to-quest metadata supplied by the simulator host.
//! No native item/quest catalogue or association inference is provided.

use crate::lua_api::methods::{borrow_state, create_table, table_set_static};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ContainerItemQuestMetadata {
    pub is_quest_item: bool,
    pub quest_id: Option<i32>,
}

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = super::ensure_namespace(state, "C_Container")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetContainerItemQuestInfo",
        get_quest_info,
    )
}

/// Rich active-log membership is authoritative; completion history is not.
/// Empty/invalid slots have no quest metadata by authored simulator policy.
pub(crate) fn read_item_quest_info(
    state: &LuaState,
) -> LuaResult<(ContainerItemQuestMetadata, bool)> {
    let bag = i32::from_stack(state, 1)?;
    let slot = i32::from_stack(state, 2)?;
    let sim = borrow_state(state)?;
    if slot <= 0 || slot > sim.bag_num_slots(bag) {
        return Ok((ContainerItemQuestMetadata::default(), false));
    }
    let metadata = sim
        .get_bag_item(bag, slot)
        .and_then(|(item_id, _)| sim.container_item_quests.get(&item_id))
        .copied()
        .unwrap_or_default();
    let is_active = metadata.quest_id.is_some_and(|id| {
        sim.quest_log_entries
            .entries
            .iter()
            .any(|entry| entry.quest_id == id)
    });
    Ok((metadata, is_active))
}

fn get_quest_info(state: &mut LuaState) -> LuaResult<u32> {
    let (metadata, is_active) = read_item_quest_info(state)?;
    let info = create_table(state);
    table_set_static(
        state,
        info,
        "isQuestItem",
        Val::Bool(metadata.is_quest_item),
    );
    let quest_id = metadata
        .quest_id
        .map_or(Val::Nil, |id| Val::Num(f64::from(id)));
    table_set_static(state, info, "questID", quest_id);
    table_set_static(state, info, "isActive", Val::Bool(is_active));
    state.push(info);
    Ok(1)
}
