//! Integration tests for the panel-side `C_ArtifactUI` surface
//! registered in `src/c_api/c_artifact_ui.rs`. The action-bar subset
//! has its own tests in `c_artifact_ui_globals.rs`; this file covers
//! the LoD `Blizzard_ArtifactUI` panel methods that read from
//! `state.viewed_artifact`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{
    ArtifactAppearanceInfo, ArtifactAppearanceSetInfo, ArtifactArtInfo, ArtifactInfo,
    ArtifactPowerInfo, ColorRgb, CursorInfo, CursorItemOrigin, MetaPowerEntry, RelicSlotInfo,
};

const SAMPLE_ITEM_ID: i32 = 128_910;
const SAMPLE_TIER: i32 = 2;

fn sample_artifact() -> ArtifactInfo {
    ArtifactInfo {
        item_id: SAMPLE_ITEM_ID,
        alt_item_id: 128_911,
        name: "Ashbringer".to_string(),
        icon: "Interface/Icons/inv_sword_2h_artifactashbringer_d_01".to_string(),
        total_xp: 12_500,
        points_spent: 7,
        quality: 6,
        artifact_appearance_id: 41,
        appearance_mod_id: 0,
        item_appearance_id: 0,
        alt_item_appearance_id: 0,
        alt_on_top: false,
        tier: SAMPLE_TIER,
        maxed: false,
        disabled: false,
        category: 1,
    }
}

fn seed_artifact(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.viewed_artifact.info = Some(sample_artifact());
    state.viewed_artifact.points_remaining = 3;
    state.viewed_artifact.total_purchased_ranks = 7;
    state.viewed_artifact.num_obtained_artifacts = 4;
    state.viewed_artifact.is_at_forge = true;
    state.viewed_artifact.is_disabled = false;
    state.viewed_artifact.is_maxed_by_rules = false;
    state.viewed_artifact.is_viewed_equipped = true;
    state.viewed_artifact.respec_npc_active = true;
    state.viewed_artifact.forge_rotation = (1.0, 2.0, 3.0);
}

#[test]
fn c_artifact_ui_panel_methods_are_registered() {
    let env = WowLuaEnv::new().expect("env");
    for fn_name in [
        "GetArtifactInfo",
        "GetArtifactItemID",
        "GetArtifactTier",
        "GetArtifactArtInfo",
        "GetPointsRemaining",
        "GetTotalPurchasedRanks",
        "GetNumObtainedArtifacts",
        "IsArtifactDisabled",
        "IsAtForge",
        "IsMaxedByRulesOrEffect",
        "IsViewedArtifactEquipped",
        "CheckRespecNPC",
        "GetPowerInfo",
        "GetPowers",
        "GetPowerLinks",
        "GetMetaPowerInfo",
        "GetPowerHyperlink",
        "GetTotalPowerCost",
        "GetPowersAffectedByRelic",
        "GetPowersAffectedByRelicItemLink",
        "IsPowerKnown",
        "GetNumAppearanceSets",
        "GetAppearanceSetInfo",
        "GetAppearanceInfo",
        "GetAppearanceInfoByID",
        "GetPreviewAppearance",
        "GetNumRelicSlots",
        "GetRelicInfo",
        "GetRelicInfoByItemID",
        "GetRelicLockedReason",
        "GetRelicSlotType",
        "CanApplyCursorRelicToSlot",
        "CanApplyRelicItemIDToSlot",
        "GetForgeRotation",
        "ShouldSuppressForgeRotation",
        "SetForgeRotation",
        "AddPower",
        "Clear",
        "ConfirmRespec",
        "SetAppearance",
        "SetPreviewAppearance",
        "ApplyCursorRelicToSlot",
    ] {
        let kind: String = env
            .eval(&format!("return type(C_ArtifactUI.{fn_name})"))
            .unwrap();
        assert_eq!(kind, "function", "{fn_name} must be a Rust-bound function");
    }
}

