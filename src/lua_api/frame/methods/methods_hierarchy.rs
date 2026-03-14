//! Hierarchy methods: GetParent, SetParent, GetNumChildren, GetChildren, GetRegions.

use super::super::handle::{FrameRef, extract_frame_id, frame_ref};
use crate::lua_api::frame::handle::get_sim_state;
use crate::widget::{FrameStrata, WidgetRegistry};
use mlua::Value;

/// Add hierarchy methods: parent access, children, regions.
pub fn add_hierarchy_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_parent_methods(methods);
    add_parent_key_methods(methods);
    add_children_frame_methods(methods);
    add_children_region_methods(methods);
}

fn add_parent_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetParent", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(id)
            && let Some(parent_id) = frame.parent_id
        {
            drop(state);
            return frame_ref(lua, parent_id);
        }
        Ok(Value::Nil)
    });

    methods.add_method("SetParent", |lua, this, parent: Value| {
        let id = this.0;
        let new_parent_id = extract_frame_id(&parent);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        reparent_widget(&mut state.widgets, id, new_parent_id);
        // Explicit SetParent call clears the default_parent flag — the parent
        // is now explicitly known, so SetAllPoints() should anchor to it.
        if let Some(f) = state.widgets.get_mut_visual(id) {
            f.default_parent = false;
        }
        state.visible_on_update_cache = None;
        state.widgets.mark_rect_dirty(id);
        Ok(())
    });
}

/// Move a widget to a new parent, updating children lists and inheriting strata/level.
pub fn reparent_widget(widgets: &mut WidgetRegistry, child_id: u64, new_parent_id: Option<u64>) {
    let old_parent_id = widgets.get(child_id).and_then(|f| f.parent_id);
    let same_parent = old_parent_id.is_some() && old_parent_id == new_parent_id;

    if !same_parent {
        detach_from_old_parent(widgets, child_id, old_parent_id);
    }

    let parent_props = read_parent_props(widgets, new_parent_id);
    update_child_parent_link(widgets, child_id, new_parent_id, same_parent, parent_props);
    propagate_strata_level(widgets, child_id);

    let parent_eff_alpha = parent_props.map(|(_, _, a, _)| a).unwrap_or(1.0);
    let parent_eff_scale = parent_props.map(|(_, _, _, s)| s).unwrap_or(1.0);
    widgets.propagate_effective_alpha(child_id, parent_eff_alpha);
    widgets.propagate_effective_scale(child_id, parent_eff_scale);

    if !same_parent {
        attach_to_new_parent(widgets, child_id, new_parent_id);
    }
}

/// Remove child from old parent's children list and clear parent_key.
fn detach_from_old_parent(widgets: &mut WidgetRegistry, child_id: u64, old_parent_id: Option<u64>) {
    if let Some(old_pid) = old_parent_id
        && let Some(old_parent) = widgets.get_mut_visual(old_pid)
    {
        old_parent.children.retain(|&id| id != child_id);
        old_parent.children_keys.retain(|_, &mut v| v != child_id);
    }
    if let Some(child) = widgets.get_mut_visual(child_id) {
        child.parent_key = None;
    }
}

/// Read (strata, level, eff_alpha, eff_scale) from the new parent, if any.
fn read_parent_props(
    widgets: &WidgetRegistry,
    new_parent_id: Option<u64>,
) -> Option<(FrameStrata, i32, f32, f32)> {
    new_parent_id.and_then(|pid| {
        widgets.get(pid).map(|p| {
            (
                p.frame_strata,
                p.frame_level,
                p.effective_alpha,
                p.effective_scale,
            )
        })
    })
}

/// Set child's parent_id and inherit strata/level when moving to a new parent.
fn update_child_parent_link(
    widgets: &mut WidgetRegistry,
    child_id: u64,
    new_parent_id: Option<u64>,
    same_parent: bool,
    parent_props: Option<(FrameStrata, i32, f32, f32)>,
) {
    let Some(frame) = widgets.get_mut_visual(child_id) else {
        return;
    };
    frame.parent_id = new_parent_id;
    if same_parent {
        return;
    }
    if let Some((parent_strata, parent_level, _, _)) = parent_props {
        if !frame.has_fixed_frame_strata {
            frame.frame_strata = parent_strata;
        }
        if !frame.has_fixed_frame_level {
            let offset = frame.frame_level_offset.unwrap_or(1);
            frame.frame_level = parent_level + offset;
        }
    }
}

/// Add child to new parent's children list if not already present.
fn attach_to_new_parent(widgets: &mut WidgetRegistry, child_id: u64, new_parent_id: Option<u64>) {
    if let Some(new_pid) = new_parent_id
        && let Some(new_parent) = widgets.get_mut_visual(new_pid)
        && !new_parent.children.contains(&child_id)
    {
        new_parent.children.push(child_id);
    }
}

fn add_parent_key_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_parent_key(methods);
    add_get_parent_key(methods);
}

