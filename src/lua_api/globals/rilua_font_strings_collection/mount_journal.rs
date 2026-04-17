//! C_MountJournal namespace.

use crate::lua_api::rilua_methods::{borrow_state, create_string};
use crate::lua_bridge::{FromStack, IntoStack, TableBuilder};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

use super::set_global_val;

fn push_mount_info(
    state: &mut LuaState,
    name: &str,
    spell_id: f64,
    icon: f64,
    is_usable: bool,
    is_collected: bool,
    mount_id: f64,
) -> u32 {
    let name_val = create_string(state, name);
    state.push(name_val);
    state.push(Val::Num(spell_id));
    state.push(Val::Num(icon));
    state.push(Val::Bool(false));
    state.push(Val::Bool(is_usable));
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    state.push(Val::Nil);
    state.push(Val::Bool(false));
    state.push(Val::Bool(is_collected));
    state.push(Val::Num(mount_id));
    12
}

pub fn register_rilua_mount_journal(lua: &mut rilua::Lua) -> LuaResult<()> {
    let t = TableBuilder::new(lua.state_mut())
        .set_function("GetNumMounts", |state| {
            let st = borrow_state(state)?;
            let n = st.world.mounts.len() as i32;
            drop(st);
            n.into_stack(state)
        })?
        .set_function("GetNumDisplayedMounts", |state| {
            let st = borrow_state(state)?;
            let n = st.world.mounts.len() as i32;
            drop(st);
            n.into_stack(state)
        })?
        .set_function("GetDisplayedMountInfo", |state| {
            let index = i32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let i = (index - 1) as usize;
            let Some(m) = st.world.mounts.get(i) else {
                drop(st);
                return Ok(0);
            };
            let name = m.name.clone();
            let spell_id = m.spell_id as f64;
            let icon = m.icon as f64;
            let is_usable = m.is_usable;
            let is_collected = m.is_collected;
            let mount_id = m.mount_id as f64;
            drop(st);
            Ok(push_mount_info(
                state,
                &name,
                spell_id,
                icon,
                is_usable,
                is_collected,
                mount_id,
            ))
        })?
        .set_function("GetMountInfoByID", |state| {
            let mount_id = u32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let Some(m) = st.world.mounts.iter().find(|m| m.mount_id == mount_id) else {
                drop(st);
                return Ok(0);
            };
            let name = m.name.clone();
            let spell_id = m.spell_id as f64;
            let icon = m.icon as f64;
            let is_usable = m.is_usable;
            let is_collected = m.is_collected;
            let mid = m.mount_id as f64;
            drop(st);
            Ok(push_mount_info(
                state,
                &name,
                spell_id,
                icon,
                is_usable,
                is_collected,
                mid,
            ))
        })?
        .set_function("GetMountInfoExtraByID", |state| {
            let mount_id = u32::from_stack(state, 1)?;
            let st = borrow_state(state)?;
            let Some(m) = st.world.mounts.iter().find(|m| m.mount_id == mount_id) else {
                drop(st);
                return Ok(0);
            };
            let mount_type = m.mount_type as f64;
            drop(st);
            let empty = create_string(state, "");
            let source = create_string(state, "Drop");
            state.push(Val::Num(0.0));
            state.push(empty);
            state.push(source);
            state.push(Val::Bool(false));
            state.push(Val::Num(mount_type));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            state.push(Val::Bool(false));
            Ok(9)
        })?
        .set_function("GetMountIDs", |state| {
            use crate::lua_api::rilua_methods::create_table;
            create_table(state).into_stack(state)
        })?
        .set_function("GetNumMountsNeedingFanfare", |state| {
            (0i32).into_stack(state)
        })?
        .set_function("GetCollectedFilterSetting", |state| true.into_stack(state))?
        .set_function("SetCollectedFilterSetting", |_state| Ok(0))?
        .set_function("GetIsFavorite", |state| (false, false).into_stack(state))?
        .set_function("SetIsFavorite", |_state| Ok(0))?
        .set_function("Summon", |_state| Ok(0))?
        .set_function("Dismiss", |_state| Ok(0))?
        .build();

    set_global_val(lua.state_mut(), "C_MountJournal", t);
    Ok(())
}
