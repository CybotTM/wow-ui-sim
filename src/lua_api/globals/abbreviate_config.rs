//! AbbreviateConfig table-backed proxy for WoW number abbreviation configuration.
//!
//! Implements `CreateAbbreviateConfig(configTable)` which returns a table proxy
//! wrapping a hidden userdata. The proxy supports `GetAbbreviateNumberData` /
//! `SetAbbreviateNumberData` methods and arbitrary per-instance field storage via
//! the userdata's user-value table.

use crate::lua_api::proxy_helpers::{lookup_registered_method, proxy_userdata, wrap_fn_with_userdata};
use mlua::{AnyUserData, Lua, Result, UserData, UserDataMethods, Value};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

const PROXY_MT_KEY: &str = "__abbreviate_config_proxy_mt";
const BIND_METHOD_KEY: &str = "__abbreviate_config_bind_method_helper";

/// Method names that are read-only (cannot be assigned by user code).
const METHOD_NAMES: &[&str] = &["GetAbbreviateNumberData", "SetAbbreviateNumberData"];

/// Metamethod names that are read-only.
const META_NAMES: &[&str] = &["__eq", "__index", "__metatable", "__newindex", "__tostring"];

/// Hidden userdata backing each AbbreviateConfig proxy.
pub struct AbbreviateConfig {
    id: u64,
}

impl AbbreviateConfig {
    fn new() -> Self {
        AbbreviateConfig {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}

impl UserData for AbbreviateConfig {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("GetAbbreviateNumberData", |_, ud: AnyUserData| {
            let fields: mlua::Table = ud.user_value()?;
            let data: Value = fields.raw_get("__data")?;
            Ok(data)
        });

        methods.add_function("SetAbbreviateNumberData", |_, (ud, data): (AnyUserData, Value)| {
            let fields: mlua::Table = ud.user_value()?;
            fields.raw_set("__data", data)?;
            Ok(())
        });
    }
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
            return Ok("AbbreviateConfig: 0x0000000000000000".to_string());
        };
        let id = userdata
            .borrow::<AbbreviateConfig>()
            .map(|c| c.id)
            .unwrap_or(0);
        Ok(format!("AbbreviateConfig: 0x{:016x}", id))
    })
}

fn is_readonly_key(key: &str) -> bool {
    METHOD_NAMES.contains(&key) || META_NAMES.contains(&key)
}

/// Register `CreateAbbreviateConfig` in the Lua globals.
pub fn register_abbreviate_config(lua: &Lua) -> Result<()> {
    ensure_proxy_support(lua)?;
    lua.globals().set(
        "CreateAbbreviateConfig",
        lua.create_function(|lua, _config: Value| {
            ensure_proxy_support(lua)?;
            let userdata = lua.create_userdata(AbbreviateConfig::new())?;
            create_proxy(lua, userdata)
        })?,
    )
}
