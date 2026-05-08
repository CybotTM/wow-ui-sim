use super::*;

#[test]
fn test_heirloom_get_heirloom_info_first_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Heirloom.GetHeirloomInfo(1) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_heirloom_get_heirloom_info_known_item() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local name, equipLoc, isPvP, texture, upgradeLevel, source,
                  searchFiltered, effectiveLevel, minLevel, maxLevel
                  = C_Heirloom.GetHeirloomInfo(122245)
            if name ~= "Burnished Helm of Might" then return "name=" .. tostring(name) end
            if equipLoc ~= "INVTYPE_HEAD" then return "loc=" .. tostring(equipLoc) end
            if isPvP then return "isPvP" end
            if upgradeLevel ~= 6 then return "upgrade=" .. tostring(upgradeLevel) end
            if source ~= "Vendor" then return "source=" .. tostring(source) end
            if minLevel ~= 1 then return "min=" .. tostring(minLevel) end
            if maxLevel ~= 50 then return "max=" .. tostring(maxLevel) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_heirloom_get_heirloom_max_upgrade_level() {
    let env = env();
    let level: i32 = env
        .eval("return C_Heirloom.GetHeirloomMaxUpgradeLevel(1)")
        .unwrap();
    assert_eq!(level, 0);
}

#[test]
fn test_heirloom_get_num_heirlooms() {
    let env = env();
    let count: i32 = env.eval("return C_Heirloom.GetNumHeirlooms()").unwrap();
    assert_eq!(count, 11, "default world has 11 heirlooms");
}

#[test]
fn test_heirloom_get_num_known_heirlooms() {
    let env = env();
    let count: i32 = env
        .eval("return C_Heirloom.GetNumKnownHeirlooms()")
        .unwrap();
    assert_eq!(count, 11, "all default heirlooms are collected");
}

#[test]
fn test_heirloom_get_num_displayed_heirlooms() {
    let env = env();
    let count: i32 = env
        .eval("return C_Heirloom.GetNumDisplayedHeirlooms()")
        .unwrap();
    assert_eq!(count, 11);
}

#[test]
fn test_heirloom_get_item_id_from_displayed_index() {
    let env = env();
    let id: i32 = env
        .eval("return C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(1)")
        .unwrap();
    assert_eq!(id, 122245, "first heirloom is Burnished Helm of Might");
    let zero: i32 = env
        .eval("return C_Heirloom.GetHeirloomItemIDFromDisplayedIndex(99)")
        .unwrap();
    assert_eq!(zero, 0, "out of range returns 0");
}

#[test]
fn test_heirloom_player_has_heirloom_unknown() {
    let env = env();
    let has: bool = env.eval("return C_Heirloom.PlayerHasHeirloom(1)").unwrap();
    assert!(!has, "unknown item ID should not be owned");
}

#[test]
fn test_heirloom_player_has_heirloom_collected() {
    let env = env();
    let has: bool = env
        .eval("return C_Heirloom.PlayerHasHeirloom(122245)")
        .unwrap();
    assert!(has, "Burnished Helm should be collected by default");
}

#[test]
fn test_heirloom_get_heirloom_link_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Heirloom.GetHeirloomLink(1) == nil")
        .unwrap();
    assert!(is_nil, "unknown item should return nil");
}

#[test]
fn test_heirloom_get_heirloom_link_known() {
    let env = env();
    let link: String = env
        .eval("return C_Heirloom.GetHeirloomLink(122245)")
        .unwrap();
    assert!(link.contains("Burnished Helm of Might"));
    assert!(link.contains("|Hitem:122245"));
}

#[test]
fn test_heirloom_filter_stubs() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if not C_Heirloom.GetCollectedHeirloomFilter() then return "collected not true" end
            if not C_Heirloom.GetUncollectedHeirloomFilter() then return "uncollected not true" end
            C_Heirloom.SetCollectedHeirloomFilter(false)
            C_Heirloom.SetUncollectedHeirloomFilter(false)
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_admin_collect_uncollect_heirloom() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- 99999 is not collected
            if C_Heirloom.PlayerHasHeirloom(99999) then return "already has" end
            A_Admin.CollectHeirloom(99999)
            if not C_Heirloom.PlayerHasHeirloom(99999) then return "not collected" end
            A_Admin.UncollectHeirloom(99999)
            if C_Heirloom.PlayerHasHeirloom(99999) then return "still collected" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_heirloom_can_heirloom_upgrade_from_pending() {
    let env = env();
    let can: bool = env
        .eval("return C_Heirloom.CanHeirloomUpgradeFromPending(1)")
        .unwrap();
    assert!(!can);
}

#[test]
fn test_heirloom_get_class_and_spec_filters() {
    let env = env();
    let (class_filter, spec_filter): (i32, i32) = env
        .eval("return C_Heirloom.GetClassAndSpecFilters()")
        .unwrap();
    assert_eq!(class_filter, 0);
    assert_eq!(spec_filter, 0);
}

#[test]
fn test_heirloom_source_filter_surface() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_HeirloomInfo.IsHeirloomSourceValid(1) then return "source valid" end
            if not C_Heirloom.GetHeirloomSourceFilter(1) then return "source unchecked" end
            C_Heirloom.SetHeirloomSourceFilter(1, false)
            C_HeirloomInfo.SetAllSourceFilters(true)
            C_HeirloomInfo.SetDefaultFilters()
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}
