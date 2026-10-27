#![cfg(feature = "client-wrath")]

//! Contracts from native 12340 (laptop answer #7), not modern-client parity.

use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_wrath_inventory(env: &WowLuaEnv) {
    common::load_profile_framexml(env);
    {
        let mut state = env.state().borrow_mut();
        state.player.equipped_items.clear();
        state.bag_items.clear();
        state.equippable_items.clear();
        // Existing explicit equip eligibility, not invented proficiency/level state.
        state.equippable_items.extend([49803, 49812, 49819]);
        state.bank_frame_open = false;
        state
            .bag_info
            .get_mut(&5)
            .expect("default bag metadata includes bank bag 5")
            .num_slots = 16;
    }
    env.exec(
        r#"
        -- Catalog fixtures: two rings and a head item, all marked equippable.
        assert(select(9, GetItemInfo(49803)) == "INVTYPE_FINGER")
        assert(select(9, GetItemInfo(49812)) == "INVTYPE_FINGER")
        assert(select(9, GetItemInfo(49819)) == "INVTYPE_HEAD")
        A_Admin.EquipItem(11, 49812)
        A_Admin.EquipItem(12, 49803)
        A_Admin.AddBagItem(0, 1, 49803)
        A_Admin.AddBagItem(1, 2, 49803)
        A_Admin.AddBagItem(0, 3, 49819)
        A_Admin.AddBagItem(5, 1, 49812) -- populated bank bag; banker is closed
        assert(GetContainerItemID(5, 1) == 49812, "bank candidate exists in modeled storage")
        GearManagerDialog:Hide()
        assert(CharacterFinger0Slot:GetID() == 11)
        assert(type(PaperDollFrameItemFlyout_Show) == "function")
        assert(type(EquipmentManager_EquipSet) == "function")
        "#,
    )
    .expect("selected Wrath FrameXML and catalog-backed inventory fixture must load");
}

#[test]
fn original_wrath_flyout_tracks_slot_candidates_by_location() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("initialize Wrath Lua environment");
        load_wrath_inventory(&env);
        env.exec(r##"
            local function findLocation(items, wantedSlot, wantedBag)
                for location in pairs(items) do
                    local player, bank, bags, slot, bag = EquipmentManager_UnpackLocation(location)
                    if not bank and slot == wantedSlot then
                        if wantedBag == nil and player and not bags then return location end
                        if wantedBag ~= nil and bags and bag == wantedBag then return location end
                    end
                end
            end
            local function readFlyout()
                local shown, count = {}, 0
                for _, button in ipairs(PaperDollFrameItemFlyout.buttons) do
                    local location = button.location
                    if button:IsShown() and location < PDFITEMFLYOUT_FIRST_SPECIAL_LOCATION then
                        shown[location] = true
                        count = count + 1
                    end
                end
                return shown, count
            end

            -- Invoke the unchanged selected consumer before examining its output.
            PaperDollFrameItemFlyout_Show(CharacterFinger0Slot)
            local shown, count = readFlyout()
            assert(count == 3, "flyout must show other equipped ring and both carried copies")
            assert(findLocation(shown, 12) and findLocation(shown, 1, 0) and findLocation(shown, 2, 1))
            assert(not findLocation(shown, 11), "consumer removes currently equipped slot")

            local items = {}
            local returned = GetInventoryItemsForSlot(11, items)
            assert(returned == items, "producer returns the supplied populated table")
            local current = assert(findLocation(items, 11), "current equipped location")
            local other = assert(findLocation(items, 12), "other equipped location")
            local carriedA = assert(findLocation(items, 1, 0), "backpack location")
            local carriedB = assert(findLocation(items, 2, 1), "carried bag location")
            assert(items[current] == 49812, "producer includes current equipped item")
            assert(items[other] == 49803, "other ring slot is eligible")
            assert(items[carriedA] == 49803 and items[carriedB] == 49803,
                "equal item IDs at different locations are not deduplicated")
            local total = 0
            for location, itemID in pairs(items) do
                assert(type(location) == "number" and type(itemID) == "number")
                total = total + 1
            end
            assert(total == 4, "head item and closed-bank bag item must be absent")
            assert(not findLocation(items, 3, 0), "equippable head item cannot enter ring slot")

            -- A second observation rejects a frozen catalog and no-op producer.
            A_Admin.RemoveBagItem(0, 1)
            local changed = {}
            assert(GetInventoryItemsForSlot(11, changed) == changed)
            assert(changed[carriedA] == nil and changed[carriedB] == 49803)
            PaperDollFrameItemFlyout_Show(CharacterFinger0Slot)
            local refreshed, remaining = readFlyout()
            assert(not refreshed[carriedA], "removed bag item must leave flyout")
            assert(refreshed[other] and refreshed[carriedB])
            assert(remaining == 2, "flyout refresh retains only live candidates")

            for _, slot in ipairs({-1, 0, 20}) do
                local outside = {}
                assert(select("#", GetInventoryItemsForSlot(slot, outside)) == 0,
                    "out-of-range numeric slots return no results")
                assert(next(outside) == nil)
            end
        "##).expect("native 12340 (laptop answer #7): original Wrath flyout must consume live candidates");
    }
}

