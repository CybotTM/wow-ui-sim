#![cfg(feature = "client-mists")]

use std::process::Command;

#[test]
fn mists_world_map_opens_with_zone_art_and_quest_pins() {
    let output = Command::new("timeout")
        .arg("90")
        .arg(env!("CARGO_BIN_EXE_wow-sim"))
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            r#"
            ToggleWorldMap()
            if not (WorldMapFrame and WorldMapFrame:IsShown()) then
                error("WorldMapFrame did not open")
            end

            if WorldMapFrame.RefreshAllDataProviders then
                WorldMapFrame:RefreshAllDataProviders()
            end

            local mapID = WorldMapFrame.GetMapID and WorldMapFrame:GetMapID()
            if not mapID or mapID == 0 then
                error("WorldMapFrame has no map ID")
            end

            local canvas = WorldMapFrame.ScrollContainer
                and WorldMapFrame.ScrollContainer.Child
            if not canvas or canvas:GetNumChildren() == 0 then
                error("WorldMapFrame has no scroll-canvas children")
            end

            local questID, questLogIndex = QuestPOIGetQuestIDByVisibleIndex(2)
            if not questID or questID == 0 or not questLogIndex or questLogIndex == 0 then
                error("quest POI visible-index lookup returned no quest")
            end

            local questMapID = GetQuestUiMapID(questID)
            if not questMapID or questMapID == 0 then
                error("quest POI has no UiMapID")
            end

            if InActiveBattlefield() then
                error("default simulator state should not be an active battlefield")
            end
            "#,
            "lua-errors",
        ])
        .output()
        .expect("failed to run wow-sim");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "wow-sim failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert_no_lua_errors(&stdout, &stderr);
}

fn assert_no_lua_errors(stdout: &str, stderr: &str) {
    assert!(
        !stdout.contains("Lua error") && !stderr.contains("Lua error"),
        "world map opened with Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
