//! Integration tests for the `C_AnimaDiversion` namespace.
//!
//! Drives `Blizzard_AnimaDiversionUI/Blizzard_AnimaDiversionUI.lua`
//! (`AnimaDiversionFrameMixin:OnEvent`, `:UpdateBolsterProgress`,
//! `:TryShow`, `:SetExclusiveSelectionNode`,
//! `AnimaDiversionUtil.IsNodeActive`) and the data-provider mixin in
//! `AnimaDiversionDataProvider.lua` (`AnimaDiversionPinMixin:Init`).
//! Real WoW resolves the surface to seven probes documented in
//! `vendor/wow-ui-source/Interface/AddOns/Blizzard_APIDocumentationGenerated/AnimaDiversionUIDocumentation.lua`;
//! the simulator backs them with `SimState.anima_diversion`.

use wow_ui_sim::event::EventArg;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{AnimaDiversionCostInfo, AnimaDiversionNodeInfo};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn seed_two_node_diversion(env: &WowLuaEnv) {
    let mut sim = env.state().borrow_mut();
    sim.anima_diversion.texture_kit = "kyrian".to_string();
    sim.anima_diversion.title = "Forge of Bolstering".to_string();
    sim.anima_diversion.map_id = 1543;
    sim.anima_diversion.origin_position = Some((0.42, 0.61));
    sim.anima_diversion.reinforce_progress = 0.75;
    sim.anima_diversion.nodes = vec![bastion_ward_node(), ascendant_echo_node()];
}

fn bastion_ward_node() -> AnimaDiversionNodeInfo {
    AnimaDiversionNodeInfo {
        talent_id: 101,
        name: "Bastion Ward".to_string(),
        description: "Increase Bastion's resilience.".to_string(),
        costs: vec![cost(1813, 250)],
        currency_id: 1813,
        icon: 3528287,
        normalized_position_x: 0.25,
        normalized_position_y: 0.5,
        state: 1, // Available
    }
}

fn ascendant_echo_node() -> AnimaDiversionNodeInfo {
    AnimaDiversionNodeInfo {
        talent_id: 102,
        name: "Ascendant Echo".to_string(),
        description: "Permanent power for the Ascended.".to_string(),
        costs: vec![cost(1813, 500), cost(1822, 50)],
        currency_id: 1813,
        icon: 3528288,
        normalized_position_x: 0.75,
        normalized_position_y: 0.5,
        state: 3, // SelectedPermanent
    }
}

fn cost(currency_id: i64, quantity: i64) -> AnimaDiversionCostInfo {
    AnimaDiversionCostInfo {
        currency_id,
        quantity,
    }
}

#[test]
fn namespace_exposes_expected_methods() {
    let env = env();
    let kinds: String = env
        .eval(
            r#"
            local fns = {
                "GetAnimaDiversionNodes",
                "GetOriginPosition",
                "GetReinforceProgress",
                "GetTextureKit",
                "OpenAnimaDiversionUI",
                "SelectAnimaNode",
                "CloseUI",
            }
            for _, name in ipairs(fns) do
                if type(C_AnimaDiversion[name]) ~= "function" then
                    return "missing_" .. name
                end
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        kinds, "ok",
        "C_AnimaDiversion must expose all seven documented probes"
    );
}

#[test]
fn get_texture_kit_returns_seeded_kit() {
    let env = env();
    seed_two_node_diversion(&env);
    let kit: String = env.eval("return C_AnimaDiversion.GetTextureKit()").unwrap();
    assert_eq!(
        kit, "kyrian",
        "GetTextureKit must surface SimState.anima_diversion.texture_kit"
    );
}

#[test]
fn get_reinforce_progress_returns_seeded_fill() {
    let env = env();
    seed_two_node_diversion(&env);
    let progress: f64 = env
        .eval("return C_AnimaDiversion.GetReinforceProgress()")
        .unwrap();
    assert!(
        (progress - 0.75).abs() < f64::EPSILON,
        "GetReinforceProgress must surface SimState fill fraction; got {progress}"
    );
}

#[test]
fn get_origin_position_returns_vector2d_table() {
    let env = env();
    seed_two_node_diversion(&env);
    let x: f64 = env
        .eval("return C_AnimaDiversion.GetOriginPosition().x")
        .unwrap();
    let y: f64 = env
        .eval("return C_AnimaDiversion.GetOriginPosition().y")
        .unwrap();
    assert!(
        (x - 0.42).abs() < f64::EPSILON && (y - 0.61).abs() < f64::EPSILON,
        "GetOriginPosition must return a Vector2DMixin-shaped table; got ({x}, {y})"
    );
}

#[test]
fn get_origin_position_returns_nil_when_unset() {
    let env = env();
    let kind: String = env
        .eval("return type(C_AnimaDiversion.GetOriginPosition())")
        .unwrap();
    assert_eq!(
        kind, "nil",
        "GetOriginPosition is documented as Nilable; default state must return nil"
    );
}

