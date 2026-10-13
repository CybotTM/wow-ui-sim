//! Host-observed pet-hover contract; behavioral RED/GREEN not yet executed.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

const FIRST: &str = " opaque Pet-901001 ";
const SECOND: &str = "not-a-native-guid/901002";

fn assert_hover(env: &WowLuaEnv, expected: Option<&str>) {
    assert_eq!(
        env.state()
            .borrow()
            .pet_journal_hover
            .hovered_battle_pet_guid
            .as_deref(),
        expected,
        "owned hovered GUID after Lua API call"
    );
}

#[test]
fn pet_hover_retains_exact_supplied_string_and_replaces_previous_value() {
    let env = WowLuaEnv::new().expect("pet hover environment");
    assert_hover(&env, None);
    for guid in [FIRST, SECOND, "", "pet-901003", "Pet-901003", "宠物-é-🐾"] {
        env.exec(&format!(
            "assert(select('#', C_PetJournal.SetHoveredBattlePet({guid:?})) == 0)"
        ))
        .expect("void setter accepts supplied opaque string");
        assert_hover(&env, Some(guid));
    }
}

#[test]
fn pet_hover_clear_removes_owned_value_and_is_idempotent() {
    let env = WowLuaEnv::new().expect("pet hover environment");
    // Legitimate host-supplied model input isolates clear from the still-no-op setter.
    env.state()
        .borrow_mut()
        .pet_journal_hover
        .hovered_battle_pet_guid = Some(FIRST.into());
    for _ in 0..2 {
        env.exec("assert(select('#', C_PetJournal.ClearHoveredBattlePet()) == 0)")
            .expect("void clear");
        assert_hover(&env, None);
    }
}

#[test]
fn pet_hover_mutations_are_environment_local() {
    let first = WowLuaEnv::new().expect("first pet hover environment");
    let second = WowLuaEnv::new().expect("second pet hover environment");
    first
        .exec(&format!("C_PetJournal.SetHoveredBattlePet({FIRST:?})"))
        .expect("first setter");
    assert_hover(&first, Some(FIRST));
    assert_hover(&second, None);
    second
        .exec(&format!("C_PetJournal.SetHoveredBattlePet({SECOND:?})"))
        .expect("second setter");
    assert_hover(&first, Some(FIRST));
    assert_hover(&second, Some(SECOND));
    first
        .exec("C_PetJournal.ClearHoveredBattlePet()")
        .expect("first clear");
    assert_hover(&first, None);
    assert_hover(&second, Some(SECOND));
}

#[test]
fn pet_hover_untainted_secret_string_retains_exact_guid_in_owned_state() {
    let env = WowLuaEnv::new().expect("secret pet hover environment");
    env.exec(&format!(
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil)
        PetHoverSecretFixture = secretwrap({FIRST:?})
        assert(issecretvalue(PetHoverSecretFixture))
        C_PetJournal.SetHoveredBattlePet(PetHoverSecretFixture)
        assert(issecretvalue(PetHoverSecretFixture), 'original wrapper remains secret')
        assert(debug.getstacktaint() == nil, 'caller remains untainted')
        "#
    ))
    .expect("untainted VM secret string");
    assert_hover(&env, Some(FIRST));
}

#[test]
fn pet_hover_public_mutations_from_tainted_caller_update_owned_state() {
    let env = WowLuaEnv::new().expect("tainted pet hover environment");
    env.exec(&format!(
        r#"
        debug.settaintmode(true)
        local function addon()
            assert(debug.getstacktaint() == 'PetHoverProbe')
            assert(select('#', C_PetJournal.SetHoveredBattlePet({FIRST:?})) == 0)
            assert(debug.getstacktaint() == 'PetHoverProbe')
        end
        debug.setobjecttaint(addon, 'PetHoverProbe')
        addon()
        assert(debug.getstacktaint() == nil)
        "#
    ))
    .expect("tainted public string setter");
    assert_hover(&env, Some(FIRST));
    env.exec(
        r#"
        local function addon()
            assert(debug.getstacktaint() == 'PetHoverProbe')
            assert(select('#', C_PetJournal.ClearHoveredBattlePet()) == 0)
            assert(debug.getstacktaint() == 'PetHoverProbe')
        end
        debug.setobjecttaint(addon, 'PetHoverProbe')
        addon()
        assert(debug.getstacktaint() == nil)
        "#,
    )
    .expect("tainted clear");
    assert_hover(&env, None);
}

#[test]
fn pet_hover_rejected_inputs_preserve_previous_owned_state() {
    let env = WowLuaEnv::new().expect("pet hover failure environment");
    env.state()
        .borrow_mut()
        .pet_journal_hover
        .hovered_battle_pet_guid = Some(FIRST.into());
    for argument in ["", "nil", "901001", "false", "{}", "function() end"] {
        env.exec(&format!(
            "assert(not pcall(function() C_PetJournal.SetHoveredBattlePet({argument}) end))"
        ))
        .expect("invalid required string rejects");
        assert_hover(&env, Some(FIRST));
    }
    for payload in ["901001", "false", "{}", "nil"] {
        env.exec(&format!(
            r#"
            debug.settaintmode(true)
            assert(debug.getstacktaint() == nil)
            local value = secretwrap({payload})
            assert(issecretvalue(value))
            local ok, message = pcall(C_PetJournal.SetHoveredBattlePet, value)
            assert(not ok and type(message) == 'string')
            assert(message:find('string', 1, true), 'secure input reaches type validation')
            assert(not message:find('untainted caller', 1, true))
            assert(issecretvalue(value), 'malformed wrapper retained')
            assert(debug.getstacktaint() == nil)
            "#
        ))
        .expect("untainted malformed secret rejects before mutation");
        assert_hover(&env, Some(FIRST));
    }
    // Each separate execution is followed by a host observation; a later successful
    // call cannot conceal an earlier security-error mutation.
    for payload in ["'rejected-secret-guid'", "901001", "false", "{}", "nil"] {
        env.exec(&format!(
            r#"
            debug.settaintmode(true)
            assert(debug.getstacktaint() == nil)
            local value = secretwrap({payload})
            assert(issecretvalue(value))
            local function addon()
                local ok, message = pcall(C_PetJournal.SetHoveredBattlePet, value)
                assert(not ok and message:find('untainted caller', 1, true))
                assert(issecretvalue(value), 'denied wrapper retained')
                assert(debug.getstacktaint() == 'PetHoverProbe', 'taint retained')
            end
            debug.setobjecttaint(addon, 'PetHoverProbe')
            addon()
            assert(debug.getstacktaint() == nil)
            "#
        ))
        .expect("security error precedes mutation");
        assert_hover(&env, Some(FIRST));
    }
}
