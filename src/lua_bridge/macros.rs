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
//! 2. Argument 1 is `self` — a frame-backed `Val::Table`.
//! 3. The bridge resolves it through [`FromMethodSelf`] into `&Frame` or
//!    `&mut Frame` using the arena stored in `state` app_data.
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
#[macro_export]
macro_rules! define_methods {
    ($state:expr, $table:expr, {
        $( $name:literal => |$frame_pat:ident : $frame_ty:ty $(, $arg_pat:ident : $arg_ty:ty)* $(,)?| $(-> $ret_ty:ty)? $body:block ),* $(,)?
    }) => {
        {
            let __result: ::rilua::LuaResult<()> = (|| {
                $(
                    {
                        fn __method(state: &mut ::rilua::vm::state::LuaState) -> ::rilua::LuaResult<u32> {
                            let mut __idx: i32 = 2;
                            $(
                                let $arg_pat: $arg_ty = <$arg_ty as $crate::lua_bridge::FromStack>::from_stack(state, __idx)?;
                                __idx += 1;
                            )*
                            let _ = __idx;

                            let __result: ::rilua::LuaResult<_> = {
                                let $frame_pat: $frame_ty =
                                    <$frame_ty as $crate::lua_bridge::FromMethodSelf<'_>>::from_method_self(state, 1)?;
                                $body
                            };
                            let __val = __result?;
                            $crate::lua_bridge::IntoStack::into_stack(__val, state)
                        }
                        $crate::lua_bridge::table_set_rust_fn($state, $table, $name, __method)?;
                    }
                )*
                Ok(())
            })();
            __result
        }
    };
}

// ---------------------------------------------------------------------------
// define_functions!
// ---------------------------------------------------------------------------

#[doc(hidden)]
#[macro_export]
macro_rules! __lua_bridge_extract_args {
    ($state:expr, $idx:ident,) => {};
    ($state:expr, $idx:ident, $arg_pat:ident : $arg_ty:ty $(, $rest_pat:ident : $rest_ty:ty)* $(,)?) => {
        let $arg_pat: $arg_ty = <$arg_ty as $crate::lua_bridge::FromStack>::from_stack($state, $idx)?;
        $idx += 1;
        $crate::__lua_bridge_extract_args!($state, $idx, $($rest_pat : $rest_ty),*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __lua_bridge_define_function_entries {
    ($state:expr, $table:expr,) => {
        Ok(())
    };
    ($state:expr, $table:expr, $name:literal => || $(-> $ret_ty:ty)? $body:block $(, $($rest:tt)*)?) => {{
        fn __func(state: &mut ::rilua::vm::state::LuaState) -> ::rilua::LuaResult<u32> {
            let __result: ::rilua::LuaResult<_> = $body;
            let __val = __result?;
            $crate::lua_bridge::IntoStack::into_stack(__val, state)
        }
        $crate::lua_bridge::table_set_rust_fn($state, $table, $name, __func)?;
        $crate::__lua_bridge_define_function_entries!($state, $table $(, $($rest)*)?)
    }};
    ($state:expr, $table:expr, $name:literal => |$first_pat:ident : $first_ty:ty $(, $arg_pat:ident : $arg_ty:ty)* $(,)?| $(-> $ret_ty:ty)? $body:block $(, $($rest:tt)*)?) => {{
        fn __func(state: &mut ::rilua::vm::state::LuaState) -> ::rilua::LuaResult<u32> {
            let mut __idx: i32 = 1;
            $crate::__lua_bridge_extract_args!(state, __idx, $first_pat : $first_ty $(, $arg_pat : $arg_ty)*);
            let _ = __idx;

            let __result: ::rilua::LuaResult<_> = $body;
            let __val = __result?;
            $crate::lua_bridge::IntoStack::into_stack(__val, state)
        }
        $crate::lua_bridge::table_set_rust_fn($state, $table, $name, __func)?;
        $crate::__lua_bridge_define_function_entries!($state, $table $(, $($rest)*)?)
    }};
}

/// Register global (non-method) Rust functions on a rilua table.
///
/// Each arm becomes a `RustFn` that extracts arguments starting at index 1
/// and pushes return values.
///
/// TODO: Support closures capturing state once rilua adds `RustFnMut`.
#[macro_export]
macro_rules! define_functions {
    ($state:expr, $table:expr, { $($entries:tt)* }) => {
        {
            let __result: ::rilua::LuaResult<()> = (|| {
                $crate::__lua_bridge_define_function_entries!($state, $table, $($entries)*)
            })();
            __result
        }
    };
}
