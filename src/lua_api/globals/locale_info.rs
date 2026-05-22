//! `GetAvailableLocaleInfo()` — list of shipped retail WoW locales.
//!
//! Each entry mirrors the Blizzard `LocaleInfo` shape:
//!
//! ```text
//! { localeId: integer, localeName: "xxYY", englishName, displayName }
//! ```
//!
//! `localeId` values follow Blizzard's 1..N enumeration order; consumers
//! (`Settings/LanguageDropdown`, `Blizzard_Settings`) iterate the list and
//! match on `localeName`, so the numbering only matters for stable order.
//!
//! The data is a compile-time `const` here — no SimState knob because the
//! list is fixed per client build. If a test needs to fake a locale, it can
//! override `GetAvailableLocaleInfo` at runtime.

use crate::lua_api::methods::{create_string, create_table, create_table_with_capacity};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

struct LocaleInfo {
    locale_id: i32,
    locale_name: &'static str,
    english_name: &'static str,
    display_name: &'static str,
}

const LOCALE_INFO_ENTRY_HASH_FIELDS: usize = 4;

const LOCALES: &[LocaleInfo] = &[
    LocaleInfo {
        locale_id: 1,
        locale_name: "enUS",
        english_name: "English (US)",
        display_name: "English (US)",
    },
    LocaleInfo {
        locale_id: 2,
        locale_name: "enGB",
        english_name: "English (UK)",
        display_name: "English (UK)",
    },
    LocaleInfo {
        locale_id: 3,
        locale_name: "frFR",
        english_name: "French",
        display_name: "Français",
    },
    LocaleInfo {
        locale_id: 4,
        locale_name: "deDE",
        english_name: "German",
        display_name: "Deutsch",
    },
    LocaleInfo {
        locale_id: 5,
        locale_name: "esES",
        english_name: "Spanish (Spain)",
        display_name: "Español (EU)",
    },
    LocaleInfo {
        locale_id: 6,
        locale_name: "esMX",
        english_name: "Spanish (Latin America)",
        display_name: "Español (AL)",
    },
    LocaleInfo {
        locale_id: 7,
        locale_name: "itIT",
        english_name: "Italian",
        display_name: "Italiano",
    },
    LocaleInfo {
        locale_id: 8,
        locale_name: "ptBR",
        english_name: "Portuguese (Brazil)",
        display_name: "Português (Brasil)",
    },
    LocaleInfo {
        locale_id: 9,
        locale_name: "ruRU",
        english_name: "Russian",
        display_name: "Русский",
    },
    LocaleInfo {
        locale_id: 10,
        locale_name: "koKR",
        english_name: "Korean",
        display_name: "한국어",
    },
    LocaleInfo {
        locale_id: 11,
        locale_name: "zhCN",
        english_name: "Chinese (Simplified)",
        display_name: "简体中文",
    },
    LocaleInfo {
        locale_id: 12,
        locale_name: "zhTW",
        english_name: "Chinese (Traditional)",
        display_name: "繁體中文",
    },
];

fn set_entry_field(state: &mut LuaState, table_val: Val, key: &'static str, value: Val) {
    let Val::Table(table_ref) = table_val else {
        return;
    };
    let key_ref = state.gc.intern_string_static(key.as_bytes());
    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        let _ = table.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(table_ref);
}

fn set_array_index(state: &mut LuaState, list_val: Val, index: i64, value: Val) {
    let Val::Table(list_ref) = list_val else {
        return;
    };
    if let Some(list) = state.gc.tables.get_mut(list_ref) {
        let _ = list.raw_set(Val::Num(index as f64), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(list_ref);
}

pub fn get_available_locale_info(state: &mut LuaState) -> LuaResult<u32> {
    let list = create_table(state);
    for (idx, locale) in LOCALES.iter().enumerate() {
        let entry = create_table_with_capacity(state, LOCALE_INFO_ENTRY_HASH_FIELDS);
        set_entry_field(state, entry, "localeId", Val::Num(locale.locale_id as f64));
        let locale_name = create_string(state, locale.locale_name);
        set_entry_field(state, entry, "localeName", locale_name);
        let english_name = create_string(state, locale.english_name);
        set_entry_field(state, entry, "englishName", english_name);
        let display_name = create_string(state, locale.display_name);
        set_entry_field(state, entry, "displayName", display_name);
        set_array_index(state, list, (idx + 1) as i64, entry);
    }
    state.push(list);
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    table_set_rust_fn_static(
        state,
        state.global,
        "GetAvailableLocaleInfo",
        get_available_locale_info,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::LOCALES;

    #[test]
    fn every_locale_has_four_letter_tag() {
        for locale in LOCALES {
            assert_eq!(
                locale.locale_name.len(),
                4,
                "locale name {:?} should be 4 chars",
                locale.locale_name,
            );
        }
    }

    #[test]
    fn locale_ids_are_dense_and_start_at_one() {
        for (idx, locale) in LOCALES.iter().enumerate() {
            assert_eq!(locale.locale_id, (idx + 1) as i32);
        }
    }

    #[test]
    fn locale_names_are_unique() {
        let mut names: Vec<_> = LOCALES.iter().map(|l| l.locale_name).collect();
        names.sort();
        let before = names.len();
        names.dedup();
        assert_eq!(names.len(), before, "locale_names should all be unique");
    }

    #[test]
    fn all_twelve_retail_locales_present() {
        let expected = [
            "enUS", "enGB", "frFR", "deDE", "esES", "esMX", "itIT", "ptBR", "ruRU", "koKR", "zhCN",
            "zhTW",
        ];
        let actual: Vec<_> = LOCALES.iter().map(|l| l.locale_name).collect();
        assert_eq!(actual, expected);
    }
}
