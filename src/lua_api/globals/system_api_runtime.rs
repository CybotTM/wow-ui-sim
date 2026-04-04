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
    register_cursor_position(lua)?;
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
        lua.create_function(|_, ()| Ok((0.0f64, 0.0f64, 0.0f64, 0.0f64)))?,
    )?;
    globals.set(
        "GetAvailableBandwidth",
        lua.create_function(|_, ()| Ok(0.0f64))?,
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
    let st = Rc::clone(state);
    globals.set(
        "SetScreenSize",
        lua.create_function(move |_, (w, h): (f32, f32)| {
            let mut s = st.borrow_mut();
            s.screen_width = w;
            s.screen_height = h;
            s.strata_buckets = None;
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
    let request_fn = lua.create_function(move |lua, ()| {
        let total_played = 15 * 24 * 3600;
        let level_played = 3 * 24 * 3600;

        let listeners =
            crate::lua_api::script_helpers::get_event_listeners_lua_order(lua, "TIME_PLAYED_MSG")?;

        for widget_id in listeners {
            if let Some(handler) =
                crate::lua_api::script_helpers::get_script(lua, widget_id, "OnEvent")
                && let Some(frame) = crate::lua_api::script_helpers::get_frame_ref(lua, widget_id)
            {
                let args = vec![
                    frame,
                    Value::String(lua.create_string("TIME_PLAYED_MSG")?),
                    Value::Integer(total_played),
                    Value::Integer(level_played),
                ];
                if let Err(e) = handler.call::<()>(mlua::MultiValue::from_vec(args)) {
                    crate::lua_api::script_helpers::call_error_handler(lua, &e.to_string());
                }
            }
        }

        Ok(())
    })?;
    lua.globals().set("RequestTimePlayed", request_fn)?;
    Ok(())
}

fn register_cursor_position(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "GetCursorPosition",
        lua.create_function(|_, ()| Ok((512.0_f64, 384.0_f64)))?,
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
    let globals = lua.globals();

    let animate_callout = lua.create_table()?;
    animate_callout.set(
        "Start",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    animate_callout.set(
        "Stop",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    globals.set("AnimateCallout", animate_callout)?;

    let wow_style1 = lua.create_table()?;
    wow_style1.set(
        "OnLoad",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    globals.set("WowStyle1DropdownMixin", wow_style1)?;

    let animate_mouse = lua.create_table()?;
    animate_mouse.set(
        "Start",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    animate_mouse.set(
        "Stop",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    globals.set("AnimateMouse", animate_mouse)?;

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

    if let Ok(c_unit_auras) = globals.get::<mlua::Table>("C_UnitAuras") {
        super::c_unit_auras_api::patch_c_unit_auras(lua, &c_unit_auras, state)?;
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
