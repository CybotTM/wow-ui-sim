//! Supplied party AI membership contract; behavioral RED not yet observed.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use rilua::LuaApiMut;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::paths::default_blizzard_ui_addons_path;

const FOLLOWER_DUNGEON: i32 = 31001;
const ORDINARY_DUNGEON: i32 = 31002;

fn fixture() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("isolated party AI draft environment");
    env.exec(
        r#"
        A_Admin.SetPartySize(0)
        A_Admin.SetPartySize(2)
        A_Admin.SetPartyMember(1, 'Supplied Ally', 2, 80)
        A_Admin.SetPartyMember(2, 'Ordinary Ally', 1, 80)
        ClearTarget()
        ClearFocus()
        assert(UnitInParty('party1') and UnitInParty('party2'))
        assert(UnitGUID('party1') ~= UnitGUID('party2'))
        "#,
    )
    .expect("real active two-member roster, no fabricated AI entities");
    env
}

fn guid(env: &WowLuaEnv, unit: &str) -> String {
    env.eval(&format!("return UnitGUID({unit:?})"))
        .expect("fixture unit has an existing GUID")
}

fn supply_ai(env: &WowLuaEnv, unit: &str) {
    let existing_guid = guid(env, unit);
    // Environment-owned fixture input. No token-keyed Lua API or test-only producer.
    env.state()
        .borrow_mut()
        .party_ai_guids
        .insert(existing_guid);
}

fn is_ai(env: &WowLuaEnv, unit: &str) -> bool {
    let (count, kind, value): (i32, String, bool) = env
        .eval(&format!(
            "local function observe(...) return select('#', ...), type((...)), (...) end \
             return observe(UnitInPartyIsAI({unit:?}))"
        ))
        .expect("public AI membership query");
    assert_eq!(count, 1, "one non-nil result");
    assert_eq!(kind, "boolean");
    value
}

#[test]
fn supplied_party_guid_is_ai_ordinary_member_is_not() {
    let env = fixture();
    assert!(!is_ai(&env, "party1"), "empty supplied set");
    supply_ai(&env, "party1");
    assert!(is_ai(&env, "party1"));
    assert!(!is_ai(&env, "party2"));
    assert!(!is_ai(&env, "player"), "no implicit AI player");
    // INFERRED from current UnitInParty's active-player inclusion, not native AI proof.
    supply_ai(&env, "player");
    assert!(
        is_ai(&env, "player"),
        "explicit player GUID under existing group policy"
    );
    env.exec("assert(UnitIsPlayer('party1') and UnitIsHumanPlayer('party1'))")
        .expect("AI membership does not remodel existing human/player identity");
}

#[test]
fn aliases_follow_existing_guid_not_target_spelling_or_display_name() {
    let env = fixture();
    // Same display name is deliberately not an identity key.
    env.exec("A_Admin.SetPartyMember(2, 'Supplied Ally', 1, 80)")
        .unwrap();
    supply_ai(&env, "party1");
    env.exec("TargetUnit('party1'); FocusUnit('target')")
        .unwrap();
    assert_eq!(guid(&env, "target"), guid(&env, "party1"));
    assert_eq!(guid(&env, "focus"), guid(&env, "party1"));
    assert!(is_ai(&env, "target"));
    assert!(is_ai(&env, "focus"));
    env.exec("TargetUnit('party2')").unwrap();
    assert_eq!(guid(&env, "target"), guid(&env, "party2"));
    assert!(!is_ai(&env, "target"), "new target does not inherit AI");
    assert!(is_ai(&env, "focus"), "retained alias still names AI member");
}

#[test]
fn missing_unknown_and_supplied_nonparty_identity_are_false() {
    let env = fixture();
    supply_ai(&env, "party1");
    for unit in [
        "",
        "unknown-unit",
        "party3",
        "target",
        "focus",
        "pet",
        "vehicle",
    ] {
        assert!(!is_ai(&env, unit), "no existing eligible identity: {unit}");
    }
    env.exec("TargetUnit('enemy1'); FocusUnit('target')")
        .unwrap();
    let outsider = guid(&env, "target");
    assert_ne!(outsider, guid(&env, "party1"));
    assert_ne!(outsider, guid(&env, "party2"));
    env.state().borrow_mut().party_ai_guids.insert(outsider);
    assert!(
        !is_ai(&env, "target"),
        "supplied identity is not on party roster"
    );
    assert!(
        !is_ai(&env, "focus"),
        "same nonparty identity via another alias"
    );
    assert!(
        is_ai(&env, "party1"),
        "negative controls do not clear real input"
    );
}

