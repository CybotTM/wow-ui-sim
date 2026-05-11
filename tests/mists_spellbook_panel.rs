#![cfg(feature = "client-mists")]

use std::process::Command;

#[test]
fn mists_spellbook_populates_visible_spell_buttons() {
    let output = Command::new("timeout")
        .arg("90")
        .arg(env!("CARGO_BIN_EXE_wow-sim"))
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            r#"
            ToggleSpellBook(BOOKTYPE_SPELL)
            if SpellBookFrame and SpellBookFrame.Update then
                SpellBookFrame:Update()
            end

            local populated = 0
            for i = 1, 12 do
                local button = _G["SpellButton" .. i]
                local text = button and button.SpellName and button.SpellName:GetText()
                local texture = button and button.IconTexture and button.IconTexture:GetTexture()
                if text and text ~= "" and texture then
                    populated = populated + 1
                end
            end

            if populated == 0 then
                error("spellbook has no populated spell buttons")
            end
            "#,
            "dump-tree",
            "--filter-key",
            "SpellBookFrame",
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
        "spellbook opened with Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
