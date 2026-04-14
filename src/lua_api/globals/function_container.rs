//! LuaFunctionContainer table-backed proxy for WoW C_FunctionContainers API.
//!
//! Implements `C_FunctionContainers.CreateCallback(func)` which returns a table proxy
//! wrapping a hidden userdata. The proxy supports `Cancel`, `IsCancelled`, and `Invoke`
//! methods and arbitrary per-instance field storage via the userdata's user-value table.
//!
//! Also used as the handle returned by `C_Timer.NewTimer` and `C_Timer.NewTicker`.

use crate::lua_api::proxy_helpers::{lookup_registered_method, proxy_userdata, wrap_fn_with_userdata};
use crate::lua_api::script_helpers::lua_error;
use mlua::{AnyUserData, Lua, Result, UserData, UserDataMethods, Value};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FC_ID: AtomicU64 = AtomicU64::new(1);

const PROXY_MT_KEY: &str = "__fc_proxy_mt";
const BIND_METHOD_KEY: &str = "__fc_bind_method_helper";

/// Method names that are read-only (cannot be overwritten via __newindex).
const METHOD_NAMES: &[&str] = &["Cancel", "IsCancelled", "Invoke"];

/// Metamethod names that are read-only.
const META_NAMES: &[&str] = &["__eq", "__index", "__metatable", "__newindex", "__tostring"];

/// Shared interior state for a FunctionContainer instance pair (original + proxy).
pub struct FcInner {
    /// Whether this container has been cancelled.
    pub cancelled: Cell<bool>,
    /// Optional timer ID for cancelling associated timers.
    pub timer_id: Option<u64>,
}

impl FcInner {
    fn new(timer_id: Option<u64>) -> Rc<Self> {
        Rc::new(FcInner {
            cancelled: Cell::new(false),
            timer_id,
        })
    }
}

/// WoW LuaFunctionContainer userdata object.
///
/// Wraps a Lua function with `Cancel`/`IsCancelled`/`Invoke` semantics.
/// Arbitrary field storage is supported via the userdata's user-value table.
/// Methods are read-only (assignment fails with WoW's error message).
pub struct FunctionContainer {
    /// Unique ID for this container pair (used for `__tostring` and `__eq` comparison via Rc identity).
    pub fc_id: u64,
    /// The wrapped Lua function (stored in Lua registry).
    pub callback: mlua::RegistryKey,
    /// Shared state between original and proxy (cancelled flag, timer linkage).
    pub inner: Rc<FcInner>,
    /// SimState reference for timer cancellation (None for plain CreateCallback).
    pub state: Option<Rc<RefCell<crate::lua_api::SimState>>>,
}

impl FunctionContainer {
    /// Create a new FunctionContainer wrapping the given Lua function.
    pub fn new(
        lua: &Lua,
        callback: mlua::Function,
        state: Option<Rc<RefCell<crate::lua_api::SimState>>>,
    ) -> Result<Self> {
        let fc_id = NEXT_FC_ID.fetch_add(1, Ordering::Relaxed);
        let callback_key = lua.create_registry_value(callback)?;
        Ok(FunctionContainer {
            fc_id,
            callback: callback_key,
            inner: FcInner::new(None),
            state,
        })
    }

    /// Create a new FunctionContainer for a timer, sharing inner state with the original.
    pub fn new_timer(
        lua: &Lua,
        callback: mlua::Function,
        state: Rc<RefCell<crate::lua_api::SimState>>,
        timer_id: u64,
    ) -> Result<Self> {
        let fc_id = NEXT_FC_ID.fetch_add(1, Ordering::Relaxed);
        let callback_key = lua.create_registry_value(callback)?;
        Ok(FunctionContainer {
            fc_id,
            callback: callback_key,
            inner: FcInner::new(Some(timer_id)),
            state: Some(state),
        })
    }

    /// Create a proxy FunctionContainer that shares inner state with the original.
    ///
    /// The proxy has the same `Rc<FcInner>` as the original, so:
    /// - `proxy == original` via `__eq` (same Rc pointer identity)
    /// - Cancelling either cancels both
    ///
    /// The proxy is a distinct Lua UserData object (different pointer), so
    /// table key lookup `{ [original] = true }[proxy]` returns nil.
    pub fn new_proxy(lua: &Lua, original: &FunctionContainer) -> Result<Self> {
        let orig_func = lua.registry_value::<mlua::Function>(&original.callback)?;
        let callback_key = lua.create_registry_value(orig_func)?;
        Ok(FunctionContainer {
            fc_id: original.fc_id, // Same display ID — tostring matches
            callback: callback_key,
            inner: Rc::clone(&original.inner),
            state: original.state.as_ref().map(Rc::clone),
        })
    }
}

