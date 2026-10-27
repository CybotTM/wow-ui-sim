use std::process::Command;
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};

#[test]
fn cli_class_and_spec_are_visible_to_exec_lua() {
    if ACTIVE != ClientProfile::Retail {
        return;
    }
    for (class, spec, expected) in [
        ("dRuId", "restoration", "CLI_CONTEXT:Druid:4"),
        ("Mage", "FIRE", "CLI_CONTEXT:Mage:2"),
    ] {
        let root = tempfile::tempdir().unwrap();
        let output = Command::new("timeout")
            .args(["90", env!("CARGO_BIN_EXE_wow-sim")])
            .args([
                "--no-addons",
                "--no-saved-vars",
                "--class",
                class,
                "--spec",
                spec,
                "--exec-lua",
                "print('CLI_CONTEXT:' .. UnitClass('player') .. ':' .. GetSpecialization())",
                "dump-tree",
                "--filter",
                "__no_such_frame__",
            ])
            .env("XDG_DATA_HOME", root.path().join("data"))
            .env("WOW_SIM_WTF_PATH", root.path().join("wtf"))
            .env("WOW_SIM_NO_SOUND", "1")
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stdout}\n{stderr}");
        assert!(stderr.contains(expected), "{stdout}\n{stderr}");
    }
}
