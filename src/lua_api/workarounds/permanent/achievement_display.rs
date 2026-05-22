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

const ACHIEVEMENT_DISPLAY_REAPPLY_LUA: &str = r#"
    if type(AchievementDisplayMixin) ~= "table" then
        AchievementDisplayMixin = {}
    end
    AchievementDisplayMixin.SetAchievements = function(self, achievementIds)
        self.achievementIds = achievementIds
    end
"#;

pub(crate) fn reapply_after_blizzard_load(env: &crate::lua_api::WowLuaEnv) {
    // Blizzard_FrameXML/AchievementDisplayFrame.lua reassigns
    // `AchievementDisplayMixin = {}` and re-defines `:SetAchievements`
    // on top of the bootstrap stub. Reinstate the permanent no-render
    // compatibility behavior after all Blizzard files have loaded.
    let _ = env.exec(ACHIEVEMENT_DISPLAY_REAPPLY_LUA);
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    use super::*;

    #[test]
    fn reapply_restores_stub_after_blizzard_redefines_mixin() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            AchievementDisplayMixin = {
                SetAchievements = function()
                    error("Blizzard body needs unsupported criteria rendering")
                end,
            }
            "#,
        )
        .expect("achievement display test surface should install");

        reapply_after_blizzard_load(&env);

        let stored_id: i64 = env
            .eval(
                r#"
                local frame = {}
                AchievementDisplayMixin.SetAchievements(frame, { 12345 })
                return frame.achievementIds[1]
                "#,
            )
            .expect("reapplied achievement display stub should run");

        assert_eq!(stored_id, 12345);
    }
}
