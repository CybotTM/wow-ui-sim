//! Mists post-load Lua workarounds — patches that wrap functions defined
//! by FrameXML / Blizzard_* addons.

const MISTS_POST_LOAD_LUA: &str = include_str!("post_load.lua");

pub fn apply(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(MISTS_POST_LOAD_LUA);
}

pub fn apply_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if matches!(
        addon_name,
        "Blizzard_CharacterFrame" | "Blizzard_Collections" | "Blizzard_TalentUI"
    ) {
        let _ = env.exec(MISTS_POST_LOAD_LUA);
    }
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn post_load_hides_auction_house_until_opened() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            AuctionHouseFrame = CreateFrame("Frame", "AuctionHouseFrame", UIParent)
            AuctionHouseFrame:Show()
            "#,
        )
        .expect("auction house setup should run");

        super::apply(&env);

        let shown: bool = env
            .eval("return AuctionHouseFrame:IsShown()")
            .expect("auction house visibility probe should run");
        assert!(!shown, "AuctionHouseFrame should not start open");
    }

    #[test]
    fn post_load_remembers_settings_categories_by_id_for_addons() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            Settings = {}
            local nextID = 0
            local Category = {}
            Category.__index = Category
            function Category:GetID() return self.ID end
            function Category:GetName() return self.name end
            function Category:CreateSubcategory(name)
              nextID = nextID + 1
              local subcategory = setmetatable({ ID = nextID, name = name, parent = self }, Category)
              return subcategory
            end
            function Settings.RegisterCanvasLayoutCategory(frame, name)
              nextID = nextID + 1
              return setmetatable({ ID = nextID, name = name }, Category), {}
            end
            function Settings.RegisterCanvasLayoutSubcategory(parentCategory, frame, name)
              return parentCategory:CreateSubcategory(name), {}
            end
            function Settings.RegisterAddOnCategory(category) end
            function Settings.GetCategory(categoryID) return nil end
            "#,
        )
        .expect("settings setup should run");

        super::apply(&env);

        let ok: bool = env
            .eval(
                r#"
                local category = Settings.RegisterCanvasLayoutCategory(CreateFrame("Frame"), "Probe")
                local id = category:GetID()
                Settings.RegisterAddOnCategory(category)
                local found = Settings.GetCategory(id)
                local subcategory = Settings.RegisterCanvasLayoutSubcategory(found, CreateFrame("Frame"), "Sub")
                return found == category and subcategory.parent == category
                "#,
            )
            .expect("settings category lookup probe should run");
        assert!(ok, "registered addon categories should resolve by ID");
    }

    #[test]
    fn settings_category_lookup_survives_runtime_surface_restore() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            Settings = {}
            local nextID = 0
            local Category = {}
            Category.__index = Category
            function Category:GetID() return self.ID end
            function Category:GetName() return self.name end
            function Settings.RegisterCanvasLayoutCategory(frame, name)
              nextID = nextID + 1
              return setmetatable({ ID = nextID, name = name }, Category), {}
            end
            function Settings.RegisterAddOnCategory(category) end
            function Settings.GetCategory(categoryID) return nil end
            "#,
        )
        .expect("settings setup should run");

        super::apply(&env);
        env.restore_post_cleanup_globals();

        let ok: bool = env
            .eval(
                r#"
                local category = Settings.RegisterCanvasLayoutCategory(CreateFrame("Frame"), "Probe")
                local id = category:GetID()
                Settings.RegisterAddOnCategory(category)
                return Settings.GetCategory(id) == category
                "#,
            )
            .expect("settings category lookup probe should run");
        assert!(
            ok,
            "runtime surface restore should not replace the Mists Settings.GetCategory wrapper"
        );
    }

    #[test]
    fn post_load_disables_blizzmove_startup_frame_scan() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            BlizzMove = {}
            function BlizzMove:ProcessFrames(addOnName)
              rawset(_G, "__blizzmove_processed", addOnName)
            end
            "#,
        )
        .expect("blizzmove setup should run");

        super::apply(&env);
        env.exec("BlizzMove:ProcessFrames('Blizzard_UIParent')")
            .expect("wrapped ProcessFrames should run");

        let processed: Option<String> = env
            .eval("return rawget(_G, '__blizzmove_processed')")
            .expect("blizzmove probe should run");
        assert_eq!(
            processed, None,
            "Mists startup should not let BlizzMove recursively scan simulator frame geometry"
        );
    }

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
    fn post_load_hides_duplicate_character_frame_title() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            CharacterFrame = CreateFrame("Frame", "CharacterFrame", UIParent)
            CharacterFrame.TitleText = CharacterFrame:CreateFontString("CharacterFrameDirectTitle")
            CharacterFrame.TitleText:Show()
            CharacterFrame.TitleContainer = CreateFrame("Frame", "CharacterFrameTitleContainer", CharacterFrame)
            CharacterFrame.TitleContainer.TitleText = CharacterFrame.TitleContainer:CreateFontString("CharacterFrameContainerTitle")
            CharacterFrame.TitleContainer.TitleText:Show()
            "#,
        )
        .expect("character title setup should run");

        super::apply(&env);

        let (direct_shown, container_shown): (bool, bool) = env
            .eval(
                r#"
                return CharacterFrame.TitleText:IsShown(),
                       CharacterFrame.TitleContainer.TitleText:IsShown()
                "#,
            )
            .expect("title visibility probe should run");
        assert!(
            direct_shown,
            "direct CharacterFrame title should remain visible"
        );
        assert!(
            !container_shown,
            "stale TitleContainer title should be hidden to avoid doubled panel titles"
        );
    }

    #[test]
    fn post_load_keeps_main_backpack_slot_normal_textures_after_update() {
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

        let (backpack_kept, other_bag_kept, original_called): (bool, bool, bool) = env
            .eval(
                r#"
                return ContainerFrame1Item1:GetNormalTexture():GetTexture() ~= nil
                   and ContainerFrame1Item2:GetNormalTexture():GetTexture() ~= nil,
                   ContainerFrame2Item1:GetNormalTexture():GetTexture() ~= nil,
                   __mists_test_last_updated_container == "ContainerFrame2"
                "#,
            )
            .expect("normal texture probe should run");
        assert!(
            backpack_kept,
            "main backpack item buttons should keep their authored Mists quickslot normal art"
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
