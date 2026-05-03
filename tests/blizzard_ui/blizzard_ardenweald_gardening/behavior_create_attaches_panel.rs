//! Factory behavior for `Blizzard_ArdenwealdGardening`.

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArdenwealdGardening";

#[test]
fn create_returns_independent_panels_with_default_garden_state() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        assert_create_returns_independent_panels(env);
    });
}

#[test]
fn create_returns_independent_panels_with_seeded_garden_state() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_garden_state(env);
        assert_create_returns_independent_panels(env);
    });
}

type CreateProbe = (String, bool, String, bool, bool, bool, bool, bool);

fn seed_garden_state(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.gardenweald.accessible = true;
    state.gardenweald.active = 2;
    state.gardenweald.ready = 1;
    state.gardenweald.remaining_seconds = 600;
}

fn assert_create_returns_independent_panels(env: &WowLuaEnv) {
    let probe: CreateProbe = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "ArdenwealdGardeningFactoryParent", UIParent)
            local panel1 = ArdenwealdGardening.Create(parent)
            local panel2 = ArdenwealdGardening.Create(parent)
            panel1:Hide()

            return panel1:GetObjectType(),
                   panel1:GetParent() == parent,
                   panel2:GetObjectType(),
                   panel2:GetParent() == parent,
                   panel1 ~= panel2,
                   panel1.Background ~= panel2.Background,
                   panel1.Label ~= panel2.Label,
                   panel2:IsShown()
            "#,
        )
        .expect("Ardenweald Gardening factory probe must run cleanly");

    assert_create_probe(probe);
}

fn assert_create_probe(probe: CreateProbe) {
    let (
        first_panel_type,
        first_parent_matches,
        second_panel_type,
        second_parent_matches,
        panels_are_distinct,
        backgrounds_are_distinct,
        labels_are_distinct,
        second_panel_still_shown,
    ) = probe;

    assert_panel_is_attached(first_panel_type, first_parent_matches);
    assert_panel_is_attached(second_panel_type, second_parent_matches);
    assert_independent_panels(
        panels_are_distinct,
        backgrounds_are_distinct,
        labels_are_distinct,
        second_panel_still_shown,
    );
}

fn assert_panel_is_attached(panel_type: String, parent_matches: bool) {
    assert_eq!(
        panel_type, "Frame",
        "`ArdenwealdGardening.Create(parent)` must return a Frame"
    );
    assert!(
        parent_matches,
        "`ArdenwealdGardening.Create(parent)` must parent the panel to the supplied frame"
    );
}

fn assert_independent_panels(
    panels_are_distinct: bool,
    backgrounds_are_distinct: bool,
    labels_are_distinct: bool,
    second_panel_still_shown: bool,
) {
    assert!(
        panels_are_distinct,
        "`ArdenwealdGardening.Create` must return a fresh panel for each call"
    );
    assert!(
        backgrounds_are_distinct,
        "fresh panels must not share `Background` children"
    );
    assert!(
        labels_are_distinct,
        "fresh panels must not share `Label` children"
    );
    assert!(
        second_panel_still_shown,
        "hiding the first panel must not hide the second panel"
    );
}
