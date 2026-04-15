use crate::lua_api::SimState;
use crate::lua_api::frame::frame_ref;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn register_runtime_system_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_time_functions(lua, &state)?;
    register_streaming_stubs(lua)?;
    register_error_callstack_stubs(lua)?;
    register_network_stubs(lua, &state)?;
    register_input_state_stubs(lua, &state)?;
    register_screen_size_functions(lua, &state)?;
    register_request_time_played(lua)?;
    register_cursor_position(lua, &state)?;
    register_localization_stubs(lua)?;
    register_ui_object_stubs(lua, state)?;
    register_ui_parent_stubs(lua)?;
    Ok(())
}

/// Register `GetTime()` and related timing globals.
fn register_time_functions(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let st = Rc::clone(state);
    let get_time =
        lua.create_function(move |_, ()| Ok(st.borrow().start_time.elapsed().as_secs_f64()))?;
    lua.globals().set("GetTime", get_time)?;

    let st = Rc::clone(state);
    lua.globals().set(
        "debugprofilestop",
        lua.create_function(move |_, ()| {
            Ok(st.borrow().start_time.elapsed().as_secs_f64() * 1000.0)
        })?,
    )?;
    lua.globals()
        .set("debugprofilestart", lua.create_function(|_, ()| Ok(()))?)?;

    let st = Rc::clone(state);
    lua.globals().set(
        "GetTimePreciseSec",
        lua.create_function(move |_, ()| Ok(st.borrow().start_time.elapsed().as_secs_f64()))?,
    )?;

    Ok(())
}

fn register_streaming_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetFileStreamingStatus",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    globals.set(
        "GetBackgroundLoadingStatus",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    globals.set(
        "GetMovieDownloadProgress",
        lua.create_function(|_, _movie_id: i32| Ok((false, 0i32, 0i32)))?,
    )?;
    Ok(())
}

fn register_error_callstack_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set("GetCallstackHeight", lua.create_function(|_, ()| Ok(2i32))?)?;
    globals.set(
        "SetErrorCallstackHeight",
        lua.create_function(|_, _height: Option<i32>| Ok(()))?,
    )?;
    Ok(())
}

fn register_network_stubs(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetNetStats",
        lua.create_function(|_, ()| Ok(seeded_net_stats()))?,
    )?;
    globals.set(
        "GetAvailableBandwidth",
        lua.create_function(|_, ()| Ok(seeded_available_bandwidth()))?,
    )?;
    globals.set(
        "GetDownloadedPercentage",
        lua.create_function(|_, ()| Ok(1.0f64))?,
    )?;
    let st = Rc::clone(state);
    globals.set(
        "GetFramerate",
        lua.create_function(move |_, ()| Ok(st.borrow().fps as f64))?,
    )?;
    Ok(())
}

fn seeded_net_stats() -> (f64, f64, f64, f64) {
    (512.0, 128.0, 28.0, 34.0)
}

fn seeded_available_bandwidth() -> f64 {
    128.0
}

fn register_input_state_stubs(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    register_keyboard_stubs(lua)?;
    register_mouse_state_stubs(lua, state)
}

fn register_keyboard_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set("IsShiftKeyDown", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsControlKeyDown", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsAltKeyDown", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsModifierKeyDown", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set(
        "IsModifiedClick",
        lua.create_function(|_, _action: Option<String>| Ok(false))?,
    )?;
    globals.set(
        "IsKeyDown",
        lua.create_function(|_, _key: String| Ok(false))?,
    )?;
    Ok(())
}

fn register_mouse_state_stubs(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "IsMouseButtonDown",
        lua.create_function(|_, _btn: Option<Value>| Ok(false))?,
    )?;
    let st = Rc::clone(state);
    globals.set(
        "GetMouseFocus",
        lua.create_function(move |lua, ()| {
            let hovered = st.borrow().hovered_frame;
            match hovered {
                Some(id) => frame_ref(lua, id),
                None => Ok(Value::Nil),
            }
        })?,
    )?;
    register_mouse_foci(lua, state)?;
    globals.set(
        "GetMouseButtonClicked",
        lua.create_function(|_, ()| Ok(""))?,
    )?;
    Ok(())
}

fn register_mouse_foci(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let st = Rc::clone(state);
    lua.globals().set(
        "GetMouseFoci",
        lua.create_function(move |lua, ()| {
            let tbl = lua.create_table()?;
            let hovered = st.borrow().hovered_frame;
            if let Some(id) = hovered {
                tbl.raw_set(1, frame_ref(lua, id)?)?;
            }
            Ok(tbl)
        })?,
    )
}

