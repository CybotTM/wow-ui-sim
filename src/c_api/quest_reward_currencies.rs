//! Shared rich-state reward currency projection for the two quest namespaces.

use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::table_security::unwrap_secret;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    if !super::quest_objectives::ENABLED {
        return Ok(());
    }
    let namespace = super::ensure_namespace(state, "C_QuestInfoSystem")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetQuestRewardCurrencies",
        get_quest_reward_currencies,
    )
}

// QuestInfoSystemDocumentation.lua:58-72 permits nil and no result;
// unlike C_QuestLog's required ID. Nil uses this simulator's selected context.
fn get_quest_reward_currencies(state: &mut LuaState) -> LuaResult<u32> {
    let original = unwrap_secret(state, stack_val(state, 1))?;
    let quest_id = match original {
        Val::Nil => borrow_state(state)?
            .selected_quest_log_id
            .and_then(|id| i32::try_from(id).ok()),
        Val::Num(id)
            if id.is_finite() && id.fract() == 0.0 && (0.0..=f64::from(u32::MAX)).contains(&id) =>
        {
            i32::try_from(id as u32).ok()
        }
        _ => {
            return Err(rilua::runtime_error(
                "C_QuestInfoSystem.GetQuestRewardCurrencies: questID must be a finite integral u32 number or nil",
            ));
        }
    };
    let Some(quest_id) = quest_id else {
        return Ok(0);
    };
    push_currency_rewards(state, quest_id)
}

/// Both namespaces read and serialize the same supplied records.
pub(crate) fn push_currency_rewards(state: &mut LuaState, quest_id: i32) -> LuaResult<u32> {
    let currencies = {
        let sim = borrow_state(state)?;
        sim.quest_log_entries
            .entries
            .iter()
            .find(|entry| entry.quest_id == quest_id)
            .map(|entry| entry.currency_rewards.clone())
            .unwrap_or_default()
    };
    let list = create_table(state);
    let Val::Table(list_ref) = list else {
        return Ok(0);
    };
    for (index, currency) in currencies.iter().enumerate() {
        let entry = build_currency_reward_table(state, currency);
        if let Some(table) = state.gc.tables.get_mut(list_ref) {
            let _ = table.raw_set(Val::Num((index + 1) as f64), entry, &state.gc.string_arena);
        }
        state.gc.barrier_back(list_ref);
    }
    state.push(list);
    Ok(1)
}

fn build_currency_reward_table(
    state: &mut LuaState,
    currency: &crate::lua_api::state::QuestRewardCurrency,
) -> Val {
    let entry = create_table(state);
    let name = create_string(state, &currency.name);
    let texture = create_string(state, &currency.texture);
    table_set(
        state,
        entry,
        "currencyID",
        Val::Num(currency.currency_id as f64),
    );
    table_set(state, entry, "name", name);
    table_set(state, entry, "texture", texture);
    table_set(
        state,
        entry,
        "totalRewardAmount",
        Val::Num(currency.total_reward_amount as f64),
    );
    if let Some(base) = currency.base_reward_amount {
        table_set(state, entry, "baseRewardAmount", Val::Num(base as f64));
    }
    entry
}
