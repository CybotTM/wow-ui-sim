//! LuaDurationObject table-backed proxy for WoW duration tracking.
//!
//! Implements `C_DurationUtil.CreateDuration()` which returns a table proxy
//! wrapping a hidden userdata. The proxy supports all duration methods and
//! arbitrary per-instance field storage via the userdata's user-value table.

use crate::lua_api::proxy_helpers::{lookup_registered_method, proxy_userdata, wrap_fn_with_userdata};
use mlua::{AnyUserData, Lua, MultiValue, Result, UserData, UserDataMethods, Value};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

const PROXY_MT_KEY: &str = "__lua_duration_proxy_mt";
const BIND_METHOD_KEY: &str = "__lua_duration_bind_method_helper";

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
}

impl UserData for LuaDurationObject {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_copy_and_assign_methods(methods);
        add_evaluate_methods(methods);
        add_getter_methods(methods);
        add_setter_methods(methods);
    }
}

/// Returns true if the key is a method or metamethod name (read-only).
fn is_readonly_key(key: &str) -> bool {
    METHOD_NAMES.contains(&key) || META_NAMES.contains(&key)
}

/// Register Copy and Assign methods.
fn add_copy_and_assign_methods<M: UserDataMethods<LuaDurationObject>>(methods: &mut M) {
    methods.add_function("Assign", |_, (_ud, _other): (AnyUserData, Value)| Ok(()));
    methods.add_function("Copy", |lua, _ud: AnyUserData| {
        ensure_proxy_support(lua)?;
        let new_userdata = lua.create_userdata(LuaDurationObject::new())?;
        create_proxy(lua, new_userdata)
    });
}

