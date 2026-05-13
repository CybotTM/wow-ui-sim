//! Coverage for the data + transaction layer the `Blizzard_Professions`
//! crafting page depends on: load the addon chain, drive the same
//! recipe-list / recipe-details / craft pipeline `CraftingPage:Init`
//! eventually reaches, assert reagents drain and the output drops in
//! bag 0.
//!
//! `ProfessionsFrame.CraftingPage` itself comes back as nil under the
//! current panel fixtures because the parent-key wiring for nested
//! `<Frame parentKey="CraftingPage">` doesn't propagate (the existing
//! `professions_frame_loads_and_populates_specialization_tab` test in
//! `test_showuipanel_lod.rs` is failing for the same reason). That's
//! tracked as a separate fixture-level fix; this test exercises the
//! crafting-state slice the panel would query if the parentKey wiring
//! worked.

use crate::common;

use wow_ui_sim::loader::load_addon;

#[test]
fn professions_crafting_data_pipeline_lists_recipes_details_and_crafts_one() {
    test_timeout! {
        let env = common::panel_fixtures::setup_env();
        let ui = common::panel_fixtures::blizzard_ui_dir();
        for (name, toc) in [
            ("Blizzard_FrameXMLUtil", "Blizzard_FrameXMLUtil.toc"),
            ("Blizzard_ProfessionsTemplates", "Blizzard_ProfessionsTemplates.toc"),
            ("Blizzard_SharedTalentUI", "Blizzard_SharedTalentUI.toc"),
            ("Blizzard_Professions", "Blizzard_Professions.toc"),
        ] {
            let toc_path = ui.join(name).join(toc);
            load_addon(&env.loader_env(), &toc_path)
                .unwrap_or_else(|err| panic!("failed to load {name}: {err}"));
        }

        let result: String = env.eval(r#"
            -- Seed: select Blacksmithing, learn the recipe, stock reagents.
            A_Admin.SetSelectedProfession(164)
            A_Admin.LearnRecipe(100001)
            A_Admin.SeedReagentsForRecipe(100001, 1)

            -- Recipe list: GetFilteredRecipeIDs is what the crafting page
            -- iterates to build its data provider.
            local ids = C_TradeSkillUI.GetFilteredRecipeIDs()
            if type(ids) ~= "table" or #ids == 0 then
                return "recipe_list_empty=" .. tostring(ids and #ids or 0)
            end
            local saw_target = false
            for _, id in ipairs(ids) do
                if id == 100001 then saw_target = true break end
            end
            if not saw_target then
                return "recipe_100001_not_in_filtered_ids"
            end

            -- Recipe details: GetRecipeInfo is what SchematicForm.recipeInfo
            -- ultimately resolves to.
            local recipeInfo = C_TradeSkillUI.GetRecipeInfo(100001)
            if not recipeInfo then return "recipe_info_nil" end
            if recipeInfo.recipeID ~= 100001 then
                return "recipe_id=" .. tostring(recipeInfo.recipeID)
            end

            -- Reagent details: GetRecipeReagentInfo is the per-row
            -- reagent payload the SchematicForm reagent slots read.
            local r1 = C_TradeSkillUI.GetRecipeReagentInfo(100001, 1)
            local r2 = C_TradeSkillUI.GetRecipeReagentInfo(100001, 2)
            if not r1 or r1.itemID ~= 210934 or r1.numRequired ~= 12 then
                return "reagent1=" .. tostring(r1 and r1.itemID) .. "/" .. tostring(r1 and r1.numRequired)
            end
            if not r2 or r2.itemID ~= 210937 or r2.numRequired ~= 2 then
                return "reagent2=" .. tostring(r2 and r2.itemID) .. "/" .. tostring(r2 and r2.numRequired)
            end
            if C_TradeSkillUI.GetRecipeNumReagents(100001) ~= 2 then
                return "reagent_count=" .. tostring(C_TradeSkillUI.GetRecipeNumReagents(100001))
            end

            -- Craftability + transaction.
            if not C_TradeSkillUI.IsRecipeCraftable(100001) then
                return "not_craftable_post_seed"
            end

            local function count_item(item_id)
                local total = 0
                for slot = 1, C_Container.GetContainerNumSlots(0) do
                    local info = C_Container.GetContainerItemInfo(0, slot)
                    if info and info.itemID == item_id then
                        total = total + (info.stackCount or 0)
                    end
                end
                return total
            end

            local before_r1 = count_item(210934)
            local before_r2 = count_item(210937)
            local before_out = count_item(211993)

            local ok = C_TradeSkillUI.CraftRecipe(100001, 1)
            if ok ~= true then return "craft=" .. tostring(ok) end

            if (before_r1 - count_item(210934)) ~= 12 then
                return "r1_delta=" .. tostring(before_r1 - count_item(210934))
            end
            if (before_r2 - count_item(210937)) ~= 2 then
                return "r2_delta=" .. tostring(before_r2 - count_item(210937))
            end
            if (count_item(211993) - before_out) ~= 1 then
                return "out_delta=" .. tostring(count_item(211993) - before_out)
            end
            if C_TradeSkillUI.IsRecipeCraftable(100001) then
                return "still_craftable_after_drain"
            end

            return "ok"
        "#).unwrap();

        assert_eq!(
            result,
            "ok",
            "Crafting data pipeline should list recipes, expose details, and complete a craft: {result}"
        );
    }
}
