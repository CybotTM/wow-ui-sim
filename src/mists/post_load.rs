//! Mists post-load Lua workarounds — patches that wrap functions defined
//! by FrameXML / Blizzard_* addons.

const MISTS_POST_LOAD_LUA: &str = include_str!("post_load.lua");

pub fn apply(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(MISTS_POST_LOAD_LUA);
}

pub fn apply_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if matches!(
        addon_name,
        "Blizzard_CharacterFrame" | "Blizzard_Collections"
    ) {
        let _ = env.exec(MISTS_POST_LOAD_LUA);
    }
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn post_load_hides_inactive_pvp_ready_dialog() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            PVPReadyDialog = CreateFrame("Frame", "PVPReadyDialog", UIParent)
            PVPReadyDialog:Show()
            "#,
        )
        .expect("dialog setup should run");

        super::apply(&env);

        let shown: bool = env
            .eval("return PVPReadyDialog:IsShown()")
            .expect("visibility probe should run");
        assert!(!shown, "inactive PVPReadyDialog should not show at startup");
    }

    #[test]
    fn post_load_initializes_absent_pet_character_tab() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            CharacterFrame = CreateFrame("Frame", "CharacterFrame", UIParent)
            CharacterFrameTab2 = CreateFrame("Button", "CharacterFrameTab2", CharacterFrame)
            CharacterFrameTab2:Show()
            CharacterFrameTab3 = CreateFrame("Button", "CharacterFrameTab3", CharacterFrame)
            PetPaperDollFrame = CreateFrame("Frame", "PetPaperDollFrame", CharacterFrame)
            PetPaperDollFrame.hidden = nil

            function HasPetUI()
                return false, false
            end

            function PetPaperDollFrame_UpdateIsAvailable()
                if not HasPetUI() then
                    PetPaperDollFrame.hidden = true
                    CharacterFrameTab2:Hide()
                    CharacterFrameTab3:SetPoint("LEFT", "CharacterFrameTab2", "LEFT", 0, 0)
                else
                    PetPaperDollFrame.hidden = false
                    CharacterFrameTab2:Show()
                    CharacterFrameTab3:SetPoint("LEFT", "CharacterFrameTab2", "RIGHT", -16, 0)
                end
            end
            "#,
        )
        .expect("character pet tab setup should run");

        super::apply(&env);

        let (hidden, tab_shown): (bool, bool) = env
            .eval("return PetPaperDollFrame.hidden == true, CharacterFrameTab2:IsShown()")
            .expect("pet availability probe should run");
        assert!(hidden, "absent pet UI should mark PetPaperDollFrame hidden");
        assert!(!tab_shown, "absent pet UI should hide CharacterFrameTab2");
    }
}
