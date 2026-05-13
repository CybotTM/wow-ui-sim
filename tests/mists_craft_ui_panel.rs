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
    let output = Command::new("timeout")
        .arg("90")
        .arg(wow_sim_binary())
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
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
            "screenshot",
            "--filter",
            "CraftFrame",
            "--output",
            "/tmp/mists-craft-ui-panel",
        ])
        .output()
        .expect("failed to run wow-sim screenshot");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "CraftFrame screenshot failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        !stdout.contains("Lua error")
            && !stderr.contains("Lua error")
            && !stderr.contains("[exec-lua] error"),
        "CraftFrame screenshot emitted Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        parse_texture_request_count(&stderr).is_some_and(|count| count > 1),
        "CraftFrame screenshot did not render panel textures\nstderr:\n{stderr}"
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
