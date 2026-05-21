//! Temporary sanitizer for imported Details! SavedVariables.
//!
//! Real WTF profiles can carry window geometry from a different viewport. The
//! simulator also seeds short fake combat data for smoke screenshots, so very
//! narrow imported Details windows make row columns overlap at startup. Keep
//! this scoped to read-only WTF import; simulator-local saved variables should
//! preserve the user's explicit choices.

use crate::lua_api::methods::{table_get_static, table_set_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, Val};

const MIN_DETAILS_METER_WIDTH: f64 = 300.0;

pub(crate) fn sanitize_imported_wtf_addon(state: &mut LuaState, addon_name: &str) {
    if addon_name != "Details" {
        return;
    }
    sanitize_details_database(state);
}

fn sanitize_details_database(state: &mut LuaState) {
    sanitize_account_profiles(state);
    sanitize_character_instances(state);
}

fn sanitize_account_profiles(state: &mut LuaState) {
    let global = LuaApiMut::get_global_val(state, "_detalhes_global");
    let profiles = table_get_static(state, global, "__profiles");
    let Some(profiles_ref) = table_ref(profiles) else {
        return;
    };

    for profile in table_hash_values(state, profiles_ref) {
        sanitize_profile(state, profile);
    }
}

fn sanitize_profile(state: &mut LuaState, profile: Val) {
    let instances = table_get_static(state, profile, "instances");
    let Some(instances_ref) = table_ref(instances) else {
        return;
    };

    for index in 1..=array_len(state, instances_ref) {
        let instance = table_get_int(state, instances_ref, index as i64);
        let pos = table_get_static(state, instance, "__pos");
        let normal = table_get_static(state, pos, "normal");
        clamp_window_width(state, normal);
    }
}

fn sanitize_character_instances(state: &mut LuaState) {
    let database = LuaApiMut::get_global_val(state, "_detalhes_database");
    let configs = table_get_static(state, database, "local_instances_config");
    let Some(configs_ref) = table_ref(configs) else {
        return;
    };

    for index in 1..=array_len(state, configs_ref) {
        let instance_config = table_get_int(state, configs_ref, index as i64);
        sanitize_instance_config(state, instance_config);
    }
}

fn sanitize_instance_config(state: &mut LuaState, instance_config: Val) {
    let pos = table_get_static(state, instance_config, "pos");
    let normal = table_get_static(state, pos, "normal");
    clamp_window_width(state, normal);
}

fn clamp_window_width(state: &mut LuaState, window_config: Val) {
    let width = table_get_static(state, window_config, "w");
    let Val::Num(width) = width else {
        return;
    };
    if width >= MIN_DETAILS_METER_WIDTH {
        return;
    }
    table_set_static(state, window_config, "w", Val::Num(MIN_DETAILS_METER_WIDTH));
}

fn table_ref(value: Val) -> Option<GcRef<Table>> {
    match value {
        Val::Table(table_ref) => Some(table_ref),
        _ => None,
    }
}

fn array_len(state: &LuaState, table_ref: GcRef<Table>) -> usize {
    state
        .gc
        .tables
        .get(table_ref)
        .map(|table| table.array_slice().len())
        .unwrap_or(0)
}

fn table_hash_values(state: &LuaState, table_ref: GcRef<Table>) -> Vec<Val> {
    state
        .gc
        .tables
        .get(table_ref)
        .map(|table| {
            table
                .hash_entries()
                .into_iter()
                .map(|(_, value)| value)
                .collect()
        })
        .unwrap_or_default()
}

fn table_get_int(state: &LuaState, table_ref: GcRef<Table>, key: i64) -> Val {
    state
        .gc
        .tables
        .get(table_ref)
        .map(|table| table.get_int(key))
        .unwrap_or(Val::Nil)
}
