//! State-backed action-highlight bookkeeping globals consumed by `Blizzard_ActionBar`.
//!
//! Mirrors `Blizzard_ActionBar/Shared/ActionButton.lua` lines 27-107:
//!
//! - `MarkNewActionHighlight(action)` / `ClearNewActionHighlight(action, preventClearingIdentical)`
//!   / `GetNewActionHighlightMark(action)` — `ACTION_HIGHLIGHT_MARKS` table
//! - `ClearOnBarHighlightMarks()` / `GetOnBarHighlightMark(action)` —
//!   `ON_BAR_HIGHLIGHT_MARKS` table (action → kind)
//! - `UpdateOnBarHighlightMarksBySpell(spellID)` /
//!   `UpdateOnBarHighlightMarksByFlyout(flyoutID)` /
//!   `UpdateOnBarHighlightMarksByPetAction(petAction)` — rebuild the on-bar
//!   marks by scanning `state.action_bars` for slots whose bound spell matches
//!   the highlighted source.
//! - `GetActionButtonForID(id)` — resolves to the `ActionButton{id}` global.
//!
//! The simulator stores both maps in `SimState.action_highlights` (HashSet of
//! action ids for "new" marks, HashMap<i32, ActionHighlightKind> for "on bar"
//! marks) so reads are pure-Rust without indexing Lua tables.

use crate::lua_api::SimState;
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_api::state::ActionHighlightKind;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn stack_u32(state: &LuaState, index: i32) -> Option<u32> {
    match stack_val(state, index) {
        Val::Num(n) if n >= 0.0 => Some(n as u32),
        _ => None,
    }
}

fn stack_bool(state: &LuaState, index: i32) -> bool {
    matches!(stack_val(state, index), Val::Bool(true))
}

/// Slots in `state.action_bars` whose bound spell matches `spell_id`.
fn slots_bound_to_spell(sim: &SimState, spell_id: u32) -> Vec<i32> {
    sim.action_bars
        .iter()
        .filter(|(_, bound)| **bound == spell_id)
        .map(|(slot, _)| *slot as i32)
        .collect()
}

fn rebuild_on_bar_marks(state: &mut LuaState, spell_id: u32, kind: ActionHighlightKind) {
    let Ok(mut sim) = borrow_state_mut(state) else {
        return;
    };
    let slots = slots_bound_to_spell(&sim, spell_id);
    sim.action_highlights.on_bar.clear();
    for slot in slots {
        sim.action_highlights.on_bar.insert(slot, kind);
    }
}

/// `MarkNewActionHighlight(action)` — flags `action` as a brand-new ability
/// so the button glow path can render the "new" badge.
fn mark_new_action_highlight(state: &mut LuaState) -> LuaResult<u32> {
    let Some(action) = stack_i32(state, 1) else {
        return Ok(0);
    };
    borrow_state_mut(state)?
        .action_highlights
        .new
        .insert(action);
    Ok(0)
}

/// `ClearNewActionHighlight(action, preventIdenticalActionsFromClearing)` —
/// drops `action`'s new mark. When `preventIdenticalActionsFromClearing` is
/// false, also clears every other slot whose bound spell matches (Blizzard
/// behavior: re-cast/hover suppresses the badge across duplicate slots).
fn clear_new_action_highlight(state: &mut LuaState) -> LuaResult<u32> {
    let Some(action) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let prevent_identical = stack_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    sim.action_highlights.new.remove(&action);
    if prevent_identical {
        return Ok(0);
    }
    let Some(spell_id) = sim.action_bars.get(&(action as u32)).copied() else {
        return Ok(0);
    };
    let identical_slots: std::collections::HashSet<i32> = sim
        .action_bars
        .iter()
        .filter(|(_, bound)| **bound == spell_id)
        .map(|(slot, _)| *slot as i32)
        .collect();
    sim.action_highlights
        .new
        .retain(|slot| !identical_slots.contains(slot));
    Ok(0)
}

/// `GetNewActionHighlightMark(action)` — true when `action` is currently
/// flagged for a new-ability badge, nil otherwise.
fn get_new_action_highlight_mark(state: &mut LuaState) -> LuaResult<u32> {
    let Some(action) = stack_i32(state, 1) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let value = match borrow_state(state)?.action_highlights.new.get(&action) {
        Some(_) => Val::Bool(true),
        None => Val::Nil,
    };
    state.push(value);
    Ok(1)
}

