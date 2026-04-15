//! Post-load workarounds that are still required on the live rilua path.

pub fn apply(env: &crate::lua_api::WowLuaEnv) {
    crate::lua_api::workarounds_editmode::patch_edit_mode_manager(env);
    crate::lua_api::workarounds_editmode::init_edit_mode_layout(env);
}

pub fn apply_post_event(_env: &crate::lua_api::WowLuaEnv) {}