#[test]
fn live_inactivation_and_roster_removal_clear_effective_membership() {
    let env = fixture();
    supply_ai(&env, "party2");
    supply_ai(&env, "player");
    env.exec("TargetUnit('party2'); FocusUnit('target')")
        .unwrap();
    let removed_guid = guid(&env, "party2");
    assert!(is_ai(&env, "focus"));
    env.state().borrow_mut().party_group_active = false;
    for unit in ["party2", "target", "focus", "player"] {
        assert!(!is_ai(&env, unit), "inactive group: {unit}");
    }
    env.state().borrow_mut().party_group_active = true;
    assert!(
        is_ai(&env, "focus"),
        "effective gate, not destructive set clearing"
    );
    env.exec("A_Admin.SetPartySize(1)").unwrap();
    let absent: Option<String> = env.eval("return UnitGUID('party2')").unwrap();
    assert_eq!(absent, None);
    assert_eq!(guid(&env, "target"), removed_guid);
    assert_eq!(guid(&env, "focus"), removed_guid);
    for unit in ["party2", "target", "focus"] {
        assert!(
            !is_ai(&env, unit),
            "removed identity despite retained snapshots: {unit}"
        );
    }
    env.exec("A_Admin.SetPartySize(0)").unwrap();
    assert!(
        !is_ai(&env, "player"),
        "explicit player classification is group-gated"
    );
    // No assertion about physical set pruning or future slot reuse.
}

#[test]
fn supplied_membership_is_reversible_and_environment_owned() {
    let first = fixture();
    let second = fixture();
    assert_eq!(guid(&first, "party1"), guid(&second, "party1"));
    supply_ai(&first, "party1");
    assert!(is_ai(&first, "party1"));
    assert!(!is_ai(&second, "party1"));
    supply_ai(&second, "party1");
    first.state().borrow_mut().party_ai_guids.clear();
    assert!(!is_ai(&first, "party1"));
    assert!(is_ai(&second, "party1"));
    assert_eq!(first.state().borrow().party_members.len(), 2);
    assert_eq!(second.state().borrow().party_members.len(), 2);
}

fn set_dungeon(env: &WowLuaEnv, follower: bool) {
    let mut state = env.state().borrow_mut();
    let mut row = state
        .lfd_dungeons
        .first()
        .expect("existing catalog row")
        .clone();
    row.dungeon_id = if follower {
        FOLLOWER_DUNGEON
    } else {
        ORDINARY_DUNGEON
    };
    row.name = "Party AI Draft Dungeon".into();
    row.is_follower_dungeon = follower;
    state.world.in_instance = true;
    state.world.instance_lfg_dungeon_id = Some(row.dungeon_id);
    state.lfd_dungeons = vec![row];
}

#[test]
fn follower_dungeon_does_not_supply_ai_and_ai_does_not_require_dungeon() {
    let env = fixture();
    let party_guid = guid(&env, "party1");
    set_dungeon(&env, true);
    env.exec("assert(C_LFGInfo.IsInLFGFollowerDungeon())")
        .unwrap();
    assert!(!is_ai(&env, "party1"));
    assert!(!is_ai(&env, "party2"));
    assert_eq!(
        env.state().borrow().party_members.len(),
        2,
        "no spawned followers"
    );
    assert_eq!(guid(&env, "party1"), party_guid);
    supply_ai(&env, "party1");
    set_dungeon(&env, false);
    assert!(is_ai(&env, "party1"), "AI is not dungeon classification");
    env.state().borrow_mut().world.in_instance = false;
    assert!(
        is_ai(&env, "party1"),
        "explicit current party AI outside instances"
    );
}

#[test]
fn nullable_argument_and_plain_type_policy_are_preserved() {
    let env = fixture();
    supply_ai(&env, "party1");
    env.exec(
        r#"
        assert(select('#', UnitInPartyIsAI()) == 1)
        assert(UnitInPartyIsAI() == false)
        assert(select('#', UnitInPartyIsAI(nil)) == 1)
        assert(UnitInPartyIsAI(nil) == false)
        assert(not pcall(UnitInPartyIsAI, {}))
        assert(not pcall(UnitInPartyIsAI, true))
        assert(not pcall(UnitInPartyIsAI, 1))
        assert(not pcall(UnitInPartyIsAI, function() end))
        assert(UnitInPartyIsAI('party1', 'ignored extra argument') == true)
        "#,
    )
    .expect("existing nullable strict-string extraction and ignored extras");
}

