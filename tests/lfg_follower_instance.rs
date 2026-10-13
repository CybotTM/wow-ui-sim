//! Current follower-instance membership over existing world and dungeon state.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::paths::default_blizzard_ui_addons_path;

const FOLLOWER: i32 = 31001;
const ORDINARY: i32 = 31002;
const UNKNOWN: i32 = 31999;

fn fixture() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("create isolated LFG environment");
    {
        let mut state = env.state().borrow_mut();
        // Clone an existing catalog row, avoiding any new production fields.
        let mut follower = state
            .lfd_dungeons
            .first()
            .expect("seed catalog row")
            .clone();
        follower.dungeon_id = FOLLOWER;
        follower.name = "Fixture Follower Dungeon".into();
        follower.is_follower_dungeon = true;
        let mut ordinary = follower.clone();
        ordinary.dungeon_id = ORDINARY;
        ordinary.name = "Fixture Ordinary Dungeon".into();
        ordinary.is_follower_dungeon = false;
        state.lfd_dungeons = vec![follower, ordinary];
        state.world.in_instance = false;
        state.world.instance_lfg_dungeon_id = None;
    }
    env
}

fn set_current_instance(env: &WowLuaEnv, in_instance: bool, dungeon_id: Option<i32>) {
    let mut state = env.state().borrow_mut();
    state.world.in_instance = in_instance;
    state.world.instance_lfg_dungeon_id = dungeon_id;
}

fn query(env: &WowLuaEnv) -> bool {
    let (count, kind, result): (i32, String, bool) = env
        .eval(
            r#"
            local function observe(...)
                return select('#', ...), type((...)), (...)
            end
            return observe(C_LFGInfo.IsInLFGFollowerDungeon())
            "#,
        )
        .expect("zero-argument follower query");
    assert_eq!(count, 1, "one non-nil result");
    assert_eq!(kind, "boolean");
    result
}

#[test]
fn in_lfg_follower_dungeon_reports_active_catalog_classification() {
    let env = fixture();
    set_current_instance(&env, true, Some(FOLLOWER));
    assert!(query(&env));
    // Existing classifier is a control, not an additional validation contract.
    assert!(
        env.eval::<bool>("return C_LFGInfo.IsLFGFollowerDungeon(31001)")
            .unwrap()
    );
}

#[test]
fn in_lfg_follower_dungeon_is_false_without_active_known_follower() {
    let env = fixture();
    for (label, in_instance, dungeon_id) in [
        ("ordinary catalog row", true, Some(ORDINARY)),
        ("unknown catalog ID", true, Some(UNKNOWN)),
        ("missing current LFG ID", true, None),
        ("outside with stale follower ID", false, Some(FOLLOWER)),
        ("outside without ID", false, None),
    ] {
        set_current_instance(&env, in_instance, dungeon_id);
        assert!(!query(&env), "{label}");
    }
}

#[test]
fn in_lfg_follower_dungeon_observes_live_world_and_catalog_changes() {
    let env = fixture();
    set_current_instance(&env, true, Some(FOLLOWER));
    assert!(query(&env));
    env.state().borrow_mut().lfd_dungeons[0].is_follower_dungeon = false;
    assert!(!query(&env), "classification is not cached");
    env.state().borrow_mut().lfd_dungeons[0].is_follower_dungeon = true;
    assert!(query(&env));
    set_current_instance(&env, true, Some(ORDINARY));
    assert!(!query(&env), "current ID is not cached");
    set_current_instance(&env, true, Some(FOLLOWER));
    assert!(query(&env));
    env.state().borrow_mut().lfd_dungeons.remove(0);
    assert!(!query(&env), "removed classification is an unknown ID");
}

#[test]
fn in_lfg_follower_dungeon_isolated_between_environments() {
    let first = fixture();
    let second = fixture();
    set_current_instance(&first, true, Some(FOLLOWER));
    set_current_instance(&second, true, Some(ORDINARY));
    assert!(query(&first));
    assert!(!query(&second));
    second.state().borrow_mut().lfd_dungeons[1].is_follower_dungeon = true;
    assert!(query(&second));
    first.state().borrow_mut().world.in_instance = false;
    assert!(!query(&first));
    assert!(query(&second), "world changes do not cross environments");
    assert!(!first.state().borrow().lfd_dungeons[1].is_follower_dungeon);
}

