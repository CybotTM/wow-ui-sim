//! Passive A1 query surface. No supplied transform state or motion is modeled.
//! Neutral return values are scaffold policy, not native defaults.
use crate::lua_api::animation::AnimationType;
use crate::lua_api::methods::create_string;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::closure::RustFn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

pub(super) fn attach_transform_methods(
    state: &mut LuaState,
    frame: Val,
    anim_type: AnimationType,
) -> LuaResult<()> {
    // Existing HEAD setters remain unchanged, including inherited SetScale.
    let methods: &[(&str, RustFn)] = match anim_type {
        AnimationType::Translation => &[("GetOffset", get_zero_pair)],
        AnimationType::Scale => &[
            ("GetScale", get_identity_pair),
            ("GetScaleFrom", get_identity_pair),
            ("GetScaleTo", get_identity_pair),
            ("GetOrigin", get_origin),
        ],
        AnimationType::Rotation => &[
            ("GetDegrees", get_zero),
            ("GetRadians", get_zero),
            ("GetOrigin", get_origin),
        ],
        _ => return Ok(()),
    };
    let Val::Table(table) = frame else {
        return Err(runtime_error("animation reference must be a table"));
    };
    for (name, function) in methods {
        table_set_rust_fn_static(state, table, name, *function)?;
    }
    Ok(())
}

fn get_zero_pair(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(2)
}

fn get_identity_pair(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(1.0));
    state.push(Val::Num(1.0));
    Ok(2)
}

fn get_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_origin(state: &mut LuaState) -> LuaResult<u32> {
    let point = create_string(state, "CENTER");
    state.push(point);
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(3)
}
