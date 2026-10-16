#![cfg(feature = "client-wowforever")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn forever_documented_events_register_and_reject_unknown_names() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local frame = CreateFrame("Frame")
        for _, event in ipairs({"DISCORD_SERVER_LIST_UPDATE", "LEGACY_FRIEND_SYSTEM_STATUS_UPDATED"}) do
            frame:RegisterEvent(event)
            assert(frame:IsEventRegistered(event))
            frame:UnregisterEvent(event)
            assert(not frame:IsEventRegistered(event))
        end
        assert(not pcall(frame.RegisterEvent, frame, "FOREVER_INVENTED_EVENT"))
    "#).unwrap();
}

#[test]
fn forever_guild_and_friends_documented_events_register() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local frame = CreateFrame("Frame")
        local rejected = {}
        for _, event in ipairs({
            "DISCORD_GUILD_LOBBY_UPDATE",
            "DISCORD_GUILD_SETTINGS_UPDATE",
            "SOCIAL_UI_FRIENDS_LIST_SYSTEM_STATUS_UPDATED",
            "NEW_MATCHMAKING_PARTY_INVITE",
        }) do
            local ok = pcall(frame.RegisterEvent, frame, event)
            if not ok then
                rejected[#rejected + 1] = event
            else
                assert(frame:IsEventRegistered(event))
                frame:UnregisterEvent(event)
                assert(not frame:IsEventRegistered(event))
            end
        end
        assert(not pcall(frame.RegisterEvent, frame, "FOREVER_INVENTED_EVENT"))
        assert(#rejected == 0, table.concat(rejected, ", "))
    "#,
    )
    .unwrap();
}

#[test]
fn forever_gamepad_override_onload_initializes_mapping() {
    wow_ui_sim::xml::clear_templates();
    let cache = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &cache,
        &["Blizzard_ActionBar", "Blizzard_GamepadActionBars"],
        &[],
    );
    env.exec(
        r#"
        local anchor = GamepadMainActionBarFramePageUnit.TopCenteredAnchor
        local failures = {}
        for _, entry in ipairs({
            {"PossessBar", "GamepadPossessBarOverride", "GAMEPAD_POSSESS_BAR_OVERRIDE_CHANGED"},
            {"StanceBar", "GamepadStanceBarOverride", "GAMEPAD_STANCE_BAR_OVERRIDE_CHANGED"},
        }) do
            local bar = assert(anchor[entry[1]], entry[1])
            assert(bar.overrideCVar == entry[2])
            assert(bar.overrideCVarChangedEvent == entry[3])
            local ok, err = pcall(GamepadOverrideBarMixin.OnLoad, bar)
            if not ok then
                failures[#failures + 1] = tostring(err)
            else
                assert(bar:IsEventRegistered(entry[3]))
                assert(bar:IsEventRegistered("PLAYER_SOFT_ENEMY_CHANGED"))
                assert(type(bar.overrideMap) == "table" and next(bar.overrideMap) == nil)
                assert(bar.isOverrideBarActive == false)
                local owner = CreateFrame("Frame")
                bar.pagingUnitOwner = owner
                bar:SetOverrideMapping(1, function(page) assert(page == owner); return anchor end, 2)
                assert(bar:GetLinkedOverrideBarAnchorFrame("1") == anchor)
                assert(bar:GetLinkedOverrideBarPage("1") == 2)
                bar:UnregisterEvent(entry[3])
                assert(not bar:IsEventRegistered(entry[3]))
            end
        end
        assert(#failures == 0, table.concat(failures, "\n"))
    "#,
    )
    .unwrap();
}

#[test]
fn forever_aura_styles_support_deprecated_vendor_aliases() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local styles = Enum.CustomAuraButtonDispelTypeTextureStyle
        for index, name in ipairs({"Border", "BorderWithIcon", "Icon", "PreserveAsset", "CustomAsset"}) do
            assert(styles[name] == index - 1)
        end
        local meta = Enum.CustomAuraButtonDispelTypeTextureStyleMeta
        assert(meta.MinValue == 0 and meta.MaxValue == 4 and meta.NumValues == 5)
    "#).unwrap();
    let source = std::fs::read_to_string(
        wow_ui_sim::blizzard_ui_sync::default_cache_addons_path()
            .unwrap()
            .join("Blizzard_Deprecated/Shared/Deprecated_12_1_0.lua"),
    )
    .unwrap();
    env.exec(&source).unwrap();
    env.exec("assert(AuraButtonBorderStyle.Atlas == 1 and AuraButtonBorderStyle.Color == 3)")
        .unwrap();
}

