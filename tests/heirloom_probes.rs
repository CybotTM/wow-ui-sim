//! Tests for `C_Heirloom` probes backed by `WorldState.heirlooms`:
//!
//! - `C_Heirloom.GetHeirloomInfo(itemID)` — 10 retail return values.
//! - `C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(index)` — 1-based
//!   lookup, `0` out of range. Replaces the dead
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
fn get_heirloom_item_id_returns_zero_out_of_range() {
    let env = env();
    let (zero, negative, big): (i32, i32, i32) = env
        .eval(
            r#"
            return C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(0),
                   C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(-1),
                   C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(9999)
            "#,
        )
        .unwrap();
    assert_eq!(zero, 0, "zero index is out of range (1-based)");
    assert_eq!(negative, 0);
    assert_eq!(big, 0);
}

#[test]
fn set_heirloom_by_item_id_populates_game_tooltip() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            GameTooltip:ClearLines()
            GameTooltip:SetHeirloomByItemID(122245)
            if GameTooltip:NumLines() == 0 then
                return "missing lines"
            end
            local left = GameTooltip:GetLeftLine(1)
            return left and left:GetText() or "missing first line"
            "#,
        )
        .unwrap();

    assert_eq!(result, "Polished Helm of Valor");
}

#[test]
fn create_heirloom_adds_collected_item_to_bag() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.ClearBags()
            C_Heirloom.CreateHeirloom(122245)
            local info = C_Container.GetContainerItemInfo(0, 1)
            if not info then return "missing item" end
            if info.itemID ~= 122245 then return "itemID=" .. tostring(info.itemID) end
            if info.stackCount ~= 1 then return "stack=" .. tostring(info.stackCount) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn create_heirloom_ignores_uncollected_item() {
    let env = env();
    let is_empty: bool = env
        .eval(
            r#"
            A_Admin.ClearBags()
            A_Admin.UncollectHeirloom(122245)
            C_Heirloom.CreateHeirloom(122245)
            return C_Container.GetContainerItemInfo(0, 1) == nil
            "#,
        )
        .unwrap();
    assert!(
        is_empty,
        "uncollected heirlooms should not create a bag copy"
    );
}
