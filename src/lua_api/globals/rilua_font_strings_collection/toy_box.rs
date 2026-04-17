//! C_ToyBox namespace.

use crate::lua_api::rilua_methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::{FromStack, IntoStack, TableBuilder};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

use super::set_global_val;

fn push_toy_info(state: &mut LuaState, tid: f64, name: &str, icon: f64) -> u32 {
    let name_val = create_string(state, name);
    state.push(Val::Num(tid));
    state.push(name_val);
    state.push(Val::Num(icon));
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    state.push(Val::Num(1.0));
    6
}

fn register_toy_queries(b: TableBuilder) -> LuaResult<TableBuilder> {
    b.set_function("GetNumTotalDisplayedToys", |state| {
        let st = borrow_state(state)?;
        let n = st.world.toys.len() as i32;
        drop(st);
        n.into_stack(state)
    })?
    .set_function("GetNumLearnedDisplayedToys", |state| {
        let st = borrow_state(state)?;
        let n = st.world.toys.iter().filter(|t| t.is_collected).count() as i32;
        drop(st);
        n.into_stack(state)
    })?
    .set_function("GetNumToys", |state| {
        let st = borrow_state(state)?;
        let n = st.world.toys.len() as i32;
        drop(st);
        n.into_stack(state)
    })?
    .set_function("GetNumFilteredToys", |state| {
        let st = borrow_state(state)?;
        let n = st.world.toys.len() as i32;
        drop(st);
        n.into_stack(state)
    })?
    .set_function("GetToyFromIndex", |state| {
        let index = i32::from_stack(state, 1)?;
        let st = borrow_state(state)?;
        let i = (index - 1) as usize;
        let id = st.world.toys.get(i).map_or(0i32, |t| t.item_id as i32);
        drop(st);
        id.into_stack(state)
    })?
    .set_function("GetToyInfo", |state| {
        let item_id = u32::from_stack(state, 1)?;
        let st = borrow_state(state)?;
        let Some(toy) = st.world.toys.iter().find(|t| t.item_id == item_id) else {
            drop(st);
            return Ok(0);
        };
        let tid = toy.item_id as f64;
        let name = toy.name.clone();
        let icon = toy.icon as f64;
        drop(st);
        Ok(push_toy_info(state, tid, &name, icon))
    })?
    .set_function("IsToyUsable", |state| {
        let item_id = i32::from_stack(state, 1)?;
        let st = borrow_state(state)?;
        let usable = st
            .world
            .toys
            .iter()
            .find(|t| t.item_id == item_id as u32)
            .map(|t| t.is_usable)
            .unwrap_or(false);
        drop(st);
        usable.into_stack(state)
    })?
    .set_function("GetToyLink", |state| {
        let item_id = i32::from_stack(state, 1)?;
        let st = borrow_state(state)?;
        let link = st
            .world
            .toys
            .iter()
            .find(|t| t.item_id == item_id as u32)
            .map(|toy| {
                format!(
                    "|cff0070dd|Hitem:{}::::::::1:0|h[{}]|h|r",
                    toy.item_id, toy.name
                )
            });
        drop(st);
        match link {
            Some(s) => create_string(state, &s).into_stack(state),
            None => Val::Nil.into_stack(state),
        }
    })
}

fn register_toy_favorites(b: TableBuilder) -> LuaResult<TableBuilder> {
    b.set_function("GetIsFavorite", |state| {
        let item_id = i32::from_stack(state, 1)?;
        let st = borrow_state(state)?;
        let fav = st.world.favorite_toys.contains(&(item_id as u32));
        drop(st);
        fav.into_stack(state)
    })?
    .set_function("HasFavorites", |state| {
        let st = borrow_state(state)?;
        let has = !st.world.favorite_toys.is_empty();
        drop(st);
        has.into_stack(state)
    })?
    .set_function("SetIsFavorite", |state| {
        let item_id = i32::from_stack(state, 1)?;
        let is_fav = bool::from_stack(state, 2)?;
        let mut st = borrow_state_mut(state)?;
        if is_fav {
            st.world.favorite_toys.insert(item_id as u32);
        } else {
            st.world.favorite_toys.remove(&(item_id as u32));
        }
        Ok(0)
    })
}

fn register_toy_filter_stubs(b: TableBuilder) -> LuaResult<TableBuilder> {
    b.set_function("GetCollectedShown", |state| true.into_stack(state))?
        .set_function("GetUncollectedShown", |state| true.into_stack(state))?
        .set_function("GetUnusableShown", |state| true.into_stack(state))?
        .set_function("SetCollectedShown", |_state| Ok(0))?
        .set_function("SetUncollectedShown", |_state| Ok(0))?
        .set_function("SetUnusableShown", |_state| Ok(0))?
        .set_function("ForceToyRefilter", |_state| Ok(0))
}

pub fn register_rilua_toy_box(lua: &mut rilua::Lua) -> LuaResult<()> {
    let b = TableBuilder::new(lua.state_mut());
    let b = register_toy_queries(b)?;
    let b = register_toy_favorites(b)?;
    let t = register_toy_filter_stubs(b)?.build();
    set_global_val(lua.state_mut(), "C_ToyBox", t);
    Ok(())
}