#[test]
fn forever_curio_rarity_supports_epic_tier_two_keys() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local rarity = Enum.CurioRarity
        assert(rarity.Common == 1 and rarity.Uncommon == 2 and rarity.Rare == 3)
        assert(rarity.Epic == 4 and rarity.EpicTier2 == 5)
        local colors = {[rarity.EpicTier2] = "epic-tier-two"}
        assert(colors[5] == "epic-tier-two")
        local meta = Enum.CurioRarityMeta
        assert(meta.MinValue == 1 and meta.MaxValue == 5 and meta.NumValues == 5)
    "#,
    )
    .unwrap();
}

#[test]
fn forever_player_storage_original_binding_consumer_uses_relative_offsets() {
    let env = WowLuaEnv::new().unwrap();
    let cache = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let path = cache.join("Blizzard_GamepadActionBars/ActionBindingUtil.lua");
    let source = std::fs::read_to_string(&path).unwrap();
    env.exec(&source).unwrap();
    env.exec(
        r#"
        local map = GamepadActionBarBindingUtil.GetGamepadStorageSlotIndexFromPageAndPageUnitSlotID
        for slot = 5, 8 do
            assert(map(1, slot) == nil, "reserved slot must not allocate storage")
        end
        assert(map(4, 1) == nil, "special page must not allocate standard storage")
        local ok, first = pcall(map, 1, 1)
        assert(ok, "original binding consumer failed: " .. tostring(first))
        assert(type(first) == "number" and first >= 1 and first == math.floor(first))
        assert(first == C_GamepadUI.GetFirstGamepadActionStorageSlotIndex())
        for _, case in ipairs({{1, 9, 4}, {1, 32, 27}, {2, 1, 28}, {3, 32, 83}}) do
            assert(map(case[1], case[2]) == first + case[3], "wrong relative storage slot")
        end
        "#,
    )
    .unwrap();
}

#[test]
fn forever_player_storage_normal_xml_initializes_both_page_units() {
    wow_ui_sim::xml::clear_templates();
    let cache = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let env = crate::common::blizzard_addon_harness::new_blizzard_addon_env(&cache);
    // Explicit test inputs before loading; no getter replacement or lifecycle replay.
    env.exec(
        r#"
        assert(SetCVar("GamepadPossessBarOverride", "4"))
        assert(SetCVar("GamepadStanceBarOverride", "3"))
        "#,
    )
    .unwrap();
    let loaded = crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env(
        &env,
        &cache,
        &["Blizzard_ActionBar", "Blizzard_GamepadActionBars"],
        &[],
    );
    assert!(
        loaded
            .iter()
            .any(|name| name == "Blizzard_GamepadActionBars")
    );
    assert_forever_player_storage_page_units(&env);
    for event in ["UPDATE_BONUS_ACTIONBAR", "PLAYER_ENTERING_WORLD"] {
        env.fire_event(event).unwrap();
        assert_forever_player_storage_page_units(&env);
    }
}