#[test]
fn original_wrath_equipment_location_reads_socketed_gems() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("initialize Wrath Lua environment");
        load_wrath_inventory(&env);
        env.state().borrow_mut().player.equipped_items.get_mut(&12)
            .expect("fixture ring is equipped").gem_ids = [40111, 40112, 0];
        env.exec(r##"
            local id, _, _, _, _, _, _, _, _, _, _, _, gem1, gem2, gem3 =
                EquipmentManager_GetItemInfoByLocation(ITEM_INVENTORY_LOCATION_PLAYER + 12)
            assert(id == 49803)
            assert(gem1 == 40111 and gem2 == 40112 and gem3 == nil,
                "original equipment consumer must return supplied socketed gem IDs")
            assert(select("#", GetInventoryItemGems(19)) == 0,
                "empty equipment slot returns no gems")
        "##).expect("Wrath equipment location consumes modeled socketed gems");
    }
}

fn seed_bag_observations(env: &WowLuaEnv) {
    load_wrath_inventory(env);
    let mut state = env.state().borrow_mut();
    let item = state.bag_items.get_mut(&(0, 1)).expect("carried ring");
    item.hyperlink = Some("|Hitem:49803:0:40111:40112:0:0|h[Ring]|h".into());
    item.durability = Some((27, 83));
    state.item_cooldowns.insert(
        49803,
        wow_ui_sim::lua_api::state::SpellCooldownState {
            start: 11.0,
            duration: 31.0,
        },
    );
}

#[test]
fn wrath_carried_cooldown_projects_supplied_item_state() {
    test_timeout! {
        let env = WowLuaEnv::new().unwrap();
        seed_bag_observations(&env);
        env.exec("local start, duration = GetContainerItemCooldown(0, 1); assert(start == 11 and duration == 31)").unwrap();
    }
}

#[test]
fn wrath_carried_durability_projects_supplied_item_state() {
    test_timeout! {
        let env = WowLuaEnv::new().unwrap();
        seed_bag_observations(&env);
        env.exec("local current, maximum = GetContainerItemDurability(0, 1); assert(current == 27 and maximum == 83)").unwrap();
    }
}

#[test]
fn wrath_carried_gems_project_supplied_item_link() {
    test_timeout! {
        let env = WowLuaEnv::new().unwrap();
        seed_bag_observations(&env);
        env.exec("local first, second = GetContainerItemGems(0, 1); assert(first == 40111 and second == 40112)").unwrap();
    }
}

fn seed_two_bag_observations(env: &WowLuaEnv) {
    seed_bag_observations(env);
    let mut state = env.state().borrow_mut();
    let second = state.bag_items.get_mut(&(1, 2)).expect("second carried ring");
    second.item_id = 49812;
    second.hyperlink = Some("|Hitem:49812:0:40201:40202:0:0|h[Ring]|h".into());
    second.durability = Some((42, 91));
    state.item_cooldowns.insert(
        49812,
        wow_ui_sim::lua_api::state::SpellCooldownState { start: 19.0, duration: 47.0 },
    );
}

