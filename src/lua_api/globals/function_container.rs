//! LuaFunctionContainer UserData type for WoW C_FunctionContainers API.
//!
//! Implements `C_FunctionContainers.CreateCallback(func)` which returns a UserData
//! object with `Cancel`, `IsCancelled`, and `Invoke` methods.
//!
//! Also used as the handle returned by `C_Timer.NewTimer` and `C_Timer.NewTicker`.
//!
//! The metatable is automatically hidden by mlua (`getmetatable` returns `false`).

use mlua::{AnyUserData, Lua, MetaMethod, Result, UserData, UserDataMethods, Value};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FC_ID: AtomicU64 = AtomicU64::new(1);

/// Method names that are read-only (cannot be overwritten via __newindex).
const METHOD_NAMES: &[&str] = &["Cancel", "IsCancelled", "Invoke"];

/// Shared interior state for a FunctionContainer instance pair (original + proxy).
pub struct FcInner {
    /// Whether this container has been cancelled.
    pub cancelled: Cell<bool>,
    /// Optional timer ID for cancelling associated timers.
    pub timer_id: Option<u64>,
    /// Per-instance user field storage, shared between original and proxy.
    pub fields: RefCell<HashMap<String, Value>>,
}

impl FcInner {
    fn new(timer_id: Option<u64>) -> Rc<Self> {
        Rc::new(FcInner {
            cancelled: Cell::new(false),
            timer_id,
            fields: RefCell::new(HashMap::new()),
        })
    }
}

/// WoW LuaFunctionContainer userdata object.
///
/// Wraps a Lua function with `Cancel`/`IsCancelled`/`Invoke` semantics.
/// Arbitrary field storage is supported via `__index`/`__newindex`.
/// Methods are read-only (assignment fails with WoW's error message).
pub struct FunctionContainer {
    /// Unique ID for this container pair (used for `__eq` comparison via Rc identity).
    pub fc_id: u64,
    /// The wrapped Lua function (stored in Lua registry).
    pub callback: mlua::RegistryKey,
    /// Shared state between original and proxy (cancelled flag, timer linkage, fields).
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
    /// - Fields set on the original are visible on the proxy
    /// - Cancelling either cancels both
    ///
    /// The proxy is a distinct Lua UserData object (different pointer), so
    /// table key lookup `{ [original] = true }[proxy]` returns nil.
    pub fn new_proxy(lua: &Lua, original: &FunctionContainer) -> Result<Self> {
        let fc_id = NEXT_FC_ID.fetch_add(1, Ordering::Relaxed);
        let orig_func = lua.registry_value::<mlua::Function>(&original.callback)?;
        let callback_key = lua.create_registry_value(orig_func)?;
        Ok(FunctionContainer {
            fc_id,
            callback: callback_key,
            inner: Rc::clone(&original.inner),
            state: original.state.as_ref().map(Rc::clone),
        })
    }
}

impl UserData for FunctionContainer {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        add_fc_methods(methods);
        add_fc_index(methods);
        add_fc_newindex(methods);
        add_fc_eq(methods);
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(format!("LuaFunctionContainer: 0x{:016x}", this.fc_id))
        });
    }
}

fn add_fc_methods<M: UserDataMethods<FunctionContainer>>(methods: &mut M) {
    methods.add_method("Cancel", |_, this, ()| {
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

    methods.add_method("IsCancelled", |_, this, ()| {
        Ok(this.inner.cancelled.get())
    });

    methods.add_method("Invoke", |lua, this, args: mlua::MultiValue| {
        if this.inner.cancelled.get() {
            return Ok(());
        }
        let callback = lua.registry_value::<mlua::Function>(&this.callback)?;
        // Invoke calls the function but discards all return values.
        // We ignore errors to match WoW's pcall-like behavior.
        let _ = callback.call::<mlua::MultiValue>(args);
        Ok(())
    });
}

fn add_fc_index<M: UserDataMethods<FunctionContainer>>(methods: &mut M) {
    methods.add_meta_function(
        MetaMethod::Index,
        |_lua: &Lua, (ud, key): (AnyUserData, Value)| {
            let fc = ud.borrow::<FunctionContainer>()?;
            let inner = Rc::clone(&fc.inner);
            drop(fc);

            let key_str = match &key {
                Value::String(s) => s.to_string_lossy().to_string(),
                _ => return Ok(Value::Nil),
            };

            // Metamethods are not exposed through __index
            if key_str.starts_with("__") {
                return Ok(Value::Nil);
            }

            // Method names: mlua's generated __index checks methods table first,
            // then calls our __index. So we only reach here for non-method keys.
            let fields = inner.fields.borrow();
            Ok(fields.get(&key_str).cloned().unwrap_or(Value::Nil))
        },
    );
}

fn add_fc_newindex<M: UserDataMethods<FunctionContainer>>(methods: &mut M) {
    methods.add_meta_function(
        MetaMethod::NewIndex,
        |_lua: &Lua, (ud, key, value): (AnyUserData, String, Value)| {
            let fc = ud.borrow::<FunctionContainer>()?;
            let inner = Rc::clone(&fc.inner);
            drop(fc);

            // Block method name assignment
            if METHOD_NAMES.contains(&key.as_str()) {
                return Err(mlua::Error::RuntimeError(format!(
                    "Attempted to assign to read-only key {}",
                    key
                )));
            }

            // Block metamethod assignment
            if key.starts_with("__") {
                return Err(mlua::Error::RuntimeError(format!(
                    "Attempted to assign to read-only key {}",
                    key
                )));
            }

            // Store or remove from per-instance field table
            let mut fields = inner.fields.borrow_mut();
            if let Value::Nil = value {
                fields.remove(&key);
            } else {
                fields.insert(key, value);
            }
            Ok(())
        },
    );
}

fn add_fc_eq<M: UserDataMethods<FunctionContainer>>(methods: &mut M) {
    methods.add_meta_method(MetaMethod::Eq, |_, this, other: AnyUserData| {
        let other_fc = other.borrow::<FunctionContainer>()?;
        // Equal if they share the same inner state (original and proxy pairs).
        Ok(Rc::ptr_eq(&this.inner, &other_fc.inner))
    });
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
    let t = lua.create_table()?;

    t.set(
        "CreateCallback",
        lua.create_function(|lua, func: mlua::Function| {
            // Reject C functions - WoW only accepts Lua functions
            if !is_lua_function(lua, &func)? {
                return Err(mlua::Error::RuntimeError(
                    "Usage: C_FunctionContainers.CreateCallback(func)".to_string(),
                ));
            }
            FunctionContainer::new(lua, func, None)
        })?,
    )?;

    lua.globals().set("C_FunctionContainers", t)?;
    Ok(())
}
