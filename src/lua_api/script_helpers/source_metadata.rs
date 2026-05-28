use rilua::Val;
use rilua::vm::state::LuaState;

use super::{
    ScriptBinding, registry_table, registry_table_or_create, script_frame_table,
    script_handler_key_ref, table_get_str_ref, table_set_str_ref,
};

const SCRIPT_SOURCES_PRECALL_KEY: &str = "__script_sources_pre";
const SCRIPT_SOURCES_KEY: &str = "__script_sources";
const SCRIPT_SOURCES_POSTCALL_KEY: &str = "__script_sources_post";

impl ScriptBinding {
    fn source_registry_key(self) -> &'static str {
        match self {
            Self::Precall => SCRIPT_SOURCES_PRECALL_KEY,
            Self::Normal => SCRIPT_SOURCES_KEY,
            Self::Postcall => SCRIPT_SOURCES_POSTCALL_KEY,
        }
    }
}

pub fn get_script_source_binding(
    state: &mut LuaState,
    widget_id: u64,
    handler_name: &str,
    binding: ScriptBinding,
) -> Option<String> {
    let sources = registry_table(state, binding.source_registry_key())?;
    let source_table = script_frame_table(state, sources, widget_id, false)?;
    let handler_key = script_handler_key_ref(state, handler_name);
    let Val::Str(source_ref) = table_get_str_ref(state, source_table, handler_key) else {
        return None;
    };
    let source = state.gc.string_arena.get(source_ref)?;
    Some(String::from_utf8_lossy(source.data()).into_owned())
}

pub(super) fn set_script_source_binding(
    state: &mut LuaState,
    widget_id: u64,
    handler_name: &str,
    binding: ScriptBinding,
    func: Val,
) {
    let Some(source_label) = script_source_label(state, func) else {
        remove_script_source_binding(state, widget_id, handler_name, binding);
        return;
    };
    let sources = registry_table_or_create(state, binding.source_registry_key());
    let source_table =
        script_frame_table(state, sources, widget_id, true).expect("created source table");
    let handler_key = script_handler_key_ref(state, handler_name);
    let source_ref = state.gc.intern_string(source_label.as_bytes());
    table_set_str_ref(state, source_table, handler_key, Val::Str(source_ref));
}

pub(super) fn remove_script_source_binding(
    state: &mut LuaState,
    widget_id: u64,
    handler_name: &str,
    binding: ScriptBinding,
) {
    if let Some(sources) = registry_table(state, binding.source_registry_key())
        && let Some(source_table) = script_frame_table(state, sources, widget_id, false)
    {
        let handler_key = script_handler_key_ref(state, handler_name);
        table_set_str_ref(state, source_table, handler_key, Val::Nil);
    }
}

fn script_source_label(state: &LuaState, func: Val) -> Option<String> {
    let Val::Function(func_ref) = func else {
        return None;
    };
    let closure = state.gc.closures.get(func_ref)?;
    let lua_closure = closure.as_lua()?;
    let proto = lua_closure.proto.as_ref();
    if proto.short_source.is_empty() {
        return None;
    }

    Some(format!("{}:{}", proto.short_source, proto.line_defined))
}
