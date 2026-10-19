//! Supplied manual Wrath watches through unchanged original consumers.
#![cfg(feature = "client-wrath")]

#[path = "common/wrath_full_ui.rs"]
mod wrath_full_ui;

use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::GossipQuestRow;

struct Case {
    env: WowLuaEnv,
    errors_before: String,
}

fn errors(env: &WowLuaEnv) -> String {
    let sim = env.state().borrow();
    let counts: std::collections::BTreeMap<_, _> = sim.lua_error_counts.iter().collect();
    serde_json::to_string(&(&sim.lua_errors, counts)).unwrap()
}

fn exec(case: &Case, source: &str) {
    let result = case.env.exec(source);
    let caught: String = case
        .env
        .eval("return table.concat(N3Errors, '\\n')")
        .unwrap();
    assert!(caught.is_empty(), "{caught}");
    assert_eq!(
        errors(&case.env),
        case.errors_before,
        "new simulator errors"
    );
    result.expect("unchanged original consumer and concrete assertions");
}

fn assert_original_sources() {
    for (path, expected) in [
        (
            "/home/osso/.cache/wow-ui-sim/blizzard-ui/wrath/FrameXML/QuestLogFrame.lua",
            "6ac6ca92b3036417082b88cc2f8b5c9fc8889f51f82aa726fe0bcd44990bc5e2",
        ),
        (
            "/home/osso/.cache/wow-ui-sim/blizzard-ui/wrath/FrameXML/WatchFrame.lua",
            "ad67e96468a14734212aa53cb62c43619f4006dc15173affb43a0be29476751b",
        ),
        (
            "/home/osso/.cache/wow-ui-sim/blizzard-ui/wrath/FrameXML/QuestLogFrame.xml",
            "530c396e36af4b23a725086d37b6cb68abaf1966db77ba26c4e3fed3ce76f154",
        ),
        (
            "/home/osso/.cache/wow-ui-sim/blizzard-ui/wrath/FrameXML/WatchFrame.xml",
            "d957db59c5a827f06d586333701de58a1585172f7c3d07625604fb42c5aebf58",
        ),
    ] {
        let bytes = std::fs::read(path).unwrap();
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), expected, "{path}");
    }
}

fn supply_active_rows(env: &WowLuaEnv) {
    let mut sim = env.state().borrow_mut();
    sim.quest_log_entries.entries = [
        (991301, "N3 supplied A"),
        (991302, "N3 supplied B"),
        (991303, "N3 supplied C"),
        (991304, "N3 supplied D"),
    ]
    .into_iter()
    .map(|(quest_id, title)| {
        sim.quest_log_entries
            .snapshot_gossip_offer(&GossipQuestRow {
                quest_id,
                title: title.into(),
                quest_level: 23,
                ..Default::default()
            })
            .unwrap()
    })
    .collect();
    sim.quest_log = vec![991301, 991302, 991303, 991304];
    sim.selected_quest_log_id = None;
    sim.abandon_quest_id = None;
    sim.pending_quest_offer = None;
}

fn supplied() -> Case {
    assert_original_sources();
    let env = wrath_full_ui::load_original_wrath_startup();
    let errors_before = errors(&env);
    eprintln!("N3 full-startup snapshot: {errors_before}");
    assert_eq!(
        errors_before,
        include_str!("fixtures/wrath_manual_watch_startup_errors.json").trim(),
        "full-startup error precondition changed"
    );
    env.exec("N3Errors = {}; seterrorhandler(function(e) N3Errors[#N3Errors+1] = tostring(e) end)")
        .unwrap();
    supply_active_rows(&env);
    env.exec(include_str!("fixtures/wrath_quest_watch_manual.lua"))
        .unwrap();
    let case = Case { env, errors_before };
    exec(&case, "N3.prepare(); N3.request_membership({2}); N3.open()");
    case
}

#[test]
fn n3_supplied_manual_membership_reaches_original_visible_ui() {
    let case = supplied();
    exec(&case, "N3.expect_membership({[2]='N3 supplied B'})");
}

#[test]
fn n3_original_shift_click_adds_supplied_first_row() {
    let case = supplied();
    // Separate case reaches original add branch before membership oracle.
    exec(
        &case,
        "N3.click_log(1); N3.expect_membership({[1]='N3 supplied A',[2]='N3 supplied B'})",
    );
}

#[test]
fn n3_original_shift_click_removes_supplied_watched_row() {
    let case = supplied();
    exec(&case, "N3.click_log(2); N3.expect_membership({})");
}

#[test]
fn n3_original_tracker_shift_click_unwatches() {
    let case = supplied();
    exec(
        &case,
        r#"
        N3.expect_membership({[2]='N3 supplied B'})
        local link
        for _, b in ipairs(WATCHFRAME_LINKBUTTONS) do
            if b:IsVisible() and b.type == 'QUEST' and GetQuestIndexForWatch(b.index) == 2 then link = b end
        end
        assert(link, 'real original tracker link')
        A_Admin.SetShiftKeyDown(true)
        link:Click('LeftButton')
        A_Admin.SetShiftKeyDown(false)
        N3.expect_membership({})
    "#,
    );
}

#[test]
fn n3_reordered_supplied_snapshot_uses_current_log_indices() {
    let case = supplied();
    exec(&case, "N3.expect_membership({[2]='N3 supplied B'})");
    {
        let mut sim = case.env.state().borrow_mut();
        sim.quest_log_entries.entries.swap(0, 1);
        sim.quest_log.swap(0, 1);
    }
    // This deliberately supplies a NEW manual membership snapshot after reorder.
    // It tests current-index/display coherence, NOT automatic native remapping.
    exec(
        &case,
        "N3.request_membership({1}); QuestLog_Update(); WatchFrame_Update(); N3.expect_membership({[1]='N3 supplied B'})",
    );
    // PENDING: with no resupply, does B remain watched, and in what watch order?
}

#[test]
fn n3_original_abandon_of_watched_row_has_no_stale_visible_identity() {
    let case = supplied();
    exec(
        &case,
        r#"
        N3.expect_membership({[2]='N3 supplied B'})
        QuestLog_SetSelection(2)
        QuestLogFrameAbandonButton:Click()
        local popup = StaticPopup_FindVisible('ABANDON_QUEST')
        assert(popup and popup:IsVisible())
        popup.button1:Click()
    "#,
    );
    assert!(
        !case
            .env
            .state()
            .borrow()
            .quest_log_entries
            .entries
            .iter()
            .any(|q| q.quest_id == 991302)
    );
    let queued = case.env.state().borrow_mut().events.drain();
    for event in queued {
        // Deliver only the already-merged producer's real queued log updates.
        // Do not fabricate native watch events or assert their timing.
        if event.name == "QUEST_LOG_UPDATE" {
            case.env.fire_event(&event.name).unwrap();
        }
    }
    exec(&case, "N3.expect_no_removed_identity(991302)");
    // PENDING: exact post-removal watch count/order, survivor remapping, native trace.
}
