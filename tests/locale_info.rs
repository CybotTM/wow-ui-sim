//! `GetAvailableLocaleInfo()` — exercise the full 12-locale retail list
//! from the Lua side.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn locale_info_global_is_classified_as_real_lua_api() {
    let globals_mod = include_str!("../src/lua_api/globals/mod.rs");
    let real_mod = include_str!("../src/lua_api/globals/real/mod.rs");
    let registrar = include_str!("../src/lua_api/globals/register.rs");

    assert!(
        !globals_mod.contains("pub mod locale_info;"),
        "retail locale list should not live in the globals base module"
    );
    assert!(
        real_mod.contains("pub mod locale_info;"),
        "retail locale list should be classified under globals::real"
    );
    assert!(
        registrar.contains("real::locale_info::register_all"),
        "global registrar should wire GetAvailableLocaleInfo through globals::real"
    );
}

#[test]
fn returns_twelve_entries() {
    let env = WowLuaEnv::new().unwrap();
    let count: i32 = env.eval(r#"return #GetAvailableLocaleInfo()"#).unwrap();
    assert_eq!(count, 12);
}

#[test]
fn entries_have_four_canonical_fields() {
    let env = WowLuaEnv::new().unwrap();
    let (a, b, c, d): (String, String, String, String) = env
        .eval(
            r#"
            local locales = GetAvailableLocaleInfo()
            local e = locales[1]
            return type(e.localeId) == "number" and "ok" or "bad",
                   type(e.localeName) == "string" and "ok" or "bad",
                   type(e.englishName) == "string" and "ok" or "bad",
                   type(e.displayName) == "string" and "ok" or "bad"
            "#,
        )
        .unwrap();
    assert_eq!(
        (a.as_str(), b.as_str(), c.as_str(), d.as_str()),
        ("ok", "ok", "ok", "ok")
    );
}

#[test]
fn en_us_is_first() {
    let env = WowLuaEnv::new().unwrap();
    let (id, name): (i32, String) = env
        .eval(
            r#"
            local first = GetAvailableLocaleInfo()[1]
            return first.localeId, first.localeName
            "#,
        )
        .unwrap();
    assert_eq!(id, 1);
    assert_eq!(name, "enUS");
}

#[test]
fn all_twelve_retail_locales_present_in_order() {
    let env = WowLuaEnv::new().unwrap();
    let joined: String = env
        .eval(
            r#"
            local parts = {}
            for _, l in ipairs(GetAvailableLocaleInfo()) do
                parts[#parts + 1] = l.localeName
            end
            return table.concat(parts, ",")
            "#,
        )
        .unwrap();
    assert_eq!(
        joined,
        "enUS,enGB,frFR,deDE,esES,esMX,itIT,ptBR,ruRU,koKR,zhCN,zhTW",
    );
}

#[test]
fn locale_ids_are_dense_from_one() {
    let env = WowLuaEnv::new().unwrap();
    let ok: bool = env
        .eval(
            r#"
            local locales = GetAvailableLocaleInfo()
            for i, l in ipairs(locales) do
                if l.localeId ~= i then return false end
            end
            return true
            "#,
        )
        .unwrap();
    assert!(ok, "localeId values should be dense starting at 1");
}

#[test]
fn display_names_use_native_script() {
    // Sanity-check that we're returning the native script forms for locales
    // where they differ from English (not just re-using englishName).
    let env = WowLuaEnv::new().unwrap();
    let (ru, ko, zh_cn): (String, String, String) = env
        .eval(
            r#"
            local want = { ruRU = nil, koKR = nil, zhCN = nil }
            for _, l in ipairs(GetAvailableLocaleInfo()) do
                if want[l.localeName] == nil and want[l.localeName] == nil then
                    want[l.localeName] = l.displayName
                end
            end
            return want.ruRU, want.koKR, want.zhCN
            "#,
        )
        .unwrap();
    assert_eq!(ru, "Русский");
    assert_eq!(ko, "한국어");
    assert_eq!(zh_cn, "简体中文");
}

#[test]
fn return_is_a_table() {
    let env = WowLuaEnv::new().unwrap();
    let t: String = env
        .eval(r#"return type(GetAvailableLocaleInfo())"#)
        .unwrap();
    assert_eq!(t, "table");
}