#[test]
fn get_artifact_info_returns_nothing_when_no_artifact_viewed() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_ArtifactUI.GetArtifactInfo() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_artifact_info_returns_thirteen_values_when_viewed() {
    let env = WowLuaEnv::new().expect("env");
    seed_artifact(&env);
    env.exec(
        "id, alt, name, icon, xp, points, quality, appID, modID, itemAppID, altAppID, altTop, tier = C_ArtifactUI.GetArtifactInfo()",
    )
    .unwrap();
    let id: f64 = env.eval("return id").unwrap();
    let name: String = env.eval("return name").unwrap();
    let tier: f64 = env.eval("return tier").unwrap();
    assert!((id - SAMPLE_ITEM_ID as f64).abs() < 1e-6);
    assert_eq!(name, "Ashbringer");
    assert!((tier - SAMPLE_TIER as f64).abs() < 1e-6);
}

#[test]
fn get_artifact_item_id_returns_nothing_when_no_artifact() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_ArtifactUI.GetArtifactItemID() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_artifact_tier_returns_nil_when_no_artifact() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_ArtifactUI.GetArtifactTier() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_artifact_tier_reads_state_when_viewed() {
    let env = WowLuaEnv::new().expect("env");
    seed_artifact(&env);
    let tier: f64 = env.eval("return C_ArtifactUI.GetArtifactTier()").unwrap();
    assert!((tier - SAMPLE_TIER as f64).abs() < 1e-6);
}

#[test]
fn get_artifact_art_info_returns_nothing_when_no_artifact() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_ArtifactUI.GetArtifactArtInfo() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_artifact_art_info_returns_titled_table() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.info = Some(sample_artifact());
        st.viewed_artifact.art_info = ArtifactArtInfo {
            texture_kit: "ashbringer".to_string(),
            title_name: "The Ashbringer".to_string(),
            title_color: ColorRgb {
                r: 1.0,
                g: 0.5,
                b: 0.0,
            },
            bar_connected_color: ColorRgb {
                r: 0.0,
                g: 1.0,
                b: 0.0,
            },
            bar_disconnected_color: ColorRgb {
                r: 0.5,
                g: 0.5,
                b: 0.5,
            },
            ui_model_scene_id: 12,
            spell_visual_kit_id: 99,
        };
    }
    let title: String = env
        .eval("return C_ArtifactUI.GetArtifactArtInfo().titleName")
        .unwrap();
    let kit: String = env
        .eval("return C_ArtifactUI.GetArtifactArtInfo().textureKit")
        .unwrap();
    let scene: f64 = env
        .eval("return C_ArtifactUI.GetArtifactArtInfo().uiModelSceneID")
        .unwrap();
    assert_eq!(title, "The Ashbringer");
    assert_eq!(kit, "ashbringer");
    assert!((scene - 12.0).abs() < 1e-6);
}

#[test]
fn points_and_ranks_and_obtained_count_read_state() {
    let env = WowLuaEnv::new().expect("env");
    seed_artifact(&env);
    let points: f64 = env
        .eval("return C_ArtifactUI.GetPointsRemaining()")
        .unwrap();
    let ranks: f64 = env
        .eval("return C_ArtifactUI.GetTotalPurchasedRanks()")
        .unwrap();
    let obtained: f64 = env
        .eval("return C_ArtifactUI.GetNumObtainedArtifacts()")
        .unwrap();
    assert!((points - 3.0).abs() < 1e-6);
    assert!((ranks - 7.0).abs() < 1e-6);
    assert!((obtained - 4.0).abs() < 1e-6);
}

#[test]
fn flag_getters_read_viewed_artifact_state() {
    let env = WowLuaEnv::new().expect("env");
    seed_artifact(&env);
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.is_disabled = true;
        st.viewed_artifact.is_maxed_by_rules = true;
    }
    let disabled: bool = env
        .eval("return C_ArtifactUI.IsArtifactDisabled()")
        .unwrap();
    let at_forge: bool = env.eval("return C_ArtifactUI.IsAtForge()").unwrap();
    let maxed: bool = env
        .eval("return C_ArtifactUI.IsMaxedByRulesOrEffect()")
        .unwrap();
    let viewed: bool = env
        .eval("return C_ArtifactUI.IsViewedArtifactEquipped()")
        .unwrap();
    let respec: bool = env.eval("return C_ArtifactUI.CheckRespecNPC()").unwrap();
    assert!(disabled);
    assert!(at_forge);
    assert!(maxed);
    assert!(viewed);
    assert!(respec);
}

