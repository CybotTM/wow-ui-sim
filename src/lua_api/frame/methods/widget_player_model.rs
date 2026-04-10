//! PlayerModel widget methods.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::get_sim_state;

pub fn add_player_model_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_player_model_stubs(methods);
}

fn add_player_model_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "ApplySpellVisualKit",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("CanSetUnit", |_, _this, ()| Ok(true));
    methods.add_method(
        "FreezeAnimation",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("GetDisplayInfo", |lua, this, ()| {
        Ok(read_player_model_frame(lua, this.0, |frame| {
            frame.model_appearance.display_info.unwrap_or(0) as i64
        })
        .unwrap_or(0))
    });
    methods.add_method("GetDoBlend", |lua, this, ()| {
        Ok(
            read_player_model_frame(lua, this.0, |frame| frame.player_model_state.do_blend)
                .unwrap_or(false),
        )
    });
    methods.add_method("GetKeepModelOnHide", |lua, this, ()| {
        Ok(read_player_model_frame(lua, this.0, |frame| {
            frame.player_model_state.keep_model_on_hide
        })
        .unwrap_or(false))
    });
    methods.add_method("HasAnimation", |lua, this, ()| {
        Ok(read_player_model_frame(lua, this.0, |frame| {
            frame.model_appearance.animation_id.is_some()
        })
        .unwrap_or(false))
    });
    methods.add_method("PlayAnimKit", |lua, this, args: mlua::MultiValue| {
        update_player_model_frame(lua, this.0, |frame| {
            frame.player_model_state.active_anim_kit = parse_first_player_model_i32(&args);
        });
        Ok(())
    });
    methods.add_method(
        "SetBarberShopAlternateForm",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("SetDoBlend", |lua, this, args: mlua::MultiValue| {
        update_player_model_frame(lua, this.0, |frame| {
            frame.player_model_state.do_blend = parse_first_player_model_bool(&args);
        });
        Ok(())
    });
    methods.add_method("SetItem", |lua, this, args: mlua::MultiValue| {
        update_player_model_frame(lua, this.0, |frame| {
            frame.player_model_state.last_item = parse_first_player_model_string(&args);
        });
        Ok(())
    });
    methods.add_method("SetItemAppearance", |lua, this, args: mlua::MultiValue| {
        update_player_model_frame(lua, this.0, |frame| {
            frame.player_model_state.last_item_appearance = parse_first_player_model_string(&args);
        });
        Ok(())
    });
    methods.add_method("SetKeepModelOnHide", |lua, this, args: mlua::MultiValue| {
        update_player_model_frame(lua, this.0, |frame| {
            frame.player_model_state.keep_model_on_hide = parse_first_player_model_bool(&args);
        });
        Ok(())
    });
    methods.add_method("StopAnimKit", |lua, this, _args: mlua::MultiValue| {
        update_player_model_frame(lua, this.0, |frame| {
            frame.player_model_state.active_anim_kit = None;
        });
        Ok(())
    });
    methods.add_method("ZeroCachedCenterXY", |_, _this, _args: mlua::MultiValue| {
        Ok(())
    });
}

fn update_player_model_frame(
    lua: &mlua::Lua,
    id: u64,
    update: impl FnOnce(&mut crate::widget::Frame),
) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        update(frame);
    }
}

fn read_player_model_frame<T>(
    lua: &mlua::Lua,
    id: u64,
    read: impl FnOnce(&crate::widget::Frame) -> T,
) -> Option<T> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state.widgets.get(id).map(read)
}

fn parse_first_player_model_bool(args: &mlua::MultiValue) -> bool {
    args.front().map(lua_value_to_bool).unwrap_or(false)
}

fn parse_first_player_model_i32(args: &mlua::MultiValue) -> Option<i32> {
    args.front().map(lua_value_to_i32)
}

fn parse_first_player_model_string(args: &mlua::MultiValue) -> Option<String> {
    args.front().map(lua_value_to_string)
}

fn lua_value_to_bool(value: &mlua::Value) -> bool {
    match value {
        mlua::Value::Boolean(flag) => *flag,
        mlua::Value::Number(n) => *n != 0.0,
        mlua::Value::Integer(n) => *n != 0,
        _ => false,
    }
}

fn lua_value_to_i32(value: &mlua::Value) -> i32 {
    match value {
        mlua::Value::Number(n) => *n as i32,
        mlua::Value::Integer(n) => *n as i32,
        _ => 0,
    }
}

fn lua_value_to_string(value: &mlua::Value) -> String {
    match value {
        mlua::Value::String(text) => text.to_string_lossy(),
        mlua::Value::Number(n) => n.to_string(),
        mlua::Value::Integer(n) => n.to_string(),
        mlua::Value::Boolean(flag) => flag.to_string(),
        _ => String::new(),
    }
}
