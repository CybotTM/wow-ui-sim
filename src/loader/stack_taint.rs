use crate::lua_api::taint::{clear_active_stack_taint, restore_active_stack_taint};
use rilua::vm::state::LuaState;

pub(crate) type StackTaints = Vec<Option<String>>;

pub(crate) fn clear(state: &mut LuaState) -> StackTaints {
    clear_active_stack_taint(state)
}

pub(crate) fn restore(state: &mut LuaState, saved_taints: StackTaints) {
    restore_active_stack_taint(state, saved_taints);
}

pub(crate) fn with_secure_stack<T, E>(
    state: &mut LuaState,
    f: impl FnOnce(&mut LuaState) -> Result<T, E>,
) -> Result<T, E> {
    let saved_taints = clear(state);
    let result = f(state);
    restore(state, saved_taints);
    result
}
