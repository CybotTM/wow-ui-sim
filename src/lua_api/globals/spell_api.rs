//! Minimal spell helpers kept alive on the rilua path.

use crate::lua_api::SimState;
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_spell_api<T>(_lua: &T, _state: Rc<RefCell<SimState>>) -> crate::Result<()> {
    Ok(())
}

/// Cast time in milliseconds for spells that have one (WoW API returns ms).
pub fn spell_cast_time(spell_id: i32) -> i32 {
    match spell_id {
        19750 => 1500, // Flash of Light
        82326 => 2500, // Holy Light
        7328 => 10000, // Redemption
        _ => 0,
    }
}
