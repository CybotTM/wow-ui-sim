//! Original cached CF-2 consumers. Filter matching and configured alert reads
//! remain PENDING-NATIVE-ORACLE; boundary tests are not proof of those behaviors.

use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::AuraInfo;

fn load_original_consumer(root: &str, applicable: &[ClientProfile]) -> Option<WowLuaEnv> {
    if !applicable.contains(&ACTIVE) {
        eprintln!("CF-2 {root} consumer is not applicable to {ACTIVE:?}; not coverage proof");
        return None;
    }
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
    Some(env)
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
fn original_combat_processor_applies_selected_event_settings() {
    let Some(env) = load_original_consumer(
        "Blizzard_CombatLogProcessor",
        &[ClientProfile::Retail, ClientProfile::Ptr],
    ) else {
        return;
    };
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

fn fixture_aura(id: i32) -> AuraInfo {
    AuraInfo {
        name: format!("CF-2 blocked aura {id}"),
        spell_id: 99000 + id,
        icon: 134973,
        duration: 30.0,
        expiration_time: 45.0,
        applications: 1,
        source_unit: "player".into(),
        is_helpful: true,
        is_raid: false,
        is_nameplate_only: false,
        is_stealable: false,
        can_apply_aura: true,
        is_from_player_or_player_pet: true,
        dispel_type: None,
        aura_instance_id: id,
    }
}

fn seed_blockable_auras(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.party_group_active = true;
    state.player.buffs = vec![fixture_aura(101)];
    let party = state
        .party_members
        .first_mut()
        .expect("seeded party member");
    party.buffs = vec![fixture_aura(201), fixture_aura(202)];
    party.debuffs.clear();
}

#[test]
fn original_arena_cleanup_restores_selected_unit_aura_enumeration() {
    let Some(env) = load_original_consumer(
        "Blizzard_UnitFrame",
        &[ClientProfile::Retail, ClientProfile::Ptr],
    ) else {
        return;
    };
    seed_blockable_auras(&env);
    assert_original_source(
        &env,
        "ArenaUnitFrameDebuffMixin.Update",
        "Blizzard_UnitFrame/Mainline/CompactArenaFrame.lua",
    );
    assert_original_source(
        &env,
        "BasePrivateAuraBehaviorMixin.UpdatePrivateAuras",
        "Blizzard_UnitFrame/Shared/CompactUnitFrame.lua",
    );
    env.exec(
        r#"
        -- All behavior comes from original mixins; real widgets hold output.
        -- BasePrivateAuraBehaviorMixin is Blizzard's own no-anchor behavior.
        Cf2AuraParent = CreateFrame('Frame', nil, UIParent)
        Mixin(Cf2AuraParent, BasePrivateAuraBehaviorMixin)
        Cf2AuraDebuff = CreateFrame('Frame', nil, Cf2AuraParent, 'ArenaUnitFrameDebuffTemplate')
        Cf2AuraDebuff.unitToken = 'party1'
        assert(C_LossOfControl.GetActiveLossOfControlDataCountByUnit('party1') == 0,
            'fixture must use no-loss-of-control branch')
        C_UnitAuras.AddBlockedAura('player', 101)
        C_UnitAuras.AddBlockedAura('party1', 201)
        C_UnitAuras.AddBlockedAura('party1', 202)
        assert(C_UnitAuras.GetAuraDataByIndex('party1', 1, 'HELPFUL') == nil)
        assert(C_UnitAuras.GetAuraDataByIndex('player', 1, 'HELPFUL') == nil)
        Cf2AuraDebuff:Show()
        "#,
    )
    .expect("real arena output fixture and existing public blocked-aura seeding");
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec(
        r#"
        for pass = 1, 3 do
            if pass == 3 then
                C_UnitAuras.AddBlockedAura('party1', 201)
                C_UnitAuras.AddBlockedAura('party1', 202)
                assert(C_UnitAuras.GetAuraDataByIndex('party1', 1, 'HELPFUL') == nil)
            end
            Cf2AuraDebuff:Update()
            assert(Cf2AuraDebuff.auraData == nil, 'no active loss-of-control aura')
            assert(not Cf2AuraDebuff:IsShown(), 'original cleanup hides stale debuff widget')
            assert(C_UnitAuras.GetAuraDataByIndex('party1', 1, 'HELPFUL').auraInstanceID == 201)
            assert(C_UnitAuras.GetAuraDataByIndex('party1', 2, 'HELPFUL').auraInstanceID == 202)
            assert(C_UnitAuras.GetAuraDataByIndex('party1', 3, 'HELPFUL') == nil)
            assert(C_UnitAuras.GetAuraDataByIndex('player', 1, 'HELPFUL') == nil,
                'clearing party1 must preserve another unit block')
        end
        "#,
    )
    .expect("original arena cleanup must clear selected unit blocks without error");
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
    let state = env.state().borrow();
    assert_eq!(state.player.buffs[0].aura_instance_id, 101);
    assert_eq!(state.party_members[0].buffs.len(), 2);
    assert_eq!(state.party_members[0].buffs[0].aura_instance_id, 201);
    assert_eq!(state.party_members[0].buffs[1].aura_instance_id, 202);
}

#[test]
fn original_group_buff_manager_reads_api_then_maps_supplied_alert_records() {
    let Some(env) = load_original_consumer("Blizzard_FrameXMLUtil", &[ClientProfile::WowForever])
    else {
        return;
    };
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
