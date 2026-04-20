//! Template helpers shared by the rilua loader/runtime path.

pub(crate) mod direct;

use crate::lua_api::methods::{frame_ref, sync_child_to_rilua, table_set};
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub fn set_intrinsic(state: &mut LuaState, frame_id: u64, base: &str) {
    let Ok(frame) = frame_ref(state, frame_id) else {
        return;
    };
    let value = crate::lua_api::methods::create_string(state, base);
    table_set(state, frame, "intrinsic", value);
}

pub fn assign_parent_key(
    state: &mut LuaState,
    parent_id: u64,
    parent_key: &str,
    child_id: u64,
) -> LuaResult<()> {
    let (target_parent_id, resolved_key) = resolve_parent_key_target(state, parent_id, parent_key);
    let Some(target_parent_id) = target_parent_id else {
        return Ok(());
    };

    {
        let mut sim = crate::lua_api::methods::borrow_state_mut(state)?;
        if let Some(parent) = sim.widgets.get_mut_visual(target_parent_id) {
            parent.children_keys.insert(resolved_key.clone(), child_id);
        }
        if sim.widgets.get(child_id).and_then(|child| child.parent_id) == Some(target_parent_id)
            && let Some(child) = sim.widgets.get_mut_visual(child_id)
        {
            child.parent_key = Some(resolved_key.clone());
        }
    }

    sync_child_to_rilua(state, target_parent_id, &resolved_key, child_id)
}

fn resolve_parent_key_target(
    state: &LuaState,
    parent_id: u64,
    parent_key: &str,
) -> (Option<u64>, String) {
    if let Some(key) = parent_key.strip_prefix("$parent.") {
        let target_parent = crate::lua_api::methods::borrow_state(state)
            .ok()
            .and_then(|sim| {
                sim.widgets
                    .get(parent_id)
                    .and_then(|parent| parent.parent_id)
            });
        return (target_parent, key.to_string());
    }
    (Some(parent_id), parent_key.to_string())
}

pub fn fire_deferred_child_onloads(_state: &mut LuaState) -> usize {
    0
}

pub(super) fn get_size_values(size: &crate::xml::SizeXml) -> (Option<f32>, Option<f32>) {
    if size.x.is_some() || size.y.is_some() {
        (size.x, size.y)
    } else if let Some(abs) = &size.abs_dimension {
        (abs.x, abs.y)
    } else {
        (None, None)
    }
}
