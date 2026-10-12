//! Supplied friend-name queries and unchanged current-profile note-popup behavior.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

const NAME: &str = "ExactFriendProbe-Realm";
const NOTE: &str = "Supplied note: Tuesday raid";
const CHANGED_NOTE: &str = "Live note: Friday raid";

fn supplied_env(note: &str) -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("friend fixture environment");
    {
        let mut state = env.state().borrow_mut();
        let mut friend = state
            .social_friends
            .first()
            .expect("existing SocialFriend seed")
            .clone();
        friend.name = NAME.to_string();
        friend.note = note.to_string();
        state.social_friends = vec![friend];
    }
    env
}

fn assert_exact_info(env: &WowLuaEnv, note: &str) {
    env.exec(&format!(
        r#"
        local query = rawget(C_FriendList, 'GetFriendInfo')
        assert(type(query) == 'function', 'missing exact C_FriendList.GetFriendInfo')
        local function check(...)
            assert(select('#', ...) == 1, 'friend result arity')
            local info = ...
            assert(type(info) == 'table', 'friend result table')
            assert(info.name == {NAME:?}, 'supplied friend name')
            assert(info.notes == {note:?}, 'supplied live friend note')
        end
        check(query({NAME:?}))
        "#
    ))
    .expect("exact friend lookup must read supplied SocialFriend");
}

#[test]
fn exact_friend_lookup_reads_live_notes_without_environment_leaks() {
    let first = supplied_env(NOTE);
    let second = supplied_env("Independent environment note");
    assert_exact_info(&first, NOTE);
    assert_exact_info(&second, "Independent environment note");
    first.state().borrow_mut().social_friends[0].note = CHANGED_NOTE.to_string();
    assert_exact_info(&first, CHANGED_NOTE);
    assert_exact_info(&second, "Independent environment note");
}

#[test]
fn exact_friend_lookup_rejects_nonstring_types_and_returns_nil_for_miss() {
    let env = supplied_env(NOTE);
    assert_exact_info(&env, NOTE);
    env.exec(
        r#"
        local query = rawget(C_FriendList, 'GetFriendInfo')
        assert(not pcall(query), 'missing required name')
        assert(not pcall(query, nil), 'nil name')
        for _, value in ipairs({false, true, {}, function() end}) do
            assert(not pcall(query, value), 'nonstring name')
        end
        local function checkMiss(...)
            assert(select('#', ...) == 1, 'simulator miss arity')
            assert((...) == nil, 'unknown supplied name must miss')
        end
        checkMiss(query('NotSuppliedFriend-Realm'))
        "#,
    )
    .expect("exact friend lookup input and miss controls");
}

#[test]
fn exact_friend_lookup_authenticates_secret_name_by_caller_taint() {
    let env = supplied_env(NOTE);
    assert_exact_info(&env, NOTE);
    env.exec(&format!(
        r#"
        local query = rawget(C_FriendList, 'GetFriendInfo')
        assert(debug.getstacktaint() == nil, 'untainted setup')
        local name = secretwrap({NAME:?})
        assert(issecretvalue(name) and canaccessvalue(name))
        local function check(...)
            assert(select('#', ...) == 1, 'secret lookup arity')
            local info = ...
            assert(type(info) == 'table')
            assert(info.name == {NAME:?} and info.notes == {NOTE:?})
        end
        check(query(name))
        local ok, message = pcall(query, secretwrap(901001))
        assert(not ok and message:find('string', 1, true), 'wrapped nonstring name')
        local function probe()
            assert(debug.getstacktaint() == 'ExactFriendProbe')
            assert(issecretvalue(name) and not canaccessvalue(name))
            assert(not pcall(query, name), 'tainted secret name must be rejected')
            check(query({NAME:?}))
            assert(debug.getstacktaint() == 'ExactFriendProbe')
        end
        debug.setobjecttaint(probe, 'ExactFriendProbe')
        probe()
        assert(debug.getstacktaint() == nil, 'caller taint restored')
        check(query(name))
        "#
    ))
    .expect("AllowedWhenUntainted secret argument controls");
}

