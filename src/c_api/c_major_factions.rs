//! `C_MajorFactions` Renown surface consumed by
//! `Blizzard_ActionBar/Shared/ReputationBar.lua`.
//!
//! State sources:
//!
//! - `state.major_factions: HashMap<factionID, MajorFactionData>` —
//!   `GetMajorFactionData(factionID)` returns the matching row, or nil when
//!   the id isn't registered. `ReputationStatusBarMixin:Update` reads
//!   `renownLevel` / `renownLevelThreshold` to drive the bar's blue Renown
//!   look.
//! - `state.major_faction_renown_levels: HashMap<factionID, Vec<RenownLevelInfo>>`
//!   — `GetRenownLevels(factionID)` returns a Lua sequence built from the
//!   matching vec. The mixin uses the **last** entry's `level` to clamp the
//!   bar via `:GetMaxLevel()`. An unknown id yields an empty sequence.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::{
    borrow_state, create_string, create_table, create_table_with_fields, table_set_num,
};
use crate::lua_api::state::{MajorFactionData, RenownLevelInfo};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_major_factions_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_MajorFactions")?;
    table_set_rust_fn_static(state, ns, "GetMajorFactionData", get_major_faction_data)?;
    table_set_rust_fn_static(state, ns, "GetRenownLevels", get_renown_levels)?;
    Ok(())
}

fn get_major_faction_data(state: &mut LuaState) -> LuaResult<u32> {
    let Ok(faction_id) = i64::from_stack(state, 1) else {
        return Ok(0);
    };
    let Some(data) = borrow_state(state)?.major_factions.get(&faction_id).cloned() else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let table = build_major_faction_data_table(state, &data);
    state.push(table);
    Ok(1)
}

fn get_renown_levels(state: &mut LuaState) -> LuaResult<u32> {
    let Ok(faction_id) = i64::from_stack(state, 1) else {
        return Ok(0);
    };
    let levels = borrow_state(state)?
        .major_faction_renown_levels
        .get(&faction_id)
        .cloned()
        .unwrap_or_default();
    let sequence = create_table(state);
    let Val::Table(sequence_ref) = sequence else {
        unreachable!("create_table must return a table");
    };
    for (index, level) in levels.iter().enumerate() {
        let entry = build_renown_level_entry(state, level);
        table_set_num(state, sequence_ref, (index + 1) as f64, entry);
    }
    state.push(sequence);
    Ok(1)
}

fn build_major_faction_data_table(state: &mut LuaState, data: &MajorFactionData) -> Val {
    let name = create_string(state, &data.name);
    let texture_kit = create_string(state, &data.texture_kit);
    let unlock_description = optional_string_val(state, data.unlock_description.as_ref());
    let faction_id = Val::Num(data.faction_id as f64);
    let expansion_filter = Val::Num(data.expansion_filter as f64);
    let renown_level = Val::Num(data.renown_level as f64);
    let renown_reputation_earned = Val::Num(data.renown_reputation_earned as f64);
    let renown_level_threshold = Val::Num(data.renown_level_threshold as f64);
    let celebration_sound_kit = Val::Num(data.celebration_sound_kit as f64);
    let renown_fanfare_sound_kit_id = Val::Num(data.renown_fanfare_sound_kit_id as f64);
    create_table_with_fields(
        state,
        &[
            ("factionID", faction_id),
            ("name", name),
            ("expansionFilter", expansion_filter),
            ("renownLevel", renown_level),
            ("renownReputationEarned", renown_reputation_earned),
            ("renownLevelThreshold", renown_level_threshold),
            ("isUnlocked", Val::Bool(data.is_unlocked)),
            ("unlockDescription", unlock_description),
            ("celebrationSoundKit", celebration_sound_kit),
            ("renownFanfareSoundKitID", renown_fanfare_sound_kit_id),
            ("textureKit", texture_kit),
        ],
    )
}

fn optional_string_val(state: &mut LuaState, text: Option<&String>) -> Val {
    match text {
        Some(s) => create_string(state, s),
        None => Val::Nil,
    }
}

fn build_renown_level_entry(state: &mut LuaState, level: &RenownLevelInfo) -> Val {
    create_table_with_fields(
        state,
        &[
            ("factionID", Val::Num(level.faction_id as f64)),
            ("level", Val::Num(level.level as f64)),
            ("locked", Val::Bool(level.locked)),
            ("isMilestone", Val::Bool(level.is_milestone)),
            ("isCapstone", Val::Bool(level.is_capstone)),
        ],
    )
}
