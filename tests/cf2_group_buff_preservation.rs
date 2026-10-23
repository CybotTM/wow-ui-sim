//! Alert payload preservation, not configured getter or producer acceptance.
#![cfg(any(feature = "client-wowforever",))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn load_original_consumer(root: &str) -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let env = crate::common::blizzard_addon_harness::new_blizzard_addon_env(&ui);
    let loaded = crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env(
        &env,
        &ui,
        &[root],
        &[],
    );
    assert!(loaded.iter().any(|name| name == root));
    eprintln!("CF-2 original consumer cache: {}", ui.display());
    env
}

fn assert_original_source(env: &WowLuaEnv, function: &str, suffix: &str) {
    env.exec(&format!(
        r#"
        local info = debug.getinfo({function}, 'S')
        local source = info.source:gsub('\\', '/')
        local suffix = '{suffix}'
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must execute the original cached Blizzard consumer')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)
        "#,
    ))
    .expect("original cached consumer provenance");
}

#[test]
fn original_group_buff_manager_reads_api_then_maps_supplied_alert_records_preserves_boundary() {
    let env = load_original_consumer("Blizzard_FrameXMLUtil");
    assert_original_source(
        &env,
        "GroupBuffMixin.RefreshGroupBuffVisualAlerts",
        "Blizzard_FrameXMLUtil/AuraUtil.lua",
    );
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec(
        r#"
        local manager = CreateFromMixins(GroupBuffMixin)
        -- Omitted argument must reach the real getter, not a replacement or
        -- an explicit payload that would bypass the missing member.
        manager:RefreshGroupBuffVisualAlerts()

        -- These records exercise original payload mapping only. They do NOT
        -- seed or establish the getter's configured data/default policy.
        manager:OnGroupBuffVisualAlertsChanged({
            {spellID = 1459, visualValue = Enum.VisualAlertType.MarchingAnts},
            {spellID = 21562, visualValue = Enum.VisualAlertType.FlashRed},
        })
        -- Concrete values from cached VisualAlertConstantsDocumentation.lua;
        -- nil/missing enum members must not make nil == nil look successful.
        assert(manager:GetGroupBuffVisualAlert(1459) == 1, 'MarchingAnts value')
        assert(manager:GetGroupBuffVisualAlert(21562) == 8, 'FlashRed value')
        manager:OnGroupBuffVisualAlertsChanged({
            {spellID = 1459, visualValue = Enum.VisualAlertType.FlashBlue},
        })
        assert(manager:GetGroupBuffVisualAlert(1459) == 10, 'FlashBlue replacement value')
        assert(manager:GetGroupBuffVisualAlert(21562) == nil, 'replace, not merge, alert records')
        "#,
    )
    .expect("original implicit getter and populated alert payload consumer must not error");
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}
