#![cfg(any(feature = "client-retail", feature = "client-ptr", feature = "client-mists"))]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
use wow_ui_sim::screen::ScreenKind;

#[test]
fn original_lfg_listing_controls_follow_supplied_squelch_transitions() {
    let ui = blizzard_ui_dir();
    let discovered = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    assert!(discovered.iter().any(|(name, _)| name == "Blizzard_GroupFinder"));
    let env = new_blizzard_addon_env(&ui);
    env.state().borrow_mut().social_restrictions.is_squelched = false;
    let loaded = load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_GroupFinder"], &[]);
    assert!(loaded.iter().any(|name| name == "Blizzard_GroupFinder"));
    env.exec(r#"
        local source = debug.getinfo(LFGListCategorySelection_UpdateNavButtons, "S").source:gsub("\\", "/")
        local flavor = WOW_PROJECT_ID == WOW_PROJECT_MAINLINE and "Mainline" or "Classic"
        assert(source:find("Blizzard_GroupFinder/" .. flavor .. "/LFGList.lua", 1, true), source)
        assert(LFGListFrame.CategorySelection.FindGroupButton)
        assert(LFGListFrame.CategorySelection.StartGroupButton)
        LFGListFrame.CategorySelection.selectedCategory = 2
        assert(not C_SocialRestrictions.IsSilenced(), "fixture must not be silenced")
    "#).unwrap();

    let independent = new_blizzard_addon_env(&ui);
    independent.state().borrow_mut().social_restrictions.is_squelched = true;
    for squelched in [false, true, false] {
        env.state().borrow_mut().social_restrictions.is_squelched = squelched;
        env.exec(&format!("expectedSquelched = {squelched}")).unwrap();
        env.exec(r#"
            local panel = LFGListFrame.CategorySelection
            LFGListCategorySelection_UpdateNavButtons(panel)
            assert(panel.FindGroupButton:IsEnabled() == not expectedSquelched, "find enabled state")
            assert(panel.StartGroupButton:IsEnabled() == not expectedSquelched, "start enabled state")
            local tooltip = expectedSquelched and RED_FONT_COLOR:WrapTextInColorCode(ERR_USER_SQUELCHED) or nil
            assert(panel.FindGroupButton.tooltip == tooltip, "find restriction tooltip")
            assert(panel.StartGroupButton.tooltip == tooltip, "start restriction tooltip")
        "#).expect("original LFG controls must follow false/true/false supplied restriction state");
        independent.exec("assert(C_SocialRestrictions.IsSquelched() == true)").unwrap();
    }
}
