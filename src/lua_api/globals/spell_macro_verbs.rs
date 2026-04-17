//! Spell / talent / macro pickup verbs. Routes through
//! `SimState.cursor_item` and the macro table.
//!
//! Migrates 8 entries off `GLOBAL_NIL_STUBS`:
//!
//! - `PickupSpell(spellId)`        — cursor = Spell(spellId)
//! - `PickupTalent(talentId)`      — cursor = Talent(talentId, pvp=false)
//! - `PickupPvpTalent(talentId)`   — cursor = Talent(talentId, pvp=true)
//! - `PickupPetAction(slot)`       — cursor = PetAction(slot, synthesized
//!                                     spell_id = `1_000_000 + slot`)
//! - `PickupMacro(index)`          — cursor = Macro(macro_index)
//! - `RunMacro(index_or_name)`     — set `running_macro` to the resolved
//!                                     slot index. Silent no-op for unknown.
//! - `StopMacro()`                 — clears `running_macro`.
//! - `EditMacro(index_or_name, name?, icon?, body?)` — update macro
//!                                     name / icon / body in-place. Auto-
//!                                     grows the macro table when passing
//!                                     an index beyond current length.
//!
//! Registered from `register_tail_globals` after `missing_surface`.

use crate::lua_api::methods::borrow_state_mut;
use crate::lua_api::state::MacroInfo;
use crate::lua_api::state_types::CursorInfo;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

const PET_ACTION_SPELL_OFFSET: u32 = 1_000_000;

fn stack_u32(state: &mut LuaState, index: i32) -> Option<u32> {
    match stack_val(state, index) {
        Val::Num(n) if n >= 0.0 => Some(n as u32),
        _ => None,
    }
}

fn stack_string(state: &mut LuaState, index: i32) -> Option<String> {
    Option::<String>::from_stack(state, index).ok().flatten()
}

/// Resolve a macro reference — numeric = 1-based slot, string = name —
/// into a 0-based slot index within the macro table.
fn resolve_macro_slot(state: &mut LuaState, index: i32, macros: &[MacroInfo]) -> Option<usize> {
    match stack_val(state, index) {
        Val::Num(n) if n >= 1.0 => Some(n as usize - 1),
        Val::Str(_) => {
            let name = stack_string(state, index)?;
            macros.iter().position(|m| m.name == name)
        }
        _ => None,
    }
}

/// `PickupSpell(spellId)` — cursor = Spell.
fn pickup_spell(state: &mut LuaState) -> LuaResult<u32> {
    let Some(spell_id) = stack_u32(state, 1) else {
        return Ok(0);
    };
    borrow_state_mut(state)?.cursor_item = Some(CursorInfo::Spell { spell_id });
    Ok(0)
}

/// `PickupTalent(talentId)` — cursor = Talent (pvp=false).
fn pickup_talent(state: &mut LuaState) -> LuaResult<u32> {
    let Some(talent_id) = stack_u32(state, 1) else {
        return Ok(0);
    };
    borrow_state_mut(state)?.cursor_item = Some(CursorInfo::Talent {
        talent_id,
        pvp: false,
    });
    Ok(0)
}

/// `PickupPvpTalent(talentId)` — cursor = Talent (pvp=true).
fn pickup_pvp_talent(state: &mut LuaState) -> LuaResult<u32> {
    let Some(talent_id) = stack_u32(state, 1) else {
        return Ok(0);
    };
    borrow_state_mut(state)?.cursor_item = Some(CursorInfo::Talent {
        talent_id,
        pvp: true,
    });
    Ok(0)
}

/// `PickupPetAction(slot)` — cursor = PetAction with synthesized spell_id.
fn pickup_pet_action(state: &mut LuaState) -> LuaResult<u32> {
    let Some(slot) = stack_u32(state, 1) else {
        return Ok(0);
    };
    borrow_state_mut(state)?.cursor_item = Some(CursorInfo::PetAction {
        slot,
        spell_id: PET_ACTION_SPELL_OFFSET.saturating_add(slot),
    });
    Ok(0)
}

/// `PickupMacro(index)` — cursor = Macro(index).
fn pickup_macro(state: &mut LuaState) -> LuaResult<u32> {
    let Some(macro_index) = stack_u32(state, 1) else {
        return Ok(0);
    };
    borrow_state_mut(state)?.cursor_item = Some(CursorInfo::Macro { macro_index });
    Ok(0)
}

/// `RunMacro(index_or_name)` — flip `running_macro` to the resolved slot.
fn run_macro(state: &mut LuaState) -> LuaResult<u32> {
    let macros = borrow_state_mut(state)?.macros.clone();
    let Some(zero_based) = resolve_macro_slot(state, 1, &macros) else {
        return Ok(0);
    };
    if zero_based >= macros.len() {
        return Ok(0);
    }
    borrow_state_mut(state)?.running_macro = Some((zero_based + 1) as u32);
    Ok(0)
}

/// `StopMacro()` — clear `running_macro`.
fn stop_macro(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.running_macro = None;
    Ok(0)
}

/// `EditMacro(index_or_name, name?, icon?, body?)` — update a macro slot
/// in-place. Passing an index beyond current length grows the macro table
/// with empty entries until the slot exists.
fn edit_macro(state: &mut LuaState) -> LuaResult<u32> {
    let macros = borrow_state_mut(state)?.macros.clone();
    let slot = match stack_val(state, 1) {
        Val::Num(n) if n >= 1.0 => Some(n as usize - 1),
        Val::Str(_) => {
            let Some(name) = stack_string(state, 1) else {
                return Ok(0);
            };
            macros.iter().position(|m| m.name == name)
        }
        _ => None,
    };
    let Some(slot) = slot else {
        return Ok(0);
    };

    let new_name = stack_string(state, 2);
    let new_icon = stack_string(state, 3);
    let new_body = stack_string(state, 4);

    let mut st = borrow_state_mut(state)?;
    while st.macros.len() <= slot {
        st.macros.push(MacroInfo::default());
    }
    let entry = &mut st.macros[slot];
    if let Some(name) = new_name {
        entry.name = name;
    }
    if let Some(icon) = new_icon {
        entry.icon = icon;
    }
    if let Some(body) = new_body {
        entry.body = body;
    }
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "PickupSpell", pickup_spell)?;
    LuaApiMut::register_function(lua, "PickupTalent", pickup_talent)?;
    LuaApiMut::register_function(lua, "PickupPvpTalent", pickup_pvp_talent)?;
    LuaApiMut::register_function(lua, "PickupPetAction", pickup_pet_action)?;
    LuaApiMut::register_function(lua, "PickupMacro", pickup_macro)?;
    LuaApiMut::register_function(lua, "RunMacro", run_macro)?;
    LuaApiMut::register_function(lua, "StopMacro", stop_macro)?;
    LuaApiMut::register_function(lua, "EditMacro", edit_macro)?;
    Ok(())
}
