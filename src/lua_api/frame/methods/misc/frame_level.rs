//! Frame level, raise/lower, and parent-level hierarchy methods.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn register(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, mt, "Lower", lower)?;
    table_set_rust_fn_static(state, mt, "Raise", raise)?;
    table_set_rust_fn_static(state, mt, "GetHighestFrameLevel", get_highest_frame_level)?;
    table_set_rust_fn_static(state, mt, "GetRaisedFrameLevel", get_raised_frame_level)?;
    table_set_rust_fn_static(state, mt, "IsUsingParentLevel", is_using_parent_level)?;
    table_set_rust_fn_static(state, mt, "SetUsingParentLevel", set_using_parent_level)?;
    Ok(())
}

pub fn lower(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    borrow_state_mut(state)?.lower_frame(id);
    Ok(0)
}

pub fn raise(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    borrow_state_mut(state)?.raise_frame(id);
    Ok(0)
}

pub fn get_highest_frame_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let iterate_all = Option::<bool>::from_stack(state, 2)?.unwrap_or(false);
    let level = highest_frame_level(&borrow_state(state)?.widgets, id, iterate_all);
    state.push(Val::Num(level as f64));
    Ok(1)
}

fn highest_frame_level(
    widgets: &crate::widget::WidgetRegistry,
    root_id: u64,
    iterate_all_children: bool,
) -> i32 {
    let Some(root) = widgets.get(root_id) else {
        return 0;
    };
    if !iterate_all_children {
        return root.frame_level;
    }
    let mut highest = root.frame_level;
    let mut queue = root.children.clone();
    while let Some(child_id) = queue.pop() {
        let Some(child) = widgets.get(child_id) else {
            continue;
        };
        highest = highest.max(child.frame_level);
        queue.extend(child.children.iter().copied());
    }
    highest
}

pub fn get_raised_frame_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // Retail 12.0.5 reports 0 for simple sibling frames before and after
    // Raise()/Lower(). Internal raise_order remains render bookkeeping, not
    // part of this Lua-visible getter.
    let _ = id;
    state.push(Val::Num(0.0));
    Ok(1)
}

pub fn is_using_parent_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = borrow_state(state)?
        .widgets
        .get(id)
        .map(|f| !f.has_fixed_frame_level)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn set_using_parent_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let using_parent_level = bool::from_stack(state, 2)?;
    // NOTE: lockdown check omitted — requires mlua context for combat_lockdown::check_and_fire
    let mut sim = borrow_state_mut(state)?;
    let inherited_level = inherited_parent_level(&sim.widgets, id);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.has_fixed_frame_level = !using_parent_level;
        if let Some(level) = inherited_level.filter(|_| using_parent_level) {
            frame.frame_level = level;
        }
    }
    super::super::methods_hierarchy::propagate_strata_level_pub(&mut sim.widgets, id);
    Ok(0)
}

fn inherited_parent_level(widgets: &crate::widget::WidgetRegistry, id: u64) -> Option<i32> {
    let frame = widgets.get(id)?;
    let parent_level = widgets.get(frame.parent_id?)?.frame_level;
    Some(parent_level + frame.frame_level_offset.unwrap_or(1))
}
