//! Quest dialog details choose height/background and portrait branch.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::{AdventureMapQuestInfo, AdventureMapQuestPortrait};

const ROOT: &str = "Blizzard_AdventureMap";
const PORTRAIT_QUEST_ID: i64 = 40519;
const NO_PORTRAIT_QUEST_ID: i64 = 40520;
const HEIGHT_LONG: i64 = 456;
const HEIGHT_SHORT: i64 = 380;
const LONG_ATLAS: &str = "AdventureMapQuest-QuestPane-9sliced";
const SHORT_ATLAS: &str = "AdventureMapQuest-QuestPane";
const REFRESH_DETAILS_PROBE: &str = r#"
local originalGetWidgetSet = C_TaskQuest.GetQuestUIWidgetSetByType
local originalShowPortrait = QuestFrame_ShowQuestPortrait
local originalHidePortrait = QuestFrame_HideQuestPortrait
local showPortraitCount = 0
local hidePortraitCount = 0
local shownPortraitID = nil
local shownPortraitName = nil

C_TaskQuest.GetQuestUIWidgetSetByType = function()
    return __refreshDetailsHasWidgets and 777 or nil
end
QuestFrame_ShowQuestPortrait = function(parent, portraitDisplayID, mountDisplayID, modelSceneID, text, name)
    showPortraitCount = showPortraitCount + 1
    shownPortraitID = portraitDisplayID
    shownPortraitName = name
end
QuestFrame_HideQuestPortrait = function()
    hidePortraitCount = hidePortraitCount + 1
end

local dialog = {
    questID = __refreshDetailsQuestID,
    anchorRegion = {},
    rewardsHeight = 50,
    SetHeight = function(self, height) self.height = height end,
    SetPoint = function(self, point, relativeTo, x, y)
        self.point = point
        self.pointX = x
        self.pointY = y
    end,
    Background = {
        SetAtlas = function(self, atlas)
            self.atlas = atlas
        end,
    },
    Rewards = {
        IsShown = function()
            return __refreshDetailsHasRewards
        end,
    },
    RewardsHeader = {},
    Details = {
        SetHeight = function(self, height) self.height = height end,
        Show = function(self) self.shown = true end,
        Hide = function(self) self.hidden = true end,
        Child = {
            TitleHeader = { SetText = function(self, text) self.text = text end },
            DescriptionText = { SetText = function(self, text) self.text = text end },
            ObjectivesText = { SetText = function(self, text) self.text = text end },
            Elements = { { GetHeight = function() return 11 end } },
            SetHeight = function(self, height) self.height = height end,
        },
    },
}

if __refreshDetailsHasWidgets then
    dialog.widgetContainer = {
        numWidgetsShowing = 1,
        RegisterForWidgetSet = function(self, widgetSetID) self.widgetSetID = widgetSetID end,
        SetPoint = function(self, point, relativeTo, relativePoint, x, y)
            self.point = point
            self.relativePoint = relativePoint
            self.x = x
            self.y = y
        end,
        GetHeight = function() return 40 end,
    }
end

AdventureMapQuestChoiceDialogMixin.RefreshDetails(dialog)

C_TaskQuest.GetQuestUIWidgetSetByType = originalGetWidgetSet
QuestFrame_ShowQuestPortrait = originalShowPortrait
QuestFrame_HideQuestPortrait = originalHidePortrait

return dialog.height,
       dialog.Background.atlas,
       dialog.Details.height,
       dialog.Details.shown == true,
       dialog.Details.Child.TitleHeader.text,
       dialog.Details.Child.DescriptionText.text,
       dialog.Details.Child.ObjectivesText.text,
       showPortraitCount,
       hidePortraitCount,
       shownPortraitID,
       shownPortraitName,
       dialog.pointX or 0,
       dialog.pointY or 0
"#;

#[test]
fn quest_dialog_refresh_details_branches_on_widgets_rewards_and_portrait() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_quest_dialog_details(env);

        let long_surface = refresh_details_surface(env, PORTRAIT_QUEST_ID, true, true);
        assert_refresh_details_surface(
            long_surface,
            ExpectedDetails {
                height: HEIGHT_LONG,
                atlas: LONG_ATLAS,
                show_portrait_count: 1,
                hide_portrait_count: 0,
                portrait_id: Some(50_523),
                portrait_name: Some("Lady Hyrja"),
            },
        );

        let short_surface = refresh_details_surface(env, NO_PORTRAIT_QUEST_ID, false, true);
        assert_refresh_details_surface(
            short_surface,
            ExpectedDetails {
                height: HEIGHT_SHORT,
                atlas: SHORT_ATLAS,
                show_portrait_count: 0,
                hide_portrait_count: 1,
                portrait_id: None,
                portrait_name: None,
            },
        );
    });
}

#[derive(Clone, Copy)]
struct ExpectedDetails {
    height: i64,
    atlas: &'static str,
    show_portrait_count: i64,
    hide_portrait_count: i64,
    portrait_id: Option<i64>,
    portrait_name: Option<&'static str>,
}

