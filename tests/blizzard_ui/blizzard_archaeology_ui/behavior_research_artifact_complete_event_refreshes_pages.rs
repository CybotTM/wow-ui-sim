//! Completion-event handling for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use crate::common::panel_fixtures::{blizzard_ui_dir, load_panel_addons};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ArchaeologyArtifact, ArchaeologyRace, SelectedArtifact};
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArchaeologyUI";
const ARTIFACT_NAME: &str = "Canopic Jar";

#[test]
fn archaeology_research_artifact_complete_event_plays_glow_without_page_refresh() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);

    load_panel_addons(&env);
    seed_completed_artifact_event_state(&env);
    load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);

    let completion_event = fire_completion_event_probe(&env);
    assert_completion_event_plays_artifact_glow(&completion_event);
    assert_completion_event_does_not_refresh_pages(&completion_event);

    update_summary_race_counts(&env);
    let generic_refresh = fire_generic_update_probe(&env);
    assert_generic_event_refreshes_active_summary_page(&generic_refresh);
}

struct CompletionEventProbe {
    glow_playing: bool,
    glow_frame_level: i32,
    frame_level: i32,
    summary_updates: i32,
    completed_updates: i32,
}

struct GenericRefreshProbe {
    summary_updates: i32,
    completed_updates: i32,
    summary_row_text: String,
}

fn seed_completed_artifact_event_state(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology.races = vec![ArchaeologyRace {
        name: "Tol'vir".to_string(),
        texture: 442739,
        race_item_id: 0,
        currency_amount: 4,
        project_amount: 35,
        artifacts: vec![ArchaeologyArtifact::default()],
    }];
    sim.archaeology.selected = Some(SelectedArtifact {
        race_id: 1,
        artifact_id: None,
        name: ARTIFACT_NAME.to_string(),
        description: "A jar sealed with ancient resin.".to_string(),
        rarity: 0,
        icon: 134400,
        spell_description: "Preserved for an impossible age.".to_string(),
        num_sockets: 0,
        base_progress: 4,
        adjust_progress: 0,
        total_cost: 35,
        can_solve: false,
        ..SelectedArtifact::default()
    });
}

fn fire_completion_event_probe(env: &WowLuaEnv) -> CompletionEventProbe {
    let (glow_playing, glow_frame_level, frame_level, summary_updates, completed_updates) = env
        .eval(
            r#"
            ArchaeologyFrame_ShowArtifact(1)

            local summaryUpdates = 0
            local completedUpdates = 0
            local summaryUpdate = ArchaeologyFrame.summaryPage.UpdateFrame
            local completedUpdate = ArchaeologyFrame.completedPage.UpdateFrame

            ArchaeologyFrame.summaryPage.UpdateFrame = function(self)
                summaryUpdates = summaryUpdates + 1
                return summaryUpdate(self)
            end
            ArchaeologyFrame.completedPage.UpdateFrame = function(self)
                completedUpdates = completedUpdates + 1
                return completedUpdate(self)
            end

            FireEvent("RESEARCH_ARTIFACT_COMPLETE", "Canopic Jar")

            return ArchaeologyFrame.artifactPage.glow.completeAnim:IsPlaying(),
                   ArchaeologyFrame.artifactPage.glow:GetFrameLevel(),
                   ArchaeologyFrame:GetFrameLevel(),
                   summaryUpdates,
                   completedUpdates
            "#,
        )
        .expect("RESEARCH_ARTIFACT_COMPLETE probe must run cleanly");

    CompletionEventProbe {
        glow_playing,
        glow_frame_level,
        frame_level,
        summary_updates,
        completed_updates,
    }
}

fn update_summary_race_counts(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    let race = sim
        .archaeology
        .races
        .first_mut()
        .expect("test setup must seed one archaeology race");
    race.currency_amount = 18;
    race.project_amount = 35;
}

fn fire_generic_update_probe(env: &WowLuaEnv) -> GenericRefreshProbe {
    let (summary_updates, completed_updates, summary_row_text) = env
        .eval(
            r#"
            ArchaeologyFrame.tab1:Click()

            local summaryUpdates = 0
            local completedUpdates = 0
            local summaryUpdate = ArchaeologyFrame.summaryPage.UpdateFrame
            local completedUpdate = ArchaeologyFrame.completedPage.UpdateFrame

            ArchaeologyFrame.summaryPage.UpdateFrame = function(self)
                summaryUpdates = summaryUpdates + 1
                return summaryUpdate(self)
            end
            ArchaeologyFrame.completedPage.UpdateFrame = function(self)
                completedUpdates = completedUpdates + 1
                return completedUpdate(self)
            end

            FireEvent("CURRENCY_DISPLAY_UPDATE")

            return summaryUpdates,
                   completedUpdates,
                   ArchaeologyFrame.summaryPage.race1.raceName:GetText()
            "#,
        )
        .expect("generic archaeology update event probe must run cleanly");

    GenericRefreshProbe {
        summary_updates,
        completed_updates,
        summary_row_text,
    }
}

fn assert_completion_event_plays_artifact_glow(probe: &CompletionEventProbe) {
    assert!(
        probe.glow_playing,
        "matching RESEARCH_ARTIFACT_COMPLETE must play the artifact-page completion glow"
    );
    assert_eq!(
        probe.glow_frame_level,
        probe.frame_level + 3,
        "completion glow must be raised above ArchaeologyFrame while it plays"
    );
}

fn assert_completion_event_does_not_refresh_pages(probe: &CompletionEventProbe) {
    assert_eq!(
        probe.summary_updates, 0,
        "Blizzard's RESEARCH_ARTIFACT_COMPLETE branch is glow-only and must not refresh summaryPage"
    );
    assert_eq!(
        probe.completed_updates, 0,
        "Blizzard's RESEARCH_ARTIFACT_COMPLETE branch is glow-only and must not refresh completedPage"
    );
}

fn assert_generic_event_refreshes_active_summary_page(probe: &GenericRefreshProbe) {
    assert_eq!(
        probe.summary_updates, 1,
        "non-special archaeology update events must refresh the active summary page"
    );
    assert_eq!(
        probe.completed_updates, 0,
        "summary-page refresh must not also invoke completedPage.UpdateFrame"
    );
    assert_eq!(
        probe.summary_row_text, "Tol'vir|n18/35",
        "active summary-page refresh must pick up post-event archaeology race counts"
    );
}
