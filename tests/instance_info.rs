//! Tests for instance / mirror-timer probe globals backed by
//! `SimState.world`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::MirrorTimer;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_instance_info_defaults_to_open_world() {
    let env = env();
    let (name, kind, diff_id, diff_name, max_players, dyn_diff, is_dyn): (
        String,
        String,
        i32,
        String,
        i32,
        i32,
        bool,
    ) = env
        .eval(
            r#"
            local name, kind, diffID, diffName, maxPlayers, dynDiff, isDyn =
                GetInstanceInfo()
            return name, kind, diffID, diffName, maxPlayers, dynDiff, isDyn
            "#,
        )
        .unwrap();

    assert_eq!(name, "", "open world: no instance name");
    assert_eq!(kind, "none");
    assert_eq!(diff_id, 0);
    assert_eq!(diff_name, "");
    assert_eq!(max_players, 0);
    assert_eq!(dyn_diff, 0);
    assert!(!is_dyn);
}

#[test]
fn get_instance_info_reports_all_ten_fields() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.instance_name = "Nerub'ar Palace".into();
        state.world.instance_type = "raid".into();
        state.world.instance_difficulty = 16;
        state.world.instance_difficulty_name = "Mythic".into();
        state.world.instance_max_players = 20;
        state.world.instance_dynamic_difficulty = 0;
        state.world.instance_is_dynamic = false;
        state.world.instance_id = 2657;
        state.world.instance_group_size = 20;
        state.world.instance_lfg_dungeon_id = Some(2295);
        state.world.in_instance = true;
    }

    let (
        name,
        kind,
        diff_id,
        diff_name,
        max_players,
        dyn_diff,
        is_dyn,
        instance_id,
        group_size,
        lfg_dungeon_id,
    ): (String, String, i32, String, i32, i32, bool, i32, i32, i32) = env
        .eval(
            r#"
            return GetInstanceInfo()
            "#,
        )
        .unwrap();

    assert_eq!(name, "Nerub'ar Palace");
    assert_eq!(kind, "raid");
    assert_eq!(diff_id, 16);
    assert_eq!(diff_name, "Mythic");
    assert_eq!(max_players, 20);
    assert_eq!(dyn_diff, 0);
    assert!(!is_dyn);
    assert_eq!(instance_id, 2657);
    assert_eq!(group_size, 20);
    assert_eq!(lfg_dungeon_id, 2295);
}

#[test]
fn get_instance_info_returns_nil_for_absent_lfg_dungeon_id() {
    let env = env();
    let lfg_nil: bool = env
        .eval("return select(10, GetInstanceInfo()) == nil")
        .unwrap();
    assert!(
        lfg_nil,
        "lfgDungeonID is nil when not queued via Group Finder"
    );
}

#[test]
fn saved_raid_instance_counts_default_to_empty_lists() {
    let env = env();
    let (saved_instances, world_bosses): (i32, i32) = env
        .eval("return GetNumSavedInstances(), GetNumSavedWorldBosses()")
        .unwrap();

    assert_eq!(saved_instances, 0);
    assert_eq!(world_bosses, 0);
}

#[test]
fn absent_saved_raid_instance_rows_return_nil_fields() {
    let env = env();
    let (instance_is_nil, boss_is_nil, extend_return_count): (bool, bool, i32) = env
        .eval(
            r##"
            local instanceName = GetSavedInstanceInfo(1)
            local bossName = GetSavedWorldBossInfo(1)
            return instanceName == nil, bossName == nil, select("#", SetSavedInstanceExtend(1, true))
            "##,
        )
        .unwrap();

    assert!(instance_is_nil);
    assert!(boss_is_nil);
    assert_eq!(extend_return_count, 0);
}

#[test]
fn get_mirror_timer_info_returns_unknown_sentinel_when_unset() {
    let env = env();
    let (name, start, max, scale, paused, label, spell_id): (
        String,
        f64,
        f64,
        f64,
        i32,
        String,
        i32,
    ) = env.eval("return GetMirrorTimerInfo(1)").unwrap();
    assert_eq!(name, "UNKNOWN");
    assert_eq!(start, 0.0);
    assert_eq!(max, 0.0);
    assert_eq!(scale, 0.0);
    assert_eq!(paused, 0);
    assert_eq!(label, "");
    assert_eq!(spell_id, 0);
}

#[test]
fn get_mirror_timer_info_returns_seven_values_by_index() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.mirror_timers.push(MirrorTimer {
            name: "BREATH".into(),
            start_value: 60000.0,
            max_value: 180000.0,
            scale: -1.0,
            paused: 0,
            label: "Breath".into(),
            spell_id: 58428,
            progress: 58000.0,
        });
    }

    let (name, start, max, scale, paused, label, spell_id): (
        String,
        f64,
        f64,
        f64,
        i32,
        String,
        i32,
    ) = env.eval("return GetMirrorTimerInfo(1)").unwrap();

    assert_eq!(name, "BREATH");
    assert_eq!(start, 60000.0);
    assert_eq!(max, 180000.0);
    assert_eq!(scale, -1.0);
    assert_eq!(paused, 0);
    assert_eq!(label, "Breath");
    assert_eq!(spell_id, 58428);
}

#[test]
fn get_mirror_timer_info_out_of_range_returns_unknown_sentinel() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.mirror_timers.push(MirrorTimer {
            name: "BREATH".into(),
            ..MirrorTimer::default()
        });
    }

    let (name, start, max, scale, paused, label, spell_id): (
        String,
        f64,
        f64,
        f64,
        i32,
        String,
        i32,
    ) = env.eval("return GetMirrorTimerInfo(99)").unwrap();
    assert_eq!(name, "UNKNOWN");
    assert_eq!(start, 0.0);
    assert_eq!(max, 0.0);
    assert_eq!(scale, 0.0);
    assert_eq!(paused, 0);
    assert_eq!(label, "");
    assert_eq!(spell_id, 0);
}

#[test]
fn get_mirror_timer_progress_reads_by_name() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.mirror_timers.push(MirrorTimer {
            name: "BREATH".into(),
            progress: 42000.0,
            ..MirrorTimer::default()
        });
        state.world.mirror_timers.push(MirrorTimer {
            name: "EXHAUSTION".into(),
            progress: 5000.0,
            ..MirrorTimer::default()
        });
    }

    let breath: f64 = env
        .eval(r#"return GetMirrorTimerProgress("BREATH")"#)
        .unwrap();
    assert_eq!(breath, 42000.0);

    let exhaustion: f64 = env
        .eval(r#"return GetMirrorTimerProgress("EXHAUSTION")"#)
        .unwrap();
    assert_eq!(exhaustion, 5000.0);

    let missing_is_nil: bool = env
        .eval(r#"return GetMirrorTimerProgress("FEIGNDEATH") == nil"#)
        .unwrap();
    assert!(missing_is_nil);
}
