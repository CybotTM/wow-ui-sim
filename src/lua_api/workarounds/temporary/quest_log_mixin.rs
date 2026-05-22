//! Temporary QuestLog startup parent guard.
//!
//! Startup events can hit QuestLog methods before `QuestMapFrame` is parented
//! to `WorldMapFrame`. Guard those methods until startup ordering matches the
//! Blizzard path more closely.

use crate::lua_api::LoaderEnv;
#[cfg(test)]
use crate::lua_api::WowLuaEnv;

const QUEST_LOG_MIXIN_LUA: &str = r#"
local function SafeGetCurrentMapID(self)
    local parent = self:GetParent()
    if parent and parent:IsShown() then
        return parent:GetMapID()
    end
    return C_Map.GetBestMapForUnit("player")
end
-- Patch the mixin for future frames
if QuestLogMixin ~= nil then
    QuestLogMixin.GetCurrentMapID = SafeGetCurrentMapID
end
-- Patch the existing QuestMapFrame instance directly
if QuestMapFrame then
    QuestMapFrame.GetCurrentMapID = SafeGetCurrentMapID
end

if type(QuestMapFrame_UpdateAll) == "function" and not rawget(_G, "__wow_quest_map_update_all_patched") then
    local originalUpdateAll = QuestMapFrame_UpdateAll
    QuestMapFrame_UpdateAll = function(numPOIs)
        local parent = QuestMapFrame and QuestMapFrame:GetParent() or nil
        if parent == nil then
            QuestMapFrame.UpdatePOIs(QuestMapFrame)
            if not numPOIs then
                QuestMapUpdateAllQuests()
            end
            return
        end
        return originalUpdateAll(numPOIs)
    end
    rawset(_G, "__wow_quest_map_update_all_patched", true)
end
"#;

pub(crate) fn patch(env: &LoaderEnv<'_>) -> Result<(), crate::Error> {
    env.exec(QUEST_LOG_MIXIN_LUA)
}

#[cfg(test)]
fn patch_env(env: &WowLuaEnv) {
    env.exec(QUEST_LOG_MIXIN_LUA)
        .expect("QuestLog mixin patch should install");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_current_map_id_uses_parent_when_shown() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_quest_log_fixture(&env, "shown");

        patch_env(&env);

        let map_id: i64 = env
            .eval("return QuestMapFrame:GetCurrentMapID()")
            .expect("QuestMapFrame map id should resolve from parent");

        assert_eq!(map_id, 84);
    }

    #[test]
    fn get_current_map_id_falls_back_without_parent() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_quest_log_fixture(&env, "missing");

        patch_env(&env);

        let map_id: i64 = env
            .eval("return QuestMapFrame:GetCurrentMapID()")
            .expect("QuestMapFrame map id should fall back to player map");

        assert_eq!(map_id, 12);
    }

    #[test]
    fn update_all_updates_pois_when_parent_is_missing() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_quest_log_fixture(&env, "missing");

        patch_env(&env);

        let (poi_updates, all_quest_updates, original_updates): (i64, i64, i64) = env
            .eval(
                r#"
                QuestMapFrame_UpdateAll()
                return QuestMapFrame.poiUpdates, allQuestUpdates, originalUpdates
                "#,
            )
            .expect("QuestMapFrame_UpdateAll should guard missing parent");

        assert_eq!(
            (poi_updates, all_quest_updates, original_updates),
            (1, 1, 0)
        );
    }

    fn install_quest_log_fixture(env: &WowLuaEnv, parent_mode: &str) {
        install_map_fixture(env);
        install_quest_map_frame_fixture(env, parent_mode);
        install_update_all_fixture(env);
    }

    fn install_map_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            QuestLogMixin = {}
            C_Map = {
                GetBestMapForUnit = function()
                    return 12
                end,
            }
            parent = {
                IsShown = function()
                    return true
                end,
                GetMapID = function()
                    return 84
                end,
            }
            "#,
        )
        .expect("QuestLog map fixture should install");
    }

    fn install_quest_map_frame_fixture(env: &WowLuaEnv, parent_mode: &str) {
        let fixture = format!(
            r#"
            local parentMode = "{parent_mode}"
            QuestMapFrame = {{
                poiUpdates = 0,
                GetParent = function()
                    if parentMode == "missing" then
                        return nil
                    end
                    return parent
                end,
                UpdatePOIs = function(self)
                    self.poiUpdates = self.poiUpdates + 1
                end,
            }}
            "#,
        );
        env.exec(&fixture)
            .expect("QuestMapFrame fixture should install");
    }

    fn install_update_all_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            allQuestUpdates = 0
            originalUpdates = 0
            QuestMapUpdateAllQuests = function()
                allQuestUpdates = allQuestUpdates + 1
            end
            QuestMapFrame_UpdateAll = function()
                originalUpdates = originalUpdates + 1
            end
            "#,
        )
        .expect("QuestMapFrame_UpdateAll fixture should install");
    }
}
