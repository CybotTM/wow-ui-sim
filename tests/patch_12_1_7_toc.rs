//! Patch 12.1.7 TOC regression consumers; original fixture provenance lives beside the bytes.

use std::path::{Path, PathBuf};
use wow_ui_sim::client_profile::{ACTIVE, ACTIVE_INTERFACE_VERSION, ClientProfile};
use wow_ui_sim::loader::{find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

fn write_addon(root: &Path, contents: &str) -> PathBuf {
    let addon = root.join("Blizzard_LoadLocale");
    std::fs::create_dir_all(&addon).unwrap();
    std::fs::write(addon.join("Blizzard_LoadLocale.toc"), contents).unwrap();
    addon
}

#[test]
fn p1217_selected_interface_is_visible_to_addons() {
    let expected = match ACTIVE {
        ClientProfile::Ptr => 120107,
        ClientProfile::WowForever => 16001,
        _ => ACTIVE_INTERFACE_VERSION,
    };
    let env = WowLuaEnv::new().unwrap();
    assert_eq!(
        env.eval::<i64>("return select(4, GetBuildInfo())").unwrap(),
        i64::from(expected)
    );
    assert_eq!(ACTIVE.interface_version(), expected);
}

#[test]
#[cfg(feature = "client-ptr")]
fn p1217_selected_interface_admits_current_addon() {
    use wow_ui_sim::toc::TocFile;

    let expected = 120107;
    let addon = tempfile::tempdir().unwrap();
    let toc = TocFile::parse(addon.path(), &format!("## Interface: {expected}\n"));
    assert!(toc.supports_interface_version(ACTIVE_INTERFACE_VERSION));
}

#[test]
fn p1217_metadata_variables_are_ignored_only_in_new_epoch() {
    let env = WowLuaEnv::new().unwrap();
    let root = tempfile::tempdir().unwrap();
    let addon = write_addon(
        root.path(),
        r#"## Title: Original title
## X-Probe: previous
## Dependencies: Stable
## Notes: conditional [AllowLoadGameType mainline]
## X-Conditional: allowed [AllowLoadGameType mainline]
## X-Conditional: excluded [AllowLoadGameType vanilla]
## X-Later: before
## X-Probe: bad[Game]
## Title: bad[Family]
## Dependencies: Phantom[TextLocale]
## OptionalDeps: Phantom[RegionalTextLocale]
## X-OnlyVariable: [RegionalTextLocale]
## X-Later: after [AllowLoadGameType mainline]
[Family]\[Game]\[TextLocale].lua
"#,
    );
    let family = if matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::WowForever
    ) {
        "Mainline"
    } else {
        "Classic"
    };
    let game = match ACTIVE {
        ClientProfile::Retail | ClientProfile::Ptr => "Standard",
        ClientProfile::WowForever => "Camelot",
        ClientProfile::Wrath => "Wrath",
        ClientProfile::Mists => "Mists",
        ClientProfile::Era | ClientProfile::Anniversary => "Vanilla",
    };
    let files = addon.join(family).join(game);
    std::fs::create_dir_all(&files).unwrap();
    std::fs::write(files.join("enUS.lua"), "TOC_FILE_VARIABLES_LOADED = true").unwrap();
    let toc_path = find_toc_file(&addon).unwrap();
    let result = load_addon(&env.loader_env(), &toc_path).unwrap();
    assert!(result.warnings.is_empty(), "{:?}", result.warnings);
    assert_eq!(result.lua_files, 1);
    assert!(
        env.eval::<bool>("return TOC_FILE_VARIABLES_LOADED")
            .unwrap()
    );
    let metadata: (String, String, String, bool, String) = env
        .eval(
            r#"
        local name = "Blizzard_LoadLocale"
        return C_AddOns.GetAddOnMetadata(name, "X-Probe"),
            C_AddOns.GetAddOnMetadata(name, "Title"),
            table.concat({C_AddOns.GetAddOnDependencies(name)}, ","),
            C_AddOns.GetAddOnMetadata(name, "X-OnlyVariable") == nil,
            C_AddOns.GetAddOnMetadata(name, "OptionalDeps") or "<absent>"
    "#,
        )
        .unwrap();
    let rejects_variables = ACTIVE == ClientProfile::Ptr;
    let expected = if rejects_variables {
        ("previous", "Original title", "Stable", true, "<absent>")
    } else {
        (
            "bad[Game]",
            "bad[Family]",
            "Stable,Phantom[TextLocale]",
            false,
            "Phantom[RegionalTextLocale]",
        )
    };
    assert_eq!(
        metadata,
        (
            expected.0.into(),
            expected.1.into(),
            expected.2.into(),
            expected.3,
            expected.4.into()
        )
    );
    if matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::WowForever
    ) {
        assert_eq!(
            env.eval::<(String, String)>(
                r#"
            return C_AddOns.GetAddOnMetadata("Blizzard_LoadLocale", "X-Conditional"),
                C_AddOns.GetAddOnMetadata("Blizzard_LoadLocale", "X-Later")
        "#
            )
            .unwrap(),
            ("allowed".into(), "after".into())
        );
    }
}

#[test]
#[cfg(any(feature = "client-ptr", feature = "client-wowforever"))]
fn p1217_original_loadlocale_default_us_consumer() {
    let env = WowLuaEnv::new().unwrap();
    let root = tempfile::tempdir().unwrap();
    let addon = write_addon(
        root.path(),
        include_str!("fixtures/p1217-loadlocale/Blizzard_LoadLocale.toc"),
    );
    for locale in ["enUS", "enGB"] {
        let directory = addon.join(locale);
        std::fs::create_dir(&directory).unwrap();
        let bytes: &[u8] = match locale {
            "enUS" => include_bytes!("fixtures/p1217-loadlocale/enUS/LoadLocale.lua"),
            _ => include_bytes!("fixtures/p1217-loadlocale/enGB/LoadLocale.lua"),
        };
        std::fs::write(directory.join("LoadLocale.lua"), bytes).unwrap();
    }
    let result = load_addon(&env.loader_env(), &find_toc_file(&addon).unwrap()).unwrap();
    assert!(result.warnings.is_empty(), "{:?}", result.warnings);
    assert_eq!(result.lua_files, 1);
    assert_eq!(
        env.eval::<(String, bool, bool)>(
            "return UI_LOCALE, LOCALE_enUS == true, LOCALE_enGB == nil"
        )
        .unwrap(),
        ("enUS".into(), true, true)
    );
}
