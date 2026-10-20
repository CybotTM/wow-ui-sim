//! INFERRED host-declared classifications, not a native spell catalog.
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::table_security::{SecretPayloadKind, inspect_host_secret_number, secret_payload_kind};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Default)]
pub struct SpellClassification {
    pub consumable: bool,
    pub crowd_control: bool,
    pub external_defensive: bool,
}

pub type SpellClassifications = HashMap<u32, SpellClassification>;

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    let ns = super::ensure_namespace(state, "C_Spell")?;
    for (name, function) in [
        ("IsSpellImportant", important as rilua::RustFn),
        ("IsSpellCrowdControl", crowd_control),
        ("IsConsumableSpell", consumable),
        ("IsExternalDefensive", external_defensive),
    ] {
        table_set_rust_fn_static(state, ns, name, function)?;
    }
    Ok(())
}

fn read_tainted_numeric_id(state: &LuaState) -> LuaResult<Option<u32>> {
    let value = stack_val(state, 1);
    let enabled = cfg!(all(
        feature = "retail-12-0-5",
        any(feature = "profile-retail", feature = "client-ptr")
    ));
    if enabled && secret_payload_kind(state, value) == Some(SecretPayloadKind::Number) {
        return inspect_host_secret_number(state, value, |number| {
            Ok(super::c_spell::numeric_spell_id_value(
                state,
                Val::Num(number),
            ))
        });
    }
    read_id(state)
}

fn read_id(state: &LuaState) -> LuaResult<Option<u32>> {
    // ExternalDefensive and non-numeric payloads retain VM authentication.
    let value = rilua::table_security::unwrap_secret(state, stack_val(state, 1))?;
    // Use the same number/name lookup as GetSpellName and the LoC duration API.
    // Alias override selection is outside this classification input contract.
    Ok(super::c_spell::numeric_spell_id_value(state, value))
}

fn important(state: &mut LuaState) -> LuaResult<u32> {
    let id = read_tainted_numeric_id(state)?;
    let sim = borrow_state(state)?;
    let important = id.is_some_and(|id| {
        sim.aura_filter_facts
            .important_spell_ids
            .contains(&(id as i32))
    });
    drop(sim);
    state.push(Val::Bool(important));
    Ok(1)
}

fn classify(state: &LuaState, id: Option<u32>) -> LuaResult<SpellClassification> {
    let sim = borrow_state(state)?;
    Ok(id
        .and_then(|id| sim.spell_classifications.get(&id).copied())
        .unwrap_or_default())
}
fn crowd_control(state: &mut LuaState) -> LuaResult<u32> {
    let id = read_tainted_numeric_id(state)?;
    let value = classify(state, id)?.crowd_control;
    state.push(Val::Bool(value));
    Ok(1)
}
fn consumable(state: &mut LuaState) -> LuaResult<u32> {
    let id = read_tainted_numeric_id(state)?;
    let value = classify(state, id)?.consumable;
    state.push(Val::Bool(value));
    Ok(1)
}
fn external_defensive(state: &mut LuaState) -> LuaResult<u32> {
    let id = read_id(state)?;
    let value = classify(state, id)?.external_defensive;
    state.push(Val::Bool(value));
    Ok(1)
}
