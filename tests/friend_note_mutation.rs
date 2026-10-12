//! Exact-name note mutation over owned friends; no native persistence/event claim.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::SocialFriend;
use wow_ui_sim::screen::ScreenKind;

const NAME: &str = "ExactFriendProbe-Realm";
const NOTE: &str = "Supplied note: Tuesday raid";
const OTHER_NAME: &str = "OtherFriendProbe-Realm";
const OTHER_NOTE: &str = "Other friend stays unchanged";
const CHANGED_NOTE: &str = "Live note: Friday raid";

fn supplied_env(note: &str) -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("friend note fixture environment");
    {
        let mut state = env.state().borrow_mut();
        let mut friend = state
            .social_friends
            .first()
            .expect("existing friend seed")
            .clone();
        friend.name = NAME.to_string();
        friend.note = note.to_string();
        friend.is_online = true;
        let mut other = friend.clone();
        other.name = OTHER_NAME.to_string();
        other.note = OTHER_NOTE.to_string();
        other.guid = "Player-1-0000B002".to_string();
        other.is_online = false;
        // Reverse display order: C_FriendList sorts online first, C_Social does not.
        state.social_friends = vec![other, friend];
    }
    env
}

fn assert_friends_match(env: &WowLuaEnv, expected: &[SocialFriend]) {
    let state = env.state().borrow();
    assert_eq!(state.social_friends.len(), expected.len(), "friend count");
    for (actual, expected) in state.social_friends.iter().zip(expected) {
        assert_eq!(actual.name, expected.name, "friend identity/order");
        assert_eq!(actual.level, expected.level, "level");
        assert_eq!(actual.area, expected.area, "area");
        assert_eq!(actual.class_name, expected.class_name, "class");
        assert_eq!(actual.note, expected.note, "owned note");
        assert_eq!(actual.is_online, expected.is_online, "connection state");
        assert_eq!(actual.guid, expected.guid, "GUID");
    }
}

