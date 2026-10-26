//! PTR 70493 declaration-backed Edit Mode consumer boundaries.
#![cfg(feature = "client-ptr")]

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen_with_overrides, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn load_edit_mode() -> (WowLuaEnv, PathBuf) {
    let root = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let env = crate::common::blizzard_addon_harness::new_blizzard_addon_env(&root);
    let addons = discover_blizzard_addon_closure_for_screen_with_overrides(
        &root,
        ScreenKind::Game,
        &["Blizzard_EditMode"],
        &[],
    );
    let (_, toc_path) = addons
        .iter()
        .find(|(name, _)| name == "Blizzard_EditMode")
        .unwrap();
    let toc = TocFile::from_file(toc_path).unwrap();
    for relative in [
        "Shared/EditModeSettingDisplayInfo.lua",
        "Shared/EditModePresetLayoutsManager.lua",
    ] {
        assert!(
            toc.files
                .iter()
                .any(|path| path == std::path::Path::new(relative)),
            "selected original {relative}"
        );
    }
    for (name, toc) in addons {
        let loaded = load_addon(&env.loader_env(), &toc).unwrap();
        if name == "Blizzard_EditMode" {
            for warning in loaded.warnings {
                eprintln!("original EditMode warning: {warning}");
            }
        }
    }
    (env, root.join("Blizzard_EditMode"))
}

#[test]
fn boss_growth_dropdown_consumes_three_declared_directions() {
    let (env, root) = load_edit_mode();
    // The complete selected file has other declaration gaps. Exercise its unchanged
    // dropdown constructor separately, without supplying any replacement enums.
    let source =
        std::fs::read_to_string(root.join("Shared/EditModeSettingDisplayInfo.lua")).unwrap();
    let start = source.find("\t\t-- Grow Direction").unwrap();
    let end = start + source[start..].find("\t\t-- ShowCastTime").unwrap();
    let padding = "\n".repeat(
        source[..start]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count(),
    );
    env.exec_named(
        &format!(
            "{padding}ObservedBossGrowth = {{{}\n}}",
            &source[start..end]
        ),
        "@Interface/AddOns/Blizzard_EditMode/Shared/EditModeSettingDisplayInfo.lua",
    )
    .expect("original dropdown constructor at lines 250-261");
    env.exec(
        r#"
        local options = ObservedBossGrowth[1].options
        assert(#options == 3)
        for index, expected in ipairs({0, 1, 2}) do
            assert(type(options[index].value) == 'number')
            assert(options[index].value == expected, 'original direction option')
        end
        assert(Enum.BossFrameGrowDirection.Left == 0)
        assert(Enum.BossFrameGrowDirection.Center == 1)
        assert(Enum.BossFrameGrowDirection.Right == 2)
        local meta = Enum.BossFrameGrowDirectionMeta
        assert(meta.NumValues == 3 and meta.MinValue == 0 and meta.MaxValue == 2)
    "#,
    )
    .unwrap();
}

#[test]
fn preset_manager_indexes_declared_gamepad_layout_separately() {
    let (env, root) = load_edit_mode();
    // Supplied maps isolate this separately reached manager from the earlier
    // Mainline preset's unrelated missing settings. No functions are replaced.
    env.exec(
        r#"
        EDIT_MODE_MODERN_SYSTEM_MAP = {}
        EDIT_MODE_CLASSIC_SYSTEM_MAP = {}
        EDIT_MODE_GAMEPAD_SYSTEM_MAP = {
            [Enum.EditModeSystem.Minimap] = { settings = {}, anchorInfo = { offsetX = 73 } }
        }
    "#,
    )
    .unwrap();
    let source =
        std::fs::read_to_string(root.join("Shared/EditModePresetLayoutsManager.lua")).unwrap();
    env.exec_named(
        &source,
        "@Interface/AddOns/Blizzard_EditMode/Shared/EditModePresetLayoutsManager.lua",
    )
    .expect("original manager Gamepad index at line 41");
    env.exec(
        r#"
        assert(Enum.EditModePresetLayouts.Modern == 0)
        assert(Enum.EditModePresetLayouts.Classic == 1)
        assert(Enum.EditModePresetLayouts.Gamepad == 2)
        local meta = Enum.EditModePresetLayoutsMeta
        assert(meta.NumValues == 3 and meta.MinValue == 0 and meta.MaxValue == 2)
        local manager = EditModePresetLayoutManager
        local layouts = manager:GetCopyOfPresetLayouts()
        assert(#layouts == 3 and layouts[3].layoutIndex == 2)
        assert(layouts[3].interfaceStyle == Enum.InputDeviceInterfaceType.Gamepad)
        local map = manager:GetPresetLayoutMapByIndex(2)
        assert(map == EDIT_MODE_GAMEPAD_SYSTEM_MAP)
        assert(manager:GetPresetLayoutMapByIndex(0) == EDIT_MODE_MODERN_SYSTEM_MAP)
        assert(manager:GetPresetLayoutMapByIndex(1) == EDIT_MODE_CLASSIC_SYSTEM_MAP)
        local anchor = manager:GetPresetLayoutSystemAnchorInfo(2, Enum.EditModeSystem.Minimap)
        assert(anchor.offsetX == 73)
        anchor.offsetX = -19
        assert(map[Enum.EditModeSystem.Minimap].anchorInfo.offsetX == 73)
    "#,
    )
    .unwrap();
}
