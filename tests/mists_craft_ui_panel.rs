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
fn mists_craft_frame_load_ui_creates_renderable_craft_frame() {
    assert_panel_loads_cleanly(
        "CraftFrame",
        "CraftFrame",
        r#"
            CraftFrame_LoadUI()
            if not CraftFrame then
                error("CraftFrame was not created")
            end
            ShowUIPanel(CraftFrame)
            if not CraftFrame:IsShown() then
                error("CraftFrame did not show")
            end
            if (CraftFrame:GetWidth() or 0) <= 0 or (CraftFrame:GetHeight() or 0) <= 0 then
                error("CraftFrame has no renderable bounds")
            end
            "#,
    );
}

#[test]
fn mists_trade_skill_frame_load_ui_creates_renderable_trade_skill_frame() {
    assert_panel_loads_cleanly(
        "TradeSkillFrame",
        "TradeSkillFrame",
        r#"
            TradeSkillFrame_LoadUI()
            if not TradeSkillFrame then
                error("TradeSkillFrame was not created")
            end
            ShowUIPanel(TradeSkillFrame)
            if not TradeSkillFrame:IsShown() then
                error("TradeSkillFrame did not show")
            end
            if (TradeSkillFrame:GetWidth() or 0) <= 0 or (TradeSkillFrame:GetHeight() or 0) <= 0 then
                error("TradeSkillFrame has no renderable bounds")
            end
            "#,
    );
}

fn assert_panel_loads_cleanly(panel_name: &str, filter_name: &str, exec_lua: &str) {
    let output_path = format!("/tmp/mists-{}-panel", panel_name.to_ascii_lowercase());
    let output = Command::new("timeout")
        .arg("90")
        .arg(wow_sim_binary())
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            exec_lua,
            "screenshot",
            "--filter",
            filter_name,
            "--output",
            &output_path,
        ])
        .output()
        .expect("failed to run wow-sim screenshot");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "{panel_name} screenshot failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        !stdout.contains("Lua error")
            && !stderr.contains("Lua error")
            && !stderr.contains("[exec-lua] error"),
        "{panel_name} screenshot emitted Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        parse_texture_request_count(&stderr).is_some_and(|count| count > 1),
        "{panel_name} screenshot did not render panel textures\nstderr:\n{stderr}"
    );
}

fn parse_texture_request_count(stderr: &str) -> Option<u32> {
    stderr.lines().rev().find_map(|line| {
        let (_, suffix) = line.split_once("QuadBatch: ")?;
        let (_, texture_suffix) = suffix.split_once(" quads, ")?;
        let (count, _) = texture_suffix.split_once(" texture requests")?;
        count.parse().ok()
    })
}