/// Register Evaluate* methods (return 0.0 stubs).
fn add_evaluate_methods<M: UserDataMethods<LuaDurationObject>>(methods: &mut M) {
    methods.add_function("EvaluateElapsedDuration", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("EvaluateElapsedPercent", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("EvaluateRemainingDuration", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("EvaluateRemainingPercent", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
}

/// Register Get* query methods (return 0.0 / false stubs).
fn add_getter_methods<M: UserDataMethods<LuaDurationObject>>(methods: &mut M) {
    methods.add_function("GetClockTime", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("GetElapsedDuration", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("GetElapsedPercent", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("GetEndTime", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("GetModRate", |_, (_ud,): (AnyUserData,)| Ok(1.0f64));
    methods.add_function("GetRemainingDuration", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("GetRemainingPercent", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("GetStartTime", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("GetTotalDuration", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(0.0f64));
    methods.add_function("HasSecretValues", |_, (_ud,): (AnyUserData,)| Ok(false));
    methods.add_function("IsZero", |_, (_ud,): (AnyUserData,)| Ok(true));
}

/// Register Set* and Reset mutating methods (no-op stubs).
fn add_setter_methods<M: UserDataMethods<LuaDurationObject>>(methods: &mut M) {
    methods.add_function("Reset", |_, (_ud,): (AnyUserData,)| Ok(()));
    methods.add_function("SetTimeFromEnd", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(()));
    methods.add_function("SetTimeFromStart", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(()));
    methods.add_function("SetTimeSpan", |_, (_ud, _args): (AnyUserData, MultiValue)| Ok(()));
    methods.add_function("SetToDefaults", |_, (_ud,): (AnyUserData,)| Ok(()));
}

fn ensure_proxy_support(lua: &Lua) -> Result<()> {
    register_bind_method_helper(lua)?;
    install_proxy_metatable(lua)
}

fn register_bind_method_helper(lua: &Lua) -> Result<()> {
    if lua
        .named_registry_value::<mlua::Function>(BIND_METHOD_KEY)
        .is_ok()
    {
        return Ok(());
    }
    lua.set_named_registry_value(
        BIND_METHOD_KEY,
        crate::lua_api::cfunc_wrap::create_bind_factory(lua)?,
    )
}

fn install_proxy_metatable(lua: &Lua) -> Result<()> {
    if lua
        .named_registry_value::<mlua::Table>(PROXY_MT_KEY)
        .is_ok()
    {
        return Ok(());
    }
    let mt = create_proxy_metatable(lua)?;
    lua.set_named_registry_value(PROXY_MT_KEY, mt)
}

fn create_proxy(lua: &Lua, userdata: mlua::AnyUserData) -> Result<Value> {
    userdata.set_user_value(lua.create_table()?)?;
    let proxy = lua.create_table()?;
    proxy.raw_set("__lud", userdata)?;
    let mt: mlua::Table = lua.named_registry_value(PROXY_MT_KEY)?;
    proxy.set_metatable(Some(mt));
    Ok(Value::Table(proxy))
}

fn create_proxy_metatable(lua: &Lua) -> Result<mlua::Table> {
    let mt = lua.create_table()?;
    mt.raw_set("__index", create_proxy_index(lua)?)?;
    mt.raw_set("__newindex", create_proxy_newindex(lua)?)?;
    mt.raw_set("__tostring", create_proxy_tostring(lua)?)?;
    Ok(mt)
}

fn create_proxy_index(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|lua, (this, key): (mlua::Table, Value)| {
        // Metamethod names always return nil.
        if let Value::String(ref s) = key {
            if s.to_string_lossy().starts_with("__") {
                return Ok(Value::Nil);
            }
        }

        let proxy_value = Value::Table(this);
        let Some(userdata) = proxy_userdata(&proxy_value) else {
            return Ok(Value::Nil);
        };

        // Check per-instance fields first.
        if let Ok(fields) = userdata.user_value::<mlua::Table>() {
            let field_value: Value = fields.raw_get(key.clone())?;
            if !field_value.is_nil() {
                return Ok(field_value);
            }
        }

        // Fall back to registered methods on the userdata metatable.
        let registered = lookup_registered_method(&userdata, &key)?;
        if let Value::Function(function) = registered {
            return Ok(Value::Function(wrap_fn_with_userdata(
                lua, function, userdata, BIND_METHOD_KEY,
            )?));
        }
        Ok(registered)
    })
}

fn create_proxy_newindex(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|_, (this, key, value): (mlua::Table, Value, Value)| {
        // Reject writes to method and metamethod names.
        if let Value::String(ref s) = key {
            let key_str = s.to_string_lossy();
            if is_readonly_key(&key_str) {
                return Err(mlua::Error::RuntimeError(format!(
                    "Attempted to assign to read-only key {}",
                    key_str
                )));
            }
        }

        let proxy_value = Value::Table(this);
        let Some(userdata) = proxy_userdata(&proxy_value) else {
            return Ok(());
        };
        let fields: mlua::Table = userdata.user_value()?;
        fields.raw_set(key, value)?;
        Ok(())
    })
}

fn create_proxy_tostring(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|_, this: mlua::Table| {
        let proxy_value = Value::Table(this);
        let Some(userdata) = proxy_userdata(&proxy_value) else {
            return Ok("LuaDurationObject: 0x0000000000000000".to_string());
        };
        let id = userdata
            .borrow::<LuaDurationObject>()
            .map(|c| c.id)
            .unwrap_or(0);
        Ok(format!("LuaDurationObject: 0x{id:016x}"))
    })
}

/// Register `C_DurationUtil.CreateDuration` in the Lua globals.
///
/// Also registers `C_DurationUtil.GetCurrentTime` if not already present.
pub fn register_lua_duration_object(lua: &Lua) -> Result<()> {
    ensure_proxy_support(lua)?;
    let g = lua.globals();
    let t: mlua::Table = match g.get::<Value>("C_DurationUtil")? {
        Value::Table(t) => t,
        _ => lua.create_table()?,
    };

    t.set(
        "CreateDuration",
        lua.create_function(|lua, ()| {
            ensure_proxy_support(lua)?;
            let userdata = lua.create_userdata(LuaDurationObject::new())?;
            create_proxy(lua, userdata)
        })?,
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
