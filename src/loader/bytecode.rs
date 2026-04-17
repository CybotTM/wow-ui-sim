use crate::loader::error::LoadError;
use rilua::Function;
use rilua::vm::closure::Closure;
use rilua::vm::state::LuaState;

pub(crate) fn dump_function(state: &mut LuaState, func: &Function) -> Result<Vec<u8>, LoadError> {
    let closure = state
        .gc
        .closures
        .get(func.gc_ref())
        .ok_or_else(|| LoadError::Lua("compiled closure missing from GC".to_string()))?;
    let proto = match closure {
        Closure::Lua(closure) => closure.proto.clone(),
        Closure::Rust(_) => {
            return Err(LoadError::Lua(
                "expected Lua closure when dumping cached bytecode".to_string(),
            ));
        }
    };
    Ok(rilua::vm::dump::dump(
        &proto,
        Some(&state.gc.string_arena),
        false,
    ))
}