#[test]
fn original_wrath_bag_location_reads_cooldown() {
    test_timeout! {
        let env = WowLuaEnv::new().unwrap();
        seed_two_bag_observations(&env);
        env.exec(r#"
            -- Other missing producers are prerequisites for the consumer's return.
            GetContainerItemDurability = function() return 1, 1 end
            GetContainerItemGems = function() return 1, 2 end
            local function check(bag, slot, expectedStart, expectedDuration)
                local location = ITEM_INVENTORY_LOCATION_PLAYER + ITEM_INVENTORY_LOCATION_BAGS + bag * 256 + slot
                local _, _, _, _, _, _, _, _, start, duration = EquipmentManager_GetItemInfoByLocation(location)
                assert(start == expectedStart and duration == expectedDuration, "consumer cooldown mismatch")
            end
            check(0, 1, 11, 31)
            check(1, 2, 19, 47)
        "#).unwrap();
    }
}

#[test]
fn original_wrath_bag_location_reads_durability() {
    test_timeout! {
        let env = WowLuaEnv::new().unwrap();
        seed_two_bag_observations(&env);
        env.exec(r#"
            -- Isolate the later consumer boundary from the preceding missing producer.
            GetContainerItemCooldown = function() return 0, 0, 1 end
            GetContainerItemGems = function() return 1, 2 end
            local function check(bag, slot, expectedCurrent, expectedMaximum)
                local location = ITEM_INVENTORY_LOCATION_PLAYER + ITEM_INVENTORY_LOCATION_BAGS + bag * 256 + slot
                local _, _, _, _, current, maximum = EquipmentManager_GetItemInfoByLocation(location)
                assert(current == expectedCurrent and maximum == expectedMaximum, "consumer durability mismatch")
            end
            check(0, 1, 27, 83)
            check(1, 2, 42, 91)
        "#).unwrap();
    }
}

#[test]
fn original_wrath_bag_location_reads_gems() {
    test_timeout! {
        let env = WowLuaEnv::new().unwrap();
        seed_two_bag_observations(&env);
        env.exec(r#"
            -- Isolate the gem boundary from the two preceding missing producers.
            GetContainerItemCooldown = function() return 0, 0, 1 end
            GetContainerItemDurability = function() return 1, 1 end
            local function check(bag, slot, expectedFirst, expectedSecond)
                local location = ITEM_INVENTORY_LOCATION_PLAYER + ITEM_INVENTORY_LOCATION_BAGS + bag * 256 + slot
                local _, _, _, _, _, _, _, _, _, _, _, _, first, second = EquipmentManager_GetItemInfoByLocation(location)
                assert(first == expectedFirst and second == expectedSecond, "consumer gems mismatch")
            end
            check(0, 1, 40111, 40112)
            check(1, 2, 40201, 40202)
        "#).unwrap();
    }
}

#[test]
fn original_wrath_container_cooldown_displays_supplied_item_state() {
    test_timeout! {
        let env = WowLuaEnv::new().unwrap();
        seed_two_bag_observations(&env);
        env.exec(r#"
            local source = debug.getinfo(ContainerFrame_UpdateCooldown, "S").source
            assert(source:find("ContainerFrame.lua", 1, true), "selected original container consumer")
            local button = CreateFrame("Button", "WrathCooldownProbe", UIParent)
            button:CreateTexture("WrathCooldownProbeIconTexture")
            local cooldown = CreateFrame("Cooldown", "WrathCooldownProbeCooldown", button)
            local function check(bag, slot, expectedStart, expectedDuration)
                button:SetID(slot)
                ContainerFrame_UpdateCooldown(bag, button)
                local start, duration = cooldown:GetCooldownTimes()
                assert(start == expectedStart * 1000 and duration == expectedDuration * 1000,
                    "original container cooldown must show supplied item state")
            end
            check(0, 1, 11, 31)
            check(1, 2, 19, 47)
        "#).unwrap();
    }
}
