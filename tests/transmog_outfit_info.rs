use wow_ui_sim::lua_api::WowLuaEnv;

const TRANSMOG_OUTFIT_INFO_SCRIPT: &str = r#"
    if C_TransmogOutfitInfo.GetActiveOutfitID() ~= 0 or C_TransmogOutfitInfo.GetCurrentlyViewedOutfitID() ~= 0 then
        return "wrong_initial_outfit_state"
    end

    local categoryInfo = C_TransmogOutfitInfo.GetAllTransmogOutfitOptionSheatheCategoryInfo(190001)
    if not categoryInfo or #categoryInfo ~= 4 then
        return "wrong_category_count"
    end

    if categoryInfo[1].sheatheCategory ~= Enum.TransmogOutfitSlotOptionSheatheCategory.Default or categoryInfo[1].categoryName ~= "Default" then
        return "wrong_default_category"
    end

    if categoryInfo[4].sheatheCategory ~= Enum.TransmogOutfitSlotOptionSheatheCategory.Hide or categoryInfo[4].categoryName ~= "Hide" then
        return "wrong_hide_category"
    end

    if C_TransmogOutfitInfo.GetAllTransmogOutfitOptionSheatheCategoryInfo(0) ~= nil then
        return "expected_nil_category_info"
    end

    C_TransmogOutfitInfo.ChangeToOutfit(7, false)
    if C_TransmogOutfitInfo.GetActiveOutfitID() ~= 7 or C_TransmogOutfitInfo.GetCurrentlyViewedOutfitID() ~= 7 then
        return "change_to_outfit_failed"
    end

    C_TransmogOutfitInfo.SetPendingTransmogSheatheCategory(16, 2, Enum.TransmogOutfitSlotOptionSheatheCategory.Side)
    local pendingSheatheCategories = rawget(C_TransmogOutfitInfo, "__pendingSheatheCategories")
    if not pendingSheatheCategories or pendingSheatheCategories["16:2"] ~= Enum.TransmogOutfitSlotOptionSheatheCategory.Side then
        return "pending_sheathe_not_recorded"
    end

    C_TransmogOutfitInfo.ChangeToOutfit(7, true)
    if C_TransmogOutfitInfo.GetActiveOutfitID() ~= 0 or C_TransmogOutfitInfo.GetCurrentlyViewedOutfitID() ~= 0 then
        return "toggle_clear_failed"
    end

    local clearedPending = rawget(C_TransmogOutfitInfo, "__pendingSheatheCategories")
    if not clearedPending or next(clearedPending) ~= nil then
        return "pending_sheathe_not_cleared"
    end

    C_TransmogOutfitInfo.ChangeToOutfit(9, false)
    C_TransmogOutfitInfo.ClearOutfit()
    if C_TransmogOutfitInfo.GetActiveOutfitID() ~= 0 or C_TransmogOutfitInfo.GetCurrentlyViewedOutfitID() ~= 0 then
        return "clear_outfit_failed"
    end

    return "ok"
"#;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn transmog_outfit_info_methods_track_outfit_and_sheathe_state() {
    let env = env();
    let result: String = env
        .eval(TRANSMOG_OUTFIT_INFO_SCRIPT)
        .expect("C_TransmogOutfitInfo methods should be queryable");
    assert_eq!(result, "ok");
}
