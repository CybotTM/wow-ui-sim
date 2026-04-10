//! Batched OnUpdate/OnPostUpdate dispatch.
//!
//! Passes all visible frame IDs to a single Lua call per suffix, avoiding
//! per-handler FFI round-trips and Lua table allocations. A Rust callback
//! (`__set_executing_addon`) keeps `executing_addon_index` in sync so that
//! CreateFrame / C_Timer calls during handlers get correct addon ownership.

use super::state::SimState;
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

/// OnUpdate dispatch loop — pure Lua, called once per suffix with all IDs.
const ON_UPDATE_DISPATCH_LUA: &str = r#"
    local reg = debug.getregistry()
    local scripts = reg.__scripts
    local owners = reg.__frame_owners
    local timing = reg.__addon_timing
    local addon_names = reg.__addon_names
    local setobjecttaint = debug.setobjecttaint
    local set_addon = reg.__set_executing_addon
    local set_dirty_source = reg.__set_render_dirty_source
    local G = _G
    local stderr = io.stderr
    return function(ids, elapsed, suffix)
        local profile = debugprofilestop
        local handler = geterrorhandler()
        for i = 1, #ids do
            local id = ids[i]
            local func = scripts[id .. suffix]
            if func then
                local frame = rawget(G, "__frame_" .. id)
                if frame then
                    local owner = owners[id]
                    local taint = owner and addon_names[owner]
                    if taint then setobjecttaint(func, taint) end
                    set_addon(owner)
                    set_dirty_source(id, string.sub(suffix, 2))
                    local t0 = profile()
                    local ok, err = pcall(func, frame, elapsed)
                    local dt = profile() - t0
                    set_dirty_source(nil, nil)
                    set_addon(nil)
                    if dt > 5 then
                        local n = frame.GetDebugName and frame:GetDebugName() or tostring(id)
                        local ts = (G.GetTimePreciseSec and G.GetTimePreciseSec())
                            or (G.GetTime and G.GetTime())
                            or os.clock()
                        stderr:write(string.format("[%7.3fs] [OnUpdate] %7.1fms  %s%s\n", ts, dt, n, suffix))
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

    // Rust callback: Lua calls this to set executing_addon_index per handler.
    let st = Rc::clone(state);
    let set_addon = lua.create_function(move |_, idx: Option<u16>| {
        st.borrow_mut().executing_addon_index = idx;
        Ok(())
    })?;
    lua.set_named_registry_value("__set_executing_addon", set_addon)?;

    let dirty_state = Rc::clone(state);
    let set_dirty_source = lua.create_function(
        move |_, (frame_id, method): (Option<u64>, Option<String>)| {
            let source = match (frame_id, method) {
                (Some(frame_id), Some(method)) => {
                    Some(crate::widget::RenderDirtySource { frame_id, method })
                }
                _ => None,
            };
            dirty_state.borrow().widgets.set_render_dirty_source(source);
            Ok(())
        },
    )?;
    lua.set_named_registry_value("__set_render_dirty_source", set_dirty_source)?;

    let factory: mlua::Function = lua.load(ON_UPDATE_DISPATCH_LUA).into_function()?;
    let dispatch = factory.call::<mlua::Function>(())?;
    lua.set_named_registry_value("__dispatch_on_update", dispatch)?;

    // Pre-allocate a reusable table for passing frame IDs.
    lua.set_named_registry_value("__dispatch_ids", lua.create_table()?)?;

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
    let mut i = new_len + 1;
    while ids_table.get::<Option<i64>>(i).unwrap_or(None).is_some() {
        ids_table.set(i, mlua::Nil).unwrap();
        i += 1;
    }

    if let Err(e) = dispatch.call::<()>((&ids_table, elapsed, suffix)) {
        eprintln!("[OnUpdate] dispatch error: {e}");
    }
}

/// Fire OnUpdate + OnPostUpdate for visible frames, then tick animations.
pub(crate) fn fire(env: &super::env::WowLuaEnv, elapsed: f64) -> crate::Result<()> {
    let frame_ids = get_visible_on_update_frames(&env.state);

    if !frame_ids.is_empty() {
        let t = Instant::now();
        dispatch(&env.lua, &frame_ids, elapsed, "_OnUpdate");
        let on_update_dur = t.elapsed();
        dispatch(&env.lua, &frame_ids, elapsed, "_OnPostUpdate");
        let total = t.elapsed();
        if total.as_millis() > 20 {
            eprintln!(
                "{} [fire_on_update] {} handlers: OnUpdate={on_update_dur:.1?} total={total:.1?}",
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
    use crate::widget::RenderDirtySource;
    use std::collections::HashSet;

    #[test]
    fn on_update_dirty_batch_records_handler_source_frame() {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            DirtyDriver = CreateFrame("Frame", "DirtyDriver", UIParent)
            DirtyTarget = CreateFrame("Frame", "DirtyTarget", UIParent)
            DirtyTarget:SetSize(10, 10)
            DirtyDriver:SetScript("OnUpdate", function(self, elapsed)
                DirtyTarget:SetWidth(25)
                self:SetScript("OnUpdate", nil)
            end)
            "#,
        )
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
                method: "OnUpdate".to_string(),
            }])
        );
    }
}
