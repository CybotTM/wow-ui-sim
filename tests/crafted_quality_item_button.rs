//! Separate whole-original cached consumer candidate; unexecuted, not a valid-fixture claim.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use std::path::Path;
use wow_ui_sim::c_api::c_trade_skill_quality::CraftingQualityInfo;
use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen_with_overrides, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

#[path = "common/blizzard_addon_closure.rs"]
mod crafted_closure;

const ITEM_ID: u32 = 900001;
const OVERLAY_ATLAS: &str = "professions-icon-quality-tier3-small";

#[test]
fn crafted_quality_original_item_button_observes_supplied_quality_and_visibility() {
    let ui = crafted_closure::blizzard_ui_dir();
    let env = crafted_closure::new_blizzard_addon_env(&ui);
    load_item_button_closure(&env, &ui);
    exec_consumer_boundary(
        &env,
        "actual public sibling and empty-input baseline",
        r#"
        assert(C_AddOns.IsAddOnLoaded('Blizzard_ItemButton'), 'whole original addon required')
        assert(type(SetItemCraftingQualityOverlay) == 'function', 'original handler required')
        local info = debug.getinfo(SetItemCraftingQualityOverlay, 'S')
        print('[crafted consumer provenance]', info.what, info.source, info.linedefined, info.lastlinedefined)
        assert(info.what == 'Lua', 'original cached Lua handler required')
        assert(not debug.getstacktaint(), 'untainted host consumer setup')
        local reagent = C_TradeSkillUI.GetItemReagentQualityInfo(900001)
        print('[crafted actual reagent baseline]', type(reagent), tostring(reagent))
        assert(reagent == nil, 'sibling actual public baseline must be nil; do not override it')
        local crafted = C_TradeSkillUI.GetItemCraftedQualityInfo(900001)
        print('[crafted actual empty-input baseline]', type(crafted), tostring(crafted))
        assert(crafted == nil, 'empty supplied state must be unknown')
        craftedButton = CreateFrame('Button', nil, UIParent)
        craftedButton:SetSize(32, 32)
        craftedButton:Show()
        assert(craftedButton:GetObjectType() == 'Button')
        SetItemCraftingQualityOverlay(craftedButton, 900001)
        assert(craftedButton.isCraftedItem == false and craftedButton.isProfessionItem == false)
        assert(craftedButton.ProfessionQualityOverlay == nil, 'no quality must not create overlay')
        -- Explicit host UI input: a real simulator Frame, not a table/spied IsShown.
        -- This is NOT the full original professions panel or native panel acceptance.
        assert(ProfessionsFrame == nil, 'minimal ItemButton closure must not replace an existing panel')
        ProfessionsFrame = CreateFrame('Frame', 'CraftedQualityVisibilityInput', UIParent)
        assert(ProfessionsFrame:GetObjectType() == 'Frame')
        ProfessionsFrame:Hide()
        craftedButton.alwaysShowProfessionsQuality = false
    "#,
    );
    env.state().borrow_mut().crafted_item_quality.insert(
        ITEM_ID,
        CraftingQualityInfo {
            quality: 3,
            icon: "consumer-icon".into(),
            icon_small: "consumer-small".into(),
            icon_inventory: OVERLAY_ATLAS.into(),
            icon_mixed: "consumer-mixed".into(),
            icon_appear: "consumer-appear".into(),
            icon_dissolve: "consumer-dissolve".into(),
            bar_fill: "consumer-fill".into(),
            bar_background: "consumer-background".into(),
            bar_background_cap: "consumer-cap".into(),
            bar_highlight: "consumer-highlight".into(),
            icon_chat: "consumer-chat".into(),
            icon_quest_objective: "consumer-quest".into(),
        },
    );
    exec_consumer_boundary(
        &env,
        "original handler positive-map RED boundary",
        r#"
        print('[crafted positive host input]', 900001, 'professions-icon-quality-tier3-small')
        SetItemCraftingQualityOverlay(craftedButton, 900001)
        print('[crafted original positive effects]', tostring(craftedButton.ProfessionQualityOverlay),
            tostring(craftedButton.isCraftedItem), tostring(craftedButton.isProfessionItem))
        assert(craftedButton.isCraftedItem == true, 'positive supplied item must be crafted, not auto-nil')
        assert(craftedButton.isProfessionItem == true)
        local overlay = craftedButton.ProfessionQualityOverlay
        assert(overlay and overlay:GetObjectType() == 'Texture', 'real consumer-created Texture required')
        assert(overlay:GetAtlas() == 'professions-icon-quality-tier3-small')
        local point, relative, relativePoint, x, y = overlay:GetPoint(1)
        assert(point == 'TOPLEFT' and relative == craftedButton and relativePoint == 'TOPLEFT')
        assert(x == -3 and y == 2)
        local layer, sublevel = overlay:GetDrawLayer()
        assert(layer == 'OVERLAY' and sublevel == 7)
        assert(not overlay:IsShown(), 'crafted hidden when real host profession frame hidden')
        ProfessionsFrame:Show()
        SetItemCraftingQualityOverlay(craftedButton, 900001)
        assert(overlay:IsShown(), 'whole original handler uses real shown profession frame')
        ProfessionsFrame:Hide()
        SetItemCraftingQualityOverlay(craftedButton, 900001)
        assert(not overlay:IsShown())
        craftedButton.alwaysShowProfessionsQuality = true
        SetItemCraftingQualityOverlay(craftedButton, 900001)
        assert(overlay:IsShown(), 'explicit boolean always-show input')
        craftedButton.alwaysShowProfessionsQuality = false
        SetItemCraftingQualityOverlay(craftedButton, 900001)
        assert(not overlay:IsShown())
        ClearItemCraftingQualityOverlay(craftedButton)
        assert(not overlay:IsShown() and overlay:GetTexture() == nil)
        assert(craftedButton.isCraftedItem == true and craftedButton.isProfessionItem == true,
            'quality-only clear does not reset flags')
        SetItemCraftingQualityOverlay(craftedButton, 900001)
        assert(overlay:GetAtlas() == 'professions-icon-quality-tier3-small')
        ClearItemButtonOverlay(craftedButton)
        assert(not overlay:IsShown() and overlay:GetTexture() == nil)
        assert(craftedButton.isCraftedItem == false and craftedButton.isProfessionItem == false)
    "#,
    );
    env.state()
        .borrow_mut()
        .crafted_item_quality
        .remove(&ITEM_ID);
    exec_consumer_boundary(
        &env,
        "original cleared handler after host removal",
        r#"
        SetItemCraftingQualityOverlay(craftedButton, 900001)
        assert(craftedButton.isCraftedItem == false and craftedButton.isProfessionItem == false)
        assert(not craftedButton.ProfessionQualityOverlay:IsShown())
    "#,
    );
}

