//! Visibility methods (Show, Hide, SetShown) and OnShow/OnHide recursive firing.

use crate::lua_api::frame::handle::{frame_lud, get_sim_state, lud_to_id};
use mlua::{LightUserData, Lua};

/// Fire OnShow on a frame and recursively on its visible children.
pub(crate) fn fire_on_show_recursive(lua: &Lua, id: u64) -> mlua::Result<()> {
    fire_script_recursive(lua, id, "OnShow")
}

/// Fire OnHide on a frame and recursively on its visible children.
///
/// Children are collected BEFORE the handler fires so their OnHide runs too
/// (matches WoW: parent hides → children become effectively hidden).
pub(crate) fn fire_on_hide_recursive(lua: &Lua, id: u64) -> mlua::Result<()> {
    fire_script_recursive(lua, id, "OnHide")
}

/// Register Show, Hide, SetShown methods.
pub(super) fn add_show_hide_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("Show", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let was_hidden = {
            let state = state_rc.borrow();
            state.widgets.get(id).map(|f| !f.visible).unwrap_or(false)
        };
        state_rc.borrow_mut().set_frame_visible(id, true);
        if was_hidden {
            fire_on_show_recursive(lua, id)?;
        }
        Ok(())
    })?)?;

    methods.set("Hide", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let was_visible = {
            let state = state_rc.borrow();
            state.widgets.get(id).map(|f| f.visible).unwrap_or(false)
        };
        if was_visible {
            fire_on_hide_recursive(lua, id)?;
        }
        state_rc.borrow_mut().set_frame_visible(id, false);
        Ok(())
    })?)?;

    methods.set("SetShown", lua.create_function(|lua, (ud, shown): (LightUserData, bool)| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let was_hidden = {
            let state = state_rc.borrow();
            state.widgets.get(id).map(|f| !f.visible).unwrap_or(false)
        };
        let was_visible = !was_hidden;
        state_rc.borrow_mut().set_frame_visible(id, shown);
        if shown && was_hidden {
            fire_on_show_recursive(lua, id)?;
        } else if !shown && was_visible {
            fire_on_hide_recursive(lua, id)?;
        }
        Ok(())
    })?)?;

    Ok(())
}

/// Shared implementation: fire a script handler on a frame, then recurse
/// into visible children.
fn fire_script_recursive(lua: &Lua, id: u64, handler_name: &str) -> mlua::Result<()> {
    // Collect visible children first (before the handler potentially hides them).
    let children: Vec<u64> = {
        let state_rc = get_sim_state(lua);
        let st = state_rc.borrow();
        st.widgets
            .get(id)
            .map(|f| {
                f.children
                    .iter()
                    .filter(|&&cid| st.widgets.get(cid).map(|c| c.visible).unwrap_or(false))
                    .copied()
                    .collect()
            })
            .unwrap_or_default()
    };

    if let Some(handler) = crate::lua_api::script_helpers::get_script(lua, id, handler_name) {
        let frame_val = frame_lud(id);
        if let Err(e) = handler.call::<()>(frame_val) {
            crate::lua_api::script_helpers::call_error_handler(lua, &e.to_string());
        }
    }

    for child_id in children {
        fire_script_recursive(lua, child_id, handler_name)?;
    }

    Ok(())
}
