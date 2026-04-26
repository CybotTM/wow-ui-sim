//! Permissive `Menu.*` descriptor fallback installed after
//! `Blizzard_Menu` fails mid-load — see
//! `MENU_DESCRIPTOR_FALLBACK_LUA` in `src/lua_api/loader_env.rs`.
//!
//! These tests pin the contract: unknown methods return `self`, and
//! the five known iterator methods yield an empty iterator. Delete
//! these (and the fallback) once Menu.lua loads cleanly.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env_with_fallback() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("WowLuaEnv init");
    env.loader_env()
        .ensure_menu_descriptor_fallback()
        .expect("fallback install");
    env
}

#[test]
fn create_root_returns_table() {
    let env = env_with_fallback();
    let is_table: bool = env
        .eval(r#"return type(Menu.CreateRootMenuDescription()) == "table""#)
        .unwrap();
    assert!(is_table);
}

#[test]
fn create_element_returns_table() {
    let env = env_with_fallback();
    let is_table: bool = env
        .eval(r#"return type(Menu.CreateMenuElementDescription()) == "table""#)
        .unwrap();
    assert!(is_table);
}

#[test]
fn menuutil_delegates_to_menu_create_root() {
    let env = env_with_fallback();
    let is_table: bool = env
        .eval(r#"return type(MenuUtil.CreateRootMenuDescription()) == "table""#)
        .unwrap();
    assert!(is_table);
}

#[test]
fn unknown_method_returns_self_for_chaining() {
    let env = env_with_fallback();
    let chain_ok: bool = env
        .eval(
            r#"
            local desc = Menu.CreateRootMenuDescription()
            local leaf = desc:CreateRadio("label"):SetEnabled(false):SetResponse(1)
            return leaf == desc or type(leaf) == "table"
            "#,
        )
        .unwrap();
    assert!(chain_ok, "chained unknown methods must yield a table");
}

#[test]
fn iterator_methods_yield_empty_sequence() {
    let env = env_with_fallback();
    let counts: (i64, i64, i64, i64, i64) = env
        .eval(
            r#"
            local desc = Menu.CreateRootMenuDescription()
            local function count(iter)
                local n = 0
                for _ in iter do n = n + 1 end
                return n
            end
            return count(desc:EnumerateElementDescriptions()),
                   count(desc:EnumerateActiveElementDescriptions()),
                   count(desc:EnumerateChildren()),
                   count(desc:EnumerateInitializers()),
                   count(desc:EnumerateFrames())
            "#,
        )
        .unwrap();
    assert_eq!(counts, (0, 0, 0, 0, 0));
}

#[test]
fn populate_description_invokes_generator_under_pcall() {
    let env = env_with_fallback();
    let calls: i64 = env
        .eval(
            r#"
            local n = 0
            local function gen(owner, desc)
                n = n + 1
                -- must be safe to call unknown methods on desc
                desc:CreateButton("foo"):SetEnabled(true)
            end
            Menu.PopulateDescription(gen, nil, Menu.CreateRootMenuDescription())
            return n
            "#,
        )
        .unwrap();
    assert_eq!(calls, 1);
}

#[test]
fn descriptor_records_radio_entries_for_dropdown_inspection() {
    let env = env_with_fallback();
    let labels: (i64, String, String) = env
        .eval(
            r#"
            local desc = Menu.CreateRootMenuDescription()
            desc:CreateRadio("Guild Leader", function() return false end, function() end, 1)
            desc:CreateRadio("Officer", function() return true end, function() end, 2)
            return #desc.__wow_elements, desc.__wow_elements[1].text, desc.__wow_elements[2].text
            "#,
        )
        .unwrap();
    assert_eq!(labels, (2, "Guild Leader".into(), "Officer".into()));
}

#[test]
fn dropdown_generate_menu_populates_setup_menu_generator() {
    let env = env_with_fallback();
    let labels: (i64, String, String) = env
        .eval(
            r#"
            local dropdown = CreateFrame("DropdownButton", "MenuFallbackDropdown", UIParent)
            Mixin(dropdown, DropdownButtonMixin)
            dropdown:SetupMenu(function(_, rootDescription)
                rootDescription:SetTag("MENU_GUILD_RANKS")
                rootDescription:CreateRadio("Guild Leader", function() return false end, function() end, 1)
                rootDescription:CreateRadio("Officer", function() return true end, function() end, 2)
            end)
            local desc = dropdown:GenerateMenu()
            return #desc.__wow_elements, desc.__wow_elements[1].text, desc.__wow_elements[2].text
            "#,
        )
        .unwrap();
    assert_eq!(labels, (2, "Guild Leader".into(), "Officer".into()));
}

#[test]
fn dropdown_open_menu_materializes_visible_rank_rows() {
    let env = env_with_fallback();
    let labels: (String, String, bool, bool) = env
        .eval(
            r#"
            local dropdown = CreateFrame("DropdownButton", "MenuFallbackVisibleDropdown", UIParent)
            Mixin(dropdown, DropdownButtonMixin)
            dropdown:SetSize(160, 20)
            dropdown:SetupMenu(function(_, rootDescription)
                rootDescription:CreateRadio("Guild Leader", function() return false end, function() end, 1)
                rootDescription:CreateRadio("Officer", function() return true end, function() end, 2)
            end)
            dropdown:OpenMenu()
            local first = MenuFallbackVisibleDropdownMenuButton1
            local second = MenuFallbackVisibleDropdownMenuButton2
            return first:GetText(), second:GetText(), first:IsVisible(), second:IsVisible()
            "#,
        )
        .unwrap();
    assert_eq!(
        labels,
        ("Guild Leader".into(), "Officer".into(), true, true)
    );
}

#[test]
fn dropdown_open_menu_materializes_reputation_filter_rows() {
    let env = env_with_fallback();
    let rows: (String, String, String, String, bool, bool, bool, bool) = env
        .eval(
            r#"
            local dropdown = CreateFrame("DropdownButton", "MenuFallbackReputationDropdown", UIParent)
            Mixin(dropdown, DropdownButtonMixin)
            dropdown:SetSize(200, 20)
            dropdown:SetupMenu(function(_, rootDescription)
                rootDescription:SetTag("MENU_REPUTATION_FRAME_FILTER")
                rootDescription:CreateRadio("All", function(value) return value == 0 end, function() end, 0)
                rootDescription:CreateRadio("Warband", function(value) return value == 0 end, function() end, 1)
                rootDescription:CreateRadio("Varian", function(value) return value == 0 end, function() end, 2)
                rootDescription:CreateDivider()
                rootDescription:CreateCheckbox("Show Legacy Reputations", function() return true end, function() end)
            end)
            dropdown:OpenMenu()
            local first = MenuFallbackReputationDropdownMenuButton1
            local second = MenuFallbackReputationDropdownMenuButton2
            local third = MenuFallbackReputationDropdownMenuButton3
            local fourth = MenuFallbackReputationDropdownMenuButton4
            return first:GetText(), second:GetText(), third:GetText(), fourth:GetText(),
                   first:IsVisible(), second:IsVisible(), third:IsVisible(), fourth:IsVisible()
            "#,
        )
        .unwrap();
    assert_eq!(
        rows,
        (
            "All".into(),
            "Warband".into(),
            "Varian".into(),
            "Show Legacy Reputations".into(),
            true,
            true,
            true,
            true,
        )
    );
}

#[test]
fn dropdown_generate_menu_sets_selected_radio_text() {
    let env = env_with_fallback();
    let text: String = env
        .eval(
            r#"
            local dropdown = CreateFrame("DropdownButton", "MenuFallbackSelectedDropdown", UIParent)
            Mixin(dropdown, DropdownButtonMixin)
            local selected = 2
            dropdown:SetupMenu(function(_, rootDescription)
                rootDescription:CreateRadio("Guild Leader", function(value) return value == selected end, function() end, 1)
                rootDescription:CreateRadio("Officer", function(value) return value == selected end, function() end, 2)
            end)
            dropdown:GenerateMenu()
            return (dropdown:GetText() or "") .. "|" .. (dropdown.Text and dropdown.Text:GetText() or "")
            "#,
        )
        .unwrap();
    assert_eq!(text, "Officer|Officer");
}

#[test]
fn populate_description_swallows_generator_errors() {
    let env = env_with_fallback();
    let ok: bool = env
        .eval(
            r#"
            local function boom() error("nope") end
            local ran = pcall(function()
                Menu.PopulateDescription(boom, nil, Menu.CreateRootMenuDescription())
            end)
            return ran
            "#,
        )
        .unwrap();
    assert!(ok, "PopulateDescription must pcall-wrap the generator");
}

#[test]
fn install_is_idempotent() {
    let env = env_with_fallback();
    env.loader_env()
        .ensure_menu_descriptor_fallback()
        .expect("second install");
    let still_a_table: bool = env
        .eval(r#"return type(Menu.CreateRootMenuDescription()) == "table""#)
        .unwrap();
    assert!(still_a_table);
}