const LUA_CHECKS: &str = r#"
    local set = rawget(C_FriendList, 'SetFriendNotes')
    assert(type(set) == 'function', 'RED boundary: missing raw C_FriendList.SetFriendNotes; downstream unreached')
    local function found(expected, ...)
        assert(select('#', ...) == 1, 'setter must return exactly one value')
        local value = ...
        assert(type(value) == 'boolean' and value == expected, 'setter found result')
    end
    local function checkNotes(expected)
        local exact = C_FriendList.GetFriendInfo(name)
        local indexed = C_FriendList.GetFriendInfoByIndex(1)
        local social = C_Social.GetFriendInfo(2)
        local friends = C_Social.GetFriends()
        local function check(info)
            assert(type(info) == 'table' and info.name == name, 'existing projected friend')
            assert(info.notes == expected, 'live projected notes')
        end
        check(exact)
        check(indexed)
        check(social)
        check(friends[2])
        assert(#friends == 2 and friends[1].name == otherName, 'owned list count/order')
        assert(friends[1].notes == otherNote, 'unrelated friend notes')
        assert(C_FriendList.GetFriendInfo(otherName).notes == otherNote, 'unrelated exact getter')
        assert(C_FriendList.GetFriendInfoByIndex(2).notes == otherNote, 'unrelated display getter')
        assert(C_Social.GetFriendInfo(1).notes == otherNote, 'unrelated social getter')
    end
    local function typeError(argument, ...)
        local ok, message = pcall(set, ...)
        assert(not ok and type(message) == 'string', 'invalid argument must error')
        assert(message:find('argument ' .. argument, 1, true), 'correct invalid argument position')
        assert(message:find('string', 1, true), 'string type error, not access denial')
        assert(not message:find('secret access', 1, true), 'untainted malformed payload type error')
    end
"#;

fn run_case(env: &WowLuaEnv, body: &str) {
    let script = format!(
        "local name = {NAME:?}\nlocal oldNote = {NOTE:?}\nlocal otherName = {OTHER_NAME:?}\nlocal otherNote = {OTHER_NOTE:?}\n{LUA_CHECKS}\n{body}"
    );
    env.exec(&script).expect("prepared friend note contract");
}

#[test]
fn friend_note_mutation_updates_all_live_views_and_found_is_not_changed() {
    let env = supplied_env(NOTE);
    let mut expected = env.state().borrow().social_friends.clone();
    run_case(
        &env,
        r#"
        checkNotes(oldNote)
        found(true, set(name, 'Live note: Friday raid'))
        checkNotes('Live note: Friday raid')
        found(true, set(name, 'Live note: Friday raid'))
        checkNotes('Live note: Friday raid')
        found(true, set(name, ''))
        checkNotes('')
        found(true, set(name, ''))
        checkNotes('')
        "#,
    );
    expected[1].note.clear();
    assert_friends_match(&env, &expected);
}

#[test]
fn friend_note_mutation_exact_misses_do_not_mutate_or_create_records() {
    let env = supplied_env(NOTE);
    let before = env.state().borrow().social_friends.clone();
    run_case(
        &env,
        r#"
        for _, missing in ipairs({'NotSuppliedFriend-Realm', 'exactfriendprobe-realm',
            'ExactFriendProbe', ' ExactFriendProbe-Realm', ''}) do
            assert(C_FriendList.GetFriendInfo(missing) == nil, 'as-getter exact matching')
            found(false, set(missing, 'must not land on any friend'))
            checkNotes(oldNote)
        end
        "#,
    );
    assert_friends_match(&env, &before);
}

#[test]
fn friend_note_mutation_requires_two_strict_utf8_strings_without_coercion() {
    let env = supplied_env(NOTE);
    let before = env.state().borrow().social_friends.clone();
    run_case(
        &env,
        r#"
        typeError(1)
        typeError(1, nil, 'replacement')
        typeError(2, name)
        typeError(2, name, nil)
        for _, invalid in ipairs({false, true, 901001, {}, function() end,
            string.char(255), string.char(226, 130)}) do
            typeError(1, invalid, 'replacement')
            typeError(2, name, invalid)
            checkNotes(oldNote)
        end
        typeError(2, 'NotSuppliedFriend-Realm', 901001)
        checkNotes(oldNote)
        "#,
    );
    assert_friends_match(&env, &before);
}

#[test]
fn friend_note_mutation_accepts_untainted_authentic_secret_strings_on_each_argument() {
    let env = supplied_env(NOTE);
    let mut expected = env.state().borrow().social_friends.clone();
    run_case(
        &env,
        r#"
        assert(debug.getstacktaint() == nil, 'untainted setup')
        local secretName = secretwrap(name)
        local secretNote = secretwrap('Raid café Friday')
        assert(issecretvalue(secretName) and issecretvalue(secretNote), 'genuine VM wrappers')
        found(true, set(secretName, 'name-only secret'))
        checkNotes('name-only secret')
        found(true, set(name, secretNote))
        checkNotes('Raid café Friday')
        found(true, set(secretName, secretNote))
        checkNotes('Raid café Friday')
        found(false, set(secretwrap('NotSuppliedFriend-Realm'), secretNote))
        checkNotes('Raid café Friday')
        found(true, set(secretName, secretwrap('')))
        checkNotes('')
        assert(issecretvalue(secretName) and issecretvalue(secretNote), 'wrappers retained')
        assert(secretunwrap(secretName) == name, 'name payload retained')
        assert(secretunwrap(secretNote) == 'Raid café Friday', 'note payload retained')
        assert(debug.getstacktaint() == nil, 'untainted context retained')
        "#,
    );
    expected[1].note.clear();
    assert_friends_match(&env, &expected);
}

#[test]
fn friend_note_mutation_untainted_malformed_secret_payloads_have_positioned_type_errors() {
    let env = supplied_env(NOTE);
    let before = env.state().borrow().social_friends.clone();
    run_case(
        &env,
        r#"
        assert(debug.getstacktaint() == nil)
        local payloads = {901001, false, {}, function() end, string.char(255)}
        for _, payload in ipairs(payloads) do
            local wrapper = secretwrap(payload)
            typeError(1, wrapper, secretwrap('replacement'))
            typeError(2, secretwrap(name), wrapper)
            assert(issecretvalue(wrapper), 'malformed wrapper retained')
            assert(secretunwrap(wrapper) == payload, 'malformed payload retained')
            checkNotes(oldNote)
        end
        local wrappedNil = secretwrap(nil)
        typeError(1, wrappedNil, 'replacement')
        typeError(2, name, wrappedNil)
        assert(issecretvalue(wrappedNil), 'wrapped nil retained')
        assert(secretunwrap(wrappedNil) == nil, 'wrapped nil payload retained')
        assert(debug.getstacktaint() == nil, 'type errors preserve context')
        checkNotes(oldNote)
        "#,
    );
    assert_friends_match(&env, &before);
}

#[test]
fn friend_note_mutation_authenticates_both_original_arguments_before_type_or_write() {
    let env = supplied_env(NOTE);
    let before = env.state().borrow().social_friends.clone();
    run_case(
        &env,
        r#"
        assert(debug.getstacktaint() == nil)
        local secretName = secretwrap(name)
        local secretMiss = secretwrap('NotSuppliedFriend-Realm')
        local secretNote = secretwrap('blocked replacement')
        local secretInvalid = secretwrap(901001)
        local secretNil = secretwrap(nil)
        local wrappers = {secretName, secretMiss, secretNote, secretInvalid, secretNil}
        local function denied(...)
            local ok, message = pcall(set, ...)
            assert(not ok and type(message) == 'string', 'tainted secret must error')
            assert(message:find('secret access', 1, true), 'access denial before type/match')
            assert(not message:find('must be a UTF-8 string', 1, true), 'not a type error')
            assert(debug.getstacktaint() == 'FriendNoteProbe', 'error preserves caller taint')
            for _, wrapper in ipairs(wrappers) do
                assert(issecretvalue(wrapper), 'error retains secret wrapper')
            end
            checkNotes(oldNote)
        end
        local function probe()
            assert(debug.getstacktaint() == 'FriendNoteProbe', 'genuine tainted caller')
            denied(secretName, 'plain note')
            denied(secretMiss, 'plain note')
            denied(name, secretNote)
            denied('NotSuppliedFriend-Realm', secretNote)
            denied(secretName, secretNote)
            denied(secretInvalid, 'plain note')
            denied(name, secretInvalid)
            denied(secretNil, 'plain note')
            denied(name, secretNil)
            denied(secretName, false)
            -- Critical ordering: arg1's plain type is invalid, arg2 is secret.
            denied(false, secretNote)
            denied(901001, secretNote)
            denied(nil, secretNote)
            denied(string.char(255), secretNote)
        end
        debug.setobjecttaint(probe, 'FriendNoteProbe')
        probe()
        assert(debug.getstacktaint() == nil, 'outer caller restored')
        assert(secretunwrap(secretName) == name and secretunwrap(secretNote) == 'blocked replacement')
        assert(secretunwrap(secretMiss) == 'NotSuppliedFriend-Realm')
        assert(secretunwrap(secretInvalid) == 901001 and secretunwrap(secretNil) == nil)
        checkNotes(oldNote)
        "#,
    );
    assert_friends_match(&env, &before);
}

#[test]
fn friend_note_mutation_plain_tainted_calls_stay_environment_local() {
    let first = supplied_env(NOTE);
    let second = supplied_env("Independent environment note");
    let mut expected_first = first.state().borrow().social_friends.clone();
    let mut expected_second = second.state().borrow().social_friends.clone();
    run_case(
        &first,
        r#"
        local function probe()
            assert(debug.getstacktaint() == 'PlainFriendNoteProbe')
            found(true, set(name, 'Live note: Friday raid'))
            checkNotes('Live note: Friday raid')
            found(false, set('NotSuppliedFriend-Realm', 'ignored'))
            checkNotes('Live note: Friday raid')
            assert(debug.getstacktaint() == 'PlainFriendNoteProbe', 'plain setter preserves taint')
        end
        debug.setobjecttaint(probe, 'PlainFriendNoteProbe')
        probe()
        assert(debug.getstacktaint() == nil, 'outer caller restored')
        "#,
    );
    run_case(&second, "checkNotes('Independent environment note')");
    expected_first[1].note = CHANGED_NOTE.to_string();
    assert_friends_match(&first, &expected_first);
    assert_friends_match(&second, &expected_second);
    run_case(
        &second,
        "found(true, set(name, 'Second environment changed'))\ncheckNotes('Second environment changed')",
    );
    run_case(&first, "checkNotes('Live note: Friday raid')");
    expected_second[1].note = "Second environment changed".to_string();
    assert_friends_match(&first, &expected_first);
    assert_friends_match(&second, &expected_second);
}

fn load_real_friend_popup(env: &WowLuaEnv) {
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
    assert_no_popup_errors(env);
}

fn load_popup_addon(env: &WowLuaEnv, name: &str, toc: &std::path::Path) {
    let result = load_addon(&env.loader_env(), toc)
        .unwrap_or_else(|error| panic!("popup closure load {name}: {error}"));
    assert!(
        result.warnings.is_empty(),
        "popup readiness {name}: {:?}",
        result.warnings
    );
    // Same policy as friend_info_exact: optional guarded nil probes are observations.
    if !result.missing_requirements.is_empty() {
        eprintln!(
            "popup nil-probe observations {name}: {:?}",
            result.missing_requirements
        );
    }
}

fn assert_no_popup_errors(env: &WowLuaEnv) {
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "popup closure recorded Lua errors: {:?}",
        env.state().borrow().lua_errors
    );
}

