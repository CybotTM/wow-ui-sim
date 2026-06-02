//! Legacy spellbook-tab globals backed by simulator spellbook data.
//!
//! Retail addons still call `GetNumSpellTabs()` / `GetSpellTabInfo()` as
//! compatibility probes. The simulator answers these from the spellbook state
//! instead of treating them as generic nil/zero shims.

use crate::lua_api::game_data::CLASS_LABELS;
use crate::lua_api::globals::spellbook_data;
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::stack_val;
use rilua::{LuaApiMut, LuaResult, Val};

/// `GetNumSpellTabs()` — legacy spellbook tabs map to skill lines.
fn get_num_spell_tabs(state: &mut rilua::vm::state::LuaState) -> LuaResult<u32> {
    let tab_count = if cfg!(feature = "client-mists") {
        spellbook_data::num_skill_lines() as f64
    } else {
        1.0
    };
    state.push(Val::Num(tab_count));
    Ok(1)
}

/// `GetSpellTabInfo(tabIndex)` — legacy clients expect:
/// `(name, icon, offset, numSpells, isGuild, offSpecID, shouldHide, specID)`.
fn get_spell_tab_info(state: &mut rilua::vm::state::LuaState) -> LuaResult<u32> {
    let tab_index = match stack_val(state, 1) {
        Val::Num(index) => index as i32,
        _ => 0,
    };
    if !cfg!(feature = "client-mists") {
        return get_retail_spell_tab_info(state, tab_index);
    }

    let Some(skill_line) = spellbook_data::get_skill_line(tab_index) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let name = create_string(state, skill_line.name);
    let offset = spellbook_data::skill_line_offset(tab_index);
    let spell_count = skill_line.spells.len();
    let off_spec_id = skill_line.off_spec_id.unwrap_or(0);
    let spec_id = skill_line.spec_id.unwrap_or(0);

    state.push(name);
    state.push(Val::Num(skill_line.icon_id as f64));
    state.push(Val::Num(offset as f64));
    state.push(Val::Num(spell_count as f64));
    state.push(Val::Bool(false));
    state.push(Val::Num(off_spec_id as f64));
    state.push(Val::Bool(false));
    state.push(Val::Num(spec_id as f64));
    Ok(8)
}

fn get_retail_spell_tab_info(
    state: &mut rilua::vm::state::LuaState,
    tab_index: i32,
) -> LuaResult<u32> {
    if tab_index != 1 {
        state.push(Val::Nil);
        return Ok(1);
    }

    let (class_label, spec_id) = {
        let sim = borrow_state(state)?;
        let class_index = sim.player.class_index.clamp(1, CLASS_LABELS.len() as i32);
        let class_label = CLASS_LABELS[class_index as usize - 1];
        (class_label, sim.player.active_spec_index)
    };

    let name = create_string(state, class_label);
    let icon = create_string(state, "Interface\\Icons\\Spell_Holy_PowerWordShield");
    state.push(name);
    state.push(icon);
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    state.push(Val::Num(spec_id as f64));
    Ok(6)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetNumSpellTabs", get_num_spell_tabs)?;
    LuaApiMut::register_function(lua, "GetSpellTabInfo", get_spell_tab_info)?;
    Ok(())
}