#[test]
fn forever_player_storage_disjoint_actions_preserve_player_and_pet_slots() {
    use wow_ui_sim::lua_api::state::PetActionSlot;

    wow_ui_sim::xml::clear_templates();
    let cache = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &cache,
        &["Blizzard_ActionBar", "Blizzard_GamepadActionBars"],
        &[],
    );
    assert!(wow_ui_sim::spells::get_spell(19750).is_some());
    {
        let mut state = env.state().borrow_mut();
        state.action_bars.insert(1, 6603);
        state.pet_actions[0] = PetActionSlot {
            has_action: true,
            name: Some("Pet control".into()),
            spell_id: Some(16827),
            ..PetActionSlot::default()
        };
    }
    env.exec(
        r#"
        local base = C_GamepadUI.GetFirstGamepadActionStorageSlotIndex()
        assert(base == 4096, "simulator-owned disjoint player base")
        local playerBefore = {}
        for slot = 1, base - 1 do
            playerBefore[slot] = {GetActionInfo(slot)}
        end
        local petBase = C_GamepadUI.GetFirstGamepadPetActionStorageSlotIndex()
        assert(petBase == 1)
        local petName, _, _, _, _, _, petSpell = GetPetActionInfo(petBase)
        assert(petName == "Pet control" and petSpell == 16827)

        local map = GamepadActionBarBindingUtil.GetGamepadStorageSlotIndexFromPageAndPageUnitSlotID
        local count = 0
        for page = 1, 3 do
            for position = 1, 32 do
                local slot = map(page, position)
                if slot then
                    assert(slot == base + count, "contiguous 84-slot allocation")
                    assert(GetActionInfo(slot) == nil)
                    PickupSpell(19750)
                    PlaceAction(slot)
                    local kind, id = GetActionInfo(slot)
                    assert(kind == "spell" and id == 19750)
                    count = count + 1
                end
            end
        end
        assert(count == 84)
        for _, name in ipairs({"GamepadMainActionBarFramePageUnit", "GamepadActionBarEditFramePageUnit"}) do
            local button = _G[name].actionBars.topBar:GetActionButtonByIndex(1)
            assert(button:GetID() == base)
            local kind, id = GetActionInfo(button:GetID())
            assert(kind == "spell" and id == 19750)
            assert(button:HasAction(), "original player button must see stored spell")
        end
        for slot = base, base + count - 1 do
            PickupAction(slot)
            assert(GetActionInfo(slot) == nil)
            local kind, id = GetCursorInfo()
            assert(kind == "spell" and id == 19750)
            PlaceAction(slot)
            local storedKind, storedID = GetActionInfo(slot)
            assert(storedKind == "spell" and storedID == 19750)
            assert(GetCursorInfo() == nil)
        end
        for slot = 1, base - 1 do
            local before = playerBefore[slot]
            local kind, id, subtype = GetActionInfo(slot)
            assert(kind == before[1] and id == before[2] and subtype == before[3], "player alias at " .. slot)
        end
        local name, _, _, _, _, _, spell = GetPetActionInfo(petBase)
        assert(name == petName and spell == petSpell, "player writes changed pet storage")
        "#,
    )
    .unwrap();
}

fn assert_forever_player_storage_page_units(env: &WowLuaEnv) {
    let root_errors: Vec<String> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|error| {
            error.contains("Blizzard_GamepadActionBars/ActionBindingUtil.lua:")
                || error.contains("Blizzard_GamepadActionBars/GamepadOverrideBarMixin.lua:")
        })
        .cloned()
        .collect();
    assert!(root_errors.is_empty(), "{}", root_errors.join("\n"));
    env.exec(
        r#"
        assert(not C_ActionBar.HasBonusActionBar(), "fixture requires no bonus action bar")
        local names = {"GamepadMainActionBarFramePageUnit", "GamepadActionBarEditFramePageUnit"}
        for _, name in ipairs(names) do
            local page = assert(_G[name], name)
            assert(page:GetCurrentPage() == 1, name)
            local possess = assert(page.actionBars.possessBar)
            local stance = assert(page.actionBars.stanceBar)
            assert(possess:GetParent() == page.BottomCenteredAnchor, name)
            assert(stance:GetParent() == page.RightCenteredAnchor, name)
            for _, entry in ipairs({
                {possess, 4, page.BottomCenteredAnchor},
                {stance, 3, page.RightCenteredAnchor},
            }) do
                local bar, override, anchor = unpack(entry)
                assert(bar:GetLinkedOverrideBarAnchorFrame(override) == anchor, name)
                assert(bar:GetLinkedOverrideBarPage(override) == 1, name)
                assert(bar:GetActionBarLinkedWithOverrideBar() == anchor.Bar, name)
            end
            assert(not stance:IsOverrideBarActive(), name)
            assert(not stance:IsShown(), name)
            local button = page.actionBars.topBar:GetActionButtonByIndex(1)
            local expected = GamepadActionBarBindingUtil.GetGamepadStorageSlotIndexFromPageAndPageUnitSlotID(1, 1)
            assert(button:GetID() == expected, name)
        end
        "#,
    )
    .unwrap();
}
