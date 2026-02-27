//! Talent frame workarounds.
//!
//! Breaks the infinite OnUpdate loop in TalentFrameBaseMixin caused by
//! `definitionInfoCache` never being cleared after the initial LoadTalentTree.

use super::WowLuaEnv;

/// Break the infinite OnUpdate loop in TalentFrameBaseMixin.
///
/// After LoadTalentTree, `definitionInfoCache` contains entries for all
/// ~134 talent buttons. The OnUpdate else-branch checks
/// `self.definitionInfoCache[button:GetDefinitionID()]` for each button —
/// if the cache has an entry, the button gets `UpdateEntryContentInfo()`.
/// Since the cache is never cleared by the dirty-set loop (only dirty IDs
/// clear cache entries, and nothing is actually dirty), ALL buttons update
/// every tick, triggering `MarkEdgesDirty` → `RegisterOnUpdate` → infinite
/// loop. In WoW this costs ~1ms (C++ methods); in the sim it costs ~250ms
/// per tick due to metamethod overhead on 109K+ `__index` calls.
///
/// Fix: wrap OnUpdate to clear `definitionInfoCache` after processing,
/// so subsequent ticks only update actually-dirty buttons.
pub fn patch_talent_frame_update_loop(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        -- Blizzard_PlayerSpells is demand-loaded, so TalentFrameBaseMixin
        -- doesn't exist at startup. Register an ADDON_LOADED handler that
        -- patches the mixin when the addon finally loads.
        local f = CreateFrame("Frame")
        f:RegisterEvent("ADDON_LOADED")
        f:SetScript("OnEvent", function(self, event, addon)
            if addon ~= "Blizzard_PlayerSpells" then return end
            self:UnregisterEvent("ADDON_LOADED")
            if not TalentFrameBaseMixin then return end
            local origOnUpdate = TalentFrameBaseMixin.OnUpdate
            TalentFrameBaseMixin.OnUpdate = function(frame)
                origOnUpdate(frame)
                if frame.definitionInfoCache then
                    frame.definitionInfoCache = {}
                end
            end
        end)
    "#,
    );
}
