//! Alert subsystem, data provider, and EditMode frame method stubs.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::frame_ref;
use mlua::{MultiValue, Value};

/// Alert subsystem, data provider, and EditMode stubs.
pub(super) fn add_alert_and_data_provider_methods<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
) {
    add_alert_subsystem_method(methods);
    add_data_provider_stubs(methods);
    add_edit_mode_stubs(methods);
}

/// AddQueuedAlertFrameSubSystem returns a queue-backed subsystem table.
fn add_alert_subsystem_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "AddQueuedAlertFrameSubSystem",
        |lua, this, args: MultiValue| create_queued_alert_subsystem(lua, this.0, args),
    );
}

/// WorldMapFrame data provider stubs and UseRaidStylePartyFrames.
fn add_data_provider_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AddDataProvider", |lua, this, provider: Value| {
        add_frame_data_provider(lua, this.0, provider)
    });
    methods.add_method("RemoveDataProvider", |lua, this, provider: Value| {
        remove_frame_data_provider(lua, this.0, provider)
    });
    methods.add_method("UseRaidStylePartyFrames", |_, _this, ()| Ok(false));
}

/// EditModeSystemMixin stubs: delegate to mixin override or return safe defaults.
fn add_edit_mode_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsInDefaultPosition", |lua, this, ()| {
        let id = this.0;
        if let Some((func, ud)) =
            super::methods_helpers::get_mixin_override(lua, id, "IsInDefaultPosition")
        {
            return func.call::<bool>(ud);
        }
        frame_edit_mode_is_in_default_position(lua, id)
    });
    methods.add_method("IsInitialized", |lua, this, ()| {
        let id = this.0;
        if let Some((func, ud)) =
            super::methods_helpers::get_mixin_override(lua, id, "IsInitialized")
        {
            return func.call::<bool>(ud);
        }
        frame_edit_mode_is_initialized(lua, id)
    });
}