#[test]
fn get_power_info_returns_nothing_when_unknown_id() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_ArtifactUI.GetPowerInfo(42) == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_power_info_returns_table_for_known_id() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.powers.insert(
            42,
            ArtifactPowerInfo {
                spell_id: 12345,
                cost: 1,
                current_rank: 2,
                max_rank: 3,
                bonus_ranks: 0,
                num_max_rank_bonus_from_tier: 0,
                prereqs_met: true,
                is_start: false,
                is_gold_medal: true,
                is_final: false,
                tier: 1,
                position: (0.25, 0.75),
                offset: Some((1.0, 2.0)),
                linear_index: Some(7),
            },
        );
    }
    let spell: f64 = env
        .eval("return C_ArtifactUI.GetPowerInfo(42).spellID")
        .unwrap();
    let pos_x: f64 = env
        .eval("return C_ArtifactUI.GetPowerInfo(42).position.x")
        .unwrap();
    let linear: f64 = env
        .eval("return C_ArtifactUI.GetPowerInfo(42).linearIndex")
        .unwrap();
    assert!((spell - 12345.0).abs() < 1e-6);
    assert!((pos_x - 0.25).abs() < 1e-6);
    assert!((linear - 7.0).abs() < 1e-6);
}

#[test]
fn get_powers_returns_sorted_id_array() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.info = Some(sample_artifact());
        for id in [30, 10, 20] {
            st.viewed_artifact
                .powers
                .insert(id, ArtifactPowerInfo::default());
        }
    }
    let count: f64 = env.eval("return #C_ArtifactUI.GetPowers()").unwrap();
    let first: f64 = env.eval("return C_ArtifactUI.GetPowers()[1]").unwrap();
    let last: f64 = env.eval("return C_ArtifactUI.GetPowers()[3]").unwrap();
    assert!((count - 3.0).abs() < 1e-6);
    assert!((first - 10.0).abs() < 1e-6);
    assert!((last - 30.0).abs() < 1e-6);
}

#[test]
fn get_power_links_returns_state_neighbours() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .viewed_artifact
        .power_links
        .insert(7, vec![1, 2, 3]);
    let count: f64 = env.eval("return #C_ArtifactUI.GetPowerLinks(7)").unwrap();
    let first: f64 = env.eval("return C_ArtifactUI.GetPowerLinks(7)[1]").unwrap();
    assert!((count - 3.0).abs() < 1e-6);
    assert!((first - 1.0).abs() < 1e-6);
}

#[test]
fn get_meta_power_info_returns_stride_of_three() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().viewed_artifact.meta_powers = vec![
        MetaPowerEntry {
            spell_id: 100,
            cost: 1,
            current_rank: 0,
        },
        MetaPowerEntry {
            spell_id: 200,
            cost: 2,
            current_rank: 1,
        },
    ];
    env.exec("a, b, c, d, e, f = C_ArtifactUI.GetMetaPowerInfo()")
        .unwrap();
    let a: f64 = env.eval("return a").unwrap();
    let b: f64 = env.eval("return b").unwrap();
    let c: f64 = env.eval("return c").unwrap();
    let d: f64 = env.eval("return d").unwrap();
    let f: f64 = env.eval("return f").unwrap();
    assert!((a - 100.0).abs() < 1e-6);
    assert!((b - 1.0).abs() < 1e-6);
    assert!((c - 0.0).abs() < 1e-6);
    assert!((d - 200.0).abs() < 1e-6);
    assert!((f - 1.0).abs() < 1e-6);
}

#[test]
fn get_power_hyperlink_formats_artifactpower_link() {
    let env = WowLuaEnv::new().expect("env");
    let link: String = env
        .eval("return C_ArtifactUI.GetPowerHyperlink(123)")
        .unwrap();
    assert!(link.contains("Hartifactpower:123"));
    assert!(link.contains("[Artifact Trait]"));
}