#[test]
fn get_anima_diversion_nodes_returns_full_node_shape() {
    let env = env();
    seed_two_node_diversion(&env);
    let count: f64 = env
        .eval("return #C_AnimaDiversion.GetAnimaDiversionNodes()")
        .unwrap();
    assert_eq!(count as i64, 2, "two seeded nodes must round-trip");

    let first_name: String = env
        .eval("return C_AnimaDiversion.GetAnimaDiversionNodes()[1].name")
        .unwrap();
    assert_eq!(first_name, "Bastion Ward");

    let first_state: f64 = env
        .eval("return C_AnimaDiversion.GetAnimaDiversionNodes()[1].state")
        .unwrap();
    assert_eq!(
        first_state as i64, 1,
        "node state must mirror Enum.AnimaDiversionNodeState"
    );

    let normalized_x: f64 = env
        .eval("return C_AnimaDiversion.GetAnimaDiversionNodes()[1].normalizedPosition.x")
        .unwrap();
    assert!(
        (normalized_x - 0.25).abs() < f64::EPSILON,
        "normalizedPosition must be a Vector2DMixin sub-table; got x={normalized_x}"
    );

    let cost_count: f64 = env
        .eval("return #C_AnimaDiversion.GetAnimaDiversionNodes()[2].costs")
        .unwrap();
    assert_eq!(
        cost_count as i64, 2,
        "second node carries two cost rows; AnimaDiversionCostInfo array must round-trip"
    );

    let second_cost_quantity: f64 = env
        .eval("return C_AnimaDiversion.GetAnimaDiversionNodes()[2].costs[2].quantity")
        .unwrap();
    assert_eq!(
        second_cost_quantity as i64, 50,
        "AnimaDiversionCostInfo.quantity must round-trip per row"
    );
}

#[test]
fn open_anima_diversion_ui_fires_anima_diversion_open_with_frame_info() {
    let env = env();
    seed_two_node_diversion(&env);
    env.exec("C_AnimaDiversion.OpenAnimaDiversionUI()").unwrap();

    let events = env.state().borrow_mut().events.drain();
    assert_eq!(
        events.len(),
        1,
        "OpenAnimaDiversionUI must fire exactly one ANIMA_DIVERSION_OPEN event"
    );
    let event = &events[0];
    assert_eq!(event.name, "ANIMA_DIVERSION_OPEN");
    assert_eq!(
        event.args.len(),
        3,
        "AnimaDiversionFrameInfo carries textureKit, title, mapID — three positional args"
    );
    assert!(
        matches!(event.args[0], EventArg::String(ref kit) if kit == "kyrian"),
        "first arg is the texture kit"
    );
    assert!(
        matches!(event.args[1], EventArg::String(ref title) if title == "Forge of Bolstering"),
        "second arg is the dialog title"
    );
    assert!(
        matches!(event.args[2], EventArg::Number(map_id) if (map_id - 1543.0).abs() < f64::EPSILON),
        "third arg is the map id"
    );
}

#[test]
fn close_ui_fires_anima_diversion_close_event() {
    let env = env();
    env.exec("C_AnimaDiversion.CloseUI()").unwrap();
    let events = env.state().borrow_mut().events.drain();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].name, "ANIMA_DIVERSION_CLOSE");
    assert!(
        events[0].args.is_empty(),
        "ANIMA_DIVERSION_CLOSE has no payload per the official doc"
    );
}

#[test]
fn select_anima_node_records_request_and_fires_talent_updated() {
    let env = env();
    env.exec("C_AnimaDiversion.SelectAnimaNode(102, true)")
        .unwrap();

    let sim = env.state().borrow();
    assert_eq!(
        sim.anima_diversion.last_selected_talent_id,
        Some(102),
        "SelectAnimaNode must record the talent ID for round-trip inspection"
    );
    assert_eq!(
        sim.anima_diversion.last_selected_temporary,
        Some(true),
        "SelectAnimaNode must record the temporary flag"
    );
    drop(sim);

    let events = env.state().borrow_mut().events.drain();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].name, "ANIMA_DIVERSION_TALENT_UPDATED");
    assert!(matches!(events[0].args[0], EventArg::Number(id) if (id - 102.0).abs() < f64::EPSILON),);
    assert!(matches!(events[0].args[1], EventArg::Boolean(true)));
}

#[test]
fn select_anima_node_defaults_temporary_to_false_when_omitted() {
    let env = env();
    env.exec("C_AnimaDiversion.SelectAnimaNode(101)").unwrap();
    let sim = env.state().borrow();
    assert_eq!(sim.anima_diversion.last_selected_talent_id, Some(101));
    assert_eq!(
        sim.anima_diversion.last_selected_temporary,
        Some(false),
        "missing temporary arg must default to false (Lua nil → bool false)"
    );
}
