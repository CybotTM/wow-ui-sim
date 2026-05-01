//! Behavior pin: `AchievementObjectives_DisplayCriteria` (lua:1861) DOES
//! branch on `GetAchievementGuildRep(id)`'s `requiresRep` to render a
//! rep-requirement strip onto `objectivesFrame.RepCriteria`. PLAN's claim
//! is correct; the depends-on tag `GetAchievementGuildRep gap` is the
//! stale half — the C API has been implemented.
//!
//! Source map (the actual Mainline contract at
//! `Mainline/Blizzard_AchievementUI.lua:1861-1899`):
//!
//! ```lua
//! function AchievementObjectives_DisplayCriteria (objectivesFrame, id)
//!     if ( not id ) then
//!         return;
//!     end
//!     ...
//!     local requiresRep, hasRep, repLevel;
//!     if ( not objectivesFrame.completed ) then
//!         requiresRep, hasRep, repLevel = GetAchievementGuildRep(id);
//!         if ( requiresRep ) then
//!             local gender = UnitSex("player");
//!             local factionStandingtext = GetText("FACTION_STANDING_LABEL"..repLevel, gender);
//!             objectivesFrame.RepCriteria:SetFormattedText(ACHIEVEMENT_REQUIRES_GUILD_REPUTATION, factionStandingtext);
//!             if ( hasRep ) then
//!                 objectivesFrame.RepCriteria:SetTextColor(0, 1, 0);
//!             else
//!                 objectivesFrame.RepCriteria:SetTextColor(1, 0, 0);
//!             end
//!             objectivesFrame.RepCriteria:Show();
//!             AddExtraCriteriaRow();
//!         end
//!     end
//!
//!     local numCriteria = GetAchievementNumCriteria(id);
//!     if ( numCriteria == 0 and not requiresRep ) then
//!         objectivesFrame.mode = ACHIEVEMENTMODE_CRITERIA;
//!         objectivesFrame:SetHeight(0);
//!         return;
//!     end
//!     ...
//! end
//! ```
//!
//! XML widget chain (the rep-strip target):
//!
//! - `AchievementFrameAchievementsObjectivesTemplate` at xml:340 is a
//!   virtual frame with `mixin="AchievementsObjectivesMixin"` that holds
//!   the `<FontString parentKey="RepCriteria">` declaration at xml:343
//!   (anchored TOP +8/-2, white text). The template instance
//!   `AchievementFrameAchievementsObjectives` at xml:356 is the live
//!   frame the criteria-list code reaches via `objectivesFrame.RepCriteria`.
//! - `AchievementsObjectivesMixin:Clear()` at lua:1689 is the only place
//!   `RepCriteria:Hide()` is called (`self.RepCriteria:Hide()` at
//!   lua:1696). This is the symmetric counterpart to the `:Show()` call
//!   at lua:1889 — `Clear` is invoked from `OnHide` (lua:1685) and
//!   `OnLoad` (lua:1682), so the strip starts hidden and only shows when
//!   `requiresRep` is true.
//!
//! Cata mirrors the contract at
//! `Cata/Blizzard_AchievementUI.lua:1312-1342`: the same
//! `requiresRep, hasRep, repLevel = GetAchievementGuildRep(id)` call,
//! the same color writes (green = hasRep, red = !hasRep), the same
//! `RepCriteria:Show()`. The two flavors agree on the rep-strip
//! contract.
//!
//! **Spec/source agreement on the behavior axis; depends-on tag is the
//! stale half:**
//!
//! 1. `AchievementObjectives_DisplayCriteria` is a real global function
//!    declared at lua:1861 (NOT a mixin method — the only "global plus
//!    `_`" handler that survived the mixin refactor).
//! 2. The rep-strip branch is real and matches PLAN's claim. The text
//!    payload is `ACHIEVEMENT_REQUIRES_GUILD_REPUTATION` formatted with
//!    `FACTION_STANDING_LABEL{repLevel}` localized via `UnitSex("player")`.
//! 3. The depends-on tag `GetAchievementGuildRep gap` is stale — the C
//!    API is implemented at
//!    `src/lua_api/globals/missing_surface/achievement_info.rs:371`
//!    (registration) and `:608` (impl). It returns three values
//!    (`requiresRep: bool`, `hasRep: bool`, `repLevel: number?`) sourced
//!    from `state.achievement_guild_rep: HashMap<i32, AchievementGuildRep>`.
//! 4. Default `SimState.achievement_guild_rep` is empty
//!    (`state_types/collections.rs:316` documents "ungated achievements
//!    simply produce `(false, false, nil)`"). An unseeded id therefore
//!    drives the rep branch *off*, exactly as expected for a
//!    non-rep-gated achievement (e.g. seeded `Level 10` at id 6 is not
//!    rep-gated).
//!
//! Seven assertions split presence/absence:
//!
//! - **Presence half** (5): `_G.AchievementObjectives_DisplayCriteria` is
//!   a function (lua:1861); `_G.GetAchievementGuildRep` is a function
//!   (depends-on stale); `_G.AchievementsObjectivesMixin` is a table;
//!   `AchievementsObjectivesMixin.Clear` is a function (lua:1689 — the
//!   only `:Hide()` call site for `RepCriteria`); the live frame
//!   `AchievementFrameAchievementsObjectives.RepCriteria` is a
//!   FontString.
//! - **Default-state half** (2): `GetAchievementGuildRep(<seeded id>)`
//!   for an unseeded-rep id returns `requiresRep == false` and
//!   `repLevel == nil` — both required to put the rep branch into the
//!   off path so unseeded achievements don't accidentally render the
//!   strip.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_REFERENCED_DISPLAY_CRITERIA: &str = "AchievementObjectives_DisplayCriteria";
const PLAN_REFERENCED_GUILD_REP_API: &str = "GetAchievementGuildRep";
const SEEDED_NON_REP_ACHIEVEMENT_ID: i64 = 6;

