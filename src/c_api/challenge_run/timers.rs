use super::inputs::nonnegative;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

#[derive(Debug, Clone)]
pub(crate) struct ElapsedTimer {
    name: String,
    kind: i32,
    elapsed: f64,
}

pub(super) fn start(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    let timer = ElapsedTimer {
        name: String::from_stack(state, 2)?,
        kind: i32::from_stack(state, 3)?,
        elapsed: nonnegative(f64::from_stack(state, 4)?)?,
    };
    {
        let mut sim = borrow_state_mut(state)?;
        if sim.challenge_run.active.is_none() {
            return Err(rilua::runtime_error(
                "a world elapsed timer requires an active challenge run",
            ));
        }
        if sim.challenge_run.timers.contains_key(&id) {
            return Err(rilua::runtime_error("world elapsed timer already exists"));
        }
        sim.challenge_run.timers.insert(id, timer);
    }
    dispatch_event_now(state, "WORLD_STATE_TIMER_START", &[Val::Num(f64::from(id))])?;
    Ok(0)
}

pub(super) fn set(state: &mut LuaState) -> LuaResult<u32> {
    update(state, false)
}

pub(super) fn advance(state: &mut LuaState) -> LuaResult<u32> {
    update(state, true)
}

fn update(state: &mut LuaState, advance: bool) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    let seconds = nonnegative(f64::from_stack(state, 2)?)?;
    let mut sim = borrow_state_mut(state)?;
    let timer = sim
        .challenge_run
        .timers
        .get_mut(&id)
        .ok_or_else(|| rilua::runtime_error("world elapsed timer does not exist"))?;
    let elapsed = if advance {
        timer.elapsed + seconds
    } else {
        seconds
    };
    timer.elapsed = nonnegative(elapsed)?;
    Ok(0)
}

pub(crate) fn list(state: &mut LuaState) -> LuaResult<u32> {
    let ids: Vec<_> = borrow_state(state)?
        .challenge_run
        .timers
        .keys()
        .copied()
        .collect();
    for id in &ids {
        state.push(Val::Num(f64::from(*id)));
    }
    Ok(ids.len() as u32)
}

pub(crate) fn query(state: &mut LuaState) -> LuaResult<u32> {
    let id = i32::from_stack(state, 1)?;
    let timer = borrow_state(state)?.challenge_run.timers.get(&id).cloned();
    let Some(timer) = timer else { return Ok(0) };
    let name = create_string(state, &timer.name);
    state.push(name);
    state.push(Val::Num(timer.elapsed));
    state.push(Val::Num(f64::from(timer.kind)));
    Ok(3)
}
