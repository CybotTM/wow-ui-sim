//! Tests for transmog and heirloom collection APIs:
//! C_TransmogCollection, C_Transmog, TransmogUtil, C_Heirloom, C_TransmogSets.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// C_TransmogCollection
// ============================================================================

#[test]
fn test_transmog_collection_get_appearance_sources() {
    let env = env();
    // visual_id 1 is the first Head appearance in defaults
    let result: String = env
        .eval(
            r#"
            local sources = C_TransmogCollection.GetAppearanceSources(1)
            if #sources ~= 1 then return "count=" .. #sources end
            local s = sources[1]
            if s.sourceID ~= 1 then return "sourceID=" .. tostring(s.sourceID) end
            if s.visualID ~= 1 then return "visualID=" .. tostring(s.visualID) end
            if s.categoryID ~= 1 then return "categoryID=" .. tostring(s.categoryID) end
            if not s.isCollected then return "not collected" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_transmog_collection_get_appearance_sources_empty() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TransmogCollection.GetAppearanceSources(99999)")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_transmog_collection_get_source_info() {
    let env = env();
    // source_id 1 is the first Head appearance in defaults
    let (source_id, is_collected): (i32, bool) = env
        .eval(
            r#"
            local info = C_TransmogCollection.GetSourceInfo(1)
            return info.sourceID, info.isCollected
            "#,
        )
        .unwrap();
    assert_eq!(source_id, 1);
    assert!(is_collected);
}

#[test]
fn test_transmog_collection_player_has_transmog() {
    let env = env();
    let has: bool = env
        .eval("return C_TransmogCollection.PlayerHasTransmog(1, 0)")
        .unwrap();
    assert!(!has);
}

#[test]
fn test_transmog_collection_player_has_transmog_by_item_info() {
    let env = env();
    let has: bool = env
        .eval("return C_TransmogCollection.PlayerHasTransmogByItemInfo('item:1')")
        .unwrap();
    assert!(!has);
}

#[test]
fn test_transmog_collection_player_has_transmog_item_modified_appearance() {
    let env = env();
    let has: bool = env
        .eval("return C_TransmogCollection.PlayerHasTransmogItemModifiedAppearance(1)")
        .unwrap();
    assert!(!has);
}

#[test]
fn test_transmog_add_then_has() {
    let env = env();
    env.exec("A_Admin.AddTransmog(42)").unwrap();
    let has: bool = env
        .eval("return C_TransmogCollection.PlayerHasTransmog(42, 0)")
        .unwrap();
    assert!(has, "Should have transmog after AddTransmog");
}

#[test]
fn test_transmog_add_then_has_by_item_info() {
    let env = env();
    env.exec("A_Admin.AddTransmog(99)").unwrap();
    let has: bool = env
        .eval("return C_TransmogCollection.PlayerHasTransmogByItemInfo('item:99:0')")
        .unwrap();
    assert!(has, "Should find transmog via item info link");
}

#[test]
fn test_transmog_add_then_has_modified_appearance() {
    let env = env();
    env.exec("A_Admin.AddTransmog(77)").unwrap();
    let has: bool = env
        .eval("return C_TransmogCollection.PlayerHasTransmogItemModifiedAppearance(77)")
        .unwrap();
    assert!(has, "Should find transmog via modified appearance ID");
}

#[test]
fn test_transmog_remove_then_not_has() {
    let env = env();
    env.exec("A_Admin.AddTransmog(42)").unwrap();
    env.exec("A_Admin.RemoveTransmog(42)").unwrap();
    let has: bool = env
        .eval("return C_TransmogCollection.PlayerHasTransmog(42, 0)")
        .unwrap();
    assert!(!has, "Should not have transmog after RemoveTransmog");
}

#[test]
fn test_admin_add_transmog_appearance() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local before = #C_TransmogCollection.GetCategoryAppearances(1, nil)
            A_Admin.AddTransmogAppearance(9999, 1, 77777)
            local after = #C_TransmogCollection.GetCategoryAppearances(1, nil)
            if after ~= before + 1 then return "count: " .. before .. " -> " .. after end
            local info = C_TransmogCollection.GetSourceInfo(9999)
            if info.sourceID ~= 9999 then return "sourceID=" .. tostring(info.sourceID) end
            if info.categoryID ~= 1 then return "categoryID=" .. tostring(info.categoryID) end
            if info.itemID ~= 77777 then return "itemID=" .. tostring(info.itemID) end
            if not info.isCollected then return "not collected" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_transmog_uncollected_returns_false() {
    let env = env();
    env.exec("A_Admin.AddTransmog(42)").unwrap();
    let has: bool = env
        .eval("return C_TransmogCollection.PlayerHasTransmog(999, 0)")
        .unwrap();
    assert!(!has, "Different ID should not match");
}

#[test]
fn test_transmog_collection_get_item_info_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_TransmogCollection.GetItemInfo(1) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_transmog_collection_get_num_transmog_sources() {
    let env = env();
    let count: i32 = env
        .eval("return C_TransmogCollection.GetNumTransmogSources()")
        .unwrap();
    assert_eq!(count, 60, "default world has 60 transmog appearances");
}

#[test]
fn test_transmog_collection_get_all_appearance_sources_empty() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TransmogCollection.GetAllAppearanceSources(1)")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_transmog_collection_get_illusions_empty() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TransmogCollection.GetIllusions()")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_transmog_collection_get_outfits_empty() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TransmogCollection.GetOutfits()")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_transmog_collection_get_num_max_outfits() {
    let env = env();
    let count: i32 = env
        .eval("return C_TransmogCollection.GetNumMaxOutfits()")
        .unwrap();
    assert_eq!(count, 20);
}