fn register_screen_size_functions(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    register_screen_size_getters(lua, state)?;
    register_set_screen_size(lua, state)
}

fn register_screen_size_getters(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    let st = Rc::clone(state);
    globals.set(
        "GetScreenWidth",
        lua.create_function(move |_, ()| Ok(st.borrow().screen_width as f64))?,
    )?;
    let st = Rc::clone(state);
    globals.set(
        "GetScreenHeight",
        lua.create_function(move |_, ()| Ok(st.borrow().screen_height as f64))?,
    )?;
    let st = Rc::clone(state);
    globals.set(
        "GetPhysicalScreenSize",
        lua.create_function(move |_, ()| {
            let s = st.borrow();
            Ok((s.screen_width as i32, s.screen_height as i32))
        })?,
    )?;
    Ok(())
}

fn register_set_screen_size(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let st = Rc::clone(state);
    lua.globals().set(
        "SetScreenSize",
        lua.create_function(move |_, (w, h): (f32, f32)| {
            let mut s = st.borrow_mut();
            s.screen_width = w;
            s.screen_height = h;
            s.invalidate_strata_buckets();
            s.widgets.clear_all_layout_rects();
            for name in ["UIParent", "WorldFrame"] {
                if let Some(id) = s.widgets.get_id_by_name(name)
                    && let Some(f) = s.widgets.get_mut_visual(id)
                {
                    f.width = w;
                    f.height = h;
                }
            }
            Ok(())
        })?,
    )?;
    Ok(())
}

fn register_request_time_played(lua: &Lua) -> Result<()> {
    const TIME_PLAYED_EVENT: &str = "TIME_PLAYED_MSG";
    const TOTAL_PLAYED_SECONDS: i64 = 15 * 24 * 3600;
    const LEVEL_PLAYED_SECONDS: i64 = 3 * 24 * 3600;

    let request_fn = lua.create_function(move |lua, ()| {
        for widget_id in time_played_listeners(lua, TIME_PLAYED_EVENT)? {
            dispatch_time_played_listener(
                lua,
                widget_id,
                TIME_PLAYED_EVENT,
                TOTAL_PLAYED_SECONDS,
                LEVEL_PLAYED_SECONDS,
            )?;
        }

        Ok(())
    })?;
    lua.globals().set("RequestTimePlayed", request_fn)?;
    Ok(())
}

fn time_played_listeners(lua: &Lua, event_name: &str) -> Result<Vec<u64>> {
    crate::lua_api::script_helpers::get_event_listeners_lua_order(lua, event_name)
}

fn dispatch_time_played_listener(
    lua: &Lua,
    widget_id: u64,
    event_name: &str,
    total_played: i64,
    level_played: i64,
) -> Result<()> {
    let Some(frame) = crate::lua_api::script_helpers::get_frame_ref(lua, widget_id) else {
        return Ok(());
    };
    dispatch_time_played_on_event(
        lua,
        widget_id,
        &frame,
        event_name,
        total_played,
        level_played,
    )?;
    dispatch_time_played_callbacks(
        lua,
        widget_id,
        frame,
        event_name,
        total_played,
        level_played,
    )
}

fn dispatch_time_played_on_event(
    lua: &Lua,
    widget_id: u64,
    frame: &Value,
    event_name: &str,
    total_played: i64,
    level_played: i64,
) -> Result<()> {
    let Some(handler) = crate::lua_api::script_helpers::get_script(lua, widget_id, "OnEvent")
    else {
        return Ok(());
    };
    let args =
        time_played_on_event_args(lua, frame.clone(), event_name, total_played, level_played)?;
    if let Err(error) = handler.call::<()>(args) {
        crate::lua_api::script_helpers::call_error_handler(lua, &error.to_string());
    }
    Ok(())
}

fn time_played_on_event_args(
    lua: &Lua,
    frame: Value,
    event_name: &str,
    total_played: i64,
    level_played: i64,
) -> Result<mlua::MultiValue> {
    Ok(mlua::MultiValue::from_vec(vec![
        frame,
        Value::String(lua.create_string(event_name)?),
        Value::Integer(total_played),
        Value::Integer(level_played),
    ]))
}

fn dispatch_time_played_callbacks(
    lua: &Lua,
    widget_id: u64,
    frame: Value,
    event_name: &str,
    total_played: i64,
    level_played: i64,
) -> Result<()> {
    crate::lua_api::script_helpers::dispatch_frame_unit_event_callbacks(
        lua,
        widget_id,
        frame,
        &[Value::Integer(total_played), Value::Integer(level_played)],
        event_name,
    )
}

