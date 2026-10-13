//! Environment-owned C_PetJournal hover selection.

use crate::lua_api::methods::{borrow_state_mut, val_to_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::LuaResult;
use rilua::vm::state::LuaState;

/// Environment-owned hover selection; default None is simulator policy, not native proof.
#[derive(Debug, Default)]
pub struct PetJournalHoverState {
    pub hovered_battle_pet_guid: Option<String>,
}

/// Install on the legacy-created namespace after its other members are registered.
pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = super::ensure_namespace(state, "C_PetJournal")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "SetHoveredBattlePet",
        set_hovered_battle_pet,
    )?;
    table_set_rust_fn_static(
        state,
        namespace,
        "ClearHoveredBattlePet",
        clear_hovered_battle_pet,
    )
}

fn set_hovered_battle_pet(state: &mut LuaState) -> LuaResult<u32> {
    // AllowedWhenUntainted: authenticate the original argument before extraction.
    let value = rilua::table_security::unwrap_secret(state, stack_val(state, 1))?;
    let guid = val_to_string(state, value).ok_or_else(|| {
        rilua::runtime_error(
            "C_PetJournal.SetHoveredBattlePet requires a UTF-8 string battlePetGUID",
        )
    })?;
    borrow_state_mut(state)?
        .pet_journal_hover
        .hovered_battle_pet_guid = Some(guid);
    Ok(0)
}

fn clear_hovered_battle_pet(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?
        .pet_journal_hover
        .hovered_battle_pet_guid = None;
    Ok(0)
}
