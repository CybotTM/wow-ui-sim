#![cfg(feature = "client-mists")]

use std::process::Command;

#[test]
fn mists_action_micro_bag_and_status_bars_are_interactive() {
    let output = Command::new("timeout")
        .arg("90")
        .arg(env!("CARGO_BIN_EXE_wow-sim"))
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            r#"
            A_Admin.ClearActionBars()
            A_Admin.SetActionSlot(1, 19750)
            if not HasAction(1) then
                error("seeded action slot 1 is not active")
            end

            local actionType, actionID = GetActionInfo(1)
            if actionType ~= "spell" or actionID ~= 19750 then
                error("slot 1 action mismatch: " .. tostring(actionType) .. "/" .. tostring(actionID))
            end

            if not ActionButton1 then
                error("ActionButton1 missing")
            end
            ActionButton_Update(ActionButton1)
            if ActionButton1.action ~= 1 then
                error("ActionButton1 action mismatch: " .. tostring(ActionButton1.action))
            end
            if not ActionButton1Icon or not ActionButton1Icon:GetTexture() then
                error("ActionButton1 icon did not update from seeded action")
            end

            local microButtons = {
                "CharacterMicroButton",
                "SpellbookMicroButton",
                "TalentMicroButton",
                "QuestLogMicroButton",
                "MainMenuMicroButton",
            }
            for _, buttonName in ipairs(microButtons) do
                local button = _G[buttonName]
                if not button or not button:IsShown() then
                    error(buttonName .. " is missing or hidden")
                end
            end
            UpdateMicroButtons()
            MicroButtonTooltipText(nil, nil)

            if not MainMenuBarBackpackButton or not CharacterBag0Slot then
                error("Mists bag bar buttons are missing")
            end
            A_Admin.ClearBags()
            A_Admin.AddBagItem(0, 1, 6948, 1)
            MainMenuBarBackpackButton_UpdateFreeSlots()
            if CalculateTotalNumberOfFreeBagSlots() ~= 79 then
                error("total free bag slots mismatch: " .. tostring(CalculateTotalNumberOfFreeBagSlots()))
            end
            if MainMenuBarBackpackButton.freeSlots ~= 79 then
                error("backpack free slots mismatch: " .. tostring(MainMenuBarBackpackButton.freeSlots))
            end
            if C_Container.GetContainerNumSlots(0) ~= 16 then
                error("backpack slot count changed")
            end

            if not MainMenuExpBar or not MainMenuExpBar.TextString then
                error("MainMenuExpBar TextStatusBar surface missing")
            end
            MainMenuExpBar:SetMinMaxValues(0, 100)
            MainMenuExpBar:SetValue(25)
            ShowTextStatusBarText(MainMenuExpBar)
            local text = MainMenuExpBar.TextString:GetText()
            if not text or not text:find("25") then
                error("MainMenuExpBar text did not include current value: " .. tostring(text))
            end
            "#,
            "dump-tree",
            "--filter-key",
            "MainMenuBar",
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
        "Mists HUD parity probe emitted Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
