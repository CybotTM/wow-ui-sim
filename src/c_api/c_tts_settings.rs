//! Per-environment C_TTSSettings preferences, not public preference CVars.
//! Contract/evidence and unresolved native policies: docs/specs/c-factory-cf2-tts.md.

pub(crate) fn register(lua: &mut rilua::Lua) -> crate::Result<()> {
    super::ensure_namespace(lua.state_mut(), "C_TTSSettings")?;
    lua.exec(include_str!("c_tts_settings.lua"))?;
    Ok(())
}
