#![cfg(feature = "client-wrath")]

//! Build 12340 positional flags; unselected/out-of-range ranks are not a contract.

use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

fn supply_bank_permissions(env: &WowLuaEnv) {
    use wow_ui_sim::lua_api::state::GuildBankRankPermissions;
    seed_selected_rank(env);
    env.state()
        .borrow_mut()
        .world
        .guild_bank_rank_permissions
        .insert(
            1,
            GuildBankRankPermissions {
                money_per_day: 123_456,
                tab_rights: [0, 1, 2, 4, 7, -1],
                tab_stacks_per_day: [0, 11, 22, 33, 44, -1],
            },
        );
}

#[test]
fn bank_permissions_project_selected_rank_bits_limits_and_no_events() {
    let env = WowLuaEnv::new().unwrap();
    supply_bank_permissions(&env);
    env.state().borrow_mut().world.guild_player_rank = 0;
    env.state().borrow_mut().event_queue.drain();
    env.exec(include_str!("fixtures/wrath_f03/bank_contract.lua"))
        .expect("supplied selected rank, native tuple and normalized tab results");
    assert!(env.state().borrow_mut().event_queue.drain().is_empty());
    assert!(!env.state().borrow().guild_bank_frame_open);
}

#[test]
fn bank_permissions_reject_missing_and_nonnumeric_tab() {
    let env = WowLuaEnv::new().unwrap();
    supply_bank_permissions(&env);
    env.exec(
        r#"
        local ok, message = pcall(GetGuildBankTabPermissions)
        assert(not ok and string.find(message, 'Usage:'))
        for _, value in ipairs({false, {}, 'not a number'}) do
            local success, reason = pcall(GetGuildBankTabPermissions, value)
            assert(not success and string.find(reason, 'Usage:'))
        end
    "#,
    )
    .expect("native tab usage errors");
}

#[test]
fn bank_permissions_initial_rank_zero_and_received_money_replacement() {
    let env = WowLuaEnv::new().unwrap();
    // Supply a valid selection directly; never assert the source-undefined -1 rank.
    env.state().borrow_mut().world.guild_selected_rank = 1;
    env.exec(
        r#"
        for tab = 1, 6 do
            local view, deposit, edit, stacks = GetGuildBankTabPermissions(tab)
            assert(view == 1 and deposit == 1 and edit == 1 and stacks == -1)
        end
        assert(GetGuildBankWithdrawLimit() == 0)
    "#,
    )
    .expect("native initial EnterWorld rank-zero permission image");
    supply_bank_permissions(&env);
    for (money, expected) in [(9_999, 0), (10_001, 1), (-1, -1), (-20_001, -20_001)] {
        env.state()
            .borrow_mut()
            .world
            .guild_bank_rank_permissions
            .get_mut(&1)
            .unwrap()
            .money_per_day = money;
        let actual: f64 = env.eval("return GetGuildBankWithdrawLimit()").unwrap();
        assert_eq!(actual, expected as f64);
    }
    env.state()
        .borrow_mut()
        .world
        .guild_bank_rank_permissions
        .insert(
            0,
            wow_ui_sim::lua_api::state::GuildBankRankPermissions {
                money_per_day: 300_001,
                tab_rights: [0; 6],
                tab_stacks_per_day: [8; 6],
            },
        );
    env.exec(
        r#"
        GuildControlSetRank(1)
        local view, deposit, edit, stacks = GetGuildBankTabPermissions(1)
        assert(view == nil and deposit == nil and edit == nil and stacks == 8)
        assert(GetGuildBankWithdrawLimit() == 30)
    "#,
    )
    .expect("host receipt replaces rank-zero seeds without leader special casing");
}

fn seed_selected_rank(env: &WowLuaEnv) {
    env.exec(
        r#"
        A_Admin.SetGuildInfo("Rank rights fixture", "Officer", 2)
        A_Admin.SetGuildRanks({
            { name = "Leader", flags = { false, true } },
            { name = "Officer", flags = {
                true, false, true, false, true, false, true, false, true,
                false, true, false, true, true, false, true, false,
            } },
        })
        GuildControlSetRank(2)
        "#,
    )
    .expect("select concrete native rank rights");
}

#[test]
fn native_guild_control_consumer_accepts_positional_flags() {
    test_timeout! {
        let env = WowLuaEnv::new().unwrap();
        common::load_profile_framexml(&env);
        seed_selected_rank(&env);
        env.exec(r#"
            GuildControlCheckboxUpdate(GuildControlGetRankFlags())
            assert(GuildControlPopupFrameCheckbox1:GetChecked(), "enabled stored rank right")
            assert(not GuildControlPopupFrameCheckbox2:GetChecked(), "disabled stored rank right")
            assert(GuildControlPopupFrameCheckbox16:GetChecked(), "late stored rank right")
            assert(not GuildControlPopupFrameCheckbox17:GetChecked(), "trailing nil right")
        "#).expect("original native checkbox consumer must receive positional values, not a table");
    }
}

#[test]
fn selected_rank_flags_return_seventeen_number_or_nil_values_ignoring_arguments() {
    let env = WowLuaEnv::new().unwrap();
    seed_selected_rank(&env);
    env.exec(
        r#"
        local function check(...)
            assert(select('#', ...) == 17, "native result arity retains nil positions")
            local expected = {1, 3, 5, 7, 9, 11, 13, 14, 16}
            for index = 1, 17 do
                local enabled = false
                for _, position in ipairs(expected) do
                    if position == index then enabled = true end
                end
                local actual = select(index, ...)
                if enabled then
                    assert(type(actual) == "number" and actual == 1, "enabled right " .. index)
                else
                    assert(actual == nil, "disabled right " .. index)
                end
            end
        end
        check(GuildControlGetRankFlags())
        check(GuildControlGetRankFlags(1))
        check(GuildControlGetRankFlags("ignored", {}, false))
        "#,
    )
    .expect("12340 selected rank, ignored inputs, flag14 and exact result types");
}

#[test]
fn valid_selected_rank_without_guild_returns_seventeen_nils() {
    let env = WowLuaEnv::new().unwrap();
    seed_selected_rank(&env);
    // Keep a valid selection: native unselected-rank reads are undefined.
    env.state().borrow_mut().world.guild_name = None;
    env.exec(
        r#"
        local function check(...)
            assert(select('#', ...) == 17)
            for index = 1, 17 do assert(select(index, ...) == nil) end
        end
        check(GuildControlGetRankFlags())
        "#,
    )
    .expect("valid selected rank with no guild retains all seventeen nil returns");
}
