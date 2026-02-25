//! AbbreviateConfig UserData type for WoW number abbreviation configuration.
//!
//! Implements `CreateAbbreviateConfig(configTable)` which returns a UserData object
//! with `GetAbbreviateNumberData`/`SetAbbreviateNumberData` methods and per-instance
//! field storage. The metatable is hidden (`getmetatable` returns `false`), which is
//! the default mlua behaviour for all UserData types.

use mlua::{AnyUserData, Lua, MetaMethod, Result, UserData, UserDataMethods, Value};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Global Lua table name used to store per-instance custom fields.
const FIELDS_TABLE: &str = "__abbreviate_config_fields";

/// Method names that are read-only (cannot be assigned by user code).
const METHOD_NAMES: &[&str] = &["GetAbbreviateNumberData", "SetAbbreviateNumberData"];

/// Metamethod names that are read-only.
const META_NAMES: &[&str] = &["__eq", "__index", "__metatable", "__newindex", "__tostring"];

/// WoW AbbreviateConfig userdata object.
///
/// Exposes two methods required by the WoW API:
/// - `GetAbbreviateNumberData()` - returns the stored config data (table or nil)
/// - `SetAbbreviateNumberData(data)` - stores a new config data value
///
/// Additionally supports arbitrary field storage via `__index`/`__newindex` so
/// that addon code can attach arbitrary Lua values to the object instance.
pub struct AbbreviateConfig {
    id: u64,
}

impl AbbreviateConfig {
    fn new() -> Self {
        AbbreviateConfig {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    fn add_data_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetAbbreviateNumberData", |lua, this, ()| {
            let fields = get_instance_fields(lua, this.id);
            let data: Value = fields
                .and_then(|t| t.get("__data").ok())
                .unwrap_or(Value::Nil);
            Ok(data)
        });

        methods.add_method("SetAbbreviateNumberData", |lua, this, data: Value| {
            get_or_create_instance_fields(lua, this.id).set("__data", data)?;
            Ok(())
        });
    }

    fn add_index_metamethod<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(
            MetaMethod::Index,
            |lua: &Lua, (ud, key): (AnyUserData, Value)| {
                let handle = ud.borrow::<AbbreviateConfig>()?;
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
                let handle = ud.borrow::<AbbreviateConfig>()?;
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

impl UserData for AbbreviateConfig {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        Self::add_data_methods(methods);
        Self::add_index_metamethod(methods);
        Self::add_newindex_metamethod(methods);
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(format!("AbbreviateConfig: 0x{:016x}", this.id))
        });
    }
}

/// Returns true if the key is a method or metamethod name (read-only).
fn is_readonly_key(key: &str) -> bool {
    METHOD_NAMES.contains(&key) || META_NAMES.contains(&key)
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

/// Register `CreateAbbreviateConfig` in the Lua globals.
pub fn register_abbreviate_config(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "CreateAbbreviateConfig",
        lua.create_function(|_, _config: Value| Ok(AbbreviateConfig::new()))?,
    )
}
