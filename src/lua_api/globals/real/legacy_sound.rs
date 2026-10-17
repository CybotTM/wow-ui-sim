//! Wrath's legacy global name adapter; C_Sound stays numeric-only.

use crate::c_api::c_sound;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

pub(crate) fn play_sound(state: &mut LuaState) -> LuaResult<u32> {
    let Val::Str(string_ref) = stack_val(state, 1) else {
        return c_sound::play_sound(state);
    };
    let name = state
        .gc
        .string_arena
        .get(string_ref)
        .ok_or_else(|| runtime_error("PlaySound: sound name was collected"))?
        .data()
        .to_vec();
    let sound_kit_id = resolve_unique_legacy_kit(&name);
    c_sound::request_named_sound(state, name, sound_kit_id)
}

fn resolve_unique_legacy_kit(name: &[u8]) -> Option<u32> {
    // User-approved simulator policy, not native lookup/error semantics.
    // Discovery: public Flowy/cmangosDB 3.3.5a SoundEntries dump and
    // WoWDBDefs SoundEntries.dbd; no original DBC checksum verification.
    // INTERFACESOUND_LOSTTARGETUNIT has two rows (684/900): name-only.
    // Unknown names also remain name-only; no playback or lookup error.
    // See docs/specs/wrath-legacy-sound.md for source citations and bounds.
    match name {
        b"WriteQuest" => Some(3093),
        b"igAbiliityPageTurn" => Some(836),
        b"igMainMenuOptionCheckBoxOn" => Some(856),
        _ => None,
    }
}
