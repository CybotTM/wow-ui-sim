//! LuaDurationObject UserData type for WoW duration tracking.
//!
//! Implements `C_DurationUtil.CreateDuration()` which returns a UserData object
//! with methods for tracking time spans (start/end/duration). The metatable is
//! hidden (`getmetatable` returns `false`), which is the default mlua behaviour
//! for all UserData types.
//!
//! Supports per-instance field storage so addon code can attach arbitrary Lua
//! values to duration objects.

use mlua::{AnyUserData, Lua, MetaMethod, MultiValue, Result, UserData, UserDataMethods, Value};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Global Lua table name used to store per-instance custom fields.
const FIELDS_TABLE: &str = "__lua_duration_object_fields";

/// Method names that are read-only (cannot be assigned by user code).
const METHOD_NAMES: &[&str] = &[
    "Assign",
    "Copy",
    "EvaluateElapsedDuration",
    "EvaluateElapsedPercent",
    "EvaluateRemainingDuration",
    "EvaluateRemainingPercent",
    "GetClockTime",
    "GetElapsedDuration",
    "GetElapsedPercent",
    "GetEndTime",
    "GetModRate",
    "GetRemainingDuration",
    "GetRemainingPercent",
    "GetStartTime",
    "GetTotalDuration",
    "HasSecretValues",
    "IsZero",
    "Reset",
    "SetTimeFromEnd",
    "SetTimeFromStart",
    "SetTimeSpan",
    "SetToDefaults",
];

/// Metamethod names that are read-only.
const META_NAMES: &[&str] = &["__eq", "__index", "__metatable", "__newindex", "__tostring"];

/// WoW LuaDurationObject userdata — tracks a time span (startTime, endTime, modRate).
pub struct LuaDurationObject {
    id: u64,
}

impl LuaDurationObject {
    pub fn new() -> Self {
        LuaDurationObject {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    fn add_duration_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_copy_and_assign_methods(methods);
        add_evaluate_methods(methods);
        add_getter_methods(methods);
        add_setter_methods(methods);
    }

    fn add_index_metamethod<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(
            MetaMethod::Index,
            |lua: &Lua, (ud, key): (AnyUserData, Value)| {
                let handle = ud.borrow::<LuaDurationObject>()?;
                let id = handle.id;
                drop(handle);

                let key_str = match &key {
                    Value::String(s) => s.to_string_lossy().to_string(),
                    _ => return Ok(Value::Nil),
                };

                let value = get_instance_fields(lua, id)
                    .and_then(|t| t.get::<Value>(key_str.as_str()).ok())
                    .unwrap_or(Value::Nil);

                Ok(value)
            },
        );
    }

    fn add_newindex_metamethod<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(
            MetaMethod::NewIndex,
            |lua: &Lua, (ud, key, value): (AnyUserData, String, Value)| {
                let handle = ud.borrow::<LuaDurationObject>()?;
                let id = handle.id;
                drop(handle);

                if is_readonly_key(&key) {
                    return Err(mlua::Error::RuntimeError(format!(
                        "Attempted to assign to read-only key {}",
                        key
                    )));
                }

                get_or_create_instance_fields(lua, id).set(key, value)?;
                Ok(())
            },
        );
    }
}

impl UserData for LuaDurationObject {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        Self::add_duration_methods(methods);
        Self::add_index_metamethod(methods);
        Self::add_newindex_metamethod(methods);
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(format!("LuaDurationObject: 0x{:016x}", this.id))
        });
    }
}

/// Returns true if the key is a method or metamethod name (read-only).
fn is_readonly_key(key: &str) -> bool {
    METHOD_NAMES.contains(&key) || META_NAMES.contains(&key)
}

/// Register Copy and Assign methods.
fn add_copy_and_assign_methods<M: UserDataMethods<LuaDurationObject>>(methods: &mut M) {
    methods.add_method("Assign", |_, _, _other: Value| Ok(()));
    methods.add_method("Copy", |_, _, ()| Ok(LuaDurationObject::new()));
}

/// Register Evaluate* methods (return 0.0 stubs).
fn add_evaluate_methods<M: UserDataMethods<LuaDurationObject>>(methods: &mut M) {
    methods.add_method("EvaluateElapsedDuration", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("EvaluateElapsedPercent", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("EvaluateRemainingDuration", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("EvaluateRemainingPercent", |_, _, _: MultiValue| Ok(0.0f64));
}

/// Register Get* query methods (return 0.0 / false stubs).
fn add_getter_methods<M: UserDataMethods<LuaDurationObject>>(methods: &mut M) {
    methods.add_method("GetClockTime", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("GetElapsedDuration", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("GetElapsedPercent", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("GetEndTime", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("GetModRate", |_, _, ()| Ok(1.0f64));
    methods.add_method("GetRemainingDuration", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("GetRemainingPercent", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("GetStartTime", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("GetTotalDuration", |_, _, _: MultiValue| Ok(0.0f64));
    methods.add_method("HasSecretValues", |_, _, ()| Ok(false));
    methods.add_method("IsZero", |_, _, ()| Ok(true));
}

/// Register Set* and Reset mutating methods (no-op stubs).
fn add_setter_methods<M: UserDataMethods<LuaDurationObject>>(methods: &mut M) {
    methods.add_method("Reset", |_, _, ()| Ok(()));
    methods.add_method("SetTimeFromEnd", |_, _, _: MultiValue| Ok(()));
    methods.add_method("SetTimeFromStart", |_, _, _: MultiValue| Ok(()));
    methods.add_method("SetTimeSpan", |_, _, _: MultiValue| Ok(()));
    methods.add_method("SetToDefaults", |_, _, ()| Ok(()));
}

/// Get the per-instance field table for `id`, or `None` if not yet created.
fn get_instance_fields(lua: &Lua, id: u64) -> Option<mlua::Table> {
    lua.globals()
        .get::<mlua::Table>(FIELDS_TABLE)
        .ok()
        .and_then(|outer| outer.get::<mlua::Table>(id).ok())
}

/// Get or create the per-instance field table for `id`.
fn get_or_create_instance_fields(lua: &Lua, id: u64) -> mlua::Table {
    let outer = lua
        .globals()
        .get::<mlua::Table>(FIELDS_TABLE)
        .unwrap_or_else(|_| {
            let t = lua.create_table().unwrap();
            lua.globals().set(FIELDS_TABLE, t.clone()).unwrap();
            t
        });

    outer.get::<mlua::Table>(id).unwrap_or_else(|_| {
        let t = lua.create_table().unwrap();
        outer.set(id, t.clone()).unwrap();
        t
    })
}

/// Register `C_DurationUtil.CreateDuration` in the Lua globals.
///
/// Also registers `C_DurationUtil.GetCurrentTime` if not already present.
pub fn register_lua_duration_object(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    let t: mlua::Table = match g.get::<Value>("C_DurationUtil")? {
        Value::Table(t) => t,
        _ => lua.create_table()?,
    };

    t.set(
        "CreateDuration",
        lua.create_function(|_, ()| Ok(LuaDurationObject::new()))?,
    )?;

    if t.get::<Value>("GetCurrentTime")?.is_nil() {
        t.set(
            "GetCurrentTime",
            lua.create_function(|_, _: MultiValue| Ok(Value::Integer(0)))?,
        )?;
    }

    g.set("C_DurationUtil", t)?;
    Ok(())
}
