//! Host-supplied Forever inputs. These are simulator fixtures, not native defaults.

use std::collections::{HashMap, HashSet};

use crate::c_api::helpers::ensure_namespace;
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

/// Explicit inputs for 70532 consumers whose native policy is not established.
/// Empty collections mean the host supplied no entries; false flags describe
/// the empty simulator world, not an inferred native account or character.
#[derive(Debug, Default)]
pub struct StartupInputs {
    pub ammo_required: bool,
    pub cursor_visible: bool,
    pub ping_enabled: bool,
    pub purchasable_bank_types: HashSet<i32>,
    pub match_complete: bool,
    pub target_spell_id: Option<i32>,
    pub character_tts_settings_saved: bool,
    pub temporary_cvars: HashMap<String, String>,
    pub external_event: Option<ExternalEvent>,
    pub group_buff_alerts: Vec<GroupBuffAlert>,
}

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    if ACTIVE != ClientProfile::WowForever {
        return Ok(());
    }
    let paper_doll = ensure_namespace(state, "C_PaperDollInfo")?;
    table_set_rust_fn_static(state, paper_doll, "AmmoNeeded", ammo_needed)
}

fn ammo_needed(state: &mut LuaState) -> LuaResult<u32> {
    let required = borrow_state(state)?.forever_startup.ammo_required;
    state.push(Val::Bool(required));
    Ok(1)
}

#[derive(Debug)]
pub struct ExternalEvent {
    pub url: String,
    pub can_show: bool,
    pub is_new: bool,
}

#[derive(Debug)]
pub struct GroupBuffAlert {
    pub spell_id: i32,
    pub visual_value: i32,
}
