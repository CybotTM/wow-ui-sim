//! Filter settings preservation, not secure matching or producer acceptance.
#![cfg(any(feature = "client-retail", feature = "client-ptr",))]

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
fn original_combat_processor_applies_selected_event_settings_preserves_boundary() {
    let env = load_original_consumer("Blizzard_CombatLogProcessor");
    assert_original_source(
        &env,
        "CombatLogProcessor.ApplyFilterSettings",
        "Blizzard_CombatLogProcessor/Blizzard_CombatLogProcessor.lua",
    );
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec(
        r#"
        local processor = CreateFromMixins(CombatLogProcessor)
        local selected = {
            filters = {{eventList = {SPELL_DAMAGE = true, SPELL_HEAL = false}}},
            settings = {},
        }
        processor:SetFilterSettings(selected)
        assert(processor:GetFilterSettings() == selected, 'retain supplied settings')
        assert(processor:GetFilterSettings().filters[1].eventList.SPELL_DAMAGE == true)
        assert(processor:GetFilterSettings().filters[1].eventList.SPELL_HEAL == false)

        local replacement = {
            filters = {{eventList = {SPELL_DAMAGE = false, SPELL_HEAL = true}}},
            settings = {},
        }
        processor:SetFilterSettings(replacement)
        assert(processor:GetFilterSettings() == replacement, 'replace previous settings')
        assert(processor:GetFilterSettings().filters[1].eventList.SPELL_DAMAGE == false)
        assert(processor:GetFilterSettings().filters[1].eventList.SPELL_HEAL == true)
        "#,
    )
    .expect("original filter setup must traverse secure AddEventFilter without error");
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
    // Not event inclusion/exclusion proof: no supported ingest/filter model exists.
}
