use rilua::Val;
use wow_ui_sim::lua_api::WowLuaEnv;

pub fn fire_player_entering_world(env: &WowLuaEnv, initial_login: bool, is_reload: bool) {
    let _ = env.fire_event_with_args(
        "PLAYER_ENTERING_WORLD",
        &[Val::Bool(initial_login), Val::Bool(is_reload)],
    );
}
