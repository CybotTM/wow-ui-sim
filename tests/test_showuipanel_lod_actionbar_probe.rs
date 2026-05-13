//! Investigation probes for the `ActionButtonUtil` fixture decision.
//!
//! The shared LoD panel fixture publishes a narrow `ActionButtonUtil`
//! stub instead of loading the real `Blizzard_ActionBar` addon. These
//! probes capture (a) that the real addon *does* load cleanly on top of
//! the panel-load surface and (b) what `ActionButtonUtil.GetActionBar
//! StatusForSpell` actually returns once it's wired to `C_ActionBar`
//! without any seeded bar slots, so future readers can audit the
//! decision without re-doing the experiment. Both probes are
//! `#[ignore]`d — they're an investigation record, not a regression gate.

use crate::common;

use common::panel_fixtures::{
    blizzard_ui_dir, install_lua_harness_stubs, load_panel_addons, recorded_lua_errors,
    seed_addon_search_paths,
};
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn build_env_with_real_action_bar() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("env");
    env.set_screen_size(1024.0, 768.0);
    seed_addon_search_paths(&env);
    load_panel_addons(&env);
    let toc = blizzard_ui_dir()
        .join("Blizzard_ActionBar")
        .join("Blizzard_ActionBar_Mainline.toc");
    let _ = load_addon(&env.loader_env(), &toc);
    install_lua_harness_stubs(&env);
    env.apply_post_load_workarounds();
    env
}

#[test]
#[ignore = "investigation: shows real Blizzard_ActionBar loads cleanly on top of panel fixture"]
fn probe_real_actionbar_load_against_panel_fixture() {
    let env = WowLuaEnv::new().expect("env");
    env.set_screen_size(1024.0, 768.0);
    seed_addon_search_paths(&env);
    load_panel_addons(&env);
    install_lua_harness_stubs(&env);
    env.apply_post_load_workarounds();

    let toc = blizzard_ui_dir()
        .join("Blizzard_ActionBar")
        .join("Blizzard_ActionBar_Mainline.toc");
    eprintln!("=== loading {} ===", toc.display());
    match load_addon(&env.loader_env(), &toc) {
        Ok(result) => eprintln!("OK loaded: {result:?}"),
        Err(error) => eprintln!("FAILED: {error:?}"),
    }
    let errors = recorded_lua_errors(&env);
    eprintln!("recorded errors ({}):", errors.len());
    for (index, message) in errors.iter().take(20).enumerate() {
        eprintln!("  [{index}] {message}");
    }
}

/// What `ActionButtonUtil.GetActionBarStatusFor*` return once the real
/// `Blizzard_ActionBar` is loaded over the panel fixture (no seeded
/// bar slots). The lightweight stub defaults every probe to
/// `NotMissing`; the real impl walks `C_ActionBar.FindSpellActionButtons`
/// and falls through to `GetActionBarStatusFromBars(nil)` →
/// `MissingFromAllBars`. Recording the divergence so the
/// `keep-the-stub` decision is reproducible.
#[test]
#[ignore = "investigation: shows the stub-vs-real divergence on an empty bar setup"]
fn probe_real_actionbar_status_for_unbar_spell() {
    let env = build_env_with_real_action_bar();
    let report: String = env
        .eval(
            r#"
            local enum = ActionButtonUtil.ActionBarActionStatus
            local function name_of(status)
                for key, value in pairs(enum) do
                    if value == status then return key end
                end
                return tostring(status)
            end
            local lines = {
                "spell=" .. name_of(ActionButtonUtil.GetActionBarStatusForSpell(19750, true, false)),
                "pet=" .. name_of(ActionButtonUtil.GetActionBarStatusForPetAction(1)),
                "flyout=" .. name_of(ActionButtonUtil.GetActionBarStatusForFlyout(1)),
                "search_spell=" .. name_of(SpellSearchUtil.GetActionbarStatusForSpell(19750)),
            }
            return table.concat(lines, " | ")
        "#,
        )
        .unwrap_or_else(|error| format!("eval_failed={error:?}"));
    eprintln!("real ActionBar probe: {report}");
}
