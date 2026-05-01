//! C_MountJournal namespace.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::{FromStack, IntoStack, TableBuilder};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

use super::set_global_val;

fn mount_matches_search(mount: &crate::lua_api::state_types::MountData, search_text: &str) -> bool {
    search_text.is_empty() || mount.name.to_lowercase().contains(search_text)
}

fn displayed_mount_count(st: &crate::lua_api::state::SimState) -> i32 {
    st.world
        .mounts
        .iter()
        .filter(|mount| mount_matches_search(mount, &st.world.mount_search_text))
        .count() as i32
}

fn displayed_mount<'a>(
    st: &'a crate::lua_api::state::SimState,
    index: i32,
) -> Option<&'a crate::lua_api::state_types::MountData> {
    if index <= 0 {
        return None;
    }
    let i = (index - 1) as usize;
    st.world
        .mounts
        .iter()
        .filter(|mount| mount_matches_search(mount, &st.world.mount_search_text))
        .nth(i)
}

fn displayed_mount_snapshot(
    st: &crate::lua_api::state::SimState,
    index: i32,
) -> Option<MountInfoSnapshot> {
    displayed_mount(st, index).map(MountInfoSnapshot::from_mount)
}

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
        let count = borrow_state(state)?.world.mounts.len() as i32;
        count.into_stack(state)
    })?
    .set_function("GetNumDisplayedMounts", |state| {
        let count = {
            let st = borrow_state(state)?;
            displayed_mount_count(&st)
        };
        count.into_stack(state)
    })?
    .set_function("IsUsingDefaultFilters", |state| true.into_stack(state))?
    .set_function("SetDefaultFilters", |_state| Ok(0))?
    .set_function("GetDisplayedMountID", |state| {
        let index = i32::from_stack(state, 1)?;
        let mount_id = {
            let st = borrow_state(state)?;
            displayed_mount(&st, index).map(|mount| mount.mount_id as f64)
        };
        match mount_id {
            Some(id) => id.into_stack(state),
            None => Ok(0),
        }
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
        displayed_mount_snapshot(&st, index)
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
    Ok(tb
        .set_function("GetMountIDs", |state| {
            use crate::lua_api::methods::create_table;
            let ids = {
                let st = borrow_state(state)?;
                st.world
                    .mounts
                    .iter()
                    .map(|mount| mount.mount_id as f64)
                    .collect::<Vec<_>>()
            };
            let table = create_table(state);
            for (index, mount_id) in ids.into_iter().enumerate() {
                let Val::Table(table_ref) = table else {
                    return Ok(0);
                };
                if let Some(array) = state.gc.tables.get_mut(table_ref) {
                    let _ = array.raw_set(
                        Val::Num(index as f64 + 1.0),
                        Val::Num(mount_id),
                        &state.gc.string_arena,
                    );
                }
                state.gc.barrier_back(table_ref);
            }
            table.into_stack(state)
        })?
        .set_function("GetMountAllCreatureDisplayInfoByID", |state| {
            use crate::lua_api::methods::create_table;
            create_table(state).into_stack(state)
        })?
        .set_function("AreMountEquipmentEffectsSuppressed", |state| {
            false.into_stack(state)
        })?
        .set_function("GetAppliedMountEquipmentID", |_state| Ok(0))?
        .set_function("ClearRecentFanfares", |_state| Ok(0))?
        .set_function("ClearFanfare", |_state| Ok(0))?
        .set_function("NeedsFanfare", |state| false.into_stack(state))?
        .set_function("GetDynamicFlightModeSpellID", |state| {
            (0i32).into_stack(state)
        })?
        .set_function("GetMountEquipmentUnlockLevel", |state| {
            (0i32).into_stack(state)
        })?
        .set_function("GetNumMountsNeedingFanfare", |state| {
            (0i32).into_stack(state)
        })?
        .set_function("GetCollectedFilterSetting", |state| true.into_stack(state))?
        .set_function("IsDragonridingUnlocked", |state| false.into_stack(state))?
        .set_function("IsUsingDefaultFilters", |state| true.into_stack(state))?
        .set_function("SetDefaultFilters", |_state| Ok(0))?
        .set_function("IsSourceChecked", |state| false.into_stack(state))?
        .set_function("SetSourceFilter", |_state| Ok(0))?
        .set_function("IsValidSourceFilter", |state| true.into_stack(state))?
        .set_function("SetAllSourceFilters", |_state| Ok(0))?
        .set_function("IsTypeChecked", |state| false.into_stack(state))?
        .set_function("SetTypeFilter", |_state| Ok(0))?
        .set_function("IsValidTypeFilter", |state| true.into_stack(state))?
        .set_function("SetCollectedFilterSetting", |_state| Ok(0))?
        .set_function("GetIsFavorite", |state| (false, false).into_stack(state))?
        .set_function("SetIsFavorite", |_state| Ok(0))?
        .set_function("GetMountLink", |state| {
            state.push(Val::Nil);
            Ok(1)
        })?
        .set_function("GetMountUsabilityByID", |state| {
            (false, false, false).into_stack(state)
        })?
        .set_function("Summon", |_state| Ok(0))?
        .set_function("SummonByID", |_state| Ok(0))?
        .set_function("Dismiss", |_state| Ok(0))?
        .set_function("SetSearch", |state| {
            let search_text = Option::<String>::from_stack(state, 1)?
                .unwrap_or_default()
                .trim()
                .to_lowercase();
            borrow_state_mut(state)?.world.mount_search_text = search_text;
            Ok(0)
        })?
        .set_function("IsItemMountEquipment", |state| false.into_stack(state))?
        .set_function("IsMountEquipmentApplied", |state| false.into_stack(state))?
        .set_function("PickupDynamicFlightMode", |_state| Ok(0))?
        .set_function("SwapDynamicFlightMode", |_state| Ok(0))?)
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
