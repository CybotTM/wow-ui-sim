//! [`IntoStack`] trait and implementations for pushing typed values
//! onto the rilua call stack.

use rilua::LuaResult;
use rilua::Val;
use rilua::vm::state::LuaState;

// ---------------------------------------------------------------------------
// IntoStack trait
// ---------------------------------------------------------------------------

/// Push a typed Rust value onto the rilua stack.
///
/// Returns the number of `Val`s pushed (0 or more). Pushing multiple values
/// is only meaningful for tuple impls that need to return several Lua values
/// from a single `RustFn`.
///
/// The `state.push()` method handles stack growth; callers do not need to
/// call `ensure_stack` manually.
pub trait IntoStack {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32>;
}

// ---------------------------------------------------------------------------
// Passthrough: Val
// ---------------------------------------------------------------------------

impl IntoStack for Val {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        state.push(self);
        Ok(1)
    }
}

// ---------------------------------------------------------------------------
// ()
// ---------------------------------------------------------------------------

impl IntoStack for () {
    /// Pushes nothing; returns 0.
    fn into_stack(self, _state: &mut LuaState) -> LuaResult<u32> {
        Ok(0)
    }
}

// ---------------------------------------------------------------------------
// bool
// ---------------------------------------------------------------------------

impl IntoStack for bool {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        state.push(Val::Bool(self));
        Ok(1)
    }
}

// ---------------------------------------------------------------------------
// f64
// ---------------------------------------------------------------------------

impl IntoStack for f64 {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        state.push(Val::Num(self));
        Ok(1)
    }
}

// ---------------------------------------------------------------------------
// Integer types
// ---------------------------------------------------------------------------

fn push_number(state: &mut LuaState, value: f64) -> LuaResult<u32> {
    state.push(Val::Num(value));
    Ok(1)
}

macro_rules! impl_into_stack_int {
    ($($ty:ty),*) => {$(
        impl IntoStack for $ty {
            fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
                push_number(state, self as f64)
            }
        }
    )*};
}

impl_into_stack_int!(i32, i64, u32, u64);

// ---------------------------------------------------------------------------
// String / &str
// ---------------------------------------------------------------------------

impl IntoStack for String {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        let str_ref = state.gc.intern_string(self.as_bytes());
        state.push(Val::Str(str_ref));
        Ok(1)
    }
}

impl IntoStack for &str {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        let str_ref = state.gc.intern_string(self.as_bytes());
        state.push(Val::Str(str_ref));
        Ok(1)
    }
}

// ---------------------------------------------------------------------------
// Option<T>
// ---------------------------------------------------------------------------

impl<T: IntoStack> IntoStack for Option<T> {
    /// Pushes nil for `None`; pushes `T` for `Some(T)`.
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        match self {
            None => {
                state.push(Val::Nil);
                Ok(1)
            }
            Some(v) => v.into_stack(state),
        }
    }
}

// ---------------------------------------------------------------------------
// Tuples (up to 4 elements)
// ---------------------------------------------------------------------------

impl<A: IntoStack> IntoStack for (A,) {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        let n = self.0.into_stack(state)?;
        Ok(n)
    }
}

impl<A: IntoStack, B: IntoStack> IntoStack for (A, B) {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        let a = self.0.into_stack(state)?;
        let b = self.1.into_stack(state)?;
        Ok(a + b)
    }
}

impl<A: IntoStack, B: IntoStack, C: IntoStack> IntoStack for (A, B, C) {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        let a = self.0.into_stack(state)?;
        let b = self.1.into_stack(state)?;
        let c = self.2.into_stack(state)?;
        Ok(a + b + c)
    }
}

impl<A: IntoStack, B: IntoStack, C: IntoStack, D: IntoStack> IntoStack for (A, B, C, D) {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        let a = self.0.into_stack(state)?;
        let b = self.1.into_stack(state)?;
        let c = self.2.into_stack(state)?;
        let d = self.3.into_stack(state)?;
        Ok(a + b + c + d)
    }
}
