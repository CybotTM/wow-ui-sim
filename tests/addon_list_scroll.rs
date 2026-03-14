//! AddonList ScrollBox behavioral tests.
//!
//! These tests require a large collection of addons to populate the scroll list.
//! They use the local WoW addon directory and skip in CI where it doesn't exist.

mod common;

use common::env_with_shared_xml;
use wow_ui_sim::loader::find_toc_file;
use wow_ui_sim::lua_api::WowLuaEnv;

const USER_ADDONS_PATH: &str = "/home/osso/Projects/wow/Interface/AddOns";

fn has_local_addons() -> bool {
    std::path::Path::new(USER_ADDONS_PATH).exists()
}

fn blizzard_addons_base() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Load all Blizzard addons needed for AddonList + scan user addons.
fn env_with_addon_list() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let base = blizzard_addons_base();

    let addon_dirs = [
        "Blizzard_SharedXMLBase",
        "Blizzard_Colors",
        "Blizzard_SharedXML",
        "Blizzard_SharedXMLGame",
        "Blizzard_UIPanelTemplates",
        "Blizzard_GameMenu",
        "Blizzard_UIWidgets",
        "Blizzard_FrameXMLBase",
        "Blizzard_AddOnList",
    ];

    for dir_name in &addon_dirs {
        let addon_dir = base.join(dir_name);
        let toc_path = find_toc_file(&addon_dir)
            .unwrap_or_else(|| panic!("No TOC found in {}", addon_dir.display()));
        if let Err(e) = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc_path) {
            eprintln!("Warning: Failed to load {}: {}", dir_name, e);
        }
    }

    env.scan_and_register_addons(std::path::Path::new(USER_ADDONS_PATH));

    // Re-init the AddonList ScrollBox view. During XML loading, the OnLoad handler's
    // SetView call doesn't persist due to a __newindex ordering issue in method chains.
    env.exec(r#"
        if AddonList and AddonList.ScrollBox and not AddonList.ScrollBox:HasView() then
            local view = CreateScrollBoxListTreeListView(20, 5, 5, 5, 5, 8)
            view:SetElementFactory(function(factory, treeNode)
                local elementData = treeNode:GetData()
                if elementData.addonIndex then
                    factory("AddonListEntryTemplate", AddonList_InitAddon)
                elseif elementData.category then
                    factory("AddonListCategoryTemplate", AddonList_InitCategory)
                end
            end)
            ScrollUtil.InitScrollBoxListWithScrollBar(AddonList.ScrollBox, AddonList.ScrollBar, view)
        end
    "#).unwrap();

    env
}

/// Return titles of visible AddonList entries.
fn get_visible_entry_titles(env: &WowLuaEnv) -> Vec<String> {
    let result: String = env
        .eval(
            r#"
        local titles = {}
        local view = AddonList.ScrollBox:GetView()
        if view then
            local frames = view:GetFrames()
            for _, frame in ipairs(frames) do
                if frame.Title then
                    local text = frame.Title:GetText()
                    if text then
                        table.insert(titles, text)
                    end
                end
            end
        end
        return table.concat(titles, "\030")
    "#,
        )
        .unwrap();
    if result.is_empty() {
        Vec::new()
    } else {
        result.split('\x1e').map(String::from).collect()
    }
}

/// Show AddonList, populate it, and fire OnUpdate ticks.
fn init_addon_list(env: &WowLuaEnv) {
    env.exec("AddonList:Show()").unwrap();
    env.exec("AddonList_Update()").unwrap();
    for _ in 0..5 {
        env.fire_on_update(0.016).unwrap();
    }
}

