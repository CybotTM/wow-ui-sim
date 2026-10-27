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
    LuaApiMut::register_function(lua, "GetUnspentTalentPoints", get_unspent_talent_points)?;
    LuaApiMut::register_function(lua, "GetNumTalentTabs", get_num_talent_tabs)?;
    LuaApiMut::register_function(lua, "GetTalentTabInfo", get_talent_tab_info)?;
    LuaApiMut::register_function(lua, "GetNumTalents", get_num_talents)?;
    LuaApiMut::register_function(lua, "GetTalentInfo", get_talent_info)?;
    LuaApiMut::register_function(lua, "GetTalentPrereqs", get_talent_prereqs)?;
    LuaApiMut::register_function(
        lua,
        "GetGroupPreviewTalentPointsSpent",
        get_group_preview_points,
    )
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
    let socket = borrow_state(state)?
        .wrath_glyph_sockets
        .get(&(group, index))
        .cloned();
    let Some(socket) = socket else {
        return Ok(0);
    };
    push_glyph_socket_info(state, &socket);
    Ok(4)
}

fn push_glyph_socket_info(state: &mut LuaState, socket: &crate::lua_api::state::WrathGlyphSocket) {
    let has_glyph = socket.glyph_spell_id.is_some();
    state.push(if socket.enabled {
        Val::Num(1.0)
    } else {
        Val::Nil
    });
    state.push(
        socket
            .glyph_type
            .map_or(Val::Nil, |value| Val::Num(value as f64)),
    );
    state.push(
        socket
            .glyph_spell_id
            .filter(|_| has_glyph)
            .map_or(Val::Nil, |value| Val::Num(value as f64)),
    );
    let icon = socket.icon.as_deref().filter(|_| has_glyph);
    let icon = icon.map_or(Val::Nil, |value| {
        Val::Str(state.gc.intern_string(value.as_bytes()))
    });
    state.push(icon);
}

fn get_num_glyph_sockets(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(6.0));
    Ok(1)
}

fn get_unspent_talent_points(state: &mut LuaState) -> LuaResult<u32> {
    let inspect = string_to_bool(state, stack_val(state, 1));
    let pet = string_to_bool(state, stack_val(state, 2));
    let points = if inspect || pet {
        // No inspected or pet group is supplied in this scenario model.
        0
    } else {
        let group = selected_group(state, 3)?;
        borrow_state(state)?
            .wrath_unspent_talent_points
            .get(&group)
            .copied()
            .unwrap_or(0)
    };
    state.push(Val::Num(points as f64));
    Ok(1)
}

// Wrath show consumers: TalentFrameBase.lua:118,139,163,195,526,568,573.
// Signatures: 3.1.0-api-changes.txt:25-27,32; 2.3.0-api-changes.txt:86,88.
// Absent records and inspect/pet state are scenario defaults, not native claims.
fn one_based_index(state: &LuaState, index: i32) -> Option<u32> {
    match stack_val(state, index) {
        Val::Num(value) if value >= 1.0 => Some(value as u32),
        _ => None,
    }
}

fn push_string(state: &mut LuaState, value: &str) {
    let string = state.gc.intern_string(value.as_bytes());
    state.push(Val::Str(string));
}

fn push_flag(state: &mut LuaState, value: bool) {
    state.push(if value { Val::Num(1.0) } else { Val::Nil });
}

fn is_other_unit(state: &LuaState, inspect: i32, pet: i32) -> bool {
    string_to_bool(state, stack_val(state, inspect)) || string_to_bool(state, stack_val(state, pet))
}

fn get_num_talent_tabs(state: &mut LuaState) -> LuaResult<u32> {
    let count = if is_other_unit(state, 1, 2) {
        0
    } else {
        let sim = borrow_state(state)?;
        let group = sim.wrath_legacy.player_talent_group + 1;
        sim.wrath_talent_tabs
            .keys()
            .filter(|(g, _)| *g == group)
            .count()
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn get_talent_tab_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(tab) = one_based_index(state, 1) else {
        return Ok(0);
    };
    if is_other_unit(state, 2, 3) {
        return Ok(0);
    }
    let group = selected_group(state, 4)?;
    let record = borrow_state(state)?
        .wrath_talent_tabs
        .get(&(group, tab))
        .cloned();
    let Some(record) = record else {
        return Ok(0);
    };
    push_string(state, &record.name);
    push_string(state, &record.icon);
    state.push(Val::Num(record.points_spent as f64));
    push_string(state, &record.background);
    state.push(Val::Num(record.preview_points_spent as f64));
    Ok(5)
}

fn get_num_talents(state: &mut LuaState) -> LuaResult<u32> {
    let tab = one_based_index(state, 1);
    let count = if is_other_unit(state, 2, 3) {
        0
    } else {
        let sim = borrow_state(state)?;
        let group = sim.wrath_legacy.player_talent_group + 1;
        tab.and_then(|tab| sim.wrath_talent_rows.get(&(group, tab)))
            .map_or(0, Vec::len)
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn supplied_rows(state: &LuaState) -> LuaResult<Vec<crate::lua_api::state::WrathTalentRow>> {
    let Some(tab) = one_based_index(state, 1) else {
        return Ok(Vec::new());
    };
    if is_other_unit(state, 3, 4) {
        return Ok(Vec::new());
    }
    let group = selected_group(state, 5)?;
    Ok(borrow_state(state)?
        .wrath_talent_rows
        .get(&(group, tab))
        .cloned()
        .unwrap_or_default())
}

fn get_talent_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = one_based_index(state, 2) else {
        return Ok(0);
    };
    let rows = supplied_rows(state)?;
    let Some(row) = rows.get((index - 1) as usize) else {
        return Ok(0);
    };
    push_string(state, &row.name);
    push_string(state, &row.icon);
    state.push(Val::Num(row.tier as f64));
    state.push(Val::Num(row.column as f64));
    state.push(Val::Num(row.rank as f64));
    state.push(Val::Num(row.max_rank as f64));
    push_flag(state, row.exceptional);
    push_flag(state, row.meets_prereq);
    state.push(Val::Num(row.preview_rank as f64));
    push_flag(state, row.meets_preview_prereq);
    Ok(10)
}

fn get_talent_prereqs(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = one_based_index(state, 2) else {
        return Ok(0);
    };
    let rows = supplied_rows(state)?;
    let Some(row) = rows.get((index - 1) as usize) else {
        return Ok(0);
    };
    let mut count = 0;
    for prerequisite in &row.prerequisites {
        let Some(target) = prerequisite
            .talent_index
            .checked_sub(1)
            .and_then(|i| rows.get(i as usize))
        else {
            return Err(rilua::runtime_error(
                "Wrath talent prerequisite references an absent row",
            ));
        };
        state.push(Val::Num(target.tier as f64));
        state.push(Val::Num(target.column as f64));
        push_flag(state, target.rank >= prerequisite.required_rank);
        push_flag(state, target.preview_rank >= prerequisite.required_rank);
        count += 4;
    }
    Ok(count)
}

fn get_group_preview_points(state: &mut LuaState) -> LuaResult<u32> {
    let points = if string_to_bool(state, stack_val(state, 1)) {
        0
    } else {
        let group = selected_group(state, 2)?;
        borrow_state(state)?
            .wrath_talent_tabs
            .iter()
            .filter(|((g, _), _)| *g == group)
            .map(|(_, tab)| tab.preview_points_spent)
            .sum::<u32>()
    };
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
