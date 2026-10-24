//! Edit-only staged RED; no execution or native-parity claim.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]

use wow_ui_sim::c_api::club_model::{Club, Member, OWNER, Stream};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_original_consumers() -> WowLuaEnv {
    use crate::common::blizzard_addon_harness::{
        blizzard_ui_dir, build_blizzard_addon_closure_env,
    };
    use wow_ui_sim::loader::find_toc_file;
    use wow_ui_sim::screen::ScreenKind;
    use wow_ui_sim::toc::TocFile;

    let ui = blizzard_ui_dir();
    let toc_path = find_toc_file(&ui.join("Blizzard_MicroMenu")).expect("selected MicroMenu TOC");
    let expected_toc = match wow_ui_sim::client_profile::ACTIVE {
        wow_ui_sim::client_profile::ClientProfile::WowForever => "Blizzard_MicroMenu.toc",
        _ => "Blizzard_MicroMenu_Mainline.toc",
    };
    assert_eq!(toc_path.file_name().unwrap(), expected_toc);
    let toc = TocFile::from_file(&toc_path).expect("parse selected TOC");
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    assert!(
        toc.files
            .iter()
            .any(|file| file.ends_with("Mainline/MainMenuBarMicroButtons.lua"))
    );
    eprintln!("CF-2 unread selected TOC: {}", toc_path.display());
    let (env, loaded) = build_blizzard_addon_closure_env(&ui, &["Blizzard_MicroMenu"], &[]);
    assert!(loaded.iter().any(|name| name == "Blizzard_MicroMenu"));
    assert!(loaded.iter().any(|name| name == "Blizzard_FrameXMLUtil"));
    env.exec(
        r#"
        local function original(fn, suffix)
            local info = debug.getinfo(fn, 'S')
            local source = info.source:gsub('\\', '/')
            assert(info.what == 'Lua' and source:sub(-#suffix) == suffix)
            print('CF-2 unread original consumer', source, info.linedefined, info.lastlinedefined)
        end
        original(GuildMicroButton.UpdateNotificationIcon,
            'Blizzard_MicroMenu/Mainline/MainMenuBarMicroButtons.lua')
        for _, name in ipairs({ 'DoesAnyCommunityHaveUnreadMessages',
            'DoesOtherCommunityHaveUnreadMessages', 'DoesCommunityHaveOtherUnreadMessages',
            'DoesCommunityStreamHaveUnreadMessages', 'ClearAllUnreadNotifications' }) do
            original(CommunitiesUtil[name], 'Blizzard_FrameXMLUtil/CommunitiesUtil.lua')
        end
        assert(GuildMicroButton.NotificationOverlay)
        assert(C_SocialRestrictions.CanReceiveChat() == true)
        DISPLAYED_COMMUNITIES_INVITATIONS = {}
        GuildMicroButton:Enable()
    "#,
    )
    .expect("selected original methods and notification fixture");
    env
}

fn stream(id: u64, filter: Option<u8>, marker: Option<u64>) -> Stream {
    Stream {
        id,
        name: format!("Stream {id}"),
        subject: "Supplied stream".into(),
        stream_type: 0,
        leaders_and_moderators_only: false,
        creation_time: 1_700_000_000,
        notification_filter: filter,
        last_read_time: marker,
    }
}

fn supply_club(env: &WowLuaEnv, id: &str, streams: Vec<Stream>) {
    env.state().borrow_mut().clubs.clubs.insert(
        id.into(),
        Club {
            id: id.into(),
            name: id.into(),
            description: "Supplied community".into(),
            club_type: 1,
            avatar_id: 7,
            members: vec![Member::new("self", "Ari", OWNER, true)],
            candidates: Default::default(),
            invitations: Vec::new(),
            streams,
        },
    );
}

fn assert_notification(env: &WowLuaEnv, expected: bool) {
    let shown: bool = env
        .eval(
            r#"
        GuildMicroButton:UpdateNotificationIcon()
        return GuildMicroButton.NotificationOverlay:IsShown()
    "#,
        )
        .expect("original MicroMenu notification consumer must finish");
    assert_eq!(
        shown, expected,
        "notification must follow current club/stream state"
    );
}

#[test]
fn original_micro_menu_tracks_club_unread_state() {
    let env = load_original_consumers();
    assert!(env.state().borrow().clubs.initialized);
    assert!(env.state().borrow().clubs.self_invitations.is_empty());
    assert_notification(&env, false);
    supply_club(&env, "club-a", vec![stream(11, Some(2), Some(101))]);
    assert_notification(&env, true);
    supply_club(&env, "club-a", vec![stream(11, Some(2), None)]);
    assert_notification(&env, false);
    // All is required by the original per-community helper. Missing, None and
    // Mention preferences are not silently upgraded to All.
    for filter in [None, Some(0), Some(1)] {
        supply_club(&env, "club-a", vec![stream(11, filter, Some(101))]);
        assert_notification(&env, false);
    }
    supply_club(
        &env,
        "club-b",
        vec![stream(21, Some(2), None), stream(22, Some(2), Some(202))],
    );
    assert_notification(&env, true);
    env.state().borrow_mut().clubs.clubs.remove("club-b");
    assert_notification(&env, false);
    supply_club(&env, "club-a", vec![stream(12, Some(2), Some(303))]);
    assert_notification(&env, true);
    env.state().borrow_mut().clubs.clubs.remove("club-a");
    assert_notification(&env, false);
}

#[test]
fn original_community_unread_helpers_clear_supplied_markers() {
    let env = load_original_consumers();
    supply_club(
        &env,
        "club-a",
        vec![
            stream(11, Some(2), Some(101)),
            stream(12, Some(0), Some(102)),
        ],
    );
    supply_club(
        &env,
        "club-b",
        vec![stream(11, Some(2), None), stream(13, Some(0), Some(606))],
    );
    env.exec(
        r#"
        assert(CommunitiesUtil.DoesCommunityHaveUnreadMessages('club-a') == true)
        assert(not CommunitiesUtil.DoesCommunityHaveOtherUnreadMessages('club-a', 11))
        assert(not CommunitiesUtil.DoesOtherCommunityHaveUnreadMessages('club-a'))
        assert(CommunitiesUtil.DoesOtherCommunityHaveUnreadMessages('club-b') == true)
        assert(CommunitiesUtil.DoesCommunityStreamHaveUnreadMessages('club-a', 12) == true)
        assert(not CommunitiesUtil.DoesCommunityStreamHaveUnreadMessages('club-b', 11))
        local rows = C_Club.GetClubStreamNotificationSettings('club-a')
        assert(#rows == 2 and rows[1].streamId == 11 and rows[1].filter == 2)
        assert(rows[2].streamId == 12 and rows[2].filter == 0)
        rows[1].filter = 0
        assert(CommunitiesUtil.DoesCommunityHaveUnreadMessages('club-a') == true)
        assert(C_Club.GetStreamViewMarker('club-a', 11) == 101)
        assert(C_Club.GetStreamViewMarker('club-a', 12) == 102)
        local streams = C_Club.GetStreams('club-a')
        assert(#streams == 2 and streams[1].streamId == 11 and streams[2].streamId == 12)
        assert(streams[1].name == 'Stream 11' and streams[1].subject == 'Supplied stream')
        assert(streams[1].creationTime == 1700000000 and streams[1].streamType == 0)
        assert(streams[1].leadersAndModeratorsOnly == false)
        assert(C_Club.GetStreamInfo('club-a', 12).name == 'Stream 12')
        assert(C_Club.IsSubscribedToStream('club-a', 12) == true)
        assert(C_Club.IsSubscribedToStream('club-b', 12) == false)
    "#,
    )
    .expect("original unread query consumers");
    for filter in [None, Some(0), Some(1), Some(2)] {
        supply_club(
            &env,
            "club-a",
            vec![
                stream(11, filter, Some(101)),
                stream(12, Some(0), Some(102)),
            ],
        );
        let unread: bool = env
            .eval("return not not CommunitiesUtil.DoesCommunityHaveUnreadMessages('club-a')")
            .expect("original helper after preference replacement");
        assert_eq!(unread, filter == Some(2));
        let count: usize = env
            .eval("return #C_Club.GetClubStreamNotificationSettings('club-a')")
            .expect("only supplied preferences become rows");
        assert_eq!(count, if filter.is_some() { 2 } else { 1 });
    }
    env.exec(
        r#"
        assert(#C_Club.GetClubStreamNotificationSettings('club-b') == 2)
        CF2UnreadEvents = {}
        local listener = CreateFrame('Frame')
        listener:RegisterEvent('STREAM_VIEW_MARKER_UPDATED')
        listener:SetScript('OnEvent', function(_, _, clubId, streamId, ...)
            CF2UnreadEvents[#CF2UnreadEvents + 1] = {
                clubId, streamId, select('#', ...),
                C_Club.GetStreamViewMarker(clubId, streamId) == nil,
                not CommunitiesUtil.DoesCommunityHaveUnreadMessages(clubId),
            }
        end)
        CommunitiesUtil.ClearAllUnreadNotifications('club-a')
        assert(#CF2UnreadEvents == 2)
        for i, event in ipairs(CF2UnreadEvents) do
            assert(event[1] == 'club-a' and event[2] == 10 + i)
            assert(event[3] == 1 and event[4] and event[5])
        end
        assert(not CommunitiesUtil.DoesCommunityHaveUnreadMessages('club-a'))
        assert(not CommunitiesUtil.DoesCommunityStreamHaveUnreadMessages('club-a', 12))
        assert(C_Club.GetStreamViewMarker('club-a', 12) == nil)
        assert(CommunitiesUtil.DoesCommunityStreamHaveUnreadMessages('club-b', 13) == true)
        assert(C_Club.GetStreamViewMarker('club-b', 13) == 606)
        assert(C_Club.GetStreamInfo('club-a', 12).name == 'Stream 12')
    "#,
    )
    .expect("original sibling consumers observe concrete settings, markers and synchronous clear");
    assert_notification(&env, false);
    supply_club(&env, "club-b", vec![stream(11, Some(2), Some(404))]);
    env.exec(
        r#"
        assert(CommunitiesUtil.DoesCommunityHaveUnreadMessages('club-b') == true)
        assert(not CommunitiesUtil.DoesCommunityHaveUnreadMessages('club-a'))
    "#,
    )
    .expect("read state is scoped by club as well as stream");
    assert_notification(&env, true);
}

#[test]
fn guild_stream_projection_preserves_supplied_unread_across_roster_sync() {
    let env = WowLuaEnv::new().expect("environment");
    env.exec("assert(#C_Club.GetStreams('guild-0') == 2)")
        .expect("legacy guild streams");
    {
        let mut sim = env.state().borrow_mut();
        let guild = sim.clubs.clubs.get_mut("guild-0").expect("synced guild");
        assert_eq!(guild.streams.len(), 2);
        guild.streams[0].notification_filter = Some(2);
        guild.streams[0].last_read_time = Some(505);
    }
    env.exec(
        r#"
        assert(C_Club.GetClubInfo('guild-0'))
        assert(#C_Club.GetSubscribedClubs() >= 1)
        assert(C_Club.GetStreamInfo('guild-0', 1).name == 'Guild')
        assert(C_Club.GetStreamInfo('guild-0', 2).name == 'Officer')
        assert(C_Club.GetStreamViewMarker('guild-0', 1) == 505)
        assert(C_Club.DoesAnyCommunityHaveUnreadMessages() == true)
        C_Club.AdvanceStreamViewMarker('guild-0', 1)
        assert(C_Club.GetStreamViewMarker('guild-0', 1) == nil)
        assert(C_Club.DoesAnyCommunityHaveUnreadMessages() == false)
    "#,
    )
    .expect("legacy roster sync must retain stream state");
}
