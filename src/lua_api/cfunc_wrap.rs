//! Create unique C-function wrappers without consuming mlua auxiliary stack slots.
//!
//! Uses raw `lua_pushcclosure` to create C closures on the Lua heap, bypassing
//! mlua's auxiliary thread stack (limited to ~8000 entries by LUAI_MAXCSTACK).

use std::ffi::c_int;

/// Raw C closure: forwards all args to upvalue 1 (the wrapped function).
unsafe extern "C-unwind" fn forward_call(s: *mut mlua::ffi::lua_State) -> c_int {
    use mlua::ffi::*;
    let n = unsafe { lua_gettop(s) };
    unsafe { lua_pushvalue(s, lua_upvalueindex(1)) };
    unsafe { lua_insert(s, 1) };
    unsafe { lua_call(s, n, LUA_MULTRET) };
    unsafe { lua_gettop(s) }
}

/// Raw C function: takes 1 function arg, returns a new C closure wrapping it.
unsafe extern "C-unwind" fn wrap_one(s: *mut mlua::ffi::lua_State) -> c_int {
    unsafe { mlua::ffi::lua_pushvalue(s, 1) };
    unsafe { mlua::ffi::lua_pushcclosure(s, forward_call, 1) };
    1
}

/// Create a factory function that wraps any function into a unique C closure.
/// Uses 1 auxiliary slot for the factory itself; each call uses 0 permanent slots.
pub fn create_wrap_factory(lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
    unsafe { lua.create_c_function(wrap_one) }
}