#[test]
fn test_transmog_collection_get_outfit_info_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_TransmogCollection.GetOutfitInfo(1) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_transmog_collection_get_appearance_camera_id() {
    let env = env();
    let id: i32 = env
        .eval("return C_TransmogCollection.GetAppearanceCameraID(1)")
        .unwrap();
    assert_eq!(id, 0);
}

#[test]
fn test_transmog_collection_get_category_appearances() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local appearances = C_TransmogCollection.GetCategoryAppearances(1, nil)
            if #appearances ~= 5 then return "count=" .. #appearances end
            local a = appearances[1]
            if not a.visualID then return "no visualID" end
            if a.isCollected == nil then return "no isCollected" end
            if a.uiOrder ~= 1 then return "uiOrder=" .. tostring(a.uiOrder) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_transmog_collection_get_category_appearances_empty() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TransmogCollection.GetCategoryAppearances(99, nil)")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_transmog_collection_get_category_info_armor() {
    let env = env();
    let (name, is_weapon): (String, bool) = env
        .eval("return C_TransmogCollection.GetCategoryInfo(1)")
        .unwrap();
    assert_eq!(name, "Head");
    assert!(!is_weapon);
}

#[test]
fn test_transmog_collection_get_category_info_weapon() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local name, isWeapon, canEnchant, canMainHand, canOffHand = C_TransmogCollection.GetCategoryInfo(14)
            if name ~= "One-Handed Swords" then return "name=" .. tostring(name) end
            if not isWeapon then return "not weapon" end
            if not canEnchant then return "not enchantable" end
            if not canMainHand then return "not mainhand" end
            if not canOffHand then return "not offhand" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_transmog_collection_get_category_info_shield() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local name, isWeapon, canEnchant, canMainHand, canOffHand = C_TransmogCollection.GetCategoryInfo(18)
            if not isWeapon then return "not weapon" end
            if canEnchant then return "enchantable" end
            if canMainHand then return "mainhand" end
            if not canOffHand then return "not offhand" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_transmog_collection_player_knows_source() {
    let env = env();
    let knows: bool = env
        .eval("return C_TransmogCollection.PlayerKnowsSource(1)")
        .unwrap();
    assert!(!knows);
}

