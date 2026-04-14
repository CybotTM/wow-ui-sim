//! Macros for ergonomic method and function registration on rilua.
//!
//! These macros generate `RustFn`-compatible closures (actually function items)
//! that handle argument extraction via [`FromStack`] and return-value injection
//! via [`IntoStack`], keeping call-site boilerplate minimal.
//!
//! # Method registration (`define_methods!`)
//!
//! Generates entries for a metatable where the first Lua argument (`self`)
//! is a frame-backed table. The macro resolves the backing frame from the
//! widget arena and calls the provided closure.
//!
//! **Usage:**
//! ```rust,ignore
//! define_methods!(state, metatable, {
//!     "SetText" => |frame: &mut Frame, text: String| {
//!         frame.text = Some(text);
//!         Ok(())
//!     },
//!     "GetText" => |frame: &Frame| -> Option<String> {
//!         Ok(frame.text.clone())
//!     },
//! });
//! ```
//!
//! **Expansion pattern** (per method entry):
//!
//! 1. `state.stack[state.base - 1]` is the function object itself.
//! 2. Argument 1 (`FromStack` at index 1) is `self` — a `Val::Table` or
//!    `Val::Userdata` that carries a frame ID in its backing store.
//! 3. The frame ID is used to borrow `&mut Frame` from the widget arena
//!    (stored in `state` app_data — implementation detail TBD).
//! 4. Remaining arguments are extracted in order starting at index 2.
//! 5. The closure is called and its `LuaResult<R>` mapped through
//!    `IntoStack` to push return values.
//!
//! # Function registration (`define_functions!`)
//!
//! Like `define_methods!` but without a `self` frame argument. Arguments
//! start at index 1.
//!
//! **Usage:**
//! ```rust,ignore
//! define_functions!(state, table, {
//!     "CreateFrame" => |frame_type: String, name: Option<String>| {
//!         // ...
//!         Ok(Val::Nil)
//!     },
//! });
//! ```

// ---------------------------------------------------------------------------
// define_methods!
// ---------------------------------------------------------------------------

/// Register frame methods on a rilua table (metatable).
///
/// Each arm becomes a `RustFn` that:
/// - Extracts the frame handle from argument 1 (self)
/// - Resolves the backing `Frame` from the widget arena
/// - Extracts remaining arguments starting at index 2
/// - Calls the closure and pushes the result
///
/// The actual `Frame` type and arena resolution will be filled in once
/// the widget arena is available in this crate.
///
/// TODO: Replace `_frame_unused: ()` placeholder with real arena lookup.
#[macro_export]
macro_rules! define_methods {
    ($state:expr, $table:expr, {
        $( $name:literal => |$frame_pat:pat $(, $arg_pat:pat : $arg_ty:ty)* $(,)?| $(-> $ret_ty:ty)? $body:block ),* $(,)?
    }) => {
        $(
            {
                // Generate a named helper function to satisfy `RustFn = fn(...)`,
                // which requires a concrete function pointer (not a closure).
                // The body is placed inline below; in practice each arm expands
                // to a `table_set_function` call with a wrapper `fn`.
                //
                // TODO: Implement frame resolution from arena when widget types
                //       are accessible from this crate.
                fn __method(state: &mut ::rilua::vm::state::LuaState) -> ::rilua::LuaResult<u32> {
                    // arg 1 = self (frame handle) — will be resolved to &mut Frame.
                    // For now we validate it exists and is not nil.
                    let _self_val = $crate::lua_bridge::from_stack::stack_val(state, 1);

                    // TODO: resolve _self_val to &mut Frame via arena lookup.
                    let $frame_pat = (); // placeholder until Frame type is wired up

                    // Extract remaining arguments starting at position 2.
                    let mut __idx: i32 = 2;
                    $(
                        let $arg_pat: $arg_ty = <$arg_ty as $crate::lua_bridge::FromStack>::from_stack(state, __idx)?;
                        __idx += 1;
                    )*
                    let _ = __idx;

                    let __result: ::rilua::LuaResult<_> = $body;
                    let __val = __result?;
                    $crate::lua_bridge::IntoStack::into_stack(__val, state)
                }
                $crate::lua_bridge::table_builder::table_set_rust_fn($state, $table, $name, __method)?;
            }
        )*
    };
}

// ---------------------------------------------------------------------------
// define_functions!
// ---------------------------------------------------------------------------

/// Register global (non-method) Rust functions on a rilua table.
///
/// Each arm becomes a `RustFn` that extracts arguments starting at index 1
/// and pushes return values.
///
/// TODO: Support closures capturing state once rilua adds `RustFnMut`.
#[macro_export]
macro_rules! define_functions {
    ($state:expr, $table:expr, {
        $( $name:literal => |$( $arg_pat:pat : $arg_ty:ty ),* $(,)?| $(-> $ret_ty:ty)? $body:block ),* $(,)?
    }) => {
        $(
            {
                fn __func(state: &mut ::rilua::vm::state::LuaState) -> ::rilua::LuaResult<u32> {
                    let mut __idx: i32 = 1;
                    $(
                        let $arg_pat: $arg_ty = <$arg_ty as $crate::lua_bridge::FromStack>::from_stack(state, __idx)?;
                        __idx += 1;
                    )*
                    let _ = __idx;

                    let __result: ::rilua::LuaResult<_> = $body;
                    let __val = __result?;
                    $crate::lua_bridge::IntoStack::into_stack(__val, state)
                }
                $crate::lua_bridge::table_builder::table_set_rust_fn($state, $table, $name, __func)?;
            }
        )*
    };
}
