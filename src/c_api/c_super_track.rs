//! Supplied SuperTrack targets. No priority arbitration is inferred.

#[derive(Debug, Clone, Default)]
pub struct SuperTrackState {
    pub quest_id: Option<i32>,
    pub content: Option<(i32, i32)>,
    pub map_pin: Option<(i32, i32)>,
    pub vignette: Option<String>,
}

use super::helpers::{ensure_namespace, validate_supplied_relation_id};
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::closure::RustFn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const METHODS: &[(&str, RustFn)] = &[
    ("GetSuperTrackedQuestID", quest_id),
    ("GetSuperTrackedContent", content),
    ("GetSuperTrackedMapPin", map_pin),
    ("GetSuperTrackedVignette", vignette),
    ("IsSuperTrackingAnything", anything),
    ("SetSuperTrackedQuestID", set_quest),
    ("SetSuperTrackedContent", set_content),
    ("ClearSuperTrackedContent", clear_content),
    ("ClearSuperTrackedMapPin", clear_map_pin),
    ("ClearAllSuperTracked", clear_all),
];

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_SuperTrack")?;
    for &(name, function) in METHODS {
        table_set_rust_fn_static(state, namespace, name, function)?;
    }
    Ok(())
}

fn supplied_id(state: &LuaState, index: i32) -> LuaResult<i32> {
    validate_supplied_relation_id(stack_val(state, index), "SuperTrack ID must be a number")?
        .ok_or_else(|| rilua::runtime_error("SuperTrack ID must be a finite i32 integer"))
}

fn quest_id(state: &mut LuaState) -> LuaResult<u32> {
    let quest = borrow_state(state)?.super_track.quest_id;
    // Documented-optional representation, not established native sentinel parity.
    state.push(quest.map_or(Val::Nil, |id| Val::Num(f64::from(id))));
    Ok(1)
}

fn push_pair(state: &mut LuaState, pair: Option<(i32, i32)>) -> LuaResult<u32> {
    let Some((kind, id)) = pair else { return Ok(0) };
    state.push(Val::Num(f64::from(kind)));
    state.push(Val::Num(f64::from(id)));
    Ok(2)
}

fn content(state: &mut LuaState) -> LuaResult<u32> {
    let pair = borrow_state(state)?.super_track.content;
    push_pair(state, pair)
}

fn map_pin(state: &mut LuaState) -> LuaResult<u32> {
    let pair = borrow_state(state)?.super_track.map_pin;
    push_pair(state, pair)
}

fn vignette(state: &mut LuaState) -> LuaResult<u32> {
    let guid = borrow_state(state)?.super_track.vignette.clone();
    let value = guid.map_or(Val::Nil, |guid| create_string(state, &guid));
    state.push(value);
    Ok(1)
}

fn anything(state: &mut LuaState) -> LuaResult<u32> {
    let active = {
        let sim = borrow_state(state)?;
        let tracked = &sim.super_track;
        [
            tracked.quest_id.is_some(),
            tracked.content.is_some(),
            tracked.map_pin.is_some(),
            tracked.vignette.is_some(),
        ]
        .into_iter()
        .any(|active| active)
    };
    state.push(Val::Bool(active));
    Ok(1)
}

/// Simulator policy: publish after releasing the state borrow, on every mutation call.
/// Native timing, equal-write deduplication and cross-target arbitration are unknown.
fn publish_change(state: &mut LuaState) -> LuaResult<u32> {
    dispatch_event_now(state, "SUPER_TRACKING_CHANGED", &[])?;
    Ok(0)
}

fn set_quest(state: &mut LuaState) -> LuaResult<u32> {
    let id = supplied_id(state, 1)?;
    borrow_state_mut(state)?.super_track.quest_id = (id != 0).then_some(id);
    publish_change(state)
}

fn set_content(state: &mut LuaState) -> LuaResult<u32> {
    let pair = (supplied_id(state, 1)?, supplied_id(state, 2)?);
    borrow_state_mut(state)?.super_track.content = Some(pair);
    publish_change(state)
}

fn clear_content(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.super_track.content = None;
    publish_change(state)
}

fn clear_map_pin(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.super_track.map_pin = None;
    publish_change(state)
}

fn clear_all(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.super_track = SuperTrackState::default();
    publish_change(state)
}
