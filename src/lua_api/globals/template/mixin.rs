use super::lua_global_ref;
use crate::loader::chunk_cache;
use crate::lua_api::frame::{FrameRef, frame_ref, get_sim_state};
use mlua::{Lua, Value};

/// Apply mixin to a frame.
pub(super) fn apply_mixin(lua: &Lua, mixin: &Option<String>, frame_name: &str) {
    let Some(mixin) = mixin else { return };
    if apply_mixin_direct(lua, mixin, frame_name).is_ok() {
        return;
    }
    apply_mixin_via_chunk(lua, mixin, frame_name);
}

fn apply_mixin_direct(lua: &Lua, mixin: &str, frame_name: &str) -> mlua::Result<()> {
    let Some(fields) = frame_fields_by_name(lua, frame_name)? else {
        return Ok(());
    };
    let secure_methods: Value = lua.globals().get("__secureMixinMethods")?;
    for mixin_table in resolve_mixin_tables(lua, mixin)? {
        let source = mixin_source_table(&secure_methods, mixin_table)?;
        for pair in source.pairs::<Value, Value>() {
            let (key, value) = pair?;
            fields.raw_set(key, value)?;
        }
    }
    let post_init = build_mixin_post_init(mixin);
    if !post_init.is_empty() {
        let code = format!(
            "do local f = {} if f then {} end end",
            lua_global_ref(frame_name),
            post_init,
        );
        let _ = chunk_cache::exec(lua, &code, "template-mod");
    }
    Ok(())
}

fn mixin_source_table(
    secure_methods: &Value,
    mixin_table: mlua::Table,
) -> mlua::Result<mlua::Table> {
    if let Value::Table(sm) = secure_methods
        && let Value::Table(table) = sm.get::<Value>(mixin_table.clone())?
    {
        return Ok(table);
    }
    Ok(mixin_table)
}

fn resolve_mixin_tables(lua: &Lua, mixin: &str) -> mlua::Result<Vec<mlua::Table>> {
    let mut mixins = Vec::new();
    for name in mixin.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if let Value::Table(table) = resolve_top_level_value(lua, name)? {
            mixins.push(table);
        }
    }
    Ok(mixins)
}

fn apply_mixin_via_chunk(lua: &Lua, mixin: &str, frame_name: &str) {
    let mut parts = Vec::new();
    for name in mixin.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        parts.push(format!(
            "do local m = {name} or (__secureenv and rawget(__secureenv, \"{name}\")) \
             if m then Mixin(f, m) end end"
        ));
    }
    if parts.is_empty() {
        return;
    }
    let post_init = build_mixin_post_init(mixin);
    let code = format!(
        "do local f = {} if f then {} {} end end",
        lua_global_ref(frame_name),
        parts.join(" "),
        post_init,
    );
    let _ = chunk_cache::exec(lua, &code, "template-mod");
}

pub(super) fn frame_fields_by_name(
    lua: &Lua,
    frame_name: &str,
) -> mlua::Result<Option<mlua::Table>> {
    let value = resolve_frame_value_by_name(lua, frame_name)?;
    frame_fields_from_value(value)
}

fn frame_fields_from_value(value: Value) -> mlua::Result<Option<mlua::Table>> {
    let userdata = match value {
        Value::UserData(ud) => Some(ud),
        Value::Table(t) => match t.raw_get::<Value>("__lud")? {
            Value::UserData(ud) => Some(ud),
            _ => None,
        },
        _ => None,
    };
    let Some(userdata) = userdata else {
        return Ok(None);
    };
    if userdata.is::<FrameRef>() {
        return userdata.user_value::<mlua::Table>().map(Some);
    }
    Ok(None)
}

fn resolve_frame_value_by_name(lua: &Lua, frame_name: &str) -> mlua::Result<Value> {
    if let Some(id) = frame_name
        .strip_prefix("__frame_")
        .and_then(|suffix| suffix.parse::<u64>().ok())
    {
        return frame_ref(lua, id);
    }

    let globals = lua.globals();
    let value: Value = globals.raw_get(frame_name)?;
    if !value.is_nil() {
        return Ok(value);
    }

    let frame_id = get_sim_state(lua)
        .borrow()
        .widgets
        .get_id_by_name(frame_name);
    match frame_id {
        Some(id) => frame_ref(lua, id),
        None => Ok(Value::Nil),
    }
}

pub(super) fn resolve_global_path_value(lua: &Lua, path: &str) -> mlua::Result<Value> {
    let mut segments = path.split('.').filter(|segment| !segment.is_empty());
    let Some(first) = segments.next() else {
        return Ok(Value::Nil);
    };
    let mut current = resolve_top_level_value(lua, first)?;
    for segment in segments {
        current = match current {
            Value::Table(table) => table.get::<Value>(segment)?,
            _ => return Ok(Value::Nil),
        };
    }
    Ok(current)
}

fn resolve_top_level_value(lua: &Lua, name: &str) -> mlua::Result<Value> {
    let globals = lua.globals();
    let global_value = globals.get::<Value>(name)?;
    if !global_value.is_nil() {
        return Ok(global_value);
    }
    if let Ok(secureenv) = lua.named_registry_value::<mlua::Table>("__secureenv") {
        let secure_value = secureenv.get::<Value>(name)?;
        if !secure_value.is_nil() {
            return Ok(secure_value);
        }
    }
    Ok(Value::Nil)
}

/// Build post-initialization code for known mixins that need pre-seeded fields.
fn build_mixin_post_init(mixin: &str) -> String {
    let mut post_init = String::new();
    for name in mixin.split(',').map(str::trim) {
        append_mixin_post_init(&mut post_init, name);
    }
    post_init
}

fn append_mixin_post_init(post_init: &mut String, name: &str) {
    match name {
        "ActionBarMixin" => {
            post_init.push_str("f.actionButtons = f.actionButtons or {} ");
            post_init.push_str("f.shownButtonContainers = f.shownButtonContainers or {} ");
        }
        "EditModeSystemMixin" => append_edit_mode_system_post_init(post_init),
        "EventFrameMixin" | "CallbackRegistryMixin" => {
            post_init.push_str("if f.OnLoad_Intrinsic then pcall(f.OnLoad_Intrinsic, f) end ");
            post_init.push_str("f.callbackTables = f.callbackTables or {} ");
            post_init.push_str("f.executingEvents = f.executingEvents or {} ");
        }
        "UIParentManagedFrameContainerMixin" => {
            post_init.push_str("f.showingFrames = f.showingFrames or {} ");
        }
        "TabSystemMixin" => {
            post_init.push_str("f.tabs = f.tabs or {} ");
        }
        _ => {}
    }
}

fn append_edit_mode_system_post_init(post_init: &mut String) {
    for alias in [
        "SetScale",
        "SetPoint",
        "ClearAllPoints",
        "SetShown",
        "Show",
        "Hide",
        "IsShown",
    ] {
        post_init.push_str(&format!("f.{alias}Base = f.{alias} "));
    }
}
