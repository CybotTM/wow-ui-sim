//! Temporary `C_Club` notification-settings fallback.
//!
//! Club stream notification preferences are not modeled yet. The Communities
//! UI expects an options table, so return an empty settings table until
//! per-stream notification state exists.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_club_notification_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Club")?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetClubStreamNotificationSettings",
        get_club_stream_notification_settings,
    )
}

fn get_club_stream_notification_settings(state: &mut LuaState) -> LuaResult<u32> {
    let settings = create_table(state);
    state.push(settings);
    Ok(1)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn club_stream_notification_settings_defaults_to_empty_table() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let result: bool = env
            .eval(r#"return type(C_Club.GetClubStreamNotificationSettings("guild-0")) == "table""#)
            .expect("notification settings should be queryable");

        assert!(result);
    }
}
