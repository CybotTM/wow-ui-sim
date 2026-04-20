//! C_ModelInfo permanent shim — 3D model scene rendering is out of scope.

use crate::lua_bridge::{table_set_rust_fn, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

use crate::c_api::set_global_val;
use crate::lua_api::methods::create_table;

pub fn register_c_model_info(state: &mut LuaState) -> LuaResult<()> {
    let t = create_table(state);
    let Val::Table(t_ref) = t else {
        unreachable!("create_table must return a table");
    };
    register_model_scene_stubs(state, t_ref)?;
    table_set_rust_fn_static(
        state,
        t_ref,
        "GetModelSceneInfoByID",
        c_model_info_get_model_scene_info_by_id,
    )?;
    set_global_val(state, "C_ModelInfo", t);
    Ok(())
}

fn register_model_scene_stubs(state: &mut LuaState, t_ref: GcRef<Table>) -> LuaResult<()> {
    const NOOPS: &[&str] = &[
        "AddActiveModelScene",
        "AddActiveModelSceneActor",
        "ClearActiveModelScene",
        "ClearActiveModelSceneActor",
    ];
    const EMPTY_TABLE_GETTERS: &[&str] = &[
        "GetModelSceneActorDisplayInfoByID",
        "GetModelSceneActorInfoByID",
        "GetModelSceneCameraInfoByID",
    ];
    for name in NOOPS {
        table_set_rust_fn_static(state, t_ref, name, |_state| Ok(0))?;
    }
    for name in EMPTY_TABLE_GETTERS {
        table_set_rust_fn(state, t_ref, name, empty_table_result)?;
    }
    Ok(())
}

fn c_model_info_get_model_scene_info_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let camera_ids = create_table(state);
    let actor_ids = create_table(state);
    state.push(Val::Num(0.0));
    state.push(camera_ids);
    state.push(actor_ids);
    state.push(Val::Num(0.0));
    Ok(4)
}

fn empty_table_result(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}
