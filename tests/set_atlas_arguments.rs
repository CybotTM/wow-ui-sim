use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_set_atlas_reports_application_success_to_texture_kit_consumers() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r##"
        local parent = CreateFrame("Frame", nil, UIParent)
        local texture = parent:CreateTexture()
        local atlas = "worldstate-capturebar-neutralglow-bastionarmor"
        assert(C_Texture.GetAtlasInfo(atlas), "real capture-bar atlas prerequisite")
        local function CaptureReturns(...)
            return select("#", ...), ...
        end
        local count, applied = CaptureReturns(texture:SetAtlas(atlas, true))
        assert(count == 1, "SetAtlas application-success policy requires one return; actual=" .. count)
        assert(applied == true, "known atlas must report applied")
        assert(texture:GetAtlas() == atlas, "known atlas remains applied")
        local missingCount, missingApplied = CaptureReturns(texture:SetAtlas("sim-missing-texture-kit-atlas"))
        assert(missingCount == 1 and missingApplied == false, "missing atlas must report not applied")
        for _, input in ipairs({ "", "   ", 0 }) do
            local inactiveCount, inactiveApplied = CaptureReturns(texture:SetAtlas(input))
            assert(inactiveCount == 1 and inactiveApplied == false, "accepted inactive input must report not applied")
        end
        local nilCount, nilApplied = CaptureReturns(texture:SetAtlas(nil))
        assert(nilCount == 1 and nilApplied == false, "nil atlas must report not applied")
        local restoredCount, restoredApplied = CaptureReturns(texture:SetAtlas(atlas))
        assert(restoredCount == 1 and restoredApplied == true, "known atlas must report applied after inactive inputs")
        assert(texture:GetAtlas() == atlas, "restored atlas remains applied")
        "##,
    )
    .unwrap();
}

#[test]
fn test_set_atlas_argument_validation_matches_retail() {
    let env = WowLuaEnv::new().unwrap();
    let (no_arg_ok, no_arg_error, nil_ok, false_ok, false_error, zero_ok): (
        bool,
        String,
        bool,
        bool,
        String,
        bool,
    ) = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "AtlasArgsParent", UIParent)
            local tex = parent:CreateTexture("AtlasArgsTexture")
            local noArgOk, noArgErr = pcall(function() tex:SetAtlas() end)
            local nilOk = pcall(function() tex:SetAtlas(nil) end)
            local falseOk, falseErr = pcall(function() tex:SetAtlas(false) end)
            local zeroOk = pcall(function() tex:SetAtlas(0) end)
            return noArgOk, tostring(noArgErr), nilOk, falseOk, tostring(falseErr), zeroOk
            "#,
        )
        .unwrap();

    assert!(!no_arg_ok);
    assert!(no_arg_error.contains("Usage:"));
    assert!(nil_ok);
    assert!(!false_ok);
    assert!(false_error.contains("Usage:"));
    assert!(zero_ok);
}
