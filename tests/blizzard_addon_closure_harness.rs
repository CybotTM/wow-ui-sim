mod common;

use std::fs;
use std::path::Path;
use wow_ui_sim::loader::BlizzardAddonOverride;

struct TempBlizzardUiDir {
    tempdir: tempfile::TempDir,
}

impl TempBlizzardUiDir {
    fn new(prefix: &str) -> Self {
        let tempdir = tempfile::Builder::new()
            .prefix(prefix)
            .tempdir()
            .expect("failed to create temp dir");
        Self { tempdir }
    }

    fn add_addon(&self, addon_name: &str, toc: &str) {
        let addon_dir = self.path().join(addon_name);
        fs::create_dir_all(&addon_dir).expect("failed to create addon dir");
        fs::write(addon_dir.join(format!("{addon_name}.toc")), toc)
            .expect("failed to write addon toc");
        fs::write(addon_dir.join("Core.lua"), "return true\n").expect("failed to write addon lua");
    }

    fn path(&self) -> &Path {
        self.tempdir.path().as_ref()
    }
}

#[test]
fn blizzard_addon_closure_harness_runs_assertions_after_loading_the_closure() {
    let ui = TempBlizzardUiDir::new("closure-harness");
    ui.add_addon(
        "Blizzard_B",
        r#"
## Title: Blizzard_B
## AllowLoad: Both
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_C",
        r#"
## Title: Blizzard_C
## AllowLoad: Both
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_A",
        r#"
## Title: Blizzard_A
## AllowLoad: Both
## Dependencies: Blizzard_B
Core.lua
"#,
    );

    let overrides = &[BlizzardAddonOverride {
        addon: "Blizzard_A",
        extra_roots: &["Blizzard_C"],
    }];

    common::blizzard_addon_harness::with_blizzard_addon_closure_in_dir(
        ui.path(),
        &["Blizzard_A"],
        overrides,
        |env, loaded| {
            assert_eq!(
                loaded,
                ["Blizzard_B", "Blizzard_C", "Blizzard_A"],
                "closure harness should load dependencies and override roots before the target addon",
            );

            let loaded_a: bool = env
                .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_A")"#)
                .expect("addon load probe should return");
            let loaded_b: bool = env
                .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_B")"#)
                .expect("dependency load probe should return");
            let loaded_c: bool = env
                .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_C")"#)
                .expect("override root load probe should return");

            assert!(loaded_a);
            assert!(loaded_b);
            assert!(loaded_c);

            let result: String = env
                .eval(
                    r#"
                    local loaded = {}
                    for _, name in ipairs({"Blizzard_A", "Blizzard_B", "Blizzard_C"}) do
                        loaded[#loaded + 1] = tostring(C_AddOns.IsAddOnLoaded(name))
                    end
                    return table.concat(loaded, ",")
                    "#,
                )
                .expect("lua assertion probe should return");
            assert_eq!(result, "true,true,true");
        },
    );
}
