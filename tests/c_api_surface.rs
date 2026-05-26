use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("failed to create Lua environment")
}

#[test]
fn c_api_reorg_keeps_core_namespaces_registered() {
    let env = env();
    let namespaces: (
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            return type(C_AddOns) == "table",
                   type(C_Texture) == "table",
                   type(C_XMLUtil) == "table",
                   type(C_Item) == "table",
                   type(C_CurrencyInfo) == "table",
                   type(C_Container) == "table",
                   type(C_ItemUpgrade) == "table",
                   type(C_Spell) == "table",
                   type(C_SpellBook) == "table",
                   type(C_ModelInfo) == "table",
                   type(C_LFGInfo) == "table",
                   type(C_WowTokenSecure) == "table",
                   type(C_FogOfWar) == "table"
        "#,
        )
        .expect("failed to probe C_* namespace registration");

    assert!(namespaces.0, "C_AddOns should stay registered");
    assert!(namespaces.1, "C_Texture should stay registered");
    assert!(namespaces.2, "C_XMLUtil should stay registered");
    assert!(namespaces.3, "C_Item should stay registered");
    assert!(namespaces.4, "C_CurrencyInfo should stay registered");
    assert!(namespaces.5, "C_Container should stay registered");
    assert!(namespaces.6, "C_ItemUpgrade should stay registered");
    assert!(namespaces.7, "C_Spell should stay registered");
    assert!(namespaces.8, "C_SpellBook should stay registered");
    assert!(namespaces.9, "C_ModelInfo should stay registered");
    assert!(namespaces.10, "C_LFGInfo should stay registered");
    assert!(namespaces.11, "C_WowTokenSecure should stay registered");
    assert!(namespaces.12, "C_FogOfWar should stay registered");
}

#[test]
fn c_fog_of_war_unknown_id_keeps_default_shape() {
    let env = env();
    let info: (Option<String>, Option<String>, f64) = env
        .eval(
            r#"
            local info = C_FogOfWar.GetFogOfWarInfo(-1)
            return info.backgroundAtlas, info.maskAtlas, info.maskScalar
        "#,
        )
        .expect("failed to query C_FogOfWar");

    assert_eq!(info.0, None);
    assert_eq!(info.1, None);
    assert_eq!(info.2, 1.0);
}

#[test]
fn c_addons_scripts_disallowed_for_beta_defaults_false() {
    let env = env();
    let result: (String, bool) = env
        .eval(
            r#"
            return type(C_AddOns.GetScriptsDisallowedForBeta),
                   C_AddOns.GetScriptsDisallowedForBeta()
        "#,
        )
        .expect("failed to query C_AddOns.GetScriptsDisallowedForBeta");

    assert_eq!(result, ("function".to_string(), false));
}

#[test]
fn configuration_warnings_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_configuration_warnings"),
        "unmodeled C_ConfigurationWarnings defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_configuration_warnings"),
        "C_ConfigurationWarnings should not be wired through c_api registration"
    );
}

#[test]
fn item_targeting_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let item_spell = include_str!("../src/c_api/item_spell/mod.rs");

    assert!(
        !temporary_shims.contains("c_item_targeting"),
        "unmodeled C_Item targeting defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !item_spell.contains("c_item_targeting"),
        "C_Item IsHelpfulItem/IsHarmfulItem defaults should not be wired through c_api item registration"
    );
}

#[test]
fn spell_target_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_spell_target"),
        "unmodeled C_Spell target-spell metadata defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_spell_target"),
        "C_Spell TargetSpell* defaults should not be wired through c_api registration"
    );
}

#[test]
fn merchant_and_raid_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_merchant_raid_defaults"),
        "unmodeled C_MerchantFrame and C_RaidLocks defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_merchant_raid_defaults"),
        "C_MerchantFrame and C_RaidLocks no-state defaults should not be wired through c_api registration"
    );
}