#[test]
fn test_transmog_collection_is_appearance_hidden_visual() {
    let env = env();
    let hidden: bool = env
        .eval("return C_TransmogCollection.IsAppearanceHiddenVisual(1)")
        .unwrap();
    assert!(!hidden);
}

#[test]
fn test_transmog_collection_is_source_type_filter_checked() {
    let env = env();
    let checked: bool = env
        .eval("return C_TransmogCollection.IsSourceTypeFilterChecked(1)")
        .unwrap();
    assert!(checked);
}

#[test]
fn test_transmog_collection_get_show_missing_source_in_item_tooltips() {
    let env = env();
    let show: bool = env
        .eval("return C_TransmogCollection.GetShowMissingSourceInItemTooltips()")
        .unwrap();
    assert!(show);
}

// ============================================================================
// C_Transmog
// ============================================================================

#[test]
fn test_transmog_get_all_set_appearances_by_id_empty() {
    let env = env();
    let count: i32 = env
        .eval("return #C_Transmog.GetAllSetAppearancesByID(1)")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_transmog_get_applied_source_id_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Transmog.GetAppliedSourceID(1) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_admin_set_transmog_for_slot() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- No transmog applied initially
            if C_Transmog.GetAppliedSourceID(1) ~= nil then return "not nil initially" end
            A_Admin.SetTransmogForSlot(1, 42)
            local id = C_Transmog.GetAppliedSourceID(1)
            if id ~= 42 then return "id=" .. tostring(id) end
            -- Other slots unaffected
            if C_Transmog.GetAppliedSourceID(2) ~= nil then return "slot 2 affected" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_transmog_applied_altered_appearance_and_creature_display_lookup() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_Transmog.GetAppliedAlteredAppearance(1) ~= nil then return "not nil initially" end
            A_Admin.SetTransmogForSlot(1, 42)
            local altered = C_Transmog.GetAppliedAlteredAppearance(1)
            if altered ~= 42 then return "altered=" .. tostring(altered) end
            local displayID = C_Transmog.GetCreatureDisplayIDForSource(42)
            if displayID ~= 42 then return "displayID=" .. tostring(displayID) end
            if C_Transmog.GetCreatureDisplayIDForSource(999999) ~= nil then return "unknown source not nil" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_transmog_player_has_transmog_by_item_info_and_is_at_npc() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_Transmog.IsAtTransmogNPC() then return "at npc unexpectedly" end
            if not C_Transmog.PlayerHasTransmogByItemInfo("item:31110") then return "missing collected item" end
            if C_Transmog.PlayerHasTransmogByItemInfo("item:99989") then return "found uncollected item" end
            A_Admin.AddTransmog(77)
            if not C_Transmog.PlayerHasTransmogByItemInfo("item:77:0") then return "missing collected source id" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_transmog_get_slot_info() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local isTransmogrified, hasPending, isPendingCollected,
                  canTransmogrify, cannotTransmogrifyReason, hasUndo = C_Transmog.GetSlotInfo(1)
            if isTransmogrified then return "transmogrified" end
            if hasPending then return "pending" end
            if type(hasUndo) ~= "boolean" then return "hasUndo type=" .. type(hasUndo) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

// ============================================================================
// TransmogUtil
// ============================================================================

#[test]
fn test_transmog_util_get_transmog_location() {
    let env = env();
    let (slot_name, transmog_type, modification): (String, i32, i32) = env
        .eval(
            r#"
            local loc = TransmogUtil.GetTransmogLocation("HeadSlot", 0, 0)
            return loc.slotName, loc.transmogType, loc.modification
            "#,
        )
        .unwrap();
    assert_eq!(slot_name, "HeadSlot");
    assert_eq!(transmog_type, 0);
    assert_eq!(modification, 0);
}

#[test]
fn test_transmog_util_get_transmog_location_boolean_modification() {
    let env = env();
    let (slot_name, modification): (String, i32) = env
        .eval(
            r#"
            local loc = TransmogUtil.GetTransmogLocation("HEADSLOT", 0, false)
            return loc.slotName, loc.modification
            "#,
        )
        .unwrap();
    assert_eq!(slot_name, "HEADSLOT");
    assert_eq!(modification, 0);
}

#[test]
fn test_transmog_location_methods() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local loc = TransmogUtil.GetTransmogLocation("HEADSLOT", 0, false)
            if not loc:IsAppearance() then return "not appearance" end
            if loc:IsIllusion() then return "is illusion" end
            if loc:IsEitherHand() then return "is hand" end
            if loc:IsSecondary() then return "is secondary" end
            if loc:GetSlotName() ~= "HEADSLOT" then return "bad slot" end
            local loc2 = TransmogUtil.GetTransmogLocation("HEADSLOT", 0, false)
            if not loc:IsEqual(loc2) then return "not equal" end
            local loc3 = TransmogUtil.GetTransmogLocation("MAINHANDSLOT", 0, false)
            if not loc3:IsMainHand() then return "not mainhand" end
            if not loc3:IsEitherHand() then return "not either hand" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "TransmogLocation methods: {result}");
}

