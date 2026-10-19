//! Wrath's single class-tab projection of existing known-spell membership.
//! Skill-line metadata and spell-level ordering are not supplied by this model.

use std::ffi::{CStr, CString};

use crate::lua_api::game_data::CLASS_LABELS;
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_api::state::SimState;
use crate::lua_bridge::stack_val;
use crate::spells;
use rilua::vm::state::LuaState;
use rilua::{LuaError, LuaResult, RuntimeError, Val};

pub(super) fn get_spell_tab_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = numeric_argument(state).ok_or_else(tab_usage_error)? as i32;
    if index != 1 {
        state.push(Val::Nil);
        state.push(Val::Nil);
        for _ in 0..4 {
            state.push(Val::Num(0.0));
        }
        return Ok(6);
    }
    let (label, full_count, compact_count) = {
        let sim = borrow_state(state)?;
        let class_index = sim.player.class_index.clamp(1, CLASS_LABELS.len() as i32);
        let full = ordered_known_spells(&sim);
        let compact = compact_slots(&full);
        (
            CLASS_LABELS[class_index as usize - 1],
            full.len(),
            compact.len(),
        )
    };
    push_tab_info(state, label, full_count, compact_count);
    Ok(6)
}

fn push_tab_info(state: &mut LuaState, label: &str, full_count: usize, compact_count: usize) {
    let name = create_string(state, label);
    let icon = create_string(state, "Interface\\Icons\\Spell_Holy_PowerWordShield");
    state.push(name);
    state.push(icon);
    state.push(Val::Num(0.0));
    state.push(Val::Num(full_count as f64));
    state.push(Val::Num(0.0));
    state.push(Val::Num(compact_count as f64));
}

fn tab_usage_error() -> LuaError {
    LuaError::Runtime(RuntimeError {
        message: "Usage: GetSpellTabInfo(index)".into(),
        level: 1,
        traceback: vec![],
    })
}

fn numeric_argument(state: &LuaState) -> Option<f64> {
    match stack_val(state, 1) {
        Val::Num(number) => Some(number),
        Val::Str(reference) => {
            let string = state.gc.string_arena.get(reference)?;
            parse_numeric_string(string.data())
        }
        _ => None,
    }
}

fn parse_numeric_string(bytes: &[u8]) -> Option<f64> {
    let string = CString::new(bytes).ok()?;
    let mut end = std::ptr::null_mut();
    // CString supplies the terminator; strtod returns an end pointer inside it.
    let number = unsafe { libc::strtod(string.as_ptr(), &mut end) };
    if end == string.as_ptr().cast_mut() {
        return None;
    }
    // strtod's end pointer remains valid while the owning CString is alive.
    let remaining = unsafe { CStr::from_ptr(end) }.to_bytes();
    remaining
        .iter()
        .all(u8::is_ascii_whitespace)
        .then_some(number)
}

fn ordered_known_spells(sim: &SimState) -> Vec<u32> {
    let mut full: Vec<_> = sim.known_spells.iter().copied().collect();
    // ID only breaks otherwise equal keys for deterministic HashSet projection.
    full.sort_unstable_by_key(|&id| (spell_sort_key(id), id));
    full
}

fn spell_sort_key(id: u32) -> (&'static str, i32, &'static str) {
    let Some(spell) = spells::get_spell(id) else {
        // INFERRED: missing metadata is an independent rank-zero boundary.
        return ("", 0, "");
    };
    (spell.name, parse_rank(spell.subtext), spell.subtext)
}

fn compact_slots(full: &[u32]) -> Vec<usize> {
    let mut compact = Vec::new();
    for (slot, &id) in full.iter().enumerate() {
        let continues_rank_run = compact
            .last()
            .is_some_and(|&previous| same_ranked_name(full[previous], id));
        if continues_rank_run {
            *compact.last_mut().expect("existing rank run") = slot;
        } else {
            compact.push(slot);
        }
    }
    compact
}

fn same_ranked_name(first: u32, second: u32) -> bool {
    spells::get_spell(first)
        .zip(spells::get_spell(second))
        .is_some_and(|(first, second)| {
            first.name == second.name
                && parse_rank(first.subtext) != 0
                && parse_rank(second.subtext) != 0
        })
}

fn parse_rank(subtext: &str) -> i32 {
    let mut rank = 0;
    // Committed spell metadata has ranks <= 4; native overflow is unmodeled.
    for byte in subtext.bytes() {
        if byte.is_ascii_digit() {
            rank = rank * 10 + i32::from(byte - b'0');
        } else if rank != 0 {
            break;
        }
    }
    rank
}
