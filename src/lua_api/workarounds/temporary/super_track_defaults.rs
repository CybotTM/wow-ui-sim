//! Temporary unsupported priority query; retire when native arbitration is modeled.
//! Target storage and mutations belong exclusively to c_api::c_super_track.

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(
        r#"
        C_SuperTrack = C_SuperTrack or __wow_namespace()
        function C_SuperTrack.GetHighestPrioritySuperTrackingType() return nil end
    "#,
    )?;
    Ok(())
}
