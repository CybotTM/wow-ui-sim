//! Focused 12.1 service-payload compatibility contracts.

#[cfg(feature = "retail-12-1-0")]
use super::super::*;

#[cfg(feature = "retail-12-1-0")]
use crate::lua_api::state::{
    PlayerChoiceInfo, PlayerChoiceOptionButtonInfo, PlayerChoiceOptionInfo,
    PlayerChoiceOptionRewardInfo, PlayerChoiceRewardCurrencyInfo, PlayerChoiceRewardItemInfo,
    PlayerChoiceRewardReputationInfo,
};

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_battle_net_friend_level_enum() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local level = Enum.BattleNetFriendLevel
            local meta = Enum.BattleNetFriendLevelMeta
            if type(level) ~= "table" or type(meta) ~= "table" then return "tables" end
            if level.BattleTag ~= 1 or level.RealID ~= 2 or level.Title ~= 3 then return "values" end
            if meta.MinValue ~= 1 or meta.MaxValue ~= 3 or meta.NumValues ~= 3 then return "metadata" end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_pet_and_lfg_payloads() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local pet = C_PetJournal.GetPetInfoTableBySpeciesID(39)
            if type(pet) ~= "table" then return "pet-type" end
            if pet.name ~= "Mechanical Squirrel" then return "pet-name" end
            if pet.icon ~= 132932 or pet.petType ~= 9 or pet.speciesID ~= 39 then return "pet-identity" end
            if pet.isWild ~= false or pet.canBattle ~= true then return "pet-battle" end
            if pet.isTradeable ~= false or pet.isUnique ~= false or pet.obtainable ~= true then return "pet-flags" end
            if pet.canAttachToDecor ~= false or pet.creatureModelScale ~= 1 then return "pet-12-1" end
            if C_PetJournal.GetPetInfoTableBySpeciesID(999999) ~= nil then return "pet-unknown" end

            local listing = C_LFGList.GetSearchResultInfo(7)
            if type(listing) ~= "table" then return "lfg-type" end
            if listing.searchResultID ~= 7 or listing.name ~= "RBG yolo" then return "lfg-identity" end
            if listing.activityID ~= 493 or listing.activityIDs[1] ~= 493 then return "lfg-activity" end
            if listing.numMembers ~= 7 or listing.maxMembers ~= 10 then return "lfg-size" end
            if listing.partyGUID ~= "Party-3-0000-1234-00000007" then return "lfg-guid" end
            if listing.censored ~= false then return "lfg-censored" end
            if C_LFGList.GetSearchResultInfo(999999) ~= nil then return "lfg-unknown" end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_player_choice_payload_and_mutator_intent() {
    let env = WowLuaEnv::new().unwrap();
    let default_result: String = env
        .eval(
            r#"
            if type(C_PlayerChoice) ~= "table" then return "namespace" end
            if select('#', C_PlayerChoice.GetCurrentPlayerChoiceInfo()) ~= 0 then return "default-info-count" end
            if C_PlayerChoice.GetCurrentPlayerChoiceInfo() ~= nil then return "default-info" end
            if C_PlayerChoice.GetNumRerolls() ~= 0 then return "default-rerolls" end
            if C_PlayerChoice.GetRemainingTime() ~= nil then return "default-time" end
            if C_PlayerChoice.IsWaitingForPlayerChoiceResponse() ~= false then return "default-waiting" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(default_result, "ok");

    env.state().borrow_mut().player_choice.current = Some(PlayerChoiceInfo {
        object_guid: "Creature-0-0000-0000-00000-12345-0000000000".into(),
        choice_id: 42,
        question_text: "Choose your path".into(),
        pending_choice_text: "Waiting".into(),
        ui_texture_kit: "playerchoice-test".into(),
        hide_warboard_header: false,
        keep_open_after_choice: true,
        show_choices_as_list: true,
        requires_selection: true,
        show_choices_as_grid: false,
        options: vec![PlayerChoiceOptionInfo {
            id: 7,
            description: "Take the reward".into(),
            header: "Reward".into(),
            choice_art_id: 99,
            desaturated_art: false,
            disabled_option: false,
            has_rewards: true,
            reward_info: PlayerChoiceOptionRewardInfo {
                currency_rewards: vec![PlayerChoiceRewardCurrencyInfo {
                    currency_id: 2003,
                    name: "Dragon Isles Supplies".into(),
                    currency_texture: 463446,
                    quantity: 25,
                    is_currency_container: false,
                }],
                item_rewards: vec![PlayerChoiceRewardItemInfo {
                    item_id: 19019,
                    name: "Thunderfury".into(),
                    quantity: 1,
                }],
                reputation_rewards: vec![PlayerChoiceRewardReputationInfo {
                    faction_id: 72,
                    quantity: 100,
                }],
            },
            ui_texture_kit: "playerchoice-option".into(),
            max_stacks: 1,
            buttons: vec![PlayerChoiceOptionButtonInfo {
                id: 7,
                text: "Select".into(),
                disabled: false,
                show_checkmark: true,
                hide_button_show_text: false,
                selected: true,
                confirmation: Some("Confirm".into()),
                tooltip: Some("Choose this reward".into()),
                reward_quest_id: Some(70000),
                sound_kit_id: Some(12867),
                list_text: Some("Reward list entry".into()),
            }],
            widget_set_id: Some(55),
            spell_id: Some(642),
            rarity: Some(3),
            type_art_id: Some(12),
            header_icon_atlas_element: Some("playerchoice-icon".into()),
            sub_header: Some("Epic".into()),
            consolidate_widgets: true,
        }],
        sound_kit_id: Some(100),
        close_ui_sound_kit_id: Some(101),
    });
    {
        let mut state = env.state().borrow_mut();
        state.player_choice.num_rerolls = 2;
        state.player_choice.remaining_time = Some(30.5);
        state.player_choice.waiting_for_response = true;
    }

    let result: String = env
        .eval(
            r#"
            local info = C_PlayerChoice.GetCurrentPlayerChoiceInfo()
            if type(info) ~= "table" or info.choiceID ~= 42 then return "info" end
            if info.objectGUID ~= "Creature-0-0000-0000-00000-12345-0000000000" then return "info-guid" end
            if info.questionText ~= "Choose your path" or info.pendingChoiceText ~= "Waiting" then return "info-text" end
            if info.uiTextureKit ~= "playerchoice-test" or info.hideWarboardHeader ~= false then return "info-display" end
            if info.keepOpenAfterChoice ~= true or info.showChoicesAsList ~= true then return "info-list" end
            if info.requiresSelection ~= true or info.showChoicesAsGrid ~= false then return "info-layout" end
            if info.soundKitID ~= 100 or info.closeUISoundKitID ~= 101 then return "info-sounds" end

            local option = info.options[1]
            if option.id ~= 7 or option.description ~= "Take the reward" or option.header ~= "Reward" then return "option-identity" end
            if option.choiceArtID ~= 99 or option.desaturatedArt ~= false or option.disabledOption ~= false then return "option-art" end
            if option.hasRewards ~= true or option.uiTextureKit ~= "playerchoice-option" then return "option-display" end
            if option.maxStacks ~= 1 or option.widgetSetID ~= 55 or option.spellID ~= 642 then return "option-ids" end
            if option.rarity ~= 3 or option.typeArtID ~= 12 then return "option-types" end
            if option.headerIconAtlasElement ~= "playerchoice-icon" or option.subHeader ~= "Epic" then return "option-header" end
            if option.consolidateWidgets ~= true then return "option-widgets" end

            local button = option.buttons[1]
            if button.id ~= 7 or button.text ~= "Select" or button.disabled ~= false then return "button-identity" end
            if button.showCheckmark ~= true or button.hideButtonShowText ~= false or button.selected ~= true then return "button-flags" end
            if button.confirmation ~= "Confirm" or button.tooltip ~= "Choose this reward" then return "button-text" end
            if button.rewardQuestID ~= 70000 or button.soundKitID ~= 12867 or button.listText ~= "Reward list entry" then return "button-optionals" end

            local currency = option.rewardInfo.currencyRewards[1]
            if currency.currencyId ~= 2003 or currency.name ~= "Dragon Isles Supplies" then return "currency-identity" end
            if currency.currencyTexture ~= 463446 or currency.quantity ~= 25 or currency.isCurrencyContainer ~= false then return "currency-values" end
            local item = option.rewardInfo.itemRewards[1]
            if item.itemId ~= 19019 or item.name ~= "Thunderfury" or item.quantity ~= 1 then return "item" end
            local reputation = option.rewardInfo.repRewards[1]
            if reputation.factionId ~= 72 or reputation.quantity ~= 100 then return "reputation" end
            if C_PlayerChoice.GetNumRerolls() ~= 2 then return "rerolls" end
            if C_PlayerChoice.GetRemainingTime() ~= 30.5 then return "time" end
            if C_PlayerChoice.IsWaitingForPlayerChoiceResponse() ~= true then return "waiting" end
            C_PlayerChoice.SendPlayerChoiceResponse(7)
            C_PlayerChoice.RequestRerollPlayerChoice()
            C_PlayerChoice.OnUIClosed()
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");

    let state = env.state().borrow();
    assert_eq!(state.player_choice.last_response_id, Some(7));
    assert!(state.player_choice.reroll_requested);
    assert!(state.player_choice.ui_closed);
}

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_tiered_entrance_payloads() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local active = C_DelvesUI.GetActiveDelveTier()
            if active.tier ~= 4 or active.suggestedILvl ~= 610 or active.unlocked ~= true then return "active-scalars" end
            if active.tierDescription ~= "Tier 4" or active.modifierUIWidgetSetID ~= 4404 then return "active-display" end
            if active.lockedReason ~= nil or type(active.rewards) ~= "table" then return "active-optional" end
            local itemReward = active.rewards[1]
            if itemReward.id ~= 228361 or itemReward.quantity ~= 1 then return "item-reward" end
            if itemReward.rewardType ~= Enum.TieredEntranceRewardType.Item or itemReward.context ~= 0 then return "item-reward-type" end
            local currencyReward = active.rewards[2]
            if currencyReward.id ~= 2815 or currencyReward.quantity ~= 25 then return "currency-reward" end
            if currencyReward.rewardType ~= Enum.TieredEntranceRewardType.Currency or currencyReward.context ~= 0 then return "currency-reward-type" end

            local tiers = C_DelvesUI.GetDelveEntranceTiers()
            if #tiers ~= 5 or tiers[1].tier ~= 1 or tiers[5].tier ~= 5 then return "tier-order" end
            if tiers[5].unlocked ~= false or type(tiers[5].lockedReason) ~= "string" then return "locked-tier" end
            if type(tiers[1].rewards) ~= "table" or #tiers[1].rewards ~= 2 then return "tier-rewards" end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_spell_cooldown_payload() {
    let env = WowLuaEnv::new().unwrap();
    {
        let mut state = env.state().borrow_mut();
        let start = state.start_time.elapsed().as_secs_f64();
        state.spell_cooldowns.insert(
            12345,
            crate::lua_api::state::SpellCooldownState {
                start,
                duration: 12.5,
            },
        );
    }

    let result: String = env
        .eval(
            r#"
            local active = C_Spell.GetSpellCooldown(12345)
            if type(active) ~= "table" then return "active-type" end
            if active.startTime < 0 or active.duration ~= 12.5 then return "active-time" end
            if active.isEnabled ~= true or active.isActive ~= true or active.modRate ~= 1 then return "active-flags" end

            local inactive = C_Spell.GetSpellCooldown(999999)
            if type(inactive) ~= "table" then return "inactive-type" end
            if inactive.startTime ~= 0 or inactive.duration ~= 0 then return "inactive-time" end
            if inactive.isEnabled ~= true or inactive.isActive ~= false or inactive.modRate ~= 1 then return "inactive-flags" end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}
