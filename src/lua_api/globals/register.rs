//! Main register_globals function and core utilities.
//!
//! This module contains the main registration function that orchestrates
//! registering all WoW API globals, plus core Lua utilities like print,
//! type, ipairs, pairs, getmetatable, and setmetatable.

use super::super::SimState;
use rilua::LuaApiMut;
use std::cell::RefCell;
use std::rc::Rc;

/// Register the live rilua global surface.
///
/// This native registrar owns the split-module wiring so `env_init` can use
/// one entry point for the current global surface again.
pub fn register_globals(lua: &mut rilua::Lua, _state: Rc<RefCell<SimState>>) -> crate::Result<()> {
    register_bootstrap_globals(lua)?;
    register_frame_globals(lua)?;
    register_tail_globals(lua)?;
    Ok(())
}

fn register_bootstrap_globals(lua: &mut rilua::Lua) -> crate::Result<()> {
    super::strings::register_all_ui_strings(lua)?;
    super::security::register_all(lua)?;
    super::keybindings::register_all(lua)?;
    super::stubs::register_all(lua.state_mut());
    LuaApiMut::register_function(lua, "UpdateUIParentPosition", update_ui_parent_position)?;
    // Must run after stubs so the fixture aura data overrides the
    // stub_nil registrations for C_UnitAuras.GetAuraSlots & friends.
    super::auras::register_all(lua.state_mut());
    Ok(())
}

fn register_frame_globals(lua: &mut rilua::Lua) -> crate::Result<()> {
    super::create_frame::register_all(lua)?;
    super::font_strings_collection::register_all(lua)?;
    super::utility_system_spell::register_all(lua)?;
    super::net_stats::register_all(lua)?;
    super::store_frame::register_all(lua)?;
    super::unit_probes::register_all(lua)?;
    super::inventory_slot::register_all(lua)?;
    super::zone_text::register_all(lua)?;
    super::modifier_keys::register_all(lua)?;
    super::guild_logo::register_all(lua)?;
    super::guild_control::register_all(lua)?;
    super::targeting_verbs::register_all(lua)?;
    super::game_rules::register_all(lua)?;
    super::guild_info::register_all(lua)?;
    super::housing::register_all(lua)?;
    super::pet_battles::register_all(lua)?;
    super::photo_sharing::register_all(lua)?;
    super::wowlabs::register_all(lua)?;
    Ok(())
}

fn register_tail_globals(lua: &mut rilua::Lua) -> crate::Result<()> {
    super::lfg_list::register_all(lua)?;
    super::lfg_info::register_all(lua)?;
    super::locale_info::register_all(lua)?;
    super::missing_surface::register_all(lua)?;
    super::lua_duration_object::register_lua_duration_object(lua)?;
    super::combat_verbs::register_all(lua)?;
    super::inventory_verbs::register_all(lua)?;
    super::mail_verbs::register_all(lua)?;
    super::group_verbs::register_all(lua)?;
    super::guild_verbs::register_all(lua)?;
    super::quest_verbs::register_all(lua)?;
    super::close_frames::register_all(lua)?;
    super::battlefield_verbs::register_all(lua)?;
    super::channel_verbs::register_all(lua)?;
    super::spell_macro_verbs::register_all(lua)?;
    super::chat_window_verbs::register_all(lua)?;
    super::offer_verbs::register_all(lua)?;
    super::trade_verbs::register_all(lua)?;
    super::movement_verbs::register_all(lua)?;
    super::compat_overrides::register_all(lua)?;
    super::admin::register_all(lua)?;
    super::super::timer_layout::register_all(lua)?;
    Ok(())
}

fn update_ui_parent_position(_state: &mut rilua::vm::state::LuaState) -> rilua::LuaResult<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::register_globals;
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn register_globals_is_idempotent_and_keeps_core_surface_live() {
        let env = WowLuaEnv::new().expect("failed to create Lua environment");
        {
            let mut lua = env.rilua_mut();
            register_globals(&mut lua, env.state().clone()).expect("failed to re-register globals");
        }

        let result: (bool, bool, bool, bool) = env
            .eval(
                r#"
                return type(CreateFrame) == "function",
                       type(strsplit) == "function",
                       type(C_Timer) == "table",
                       type(OKAY) == "string"
                "#,
            )
            .expect("failed to probe globals");

        assert!(result.0, "CreateFrame should remain registered");
        assert!(result.1, "strsplit should remain registered");
        assert!(result.2, "C_Timer should remain registered");
        assert!(result.3, "UI strings should remain registered");
    }
}
