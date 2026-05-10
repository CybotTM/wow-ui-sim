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
fn mists_character_and_reputation_panels_have_backing_data() {
    let lua = r#"
        ToggleCharacter("PaperDollFrame")
        local head = CharacterHeadSlot
        if not head or head:GetID() ~= 1 then
            error("CharacterHeadSlot was not wired to inventory slot 1")
        end
        if not head.icon or not head.icon:GetTexture() then
            error("CharacterHeadSlot has no icon texture")
        end

        ToggleCharacter("ReputationFrame")
        if ReputationFrame_Update then
            ReputationFrame_Update()
        end
        if not ReputationBar1 or not ReputationBar1:IsShown() then
            error("ReputationBar1 was not shown")
        end
        local name = ReputationBar1FactionName and ReputationBar1FactionName:GetText()
        if type(name) ~= "string" or name == "" then
            error("ReputationBar1 has no faction name")
        end
    "#;

    let output = Command::new(wow_sim_binary())
        .env("WOW_SIM_NO_SAVED_VARS", "1")
        .env("WOW_SIM_NO_ADDONS", "1")
        .arg("--exec-lua")
        .arg(lua)
        .arg("dump-tree")
        .output()
        .expect("wow-sim dump-tree should run");

    assert!(
        output.status.success(),
        "Mists character/reputation probe failed with status {:?}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Lua error"),
        "Mists character/reputation probe emitted Lua errors:\n{stderr}"
    );
}