fn load_item_button_closure(env: &WowLuaEnv, ui: &Path) {
    eprintln!("[crafted consumer profile cache] {}", ui.display());
    let roots = &["Blizzard_ItemButton"];
    let addons =
        discover_blizzard_addon_closure_for_screen_with_overrides(ui, ScreenKind::Game, roots, &[]);
    eprintln!("[crafted consumer whole closure] {addons:?}");
    assert!(addons.iter().any(|(name, _)| name == "Blizzard_ItemButton"));
    for (name, toc) in addons {
        load_logged_addon(env, &name, &toc);
    }
}

fn load_logged_addon(env: &WowLuaEnv, name: &str, toc: &Path) {
    eprintln!("[crafted consumer loading] {name}: {}", toc.display());
    let result = load_addon(&env.loader_env(), toc);
    print_lua_records(env, name);
    let loaded = result.unwrap_or_else(|error| panic!("whole closure {name}: {error}"));
    eprintln!(
        "[crafted consumer full warnings] {name}: {:?}",
        loaded.warnings
    );
    eprintln!(
        "[crafted consumer full nil observations] {name}: {:?}",
        loaded.nil_symbol_observations
    );
    eprintln!(
        "[crafted consumer full missing requirements] {name}: {:?}",
        loaded.missing_requirements
    );
    assert!(
        loaded.warnings.is_empty(),
        "whole closure warnings for {name}: {:?}",
        loaded.warnings
    );
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "whole closure Lua errors: {name}"
    );
}

fn exec_consumer_boundary(env: &WowLuaEnv, boundary: &str, chunk: &str) {
    eprintln!("[crafted consumer boundary] {boundary}");
    let result = env.exec(chunk);
    print_lua_records(env, boundary);
    result.unwrap_or_else(|error| panic!("{boundary}: {error}"));
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "consumer Lua errors: {boundary}"
    );
}

fn print_lua_records(env: &WowLuaEnv, boundary: &str) {
    let state = env.state().borrow();
    eprintln!(
        "[crafted consumer Lua errors] {boundary}: {:?}",
        state.lua_errors
    );
    eprintln!(
        "[crafted consumer Lua records] {boundary}: {:?}",
        state.lua_error_records
    );
    eprintln!(
        "[crafted consumer Lua counts] {boundary}: {:?}",
        state.lua_error_counts
    );
}
