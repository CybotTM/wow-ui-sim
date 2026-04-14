//! Lifecycle script firing for XML-created frames (OnLoad, OnShow).

use mlua::Value;

use crate::lua_api::LoaderEnv;
use crate::lua_api::frame::frame_ref;

use super::precompiled;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct LifecycleScripts {
    pub(super) on_load: bool,
    pub(super) on_show: bool,
}

impl LifecycleScripts {
    pub(super) const fn any(self) -> bool {
        self.on_load || self.on_show
    }
}

/// Fire OnLoad and OnShow lifecycle scripts after the frame is fully configured.
pub fn fire_lifecycle_scripts(
    env: &LoaderEnv<'_>,
    frame_id: u64,
    display_name: &str,
    lifecycle: LifecycleScripts,
) {
    let Some(frame) = resolve_lifecycle_frame(env, frame_id) else {
        return;
    };
    let fns = precompiled::get(env.lua());
    if lifecycle.on_load
        && let Err(e) = fns.fire_onload.call::<()>(frame.clone())
    {
        eprintln!("[OnLoad] {} error: {}", display_name, e);
    }
    if lifecycle.on_show
        && let Err(e) = fns.fire_onshow.call::<()>(frame)
    {
        eprintln!("[OnShow] {} error: {}", display_name, e);
    }
}

fn resolve_lifecycle_frame(env: &LoaderEnv<'_>, frame_id: u64) -> Option<Value> {
    frame_ref(env.lua(), frame_id).ok()
}
