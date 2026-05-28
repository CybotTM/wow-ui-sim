//! State-backed glyph cursor globals consumed by
//! `Blizzard_ActionBar/Shared/SpellFlyout.lua`'s
//! `SpellFlyoutPopupButtonMixin:UpdateGlyphState` and the spellbook glyph
//! attach flow. All read from / write to `state.glyph: GlyphState`.
//!
//! - `HasPendingGlyphCast()`        → `glyph.pending_glyph_name.is_some()`.
//! - `HasAttachedGlyph(spellID)`    → `glyph.attached_glyphs.contains_key`.
//! - `IsPendingGlyphRemoval()`      → `glyph.pending_glyph_removal`.
//! - `GetCurrentGlyphNameForSpell(spellID)` → name from `attached_glyphs`.
//! - `GetPendingGlyphName()`        → `glyph.pending_glyph_name`.
//! - `AttachGlyphToSpell(spellID)`  → moves the pending glyph onto the
//!   spell. Removal pending → erases an existing entry. No-op when no
//!   glyph is on the cursor. Always clears the pending state and fires
//!   `GLYPH_ADDED` (or `GLYPH_REMOVED` for the removal flow) so the flyout
//!   refreshes.
//! - `IsSpellValidForPendingGlyph(spellID)` → true while a glyph is on
//!   the cursor. The simulator does not model per-spec glyph compatibility
//!   tables; matching live behavior would require glyph metadata we don't
//!   have, so any spell is considered valid as long as a glyph is pending.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_api::script_helpers::fire_named_event_state;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn has_pending_glyph_cast(state: &mut LuaState) -> LuaResult<u32> {
    let pending = borrow_state(state)?.glyph.pending_glyph_name.is_some();
    state.push(Val::Bool(pending));
    Ok(1)
}

fn has_attached_glyph(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = stack_i32(state, 1);
    let attached = match spell_id {
        Some(id) => borrow_state(state)?.glyph.attached_glyphs.contains_key(&id),
        None => false,
    };
    state.push(Val::Bool(attached));
    Ok(1)
}

fn is_pending_glyph_removal(state: &mut LuaState) -> LuaResult<u32> {
    let removal = borrow_state(state)?.glyph.pending_glyph_removal;
    state.push(Val::Bool(removal));
    Ok(1)
}

fn get_current_glyph_name_for_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = stack_i32(state, 1);
    let name = spell_id.and_then(|id| {
        borrow_state(state)
            .ok()
            .and_then(|sim| sim.glyph.attached_glyphs.get(&id).cloned())
    });
    push_optional_string(state, name.as_deref());
    Ok(1)
}

fn get_pending_glyph_name(state: &mut LuaState) -> LuaResult<u32> {
    let name = borrow_state(state)?.glyph.pending_glyph_name.clone();
    push_optional_string(state, name.as_deref());
    Ok(1)
}

fn is_spell_valid_for_pending_glyph(state: &mut LuaState) -> LuaResult<u32> {
    let pending = borrow_state(state)?.glyph.pending_glyph_name.is_some();
    state.push(Val::Bool(pending));
    Ok(1)
}

/// `AttachGlyphToSpell(spellID)` — consumes the cursor glyph. With removal
/// pending, the spell's `attached_glyphs` entry is cleared and `GLYPH_REMOVED`
/// fires; otherwise the pending name is moved into `attached_glyphs[spellID]`
/// and `GLYPH_ADDED` fires. Always clears the pending state so subsequent
/// `HasPendingGlyphCast()` returns false.
fn attach_glyph_to_spell(state: &mut LuaState) -> LuaResult<u32> {
    let Some(spell_id) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let event = {
        let mut sim = borrow_state_mut(state)?;
        let Some(name) = sim.glyph.pending_glyph_name.take() else {
            return Ok(0);
        };
        let was_removal = sim.glyph.pending_glyph_removal;
        sim.glyph.pending_glyph_removal = false;
        if was_removal {
            sim.glyph.attached_glyphs.remove(&spell_id);
            "GLYPH_REMOVED"
        } else {
            sim.glyph.attached_glyphs.insert(spell_id, name);
            "GLYPH_ADDED"
        }
    };
    let spell_arg = Val::Num(spell_id as f64);
    fire_named_event_state(state, event, &[spell_arg]);
    Ok(0)
}

fn push_optional_string(state: &mut LuaState, value: Option<&str>) {
    match value {
        Some(s) => {
            let v = create_string(state, s);
            state.push(v);
        }
        None => state.push(Val::Nil),
    }
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "HasPendingGlyphCast", has_pending_glyph_cast)?;
    LuaApiMut::register_function(lua, "HasAttachedGlyph", has_attached_glyph)?;
    LuaApiMut::register_function(lua, "IsPendingGlyphRemoval", is_pending_glyph_removal)?;
    LuaApiMut::register_function(
        lua,
        "GetCurrentGlyphNameForSpell",
        get_current_glyph_name_for_spell,
    )?;
    LuaApiMut::register_function(lua, "GetPendingGlyphName", get_pending_glyph_name)?;
    LuaApiMut::register_function(lua, "AttachGlyphToSpell", attach_glyph_to_spell)?;
    LuaApiMut::register_function(
        lua,
        "IsSpellValidForPendingGlyph",
        is_spell_valid_for_pending_glyph,
    )?;
    Ok(())
}
