#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("failed to create Lua environment")
}

#[test]
fn legacy_trainer_services_expose_selectable_recipe_rows() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if GetNumTrainerServices() < 2 then
                return "missing_services"
            end

            local headerName, _, headerType, isExpanded = GetTrainerServiceInfo(1)
            if headerName ~= "Blacksmithing" or headerType ~= "header" or not isExpanded then
                return "bad_header:" .. tostring(headerName) .. ":" .. tostring(headerType)
            end

            local serviceName, serviceSubText, serviceType = GetTrainerServiceInfo(2)
            if not serviceName or serviceSubText ~= "Blacksmithing" or serviceType ~= "available" then
                return "bad_service:" .. tostring(serviceName) .. ":" .. tostring(serviceSubText) .. ":" .. tostring(serviceType)
            end

            SelectTrainerService(2)
            if GetTrainerSelectionIndex() ~= 2 then
                return "bad_selection:" .. tostring(GetTrainerSelectionIndex())
            end

            if (GetTrainerServiceIcon(2) or 0) <= 0 then
                return "missing_icon"
            end

            local skill, rank, hasReq = GetTrainerServiceSkillReq(2)
            if skill ~= "Blacksmithing" or rank ~= 1 or not hasReq then
                return "bad_skill_req"
            end

            local cost, isProfession = GetTrainerServiceCost(2)
            if cost ~= 0 or isProfession then
                return "bad_cost"
            end

            if type(GetTrainerServiceDescription(2)) ~= "string" then
                return "bad_description"
            end

            if GetTrainerServiceNumAbilityReq(2) ~= 0 then
                return "bad_ability_req_count"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}
