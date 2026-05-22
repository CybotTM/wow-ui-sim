//! Temporary talent edge frame-level sync workaround.
//!
//! Talent edge lines are generated from button frame levels. The simulator does
//! not yet model the full dirty-edge lifecycle, so mark affected edges dirty
//! when Blizzard changes a talent button's frame level.

use crate::lua_api::{LoaderEnv, WowLuaEnv};

const TALENT_EDGE_FRAME_LEVEL_SYNC_WORKAROUND_LUA: &str = r#"
    if rawget(_G, "__wow_talent_edge_frame_level_sync_wrapped") then
        return
    end

    if type(TalentFrameBaseMixin) ~= "table"
        or type(TalentFrameBaseMixin.UpdateButtonFrameLevel) ~= "function"
        or type(TalentFrameBaseMixin.MarkEdgesDirty) ~= "function" then
        return
    end

    local originalUpdateButtonFrameLevel = TalentFrameBaseMixin.UpdateButtonFrameLevel

    TalentFrameBaseMixin.UpdateButtonFrameLevel = function(self, talentButton, ...)
        local oldLevel = (talentButton and talentButton.GetFrameLevel) and talentButton:GetFrameLevel() or nil
        local result = originalUpdateButtonFrameLevel(self, talentButton, ...)
        if not talentButton or type(self) ~= "table" or type(self.MarkEdgesDirty) ~= "function" then
            return result
        end
        local newLevel = talentButton.GetFrameLevel and talentButton:GetFrameLevel() or nil
        if oldLevel ~= nil and newLevel ~= nil and oldLevel ~= newLevel then
            self:MarkEdgesDirty(talentButton)
        end
        return result
    end

    rawset(_G, "__wow_talent_edge_frame_level_sync_wrapped", true)
"#;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(TALENT_EDGE_FRAME_LEVEL_SYNC_WORKAROUND_LUA);
}

pub(crate) fn patch_loader(env: &LoaderEnv<'_>) {
    let _ = env.exec(TALENT_EDGE_FRAME_LEVEL_SYNC_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_edges_dirty_when_button_frame_level_changes() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            TalentFrameBaseMixin = {
                UpdateButtonFrameLevel = function(self, talentButton)
                    talentButton.level = talentButton.level + 3
                    return "updated"
                end,
                MarkEdgesDirty = function(self, talentButton)
                    self.dirtyButton = talentButton
                    self.dirtyCount = (self.dirtyCount or 0) + 1
                end,
            }
            button = {
                level = 4,
                GetFrameLevel = function(self)
                    return self.level
                end,
            }
            "#,
        )
        .expect("talent edge test surface should install");

        patch(&env);

        let (wrapped, result, dirty_count, dirty_is_button): (bool, String, i64, bool) = env
            .eval(
                r#"
                local frame = {}
                setmetatable(frame, { __index = TalentFrameBaseMixin })
                local result = frame:UpdateButtonFrameLevel(button)
                return __wow_talent_edge_frame_level_sync_wrapped == true,
                    result,
                    frame.dirtyCount,
                    frame.dirtyButton == button
                "#,
            )
            .expect("patched talent edge state should be readable");

        assert!(wrapped);
        assert_eq!(result, "updated");
        assert_eq!(dirty_count, 1);
        assert!(dirty_is_button);
    }

    #[test]
    fn leaves_edges_clean_when_frame_level_is_unchanged() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            TalentFrameBaseMixin = {
                UpdateButtonFrameLevel = function()
                    return "unchanged"
                end,
                MarkEdgesDirty = function(self)
                    self.dirtyCount = (self.dirtyCount or 0) + 1
                end,
            }
            button = {
                level = 9,
                GetFrameLevel = function(self)
                    return self.level
                end,
            }
            "#,
        )
        .expect("unchanged talent edge test surface should install");

        patch(&env);

        let dirty_count: i64 = env
            .eval(
                r#"
                local frame = {}
                setmetatable(frame, { __index = TalentFrameBaseMixin })
                frame:UpdateButtonFrameLevel(button)
                return frame.dirtyCount or 0
                "#,
            )
            .expect("unchanged talent edge state should be readable");

        assert_eq!(dirty_count, 0);
    }
}
