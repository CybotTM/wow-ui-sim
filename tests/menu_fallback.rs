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
