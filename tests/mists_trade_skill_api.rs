#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

#[test]
fn legacy_trade_skill_globals_are_backed_by_recipe_data() {
    let env = env();
    let result: String = env
        .eval(
            r##"
            if GetNumTradeSkills() ~= 34 then
                return "numTradeSkills=" .. tostring(GetNumTradeSkills())
            end
            if GetFirstTradeSkill() ~= 1 then
                return "first=" .. tostring(GetFirstTradeSkill())
            end

            SelectTradeSkill(1)
            if GetTradeSkillSelectionIndex() ~= 1 then
                return "selection=" .. tostring(GetTradeSkillSelectionIndex())
            end

            local lineName, rank, maxRank = GetTradeSkillLine()
            if lineName ~= "Blacksmithing" or rank ~= 80 or maxRank ~= 100 then
                return "line=" .. tostring(lineName) .. "," .. tostring(rank) .. "," .. tostring(maxRank)
            end

            local name, skillType, numAvailable = GetTradeSkillInfo(1)
            if name ~= "Rough Sharpening Stone" then return "name=" .. tostring(name) end
            if skillType ~= "trivial" then return "skillType=" .. tostring(skillType) end
            if numAvailable ~= 1 then return "numAvailable=" .. tostring(numAvailable) end

            if GetTradeSkillNumReagents(1) ~= 1 then
                return "reagents=" .. tostring(GetTradeSkillNumReagents(1))
            end
            local reagentName, reagentTexture, reagentCount, playerReagentCount = GetTradeSkillReagentInfo(1, 1)
            if not reagentName then return "reagentName=nil" end
            if reagentTexture == nil then return "reagentTexture=nil" end
            if reagentCount ~= 1 or playerReagentCount ~= 1 then
                return "reagentCount=" .. tostring(reagentCount) .. "/" .. tostring(playerReagentCount)
            end

            local recipeLink = GetTradeSkillRecipeLink(1)
            if not recipeLink or not recipeLink:find("Henchant:2660") then
                return "recipeLink=" .. tostring(recipeLink)
            end
            if select("#", GetTradeSkillTools(1)) ~= 0 then
                return "tools=" .. tostring((GetTradeSkillTools(1)))
            end

            return "ok"
            "##,
        )
        .unwrap();
    assert_eq!(result, "ok");
}