#[test]
fn admin_instance_optional_lfg_id_drives_existing_instance_outputs() {
    let env = fixture();
    env.exec(r#"A_Admin.SetInstanceInfo("Host Follower", "party", 1, 5, 31001)"#)
        .unwrap();
    assert_eq!(
        env.state().borrow().world.instance_lfg_dungeon_id,
        Some(FOLLOWER)
    );
    assert!(query(&env));
    env.exec(
        r#"
        local name, kind, difficulty, _, players, _, _, _, _, lfgID = GetInstanceInfo()
        assert(name == "Host Follower" and kind == "party")
        assert(difficulty == 1 and players == 5 and lfgID == 31001)
        assert(IsInInstance() == true and IsInLFGDungeon() == true)
        A_Admin.SetInstanceInfo("Host Ordinary", "party", 2, 5, 31002)
        "#,
    )
    .unwrap();
    assert!(!query(&env));
    env.exec(r#"A_Admin.SetInstanceInfo("Host Unknown", "party", 1, 5, 31999)"#)
        .unwrap();
    assert!(
        !query(&env),
        "host ID is stored, not guessed or catalog-validated"
    );
    assert_eq!(
        env.state().borrow().world.instance_lfg_dungeon_id,
        Some(UNKNOWN)
    );
}

#[test]
fn admin_instance_legacy_four_args_and_explicit_nil_clear_lfg_id() {
    let env = fixture();
    for call in [
        r#"A_Admin.SetInstanceInfo("Legacy Instance", "party", 23, 5)"#,
        r#"A_Admin.SetInstanceInfo("Legacy Instance", "party", 23, 5, nil)"#,
    ] {
        env.exec(r#"A_Admin.SetInstanceInfo("Host Follower", "party", 1, 5, 31001)"#)
            .unwrap();
        assert!(query(&env));
        env.exec(call).unwrap();
        assert_eq!(env.state().borrow().world.instance_lfg_dungeon_id, None);
        assert!(!query(&env));
        env.exec(
            r#"
            local name, kind, difficulty, _, players = GetInstanceInfo()
            assert(name == "Legacy Instance" and kind == "party")
            assert(difficulty == 23 and players == 5)
            assert(IsInInstance() == true)
            assert(select(10, GetInstanceInfo()) == nil)
            assert(IsInLFGDungeon() == false)
            "#,
        )
        .unwrap();
    }
}

#[test]
fn admin_in_instance_toggle_gates_but_does_not_clear_current_lfg_id() {
    let env = fixture();
    env.exec(r#"A_Admin.SetInstanceInfo("Host Follower", "party", 1, 5, 31001)"#)
        .unwrap();
    assert!(query(&env));
    env.exec("A_Admin.SetInInstance(false)").unwrap();
    assert!(!query(&env));
    assert_eq!(
        env.state().borrow().world.instance_lfg_dungeon_id,
        Some(FOLLOWER)
    );
    env.exec("A_Admin.SetInInstance(true)").unwrap();
    assert!(query(&env), "preserve existing flag-only setter semantics");
}

fn load_cached_compact_name_caller(env: &WowLuaEnv) {
    let path = default_blizzard_ui_addons_path()
        .expect("active profile cache must already be synced")
        .join("Blizzard_UnitFrame/Shared/CompactUnitFrame.lua");
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read unchanged caller {}: {error}", path.display()));
    // Execute exact contiguous cached function definitions, not rewritten logic.
    // These two functions have no captured file-local dependencies on this path.
    for declaration in [
        "function CompactUnitFrame_IsTapDenied(frame)",
        "function CompactUnitFrame_UpdateName(frame)",
    ] {
        let start = source
            .find(declaration)
            .expect("cached function declaration");
        let tail = &source[start..];
        let end = tail.find("\nend").expect("top-level function terminator") + "\nend".len();
        env.exec(&tail[..end])
            .expect("load unchanged cached function");
    }
}

#[test]
fn cached_compact_name_caller_changes_prefix_with_live_follower_instance() {
    let env = fixture();
    load_cached_compact_name_caller(&env);
    env.exec(
        r#"
        -- Fixture-only simulator probes: no vendor function is replaced.
        -- UnitInPartyIsAI currently returns constant false; AI state is out of scope.
        UnitInPartyIsAI = function(unit) return unit == "player" end
        C_PartyInfo.IsPartyWalkIn = function() return false end
        C_Commentator.IsSpectating = function() return false end
        -- Stable formatting sentinel; not a localization/native wording assertion.
        LFG_FOLLOWER_NAME_PREFIX = "Follower: %s"
        local holder = CreateFrame("Frame")
        FollowerDraftName = holder:CreateFontString(nil, "OVERLAY")
        FollowerDraftFrame = {
            unit = "player", displayedUnit = "player", name = FollowerDraftName,
            optionTable = { updateNameUsesGetUnitName = true },
            ShouldShowName = function() return true end,
        }
        FollowerDraftBaseName = GetUnitName("player", true)
        assert(type(FollowerDraftBaseName) == "string")
        "#,
    )
    .unwrap();
    for (in_instance, dungeon_id, prefixed) in [
        (true, Some(FOLLOWER), true),
        (true, Some(ORDINARY), false),
        (true, Some(UNKNOWN), false),
        (true, None, false),
        (false, Some(FOLLOWER), false),
        (true, Some(FOLLOWER), true),
    ] {
        set_current_instance(&env, in_instance, dungeon_id);
        env.exec("CompactUnitFrame_UpdateName(FollowerDraftFrame)")
            .unwrap();
        let expected: String = env
            .eval(if prefixed {
                "return LFG_FOLLOWER_NAME_PREFIX:format(FollowerDraftBaseName)"
            } else {
                "return FollowerDraftBaseName"
            })
            .unwrap();
        let actual: String = env.eval("return FollowerDraftName:GetText()").unwrap();
        assert_eq!(
            actual, expected,
            "in_instance={in_instance}, ID={dungeon_id:?}"
        );
        assert!(
            env.eval::<bool>("return FollowerDraftName:IsShown()")
                .unwrap()
        );
    }
}
