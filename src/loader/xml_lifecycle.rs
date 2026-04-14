//! Lifecycle script firing for XML-created frames (OnLoad, OnShow).

use crate::lua_api::LoaderEnv;

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

pub fn fire_lifecycle_scripts(_env: &LoaderEnv<'_>, _name: &str, _lifecycle: LifecycleScripts) {}
