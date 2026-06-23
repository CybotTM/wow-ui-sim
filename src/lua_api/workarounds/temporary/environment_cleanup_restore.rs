//! Temporary EnvironmentCleanup runtime-surface restore.
//!
//! The simulator loads Blizzard_EnvironmentCleanup in the same run as later UI
//! addons, so globals that the cleanup file nils still need to be restored for
//! the rest of startup. Keep that repair out of the generic globals surface.

use crate::lua_api::SimState;
use std::cell::RefCell;
use std::rc::Rc;

const CHARACTER_FRAME_SUBFRAMES_RESTORE_LUA: &str = r#"
if type(CHARACTERFRAME_SUBFRAMES) ~= "table" then
    CHARACTERFRAME_SUBFRAMES = { "PaperDollFrame", "ReputationFrame", "TokenFrame" }
end
"#;

pub(crate) fn restore_post_cleanup_globals(
    lua: &mut rilua::Lua,
    state: Rc<RefCell<SimState>>,
) -> crate::Result<()> {
    crate::lua_api::env_init::init_shared_bootstrap(lua)?;
    crate::lua_api::env_init::init_runtime_surface_bootstrap(lua)?;
    crate::lua_api::env_init::init_enum_globals(lua)?;
    crate::lua_api::globals::register::register_globals(lua, state)?;
    super::debug_environment_defaults::apply_bootstrap(lua)?;
    super::secure_execute_range::apply_bootstrap(lua)?;
    lua.exec(CHARACTER_FRAME_SUBFRAMES_RESTORE_LUA)?;
    super::ui_parent_panel_toggles::apply_bootstrap(lua)
}
