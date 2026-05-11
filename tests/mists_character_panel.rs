#![cfg(feature = "client-mists")]

use std::process::Command;

#[test]
fn mists_character_panel_populates_gear_and_reputation() {
    let output = Command::new("timeout")
        .arg("90")
        .arg(env!("CARGO_BIN_EXE_wow-sim"))
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            r#"
            ToggleCharacter("PaperDollFrame")
            if not CharacterFrame or not CharacterFrame:IsShown() then
                error("CharacterFrame did not open")
            end

            local gearSlots = {
                "CharacterHeadSlot",
                "CharacterChestSlot",
                "CharacterLegsSlot",
                "CharacterFeetSlot",
                "CharacterMainHandSlot",
            }
            for _, slotName in ipairs(gearSlots) do
                local slot = _G[slotName]
                local icon = _G[slotName .. "IconTexture"]
                if not slot or not slot:IsShown() then
                    error(slotName .. " is not shown")
                end
                if not icon or not icon:GetTexture() then
                    error(slotName .. " has no icon texture")
                end
            end

            ToggleCharacter("ReputationFrame")
            if not ReputationFrame or not ReputationFrame:IsShown() then
                error("ReputationFrame did not open")
            end
            if type(ReputationFrame_Update) == "function" then
                ReputationFrame_Update()
            end
            if GetNumFactions() <= 0 then
                error("no factions are exposed")
            end

            local populatedRows = 0
            for i = 1, 15 do
                local row = _G["ReputationBar" .. i]
                local name = _G["ReputationBar" .. i .. "FactionName"]
                local bar = _G["ReputationBar" .. i .. "ReputationBar"]
                if row and row:IsShown() and name and name:GetText() and bar then
                    populatedRows = populatedRows + 1
                end
            end
            if populatedRows == 0 then
                error("ReputationFrame has no populated rows")
            end
            "#,
            "dump-tree",
            "--filter-key",
            "CharacterFrame",
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
        "character panel opened with Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