#[test]
fn test_transmog_util_create_transmog_location() {
    let env = env();
    let (slot_id, transmog_type, modification): (i32, i32, i32) = env
        .eval(
            r#"
            local loc = TransmogUtil.CreateTransmogLocation(1, 0, 0)
            return loc.slotID, loc.transmogType, loc.modification
            "#,
        )
        .unwrap();
    assert_eq!(slot_id, 1);
    assert_eq!(transmog_type, 0);
    assert_eq!(modification, 0);
}

#[test]
fn test_transmog_util_get_best_item_modified_appearance_id_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return TransmogUtil.GetBestItemModifiedAppearanceID(nil) == nil")
        .unwrap();
    assert!(is_nil);
}

// ============================================================================
// C_Heirloom
// ============================================================================

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

// ============================================================================
// C_TransmogSets
// ============================================================================

#[test]
fn test_transmog_sets_get_base_set_id() {
    let env = env();
    let id: i32 = env.eval("return C_TransmogSets.GetBaseSetID(1)").unwrap();
    assert_eq!(id, 0);
}

#[test]
fn test_transmog_sets_get_variant_sets_empty() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TransmogSets.GetVariantSets(1)")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_transmog_sets_get_set_info() {
    let env = env();
    let (set_id, name, collected): (i32, String, bool) = env
        .eval(
            r#"
            local info = C_TransmogSets.GetSetInfo(1)
            return info.setID, info.name, info.collected
            "#,
        )
        .unwrap();
    assert_eq!(set_id, 0);
    assert_eq!(name, "");
    assert!(!collected);
}

#[test]
fn test_transmog_sets_get_set_primary_appearances_empty() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TransmogSets.GetSetPrimaryAppearances(1)")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_transmog_sets_get_all_sets_empty() {
    let env = env();
    let count: i32 = env.eval("return #C_TransmogSets.GetAllSets()").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_transmog_sets_get_usable_sets_empty() {
    let env = env();
    let count: i32 = env.eval("return #C_TransmogSets.GetUsableSets()").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_transmog_sets_has_available_sets_false_when_set_inventory_is_empty() {
    let env = env();
    let has_available_sets: bool = env
        .eval("return C_TransmogSets.HasAvailableSets()")
        .unwrap();
    assert!(!has_available_sets);
}

#[test]
fn test_transmog_sets_is_base_set_collected() {
    let env = env();
    let collected: bool = env
        .eval("return C_TransmogSets.IsBaseSetCollected(1)")
        .unwrap();
    assert!(!collected);
}

#[test]
fn test_transmog_sets_get_sources_for_slot_empty() {
    let env = env();
    let count: i32 = env
        .eval("return #C_TransmogSets.GetSourcesForSlot(1, 1)")
        .unwrap();
    assert_eq!(count, 0);
}
