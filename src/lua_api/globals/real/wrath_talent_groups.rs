//! Native 12340 cached talent-group selection (TalentInfo_Scripts.cpp:583–595).
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub fn register(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "GetActiveTalentGroup", get_active_talent_group)?;
    LuaApiMut::register_function(lua, "GetNumTalentGroups", get_num_talent_groups)?;
    LuaApiMut::register_function(lua, "GetGlyphSocketInfo", get_glyph_socket_info)?;
    LuaApiMut::register_function(lua, "GetNumGlyphSockets", get_num_glyph_sockets)?;
    LuaApiMut::register_function(lua, "GetUnspentTalentPoints", get_unspent_talent_points)
}

fn get_active_talent_group(state: &mut LuaState) -> LuaResult<u32> {
    let inspect = string_to_bool(state, stack_val(state, 1));
    let pet = string_to_bool(state, stack_val(state, 2));
    let group = {
        let sim = borrow_state(state)?;
        if inspect {
            sim.wrath_legacy.inspected_talent_group + 1
        } else if pet {
            1
        } else {
            sim.wrath_legacy.player_talent_group + 1
        }
    };
    state.push(Val::Num(group as f64));
    Ok(1)
}

fn get_num_talent_groups(state: &mut LuaState) -> LuaResult<u32> {
    let inspect = string_to_bool(state, stack_val(state, 1));
    let pet = string_to_bool(state, stack_val(state, 2));
    let count = {
        let sim = borrow_state(state)?;
        let counts = &sim.wrath_legacy.talent_group_counts;
        if inspect {
            counts.inspected
        } else if pet {
            u32::from(counts.pet_tab_present && counts.pet_tab_count != 0)
        } else {
            counts.player
        }
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

// Wrath consumers: Blizzard_GlyphUI.lua:65 and Blizzard_TalentUI.lua:499.
// Signatures: data/patch-api/sources/3.1.0-api-changes.txt:24,34.
// Return semantics follow the aso 3.3.5a decompilation comparison; absent
// scenario inputs are empty or zero, not a native initialization claim.
fn selected_group(state: &LuaState, index: i32) -> LuaResult<u32> {
    match stack_val(state, index) {
        Val::Num(value) => Ok(value as u32),
        _ => Ok(borrow_state(state)?.wrath_legacy.player_talent_group + 1),
    }
}

fn get_glyph_socket_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = match stack_val(state, 1) {
        Val::Num(value) if (1.0..=6.0).contains(&value) => value as u32,
        Val::Num(_) => return Ok(0),
        _ => {
            return Err(rilua::runtime_error(
                "GetGlyphSocketInfo expects a numeric index",
            ));
        }
    };
    let group = selected_group(state, 2)?;
    let socket = {
        let sim = borrow_state(state)?;
        let group_supplied = sim
            .wrath_glyph_sockets
            .keys()
            .any(|(supplied_group, _)| *supplied_group == group);
        if group == 0 || (group > sim.wrath_legacy.talent_group_counts.player && !group_supplied) {
            return Ok(0);
        }
        sim.wrath_glyph_sockets.get(&(group, index)).cloned()
    };
    let enabled = socket
        .as_ref()
        .is_some_and(|value| value.enabled && value.glyph_spell_id.is_some());
    state.push(if enabled { Val::Num(1.0) } else { Val::Nil });
    state.push(
        socket
            .as_ref()
            .and_then(|value| value.glyph_type)
            .map_or(Val::Nil, |value| Val::Num(value as f64)),
    );
    state.push(
        socket
            .as_ref()
            .filter(|_| enabled)
            .and_then(|value| value.glyph_spell_id)
            .map_or(Val::Nil, |value| Val::Num(value as f64)),
    );
    let icon = socket
        .as_ref()
        .filter(|_| enabled)
        .and_then(|value| value.icon.as_deref());
    let icon = icon.map_or(Val::Nil, |value| {
        Val::Str(state.gc.intern_string(value.as_bytes()))
    });
    state.push(icon);
    Ok(4)
}

fn get_num_glyph_sockets(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(6.0));
    Ok(1)
}

fn get_unspent_talent_points(state: &mut LuaState) -> LuaResult<u32> {
    let group = selected_group(state, 3)?;
    let points = borrow_state(state)?
        .wrath_unspent_talent_points
        .get(&group)
        .copied()
        .unwrap_or(0);
    state.push(Val::Num(points as f64));
    Ok(1)
}

/// LoadXML StringToBOOL, not Lua truthiness and not numeric-string coercion.
fn string_to_bool(state: &LuaState, value: Val) -> bool {
    match value {
        Val::Bool(value) => value,
        Val::Num(value) => value as i32 != 0,
        Val::Str(reference) => {
            let Some(string) = state.gc.string_arena.get(reference) else {
                return false;
            };
            let bytes = string.data();
            matches!(bytes.first(), Some(b'1'..=b'9' | b'T' | b'Y' | b't' | b'y'))
                || bytes.eq_ignore_ascii_case(b"on")
                || bytes.eq_ignore_ascii_case(b"enabled")
        }
        _ => false,
    }
}
