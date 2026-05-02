//! AddonList glue out-of-date dialog behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AddOnList";

#[test]
fn try_show_addon_dialog_shows_out_of_date_dialog_once() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: OutOfDateDialogProbe = env
            .eval(
                r#"
                local originalHasOutOfDate = AddonList_HasOutOfDate
                AddonList_HasOutOfDate = function() return true end

                if not GlueAnnouncementDialog then
                    GlueAnnouncementDialog = CreateFrame("Frame", "GlueAnnouncementDialog", GlueParent)
                end
                GlueAnnouncementDialog:Hide()
                C_AddOns.SetAddonVersionCheck(true)
                HasShownAddonOutOfDateDialog = false
                AddonDialog:Hide()

                local firstResult = TryShowAddonDialog()
                local shownAfterFirst = AddonDialog:IsShown()
                local textAfterFirst = AddonDialogText:GetText()
                local whichAfterFirst = AddonDialog.which
                local flagAfterFirst = HasShownAddonOutOfDateDialog

                local secondResult = TryShowAddonDialog()

                AddonList_HasOutOfDate = originalHasOutOfDate

                return InGlue(),
                       firstResult,
                       secondResult,
                       shownAfterFirst,
                       textAfterFirst,
                       ADDONS_OUT_OF_DATE,
                       whichAfterFirst,
                       flagAfterFirst
                "#,
            )
            .expect("AddOnList glue out-of-date dialog probe must run cleanly");

        assert_out_of_date_dialog_probe(probe);
    });
}

type OutOfDateDialogProbe = (bool, bool, bool, bool, String, String, String, bool);

fn assert_out_of_date_dialog_probe(probe: OutOfDateDialogProbe) {
    let (
        in_glue,
        first_result,
        second_result,
        shown_after_first,
        text_after_first,
        expected_text,
        which_after_first,
        flag_after_first,
    ) = probe;

    assert!(
        in_glue,
        "`{ROOT}` glue harness must exercise the glue branch"
    );
    assert!(
        first_result,
        "`TryShowAddonDialog` must return true when an out-of-date addon is available"
    );
    assert_shown_out_of_date_dialog(
        shown_after_first,
        text_after_first,
        expected_text,
        which_after_first,
    );
    assert_dialog_is_one_shot(flag_after_first, second_result);
}

fn assert_shown_out_of_date_dialog(
    shown_after_first: bool,
    text_after_first: String,
    expected_text: String,
    which_after_first: String,
) {
    assert!(
        shown_after_first,
        "`TryShowAddonDialog` must show `AddonDialog` on the first match"
    );
    assert_eq!(
        text_after_first, expected_text,
        "`AddonDialog` must show the `ADDONS_OUT_OF_DATE` message"
    );
    assert_eq!(
        which_after_first, "ADDONS_OUT_OF_DATE",
        "`AddonDialog_Show` must mark the active dialog type"
    );
}

fn assert_dialog_is_one_shot(flag_after_first: bool, second_result: bool) {
    assert!(
        flag_after_first,
        "`TryShowAddonDialog` must set `HasShownAddonOutOfDateDialog`"
    );
    assert!(
        !second_result,
        "`TryShowAddonDialog` must be a one-shot while the out-of-date flag is set"
    );
}