fn load_real_friend_popup(env: &WowLuaEnv) {
    load_friend_popup_closure(env);
    assert_mainline_friend_popup_provenance(env);
}

fn load_friend_popup_closure(env: &WowLuaEnv) {
    let ui = wow_ui_sim::client_profile::blizzard_ui_addons_dir();
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    wow_ui_sim::xml::register_intrinsic_templates();
    let roots = ["Blizzard_StaticPopup_Game", "Blizzard_FriendsFrame"];
    let addons = discover_blizzard_addon_closure_for_screen(&ui, ScreenKind::Game, &roots);
    for root in roots {
        assert!(
            addons.iter().any(|(name, _)| name == root),
            "missing closure root {root}"
        );
    }
    for (name, toc) in addons {
        load_popup_addon(env, &name, &toc);
    }
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "popup closure recorded Lua errors: {:?}",
        env.state().borrow().lua_errors
    );
}

fn load_popup_addon(env: &WowLuaEnv, name: &str, toc: &std::path::Path) {
    let result = load_addon(&env.loader_env(), toc)
        .unwrap_or_else(|error| panic!("popup closure load {name}: {error}"));
    assert!(
        result.warnings.is_empty(),
        "popup readiness {name}: {:?}",
        result.warnings
    );
    // Nil-probe observations include safely guarded optional debug namespaces.
    // Preserve them without treating every probe as a required dependency.
    if !result.missing_requirements.is_empty() {
        eprintln!(
            "popup nil-probe observations {name}: {:?}",
            result.missing_requirements
        );
    }
}

fn assert_mainline_friend_popup_provenance(env: &WowLuaEnv) {
    env.exec(
        r#"
        assert(C_AddOns.IsAddOnLoaded('Blizzard_StaticPopup_Game'))
        assert(C_AddOns.IsAddOnLoaded('Blizzard_FriendsFrame'))
        assert(FriendsFrame and FriendsFrame:GetName() == 'FriendsFrame')
        local handler = StaticPopupDialogs.SET_FRIENDNOTE.OnShow
        assert(type(handler) == 'function', 'Mainline OnShow must exist')
        local source = debug.getinfo(handler, 'S').source:gsub('\\', '/')
        assert(source:find('Blizzard_StaticPopup_Game/Mainline/GameDialogDefs.lua', 1, true),
            'wrong SET_FRIENDNOTE handler source: ' .. source)
        "#,
    )
    .expect("real whole-addon popup readiness and Mainline handler provenance");
}

fn assert_popup_prefill_and_hide(env: &WowLuaEnv, note: &str) {
    env.exec(&format!(
        r#"
        FriendsFrame.NotesID = {NAME:?}
        local dialog = StaticPopup_Show('SET_FRIENDNOTE', {NAME:?})
        assert(dialog and dialog:IsShown(), 'real friend note popup must show')
        assert(dialog.which == 'SET_FRIENDNOTE')
        local editBox = dialog:GetEditBox()
        assert(editBox and editBox:GetText() == {note:?}, 'real OnShow note prefill')
        dialog:Hide()
        assert(not dialog:IsShown(), 'real popup hide')
        "#
    ))
    .expect("public StaticPopup_Show must prefill real edit box from supplied friend");
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "popup recorded Lua errors: {:?}",
        env.state().borrow().lua_errors
    );
}

#[test]
fn real_mainline_friend_note_popup_prefills_and_reopens_with_live_note() {
    let env = supplied_env(NOTE);
    load_real_friend_popup(&env);
    assert_popup_prefill_and_hide(&env, NOTE);
    env.state().borrow_mut().social_friends[0].note = CHANGED_NOTE.to_string();
    assert_popup_prefill_and_hide(&env, CHANGED_NOTE);
}
