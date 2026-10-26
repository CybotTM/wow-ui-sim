//! Wrath-only legacy positional currency API; modern currency state is independent.

use super::{CurrencyListState, CurrencyToken};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let methods: &[(&str, rilua::RustFn)] = &[
        ("GetCurrencyListSize", list_size),
        ("GetCurrencyListInfo", list_info),
        ("ExpandCurrencyList", expand),
        ("SetCurrencyBackpack", watch),
        ("GetBackpackCurrencyInfo", backpack_info),
        ("GetNumWatchedTokens", watched_count),
    ];
    for &(name, function) in methods {
        table_set_rust_fn_static(state, state.global, name, function)?;
    }
    Ok(())
}

fn list_size(state: &mut LuaState) -> LuaResult<u32> {
    let size = borrow_state(state)?.wrath_currency.displayed_rows().count();
    state.push(Val::Num(size as f64));
    Ok(1)
}

fn list_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let row = {
        let sim = borrow_state(state)?;
        sim.wrath_currency
            .displayed_row(index)
            .map(|(category, token)| {
                let category = &sim.wrath_currency.categories[category];
                (
                    category.name.clone(),
                    category.expanded,
                    token.map(|i| category.currencies[i].clone()),
                )
            })
    };
    let Some((name, expanded, token)) = row else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let header = token.is_none();
    let token = token.unwrap_or(CurrencyToken {
        name,
        ..Default::default()
    });
    push_list_row(state, &token, header, header && expanded);
    Ok(9)
}

fn push_list_row(state: &mut LuaState, token: &CurrencyToken, header: bool, expanded: bool) {
    let name = create_string(state, &token.name);
    let icon = create_string(state, &token.icon);
    for value in [
        name,
        Val::Bool(header),
        Val::Bool(expanded),
        Val::Bool(false),
        Val::Bool(token.watched),
        Val::Num(token.count as f64),
        Val::Num(token.extra_currency_type as f64),
        icon,
        token.item_id.map_or(Val::Nil, |id| Val::Num(id as f64)),
    ] {
        state.push(value);
    }
}

fn expand(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let expanded = numeric_flag(state);
    let mut sim = borrow_state_mut(state)?;
    if let Some((category, None)) = sim.wrath_currency.displayed_row(index) {
        sim.wrath_currency.categories[category].expanded = expanded;
    }
    Ok(0)
}

fn watch(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let watched = numeric_flag(state);
    let mut sim = borrow_state_mut(state)?;
    if let Some((category, Some(token))) = sim.wrath_currency.displayed_row(index) {
        sim.wrath_currency.categories[category].currencies[token].watched = watched;
    }
    Ok(0)
}

fn numeric_flag(state: &LuaState) -> bool {
    matches!(stack_val(state, 2), Val::Num(value) if value != 0.0)
}

fn backpack_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let token = {
        let sim = borrow_state(state)?;
        match index {
            1..=3 => watched_tokens(&sim.wrath_currency)
                .nth((index - 1) as usize)
                .cloned(),
            _ => None,
        }
    };
    let Some(token) = token else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let name = create_string(state, &token.name);
    let icon = create_string(state, &token.icon);
    for value in [
        name,
        Val::Num(token.count as f64),
        Val::Num(token.extra_currency_type as f64),
        icon,
        token.item_id.map_or(Val::Nil, |id| Val::Num(id as f64)),
    ] {
        state.push(value);
    }
    Ok(5)
}

fn watched_tokens(list: &CurrencyListState) -> impl Iterator<Item = &CurrencyToken> {
    list.categories
        .iter()
        .flat_map(|category| &category.currencies)
        .filter(|token| token.watched)
}

fn watched_count(state: &mut LuaState) -> LuaResult<u32> {
    let count = watched_tokens(&borrow_state(state)?.wrath_currency).count();
    state.push(Val::Num(count as f64));
    Ok(1)
}
