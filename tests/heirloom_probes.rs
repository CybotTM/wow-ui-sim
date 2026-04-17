//! Tests for `C_Heirloom` probes backed by `WorldState.heirlooms`:
//!
//! - `C_Heirloom.GetHeirloomInfo(itemID)` — 10 retail return values.
//! - `C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(index)` — 1-based
//!   lookup, nil out of range. Replaces the dead
//!   `GetHeirloomItemIDFromDisplayedSlot` stub entry.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_heirloom_info_returns_seeded_row_fields() {
    let env = env();
    let (name, equip_loc, is_pvp, texture, upgrade_level, source, search_filtered): (
        String,
        String,
        bool,
        i32,
        i32,
        String,
        bool,
    ) = env
        .eval(
            r#"
            local name, equipLoc, isPvP, texture, upgrade, source, filtered =
                C_Heirloom.GetHeirloomInfo(122245)
            return name, equipLoc, isPvP, texture, upgrade, source, filtered
            "#,
        )
        .unwrap();

    assert_eq!(name, "Burnished Helm of Might");
    assert_eq!(equip_loc, "INVTYPE_HEAD");
    assert!(!is_pvp);
    assert_eq!(texture, 133071);
    assert_eq!(upgrade_level, 6);
    assert_eq!(source, "Vendor");
    assert!(!search_filtered);
}

#[test]
fn get_heirloom_info_effective_min_max_levels_match_seed() {
    let env = env();
    let (effective_level, min_level, max_level): (i32, i32, i32) = env
        .eval(
            r#"
            local _, _, _, _, _, _, _, effective, minL, maxL =
                C_Heirloom.GetHeirloomInfo(122245)
            return effective, minL, maxL
            "#,
        )
        .unwrap();
    assert_eq!(effective_level, 50);
    assert_eq!(min_level, 1);
    assert_eq!(max_level, 50);
}

#[test]
fn get_heirloom_info_returns_nil_for_unknown_item() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Heirloom.GetHeirloomInfo(999999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn get_heirloom_item_id_from_displayed_index_returns_seeded_item() {
    let env = env();
    let first: i32 = env
        .eval("return C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(1)")
        .unwrap();
    assert_eq!(first, 122245, "first seeded heirloom is the Burnished Helm");
}

#[test]
fn get_heirloom_item_id_returns_nil_out_of_range() {
    let env = env();
    let (zero_nil, negative_nil, big_nil): (bool, bool, bool) = env
        .eval(
            r#"
            return C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(0) == nil,
                   C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(-1) == nil,
                   C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(9999) == nil
            "#,
        )
        .unwrap();
    assert!(zero_nil, "zero index is nil (1-based)");
    assert!(negative_nil);
    assert!(big_nil);
}