type RefreshDetailsSurface = (
    i64,
    String,
    i64,
    bool,
    String,
    String,
    String,
    i64,
    i64,
    Option<i64>,
    Option<String>,
    f64,
    f64,
);

fn seed_quest_dialog_details(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state
        .adventure_map
        .quest_info
        .insert(PORTRAIT_QUEST_ID, quest_info());
    state
        .adventure_map
        .quest_info
        .insert(NO_PORTRAIT_QUEST_ID, quest_info());
    state
        .adventure_map
        .quest_portraits
        .insert(PORTRAIT_QUEST_ID, portrait_info(50_523));
    state
        .adventure_map
        .quest_portraits
        .insert(NO_PORTRAIT_QUEST_ID, portrait_info(0));
}

fn quest_info() -> AdventureMapQuestInfo {
    AdventureMapQuestInfo {
        title: "Curse of the Drowned".to_string(),
        description: "Investigate the source of the curse.".to_string(),
        objective_text: "Cleanse 5 Drowned Souls.".to_string(),
    }
}

fn portrait_info(portrait_display_id: i64) -> AdventureMapQuestPortrait {
    AdventureMapQuestPortrait {
        portrait_display_id,
        mount_portrait_display_id: 0,
        model_scene_id: Some(33),
        text: "The tides themselves cry out for justice.".to_string(),
        name: "Lady Hyrja".to_string(),
    }
}

fn refresh_details_surface(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    quest_id: i64,
    has_widgets: bool,
    has_rewards: bool,
) -> RefreshDetailsSurface {
    seed_refresh_details_probe(env, quest_id, has_widgets, has_rewards);
    env.eval(REFRESH_DETAILS_PROBE)
        .expect("AdventureMap quest dialog RefreshDetails probe must run cleanly")
}

fn seed_refresh_details_probe(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    quest_id: i64,
    has_widgets: bool,
    has_rewards: bool,
) {
    env.exec(&format!(
        r#"
        __refreshDetailsQuestID = {quest_id}
        __refreshDetailsHasWidgets = {has_widgets}
        __refreshDetailsHasRewards = {has_rewards}
        "#
    ))
    .expect("AdventureMap quest dialog RefreshDetails setup must run cleanly");
}

fn assert_refresh_details_surface(surface: RefreshDetailsSurface, expected: ExpectedDetails) {
    let (
        frame_height,
        background_atlas,
        details_height,
        details_shown,
        title,
        description,
        objective,
        show_portrait_count,
        hide_portrait_count,
        portrait_id,
        portrait_name,
        anchor_x,
        anchor_y,
    ) = surface;

    assert_details_frame(
        frame_height,
        background_atlas,
        details_height,
        details_shown,
        &expected,
    );
    assert_details_text(title, description, objective);
    assert_portrait_branch(
        show_portrait_count,
        hide_portrait_count,
        portrait_id,
        portrait_name,
        &expected,
    );
    assert_dialog_anchor(anchor_x, anchor_y);
}

fn assert_details_frame(
    frame_height: i64,
    background_atlas: String,
    details_height: i64,
    details_shown: bool,
    expected: &ExpectedDetails,
) {
    assert_eq!(
        frame_height, expected.height,
        "`RefreshDetails` frame height"
    );
    assert_eq!(
        background_atlas, expected.atlas,
        "`RefreshDetails` background atlas"
    );
    assert!(
        details_height > 0,
        "`RefreshDetails` must size the details scroll"
    );
    assert!(
        details_shown,
        "`RefreshDetails` must show the details panel"
    );
}

fn assert_details_text(title: String, description: String, objective: String) {
    assert_eq!(title, "Curse of the Drowned");
    assert_eq!(description, "Investigate the source of the curse.");
    assert_eq!(objective, "Cleanse 5 Drowned Souls.");
}

fn assert_portrait_branch(
    show_portrait_count: i64,
    hide_portrait_count: i64,
    portrait_id: Option<i64>,
    portrait_name: Option<String>,
    expected: &ExpectedDetails,
) {
    assert_eq!(
        show_portrait_count, expected.show_portrait_count,
        "`RefreshDetails` show-portrait call count"
    );
    assert_eq!(
        hide_portrait_count, expected.hide_portrait_count,
        "`RefreshDetails` hide-portrait call count"
    );
    if show_portrait_count == 1 {
        assert_eq!(portrait_id, expected.portrait_id);
        assert_eq!(portrait_name.as_deref(), expected.portrait_name);
    } else {
        assert_eq!(portrait_id, expected.portrait_id);
        assert_eq!(portrait_name.as_deref(), expected.portrait_name);
    }
}

fn assert_dialog_anchor(anchor_x: f64, anchor_y: f64) {
    assert!(
        anchor_x <= 0.0,
        "portrait/no-portrait branches must leave the dialog centered or shifted left"
    );
    assert_approx_eq(anchor_y, 0.0, "dialog anchor Y");
}

fn assert_approx_eq(actual: f64, expected: f64, context: &str) {
    let delta = (actual - expected).abs();
    assert!(
        delta < 0.001,
        "{context}: expected {expected}, got {actual}"
    );
}
