//! Empty-invitation preservation only; no producer acceptance before native oracle.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary",
))]

#[test]
fn original_guild_microbutton_has_no_unseen_invitations_with_empty_clubs_preserves_boundary() {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &ui,
        &["Blizzard_MicroMenu"],
        &[],
    );
    env.state().borrow_mut().clubs.clubs.clear();
    env.exec(
        r#"
        DISPLAYED_COMMUNITIES_INVITATIONS = {}
        local button = CreateFrame("Button", nil, UIParent)
        Mixin(button, GuildMicroButtonMixin)
        assert(button:HasUnseenInvitations() == false,
            'empty supplied club state has no invitation to notify')
        DISPLAYED_COMMUNITIES_INVITATIONS['previously-displayed-club'] = true
        assert(button:HasUnseenInvitations() == false,
            'display history cannot manufacture an incoming invitation')
        local invitations = C_Club.GetInvitationsForSelf()
        assert(type(invitations) == 'table' and next(invitations) == nil,
            'empty supplied club state projects an empty invitation list')
        "#,
    )
    .expect("original microbutton empty-invitation consumer");
}
