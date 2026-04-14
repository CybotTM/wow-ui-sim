//! Batched OnUpdate/OnPostUpdate dispatch.
//!
//! Passes all visible frame IDs to a single Lua call per suffix, avoiding
//! per-handler FFI round-trips and Lua table allocations. A Rust callback
//! pair keeps `executing_addon_index` and dirty-source attribution in sync so
//! that CreateFrame / C_Timer calls during handlers get correct addon ownership.

use super::state::SimState;
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

/// OnUpdate dispatch loop — pure Lua, called once per suffix with all IDs.
const ON_UPDATE_DISPATCH_LUA: &str = r#"
    local reg = debug.getregistry()
    local on_update_scripts = reg.__on_update_scripts
    local on_post_update_scripts = reg.__on_post_update_scripts
    local frames = reg.__frame_refs
    local owners = reg.__frame_owners
    local timing = reg.__addon_timing
    local addon_names = reg.__addon_names
    local setobjecttaint = debug.setobjecttaint
    local enter_context = reg.__enter_on_update_context
    local leave_context = reg.__leave_on_update_context
    local G = _G
    local stderr = io.stderr
    return function(ids, elapsed, suffix)
        local is_post_update = suffix == "_OnPostUpdate"
        local scripts = is_post_update and on_post_update_scripts or on_update_scripts
        local profile = debugprofilestop
        local handler = geterrorhandler()
        for i = 1, #ids do
            local id = ids[i]
            local func = scripts[id]
            if func then
                local frame = frames[id]
                if frame then
                    local owner = owners[id]
                    local taint = owner and addon_names[owner]
                    if taint then setobjecttaint(func, taint) end
                    enter_context(id, owner, is_post_update)
                    local t0 = profile()
                    local ok, err = pcall(func, frame, elapsed)
                    local dt = profile() - t0
                    leave_context()
                    if dt > 5 then
                        local n = frame.GetDebugName and frame:GetDebugName() or tostring(id)
                        local ts = (G.GetTimePreciseSec and G.GetTimePreciseSec())
                            or (G.GetTime and G.GetTime())
                            or os.clock()
                        local parent = frame.GetParent and frame:GetParent()
                        local pname = parent and parent.GetDebugName and parent:GetDebugName() or "?"
                        stderr:write(string.format("[%7.3fs] [OnUpdate] %7.1fms  %s%s id=%d parent=%s\n", ts, dt, n, suffix, id, pname))
                    end
                    if not ok then handler(err) end
                    if owner then
                        timing[owner] = (timing[owner] or 0) + dt
                    end
                end
            end
        end
    end
"#;

/// Register the Lua dispatch function and its Rust callback.
pub(crate) fn register(lua: &Lua, state: &Rc<RefCell<SimState>>) -> mlua::Result<()> {
    super::script_helpers::get_or_create_scripts_table(lua);

    let enter_state = Rc::clone(state);
    let enter_context = lua.create_function(
        move |_, (frame_id, addon_idx, is_post_update): (u64, Option<u16>, bool)| {
            let method = if is_post_update {
                "OnPostUpdate"
            } else {
                "OnUpdate"
            };
            let mut state = enter_state.borrow_mut();
            state.executing_addon_index = addon_idx;
            state
                .widgets
                .set_render_dirty_source(Some(crate::widget::RenderDirtySource {
                    frame_id,
                    method,
                }));
            Ok(())
        },
    )?;
    lua.set_named_registry_value("__enter_on_update_context", enter_context)?;

    let leave_state = Rc::clone(state);
    let leave_context = lua.create_function(move |_, ()| {
        let mut state = leave_state.borrow_mut();
        state.executing_addon_index = None;
        state.widgets.set_render_dirty_source(None);
        Ok(())
    })?;
    lua.set_named_registry_value("__leave_on_update_context", leave_context)?;

    let factory: mlua::Function = lua.load(ON_UPDATE_DISPATCH_LUA).into_function()?;
    let dispatch = factory.call::<mlua::Function>(())?;
    lua.set_named_registry_value("__dispatch_on_update", dispatch)?;

    // Pre-allocate a reusable table for passing frame IDs.
    lua.set_named_registry_value("__dispatch_ids", lua.create_table()?)?;
    lua.set_named_registry_value("__dispatch_ids_len", 0usize)?;

    Ok(())
}

/// Dispatch a batch of frame IDs through the Lua OnUpdate loop.
/// Reuses a single Lua table to avoid per-call GC pressure.
pub(crate) fn dispatch(lua: &Lua, frame_ids: &[u64], elapsed: f64, suffix: &str) {
    let dispatch: mlua::Function = lua
        .named_registry_value("__dispatch_on_update")
        .expect("__dispatch_on_update not registered");
    let ids_table: mlua::Table = lua
        .named_registry_value("__dispatch_ids")
        .expect("__dispatch_ids not registered");

    // Fill the reusable table with current IDs.
    for (i, id) in frame_ids.iter().enumerate() {
        ids_table.set(i + 1, *id as i64).unwrap();
    }
    // Trim leftover entries from a previous longer batch.
    let new_len = frame_ids.len();
    let previous_len = lua
        .named_registry_value::<usize>("__dispatch_ids_len")
        .unwrap_or(0);
    for i in (new_len + 1)..=previous_len {
        ids_table.set(i, mlua::Nil).unwrap();
    }
    lua.set_named_registry_value("__dispatch_ids_len", new_len)
        .unwrap();

    if let Err(e) = dispatch.call::<()>((&ids_table, elapsed, suffix)) {
        eprintln!("[OnUpdate] dispatch error: {e}");
    }
}

