use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn get_num_applications(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn get_num_applicants(state: &mut LuaState) -> LuaResult<u32> {
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

/// `A_Admin.SetLfgApplicationCounts(total?, viewed?)` defaults missing args
/// to 0; negatives clamp to 0.
pub fn admin_set_application_counts(state: &mut LuaState) -> LuaResult<u32> {
    let total = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let viewed = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as i32;
    let mut st = borrow_state_mut(state)?;
    st.lfg_list_counts.applications_total = total.max(0);
    st.lfg_list_counts.applications_viewed = viewed.max(0);
    Ok(0)
}

/// `A_Admin.SetLfgApplicantCounts(total?, viewed?)` defaults missing args
/// to 0; negatives clamp to 0.
pub fn admin_set_applicant_counts(state: &mut LuaState) -> LuaResult<u32> {
    let total = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let viewed = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0) as i32;
    let mut st = borrow_state_mut(state)?;
    st.lfg_list_counts.applicants_total = total.max(0);
    st.lfg_list_counts.applicants_viewed = viewed.max(0);
    Ok(0)
}
