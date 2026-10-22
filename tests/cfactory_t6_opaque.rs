//! Original consumer behind Forever's erased securecallfunction callee.
#![cfg(any(feature = "retail-12-0-5", feature = "client-wowforever"))]

#[test]
fn original_aura_util_classifies_an_unlisted_spell() {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &ui,
        &["Blizzard_FrameXMLUtil"],
        &[],
    );
    env.exec(
        r#"
        local result = AuraUtil.IsBigDefensive({spellId = 999999})
        assert(type(result) == 'boolean', 'original consumer must return a boolean')
        assert(result == false, 'unlisted spells have no declared defensive classification')
        assert(AuraUtil.IsBigDefensive({spellId = 999999}) == false,
            'the original consumer preserves its cached false result')
        "#,
    )
    .unwrap();
}
