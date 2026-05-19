use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::{SavedVariablesManager, WtfConfig};

const EDIT_MODE_LAYOUT_ENV: &str = "WOW_SIM_EDIT_MODE_LAYOUT";

#[path = "edit_mode_api/cache.rs"]
mod cache;
#[path = "edit_mode_api/enums.rs"]
mod enums;
