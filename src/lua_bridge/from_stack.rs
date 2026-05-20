//! [`FromStack`] trait and implementations for extracting typed values
//! from the rilua call stack.

use std::marker::PhantomData;

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

/// Extract the `self` receiver for a frame-backed method call.
pub trait FromMethodSelf<'a>: Sized {
    fn from_method_self(state: &'a mut LuaState, index: i32) -> LuaResult<Self>;
}

/// Associates a frame type with the arena that stores it.
pub trait FrameObject: Sized + 'static {
    type Arena: FrameArena<Frame = Self> + 'static;
}

// ---------------------------------------------------------------------------
// Frame-backed table extraction
// ---------------------------------------------------------------------------

/// Host-side arena API for frame-backed Lua tables.
pub trait FrameArena {
    type Frame;

    fn frame(&self, index: u32, generation: u32) -> Option<&Self::Frame>;
    fn frame_mut(&mut self, index: u32, generation: u32) -> Option<&mut Self::Frame>;
}

/// Typed handle to a frame-backed Lua table stored in host app data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameRef<A: FrameArena + 'static> {
    index: u32,
    generation: u32,
    _marker: PhantomData<fn() -> A>,
}

impl<A: FrameArena + 'static> FrameRef<A> {
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn get<'a>(&self, state: &'a LuaState) -> LuaResult<&'a A::Frame> {
        let arena = state
            .app_data::<A>()
            .ok_or_else(|| runtime_error("missing frame arena app_data for frame-backed table"))?;
        arena.frame(self.index, self.generation).ok_or_else(|| {
            runtime_error(format!(
                "missing frame for backing ({}, {})",
                self.index, self.generation
            ))
        })
    }

    pub fn get_mut<'a>(&self, state: &'a mut LuaState) -> LuaResult<&'a mut A::Frame> {
        let arena = state
            .app_data_mut::<A>()
            .ok_or_else(|| runtime_error("missing frame arena app_data for frame-backed table"))?;
        arena.frame_mut(self.index, self.generation).ok_or_else(|| {
            runtime_error(format!(
                "missing frame for backing ({}, {})",
                self.index, self.generation
            ))
        })
    }
}

impl<A: FrameArena + 'static> FromStack for FrameRef<A> {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        let val = stack_val(state, index);
        let Val::Table(table_ref) = val else {
            return Err(type_error("frame-backed table", val.type_name(), index));
        };

        let backing = state
            .gc
            .tables
            .get(table_ref)
            .and_then(|table| table.backing())
            .ok_or_else(|| type_error("frame-backed table", "table", index))?;

        let frame_ref = Self {
            index: backing.0,
            generation: backing.1,
            _marker: PhantomData,
        };
        let _ = frame_ref.get(state)?;
        Ok(frame_ref)
    }
}

impl<'a, T: FrameObject> FromMethodSelf<'a> for &'a T {
    fn from_method_self(state: &'a mut LuaState, index: i32) -> LuaResult<Self> {
        let frame_ref = FrameRef::<T::Arena>::from_stack(state, index)?;
        frame_ref.get(state)
    }
}

impl<'a, T: FrameObject> FromMethodSelf<'a> for &'a mut T {
    fn from_method_self(state: &'a mut LuaState, index: i32) -> LuaResult<Self> {
        let frame_ref = FrameRef::<T::Arena>::from_stack(state, index)?;
        frame_ref.get_mut(state)
    }
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

impl FromStack for f32 {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        Ok(f64::from_stack(state, index)? as f32)
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
                        runtime_error(format!("string at argument {index} has been collected"))
                    })?;
                std::str::from_utf8(bytes).map(str::to_owned).map_err(|_| {
                    runtime_error(format!("string at argument {index} is not valid UTF-8"))
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

impl<A: FromStack, B: FromStack> FromStack for (A, B) {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        Ok((
            A::from_stack(state, index)?,
            B::from_stack(state, index + 1)?,
        ))
    }
}

impl<A: FromStack, B: FromStack, C: FromStack> FromStack for (A, B, C) {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        Ok((
            A::from_stack(state, index)?,
            B::from_stack(state, index + 1)?,
            C::from_stack(state, index + 2)?,
        ))
    }
}

impl<A: FromStack, B: FromStack, C: FromStack, D: FromStack> FromStack for (A, B, C, D) {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        Ok((
            A::from_stack(state, index)?,
            B::from_stack(state, index + 1)?,
            C::from_stack(state, index + 2)?,
            D::from_stack(state, index + 3)?,
        ))
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

#[cfg(test)]
mod tests {
    use super::FromStack;
    use rilua::LuaApiMut;
    use rilua::Val;

    #[test]
    fn string_from_stack_validates_before_allocating_result() {
        let mut lua = rilua::Lua::new().expect("lua should initialize");
        let state = lua.state_mut();
        let key = state.gc.intern_string(b"hello");
        state.ensure_stack(state.base + 1);
        state.stack_set(state.base, Val::Str(key));
        state.top = state.base + 1;

        let value = String::from_stack(state, 1).expect("valid UTF-8 string should convert");

        assert_eq!(value, "hello");
    }

    #[test]
    fn string_from_stack_rejects_invalid_utf8_without_copying_first() {
        let mut lua = rilua::Lua::new().expect("lua should initialize");
        let state = lua.state_mut();
        let key = state.gc.intern_string(&[0xff]);
        state.ensure_stack(state.base + 1);
        state.stack_set(state.base, Val::Str(key));
        state.top = state.base + 1;

        let error = String::from_stack(state, 1).expect_err("invalid UTF-8 should fail");

        assert!(
            error.to_string().contains("not valid UTF-8"),
            "unexpected error: {error}"
        );
    }
}
