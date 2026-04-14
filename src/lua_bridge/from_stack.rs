//! [`FromStack`] trait and implementations for extracting typed values
//! from the rilua call stack.

use rilua::LuaError;
use rilua::LuaResult;
use rilua::RuntimeError;
use rilua::Val;
use rilua::vm::state::LuaState;

// ---------------------------------------------------------------------------
// Stack helpers
// ---------------------------------------------------------------------------

/// Convert a 1-based Lua argument index to the absolute stack slot.
///
/// rilua sets `state.base` to the first register of the current frame
/// (register 0 = first argument). A 1-based index therefore maps to
/// `base + index - 1`.
///
/// Negative indices count from the top:
/// `-1` is `state.top - 1`, etc.
#[inline]
pub(crate) fn abs_index(state: &LuaState, index: i32) -> usize {
    if index > 0 {
        state.base + (index as usize) - 1
    } else if index < 0 {
        (state.top as isize + index as isize) as usize
    } else {
        // 0 is not a valid Lua stack index; treat as out-of-bounds.
        usize::MAX
    }
}

/// Return the `Val` at a (1-based or negative) stack position.
///
/// Returns `Val::Nil` for out-of-bounds positions.
#[inline]
pub(crate) fn stack_val(state: &LuaState, index: i32) -> Val {
    let abs = abs_index(state, index);
    if abs < state.stack.len() && abs < state.top {
        state.stack[abs]
    } else {
        Val::Nil
    }
}

// ---------------------------------------------------------------------------
// FromStack trait
// ---------------------------------------------------------------------------

/// Extract a typed value from a rilua call frame at a given argument position.
///
/// Positions are **1-based** (Lua convention): 1 = first argument, 2 = second, etc.
/// Negative positions count from the top of the stack (-1 = top).
///
/// Implementations produce descriptive errors that include the position and
/// the actual type name, matching the style of PUC-Rio's `luaL_checktype`.
pub trait FromStack: Sized {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self>;
}

// ---------------------------------------------------------------------------
// Passthrough: Val
// ---------------------------------------------------------------------------

impl FromStack for Val {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        Ok(stack_val(state, index))
    }
}

// ---------------------------------------------------------------------------
// ()
// ---------------------------------------------------------------------------

impl FromStack for () {
    /// Always succeeds; extracts nothing.
    fn from_stack(_state: &LuaState, _index: i32) -> LuaResult<Self> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// bool
// ---------------------------------------------------------------------------

impl FromStack for bool {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        match stack_val(state, index) {
            Val::Bool(b) => Ok(b),
            Val::Nil => Ok(false),
            _ => Ok(true),
        }
    }
}

// ---------------------------------------------------------------------------
// f64
// ---------------------------------------------------------------------------

impl FromStack for f64 {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        match stack_val(state, index) {
            Val::Num(n) => Ok(n),
            got => Err(type_error("number", got.type_name(), index)),
        }
    }
}

// ---------------------------------------------------------------------------
// Integer types
// ---------------------------------------------------------------------------

/// Shared helper: extract a Lua number and verify it is an exact integer.
fn num_to_int(val: Val, index: i32) -> LuaResult<i64> {
    match val {
        Val::Num(n) => {
            let i = n as i64;
            if i as f64 == n {
                Ok(i)
            } else {
                Err(runtime_error(format!(
                    "expected integer, got non-integer number at argument {index}"
                )))
            }
        }
        got => Err(type_error("number", got.type_name(), index)),
    }
}

impl FromStack for i64 {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        num_to_int(stack_val(state, index), index)
    }
}

impl FromStack for i32 {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        let n = num_to_int(stack_val(state, index), index)?;
        i32::try_from(n).map_err(|_| {
            runtime_error(format!(
                "expected i32, value {n} out of range at argument {index}"
            ))
        })
    }
}

impl FromStack for u32 {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        let n = num_to_int(stack_val(state, index), index)?;
        u32::try_from(n).map_err(|_| {
            runtime_error(format!(
                "expected u32, value {n} out of range at argument {index}"
            ))
        })
    }
}

// ---------------------------------------------------------------------------
// String
// ---------------------------------------------------------------------------

impl FromStack for String {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        match stack_val(state, index) {
            Val::Str(str_ref) => {
                let bytes = state
                    .gc
                    .string_arena
                    .get(str_ref)
                    .map(|s| s.data())
                    .ok_or_else(|| {
                        runtime_error(format!(
                            "string at argument {index} has been collected"
                        ))
                    })?;
                String::from_utf8(bytes.to_vec()).map_err(|_| {
                    runtime_error(format!(
                        "string at argument {index} is not valid UTF-8"
                    ))
                })
            }
            got => Err(type_error("string", got.type_name(), index)),
        }
    }
}

// ---------------------------------------------------------------------------
// Option<T>
// ---------------------------------------------------------------------------

impl<T: FromStack> FromStack for Option<T> {
    /// Returns `None` for nil or absent stack positions; `Some(T)` otherwise.
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        match stack_val(state, index) {
            Val::Nil => Ok(None),
            _ => Ok(Some(T::from_stack(state, index)?)),
        }
    }
}

// ---------------------------------------------------------------------------
// Error helpers
// ---------------------------------------------------------------------------

fn type_error(expected: &str, got: &str, index: i32) -> LuaError {
    runtime_error(format!(
        "expected {expected}, got {got} at argument {index}"
    ))
}

fn runtime_error(msg: impl Into<String>) -> LuaError {
    LuaError::Runtime(RuntimeError {
        message: msg.into(),
        level: 1,
        traceback: vec![],
    })
}