impl UserData for FunctionContainer {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("Cancel", |_, ud: AnyUserData| {
            let this = ud.borrow::<FunctionContainer>()?;
            this.inner.cancelled.set(true);
            if let (Some(timer_id), Some(state)) = (this.inner.timer_id, this.state.as_ref()) {
                let mut st = state.borrow_mut();
                for timer in st.timers.iter_mut() {
                    if timer.id == timer_id {
                        timer.cancelled = true;
                        break;
                    }
                }
            }
            Ok(())
        });

        methods.add_function("IsCancelled", |_, ud: AnyUserData| {
            let this = ud.borrow::<FunctionContainer>()?;
            Ok(this.inner.cancelled.get())
        });

        methods.add_function("Invoke", |lua, (ud, args): (AnyUserData, mlua::MultiValue)| {
            let this = ud.borrow::<FunctionContainer>()?;
            if this.inner.cancelled.get() {
                return Ok(());
            }
            let callback = lua.registry_value::<mlua::Function>(&this.callback)?;
            drop(this);
            // Invoke calls the function but discards all return values.
            // We ignore errors to match WoW's pcall-like behavior.
            let _ = callback.call::<mlua::MultiValue>(args);
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

/// Create a table proxy wrapping the given FunctionContainer userdata.
///
/// The proxy is `{__lud = userdata}` with a metatable providing `__index`,
/// `__newindex`, `__tostring`, and `__eq`. Dynamic fields are stored in the
/// userdata's user-value table.
pub fn create_fc_table_proxy(lua: &Lua, userdata: mlua::AnyUserData) -> Result<Value> {
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
    mt.raw_set("__eq", create_proxy_eq(lua)?)?;
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
            return Ok("LuaFunctionContainer: 0x0000000000000000".to_string());
        };
        let id = userdata
            .borrow::<FunctionContainer>()
            .map(|c| c.fc_id)
            .unwrap_or(0);
        Ok(format!("LuaFunctionContainer: 0x{:016x}", id))
    })
}

fn create_proxy_eq(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|_, (a, b): (Value, Value)| {
        let Some(ud_a) = proxy_userdata(&a) else {
            return Ok(false);
        };
        let Some(ud_b) = proxy_userdata(&b) else {
            return Ok(false);
        };
        let Ok(fc_a) = ud_a.borrow::<FunctionContainer>() else {
            return Ok(false);
        };
        let Ok(fc_b) = ud_b.borrow::<FunctionContainer>() else {
            return Ok(false);
        };
        Ok(Rc::ptr_eq(&fc_a.inner, &fc_b.inner))
    })
}

fn is_readonly_key(key: &str) -> bool {
    METHOD_NAMES.contains(&key) || META_NAMES.contains(&key)
}

/// Check if a Lua function is a pure Lua function (not a C function).
///
/// WoW's CreateCallback rejects C functions (like pcall, print, etc.).
/// Uses debug.iscfunction to distinguish Lua vs C functions.
fn is_lua_function(lua: &Lua, func: &mlua::Function) -> Result<bool> {
    let debug: mlua::Table = lua.globals().get("debug")?;
    let iscfunction: mlua::Function = debug.get("iscfunction")?;
    let is_c: bool = iscfunction.call(func.clone())?;
    Ok(!is_c)
}

/// Register `C_FunctionContainers` namespace in the Lua globals.
pub fn register_c_function_containers(lua: &Lua) -> Result<()> {
    ensure_proxy_support(lua)?;
    let t = lua.create_table()?;

    t.set(
        "CreateCallback",
        lua.create_function(|lua, arg: Value| {
            let func = match arg {
                Value::Function(f) => {
                    if !is_lua_function(lua, &f)? {
                        return Err(lua_error(
                            lua,
                            "Usage: C_FunctionContainers.CreateCallback(func)",
                        ));
                    }
                    f
                }
                _ => {
                    return Err(lua_error(
                        lua,
                        "Usage: C_FunctionContainers.CreateCallback(func)",
                    ));
                }
            };
            ensure_proxy_support(lua)?;
            let fc = FunctionContainer::new(lua, func, None)?;
            let userdata = lua.create_userdata(fc)?;
            create_fc_table_proxy(lua, userdata)
        })?,
    )?;

    lua.globals().set("C_FunctionContainers", t)?;
    Ok(())
}
