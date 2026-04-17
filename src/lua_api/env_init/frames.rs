//! Frame metatable and builtin-frame initialisation.

use crate::lua_api::frame::methods::{
    button_anchor_hierarchy, core_state, map_frames, misc,
    text_attribute_event, widgets,
};
use crate::lua_api::methods::{registry_set, table_set};
use rilua::{LuaApiMut, Val};
use std::cell::RefCell;
use std::rc::Rc;

use super::super::builtin_frames::create_builtin_frames;
use super::super::state::SimState;

/// Create built-in frames in the widget registry before Lua loads.
/// Registers a `__BuiltIn` pseudo-addon as their owner.
pub(crate) fn init_builtin_frames(state: &Rc<RefCell<SimState>>) {
    let mut s = state.borrow_mut();
    let owner = s.addons.len() as u16;
    s.addons.push(super::super::AddonInfo {
        folder_name: "__BuiltIn".to_string(),
        title: "Built-in Frames".to_string(),
        enabled: true,
        loaded: true,
        ..Default::default()
    });
    let (w, h) = (s.screen_width, s.screen_height);
    create_builtin_frames(&mut s.widgets, w, h, owner);
}

pub(super) fn init_frame_metatable(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let frame_mt = Val::Table(state.gc.alloc_table(rilua::vm::table::Table::new()));
    table_set(state, frame_mt, "__index", frame_mt);
    registry_set(state, "__rilua_frame_mt", frame_mt);

    let Val::Table(frame_mt_ref) = frame_mt else {
        unreachable!("frame metatable must be a table");
    };
    super::super::timer_layout::register_layout_fns_on_table(state, frame_mt_ref)?;
    core_state::register_all(state, frame_mt_ref)?;
    misc::register_all(state, frame_mt_ref)?;
    map_frames::register_all(state, frame_mt_ref)?;
    text_attribute_event::register_all(state, frame_mt_ref)?;
    button_anchor_hierarchy::register_all(state, frame_mt_ref)?;
    widgets::register_all(state, frame_mt_ref)?;

    // Replace the self-referencing `__index` with a shallow clone that omits
    // metamethod keys. Blizzard's restricted code does
    // `CopyTable(GetFrameMetatable().__index)` (RestrictedExecution.lua), which
    // would infinitely recurse if `__index` pointed back at the metatable
    // itself. The clone captures the methods registered above and stays
    // stable for the lifetime of the VM (method registration only happens
    // here at init).
    let frame_index = build_frame_index_table(state, frame_mt_ref);
    table_set(state, frame_mt, "__index", Val::Table(frame_index));
    Ok(())
}

/// Build a shallow, non-cyclic clone of the frame metatable's method entries.
///
/// Skips keys that start with `__` (metamethods) so the resulting table only
/// exposes frame methods — matching what Blizzard's restricted loader expects
/// from `GetFrameMetatable().__index`.
fn build_frame_index_table(
    state: &mut rilua::vm::state::LuaState,
    frame_mt_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> rilua::vm::gc::arena::GcRef<rilua::vm::table::Table> {
    let new_ref = state.gc.alloc_table(rilua::vm::table::Table::new());
    let entries = state
        .gc
        .tables
        .get(frame_mt_ref)
        .map(|table| table.hash_entries())
        .unwrap_or_default();
    for (key, value) in entries {
        if let Val::Str(str_ref) = key
            && let Some(name) = state.gc.string_arena.get(str_ref)
            && name.data().starts_with(b"__")
        {
            continue;
        }
        if let Some(t) = state.gc.tables.get_mut(new_ref) {
            let _ = t.raw_set(key, value, &state.gc.string_arena);
        }
    }
    new_ref
}