#[test]
fn paper_doll_stagger_default_is_not_c_api_temporary_shim() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_paper_doll_stagger"),
        "unmodeled PaperDoll stagger defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_paper_doll_stagger"),
        "C_PaperDollInfo.GetStaggerPercentage should not be wired through c_api registration"
    );
}

#[test]
fn chat_info_no_state_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_chat_info"),
        "unmodeled C_ChatInfo emote/caution/chat-line defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_chat_info"),
        "C_ChatInfo no-state defaults should not be wired through c_api registration"
    );
}

#[test]
fn container_no_state_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let item_spell = include_str!("../src/c_api/item_spell/mod.rs");

    assert!(
        !temporary_shims.contains("c_container_defaults"),
        "unmodeled C_Container purchase/quest/filter/action defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !item_spell.contains("c_container_defaults"),
        "C_Container no-state defaults should not be wired through c_api item/spell registration"
    );
}

#[test]
fn pet_battle_static_fallbacks_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_pet_battles_static_fallbacks"),
        "unmodeled C_PetBattles journal/trap/select defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_pet_battles_static_fallbacks"),
        "C_PetBattles static fallbacks should not be wired through c_api registration"
    );
}

#[test]
fn party_info_instance_abandon_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_party_info_instance_abandon"),
        "unmodeled C_PartyInfo instance-abandon vote defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_party_info_instance_abandon"),
        "C_PartyInfo instance-abandon defaults should not be wired through c_api registration"
    );
}

#[test]
fn transmog_sets_empty_inventory_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_transmog_sets"),
        "unmodeled C_TransmogSets empty wardrobe-set defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_transmog_sets"),
        "C_TransmogSets empty inventory defaults should not be wired through c_api registration"
    );
}

#[test]
fn level_link_spell_lock_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_level_link_spell_lock"),
        "temporary C_LevelLink spell-lock state belongs in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_level_link_spell_lock"),
        "C_LevelLink spell-lock defaults should not be wired through c_api registration"
    );
}

#[test]
fn campaign_covenant_placeholder_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_campaign_covenant_defaults"),
        "unmodeled C_CampaignInfo and C_CovenantSanctumUI placeholder defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_campaign_covenant_defaults"),
        "campaign/covenant placeholder defaults should not be wired through c_api registration"
    );
}

#[test]
fn date_and_time_deterministic_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_date_and_time"),
        "unmodeled C_DateAndTime deterministic calendar defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_date_and_time"),
        "C_DateAndTime deterministic defaults should not be wired through c_api registration"
    );
}

#[test]
fn contribution_collector_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_contribution_collector"),
        "unmodeled C_ContributionCollector empty/default shapes belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_contribution_collector"),
        "C_ContributionCollector empty/default shapes should not be wired through c_api registration"
    );
}

#[test]
fn prototype_dialog_temporary_state_is_not_c_api_temporary_shim() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_prototype_dialog"),
        "temporary C_PrototypeDialog active/removed transition state belongs in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_prototype_dialog"),
        "C_PrototypeDialog temporary state should not be wired through c_api registration"
    );
}

#[test]
fn pvp_talent_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_specialization_pvp_talents"),
        "unmodeled PvP talent placeholder defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_specialization_pvp_talents"),
        "PvP talent placeholder defaults should not be wired through c_api registration"
    );
}

#[test]
fn spell_metadata_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    for module in [
        "c_spell_classification",
        "c_spell_counts",
        "c_spell_priority_aura",
    ] {
        assert!(
            !temporary_shims.contains(module),
            "unmodeled C_Spell metadata/count defaults belong in lua_api::workarounds::temporary"
        );
        assert!(
            !registration.contains(module),
            "C_Spell metadata/count defaults should not be wired through c_api registration"
        );
    }
}

#[test]
fn minimap_tracking_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_minimap"),
        "unmodeled C_Minimap tracking/radius defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_minimap"),
        "C_Minimap tracking/radius defaults should not be wired through c_api registration"
    );
}