#[test]
fn friend_note_mutation_real_accept_click_and_original_enter_reopen_from_getter() {
    let env = supplied_env(NOTE);
    let mut expected = env.state().borrow().social_friends.clone();
    // Missing registration remains the first RED; it earns no popup-path coverage.
    run_case(&env, "checkNotes(oldNote)");
    load_real_friend_popup(&env);
    run_case(
        &env,
        r#"
        assert(C_AddOns.IsAddOnLoaded('Blizzard_StaticPopup_Game'))
        assert(C_AddOns.IsAddOnLoaded('Blizzard_FriendsFrame'))
        assert(FriendsFrame and FriendsFrame:GetName() == 'FriendsFrame')
        local definition = StaticPopupDialogs.SET_FRIENDNOTE
        local show, accept, enter = definition.OnShow, definition.OnAccept, definition.EditBoxOnEnterPressed
        local function source(handler, suffix)
            assert(type(handler) == 'function', 'original popup handler')
            local path = debug.getinfo(handler, 'S').source:gsub('\\', '/')
            assert(path:sub(-#suffix) == suffix, 'wrong original handler source: ' .. path)
        end
        source(show, 'Blizzard_StaticPopup_Game/Mainline/GameDialogDefs.lua')
        source(accept, 'Blizzard_StaticPopup_Game/GameDialogDefs.lua')
        source(enter, 'Blizzard_StaticPopup_Game/GameDialogDefs.lua')
        source(StaticPopup_OnClick, 'Blizzard_StaticPopup/StaticPopup.lua')
        FriendsFrame.NotesID = name
        local dialog = StaticPopup_Show('SET_FRIENDNOTE', name)
        assert(dialog and dialog:IsShown() and dialog.which == 'SET_FRIENDNOTE')
        local editBox = dialog:GetEditBox()
        assert(editBox:GetText() == oldNote, 'original OnShow prefill')
        editBox:SetText('Live note: Friday raid')
        assert(editBox:GetText() == 'Live note: Friday raid', 'real EditBox.SetText')
        local button = dialog:GetButton1()
        assert(button and button:IsShown() and button:IsEnabled(), 'actual Accept button readiness')
        source(button:GetScript('OnClick'), 'Blizzard_StaticPopup_Game/GameDialog.lua')
        button:Click('LeftButton', false)
        -- Original OnAccept is void: root StaticPopup_OnClick must still hide.
        assert(not dialog:IsShown(), 'original void Accept root must hide, not forward API true')
        checkNotes('Live note: Friday raid')
        dialog = StaticPopup_Show('SET_FRIENDNOTE', name)
        assert(dialog and dialog:IsShown())
        editBox = dialog:GetEditBox()
        assert(editBox:GetText() == C_FriendList.GetFriendInfo(name).notes)
        assert(editBox:GetText() == 'Live note: Friday raid', 'reopen reads accepted note')
        editBox:SetText('')
        assert(editBox:GetText() == '', 'real edit box clear')
        -- Invoke the loaded original whole Enter handler, not a copied body/slice.
        assert(select('#', enter(editBox, dialog.data)) == 0, 'original Enter is void, not API bool')
        assert(not dialog:IsShown(), 'original Enter hide')
        checkNotes('')
        dialog = StaticPopup_Show('SET_FRIENDNOTE', name)
        assert(dialog and dialog:IsShown())
        assert(dialog:GetEditBox():GetText() == C_FriendList.GetFriendInfo(name).notes)
        assert(dialog:GetEditBox():GetText() == '', 'reopen reads cleared note')
        assert(definition == StaticPopupDialogs.SET_FRIENDNOTE, 'definition retained')
        assert(definition.OnShow == show and definition.OnAccept == accept)
        assert(definition.EditBoxOnEnterPressed == enter, 'original handlers retained')
        dialog:Hide()
        "#,
    );
    assert_no_popup_errors(&env);
    expected[1].note.clear();
    assert_friends_match(&env, &expected);
}
