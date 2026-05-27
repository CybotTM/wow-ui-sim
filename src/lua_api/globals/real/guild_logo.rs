//! `GetGuildLogoInfo()` backed by `SimState::world.guild_logo`.
//!
//! Real WoW signature:
//!
//! ```text
//! bkgR, bkgG, bkgB, borderR, borderG, borderB,
//! emblemR, emblemG, emblemB, emblemFilename = GetGuildLogoInfo()
//! ```
//!
//! All zero channels + empty emblem filename when the player has no guild —
//! matches the Blizzard UI's "no guild" branch without the tabard-drawing
//! callsite (GuildFrame, PaperDollFrame.TabardSlot) crashing on nil.
//!
//! Admin: `A_Admin.SetGuildEmblem(emblemFilename?, bkgR?, bkgG?, bkgB?,
//! borderR?, borderG?, borderB?, emblemR?, emblemG?, emblemB?)` — every arg
//! is optional; missing values default to 0 (or `""` for filename).

use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn get_guild_logo_info(state: &mut LuaState) -> LuaResult<u32> {
    let (bkg, border, emblem, filename) = {
        let sim = borrow_state(state)?;
        let logo = &sim.world.guild_logo;
        (
            logo.background,
            logo.border,
            logo.emblem,
            logo.emblem_filename.clone(),
        )
    };
    state.push(Val::Num(bkg.0));
    state.push(Val::Num(bkg.1));
    state.push(Val::Num(bkg.2));
    state.push(Val::Num(border.0));
    state.push(Val::Num(border.1));
    state.push(Val::Num(border.2));
    state.push(Val::Num(emblem.0));
    state.push(Val::Num(emblem.1));
    state.push(Val::Num(emblem.2));
    let filename_val = create_string(state, &filename);
    state.push(filename_val);
    Ok(10)
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    table_set_rust_fn_static(state, state.global, "GetGuildLogoInfo", get_guild_logo_info)?;
    Ok(())
}