#[test]
fn super_track_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_super_track"),
        "unmodeled C_SuperTrack no-active-target defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_super_track"),
        "C_SuperTrack no-active-target defaults should not be wired through c_api registration"
    );
}

#[test]
fn trade_info_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_trade_info"),
        "unmodeled C_TradeInfo warning/no-op defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_trade_info"),
        "C_TradeInfo warning/no-op defaults should not be wired through c_api registration"
    );
}

#[test]
fn scenario_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_scenario"),
        "unmodeled C_Scenario not-in-scenario defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_scenario"),
        "C_Scenario not-in-scenario defaults should not be wired through c_api registration"
    );
}

#[test]
fn gossip_poi_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_gossip_info"),
        "unmodeled C_GossipInfo POI lookup defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_gossip_info"),
        "C_GossipInfo POI lookup defaults should not be wired through c_api registration"
    );
}

#[test]
fn mythic_plus_cache_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_mythic_plus"),
        "unmodeled C_MythicPlus weekly-chest/request defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_mythic_plus"),
        "C_MythicPlus weekly-chest/request defaults should not be wired through c_api registration"
    );
}

#[test]
fn shared_character_services_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_shared_character_services"),
        "unmodeled C_SharedCharacterServices upgrade-distribution defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_shared_character_services"),
        "C_SharedCharacterServices upgrade-distribution defaults should not be wired through c_api registration"
    );
}

#[test]
fn specialization_mastery_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_specialization_mastery"),
        "unmodeled C_SpecializationInfo mastery-spell defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_specialization_mastery"),
        "C_SpecializationInfo mastery-spell defaults should not be wired through c_api registration"
    );
}

#[test]
fn click_bindings_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api_registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_click_bindings"),
        "unmodeled C_ClickBindings profile defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api_registration.contains("c_click_bindings"),
        "C_ClickBindings profile defaults should not be wired through c_api registration"
    );
}

#[test]
fn spell_book_static_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let item_spell = include_str!("../src/c_api/item_spell/mod.rs");

    assert!(
        !temporary_shims.contains("c_spell_book_call_pet"),
        "unmodeled C_SpellBook call-pet and deprecated spellbook defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !item_spell.contains("c_spell_book_call_pet"),
        "C_SpellBook static defaults should not be wired through c_api item/spell registration"
    );
}

#[test]
fn spell_static_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let item_spell = include_str!("../src/c_api/item_spell/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_spell_static_fallbacks"),
        "unmodeled C_Spell charges/override/visibility/Maw defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !item_spell.contains("c_spell_static_fallbacks"),
        "C_Spell static defaults should not be wired through item/spell C API registration"
    );
    assert!(
        !registration.contains("c_spell_static_fallbacks"),
        "C_Spell static defaults should not be wired through shared C API registration"
    );
}

#[test]
fn map_group_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_map_groups"),
        "unmodeled C_Map group defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_map_groups"),
        "C_Map group defaults should not be wired through C API registration"
    );
}

#[test]
fn perks_program_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let c_api = include_str!("../src/c_api/mod.rs");

    assert!(
        !temporary_shims.contains("c_perks_program"),
        "unmodeled C_PerksProgram empty-catalog defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !c_api.contains("c_perks_program"),
        "C_PerksProgram empty-catalog defaults should not be wired through c_api registration"
    );
}

#[test]
fn party_info_static_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_party_info_static_fallbacks"),
        "unmodeled C_PartyInfo invite/Torghast/walk-in defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_party_info_static_fallbacks"),
        "C_PartyInfo static defaults should not be wired through C API registration"
    );
}

#[test]
fn character_services_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_character_services"),
        "unmodeled C_CharacterServices service/display/assignment defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_character_services"),
        "C_CharacterServices service/display/assignment defaults should not be wired through C API registration"
    );
}

#[test]
fn major_faction_display_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_major_faction_display"),
        "unmodeled C_MajorFactions display-policy defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_major_faction_display"),
        "C_MajorFactions display-policy defaults should not be wired through C API registration"
    );
}
