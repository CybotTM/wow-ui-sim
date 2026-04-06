//! Utility functions for frame creation: migration, orphaning, button children.

use super::super::SimState;
use super::super::frame::frame_ref;
use crate::loader::helpers::lua_global_ref;
use mlua::{Lua, Result};
use std::cell::RefCell;
use std::rc::Rc;

/// Register button's default texture children as globals.
///
/// In WoW, named buttons get globals like `ButtonNameNormalTexture`, etc.
/// Sets both the widget registry name and `_G` entry for each child.
pub(super) fn register_button_child_globals(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame_id: u64,
    button_name: &str,
) -> Result<()> {
    let keys: Vec<(String, u64)> = {
        let st = state.borrow();
        let Some(btn) = st.widgets.get(frame_id) else {
            return Ok(());
        };
        [
            "NormalTexture",
            "PushedTexture",
            "HighlightTexture",
            "DisabledTexture",
            "Text",
        ]
        .iter()
        .filter_map(|key| btn.children_keys.get(*key).map(|&id| (key.to_string(), id)))
        .collect()
    };
    let mut st = state.borrow_mut();
    for (key, child_id) in keys {
        let global_name = format!("{}{}", button_name, key);
        st.widgets.set_name(child_id, global_name.clone());
        drop(st);
        if let Ok(val) = frame_ref(lua, child_id) {
            let _ = crate::lua_api::secure_env::set_in_both_envs(lua, &global_name, val);
        }
        st = state.borrow_mut();
    }
    Ok(())
}

/// Remove old frame from its parent's children and hide it.
pub(super) fn orphan_old_frame(widgets: &mut crate::widget::WidgetRegistry, old_id: u64) {
    if let Some(old_frame) = widgets.get(old_id)
        && let Some(old_parent_id) = old_frame.parent_id
        && let Some(old_parent) = widgets.get_mut_visual(old_parent_id)
    {
        old_parent.children.retain(|&c| c != old_id);
    }
    if let Some(old_frame) = widgets.get_mut_visual(old_id) {
        old_frame.visible = false;
    }
}

/// Move all children from an old frame to a new replacement frame.
///
/// When a named frame is re-created (e.g. UIParent defined in XML replaces the
/// pre-built one), frames that were parented to the old version need to be
/// reparented to the new one so they remain in the live visibility tree.
pub(super) fn migrate_children_to_new_frame(
    widgets: &mut crate::widget::WidgetRegistry,
    old_id: u64,
    new_id: u64,
) {
    let children: Vec<u64> = widgets
        .get(old_id)
        .map(|f| f.children.clone())
        .unwrap_or_default();
    for &child_id in &children {
        if let Some(child) = widgets.get_mut_visual(child_id) {
            child.parent_id = Some(new_id);
        }
    }
    let keys: std::collections::HashMap<String, u64> = widgets
        .get(old_id)
        .map(|f| f.children_keys.clone())
        .unwrap_or_default();

    // Preserve the old frame's explicit size on the new frame if the new frame has no
    // size yet. This covers frames like UIParent which are pre-seeded with screen
    // dimensions before XML loads — the XML re-creates them with setAllPoints but no
    // explicit <Size>, so the new frame would start at 0x0 without this copy.
    let (old_width, old_height) = widgets
        .get(old_id)
        .map(|f| (f.width, f.height))
        .unwrap_or((0.0, 0.0));

    if let Some(new_frame) = widgets.get_mut_visual(new_id) {
        new_frame.children.extend(&children);
        for (k, v) in keys {
            new_frame.children_keys.entry(k).or_insert(v);
        }
        if new_frame.width == 0.0 && old_width > 0.0 {
            new_frame.width = old_width;
        }
        if new_frame.height == 0.0 && old_height > 0.0 {
            new_frame.height = old_height;
        }
    }
    if let Some(old_frame) = widgets.get_mut_visual(old_id) {
        old_frame.children.clear();
        old_frame.children_keys.clear();
    }
}

/// Check the template chain for a `parentArray` attribute and insert the frame
/// into its parent's Lua array if found.
pub(super) fn apply_parent_array_from_template(
    lua: &Lua,
    template_names: &str,
    _frame_id: u64,
    ref_name: &str,
) {
    let chain = crate::xml::get_template_chain(template_names);
    for entry in &chain {
        if let Some(parent_array) = &entry.frame.parent_array {
            let frame_ref = lua_global_ref(ref_name);
            let code = format!(
                "do local child = {frame_ref}\n\
                 if child then\n\
                     local parent = child:GetParent()\n\
                     if parent then\n\
                         parent[\"{parent_array}\"] = parent[\"{parent_array}\"] or {{}}\n\
                         table.insert(parent[\"{parent_array}\"], child)\n\
                     end\n\
                 end\nend",
            );
            let _ = lua.load(&code).exec();
            break;
        }
    }
}
