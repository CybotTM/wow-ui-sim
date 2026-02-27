//! Talent frame workarounds.
//!
//! Breaks the infinite OnUpdate loop in TalentFrameBaseMixin caused by
//! `definitionInfoCache` never being cleared after the initial LoadTalentTree.

use super::WowLuaEnv;

/// Lua code that patches a TalentFrameBaseMixin instance's OnUpdate.
///
/// Mixin() copies OnUpdate into the per-frame env[1] table, so patching
/// the mixin table has no effect — we must patch the instance directly.
const PATCH_INSTANCE_LUA: &str = r#"
    local tf = PlayerSpellsFrame and PlayerSpellsFrame.TalentsFrame
    if not tf then return end
    local origOnUpdate = tf.OnUpdate
    if not origOnUpdate then return end
    local wrapped = function(frame)
        origOnUpdate(frame)
        if frame.definitionInfoCache then
            frame.definitionInfoCache = {}
        end
    end
    -- Write to env[1] (mixin table) so it takes priority over env
    local env = debug.getfenv(tf)
    if env and env[1] then rawset(env[1], "OnUpdate", wrapped) end
"#;

/// Register an ADDON_LOADED listener that patches the talent frame
/// instance when Blizzard_PlayerSpells is demand-loaded.
pub fn patch_talent_frame_update_loop(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local f = CreateFrame("Frame")
        f:RegisterEvent("ADDON_LOADED")
        f:SetScript("OnEvent", function(self, event, addon)
            if addon ~= "Blizzard_PlayerSpells" then return end
            self:UnregisterEvent("ADDON_LOADED")
            __patchTalentInstance()
        end)
    "#,
    );
}

/// Register the `__patchTalentInstance` global called by ADDON_LOADED.
pub fn register_patch_function(env: &WowLuaEnv) {
    let _ = env.exec(&format!(
        "function __patchTalentInstance() {} end",
        PATCH_INSTANCE_LUA
    ));
}
