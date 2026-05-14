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
    assert_no_texture_directory_errors(&stdout, &stderr);
}

#[test]
fn mists_talents_and_glyphs_mutate_selected_state() {
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
            local row = PlayerTalentFrameTalents and PlayerTalentFrameTalents.tier1
            local button = row and row.talent1
            if not button then
                error("talent row button did not exist")
            end

            local talentID = button:GetID()
            PlayerTalentFrameTalent_OnClick(button, "LeftButton")
            if PlayerTalentFrame_GetTalentSelections() ~= talentID then
                error("talent row click did not select the talent")
            end

            if not LearnTalents(PlayerTalentFrame_GetTalentSelections()) then
                error("LearnTalents did not accept the selected talent")
            end
            local _, _, _, selected, _, _, _, _, _, isKnown = GetTalentInfoByID(talentID)
            if not selected or not isKnown then
                error("learned talent did not update selected/known state")
            end

            if type(PlayerTalentFrame_ShowGlyphFrame) == "function" then
                PlayerTalentFrame_ShowGlyphFrame()
            else
                ToggleGlyphFrame()
            end
            if not GlyphFrame or not GlyphFrame:IsShown() then
                error("GlyphFrame did not open")
            end

            local socket = GlyphFrameGlyph1
            if not socket or not socket:IsShown() then
                error("glyph socket did not exist")
            end

            C_GlyphInfo.UseGlyph(5001)
            if not GlyphMatchesSocket(socket:GetID()) then
                error("pending glyph did not match the socket")
            end

            GlyphFrameGlyph_OnClick(socket, "LeftButton")
            local enabled, _, _, glyphSpell, _, glyphID = GetGlyphSocketInfo(socket:GetID())
            if not enabled or glyphSpell ~= 635 or glyphID ~= 5001 then
                error("glyph socket click did not install the pending glyph")
            end
            if HasPendingGlyphCast() then
                error("glyph socket click did not consume the pending glyph")
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
    assert_no_texture_directory_errors(&stdout, &stderr);
}

#[test]
fn mists_specialization_learn_button_activates_spec_for_talents() {
    let output = Command::new("timeout")
        .arg("90")
        .arg(env!("CARGO_BIN_EXE_wow-sim"))
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            r#"
            A_Admin.SetSpec(4)
            ToggleTalentFrame()
            if not PlayerTalentFrame or not PlayerTalentFrame:IsShown() then
                error("PlayerTalentFrame did not open")
            end

            PlayerTalentTab_OnClick(PlayerTalentFrameTab1)
            PlayerTalentFrame_UpdateSpecFrame(PlayerTalentFrameSpecialization, 2)
            local learnButton = PlayerTalentFrameSpecialization.learnButton
            if not learnButton or not learnButton:IsEnabled() then
                error("specialization Learn button was not enabled")
            end

            learnButton:Click()
            local dialog = StaticPopup_FindVisible and StaticPopup_FindVisible("CONFIRM_LEARN_SPEC")
            if not dialog or not dialog.button1 then
                error("specialization Learn confirmation did not open")
            end
            dialog.button1:Click()

            if C_SpecializationInfo.GetSpecialization() ~= 2 or GetSpecialization() ~= 2 then
                error("specialization Learn did not activate the previewed spec")
            end

            PlayerTalentTab_OnClick(PlayerTalentFrameTab2)
            if not PlayerTalentFrameTalents or not PlayerTalentFrameTalents:IsShown() then
                error("talent rows did not become reachable after learning a spec")
            end

            local row = PlayerTalentFrameTalents.tier1
            local button = row and row.talent1
            if not button or not button:IsShown() or not button.icon or not button.icon:GetTexture() then
                error("learned specialization did not expose populated talent buttons")
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
    assert_no_texture_directory_errors(&stdout, &stderr);
}

fn assert_no_lua_errors(stdout: &str, stderr: &str) {
    assert!(
        !stdout.contains("Lua error") && !stderr.contains("Lua error"),
        "talents/glyphs opened with Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}

fn assert_no_texture_directory_errors(stdout: &str, stderr: &str) {
    assert!(
        !stdout.contains("BlizzardInterfaceArt/: The image format could not be determined")
            && !stderr.contains("BlizzardInterfaceArt/: The image format could not be determined"),
        "talents/glyphs tried to load the BlizzardInterfaceArt directory as a texture\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