fn frame_edit_mode_is_initialized(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<bool> {
    let fields = super::methods_misc::frame_fields(lua, frame_id)?;
    Ok(edit_mode_field_exists(&fields, "systemInfo")
        || edit_mode_field_exists(&fields, "layoutInfo"))
}

fn frame_edit_mode_is_in_default_position(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<bool> {
    let fields = super::methods_misc::frame_fields(lua, frame_id)?;
    let Value::Table(system_info) = fields.get::<Value>("systemInfo")? else {
        return Ok(false);
    };

    Ok(matches!(
        system_info.get::<Value>("isInDefaultPosition")?,
        Value::Boolean(true)
    ))
}

fn edit_mode_field_exists(fields: &mlua::Table, field_name: &str) -> bool {
    !matches!(fields.get::<Value>(field_name), Ok(Value::Nil) | Err(_))
}

fn add_frame_data_provider(lua: &mlua::Lua, frame_id: u64, provider: Value) -> mlua::Result<()> {
    let providers = frame_data_providers(lua, frame_id)?;
    if table_contains_value(&providers, &provider)? {
        return Ok(());
    }
    let next_index = providers.raw_len() + 1;
    providers.raw_set(next_index, provider)
}

fn remove_frame_data_provider(
    lua: &mlua::Lua,
    frame_id: u64,
    provider: Value,
) -> mlua::Result<()> {
    let providers = frame_data_providers(lua, frame_id)?;
    remove_matching_value(&providers, &provider)
}

fn frame_data_providers(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<mlua::Table> {
    let fields = super::methods_misc::frame_fields(lua, frame_id)?;
    match fields.get::<Value>("dataProviders")? {
        Value::Table(table) => Ok(table),
        _ => {
            let table = lua.create_table()?;
            fields.set("dataProviders", table.clone())?;
            Ok(table)
        }
    }
}

fn create_queued_alert_subsystem(
    lua: &mlua::Lua,
    frame_id: u64,
    args: MultiValue,
) -> mlua::Result<Value> {
    let subsystem = lua.create_table()?;
    let alert_subsystems = alert_frame_subsystems(lua, frame_id)?;
    let next_index = alert_subsystems.raw_len() + 1;
    let anchor_priority = 1000 + (next_index as i32) * 10;

    populate_queued_alert_subsystem(lua, frame_id, &subsystem, args, anchor_priority)?;
    alert_subsystems.raw_set(next_index, subsystem.clone())?;

    Ok(Value::Table(subsystem))
}

fn populate_queued_alert_subsystem(
    lua: &mlua::Lua,
    frame_id: u64,
    subsystem: &mlua::Table,
    args: MultiValue,
    anchor_priority: i32,
) -> mlua::Result<()> {
    let mut values = args.into_vec().into_iter();
    let alert_frame_template = values.next().unwrap_or(Value::Nil);
    let set_up_function = values.next().unwrap_or(Value::Nil);
    let max_alerts = alert_subsystem_integer_arg(values.next(), 2);
    let max_queue = alert_subsystem_integer_arg(values.next(), 6);
    let coalesce_function = values.next().unwrap_or(Value::Nil);

    subsystem.set("alertContainer", frame_ref(lua, frame_id)?)?;
    subsystem.set("alertFrameTemplate", alert_frame_template)?;
    subsystem.set("setUpFunction", set_up_function)?;
    subsystem.set("maxAlerts", max_alerts)?;
    subsystem.set("maxQueue", max_queue)?;
    subsystem.set("coalesceFunction", coalesce_function)?;
    subsystem.set("queuedAlerts", lua.create_table()?)?;
    subsystem.set("anchorPriority", anchor_priority)?;
    install_queued_alert_subsystem_methods(lua, subsystem)?;
    Ok(())
}

fn install_queued_alert_subsystem_methods(
    lua: &mlua::Lua,
    subsystem: &mlua::Table,
) -> mlua::Result<()> {
    subsystem.set(
        "SetCanShowMoreConditionFunc",
        lua.create_function(|_, args: MultiValue| {
            let (subsystem, values) = split_alert_subsystem_call(args)?;
            let func = values.into_iter().next().unwrap_or(Value::Nil);
            subsystem.set("canShowMoreConditionFunc", func)
        })?,
    )?;
    subsystem.set(
        "AddAlert",
        lua.create_function(|lua, args: MultiValue| {
            let (subsystem, values) = split_alert_subsystem_call(args)?;
            queue_alert_subsystem_alert(lua, &subsystem, values)
        })?,
    )?;
    subsystem.set(
        "RemoveAlert",
        lua.create_function(|lua, args: MultiValue| {
            let (subsystem, values) = split_alert_subsystem_call(args)?;
            remove_alert_subsystem_alert(lua, &subsystem, values)
        })?,
    )?;
    subsystem.set(
        "ClearAllAlerts",
        lua.create_function(|lua, args: MultiValue| {
            let (subsystem, _values) = split_alert_subsystem_call(args)?;
            subsystem.set("queuedAlerts", lua.create_table()?)
        })?,
    )?;
    Ok(())
}

fn queue_alert_subsystem_alert(
    lua: &mlua::Lua,
    subsystem: &mlua::Table,
    alert_values: Vec<Value>,
) -> mlua::Result<bool> {
    let queued_alerts = alert_subsystem_queue(lua, subsystem)?;
    let max_queue = alert_subsystem_max_queue(subsystem)?;
    if queued_alerts.raw_len() >= max_queue {
        return Ok(false);
    }

    let next_index = queued_alerts.raw_len() + 1;
    let alert_data = create_alert_subsystem_queued_data(lua, alert_values)?;
    queued_alerts.raw_set(next_index, alert_data)?;
    Ok(true)
}

fn remove_alert_subsystem_alert(
    lua: &mlua::Lua,
    subsystem: &mlua::Table,
    expected_values: Vec<Value>,
) -> mlua::Result<bool> {
    let queued_alerts = alert_subsystem_queue(lua, subsystem)?;
    let mut kept = Vec::new();
    let mut removed = false;

    for value in queued_alerts.sequence_values::<Value>() {
        let value = value?;
        if !removed && alert_subsystem_entry_matches(&value, &expected_values)? {
            removed = true;
            continue;
        }
        kept.push(value);
    }

    queued_alerts.clear()?;
    for (index, value) in kept.into_iter().enumerate() {
        queued_alerts.raw_set(index + 1, value)?;
    }

    Ok(removed)
}

fn create_alert_subsystem_queued_data(
    lua: &mlua::Lua,
    values: Vec<Value>,
) -> mlua::Result<mlua::Table> {
    let data = lua.create_table()?;
    for (index, value) in values.into_iter().enumerate() {
        data.raw_set(index + 1, value)?;
    }
    data.set("numElements", data.raw_len())?;
    Ok(data)
}

fn alert_subsystem_entry_matches(value: &Value, expected_values: &[Value]) -> mlua::Result<bool> {
    let Value::Table(entry) = value else {
        return Ok(false);
    };

    if entry.raw_len() != expected_values.len() {
        return Ok(false);
    }

    for (index, expected) in expected_values.iter().enumerate() {
        let actual = entry.raw_get::<Value>(index + 1)?;
        if actual != *expected {
            return Ok(false);
        }
    }

    Ok(true)
}

fn split_alert_subsystem_call(args: MultiValue) -> mlua::Result<(mlua::Table, Vec<Value>)> {
    let mut values = args.into_vec();
    let Some(self_value) = values.first().cloned() else {
        return Err(mlua::Error::RuntimeError(
            "alert subsystem method missing self".to_string(),
        ));
    };
    let Value::Table(subsystem) = self_value else {
        return Err(mlua::Error::RuntimeError(
            "alert subsystem method expected table self".to_string(),
        ));
    };
    values.remove(0);
    Ok((subsystem, values))
}

fn alert_frame_subsystems(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<mlua::Table> {
    let fields = super::methods_misc::frame_fields(lua, frame_id)?;
    match fields.get::<Value>("alertFrameSubSystems")? {
        Value::Table(table) => Ok(table),
        _ => {
            let table = lua.create_table()?;
            fields.set("alertFrameSubSystems", table.clone())?;
            Ok(table)
        }
    }
}

fn alert_subsystem_queue(lua: &mlua::Lua, subsystem: &mlua::Table) -> mlua::Result<mlua::Table> {
    match subsystem.get::<Value>("queuedAlerts")? {
        Value::Table(table) => Ok(table),
        _ => {
            let table = lua.create_table()?;
            subsystem.set("queuedAlerts", table.clone())?;
            Ok(table)
        }
    }
}

fn alert_subsystem_max_queue(subsystem: &mlua::Table) -> mlua::Result<usize> {
    match subsystem.get::<Value>("maxQueue")? {
        Value::Integer(value) if value > 0 => Ok(value as usize),
        Value::Number(value) if value.is_finite() && value > 0.0 => Ok(value as usize),
        _ => Ok(0),
    }
}

fn alert_subsystem_integer_arg(value: Option<Value>, default: i32) -> i32 {
    match value {
        Some(Value::Integer(value)) => value as i32,
        Some(Value::Number(value)) => value as i32,
        _ => default,
    }
}

fn table_contains_value(table: &mlua::Table, expected: &Value) -> mlua::Result<bool> {
    for value in table.sequence_values::<Value>() {
        if value? == *expected {
            return Ok(true);
        }
    }
    Ok(false)
}

fn remove_matching_value(table: &mlua::Table, expected: &Value) -> mlua::Result<()> {
    let mut next_index = 1;
    let mut kept = Vec::new();
    for value in table.sequence_values::<Value>() {
        let value = value?;
        if value != *expected {
            kept.push(value);
        }
    }
    table.clear()?;
    for value in kept {
        table.raw_set(next_index, value)?;
        next_index += 1;
    }
    Ok(())
}

