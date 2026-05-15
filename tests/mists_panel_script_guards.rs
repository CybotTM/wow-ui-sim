use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_script(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path)).expect("failed to read script")
}

#[test]
fn mists_panel_scripts_use_bounded_cargo_target_dir() {
    for script_path in [
        "scripts/mists-panel-parity.sh",
        "scripts/test-mists-addon-panels.sh",
    ] {
        let script = read_script(script_path);

        assert!(
            script.contains("MISTS_CARGO_TARGET_DIR"),
            "{script_path} should allow a Mists-specific Cargo target override"
        );
        assert!(
            script.contains("wow-ui-sim/cargo-targets/mists-panel-parity"),
            "{script_path} should default scripted Mists builds outside repo target/"
        );
        assert!(
            script.contains("CARGO_INCREMENTAL=\"${CARGO_INCREMENTAL:-0}\""),
            "{script_path} should disable incremental artifacts unless explicitly requested"
        );
    }
}

#[test]
fn installed_addon_panel_runner_separates_exit_and_interrupt_cleanup() {
    let script = read_script("scripts/test-mists-addon-panels.sh");

    assert!(
        script.contains("cleanup_active_addon_on_exit"),
        "addon panel runner should have a quiet EXIT cleanup path"
    );
    assert!(
        script.contains("cleanup_active_addon_on_interrupt"),
        "addon panel runner should have an interrupt-specific cleanup path"
    );
    assert!(
        !script.contains("trap cleanup_active_addon EXIT"),
        "normal EXIT cleanup should not use the interrupt/error-reporting cleanup path"
    );
    assert!(
        !script.contains("trap 'cleanup_active_addon; exit 130' INT TERM"),
        "INT/TERM cleanup should use the interrupt-specific path"
    );
}
