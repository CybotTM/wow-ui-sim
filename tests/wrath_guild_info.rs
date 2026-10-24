#![cfg(feature = "client-wrath")]

//! Original FriendsFrame.xml GuildInfoFrame OnShow, never a copied handler.
use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

fn setup() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    supply(&env, "fixture roster text", false);
    common::load_profile_framexml(&env);
    env.exec(
        r#"
        ToggleFriendsFrame(3)
        function AssertNativeGuildInfo(text, editable)
            GuildInfoFrame:Hide()
            GuildInfoFrame.cachedText = nil
            GuildInfoEditBox:SetText("poison: OnShow must replace this")
            GuildInfoFrame:Show()
            local expected = text
            if editable and text == "" then expected = GUILD_INFO_EDITLABEL end
            assert(GuildInfoFrame:IsShown(), "native info panel shown")
            assert(GuildInfoEditBox:GetText() == expected, "native text/empty edit label")
            local r, g, b = GuildInfoEditBox:GetTextColor()
            local color = editable and 1 or 0.65
            assert(math.abs(r-color) < 0.00001 and math.abs(g-color) < 0.00001
                and math.abs(b-color) < 0.00001, "native text color")
            assert(not not GuildInfoSaveButton:IsEnabled() == editable, "native save permission")
            assert(not not GuildInfoEditBox:IsMouseEnabled() == editable, "native mouse permission")
        end
        "#,
    )
    .expect("load selected native FriendsFrame and observation helper");
    env
}

fn supply(env: &WowLuaEnv, text: &str, editable: bool) {
    let mut sim = env.state().borrow_mut();
    sim.player.present = true;
    sim.world.guild_name = Some("Guild info fixture".into());
    sim.world.guild_rank = Some("Officer".into());
    sim.world.guild_id = 73;
    sim.world.guild_player_rank = 1;
    sim.world.guild_info_text = text.into();
    // Reuse existing positional rights. Position 13 represents native 0x10000.
    sim.world.guild_ranks = vec![Default::default(), Default::default()];
    sim.world.guild_ranks[1].flags = vec![false; 17];
    sim.world.guild_ranks[1].flags[12] = editable;
    // Deliberately contradict the old officer shortcut and the selected admin rank.
    sim.world.guild_is_officer = !editable;
    sim.world.guild_selected_rank = 1;
}

#[test]
fn native_info_onshow_projects_empty_and_supplied_cache_in_both_permission_branches() {
    test_timeout! {
        let env = setup();
        for (text, editable) in [("", true), ("Roster A: raid Tuesday", false),
            ("Roster B: raid Friday", true), ("", false)] {
            supply(&env, text, editable);
            let code = format!("AssertNativeGuildInfo({text:?}, {editable})");
            env.exec(&code).expect("original OnShow must project current cache and rights");
        }
        env.exec(r#"
            GuildInfoFrame:Hide()
            GuildInfoFrame.cachedText = "unsent local UI edit"
            GuildInfoFrame:Show()
            assert(GuildInfoEditBox:GetText() == "unsent local UI edit", "native UI cache wins")
        "#).unwrap();
    }
}

#[test]
fn native_info_onshow_uses_player_presence_guild_id_and_player_rank_rights() {
    test_timeout! {
        let env = setup();
        supply(&env, "Roster permission probe", true);
        env.exec("AssertNativeGuildInfo('Roster permission probe', true)").unwrap();
        env.state().borrow_mut().player.present = false;
        env.exec("AssertNativeGuildInfo('Roster permission probe', false)").unwrap();
        {
            let mut sim = env.state().borrow_mut();
            sim.player.present = true;
            sim.world.guild_id = 0;
        }
        env.exec("AssertNativeGuildInfo('Roster permission probe', false)").unwrap();
        {
            let mut sim = env.state().borrow_mut();
            sim.world.guild_id = 73;
            sim.world.guild_player_rank = 0;
        }
        env.exec("AssertNativeGuildInfo('Roster permission probe', false)").unwrap();
        supply(&env, "Second roster permission probe", true);
        env.exec("AssertNativeGuildInfo('Second roster permission probe', true)").unwrap();
    }
}

#[test]
fn initialized_world_has_no_synthetic_guild_membership_or_roster() {
    let env = WowLuaEnv::new().unwrap();
    let state = env.state().borrow();
    assert_eq!(state.world.guild_name, None);
    assert_eq!(state.world.guild_rank, None);
    assert_eq!(state.world.guild_id, 0);
    assert_eq!(state.world.guild_num_members, 0);
    assert!(state.world.guild_ranks.is_empty());
    assert!(state.world.guild_members.is_empty());
    assert!(state.world.guild_info_text.is_empty());
}

#[test]
fn getter_results_preserve_native_arity_types_ignored_arguments_and_cache_independence() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(select('#', GetGuildInfoText()) == 1 and GetGuildInfoText() == "")
        assert(select('#', CanEditGuildInfo()) == 1 and CanEditGuildInfo() == nil)
    "#,
    )
    .expect("initialized native results");
    supply(&env, "received roster text", true);
    env.exec(
        r#"
        assert(select('#', CanEditGuildInfo({}, false, 42)) == 1)
        assert(type(CanEditGuildInfo()) == "number" and CanEditGuildInfo() == 1)
        assert(select('#', GetGuildInfoText({}, false, 42)) == 1)
        assert(GetGuildInfoText({}, false, 42) == "received roster text")
    "#,
    )
    .unwrap();
    {
        let mut sim = env.state().borrow_mut();
        sim.world.guild_id = 0;
        sim.world.guild_info_text = "retained cache without membership".into();
        sim.events.drain();
    }
    env.exec(
        r#"
        assert(GetGuildInfoText() == "retained cache without membership")
        assert(select('#', CanEditGuildInfo()) == 1 and CanEditGuildInfo() == nil)
    "#,
    )
    .unwrap();
    assert!(
        env.state().borrow().events.is_empty(),
        "getters do not request roster/events"
    );
}
