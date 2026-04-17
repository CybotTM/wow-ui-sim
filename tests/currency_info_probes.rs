//! Tests for `C_CurrencyInfo` probes backed by `SimState.currency_info`:
//!
//! - `C_CurrencyInfo.GetCurrencyInfo(currencyID)` — returns the retail
//!   `CurrencyInfo` structure, or nothing when the currency id is not
//!   seeded.
//! - `C_CurrencyInfo.GetCurrencyInfoFromLink(link)` — parses the
//!   currency id out of a `|Hcurrency:<id>:...` hyperlink and returns
//!   the same structure.
//! - `C_CurrencyInfo.GetCurrencyContainerInfo(currencyID, quantity)`
//!   — returns the 6-field `CurrencyDisplayInfo` structure with
//!   `actualAmount` / `displayAmount` set to the passed quantity.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::CurrencyInfo;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_currency_info_returns_seeded_valorstones_row() {
    let env = env();
    let (name, quantity, quality, icon, is_show_in_backpack): (String, i32, i32, i32, bool) = env
        .eval(
            r#"
            local info = C_CurrencyInfo.GetCurrencyInfo(2245)
            return info.name,
                   info.quantity,
                   info.quality,
                   info.iconFileID,
                   info.isShowInBackpack
            "#,
        )
        .unwrap();
    assert_eq!(name, "Valorstones");
    assert_eq!(quantity, 1847);
    assert_eq!(quality, 3);
    assert_eq!(icon, 5868905);
    assert!(is_show_in_backpack, "Valorstones is a backpack currency");
}

#[test]
fn get_currency_info_exposes_all_retail_fields() {
    let env = env();
    let (has_id, has_name, has_desc, has_qty, has_max_qty, has_quality, has_icon): (
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local info = C_CurrencyInfo.GetCurrencyInfo(2245)
            return info.currencyID ~= nil,
                   info.name ~= nil,
                   info.description ~= nil,
                   info.quantity ~= nil,
                   info.maxQuantity ~= nil,
                   info.quality ~= nil,
                   info.iconFileID ~= nil
            "#,
        )
        .unwrap();
    assert!(has_id);
    assert!(has_name);
    assert!(has_desc);
    assert!(has_qty);
    assert!(has_max_qty);
    assert!(has_quality);
    assert!(has_icon);
}

#[test]
fn get_currency_info_returns_nothing_for_unknown_id() {
    let env = env();
    let nret: i32 = env
        .eval("return select('#', C_CurrencyInfo.GetCurrencyInfo(999999))")
        .unwrap();
    assert_eq!(nret, 0);
}

#[test]
fn get_currency_info_reflects_sim_state_mutation() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.currency_info.insert(
            42,
            CurrencyInfo {
                currency_id: 42,
                name: "Bonus Currency".into(),
                quantity: 7,
                quality: 4,
                ..CurrencyInfo::default()
            },
        );
    }

    let (name, qty, quality): (String, i32, i32) = env
        .eval(
            r#"
            local info = C_CurrencyInfo.GetCurrencyInfo(42)
            return info.name, info.quantity, info.quality
            "#,
        )
        .unwrap();
    assert_eq!(name, "Bonus Currency");
    assert_eq!(qty, 7);
    assert_eq!(quality, 4);
}

#[test]
fn get_currency_info_from_link_parses_currency_hyperlink() {
    let env = env();
    let (name, qty): (String, i32) = env
        .eval(
            r#"
            local link = "|cffa335ee|Hcurrency:2245:1847|h[Valorstones]|h|r"
            local info = C_CurrencyInfo.GetCurrencyInfoFromLink(link)
            return info.name, info.quantity
            "#,
        )
        .unwrap();
    assert_eq!(name, "Valorstones");
    assert_eq!(qty, 1847);
}

#[test]
fn get_currency_info_from_link_returns_nothing_for_non_currency_link() {
    let env = env();
    let nret: i32 = env
        .eval(
            r#"
            return select('#', C_CurrencyInfo.GetCurrencyInfoFromLink("|Hitem:12345::::|h[Some Item]|h"))
            "#,
        )
        .unwrap();
    assert_eq!(nret, 0);
}

#[test]
fn get_currency_info_from_link_returns_nothing_for_unknown_currency_id() {
    let env = env();
    let nret: i32 = env
        .eval(
            r#"
            return select('#', C_CurrencyInfo.GetCurrencyInfoFromLink("|Hcurrency:999999:10|h[X]|h"))
            "#,
        )
        .unwrap();
    assert_eq!(nret, 0);
}

#[test]
fn get_currency_container_info_returns_display_info_with_passed_quantity() {
    let env = env();
    let (actual, display, name, icon, quality): (i32, i32, String, i32, i32) = env
        .eval(
            r#"
            local info = C_CurrencyInfo.GetCurrencyContainerInfo(2245, 250)
            return info.actualAmount,
                   info.displayAmount,
                   info.name,
                   info.icon,
                   info.quality
            "#,
        )
        .unwrap();
    assert_eq!(actual, 250);
    assert_eq!(display, 250);
    assert_eq!(name, "Valorstones");
    assert_eq!(icon, 5868905);
    assert_eq!(quality, 3);
}

#[test]
fn get_currency_container_info_returns_nothing_for_unknown_id() {
    let env = env();
    let nret: i32 = env
        .eval(
            "return select('#', C_CurrencyInfo.GetCurrencyContainerInfo(999999, 1))",
        )
        .unwrap();
    assert_eq!(nret, 0);
}
