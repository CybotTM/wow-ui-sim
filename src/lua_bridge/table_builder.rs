//! [`TableBuilder`]: fluent builder for creating and populating Lua tables.

use rilua::vm::closure::{Closure, RustClosure, RustFn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table as RiluaTable;
use rilua::LuaError;
use rilua::LuaResult;
use rilua::RuntimeError;
use rilua::Val;

use crate::lua_bridge::IntoStack;

// ---------------------------------------------------------------------------
// Free helper (also used by macros)
// ---------------------------------------------------------------------------

/// Register a named `RustFn` on an existing table `GcRef`.
///
/// Used by the `define_methods!` and `define_functions!` macros so they
/// do not need to go through a full `TableBuilder`.
pub fn table_set_rust_fn(
    state: &mut LuaState,
    table_ref: GcRef<RiluaTable>,
    name: &str,
    func: RustFn,
) -> LuaResult<()> {
    let key_ref = state.gc.intern_string(name.as_bytes());
    let key = Val::Str(key_ref);
    let closure = Closure::Rust(RustClosure::new(func, name));
    let closure_ref = state.gc.alloc_closure(closure);
    let table = state.gc.tables.get_mut(table_ref).ok_or_else(|| {
        LuaError::Runtime(RuntimeError {
            message: "table has been collected".into(),
            level: 0,
            traceback: vec![],
        })
    })?;
    table.raw_set(key, Val::Function(closure_ref), &state.gc.string_arena)
}

/// Allocate a Lua table with wow-ui-sim's frame backing metadata attached.
///
/// The returned table is still a normal Lua table: Lua code can `rawset`,
/// iterate, and attach a metatable as usual.
pub fn create_frame_table(state: &mut LuaState, index: u32, generation: u32) -> GcRef<RiluaTable> {
    let mut table = RiluaTable::new();
    table.set_backing(Some((index, generation)));
    state.gc.alloc_table(table)
}

// ---------------------------------------------------------------------------
// TableBuilder
// ---------------------------------------------------------------------------

/// Fluent builder for creating Lua tables with typed values and Rust functions.
///
/// ```rust,ignore
/// let t = TableBuilder::new(state)
///     .set("version", "1.0")?
///     .set_function("GetVersion", |_state| { ... })?
///     .build();
/// ```
pub struct TableBuilder<'a> {
    state: &'a mut LuaState,
    table_ref: GcRef<RiluaTable>,
}

impl<'a> TableBuilder<'a> {
    /// Allocates a new empty table and returns a builder for it.
    pub fn new(state: &'a mut LuaState) -> Self {
        let table_ref = state.gc.alloc_table(RiluaTable::new());
        Self { state, table_ref }
    }

    /// Set a string-keyed entry to any value implementing [`IntoStack`].
    ///
    /// `IntoStack` is used only to convert the value to `Val`; no stack
    /// slot is consumed — the value is pushed and immediately popped to
    /// set the table entry.
    pub fn set(self, key: &str, value: impl IntoStack) -> LuaResult<Self> {
        // Convert through a temporary stack push + pop to obtain the Val.
        let save_top = self.state.top;
        let count = value.into_stack(self.state)?;
        let val = if count > 0 {
            self.state.stack[save_top]
        } else {
            Val::Nil
        };
        self.state.top = save_top; // restore top (undo the temporary push)

        let key_ref = self.state.gc.intern_string(key.as_bytes());
        let k = Val::Str(key_ref);
        let table = self
            .state
            .gc
            .tables
            .get_mut(self.table_ref)
            .ok_or_else(|| {
                LuaError::Runtime(RuntimeError {
                    message: "table has been collected".into(),
                    level: 0,
                    traceback: vec![],
                })
            })?;
        table.raw_set(k, val, &self.state.gc.string_arena)?;
        Ok(self)
    }

    /// Set a string-keyed entry to a named Rust function.
    pub fn set_function(self, name: &str, func: RustFn) -> LuaResult<Self> {
        table_set_rust_fn(self.state, self.table_ref, name, func)?;
        Ok(self)
    }

    /// Return the underlying `GcRef` for direct use with the macro helpers.
    pub fn table_ref(&self) -> GcRef<RiluaTable> {
        self.table_ref
    }

    /// Finish building and return the table as a `Val::Table`.
    pub fn build(self) -> Val {
        Val::Table(self.table_ref)
    }
}