/// `ClearOnBarHighlightMarks()` — wipes the on-bar highlight overlay.
fn clear_on_bar_highlight_marks(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.action_highlights.on_bar.clear();
    Ok(0)
}

/// `GetOnBarHighlightMark(action)` — returns `(true, "spell"|"flyout"|"petaction")`
/// when `action`'s slot currently has an on-bar highlight, nil otherwise.
fn get_on_bar_highlight_mark(state: &mut LuaState) -> LuaResult<u32> {
    let Some(action) = stack_i32(state, 1) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let kind = borrow_state(state)?
        .action_highlights
        .on_bar
        .get(&action)
        .copied();
    let Some(kind) = kind else {
        state.push(Val::Nil);
        return Ok(1);
    };
    state.push(Val::Bool(true));
    let tag = LuaApiMut::create_string(state, kind.type_tag().as_bytes());
    state.push(tag);
    Ok(2)
}

/// `UpdateOnBarHighlightMarksBySpell(spellID)` — rebuilds the on-bar marks by
/// scanning `state.action_bars` for slots bound to `spellID`.
fn update_on_bar_highlight_marks_by_spell(state: &mut LuaState) -> LuaResult<u32> {
    let Some(spell_id) = stack_u32(state, 1) else {
        return Ok(0);
    };
    rebuild_on_bar_marks(state, spell_id, ActionHighlightKind::Spell);
    Ok(0)
}

/// `UpdateOnBarHighlightMarksByFlyout(flyoutID)` — same shape as the spell
/// variant. The simulator does not model flyout→spell membership, so we
/// reuse the spell-id match (matches the simplest Blizzard data shape: a
/// flyout id collides with a single spell id when the bar slot is the
/// flyout itself).
fn update_on_bar_highlight_marks_by_flyout(state: &mut LuaState) -> LuaResult<u32> {
    let Some(flyout_id) = stack_u32(state, 1) else {
        return Ok(0);
    };
    rebuild_on_bar_marks(state, flyout_id, ActionHighlightKind::Flyout);
    Ok(0)
}

/// `UpdateOnBarHighlightMarksByPetAction(petAction)` — same shape, tagged as
/// pet action.
fn update_on_bar_highlight_marks_by_pet_action(state: &mut LuaState) -> LuaResult<u32> {
    let Some(pet_action) = stack_u32(state, 1) else {
        return Ok(0);
    };
    rebuild_on_bar_marks(state, pet_action, ActionHighlightKind::PetAction);
    Ok(0)
}

/// `GetActionButtonForID(id)` — resolves to the `ActionButton{id}` global.
/// Real WoW also routes through `OverrideActionBar` while it is shown, but
/// the simulator does not implement that path.
fn get_action_button_for_id(state: &mut LuaState) -> LuaResult<u32> {
    let Some(id) = stack_u32(state, 1) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let button = LuaApiMut::get_global_val(state, &format!("ActionButton{id}"));
    state.push(button);
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "MarkNewActionHighlight", mark_new_action_highlight)?;
    LuaApiMut::register_function(lua, "ClearNewActionHighlight", clear_new_action_highlight)?;
    LuaApiMut::register_function(
        lua,
        "GetNewActionHighlightMark",
        get_new_action_highlight_mark,
    )?;
    LuaApiMut::register_function(
        lua,
        "ClearOnBarHighlightMarks",
        clear_on_bar_highlight_marks,
    )?;
    LuaApiMut::register_function(lua, "GetOnBarHighlightMark", get_on_bar_highlight_mark)?;
    LuaApiMut::register_function(
        lua,
        "UpdateOnBarHighlightMarksBySpell",
        update_on_bar_highlight_marks_by_spell,
    )?;
    LuaApiMut::register_function(
        lua,
        "UpdateOnBarHighlightMarksByFlyout",
        update_on_bar_highlight_marks_by_flyout,
    )?;
    LuaApiMut::register_function(
        lua,
        "UpdateOnBarHighlightMarksByPetAction",
        update_on_bar_highlight_marks_by_pet_action,
    )?;
    LuaApiMut::register_function(lua, "GetActionButtonForID", get_action_button_for_id)?;
    Ok(())
}
