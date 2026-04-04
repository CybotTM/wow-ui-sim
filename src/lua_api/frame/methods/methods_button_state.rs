use super::super::handle::FrameRef;
use super::methods_button::button_texture_should_show;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
use mlua::Value;

pub(super) fn add_button_state_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_enable_disable_methods(methods);
    add_click_methods(methods);
    add_state_methods(methods);
}

fn add_enable_disable_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_enabled_mutator_method(methods, "SetEnabled", None);
    add_enabled_mutator_method(methods, "Enable", Some(true));
    add_enabled_mutator_method(methods, "Disable", Some(false));
    add_is_enabled_method(methods);
}

fn add_enabled_mutator_method<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &str,
    explicit_value: Option<bool>,
) {
    if let Some(enabled) = explicit_value {
        methods.add_method(name, move |lua, this, (): ()| {
            let state_rc = get_sim_state(lua);
            set_enabled_attribute(&mut state_rc.borrow_mut(), this.0, enabled);
            Ok(())
        });
        return;
    }

    methods.add_method(name, |lua, this, enabled: bool| {
        let state_rc = get_sim_state(lua);
        set_enabled_attribute(&mut state_rc.borrow_mut(), this.0, enabled);
        Ok(())
    });
}

fn add_is_enabled_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsEnabled", |lua, this, ()| {
        let id = this.0;
        if let Some((func, ud_val)) =
            super::methods_helpers::get_mixin_override(lua, id, "IsEnabled")
        {
            return func.call::<Value>(ud_val);
        }
        Ok(Value::Boolean(read_enabled_flag(lua, id)))
    });
}

fn read_enabled_flag(lua: &mlua::Lua, id: u64) -> bool {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .and_then(|f| f.attributes.get("__enabled"))
        .and_then(|v| {
            if let crate::widget::AttributeValue::Boolean(b) = v {
                Some(*b)
            } else {
                None
            }
        })
        .unwrap_or(true)
}

fn set_enabled_attribute(state: &mut crate::lua_api::SimState, id: u64, enabled: bool) {
    if let Some(frame) = state.widgets.get(id) {
        if let Some(crate::widget::AttributeValue::Boolean(cur)) = frame.attributes.get("__enabled")
        {
            if *cur == enabled {
                return;
            }
        }
    }
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.attributes.insert(
            "__enabled".to_string(),
            crate::widget::AttributeValue::Boolean(enabled),
        );
        if !enabled {
            frame.button_state = 0;
        }
    }
    update_button_texture_visibility(state, id);
}

fn update_button_texture_visibility(state: &mut crate::lua_api::SimState, button_id: u64) {
    let keys: Vec<(String, u64)> = state
        .widgets
        .get(button_id)
        .map(|f| {
            [
                "NormalTexture",
                "PushedTexture",
                "DisabledTexture",
                "HighlightTexture",
            ]
            .iter()
            .filter_map(|k| f.children_keys.get(*k).map(|&id| (k.to_string(), id)))
            .collect()
        })
        .unwrap_or_default();
    for (key, tex_id) in keys {
        let should_show = button_texture_should_show(state, button_id, &key);
        state.widgets.set_visible(tex_id, should_show);
    }
}

fn add_click_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("Click", |lua, this, ()| {
        let id = this.0;
        if let Some(handler) = crate::lua_api::script_helpers::get_script(lua, id, "OnClick") {
            let frame_val = frame_ref(lua, id)?;
            let button = lua.create_string("LeftButton")?;
            handler.call::<()>((frame_val, button, false))?;
        }
        Ok(())
    });

    methods.add_method("RegisterForClicks", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
}

fn add_state_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_button_state(methods);
    add_get_button_state(methods);
    methods.add_method("LockHighlight", |_, _this, ()| Ok(()));
    methods.add_method("UnlockHighlight", |_, _this, ()| Ok(()));
}

fn add_set_button_state<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetButtonState",
        |lua, this, (state_str, _locked): (String, Option<bool>)| {
            let id = this.0;
            let state_rc = get_sim_state(lua);
            if state_str.eq_ignore_ascii_case("PUSHED") {
                let mut state = state_rc.borrow_mut();
                if let Some(f) = state.widgets.get_mut_visual(id) {
                    f.button_state = 1;
                }
                set_enabled_attribute(&mut state, id, true);
                update_button_texture_visibility(&mut state, id);
            } else if state_str.eq_ignore_ascii_case("NORMAL") {
                let mut state = state_rc.borrow_mut();
                if let Some(f) = state.widgets.get_mut_visual(id) {
                    f.button_state = 0;
                }
                set_enabled_attribute(&mut state, id, true);
                update_button_texture_visibility(&mut state, id);
            } else if state_str.eq_ignore_ascii_case("DISABLED") {
                let mut state = state_rc.borrow_mut();
                set_enabled_attribute(&mut state, id, false);
            } else {
                return Err(mlua::Error::runtime(format!(
                    "Usage: Button:SetButtonState(\"state\"): Unknown button state ({})",
                    state_str
                )));
            }
            Ok(())
        },
    );
}

fn add_get_button_state<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetButtonState", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(f) = state.widgets.get(id) {
            let enabled = f
                .attributes
                .get("__enabled")
                .and_then(|v| match v {
                    crate::widget::AttributeValue::Boolean(b) => Some(*b),
                    _ => None,
                })
                .unwrap_or(true);
            if !enabled {
                return Ok("DISABLED".to_string());
            }
            return Ok(if f.button_state == 1 {
                "PUSHED"
            } else {
                "NORMAL"
            }
            .to_string());
        }
        Ok("NORMAL".to_string())
    });
}
