//! Variable-length Lua argument and return container.

use rilua::LuaResult;
use rilua::Val;
use rilua::vm::state::LuaState;

use crate::lua_bridge::FromStack;
use crate::lua_bridge::IntoStack;
use crate::lua_bridge::from_stack::abs_index;

/// Bridge equivalent of mlua's `MultiValue`.
#[derive(Debug, Clone, Default)]
pub struct MultiValue {
    values: Vec<Val>,
}

impl MultiValue {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn from_vec(values: Vec<Val>) -> Self {
        Self { values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn front(&self) -> Option<&Val> {
        self.values.first()
    }

    pub fn get(&self, index: usize) -> Option<&Val> {
        self.values.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn as_slice(&self) -> &[Val] {
        &self.values
    }

    pub fn push(&mut self, value: Val) {
        self.values.push(value);
    }

    pub fn into_vec(self) -> Vec<Val> {
        self.values
    }
}

impl From<Vec<Val>> for MultiValue {
    fn from(values: Vec<Val>) -> Self {
        Self::from_vec(values)
    }
}

impl FromStack for MultiValue {
    fn from_stack(state: &LuaState, index: i32) -> LuaResult<Self> {
        let start = abs_index(state, index);
        if start == usize::MAX || start >= state.top {
            return Ok(Self::new());
        }

        let mut values = Vec::with_capacity(state.top - start);
        for slot in start..state.top {
            values.push(state.stack[slot]);
        }
        Ok(Self::from_vec(values))
    }
}

impl IntoStack for MultiValue {
    fn into_stack(self, state: &mut LuaState) -> LuaResult<u32> {
        let count = self.values.len() as u32;
        for value in self.values {
            state.push(value);
        }
        Ok(count)
    }
}
