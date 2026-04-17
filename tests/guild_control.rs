//! `GuildControl*` rank probes — SimState-backed round-trip.

use wow_ui_sim::lua_api::WowLuaEnv;

fn install_three_ranks(env: &WowLuaEnv) {
    env.exec(
        r#"
        A_Admin.SetGuildRanks({
            { name = "Guild Leader", flags = { true, true, true } },
            { name = "Officer",      flags = { true, false, true } },
            { name = "Member",       flags = { false, false, false } },
        })
        "#,
    )
    .unwrap();
}

#[test]
fn defaults_no_guild() {
    let env = WowLuaEnv::new().unwrap();
    let (count, name): (i32, String) = env
        .eval(r#"return GuildControlGetNumRanks(), GuildControlGetRankName()"#)
        .unwrap();
    assert_eq!(count, 0);
    assert_eq!(name, "");

    let flags_count: i32 = env.eval(r#"return #GuildControlGetRankFlags()"#).unwrap();
    assert_eq!(flags_count, 0);
}

#[test]
fn set_ranks_updates_count() {
    let env = WowLuaEnv::new().unwrap();
    install_three_ranks(&env);
    let count: i32 = env.eval("return GuildControlGetNumRanks()").unwrap();
    assert_eq!(count, 3);
}

#[test]
fn get_rank_name_by_explicit_index() {
    let env = WowLuaEnv::new().unwrap();
    install_three_ranks(&env);
    let names: (String, String, String) = env
        .eval(
            r#"
            return GuildControlGetRankName(1),
                   GuildControlGetRankName(2),
                   GuildControlGetRankName(3)
            "#,
        )
        .unwrap();
    assert_eq!(names.0, "Guild Leader");
    assert_eq!(names.1, "Officer");
    assert_eq!(names.2, "Member");
}

#[test]
fn set_rank_selects_which_getter_returns_by_default() {
    let env = WowLuaEnv::new().unwrap();
    install_three_ranks(&env);
    env.exec("GuildControlSetRank(2)").unwrap();
    let name: String = env.eval("return GuildControlGetRankName()").unwrap();
    assert_eq!(
        name, "Officer",
        "after SetRank(2), GetRankName() should return the 2nd rank",
    );

    env.exec("GuildControlSetRank(3)").unwrap();
    let name: String = env.eval("return GuildControlGetRankName()").unwrap();
    assert_eq!(name, "Member");
}

#[test]
fn out_of_range_explicit_index_returns_empty() {
    let env = WowLuaEnv::new().unwrap();
    install_three_ranks(&env);
    let (zero, too_high): (String, String) = env
        .eval(
            r#"
            return GuildControlGetRankName(0), GuildControlGetRankName(99)
            "#,
        )
        .unwrap();
    assert_eq!(zero, "");
    assert_eq!(too_high, "");
}

#[test]
fn rank_flags_round_trip_as_boolean_array() {
    let env = WowLuaEnv::new().unwrap();
    install_three_ranks(&env);
    let flags: (bool, bool, bool) = env
        .eval(
            r#"
            local f = GuildControlGetRankFlags(2)
            return f[1], f[2], f[3]
            "#,
        )
        .unwrap();
    assert_eq!(flags, (true, false, true));
}

#[test]
fn clearing_ranks_resets_selection_and_returns() {
    let env = WowLuaEnv::new().unwrap();
    install_three_ranks(&env);
    env.exec("GuildControlSetRank(2)").unwrap();
    env.exec("A_Admin.SetGuildRanks({})").unwrap();
    let (count, name): (i32, String) = env
        .eval(r#"return GuildControlGetNumRanks(), GuildControlGetRankName()"#)
        .unwrap();
    assert_eq!(count, 0);
    assert_eq!(name, "");
}

#[test]
fn set_rank_to_out_of_range_clears_selection() {
    let env = WowLuaEnv::new().unwrap();
    install_three_ranks(&env);
    env.exec("GuildControlSetRank(2)").unwrap();
    env.exec("GuildControlSetRank(99)").unwrap();
    let name: String = env.eval("return GuildControlGetRankName()").unwrap();
    assert_eq!(
        name, "",
        "out-of-range SetRank should clear the selection; GetRankName without index returns empty",
    );
}
