//! Precompiled Lua helper functions to eliminate repeated source compilation.
//!
//! During the rilua port these helpers are reduced to typed no-ops so callers
//! can keep the same structure while the old mlua-backed function cache is gone.

#[derive(Clone, Copy, Default)]
pub struct NoopFunction;

impl NoopFunction {
    pub fn call<T>(&self, _args: T) -> crate::Result<()> {
        Ok(())
    }
}

pub struct PrecompiledFnsRef {
    pub fire_onload: NoopFunction,
    pub fire_onshow: NoopFunction,
    pub suppress_push: NoopFunction,
    pub suppress_pop: NoopFunction,
    pub assign_parent_key: NoopFunction,
    pub set_intrinsic: NoopFunction,
}

fn stubbed_fns() -> PrecompiledFnsRef {
    PrecompiledFnsRef {
        fire_onload: NoopFunction,
        fire_onshow: NoopFunction,
        suppress_push: NoopFunction,
        suppress_pop: NoopFunction,
        assign_parent_key: NoopFunction,
        set_intrinsic: NoopFunction,
    }
}

pub fn init<T>(_lua: &T) -> crate::Result<()> {
    Ok(())
}

pub fn get<T>(_lua: &T) -> PrecompiledFnsRef {
    stubbed_fns()
}

pub fn try_get<T>(_lua: &T) -> Option<PrecompiledFnsRef> {
    Some(stubbed_fns())
}