fn add_set_parent_key<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetParentKey",
        |lua, this, (key, remove_old): (String, Option<bool>)| {
            let id = this.0;
            let state_rc = get_sim_state(lua);
            let parent_id = state_rc.borrow().widgets.get(id).and_then(|f| f.parent_id);
            let Some(pid) = parent_id else { return Ok(()) };

            if remove_old.unwrap_or(false) {
                let old_keys: Vec<String> = {
                    let state = state_rc.borrow();
                    state
                        .widgets
                        .get(pid)
                        .map(|p| {
                            p.children_keys
                                .iter()
                                .filter(|&(_, &cid)| cid == id)
                                .map(|(k, _)| k.clone())
                                .collect()
                        })
                        .unwrap_or_default()
                };
                let assign_fn: mlua::Function = lua.named_registry_value("__frame_assign_fn")?;
                let parent_ref = frame_ref(lua, pid)?;
                for old_key in old_keys {
                    assign_fn.call::<()>((parent_ref.clone(), old_key, Value::Nil))?;
                }
            }

            let assign_fn: mlua::Function = lua.named_registry_value("__frame_assign_fn")?;
            let parent_ref = frame_ref(lua, pid)?;
            let child_ref = frame_ref(lua, id)?;
            assign_fn.call::<()>((parent_ref, key, child_ref))?;
            Ok(())
        },
    );
}

fn add_get_parent_key<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetParentKey", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0)
            && let Some(ref key) = frame.parent_key
        {
            return Ok(Value::String(lua.create_string(key.as_bytes())?));
        }
        Ok(Value::Nil)
    });
}

fn add_children_frame_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetNumChildren", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let count = state
            .widgets
            .get(this.0)
            .map(|f| f.children.len())
            .unwrap_or(0);
        Ok(count as i32)
    });

    methods.add_method("GetChildren", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let children = {
            let state = state_rc.borrow();
            state
                .widgets
                .get(id)
                .map(|f| f.children.clone())
                .unwrap_or_default()
        };
        let mut result = mlua::MultiValue::new();
        for child_id in children {
            result.push_back(frame_ref(lua, child_id)?);
        }
        Ok(result)
    });
}

fn add_children_region_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_num_regions(methods);
    add_get_regions(methods);
    methods.add_method("GetAdditionalRegions", |_lua, _this, ()| {
        Ok(mlua::MultiValue::new())
    });
}

fn add_get_num_regions<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetNumRegions", |lua, this, ()| {
        use crate::widget::WidgetType;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let count = state
            .widgets
            .get(this.0)
            .map(|f| {
                f.children
                    .iter()
                    .filter(|&&cid| {
                        state
                            .widgets
                            .get(cid)
                            .map(|c| {
                                matches!(
                                    c.widget_type,
                                    WidgetType::Texture | WidgetType::FontString | WidgetType::Line
                                )
                            })
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0);
        Ok(count as i32)
    });
}

fn add_get_regions<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetRegions", |lua, this, ()| {
        use crate::widget::WidgetType;
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let children = {
            let state = state_rc.borrow();
            state
                .widgets
                .get(id)
                .map(|f| f.children.clone())
                .unwrap_or_default()
        };
        let mut result = mlua::MultiValue::new();
        for child_id in children {
            let is_region = {
                let state = state_rc.borrow();
                state
                    .widgets
                    .get(child_id)
                    .map(|f| {
                        matches!(
                            f.widget_type,
                            WidgetType::Texture | WidgetType::FontString | WidgetType::Line
                        )
                    })
                    .unwrap_or(false)
            };
            if is_region {
                result.push_back(frame_ref(lua, child_id)?);
            }
        }
        Ok(result)
    });
}

/// Public wrapper for propagation, used by SetFrameLevel in methods_core.
pub fn propagate_strata_level_pub(widgets: &mut WidgetRegistry, root_id: u64) {
    propagate_strata_level(widgets, root_id);
}

/// BFS propagation of frame_strata and frame_level to all descendants.
/// Each child inherits parent_strata (unless has_fixed_frame_strata) and
/// parent_level + 1 (unless has_fixed_frame_level).
fn propagate_strata_level(widgets: &mut WidgetRegistry, root_id: u64) {
    let Some(root) = widgets.get(root_id) else {
        return;
    };
    let root_strata = root.frame_strata;
    let root_level = root.frame_level;
    let mut queue: Vec<(u64, FrameStrata, i32)> = root
        .children
        .iter()
        .map(|&id| (id, root_strata, root_level))
        .collect();

    while let Some((child_id, parent_strata, parent_level)) = queue.pop() {
        let Some(child) = widgets.get_mut_visual(child_id) else {
            continue;
        };
        if !child.has_fixed_frame_strata {
            child.frame_strata = parent_strata;
        }
        if !child.has_fixed_frame_level {
            let offset = child.frame_level_offset.unwrap_or(1);
            child.frame_level = parent_level + offset;
        }
        let child_strata = child.frame_strata;
        let child_level = child.frame_level;
        let children = child.children.clone();
        for &grandchild_id in &children {
            queue.push((grandchild_id, child_strata, child_level));
        }
    }
}