#[test]
fn test_addon_list_scroll_down_changes_entries() {
    if !has_local_addons() {
        return;
    }
    let env = env_with_addon_list();
    init_addon_list(&env);

    let before = get_visible_entry_titles(&env);
    assert!(
        !before.is_empty(),
        "AddonList should have visible entries after init"
    );
    let first_before = before[0].clone();

    for _ in 0..10 {
        env.exec("AddonList.ScrollBox:OnMouseWheel(-1)").unwrap();
        env.fire_on_update(0.016).unwrap();
    }

    let after = get_visible_entry_titles(&env);
    assert!(
        !after.is_empty(),
        "AddonList should still have visible entries after scroll"
    );

    assert_ne!(
        first_before, after[0],
        "First visible entry should change after scrolling down: was '{}', still '{}'",
        first_before, after[0]
    );
}

#[test]
fn test_addon_list_scroll_up_restores_entries() {
    if !has_local_addons() {
        return;
    }
    let env = env_with_addon_list();
    init_addon_list(&env);

    let original = get_visible_entry_titles(&env);
    assert!(
        !original.is_empty(),
        "AddonList should have visible entries"
    );

    for _ in 0..10 {
        env.exec("AddonList.ScrollBox:OnMouseWheel(-1)").unwrap();
        env.fire_on_update(0.016).unwrap();
    }

    let scrolled = get_visible_entry_titles(&env);
    assert_ne!(
        original[0], scrolled[0],
        "Should have scrolled away from initial position"
    );

    for _ in 0..10 {
        env.exec("AddonList.ScrollBox:OnMouseWheel(1)").unwrap();
        env.fire_on_update(0.016).unwrap();
    }

    let restored = get_visible_entry_titles(&env);
    assert_eq!(
        original[0], restored[0],
        "First entry should be restored after scrolling back up: expected '{}', got '{}'",
        original[0], restored[0]
    );
}

#[test]
fn test_addon_list_scroll_percentage_changes() {
    if !has_local_addons() {
        return;
    }
    let env = env_with_addon_list();
    init_addon_list(&env);

    let pct_before: f64 = env
        .eval("return AddonList.ScrollBox:GetScrollPercentage() or 0")
        .unwrap();
    assert!(
        pct_before < 0.01,
        "Initial scroll percentage should be ~0, got {}",
        pct_before
    );

    for _ in 0..5 {
        env.exec("AddonList.ScrollBox:OnMouseWheel(-1)").unwrap();
        env.fire_on_update(0.016).unwrap();
    }

    let pct_after: f64 = env
        .eval("return AddonList.ScrollBox:GetScrollPercentage() or 0")
        .unwrap();
    assert!(
        pct_after > pct_before,
        "Scroll percentage should increase after scrolling down: before={}, after={}",
        pct_before,
        pct_after
    );
}

// ============================================================================
// MinimalScrollBar Atlas Texture Tests (requires SharedXML)
// ============================================================================

#[test]
fn test_minimal_scrollbar_atlas_textures() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local sb = CreateFrame("EventFrame", "TestMinScrollBarAtlas", UIParent, "MinimalScrollBar")
        sb:SetSize(16, 200)
    "#,
    )
    .unwrap();

    let back_atlas: String = env
        .eval(
            r#"
        local tex = TestMinScrollBarAtlas.Back.Texture
        return tex:GetAtlas() or ""
    "#,
        )
        .unwrap();
    assert_eq!(
        back_atlas, "minimal-scrollbar-arrow-top",
        "Back button texture should have atlas set via OnLoad"
    );

    let back_file: String = env
        .eval(
            r#"
        local tex = TestMinScrollBarAtlas.Back.Texture
        return tex:GetTexture() or ""
    "#,
        )
        .unwrap();
    assert!(
        back_file.contains("minimalscrollbarproportional"),
        "Back texture file should be resolved from atlas: got '{}'",
        back_file
    );

    let forward_atlas: String = env
        .eval(
            r#"
        local tex = TestMinScrollBarAtlas.Forward.Texture
        return tex:GetAtlas() or ""
    "#,
        )
        .unwrap();
    assert_eq!(
        forward_atlas, "minimal-scrollbar-arrow-bottom",
        "Forward button texture should have atlas set via OnLoad"
    );
}
