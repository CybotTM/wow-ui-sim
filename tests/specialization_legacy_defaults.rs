use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn legacy_specialization_globals_are_registered_by_c_specialization_owner() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let result: String = env
        .eval(
            r##"
            local count = GetNumSpecializations()
            if count ~= C_SpecializationInfo.GetNumSpecializationsForClassID(select(3, UnitClass("player"))) then
                return "count"
            end

            local specID, name, description, icon, role, recommended, allowedForBoost =
                GetSpecializationInfoForClassID(2, 2)
            if specID ~= 66 then return "spec_id" end
            if name ~= "Protection" then return "name" end
            if type(description) ~= "string" then return "description" end
            if type(icon) ~= "number" then return "icon" end
            if role ~= "TANK" then return "role" end
            if recommended ~= false then return "recommended" end
            if allowedForBoost ~= true then return "allowed" end
            if select("#", GetSpecializationInfoForClassID(2, 4)) ~= 0 then
                return "past_end"
            end

            return "ok"
            "##,
        )
        .unwrap();
    assert_eq!(result, "ok");
}
