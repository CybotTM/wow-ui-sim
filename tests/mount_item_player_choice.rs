//! Whole cached consumer baseline; not mount-branch or native dressability proof.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use std::path::Path;

use wow_ui_sim::loader::{
    BlizzardAddonOverride, discover_blizzard_addon_closure_for_screen_with_overrides, load_addon,
};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

const ROOTS: &[&str] = &[
    "Blizzard_PlayerChoice",
    "Blizzard_ItemButton",
    "Blizzard_SharedXML",
];

const ORIGINAL_ITEM_CONSUMER: &str = r#"
    for _, addon in ipairs({
        'Blizzard_PlayerChoice', 'Blizzard_ItemButton', 'Blizzard_SharedXML'
    }) do
        assert(C_AddOns.IsAddOnLoaded(addon), 'whole addon not loaded: ' .. addon)
    end
    assert(DoesTemplateExist('PlayerChoiceBaseOptionItemRewardTemplate'),
        'original reward XML template missing')
    local reward = CreateFrame('Button', nil, UIParent,
        'PlayerChoiceBaseOptionItemRewardTemplate')
    assert(reward:GetObjectType() == 'Button', 'original XML reward Button')
    assert(reward.itemButton, 'original XML intrinsic ItemButton child missing')

    local originalRewardMethod = PlayerChoiceBaseOptionItemRewardMixin.IsDressupReward
    local originalSetItem = ItemButtonMixin.SetItem
    assert(reward.IsDressupReward == originalRewardMethod,
        'reward must retain original vendor method identity')
    assert(reward.itemButton.SetItem == originalSetItem,
        'intrinsic ItemButton must retain original vendor SetItem identity')
    assert(reward.itemButton.SetItemInternal == ItemButtonMixin.SetItemInternal,
        'original SetItemInternal identity')
    assert(reward.itemButton.GetItemInfo == ItemButtonMixin.GetItemInfo,
        'original vendor GetItemInfo identity, not a replacement getter')

    local function provenance(fn, suffix)
        local info = debug.getinfo(fn, 'S')
        local source = info.source:gsub('\\', '/')
        print('[mount-item consumer provenance]', info.what, source,
            info.linedefined, info.lastlinedefined)
        assert(info.what == 'Lua', 'expected cached vendor Lua method')
        assert(source:sub(-#suffix) == suffix, 'wrong vendor source: ' .. source)
    end
    local rewardSource = 'Blizzard_PlayerChoice/Blizzard_PlayerChoiceOptionBase.lua'
    local itemSource = 'Blizzard_ItemButton/Shared/ItemButtonTemplate.lua'
    provenance(reward.IsDressupReward, rewardSource)
    provenance(reward.itemButton.SetItem, itemSource)
    provenance(reward.itemButton.SetItemInternal, itemSource)
    provenance(reward.itemButton.GetItemInfo, itemSource)

    local name, link = C_Item.GetItemInfo(6948)
    assert(name == 'Hearthstone' and type(link) == 'string' and link ~= '',
        'generated real item metadata/link required')
    print('[mount-item consumer input]', name, link)
    print('[mount-item consumer stage]', 'original ItemButton:SetItem(6948)')
    reward.itemButton:SetItem(6948)
    assert(reward.itemButton.itemLink == link, 'real SetItem must store generated item link')
    assert(reward.itemButton:GetItemID() == 6948, 'real ItemButton item identity')
    assert(reward.itemButton.SetItem == originalSetItem,
        'SetItem identity unchanged after real setup')
    assert(reward.IsDressupReward == originalRewardMethod,
        'reward method identity unchanged before actual invocation')

    -- Observations only: no missing-API guard replaces the actual vendor call.
    print('[mount-item consumer API observations]',
        type(C_Item.IsDressableItemByID),
        type(C_MountJournal and C_MountJournal.GetMountFromItem),
        type(C_PetJournal and C_PetJournal.GetPetInfoByItemID))
    print('[mount-item consumer stage]', 'original reward:IsDressupReward({itemId=6948})')
    local value = reward:IsDressupReward({itemId=6948})
    assert(type(value) == 'boolean', 'actual original consumer must return a boolean')
    print('[mount-item consumer result]', value)
"#;

#[test]
fn mount_item_player_choice_real_item_consumer_returns_boolean() {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    eprintln!("[mount-item consumer cache] {}", ui.display());
    let item = wow_ui_sim::items::get_item(6948).expect("generated real item 6948");
    assert_eq!(item.name, "Hearthstone");
    let env = crate::common::blizzard_addon_harness::new_blizzard_addon_env(&ui);
    load_player_choice_closure(&env, &ui);
    invoke_original_item_consumer(&env);
}

fn load_player_choice_closure(env: &WowLuaEnv, ui: &Path) {
    let overrides = [BlizzardAddonOverride {
        addon: "Blizzard_PlayerChoice",
        extra_roots: &["Blizzard_ItemButton", "Blizzard_SharedXML"],
    }];
    let addons = discover_blizzard_addon_closure_for_screen_with_overrides(
        ui,
        ScreenKind::Game,
        ROOTS,
        &overrides,
    );
    eprintln!("[mount-item consumer discovered closure] {addons:?}");
    for root in ROOTS {
        assert!(
            addons.iter().any(|(name, _)| name == root),
            "whole closure root missing: {root}"
        );
    }
    for (name, toc) in addons {
        load_consumer_addon(env, &name, &toc);
    }
    assert_no_recorded_errors(env, "whole closure load");
}

fn load_consumer_addon(env: &WowLuaEnv, name: &str, toc: &Path) {
    eprintln!("[mount-item consumer load] {name}: {}", toc.display());
    let result = load_addon(&env.loader_env(), toc);
    print_recorded_errors(env, name);
    let loaded = result
        .unwrap_or_else(|error| panic!("whole closure load {name} at {}: {error}", toc.display()));
    eprintln!(
        "[mount-item consumer warnings] {name}: {:?}",
        loaded.warnings
    );
    eprintln!(
        "[mount-item consumer nil observations] {name}: {:?}",
        loaded.nil_symbol_observations
    );
    eprintln!(
        "[mount-item consumer missing requirements] {name}: {:?}",
        loaded.missing_requirements
    );
    assert!(
        loaded.warnings.is_empty(),
        "actual whole closure warnings for {name}: {:?}",
        loaded.warnings
    );
    assert_no_recorded_errors(env, name);
}

fn print_recorded_errors(env: &WowLuaEnv, boundary: &str) {
    let state = env.state().borrow();
    eprintln!(
        "[mount-item consumer Lua errors] {boundary}: {:?}",
        state.lua_errors
    );
    eprintln!(
        "[mount-item consumer Lua error records] {boundary}: {:?}",
        state.lua_error_records
    );
    eprintln!(
        "[mount-item consumer Lua error counts] {boundary}: {:?}",
        state.lua_error_counts
    );
}

fn assert_no_recorded_errors(env: &WowLuaEnv, boundary: &str) {
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{boundary} recorded Lua errors: {:?}",
        env.state().borrow().lua_errors
    );
}

fn invoke_original_item_consumer(env: &WowLuaEnv) {
    let result = env.exec(ORIGINAL_ITEM_CONSUMER);
    print_recorded_errors(env, "original SetItem / IsDressupReward invocation");
    if let Err(error) = &result {
        eprintln!("[mount-item consumer original invocation error] {error}");
    }
    result.expect("original real-item consumer call must succeed and return a boolean");
    assert_no_recorded_errors(env, "original SetItem / IsDressupReward invocation");
}
