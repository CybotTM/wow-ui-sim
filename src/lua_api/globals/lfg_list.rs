//! `C_LFGList.GetNumApplications` / `GetNumApplicants` — two-value probes
//! backed by `SimState::lfg_list_counts`.
//!
//! Each returns `(total, viewed)`:
//!
//! - `GetNumApplications()` — how many of the player's listings the server
//!                            knows about, and how many are still within the
//!                            scroll viewport.
//! - `GetNumApplicants()`   — applicant count + viewed-applicant count.
//!
//! Shape matters because `LFGListFrame` callsites do
//! `local total, viewed = C_LFGList.GetNumApplications()`. The earlier Lua
//! stub returned `(0, 0)`, which is correct for an idle sim but doesn't
//! allow tests to exercise non-empty applicant lists; the new admin API
//! `A_Admin.SetLfgApplicationCounts(total, viewed)` /
//! `A_Admin.SetLfgApplicantCounts(total, viewed)` drives the values.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_table};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn get_num_applications(state: &mut LuaState) -> LuaResult<u32> {
    let (total, viewed) = {
        let sim = borrow_state(state)?;
        (
            sim.lfg_list_counts.applications_total,
            sim.lfg_list_counts.applications_viewed,
        )
    };
    state.push(Val::Num(total as f64));
    state.push(Val::Num(viewed as f64));
    Ok(2)
}

pub fn get_num_applicants(state: &mut LuaState) -> LuaResult<u32> {
    let (total, viewed) = {
        let sim = borrow_state(state)?;
        (
            sim.lfg_list_counts.applicants_total,
            sim.lfg_list_counts.applicants_viewed,
        )
    };
    state.push(Val::Num(total as f64));
    state.push(Val::Num(viewed as f64));
    Ok(2)
}

fn ensure_c_lfg_list_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_LFGList");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(r)) = existing {
        return r;
    }
    let new_val = create_table(state);
    let Val::Table(new_ref) = new_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_ref
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let table_ref = ensure_c_lfg_list_table(state);
    table_set_rust_fn(state, table_ref, "GetNumApplications", get_num_applications)?;
    table_set_rust_fn(state, table_ref, "GetNumApplicants", get_num_applicants)?;
    Ok(())
}

/// `A_Admin.SetLfgApplicationCounts(total?, viewed?)` — missing args default
/// to 0; negatives clamp to 0.
pub fn admin_set_application_counts(state: &mut LuaState) -> LuaResult<u32> {
    let total = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let viewed = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as i32;
    let mut st = borrow_state_mut(state)?;
    st.lfg_list_counts.applications_total = total.max(0);
    st.lfg_list_counts.applications_viewed = viewed.max(0);
    Ok(0)
}

/// `A_Admin.SetLfgApplicantCounts(total?, viewed?)` — missing args default
/// to 0; negatives clamp to 0.
pub fn admin_set_applicant_counts(state: &mut LuaState) -> LuaResult<u32> {
    let total = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let viewed = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as i32;
    let mut st = borrow_state_mut(state)?;
    st.lfg_list_counts.applicants_total = total.max(0);
    st.lfg_list_counts.applicants_viewed = viewed.max(0);
    Ok(0)
}
