//! OnLoad state seeding for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ArchaeologyUI";

#[test]
fn archaeology_frame_onload_seeds_title_and_page_state() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let state: OnLoadPageState = env
            .eval(
                r#"
                local professionName = GetArchaeologyInfo()
                return professionName,
                       ArchaeologyFrame:GetTitleText():GetText(),
                       ArchaeologyFrame.helpPage.titleText:GetText(),
                       ArchaeologyFrame.summaryPage.UpdateFrame == ArchaeologyFrame_UpdateSummary,
                       ArchaeologyFrame.completedPage.UpdateFrame == ArchaeologyFrame_UpdateComplete,
                       ArchaeologyFrame.artifactPage.UpdateFrame == ArchaeologyFrame_CurrentArtifactUpdate,
                       ArchaeologyFrame.currentFrame == ArchaeologyFrame.summaryPage,
                       ArchaeologyFrame.currentFrame.currentPage
                "#,
            )
            .expect("ArchaeologyFrame OnLoad page-state probe must run cleanly");

        assert_onload_page_state(state);
    });
}

type OnLoadPageState = (String, String, String, bool, bool, bool, bool, i32);

fn assert_onload_page_state(state: OnLoadPageState) {
    let (
        profession_name,
        frame_title,
        help_title,
        summary_update_matches,
        completed_update_matches,
        artifact_update_matches,
        current_frame_is_summary,
        current_page,
    ) = state;

    assert_eq!(
        frame_title, profession_name,
        "`ArchaeologyFrame_OnLoad` must seed the frame title from `GetArchaeologyInfo()`"
    );
    assert_eq!(
        help_title, profession_name,
        "`ArchaeologyFrame_OnLoad` must seed helpPage.titleText from `GetArchaeologyInfo()`"
    );
    assert!(
        summary_update_matches,
        "`summaryPage.UpdateFrame` must reference `ArchaeologyFrame_UpdateSummary`"
    );
    assert!(
        completed_update_matches,
        "`completedPage.UpdateFrame` must reference `ArchaeologyFrame_UpdateComplete`"
    );
    assert!(
        artifact_update_matches,
        "`artifactPage.UpdateFrame` must reference `ArchaeologyFrame_CurrentArtifactUpdate`"
    );
    assert!(
        current_frame_is_summary,
        "`ArchaeologyFrame.currentFrame` must start on `summaryPage`"
    );
    assert_eq!(current_page, 1, "`summaryPage.currentPage` must start at 1");
}
