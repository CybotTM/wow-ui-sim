#![cfg(feature = "client-wrath")]

//! Build 12340 positional flags; unselected/out-of-range ranks are not a contract.

use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

fn seed_selected_rank(env: &WowLuaEnv) {
    env.exec(
        r#"
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
            assert(select("#", ...) == 17, "native result arity retains nil positions")
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
            assert(select("#", ...) == 17)
            for index = 1, 17 do assert(select(index, ...) == nil) end
        end
        check(GuildControlGetRankFlags())
        "#,
    )
    .expect("valid selected rank with no guild retains all seventeen nil returns");
}