#[test]
fn documented_secret_argument_authentication_preserves_caller_and_wrapper() {
    let env = fixture();
    supply_ai(&env, "party1");
    {
        let loader = env.loader_env();
        let mut lua = loader.rilua_mut();
        let token = rilua::table_security::wrap_host_secret_string(lua.state_mut(), "party1");
        lua.state_mut().push(token);
        let published = lua.set_global_val("PartyAISecretToken", token);
        lua.state_mut().pop();
        published.expect("root authentic VM secret string, not a fabricated wrapper");
        let number = rilua::table_security::wrap_host_secret_number(lua.state_mut(), 1.0);
        lua.state_mut().push(number);
        let published = lua.set_global_val("PartyAISecretNumber", number);
        lua.state_mut().pop();
        published.expect("root authentic VM secret wrong-type control");
    }
    env.exec(
        r#"
        assert(debug.getstacktaint() == nil)
        assert(issecretvalue(PartyAISecretToken))
        assert(UnitInPartyIsAI(PartyAISecretToken) == true)
        assert(not issecretvalue(UnitInPartyIsAI('party1')))
        local function secureInputTypeStringError(input)
            local ok, message = pcall(UnitInPartyIsAI, input)
            assert(not ok and type(message) == 'string', 'invalid secure input must error')
            assert(message:find('string', 1, true), 'required-string validation')
            assert(not message:find('untainted caller', 1, true), 'not permission denial')
        end
        secureInputTypeStringError(PartyAISecretNumber)
        assert(debug.getstacktaint() == nil)
        local function addonQuery()
            assert(debug.getstacktaint() == 'PartyAIDraftAddon')
            local ok, message = pcall(UnitInPartyIsAI, PartyAISecretToken)
            assert(not ok and type(message) == 'string', 'tainted secret input must error')
            assert(message:find('untainted caller', 1, true), 'secret authentication denial')
            assert(not message:find('expected string', 1, true), 'not raw string extraction')
            assert(debug.getstacktaint() == 'PartyAIDraftAddon')
            assert(UnitInPartyIsAI('party1') == true)
            assert(debug.getstacktaint() == 'PartyAIDraftAddon')
            assert(issecretvalue(PartyAISecretToken))
        end
        debug.setobjecttaint(addonQuery, 'PartyAIDraftAddon')
        addonQuery()
        assert(debug.getstacktaint() == nil)
        collectgarbage('collect')
        assert(issecretvalue(PartyAISecretToken))
        assert(UnitInPartyIsAI(PartyAISecretToken) == true)
        "#,
    )
    .expect("cached AllowedWhenUntainted contract; source stub currently lacks decoding");
}

fn load_cached_name_consumer(env: &WowLuaEnv) {
    let path = default_blizzard_ui_addons_path()
        .expect("active profile cache already synced; draft never syncs it")
        .join("Blizzard_UnitFrame/Shared/CompactUnitFrame.lua");
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read unchanged cached caller {}: {error}", path.display()));
    // Existing lfg_follower_instance.rs extraction pattern: exact contiguous globals.
    // These two definitions have no required file-local captures on selected path.
    for declaration in [
        "function CompactUnitFrame_IsTapDenied(frame)",
        "function CompactUnitFrame_UpdateName(frame)",
    ] {
        let start = source.find(declaration).expect("cached declaration");
        let tail = &source[start..];
        let end = tail.find("\nend").expect("top-level terminator") + "\nend".len();
        env.exec(&tail[..end])
            .expect("unchanged cached function definition");
    }
}

fn assert_rendered_name(env: &WowLuaEnv, unit: &str, prefixed: bool) {
    env.exec(&format!(
        "PartyAINameFrame.unit = {unit:?}; PartyAINameFrame.displayedUnit = {unit:?}; \
         CompactUnitFrame_UpdateName(PartyAINameFrame)"
    ))
    .expect("real cached name consumer");
    let base: String = env
        .eval(&format!("return GetUnitName({unit:?}, true)"))
        .unwrap();
    let expected = if prefixed {
        format!("Follower: {base}")
    } else {
        base
    };
    let actual: String = env.eval("return PartyAIName:GetText()").unwrap();
    assert_eq!(actual, expected, "unit={unit}, prefixed={prefixed}");
    assert!(env.eval::<bool>("return PartyAIName:IsShown()").unwrap());
}

#[test]
fn cached_name_consumer_uses_real_ai_query_and_live_membership() {
    let env = fixture();
    load_cached_name_consumer(&env);
    env.exec(
        r#"
        assert(C_PartyInfo.IsPartyWalkIn() == false, 'existing inert control, not override')
        assert(C_Commentator.IsSpectating() == false, 'existing inert control, not override')
        LFG_FOLLOWER_NAME_PREFIX = 'Follower: %s' -- formatting sentinel only
        local holder = CreateFrame('Frame')
        PartyAIName = holder:CreateFontString(nil, 'OVERLAY')
        PartyAINameFrame = {
            unit = 'party1', displayedUnit = 'party1', name = PartyAIName,
            optionTable = { updateNameUsesGetUnitName = true },
            ShouldShowName = function() return true end,
        }
        "#,
    )
    .expect("real FontString with minimal compact frame fixture; no query replacement");
    set_dungeon(&env, true);
    assert_rendered_name(&env, "party1", false);
    supply_ai(&env, "party1");
    assert_rendered_name(&env, "party1", true);
    assert_rendered_name(&env, "party2", false);
    env.exec("TargetUnit('party1'); FocusUnit('target')")
        .unwrap();
    assert_rendered_name(&env, "focus", true);
    set_dungeon(&env, false);
    assert_rendered_name(&env, "party1", false);
    set_dungeon(&env, true);
    assert_rendered_name(&env, "party1", true);
    env.exec("A_Admin.SetPartySize(0)").unwrap();
    assert_eq!(guid(&env, "focus"), guid(&env, "target"));
    assert_rendered_name(&env, "focus", false); // retained named snapshot, no missing-name formatting
}
