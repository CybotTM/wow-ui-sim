//! Smoke tests for the `A_Admin` module split.
//!
//! These assert the public admin entry points still mutate `SimState` after
//! the implementation moved into smaller focused files.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn admin_actionbars_and_bags_still_update_state() {
    let env = env();
    env.exec("A_Admin.SetActionSlot(1, 12345)")
        .expect("SetActionSlot");
    env.exec("A_Admin.AddBagItem(0, 5, 6948, 3)")
        .expect("AddBagItem");

    let state = env.state().borrow();
    assert_eq!(state.action_bars.get(&1), Some(&12345));
    let bag_item = state.bag_items.get(&(0, 5)).expect("bag item");
    assert_eq!(bag_item.item_id, 6948);
    assert_eq!(bag_item.stack_count, 3);
}

#[test]
fn admin_buffs_and_equipment_still_update_player_state() {
    let env = env();
    env.exec(r#"A_Admin.AddBuff(99001, "Test Buff", "134973", 30, 2)"#)
        .expect("AddBuff");
    env.exec("A_Admin.EquipItem(1, 211993)").expect("EquipItem");

    let state = env.state().borrow();
    assert!(state.player.buffs.iter().any(|buff| buff.spell_id == 99001));
    assert!(state.player.equipped_items.contains_key(&1));
}

#[test]
fn admin_movement_spec_and_zone_economy_still_update_state() {
    let env = env();
    env.exec("A_Admin.SetMoving(true)").expect("SetMoving");
    env.exec("A_Admin.SetSpec(3)").expect("SetSpec");
    env.exec(r#"A_Admin.SetZone("Stormwind City", 1519)"#)
        .expect("SetZone");
    env.exec("A_Admin.SetMoney(123456)").expect("SetMoney");

    let state = env.state().borrow();
    assert!(state.player.movement.moving);
    assert_eq!(state.player.active_spec_index, 3);
    assert_eq!(state.world.zone_name, "Stormwind City");
    assert_eq!(state.world.zone_id, 1519);
    assert_eq!(state.player.money, 123456);
}

#[test]
fn admin_collections_and_vault_still_update_world_state() {
    let env = env();
    env.exec("A_Admin.CollectMount(1039)")
        .expect("CollectMount");
    env.exec("A_Admin.SetVaultActivity(2, 1, 8, 5, 10)")
        .expect("SetVaultActivity");

    let state = env.state().borrow();
    assert!(state.world.collected_mounts.contains(&1039));
    assert_eq!(state.world.great_vault_activities.len(), 1);
    let activity = &state.world.great_vault_activities[0];
    assert_eq!(activity.activity_type, 2);
    assert_eq!(activity.index, 1);
    assert_eq!(activity.progress, 5);
}

#[test]
fn admin_guild_mail_premade_and_encounter_still_queue_state() {
    let env = env();
    let initial_state = env.state().borrow();
    let initial_inbox_len = initial_state.player.inbox.len();
    let initial_premade_len = initial_state.world.premade_listings.len();
    drop(initial_state);

    env.exec(r#"A_Admin.SetGuildInfo("Test Guild", "Officer", 42)"#)
        .expect("SetGuildInfo");
    env.exec(r#"A_Admin.AddMail("Thrall", "Greetings", "Welcome")"#)
        .expect("AddMail");
    env.exec(r#"A_Admin.AddPremadeListing("Test Group", "Testing", 1195, 2, 5)"#)
        .expect("AddPremadeListing");
    env.exec(r#"A_Admin.StartLootRoll(1, 30, "Sword", "tex", 4, 600)"#)
        .expect("StartLootRoll");

    let state = env.state().borrow();
    assert_eq!(state.world.guild_name.as_deref(), Some("Test Guild"));
    assert_eq!(state.player.inbox.len(), initial_inbox_len + 1);
    assert_eq!(state.world.premade_listings.len(), initial_premade_len + 1);
    assert!(state.world.loot_rolls.contains_key(&1));
}

#[test]
fn admin_mailbox_interaction_updates_state_and_fires_events() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local mailInfo = Enum.PlayerInteractionType.MailInfo
            local opened = false
            local closed = false
            local frame = CreateFrame("Frame")
            frame:RegisterEvent("PLAYER_INTERACTION_MANAGER_FRAME_SHOW")
            frame:RegisterEvent("PLAYER_INTERACTION_MANAGER_FRAME_HIDE")
            frame:SetScript("OnEvent", function(self, event, interactionType)
                if interactionType ~= mailInfo then
                    return
                end
                if event == "PLAYER_INTERACTION_MANAGER_FRAME_SHOW" then
                    opened = true
                elseif event == "PLAYER_INTERACTION_MANAGER_FRAME_HIDE" then
                    closed = true
                end
            end)

            A_Admin.OpenMailbox()
            A_Admin.CloseMailbox()

            return opened and closed and "ok" or "missing_event"
            "#,
        )
        .expect("mailbox admin interaction");
    assert_eq!(result, "ok");
    assert!(
        env.state().borrow().active_player_interactions.is_empty(),
        "CloseMailbox should remove the active MailInfo interaction"
    );
}

#[test]
fn admin_fire_event_still_preserves_payloads() {
    let env = env();
    let (received, arg1, arg2): (bool, bool, bool) = env
        .eval(
            r#"
            local received = false
            local got_arg1, got_arg2
            local frame = CreateFrame("Frame")
            frame:RegisterEvent("PLAYER_ENTERING_WORLD")
            frame:SetScript("OnEvent", function(self, event, a1, a2)
                if event == "PLAYER_ENTERING_WORLD" then
                    received = true
                    got_arg1 = a1
                    got_arg2 = a2
                end
            end)
            A_Admin.FireEvent("PLAYER_ENTERING_WORLD", 7, true)
            return received, got_arg1, got_arg2
            "#,
        )
        .expect("FireEvent");

    assert!(received);
    assert_eq!(arg1, true);
    assert_eq!(arg2, true);
}
