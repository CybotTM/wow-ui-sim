#![cfg(feature = "client-mists")]

use std::path::PathBuf;
use std::process::Command;

fn wow_sim_binary() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_wow-sim")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join("debug")
                .join("wow-sim")
        })
}

#[test]
fn mists_game_menu_options_opens_interface_settings_without_lua_errors() {
    let output = Command::new("timeout")
        .arg("90")
        .arg(wow_sim_binary())
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            r#"
            ToggleGameMenu()
            if GameMenuFrame == nil or GameMenuFrame:IsShown() ~= true then
                error("GameMenuFrame did not open")
            end
            if GameMenuButtonOptions == nil then
                error("GameMenuButtonOptions is missing")
            end

            GameMenuButtonOptions:Click()
            if SettingsPanel == nil or SettingsPanel:IsShown() ~= true then
                error("SettingsPanel did not open from game-menu options")
            end
            if GameMenuFrame:IsShown() then
                error("GameMenuFrame stayed open after opening options")
            end

            if not Settings.INTERFACE_CATEGORY_ID then
                error("interface settings category id is missing")
            end
            if not SettingsPanel:OpenToCategory(Settings.INTERFACE_CATEGORY_ID) then
                error("interface settings category did not open")
            end
            local currentCategory = SettingsPanel.GetCurrentCategory and SettingsPanel:GetCurrentCategory()
            if currentCategory == nil or currentCategory:GetID() ~= Settings.INTERFACE_CATEGORY_ID then
                error("interface settings category did not focus")
            end

            if EditModeManagerFrame ~= nil
                and type(EditModeManagerFrame.IsShown) == "function"
                and EditModeManagerFrame:IsShown() then
                error("Mists interface options must not open retail EditMode")
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
        stdout.trim().ends_with("[]")
            && !stdout.contains("Lua error")
            && !stderr.contains("Lua error")
            && !stderr.contains("[exec-lua] error"),
        "Interface options flow emitted Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
