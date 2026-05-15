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

    #[test]
    fn post_load_clears_main_backpack_slot_normal_textures_after_update() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            ContainerFrame1 = CreateFrame("Frame", "ContainerFrame1", UIParent)
            ContainerFrame1:SetID(0)
            ContainerFrame1.size = 2
            ContainerFrame1Item1 = CreateFrame("Button", "ContainerFrame1Item1", ContainerFrame1)
            ContainerFrame1Item1:SetNormalTexture("Interface\\Buttons\\UI-Quickslot2")
            ContainerFrame1Item2 = CreateFrame("Button", "ContainerFrame1Item2", ContainerFrame1)
            ContainerFrame1Item2:SetNormalTexture("Interface\\Buttons\\UI-Quickslot2")

            ContainerFrame2 = CreateFrame("Frame", "ContainerFrame2", UIParent)
            ContainerFrame2:SetID(1)
            ContainerFrame2.size = 1
            ContainerFrame2Item1 = CreateFrame("Button", "ContainerFrame2Item1", ContainerFrame2)
            ContainerFrame2Item1:SetNormalTexture("Interface\\Buttons\\UI-Quickslot2")

            function ContainerFrame_Update(frame)
              rawset(_G, "__mists_test_last_updated_container", frame:GetName())
            end
            "#,
        )
        .expect("container setup should run");

        super::apply(&env);
        env.exec(
            r#"
            ContainerFrame_Update(ContainerFrame1)
            ContainerFrame_Update(ContainerFrame2)
            "#,
        )
        .expect("wrapped container updates should run");

        let (backpack_cleared, other_bag_kept, original_called): (bool, bool, bool) = env
            .eval(
                r#"
                return ContainerFrame1Item1:GetNormalTexture():GetTexture() == nil
                   and ContainerFrame1Item2:GetNormalTexture():GetTexture() == nil,
                   ContainerFrame2Item1:GetNormalTexture():GetTexture() ~= nil,
                   __mists_test_last_updated_container == "ContainerFrame2"
                "#,
            )
            .expect("normal texture probe should run");
        assert!(
            backpack_cleared,
            "main backpack item buttons should rely on the backpack background slot wells"
        );
        assert!(
            other_bag_kept,
            "non-backpack bag item buttons should keep their normal texture"
        );
        assert!(
            original_called,
            "ContainerFrame_Update wrapper should call through"
        );
    }
}