#[test]
fn get_total_power_cost_returns_state_value_or_zero() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.info = Some(sample_artifact());
        st.viewed_artifact
            .total_power_cost_table
            .insert((1, 2, 3), 5_000);
    }
    let known: f64 = env
        .eval("return C_ArtifactUI.GetTotalPowerCost(1, 2, 3)")
        .unwrap();
    let unknown: f64 = env
        .eval("return C_ArtifactUI.GetTotalPowerCost(9, 9, 9)")
        .unwrap();
    assert!((known - 5_000.0).abs() < 1e-6);
    assert!(unknown.abs() < 1e-6);
}

#[test]
fn get_powers_affected_by_relic_returns_multireturn_ids() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .viewed_artifact
        .powers_affected_by_relic_slot
        .insert(2, vec![10, 20]);
    env.exec("a, b = C_ArtifactUI.GetPowersAffectedByRelic(2)")
        .unwrap();
    let a: f64 = env.eval("return a").unwrap();
    let b: f64 = env.eval("return b").unwrap();
    assert!((a - 10.0).abs() < 1e-6);
    assert!((b - 20.0).abs() < 1e-6);
}

#[test]
fn get_powers_affected_by_relic_item_link_returns_multireturn_ids() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .viewed_artifact
        .powers_affected_by_relic_item
        .insert("relic-link".to_string(), vec![55]);
    env.exec("v = C_ArtifactUI.GetPowersAffectedByRelicItemLink(\"relic-link\")")
        .unwrap();
    let v: f64 = env.eval("return v").unwrap();
    assert!((v - 55.0).abs() < 1e-6);
}

#[test]
fn is_power_known_reflects_known_set() {
    let env = WowLuaEnv::new().expect("env");
    let initial: bool = env.eval("return C_ArtifactUI.IsPowerKnown(7)").unwrap();
    assert!(!initial);
    env.state()
        .borrow_mut()
        .viewed_artifact
        .power_known
        .insert(7);
    let after: bool = env.eval("return C_ArtifactUI.IsPowerKnown(7)").unwrap();
    assert!(after);
}

#[test]
fn appearance_sets_count_and_info_round_trip() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .viewed_artifact
        .appearance_sets
        .push(ArtifactAppearanceSetInfo {
            set_id: 9,
            name: "Crimson".to_string(),
            description: "Crimson appearances".to_string(),
            num_appearances: 3,
        });
    let count: f64 = env
        .eval("return C_ArtifactUI.GetNumAppearanceSets()")
        .unwrap();
    env.exec("setID, name, desc, num = C_ArtifactUI.GetAppearanceSetInfo(1)")
        .unwrap();
    let set_id: f64 = env.eval("return setID").unwrap();
    let name: String = env.eval("return name").unwrap();
    let num: f64 = env.eval("return num").unwrap();
    assert!((count - 1.0).abs() < 1e-6);
    assert!((set_id - 9.0).abs() < 1e-6);
    assert_eq!(name, "Crimson");
    assert!((num - 3.0).abs() < 1e-6);
}

#[test]
fn appearance_set_info_returns_nothing_for_missing_index() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_ArtifactUI.GetAppearanceSetInfo(99) == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn appearance_info_returns_thirteen_values_when_present() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .viewed_artifact
        .appearances
        .insert((1, 1), sample_appearance());
    env.exec(
        "appID, name, _, unlocked, _, cam, _, r, g, b, op, sat, obtainable = C_ArtifactUI.GetAppearanceInfo(1, 1)",
    )
    .unwrap();
    let app_id: f64 = env.eval("return appID").unwrap();
    let name: String = env.eval("return name").unwrap();
    let unlocked: bool = env.eval("return unlocked").unwrap();
    let cam: f64 = env.eval("return cam").unwrap();
    let obtainable: bool = env.eval("return obtainable").unwrap();
    assert!((app_id - 100.0).abs() < 1e-6);
    assert_eq!(name, "Crimson Tide");
    assert!(unlocked);
    assert!((cam - 5.0).abs() < 1e-6);
    assert!(obtainable);
}

