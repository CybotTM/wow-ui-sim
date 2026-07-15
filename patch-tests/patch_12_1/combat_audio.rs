use std::{fs, path::Path};

use super::{blizzard_ui_dir, load_game_ui_without_player_choice};

const SNAPSHOT_ONLY_METHODS: &[&str] = &[
    "EnumerateInterruptCastInfo",
    "EnumerateInterruptCastSuccessInfo",
    "EnumerateSayCombatEndInfo",
    "EnumerateSayCombatStartInfo",
    "EnumeratetWhenTargetDiesInfo",
    "GetInterruptCastInfo",
    "GetInterruptCastSuccessInfo",
    "GetSayCombatEndInfo",
    "GetSayCombatStartInfo",
    "GetWhenTargetDiesInfo",
];

fn assert_source_tree_omits_methods(path: &Path) {
    for entry in fs::read_dir(path).expect("PTR AddOns directory should be readable") {
        let entry = entry.expect("PTR source entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            assert_source_tree_omits_methods(&path);
            continue;
        }

        let extension = path.extension().and_then(|value| value.to_str());
        if !matches!(extension, Some("lua" | "xml" | "toc")) {
            continue;
        }

        let source = fs::read_to_string(&path).expect("PTR source file should be UTF-8 text");
        for method in SNAPSHOT_ONLY_METHODS {
            assert!(
                !source.contains(method),
                "snapshot-only CombatAudioAlertUtil method {method} unexpectedly appears in {}",
                path.display(),
            );
        }
    }
}

/// Proves the ten proposed methods are absent from both PTR source and runtime.
#[test]
fn snapshot_only_combat_audio_methods_remain_absent() {
    assert_source_tree_omits_methods(&blizzard_ui_dir());

    let env = load_game_ui_without_player_choice();
    let (namespace_type, active_method_type, absent_count): (String, String, i32) = env
        .eval(
            r#"
            local names = {
                "EnumerateInterruptCastInfo",
                "EnumerateInterruptCastSuccessInfo",
                "EnumerateSayCombatEndInfo",
                "EnumerateSayCombatStartInfo",
                "EnumeratetWhenTargetDiesInfo",
                "GetInterruptCastInfo",
                "GetInterruptCastSuccessInfo",
                "GetSayCombatEndInfo",
                "GetSayCombatStartInfo",
                "GetWhenTargetDiesInfo",
            }
            local absentCount = 0
            for _, name in ipairs(names) do
                if CombatAudioAlertUtil[name] == nil then
                    absentCount = absentCount + 1
                end
            end
            return type(CombatAudioAlertUtil),
                type(CombatAudioAlertUtil.EnumeratePercentInfo),
                absentCount
            "#,
        )
        .expect("CombatAudioAlertUtil runtime probe succeeds");

    assert_eq!(namespace_type, "table");
    assert_eq!(active_method_type, "function");
    assert_eq!(absent_count, SNAPSHOT_ONLY_METHODS.len() as i32);
}
