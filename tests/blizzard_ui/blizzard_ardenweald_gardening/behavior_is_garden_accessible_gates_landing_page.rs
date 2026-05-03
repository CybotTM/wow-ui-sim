//! Garrison landing-page gate for `C_ArdenwealdGardening.IsGardenAccessible`.

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const NATURAL_CALLER: &str = "Blizzard_GarrisonUI";
const ROOT: &str = "Blizzard_ArdenwealdGardening";

#[test]
fn garden_accessibility_gates_landing_page_panel_load() {
    with_blizzard_addon_smoke_shape(&[NATURAL_CALLER], &[], |env, loaded| {
        assert!(
            loaded.iter().any(|name| name == NATURAL_CALLER),
            "`{NATURAL_CALLER}` must load before probing its garden section"
        );
        assert!(
            !loaded.iter().any(|name| name == ROOT),
            "`{ROOT}` must not be preloaded before the landing-page gate runs"
        );

        seed_garden_accessibility(env, false);
        let inaccessible = run_landing_page_garden_probe(env);
        assert_inaccessible_probe(inaccessible);

        seed_garden_accessibility(env, true);
        let accessible = run_landing_page_garden_probe(env);
        assert_accessible_probe(accessible);
    });
}

type GardenGateProbe = (f64, String, bool, bool, bool, bool);

fn seed_garden_accessibility(env: &WowLuaEnv, accessible: bool) {
    env.state().borrow_mut().gardenweald.accessible = accessible;
}

fn run_landing_page_garden_probe(env: &WowLuaEnv) -> GardenGateProbe {
    env.eval(
        r#"
        local loadCalls = {}
        local originalLoadAddOn = UIParentLoadAddOn

        UIParentLoadAddOn = function(name)
            loadCalls[#loadCalls + 1] = name
            return originalLoadAddOn(name)
        end

        GarrisonLandingPage:SetupGardenweald()
        UIParentLoadAddOn = originalLoadAddOn

        local panel = GarrisonLandingPage.ArdenwealdGardeningPanel
        return #loadCalls,
               loadCalls[1] or "",
               panel ~= nil,
               panel and panel:GetParent() == GarrisonLandingPage.Report.Sections or false,
               panel and panel:IsShown() or false,
               type(ArdenwealdGardening) == "table"
        "#,
    )
    .expect("Garrison landing-page garden gate probe must run cleanly")
}

fn assert_inaccessible_probe(probe: GardenGateProbe) {
    let (
        load_count,
        loaded_name,
        panel_exists,
        panel_parent_matches,
        panel_shown,
        namespace_loaded,
    ) = probe;

    assert_eq!(
        load_count, 0.0,
        "inaccessible garden must not call UIParentLoadAddOn"
    );
    assert_eq!(
        loaded_name, "",
        "inaccessible garden must not request an addon"
    );
    assert!(
        !panel_exists,
        "inaccessible garden must not instantiate ArdenwealdGardeningPanel"
    );
    assert!(
        !panel_parent_matches,
        "inaccessible garden must not attach a garden panel to Report.Sections"
    );
    assert!(!panel_shown, "inaccessible garden must not show a panel");
    assert!(
        !namespace_loaded,
        "inaccessible garden must not load the ArdenwealdGardening namespace"
    );
}

fn assert_accessible_probe(probe: GardenGateProbe) {
    let (
        load_count,
        loaded_name,
        panel_exists,
        panel_parent_matches,
        panel_shown,
        namespace_loaded,
    ) = probe;

    assert_eq!(
        load_count, 1.0,
        "accessible garden must request the Ardenweald Gardening addon exactly once"
    );
    assert_eq!(
        loaded_name, ROOT,
        "accessible garden must load Blizzard_ArdenwealdGardening"
    );
    assert!(
        namespace_loaded,
        "accessible garden must load the namespace"
    );
    assert!(panel_exists, "accessible garden must instantiate the panel");
    assert!(
        panel_parent_matches,
        "created garden panel must be attached to GarrisonLandingPage.Report.Sections"
    );
    assert!(panel_shown, "created garden panel must be shown");
}