#[test]
fn appearance_info_by_id_prefixes_with_set_id() {
    let env = WowLuaEnv::new().expect("env");
    let mut appearance = sample_appearance();
    appearance.set_id = 9;
    env.state()
        .borrow_mut()
        .viewed_artifact
        .appearances_by_id
        .insert(100, appearance);
    env.exec("setID = C_ArtifactUI.GetAppearanceInfoByID(100)")
        .unwrap();
    let set_id: f64 = env.eval("return setID").unwrap();
    assert!((set_id - 9.0).abs() < 1e-6);
}

#[test]
fn get_preview_appearance_reflects_state() {
    let env = WowLuaEnv::new().expect("env");
    let initial_nil: bool = env
        .eval("return C_ArtifactUI.GetPreviewAppearance() == nil")
        .unwrap();
    assert!(initial_nil);
    env.state().borrow_mut().viewed_artifact.preview_appearance = Some(7);
    let id: f64 = env
        .eval("return C_ArtifactUI.GetPreviewAppearance()")
        .unwrap();
    assert!((id - 7.0).abs() < 1e-6);
}

#[test]
fn relic_slot_count_and_info_read_state() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.relic_slots.push(RelicSlotInfo {
            slot_type: "Iron".to_string(),
            locked_reason: None,
            name: "Iron Relic".to_string(),
            icon: "Interface/Icons/iron".to_string(),
            link: "item:111".to_string(),
        });
        st.viewed_artifact.relic_slots.push(RelicSlotInfo {
            slot_type: "Blood".to_string(),
            locked_reason: Some("Locked".to_string()),
            name: String::new(),
            icon: String::new(),
            link: String::new(),
        });
    }
    let total: f64 = env.eval("return C_ArtifactUI.GetNumRelicSlots()").unwrap();
    let unlocked_only: f64 = env
        .eval("return C_ArtifactUI.GetNumRelicSlots(true)")
        .unwrap();
    let slot_type: String = env.eval("return C_ArtifactUI.GetRelicSlotType(2)").unwrap();
    let locked_reason: String = env
        .eval("return C_ArtifactUI.GetRelicLockedReason(2)")
        .unwrap();
    assert!((total - 2.0).abs() < 1e-6);
    assert!((unlocked_only - 1.0).abs() < 1e-6);
    assert_eq!(slot_type, "Blood");
    assert_eq!(locked_reason, "Locked");
}

#[test]
fn get_relic_info_returns_nothing_for_unsocketed_slot() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .viewed_artifact
        .relic_slots
        .push(RelicSlotInfo {
            slot_type: "Iron".to_string(),
            locked_reason: None,
            name: String::new(),
            icon: String::new(),
            link: String::new(),
        });
    let nil: bool = env
        .eval("return C_ArtifactUI.GetRelicInfo(1) == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_relic_info_returns_four_values_for_socketed_slot() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .viewed_artifact
        .relic_slots
        .push(RelicSlotInfo {
            slot_type: "Iron".to_string(),
            locked_reason: None,
            name: "Hammer".to_string(),
            icon: "Interface/Icons/hammer".to_string(),
            link: "item:222".to_string(),
        });
    env.exec("name, icon, slotType, link = C_ArtifactUI.GetRelicInfo(1)")
        .unwrap();
    let name: String = env.eval("return name").unwrap();
    let slot_type: String = env.eval("return slotType").unwrap();
    let link: String = env.eval("return link").unwrap();
    assert_eq!(name, "Hammer");
    assert_eq!(slot_type, "Iron");
    assert_eq!(link, "item:222");
}

#[test]
fn get_relic_info_by_item_id_reads_state() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .viewed_artifact
        .relic_info_by_item_id
        .insert(
            555,
            RelicSlotInfo {
                slot_type: "Fel".to_string(),
                locked_reason: None,
                name: "Fel Relic".to_string(),
                icon: "Interface/Icons/fel".to_string(),
                link: "item:555".to_string(),
            },
        );
    env.exec("name = C_ArtifactUI.GetRelicInfoByItemID(555)")
        .unwrap();
    let name: String = env.eval("return name").unwrap();
    assert_eq!(name, "Fel Relic");
}