fn register_cursor_position(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let st = Rc::clone(state);
    lua.globals().set(
        "GetCursorPosition",
        lua.create_function(move |_, ()| {
            let (x, y) = st.borrow().mouse_position.unwrap_or((512.0, 384.0));
            Ok((x as f64, y as f64))
        })?,
    )?;
    Ok(())
}

fn register_localization_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetText",
        lua.create_function(|lua, (key, gender): (String, Option<i32>)| {
            let g = lua.globals();
            let suffix = match gender {
                Some(2) => Some("_FEMALE"),
                Some(3) => Some("_NEUTRAL"),
                _ => None,
            };
            if let Some(s) = suffix
                && let Ok(val) = g.get::<String>(format!("{key}{s}"))
            {
                return Ok(val);
            }
            Ok(g.get::<String>(key.clone()).unwrap_or(key))
        })?,
    )?;
    Ok(())
}

fn register_ui_object_stubs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_mixin_stub_tables(lua)?;
    patch_namespace_stubs(lua)?;
    let globals = lua.globals();
    if let Ok(c_unit_auras) = globals.get::<mlua::Table>("C_UnitAuras") {
        super::c_unit_auras_api::patch_c_unit_auras(lua, &c_unit_auras, state)?;
    }
    Ok(())
}

/// Stub mixin tables: AnimateCallout, WowStyle1DropdownMixin, AnimateMouse.
fn register_mixin_stub_tables(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let noop = |lua: &Lua| lua.create_function(|_, _args: mlua::MultiValue| Ok(()));

    let animate_callout = lua.create_table()?;
    animate_callout.set("Start", noop(lua)?)?;
    animate_callout.set("Stop", noop(lua)?)?;
    globals.set("AnimateCallout", animate_callout)?;

    let wow_style1 = lua.create_table()?;
    wow_style1.set("OnLoad", noop(lua)?)?;
    globals.set("WowStyle1DropdownMixin", wow_style1)?;

    let animate_mouse = lua.create_table()?;
    animate_mouse.set("Start", noop(lua)?)?;
    animate_mouse.set("Stop", noop(lua)?)?;
    globals.set("AnimateMouse", animate_mouse)?;
    Ok(())
}

/// Patch existing C_PlayerInfo and C_UIWidgetManager namespaces with missing methods.
fn patch_namespace_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    if let Ok(c_player_info) = globals.get::<mlua::Table>("C_PlayerInfo") {
        c_player_info.set("IsPlayerInRPE", lua.create_function(|_, ()| Ok(false))?)?;
        c_player_info.set(
            "GetAlternateFormInfo",
            lua.create_function(|_, ()| Ok((false, false)))?,
        )?;
    }
    if let Ok(c_widget_mgr) = globals.get::<mlua::Table>("C_UIWidgetManager") {
        c_widget_mgr.set(
            "GetPowerBarWidgetSetID",
            lua.create_function(|_, ()| Ok(0i32))?,
        )?;
    }
    Ok(())
}

fn register_ui_parent_stubs(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "UpdateUIParentPosition",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    const TIME_PLAYED_EVENT: &str = "TIME_PLAYED_MSG";
    const TOTAL_PLAYED_SECONDS: i64 = 15 * 24 * 3600;
    const LEVEL_PLAYED_SECONDS: i64 = 3 * 24 * 3600;

    fn env() -> WowLuaEnv {
        WowLuaEnv::new().expect("failed to create Lua environment")
    }

    #[test]
    fn request_time_played_dispatches_on_event_and_callbacks() {
        let env = env();
        env.exec(
            r#"
            observed = {}
            local frame = CreateFrame("Frame")
            frame:RegisterEvent("TIME_PLAYED_MSG")
            frame:SetScript("OnEvent", function(self, event, total, level)
                observed.on_event = { event, total, level, self == frame }
            end)
            frame:RegisterUnitEventCallback("TIME_PLAYED_MSG", function(self, total, level)
                observed.callback = { total, level, self == frame }
            end)
            RequestTimePlayed()
            "#,
        )
        .expect("request time played script should run");

        let observed: (String, i64, i64, bool, i64, i64, bool) = env
            .eval(
                r#"
                return observed.on_event[1], observed.on_event[2], observed.on_event[3], observed.on_event[4],
                    observed.callback[1], observed.callback[2], observed.callback[3]
                "#,
            )
            .expect("request time played handlers should populate observations");

        assert_eq!(
            observed,
            (
                TIME_PLAYED_EVENT.to_string(),
                TOTAL_PLAYED_SECONDS,
                LEVEL_PLAYED_SECONDS,
                true,
                TOTAL_PLAYED_SECONDS,
                LEVEL_PLAYED_SECONDS,
                true,
            )
        );
    }
}
