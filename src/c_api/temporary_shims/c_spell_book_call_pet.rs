//! SpellBook temporary static shims.
//!
//! Pet call spell metadata is not seeded yet. Blizzard and addon code can
//! probe the legacy global, so keep the current no-call-pet result explicit
//! here until pet spell data is modeled. Spell override replacement is also
//! unmodeled; `C_SpellBook` keeps the same identity fallback as `C_Spell`.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::globals::spellbook_data;
use crate::lua_api::methods::{create_table, table_set_static};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::LuaResult;
use rilua::Val;
use rilua::vm::state::LuaState;

pub(crate) fn register_spell_book_static_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_SpellBook")?;
    table_set_rust_fn_static(state, ns, "GetOverrideSpell", get_override_spell)?;
    table_set_rust_fn_static(state, ns, "FindSpellOverrideByID", get_override_spell)?;
    table_set_rust_fn_static(state, ns, "FindFlyoutSlotBySpellID", return_no_values)?;
    table_set_rust_fn_static(state, ns, "FindBaseSpellByID", return_no_values)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetSpellBookItemAutoCast",
        get_spell_book_item_auto_cast,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetSpellBookItemLossOfControlCooldownInfo",
        get_spell_book_item_loss_of_control_cooldown_info,
    )?;
    table_set_rust_fn_static(
        state,
        state.global,
        "GetCallPetSpellInfo",
        get_call_pet_spell_info,
    )?;
    Ok(())
}

fn get_override_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Num(spell_id as f64));
    Ok(1)
}

fn return_no_values(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn get_call_pet_spell_info(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

fn get_spell_book_item_auto_cast(state: &mut LuaState) -> LuaResult<u32> {
    let _slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    Ok(2)
}

fn get_spell_book_item_loss_of_control_cooldown_info(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    let _spell_bank = Option::<i32>::from_stack(state, 2)?;
    if spellbook_data::get_spell_at_slot(slot).is_none() {
        state.push(Val::Nil);
        return Ok(1);
    }
    let info = create_table(state);
    table_set_static(state, info, "isActive", Val::Bool(false));
    table_set_static(state, info, "startTime", Val::Num(0.0));
    table_set_static(state, info, "duration", Val::Num(0.0));
    table_set_static(state, info, "modRate", Val::Num(1.0));
    table_set_static(state, info, "shouldReplaceNormalCooldown", Val::Bool(false));
    state.push(info);
    Ok(1)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn deprecated_spell_book_targets_are_explicit_noops_or_identity_fallbacks() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let result: (i32, i32, i32) = env
            .eval(
                r##"
                return C_SpellBook.FindSpellOverrideByID(116),
                    select("#", C_SpellBook.FindFlyoutSlotBySpellID(116)),
                    select("#", C_SpellBook.FindBaseSpellByID(116))
                "##,
            )
            .expect("deprecated spellbook targets should be callable");

        assert_eq!(result, (116, 0, 0));
    }
}