#[test]
fn can_apply_cursor_relic_to_slot_requires_known_relic_and_unlocked_slot() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.relic_slots.push(RelicSlotInfo {
            slot_type: "Iron".to_string(),
            locked_reason: None,
            name: String::new(),
            icon: String::new(),
            link: String::new(),
        });
        st.artifact_relic_items.insert(777);
        st.cursor_item = Some(CursorInfo::Item {
            item_id: 777,
            stack_count: 1,
            origin: CursorItemOrigin::Unknown,
        });
    }
    let allowed: bool = env
        .eval("return C_ArtifactUI.CanApplyCursorRelicToSlot(1)")
        .unwrap();
    assert!(allowed);
}

#[test]
fn can_apply_cursor_relic_to_slot_rejects_unknown_relic() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.relic_slots.push(RelicSlotInfo {
            slot_type: "Iron".to_string(),
            locked_reason: None,
            name: String::new(),
            icon: String::new(),
            link: String::new(),
        });
        st.cursor_item = Some(CursorInfo::Item {
            item_id: 1,
            stack_count: 1,
            origin: CursorItemOrigin::Unknown,
        });
    }
    let allowed: bool = env
        .eval("return C_ArtifactUI.CanApplyCursorRelicToSlot(1)")
        .unwrap();
    assert!(!allowed);
}

#[test]
fn can_apply_relic_item_id_to_slot_checks_lock_state() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.relic_slots.push(RelicSlotInfo {
            slot_type: "Iron".to_string(),
            locked_reason: Some("Locked".to_string()),
            name: String::new(),
            icon: String::new(),
            link: String::new(),
        });
        st.artifact_relic_items.insert(888);
    }
    let allowed: bool = env
        .eval("return C_ArtifactUI.CanApplyRelicItemIDToSlot(888, 1)")
        .unwrap();
    assert!(!allowed);
}

#[test]
fn forge_rotation_round_trips_through_state() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("C_ArtifactUI.SetForgeRotation(0.5, 1.5, 2.5)")
        .unwrap();
    let stored = env.state().borrow().viewed_artifact.forge_rotation;
    assert!((stored.0 - 0.5).abs() < 1e-6);
    env.exec("a, b, c = C_ArtifactUI.GetForgeRotation()")
        .unwrap();
    let a: f64 = env.eval("return a").unwrap();
    let c: f64 = env.eval("return c").unwrap();
    assert!((a - 0.5).abs() < 1e-6);
    assert!((c - 2.5).abs() < 1e-6);
}

#[test]
fn should_suppress_forge_rotation_reads_state() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .viewed_artifact
        .suppress_forge_rotation = true;
    let suppress: bool = env
        .eval("return C_ArtifactUI.ShouldSuppressForgeRotation()")
        .unwrap();
    assert!(suppress);
}

#[test]
fn add_power_marks_known_and_fires_artifact_update() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.info = Some(sample_artifact());
        st.viewed_artifact.points_remaining = 1;
    }
    env.exec(
        r#"
        EVENTS_SEEN = {}
        local listener = CreateFrame("Frame")
        listener:RegisterEvent("ARTIFACT_UPDATE")
        listener:SetScript("OnEvent", function(_, event, newItem)
            table.insert(EVENTS_SEEN, event .. ":" .. tostring(newItem))
        end)
        result = C_ArtifactUI.AddPower(42)
        "#,
    )
    .unwrap();
    let result: bool = env.eval("return result").unwrap();
    let first: String = env.eval("return EVENTS_SEEN[1]").unwrap();
    assert!(result);
    assert_eq!(first, "ARTIFACT_UPDATE:false");
    assert!(
        env.state()
            .borrow()
            .viewed_artifact
            .power_known
            .contains(&42)
    );
}

