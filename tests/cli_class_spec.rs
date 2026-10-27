use std::path::{Path, PathBuf};
use std::process::Command;
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};

fn isolated_data_dir(root: &Path) -> PathBuf {
    let data = root.join("data");
    if let Some(seed) = std::env::var_os("WOW_SIM_TEST_CVAR_SEED") {
        let cvars = data.join("wow-sim");
        std::fs::create_dir_all(&cvars).unwrap();
        std::fs::copy(seed, cvars.join("cvars.json")).unwrap();
    }
    data
}

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
            .env("XDG_DATA_HOME", isolated_data_dir(root.path()))
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

fn assert_monk_specialization(spec: &str, expected: &str) {
    if ACTIVE != ClientProfile::Retail {
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let output = Command::new("timeout")
        .args(["90", env!("CARGO_BIN_EXE_wow-sim")])
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--class",
            "Monk",
            "--spec",
            spec,
            "--exec-lua",
            r#"
                assert(C_AddOns.LoadAddOn('Blizzard_PlayerSpells'))
                local index = GetSpecialization()
                local frame = CreateFrame('Frame', nil, UIParent, 'ClassSpecContentFrameTemplate')
                frame:Setup(index, UnitSex('player'), 200, 3)
                local id, name = C_SpecializationInfo.GetSpecializationInfo(index)
                print('MONK_CONTEXT:' .. index .. ':' .. id .. ':' .. name .. ':' .. frame.SpecName:GetText())
            "#,
            "dump-tree",
            "--filter",
            "__no_such_frame__",
        ])
        .env("XDG_DATA_HOME", isolated_data_dir(root.path()))
        .env("WOW_SIM_WTF_PATH", root.path().join("wtf"))
        .env("WOW_SIM_NO_SOUND", "1")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stdout}\n{stderr}");
    assert!(stderr.contains(expected), "{stdout}\n{stderr}");
}

#[test]
fn monk_mistweaver_flag_matches_specialization_identity() {
    assert_monk_specialization("Mistweaver", "MONK_CONTEXT:2:270:Mistweaver:Mistweaver");
}

#[test]
fn monk_windwalker_flag_matches_specialization_identity() {
    assert_monk_specialization("Windwalker", "MONK_CONTEXT:3:269:Windwalker:Windwalker");
}
