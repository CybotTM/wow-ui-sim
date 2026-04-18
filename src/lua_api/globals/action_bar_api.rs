//! Minimal action-bar helpers kept alive on the rilua path.

use crate::Result;
use crate::lua_api::SimState;
use crate::lua_api::methods::{call_function_state, frame_ref, table_get};
use rilua::LuaApiMut;
use std::cell::RefCell;
use std::rc::Rc;

pub fn push_action_button_state_update(
    state: &Rc<RefCell<SimState>>,
    lua: &mut rilua::Lua,
) -> Result<()> {
    let button_ids = {
        let sim = state.borrow();
        sim.action_ui_buttons
            .iter()
            .map(|(button_id, _)| *button_id)
            .collect::<Vec<_>>()
    };
    if button_ids.is_empty() {
        return Ok(());
    }

    let state = lua.state_mut();
    for button_id in button_ids {
        let button = frame_ref(state, button_id)?;
        let update_state = table_get(state, button, "UpdateState");
        if matches!(update_state, rilua::Val::Function(_)) {
            call_function_state(state, update_state, &[button])?;
        }
    }
    Ok(())
}

pub fn spell_cooldown_times(state: &SimState, spell_id: u32, now: f64) -> (f64, f64) {
    let mut best_start = 0.0_f64;
    let mut best_end = 0.0_f64;

    if let Some((gcd_start, gcd_duration)) = state.gcd {
        let gcd_end = gcd_start + gcd_duration;
        if gcd_end > now {
            best_start = gcd_start;
            best_end = gcd_end;
        }
    }

    if let Some(cooldown) = state.spell_cooldowns.get(&spell_id) {
        let cooldown_end = cooldown.start + cooldown.duration;
        if cooldown_end > now && cooldown_end > best_end {
            best_start = cooldown.start;
            best_end = cooldown_end;
        }
    }

    if best_end > now {
        (best_start, best_end - best_start)
    } else {
        (0.0, 0.0)
    }
}

pub fn start_cooldowns<T>(_state: &Rc<RefCell<SimState>>, _lua: T, _spell_id: u32) -> Result<()> {
    Ok(())
}

pub fn start_cast<T>(
    _state: &Rc<RefCell<SimState>>,
    _lua: T,
    _spell_id: u32,
    _cast_time_ms: i32,
) -> Result<()> {
    Ok(())
}

pub fn apply_instant_spell<T>(
    _state: &Rc<RefCell<SimState>>,
    _lua: T,
    _spell_id: u32,
) -> Result<()> {
    Ok(())
}
