//! Temporary AchievementUI access gate defaults.
//!
//! The global stubs default `CanShowAchievementUI` to false. Blizzard
//! AchievementUI gating expects these probes to allow the panel in the
//! simulator until a real account/achievement visibility model owns them.

const ACHIEVEMENT_UI_ACCESS_DEFAULTS_LUA: &str = r#"
function HasCompletedAnyAchievement()
  return true
end

function CanShowAchievementUI()
  return true
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(ACHIEVEMENT_UI_ACCESS_DEFAULTS_LUA)?;
    Ok(())
}