/// Fire OnUpdate + OnPostUpdate for visible frames, then tick animations.
///
/// GC is paused for the duration of handler dispatch so that per-handler timing
/// reflects only handler work, not interleaved GC sweeps.  A single GC step runs
/// at the end and its cost is logged separately.
pub(crate) fn fire(env: &super::env::WowLuaEnv, elapsed: f64) -> crate::Result<()> {
    let frame_ids = get_visible_on_update_frames(&env.state);

    if !frame_ids.is_empty() {
        env.lua.gc_stop();

        let t = Instant::now();
        dispatch(&env.lua, &frame_ids, elapsed, "_OnUpdate");
        let on_update_dur = t.elapsed();
        dispatch(&env.lua, &frame_ids, elapsed, "_OnPostUpdate");
        let handlers_dur = t.elapsed();

        let gc_start = Instant::now();
        env.lua.gc_restart();
        let _ = env.lua.gc_step();
        let gc_dur = gc_start.elapsed();

        let total = t.elapsed();
        if total.as_millis() > 20 {
            eprintln!(
                "{} [fire_on_update] {} handlers: OnUpdate={on_update_dur:.1?} handlers={handlers_dur:.1?} gc={gc_dur:.1?} total={total:.1?}",
                crate::logging::global_elapsed_prefix(),
                frame_ids.len()
            );
        }
    }

    super::animation::tick_animation_groups(&env.state, &env.lua, elapsed)?;
    env.finalize_frame_metrics(elapsed * 1000.0);
    Ok(())
}

fn get_visible_on_update_frames(state: &Rc<RefCell<SimState>>) -> Vec<u64> {
    let mut state = state.borrow_mut();
    if let Some(ref cached) = state.visible_on_update_cache {
        return cached.clone();
    }
    let ids: Vec<u64> = state
        .on_update_frames
        .iter()
        .copied()
        .filter(|&id| state.widgets.is_ancestor_visible(id))
        .collect();
    state.visible_on_update_cache = Some(ids.clone());
    ids
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;
    use crate::widget::{RenderDirtyBatch, RenderDirtySource};
    use std::collections::HashSet;

    #[test]
    fn on_update_dirty_batch_records_handler_source_frame() {
        let (driver_id, target_id, batch) = run_dirty_source_handler("OnUpdate");
        let sources = batch.sources.get(&target_id).cloned().unwrap_or_default();
        assert!(
            batch
                .frame_ids
                .as_ref()
                .is_some_and(|ids| ids.contains(&target_id)),
            "dirty target should be included in the dirty batch"
        );
        assert_eq!(
            sources,
            HashSet::from([RenderDirtySource {
                frame_id: driver_id,
                method: "OnUpdate",
            }])
        );
    }

    #[test]
    fn on_post_update_dirty_batch_records_handler_source_frame() {
        let (driver_id, target_id, batch) = run_dirty_source_handler("OnPostUpdate");
        let sources = batch.sources.get(&target_id).cloned().unwrap_or_default();
        assert!(
            batch
                .frame_ids
                .as_ref()
                .is_some_and(|ids| ids.contains(&target_id)),
            "dirty target should be included in the dirty batch"
        );
        assert_eq!(
            sources,
            HashSet::from([RenderDirtySource {
                frame_id: driver_id,
                method: "OnPostUpdate",
            }])
        );
    }

    fn run_dirty_source_handler(handler_name: &str) -> (u64, u64, RenderDirtyBatch) {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(&format!(
            r#"
            DirtyDriver = CreateFrame("Frame", "DirtyDriver", UIParent)
            DirtyTarget = CreateFrame("Frame", "DirtyTarget", UIParent)
            DirtyTarget:SetSize(10, 10)
            DirtyDriver:SetScript("{handler_name}", function(self, elapsed)
                DirtyTarget:SetWidth(25)
                self:SetScript("{handler_name}", nil)
            end)
            "#
        ))
        .unwrap();

        let (driver_id, target_id) = {
            let state = env.state().borrow();
            (
                state.widgets.get_id_by_name("DirtyDriver").unwrap(),
                state.widgets.get_id_by_name("DirtyTarget").unwrap(),
            )
        };

        {
            let state = env.state().borrow();
            let _ = state.widgets.take_render_dirty_batch();
        }

        env.fire_on_update(0.05).unwrap();

        let batch = env.state().borrow().widgets.take_render_dirty_batch();
        (driver_id, target_id, batch)
    }
}
