//! Temporary inert defaults for additive 12.1 API names.
//!
//! Legacy bootstrap for temporary additive 12.1 API defaults.
//!
//! The currently audited safe social/housing defaults have moved to Rust-backed
//! simulator state. Keep this module as an empty version-gated hook so any
//! future temporary 12.1 bridges remain isolated instead of drifting into the
//! all-profile runtime surface.

const PATCH_12_1_INERT_DEFAULTS_LUA: &str = r#"
if type(GetBuildInfo) == "function" and select(4, GetBuildInfo()) >= 120100 then
    -- All currently audited safe 12.1 defaults are Rust-backed.
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(PATCH_12_1_INERT_DEFAULTS_LUA)?;
    Ok(())
}
