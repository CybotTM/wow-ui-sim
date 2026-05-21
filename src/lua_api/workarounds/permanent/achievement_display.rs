//! Permanent `AchievementDisplayMixin` workaround.
//!
//! Real WoW renders achievement criteria rows through pooled frames. The
//! simulator intentionally does not model that cosmetic 2D panel behavior; it
//! only preserves the achievement ID list so callers can round-trip it.

const ACHIEVEMENT_DISPLAY_BOOTSTRAP_LUA: &str = r#"
    AchievementDisplayMixin = AchievementDisplayMixin or {}
    if rawget(AchievementDisplayMixin, "SetAchievements") == nil then
        function AchievementDisplayMixin:SetAchievements(achievementIds)
            self.achievementIds = achievementIds
        end
    end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(ACHIEVEMENT_DISPLAY_BOOTSTRAP_LUA)?;
    Ok(())
}
