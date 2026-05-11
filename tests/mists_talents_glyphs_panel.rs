#![cfg(feature = "client-mists")]

use std::process::Command;

#[test]
fn mists_talents_and_glyphs_panel_populates_rows_and_sockets() {
    let output = Command::new("timeout")
        .arg("90")
        .arg(env!("CARGO_BIN_EXE_wow-sim"))
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            r#"
            ToggleTalentFrame()
            if not PlayerTalentFrame or not PlayerTalentFrame:IsShown() then
                error("PlayerTalentFrame did not open")
            end
            PlayerTalentTab_OnClick(PlayerTalentFrameTab2)
            if type(PlayerTalentFrame_Refresh) == "function" then
                PlayerTalentFrame_Refresh()
            end

            local populatedTalents = 0
            for tier = 1, 6 do
                local row = PlayerTalentFrameTalents and PlayerTalentFrameTalents["tier" .. tier]
                for column = 1, 3 do
                    local button = row and row["talent" .. column]
                    if button and button:IsShown() and button.icon and button.icon:GetTexture() then
                        populatedTalents = populatedTalents + 1
                    end
                end
            end
            if populatedTalents == 0 then
                error("talent rows have no populated buttons")
            end

            if type(PlayerTalentFrame_ShowGlyphFrame) == "function" then
                PlayerTalentFrame_ShowGlyphFrame()
            else
                ToggleGlyphFrame()
            end
            if not GlyphFrame or not GlyphFrame:IsShown() then
                error("GlyphFrame did not open")
            end

            local enabledGlyphSockets = 0
            for i = 1, 6 do
                local socket = _G["GlyphFrameGlyph" .. i]
                local enabled = GetGlyphSocketInfo(i, C_SpecializationInfo.GetActiveSpecGroup())
                if socket and socket:IsShown() and enabled then
                    enabledGlyphSockets = enabledGlyphSockets + 1
                end
            end
            if enabledGlyphSockets == 0 then
                error("glyph frame has no enabled sockets")
            end
            "#,
            "dump-tree",
            "--filter-key",
            "PlayerTalentFrame",
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
        "talents/glyphs opened with Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