type ObjectiveRepProbe = (String, String, String, String, String, bool, bool);

#[test]
fn display_criteria_branches_on_get_achievement_guild_rep_and_targets_rep_criteria_fontstring() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: ObjectiveRepProbe = env
            .eval(
                r#"
                local display_criteria_type = type(_G.AchievementObjectives_DisplayCriteria)
                local guild_rep_api_type = type(_G.GetAchievementGuildRep)
                local mixin_type = type(_G.AchievementsObjectivesMixin)
                local mixin_clear_type =
                    (type(_G.AchievementsObjectivesMixin) == "table"
                        and type(_G.AchievementsObjectivesMixin.Clear))
                    or "no-mixin-table"

                local rep_criteria_object_type = "no-objectives-frame"
                if type(_G.AchievementFrameAchievementsObjectives) == "table"
                    and type(_G.AchievementFrameAchievementsObjectives.RepCriteria) == "table"
                then
                    rep_criteria_object_type = AchievementFrameAchievementsObjectives.RepCriteria:GetObjectType()
                end

                local unseeded_requires_rep = true
                local unseeded_rep_level_is_nil = false
                if type(_G.GetAchievementGuildRep) == "function" then
                    local requires_rep, _has_rep, rep_level = GetAchievementGuildRep(6)
                    unseeded_requires_rep = requires_rep and true or false
                    unseeded_rep_level_is_nil = (rep_level == nil)
                end

                return display_criteria_type,
                       guild_rep_api_type,
                       mixin_type,
                       mixin_clear_type,
                       rep_criteria_object_type,
                       unseeded_requires_rep,
                       unseeded_rep_level_is_nil
                "#,
            )
            .expect("AchievementObjectives_DisplayCriteria rep-branch probe must run cleanly");

        let (
            display_criteria_type,
            guild_rep_api_type,
            mixin_type,
            mixin_clear_type,
            rep_criteria_object_type,
            unseeded_requires_rep,
            unseeded_rep_level_is_nil,
        ) = observations;

        assert_eq!(
            display_criteria_type, "function",
            "Expected `_G.{PLAN_REFERENCED_DISPLAY_CRITERIA}` to be a function — declared at \
             `Mainline/Blizzard_AchievementUI.lua:1861` and `Cata/Blizzard_AchievementUI.lua:1312`. \
             This is the only `AchievementObjectives_*` global that survived the mixin \
             refactor; the surrounding row-level handlers (`AchievementButton_OnClick` etc.) \
             moved onto `AchievementTemplateMixin`. Got `{display_criteria_type}`. A `nil` \
             reading would mean the addon's chunk failed to register the global; the \
             `AchievementButton_DisplayObjectives` call site at lua:1556 would then crash."
        );

        assert_eq!(
            guild_rep_api_type, "function",
            "Expected `_G.{PLAN_REFERENCED_GUILD_REP_API}` to be a function (PLAN tags this as \
             a gap, but it's implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:371` (registration) and \
             `:608` (impl), backed by `state.achievement_guild_rep: HashMap<i32, AchievementGuildRep>`). \
             Got `{guild_rep_api_type}`. The depends-on tag is stale; if this assertion fails \
             the rep-branch tuple-unpack at lua:1879 would crash with `attempt to call a nil \
             value` and `AchievementButton_DisplayObjectives` (lua:1121, 1455, 1879) would \
             cascade-fail."
        );

        assert_eq!(
            mixin_type, "table",
            "Expected `_G.AchievementsObjectivesMixin` to be a table — declared at \
             `Mainline/Blizzard_AchievementUI.lua:1674` and bound to the virtual frame \
             `AchievementFrameAchievementsObjectivesTemplate` via `mixin=\"AchievementsObjectivesMixin\"` \
             at xml:340. Got `{mixin_type}`. A non-table reading means the addon's chunk \
             failed to declare the mixin; the template's mixin attribute would then have \
             nothing to splat onto the frame."
        );

        assert_eq!(
            mixin_clear_type, "function",
            "Expected `AchievementsObjectivesMixin.Clear` to be a function — declared at \
             `Mainline/Blizzard_AchievementUI.lua:1689`, this is the ONLY call site for \
             `self.RepCriteria:Hide()` (lua:1696). It runs from both `OnLoad` (lua:1682) and \
             `OnHide` (lua:1685), so the rep strip starts hidden and is only re-shown by \
             the `:Show()` call at lua:1889 inside the `requiresRep` branch. Got \
             `{mixin_clear_type}`. A `nil` reading means the rep strip would never reset \
             between achievements — once shown for a rep-gated achievement, it would stay \
             visible on subsequent unseeded ones."
        );

        assert_eq!(
            rep_criteria_object_type, "FontString",
            "Expected `AchievementFrameAchievementsObjectives.RepCriteria` to report \
             `GetObjectType() == \"FontString\"` — declared at \
             `Mainline/Blizzard_AchievementUI.xml:343` as \
             `<FontString inherits=\"GameFontNormalSmall\" parentKey=\"RepCriteria\">` inside \
             the virtual `AchievementFrameAchievementsObjectivesTemplate` at xml:340 \
             (`mixin=\"AchievementsObjectivesMixin\"`). The instance frame at xml:356 \
             (`AchievementFrameAchievementsObjectives` inherits the template) is what \
             `AchievementButton_DisplayObjectives` (lua:1556) passes as the first argument \
             to `AchievementObjectives_DisplayCriteria(objectivesFrame, id)`. Got \
             `{rep_criteria_object_type}`. A `no-objectives-frame` reading means either the \
             template instance failed to instantiate or the parentKey routing dropped the \
             FontString."
        );

        assert!(
            !unseeded_requires_rep,
            "Expected `GetAchievementGuildRep({SEEDED_NON_REP_ACHIEVEMENT_ID})` to return \
             `requiresRep == false` for the seeded `Level 10` achievement (id 6) — that id \
             is in `state.achievements` (`src/lua_api/state.rs:2178-2191`) but NOT in \
             `state.achievement_guild_rep` (default empty at \
             `src/lua_api/state_types/collections.rs:316-325`). The impl at \
             `achievement_info.rs:608` resolves unseeded ids via \
             `.get(&id).cloned().unwrap_or_default()`, producing \
             `AchievementGuildRep {{ requires_rep: false, has_rep: false, rep_level: None }}`. \
             Got `requires_rep == true`. A true reading means either the id-6 entry was \
             accidentally seeded into `achievement_guild_rep` (and the rep branch would now \
             fire on a non-rep achievement) or the impl's default lookup changed."
        );

        assert!(
            unseeded_rep_level_is_nil,
            "Expected `GetAchievementGuildRep({SEEDED_NON_REP_ACHIEVEMENT_ID})`'s third return \
             value (`repLevel`) to be nil for the unseeded id — the impl at \
             `achievement_info.rs:617-620` pushes `Val::Nil` when `rep_level` is `None`, and \
             the default `AchievementGuildRep` has `rep_level: None`. Got nil-check failed. \
             A non-nil reading means the impl regressed to pushing a number for the \
             default-constructed entry; the rep branch at lua:1882 \
             (`GetText(\"FACTION_STANDING_LABEL\"..repLevel, gender)`) would then look up a \
             garbage standing label even when `requiresRep == false`."
        );
    });
}
