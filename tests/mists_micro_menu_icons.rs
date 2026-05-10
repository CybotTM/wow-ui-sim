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
fn mists_micro_menu_buttons_resolve_visible_textures() {
    let output = Command::new(wow_sim_binary())
        .env("WOW_SIM_NO_SAVED_VARS", "1")
        .env("WOW_SIM_NO_ADDONS", "1")
        .arg("dump-tree")
        .arg("--filter-key")
        .arg("MicroButton")
        .output()
        .expect("wow-sim dump-tree should run");

    assert!(
        output.status.success(),
        "wow-sim dump-tree failed with status {:?}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    let tree = String::from_utf8_lossy(&output.stdout);
    let missing_micro_button_textures: Vec<_> = tree
        .lines()
        .filter(|line| line.contains("UI-MicroButton") && line.contains("(MISSING)"))
        .collect();

    assert!(
        missing_micro_button_textures.is_empty(),
        "Mists micro menu should not expose missing button textures: {missing_micro_button_textures:#?}"
    );
}
