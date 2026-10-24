//! Staged original-consumer RED; execution is deferred by the edit-only contract.
#![cfg(feature = "client-retail")]

use wow_ui_sim::c_api::club_model::Member;
use wow_ui_sim::c_api::club_self_invitations::{InvitedClub, SelfInvitation};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_selected_micro_menu() -> WowLuaEnv {
    use crate::common::blizzard_addon_harness::{
        blizzard_ui_dir, build_blizzard_addon_closure_env,
    };
    use wow_ui_sim::loader::find_toc_file;
    use wow_ui_sim::screen::ScreenKind;
    use wow_ui_sim::toc::TocFile;

    let ui = blizzard_ui_dir();
    let toc_path = find_toc_file(&ui.join("Blizzard_MicroMenu")).expect("selected MicroMenu TOC");
    assert_eq!(
        toc_path.file_name().unwrap(),
        "Blizzard_MicroMenu_Mainline.toc"
    );
    let toc = TocFile::from_file(&toc_path).expect("parse selected TOC");
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    for suffix in [
        "Mainline/MainMenuBarMicroButtons.lua",
        "Mainline/MainMenuBarMicroButtons.xml",
    ] {
        assert!(toc.files.iter().any(|file| file.ends_with(suffix)));
    }
    eprintln!("CF-2 selected consumer TOC: {}", toc_path.display());
    let (env, loaded) = build_blizzard_addon_closure_env(&ui, &["Blizzard_MicroMenu"], &[]);
    assert!(loaded.iter().any(|name| name == "Blizzard_MicroMenu"));
    env.exec(
        r#"
        local info = debug.getinfo(GuildMicroButton.HasUnseenInvitations, 'S')
        local suffix = 'Blizzard_MicroMenu/Mainline/MainMenuBarMicroButtons.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must call the runtime-selected original consumer')
        print('CF-2 original consumer', source, info.linedefined, info.lastlinedefined)
        assert(GuildMicroButton.NotificationOverlay)
        DISPLAYED_COMMUNITIES_INVITATIONS = {}
        "#,
    )
    .expect("original XML-created button and Lua provenance");
    env
}

fn invitation(id: &str, club_id: &str) -> SelfInvitation {
    SelfInvitation {
        id: id.into(),
        club: InvitedClub {
            id: club_id.into(),
            name: "Weekend Raiders".into(),
            description: "Saturday raids".into(),
            broadcast: "Meet at eight".into(),
            club_type: 1,
            avatar_id: 7,
        },
        inviter: Member::new("inviter-a", "Ari", 1, false),
        leaders: vec![Member::new("leader-b", "Bela", 2, false)],
    }
}

fn assert_unseen(env: &WowLuaEnv, expected: bool) {
    let unseen: bool = env
        .eval("return GuildMicroButton:HasUnseenInvitations()")
        .expect("original MicroMenu consumer must finish through the live producer");
    assert_eq!(
        unseen, expected,
        "unseen must follow supplied invitations and displayed club IDs"
    );
}

#[test]
fn original_guild_micro_button_tracks_supplied_self_invitations() {
    let env = load_selected_micro_menu();
    assert!(env.state().borrow().clubs.initialized);
    assert!(env.state().borrow().clubs.self_invitations.is_empty());
    assert_unseen(&env, false);

    env.state().borrow_mut().clubs.self_invitations = vec![invitation("invite-a", "club-a")];
    assert_unseen(&env, true);
    env.exec(
        r#"
        local rows = C_Club.GetInvitationsForSelf()
        assert(#rows == 1)
        local row = rows[1]
        assert(row.invitationId == 'invite-a')
        assert(row.club.clubId == 'club-a' and row.club.name == 'Weekend Raiders')
        assert(row.club.description == 'Saturday raids' and row.club.broadcast == 'Meet at eight')
        assert(row.club.clubType == 1 and row.club.avatarId == 7)
        assert(row.inviter.memberId == 'inviter-a' and row.inviter.name == 'Ari')
        assert(row.inviter.isSelf == false and row.inviter.presence == 1)
        assert(#row.leaders == 1 and row.leaders[1].memberId == 'leader-b')
        assert(row.leaders[1].name == 'Bela' and row.leaders[1].isSelf == false)
        assert(row.leaders[1].presence == 1)
        DISPLAYED_COMMUNITIES_INVITATIONS['club-a'] = true
        "#,
    )
    .expect("supplied required record fields and displayed-club fixture");
    assert_unseen(&env, false);

    env.state()
        .borrow_mut()
        .clubs
        .self_invitations
        .push(invitation("invite-b", "club-b"));
    assert_unseen(&env, true);
    env.exec("DISPLAYED_COMMUNITIES_INVITATIONS['club-b'] = true")
        .expect("mark second club displayed in consumer input");
    assert_unseen(&env, false);

    env.state().borrow_mut().clubs.self_invitations = vec![invitation("invite-c", "club-c")];
    assert_unseen(&env, true);
    env.state().borrow_mut().clubs.self_invitations.clear();
    assert_unseen(&env, false);
    env.state().borrow_mut().clubs.self_invitations = vec![invitation("invite-c", "club-c")];
    assert_unseen(&env, true);
}
