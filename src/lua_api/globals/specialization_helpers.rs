//! Shared specialization Lua return helpers.

use crate::lua_api::methods::create_string;
use crate::specializations;
use rilua::Val;
use rilua::vm::state::LuaState;

pub(crate) fn push_specialization_identity(state: &mut LuaState, spec: &specializations::SpecInfo) {
    let spec_name = create_string(state, spec.name);
    let spec_description = create_string(state, spec.description);
    let spec_role = create_string(state, spec.role);
    state.push(Val::Num(spec.id as f64));
    state.push(spec_name);
    state.push(spec_description);
    state.push(Val::Num(spec.icon_file_data_id as f64));
    state.push(spec_role);
}
