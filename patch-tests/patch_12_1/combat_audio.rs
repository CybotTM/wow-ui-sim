use super::{assert_ptr_source_omits_qualified_methods, load_game_ui_without_player_choice};

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

/// Proves the ten proposed methods are absent from both PTR source and runtime.
#[test]
fn snapshot_only_combat_audio_methods_remain_absent() {
    assert_ptr_source_omits_qualified_methods("CombatAudioAlertUtil", SNAPSHOT_ONLY_METHODS);

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
