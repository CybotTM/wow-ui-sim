//! Permanent compatibility workarounds for intentionally unsupported UI scope.
//!
//! These are not WoW API implementations. They are explicit simulator
//! compromises kept out of the generic bootstrap/C API surface.

mod achievement_display;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    achievement_display::apply_bootstrap(lua)
}