#[test]
fn clear_resets_state_and_fires_artifact_close_when_present() {
    let env = WowLuaEnv::new().expect("env");
    seed_artifact(&env);
    env.exec(
        r#"
        EVENTS_SEEN = {}
        local listener = CreateFrame("Frame")
        listener:RegisterEvent("ARTIFACT_CLOSE")
        listener:SetScript("OnEvent", function(_, event)
            table.insert(EVENTS_SEEN, event)
        end)
        C_ArtifactUI.Clear()
        "#,
    )
    .unwrap();
    let first: String = env.eval("return EVENTS_SEEN[1]").unwrap();
    assert_eq!(first, "ARTIFACT_CLOSE");
    assert!(env.state().borrow().viewed_artifact.info.is_none());
}

#[test]
fn clear_is_silent_when_no_artifact_present() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        EVENTS_SEEN = {}
        local listener = CreateFrame("Frame")
        listener:RegisterEvent("ARTIFACT_CLOSE")
        listener:SetScript("OnEvent", function(_, event)
            table.insert(EVENTS_SEEN, event)
        end)
        C_ArtifactUI.Clear()
        "#,
    )
    .unwrap();
    let count: f64 = env.eval("return #EVENTS_SEEN").unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn confirm_respec_refunds_points_and_clears_known_powers() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.info = Some(sample_artifact());
        st.viewed_artifact.points_remaining = 0;
        st.viewed_artifact.total_purchased_ranks = 5;
        st.viewed_artifact.power_known.insert(7);
    }
    env.exec("C_ArtifactUI.ConfirmRespec()").unwrap();
    let st = env.state().borrow();
    assert_eq!(st.viewed_artifact.points_remaining, 5);
    assert_eq!(st.viewed_artifact.total_purchased_ranks, 0);
    assert!(st.viewed_artifact.power_known.is_empty());
}

#[test]
fn set_appearance_writes_appearance_id_to_info() {
    let env = WowLuaEnv::new().expect("env");
    seed_artifact(&env);
    env.exec("C_ArtifactUI.SetAppearance(99)").unwrap();
    let id = env
        .state()
        .borrow()
        .viewed_artifact
        .info
        .as_ref()
        .map(|info| info.artifact_appearance_id)
        .unwrap_or(-1);
    assert_eq!(id, 99);
}

#[test]
fn set_preview_appearance_clears_when_passed_nil_or_zero() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().viewed_artifact.preview_appearance = Some(5);
    env.exec("C_ArtifactUI.SetPreviewAppearance(nil)").unwrap();
    assert!(
        env.state()
            .borrow()
            .viewed_artifact
            .preview_appearance
            .is_none()
    );
    env.exec("C_ArtifactUI.SetPreviewAppearance(7)").unwrap();
    assert_eq!(
        env.state().borrow().viewed_artifact.preview_appearance,
        Some(7)
    );
    env.exec("C_ArtifactUI.SetPreviewAppearance(0)").unwrap();
    assert!(
        env.state()
            .borrow()
            .viewed_artifact
            .preview_appearance
            .is_none()
    );
}

#[test]
fn apply_cursor_relic_to_slot_writes_into_slot() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut st = env.state().borrow_mut();
        st.viewed_artifact.relic_slots.push(RelicSlotInfo {
            slot_type: "Iron".to_string(),
            locked_reason: None,
            name: String::new(),
            icon: String::new(),
            link: String::new(),
        });
        st.artifact_relic_items.insert(444);
        st.cursor_item = Some(CursorInfo::Item {
            item_id: 444,
            stack_count: 1,
            origin: CursorItemOrigin::Unknown,
        });
    }
    env.exec("C_ArtifactUI.ApplyCursorRelicToSlot(1)").unwrap();
    let st = env.state().borrow();
    let slot = &st.viewed_artifact.relic_slots[0];
    assert_eq!(slot.link, "item:444");
    assert!(slot.icon.contains("inv_relic_444"));
    assert!(slot.name.contains("444"));
}

fn sample_appearance() -> ArtifactAppearanceInfo {
    ArtifactAppearanceInfo {
        set_id: 0,
        appearance_id: 100,
        name: "Crimson Tide".to_string(),
        display_index: 1,
        unlocked: true,
        failure_description: None,
        ui_camera_id: 5,
        alt_hand_camera_id: None,
        swatch_color: ColorRgb {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        },
        model_opacity: 1.0,
        model_saturation: 0.5,
        obtainable: true,
    }
}
