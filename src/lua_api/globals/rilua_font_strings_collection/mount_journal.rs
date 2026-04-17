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

fn register_mount_counts(tb: TableBuilder) -> LuaResult<TableBuilder> {
    tb.set_function("GetNumMounts", |state| {
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
    })
}

struct MountInfoSnapshot {
    name: String,
    spell_id: f64,
    icon: f64,
    is_usable: bool,
    is_collected: bool,
    mount_id: f64,
}

impl MountInfoSnapshot {
    fn from_mount(m: &crate::lua_api::state_types::MountData) -> Self {
        Self {
            name: m.name.clone(),
            spell_id: m.spell_id as f64,
            icon: m.icon as f64,
            is_usable: m.is_usable,
            is_collected: m.is_collected,
            mount_id: m.mount_id as f64,
        }
    }

    fn push(self, state: &mut LuaState) -> u32 {
        push_mount_info(
            state,
            &self.name,
            self.spell_id,
            self.icon,
            self.is_usable,
            self.is_collected,
            self.mount_id,
        )
    }
}

fn mount_get_displayed_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let snapshot = {
        let st = borrow_state(state)?;
        let i = (index - 1) as usize;
        st.world.mounts.get(i).map(MountInfoSnapshot::from_mount)
    };
    let Some(snapshot) = snapshot else {
        return Ok(0);
    };
    Ok(snapshot.push(state))
}

fn mount_get_info_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let mount_id = u32::from_stack(state, 1)?;
    let snapshot = {
        let st = borrow_state(state)?;
        st.world
            .mounts
            .iter()
            .find(|m| m.mount_id == mount_id)
            .map(MountInfoSnapshot::from_mount)
    };
    let Some(snapshot) = snapshot else {
        return Ok(0);
    };
    Ok(snapshot.push(state))
}

fn mount_get_info_extra_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let mount_id = u32::from_stack(state, 1)?;
    let mount_type = {
        let st = borrow_state(state)?;
        st.world
            .mounts
            .iter()
            .find(|m| m.mount_id == mount_id)
            .map(|m| m.mount_type as f64)
    };
    let Some(mount_type) = mount_type else {
        return Ok(0);
    };
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
}

fn register_mount_info(tb: TableBuilder) -> LuaResult<TableBuilder> {
    tb.set_function("GetDisplayedMountInfo", mount_get_displayed_info)?
        .set_function("GetMountInfoByID", mount_get_info_by_id)?
        .set_function("GetMountInfoExtraByID", mount_get_info_extra_by_id)
}

fn register_mount_stubs(tb: TableBuilder) -> LuaResult<TableBuilder> {
    tb.set_function("GetMountIDs", |state| {
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
    .set_function("Dismiss", |_state| Ok(0))
}

pub fn register_rilua_mount_journal(lua: &mut rilua::Lua) -> LuaResult<()> {
    let tb = TableBuilder::new(lua.state_mut());
    let tb = register_mount_counts(tb)?;
    let tb = register_mount_info(tb)?;
    let tb = register_mount_stubs(tb)?;
    let t = tb.build();
    set_global_val(lua.state_mut(), "C_MountJournal", t);
    Ok(())
}
