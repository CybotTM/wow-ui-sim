//! Supplied spell-action usability; no resource or range inference.

use crate::lua_api::SimState;
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

/// Host-supplied classification used by action-button presentation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ActionUsability {
    #[default]
    Usable,
    InsufficientPower,
    Unusable,
}

fn classify_action(sim: &SimState, slot: u32) -> ActionUsability {
    if let Some(spell_id) = sim.action_bars.get(&slot) {
        return sim
            .spell_action_usability
            .get(spell_id)
            .copied()
            .unwrap_or_default();
    }
    if sim.action_outfits.contains_key(&slot) || sim.action_macros.contains_key(&slot) {
        return ActionUsability::Usable;
    }
    ActionUsability::Unusable
}

pub(crate) fn is_usable_action(state: &mut LuaState) -> LuaResult<u32> {
    let slot = match stack_val(state, 1) {
        Val::Num(number) if number >= 0.0 => Some(number as u32),
        _ => None,
    };
    let classification = {
        let sim = borrow_state(state)?;
        slot.map(|slot| classify_action(&sim, slot))
            .unwrap_or(ActionUsability::Unusable)
    };
    let usable = classification == ActionUsability::Usable;
    let insufficient_power = classification == ActionUsability::InsufficientPower;
    state.push(Val::Bool(usable));
    state.push(Val::Bool(insufficient_power));
    Ok(2)
}
