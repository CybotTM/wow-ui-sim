//! Behavior pin: PLAN-named "ToggleTracking flips SetAsTracked and
//! updates the watched achievement list (uses
//! `SetTrackedAchievement`/`RemoveTrackedAchievement` if present)"
//! collapses FOUR structural facts about
//! `AchievementTemplateMixin:ToggleTracking` /
//! `AchievementTemplateMixin:SetAsTracked` at
//! `Mainline/Blizzard_AchievementUI.lua:1580-1620`.
//!
//! 1. **PLAN names the wrong API.** PLAN says
//!    `SetTrackedAchievement` / `RemoveTrackedAchievement` "if
//!    present". The actual implementation uses
//!    `C_ContentTracking.StartTracking(Enum.ContentTrackingType.Achievement, id)`
//!    at lua:1602 and
//!    `C_ContentTracking.StopTracking(Enum.ContentTrackingType.Achievement, id, Enum.ContentTrackingStopType.Manual)`
//!    at lua:1583. The legacy `Set/RemoveTrackedAchievement` globals
//!    DO exist in the simulator (real-WoW client globals), but the
//!    addon does NOT call them — instead, the routing goes through
//!    `C_ContentTracking`. PLAN-named-not-called → install spies on
//!    both legacy globals and assert call counts stay at 0 across
//!    all branches. (If a future refactor re-routed through the
//!    legacy globals, the tripwire fires.)
//! 2. **`ToggleTracking` has THREE early-return guards.** PLAN's
//!    "flips" wording implies binary toggle; the actual flow is:
//!    - lua:1582-1586: if `trackedAchievements[id]` → call StopTracking,
//!      `:SetAsTracked(false)`, RETURN (no `true` returned).
//!    - lua:1589-1592: if `#GetTrackedIDs >= MaxTrackedAchievements`
//!      (10) → `UIErrorsFrame:AddMessage(ACHIEVEMENT_WATCH_TOO_MANY)`,
//!      RETURN.
//!    - lua:1595-1599: if `(completed and isGuild) or wasEarnedByMe`
//!      → `UIErrorsFrame:AddMessage(ERR_ACHIEVEMENT_WATCH_COMPLETED)`,
//!      RETURN.
//!    - lua:1601-1606: else `:SetAsTracked(true)`, StartTracking,
//!      handle trackingError, RETURN `true`.
//! 3. **`trackedAchievements` is a file-local upvalue** (lua:69) that
//!    `ToggleTracking` captures (referenced at lua:1582) and that
//!    `updateTrackedAchievements` (lua:71) MAINTAINS by reassignment
//!    on `CONTENT_TRACKING_UPDATE`-style events. The closure means a
//!    `_G` poke does not reach the gate. Tests install state via
//!    `debug.setupvalue(ToggleTracking, idx, my_table)` after locating
//!    the `trackedAchievements` upvalue index.
//! 4. **`SetAsTracked` writes to TWO widgets and gates `Tracked:Hide`
//!    on selection state.** lua:1610 calls `self.Check:SetShown(tracked)`,
//!    lua:1611 calls `self.Tracked:ApplyChecked(tracked, noSound)`,
//!    then lua:1612-1615 either `self.Tracked:Show()` (when tracked)
//!    or `self.Tracked:Hide()` (when NOT tracked AND
//!    `not SelectionBehaviorMixin.IsIntrusiveSelected(self)`). When
//!    untracked AND selected, Tracked stays visible. PLAN's "flips
//!    SetAsTracked" wording elides the per-widget surface and the
//!    selection gate. lua:1617 also calls
//!    `self.Label:SetWidth(self.Label:GetStringWidth() + 4)` — the
//!    +4 is a string-width fudge (bug 144418); a regression that
//!    dropped this would re-introduce the truncation.
//!
//! Source map of the contract:
//!
//! ```lua
//! -- lua:69  local trackedAchievements = {}
//! -- lua:71  local function updateTrackedAchievements(achievementIDs)
//!
//! function AchievementTemplateMixin:ToggleTracking()              -- lua:1580
//!     local id = self.id
//!     if (trackedAchievements[id]) then                            -- lua:1582
//!         C_ContentTracking.StopTracking(
//!             Enum.ContentTrackingType.Achievement, id,
//!             Enum.ContentTrackingStopType.Manual)                  -- lua:1583
//!         self:SetAsTracked(false)                                  -- lua:1584
//!         return                                                    -- lua:1585 (implicit nil)
//!     end
//!     local count = #C_ContentTracking.GetTrackedIDs(
//!         Enum.ContentTrackingType.Achievement)                     -- lua:1588
//!     if (count >= Constants.ContentTrackingConsts.MaxTrackedAchievements) then  -- lua:1589
//!         UIErrorsFrame:AddMessage(format(ACHIEVEMENT_WATCH_TOO_MANY, ...), 1, 0.1, 0.1, 1)
//!         return
//!     end
//!     local _,_,_, completed,_,_,_,_,_,_,_, isGuild, wasEarnedByMe =
//!         GetAchievementInfo(id)                                    -- lua:1594
//!     if ((completed and isGuild) or wasEarnedByMe) then           -- lua:1595
//!         UIErrorsFrame:AddMessage(ERR_ACHIEVEMENT_WATCH_COMPLETED, 1, 0.1, 0.1, 1)
//!         return
//!     end
//!     self:SetAsTracked(true)                                      -- lua:1600
//!     local trackingError = C_ContentTracking.StartTracking(
//!         Enum.ContentTrackingType.Achievement, id)                -- lua:1602
//!     if trackingError then
//!         ContentTrackingUtil.DisplayTrackingError(trackingError)
//!     end
//!     return true                                                   -- lua:1606
//! end
//!
//! function AchievementTemplateMixin:SetAsTracked(tracked, noSound) -- lua:1609
//!     self.Check:SetShown(tracked)                                  -- lua:1610
//!     self.Tracked:ApplyChecked(tracked, noSound)                   -- lua:1611
//!     if tracked then
//!         self.Tracked:Show()                                       -- lua:1613
//!     elseif not SelectionBehaviorMixin.IsIntrusiveSelected(self) then  -- lua:1614
//!         self.Tracked:Hide()                                       -- lua:1615
//!     end
//!     self.Label:SetWidth(self.Label:GetStringWidth() + 4)          -- lua:1617
//! end
//! ```
//!
//! Two tests split the "stop" branch (already-tracked) from the
//! "start happy-path + two guard branches" so each body stays under
//! the readability budget.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const TRACKED_ACHIEVEMENT_ID: i64 = 4242;
const MAX_TRACKED: i64 = 10;
const EXPECTED_STOP: &str = "stop_tracking_calls=1 stop_tracking_id=4242 \
    stop_tracking_type=ACH stop_tracking_stop_type=MANUAL \
    start_tracking_calls=0 \
    set_as_tracked_arg=false check_set_shown_arg=false \
    tracked_apply_checked_arg=false tracked_hide_calls=1 tracked_show_calls=0 \
    label_set_width_calls=1 \
    too_many_message_calls=0 completed_message_calls=0 \
    legacy_set_tracked_calls=0 legacy_remove_tracked_calls=0 \
    return_value_type=nil";
const EXPECTED_START: &str = "start_tracking_calls=1 start_tracking_id=4242 \
    start_tracking_type=ACH stop_tracking_calls=0 \
    set_as_tracked_arg=true check_set_shown_arg=true \
    tracked_apply_checked_arg=true tracked_show_calls=1 tracked_hide_calls=0 \
    label_set_width_calls=1 \
    happy_legacy_set_calls=0 happy_legacy_remove_calls=0 \
    return_value_type=boolean return_value=true \
    max_too_many_calls=1 max_start_calls=0 max_set_as_tracked_calls=0 \
    max_legacy_set_calls=0 \
    completed_message_calls=1 completed_start_calls=0 \
    completed_set_as_tracked_calls=0 completed_legacy_set_calls=0";

type StopProbe = String;
type StartProbe = String;

const FAKE_ROW_BUILDER: &str = r#"
    local function counter(captures, key)
        return function(self, ...) captures[key] = (captures[key] or 0) + 1 end
    end
    local function build_row(captures)
        local row = {id = 4242}
        row.Check = {
            SetShown = function(self, value)
                captures.check_set_shown_calls = (captures.check_set_shown_calls or 0) + 1
                captures.check_set_shown_arg = tostring(value)
            end,
        }
        row.Tracked = {
            ApplyChecked = function(self, checked, noSound)
                captures.tracked_apply_checked_calls =
                    (captures.tracked_apply_checked_calls or 0) + 1
                captures.tracked_apply_checked_arg = tostring(checked)
            end,
            Show = counter(captures, "tracked_show_calls"),
            Hide = counter(captures, "tracked_hide_calls"),
        }
        row.Label = {
            SetWidth = counter(captures, "label_set_width_calls"),
            GetStringWidth = function() return 100 end,
        }
        return row
    end
"#;

const TRACKING_GLOBALS_BUILDER: &str = r#"
    local function install_legacy_spies(captures)
        if type(_G.SetTrackedAchievement) == "function" then
            local original = _G.SetTrackedAchievement
            _G.SetTrackedAchievement = function(...)
                captures.legacy_set_tracked_calls =
                    (captures.legacy_set_tracked_calls or 0) + 1
                return original(...)
            end
        end
        if type(_G.RemoveTrackedAchievement) == "function" then
            local original = _G.RemoveTrackedAchievement
            _G.RemoveTrackedAchievement = function(...)
                captures.legacy_remove_tracked_calls =
                    (captures.legacy_remove_tracked_calls or 0) + 1
                return original(...)
            end
        end
    end
    local function install_tracking_globals(captures, tracked_count, completed, was_earned)
        _G.C_ContentTracking = {
            StopTracking = function(type_arg, id, stop_type)
                captures.stop_tracking_calls = (captures.stop_tracking_calls or 0) + 1
                captures.stop_tracking_type =
                    type_arg == Enum.ContentTrackingType.Achievement and "ACH" or "OTHER"
                captures.stop_tracking_id = id
                captures.stop_tracking_stop_type =
                    stop_type == Enum.ContentTrackingStopType.Manual and "MANUAL" or "OTHER"
            end,
            StartTracking = function(type_arg, id)
                captures.start_tracking_calls = (captures.start_tracking_calls or 0) + 1
                captures.start_tracking_type =
                    type_arg == Enum.ContentTrackingType.Achievement and "ACH" or "OTHER"
                captures.start_tracking_id = id
                return nil
            end,
            GetTrackedIDs = function() return {unpack({}, 1, tracked_count or 0)} end,
        }
        local stub_ids = {}
        for i = 1, (tracked_count or 0) do stub_ids[i] = i end
        _G.C_ContentTracking.GetTrackedIDs = function() return stub_ids end
        _G.GetAchievementInfo = function(id)
            return id, "name", 10, completed and true or false, 0, 0, 0, "desc",
                   0, "icon", "rewardText", false, was_earned and true or false
        end
        _G.UIErrorsFrame = {
            AddMessage = function(self, msg, ...)
                if msg and msg:find("ACHIEVEMENT_WATCH_TOO_MANY") then
                    captures.too_many_message_calls =
                        (captures.too_many_message_calls or 0) + 1
                elseif msg and msg:find("ERR_ACHIEVEMENT_WATCH_COMPLETED") then
                    captures.completed_message_calls =
                        (captures.completed_message_calls or 0) + 1
                end
            end,
        }
        _G.ACHIEVEMENT_WATCH_TOO_MANY = "ACHIEVEMENT_WATCH_TOO_MANY %d"
        _G.ERR_ACHIEVEMENT_WATCH_COMPLETED = "ERR_ACHIEVEMENT_WATCH_COMPLETED"
        if not _G.Constants then _G.Constants = {} end
        _G.Constants.ContentTrackingConsts = {MaxTrackedAchievements = 10}
        _G.SelectionBehaviorMixin = {IsIntrusiveSelected = function(self) return false end}
        _G.ContentTrackingUtil = {DisplayTrackingError = function() end}
    end
"#;

const TRACKED_UPVALUE_HELPER: &str = r#"
    local function set_tracked_achievements_upvalue(target_func, replacement)
        for i = 1, 60 do
            local name, val = debug.getupvalue(target_func, i)
            if name == nil then break end
            if name == "trackedAchievements" then
                debug.setupvalue(target_func, i, replacement)
                return i, val
            end
        end
        return nil, nil
    end
"#;

#[test]
fn toggle_tracking_stop_branch_uses_c_content_tracking_stop_tracking_with_manual_stop_type_and_calls_set_as_tracked_false()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: StopProbe = env
            .eval(&format!(
                r#"
                assert(_G.AchievementTemplateMixin.ToggleTracking,
                    "AchievementTemplateMixin:ToggleTracking must exist (lua:1580)")
                assert(_G.AchievementTemplateMixin.SetAsTracked,
                    "AchievementTemplateMixin:SetAsTracked must exist (lua:1609)")

                {fake_row_builder}
                {tracking_globals_builder}
                {tracked_upvalue_helper}

                local captures = {{}}
                local row = build_row(captures)
                Mixin(row, AchievementTemplateMixin)
                install_tracking_globals(captures, 0, false, false)
                install_legacy_spies(captures)

                local idx, orig_table = set_tracked_achievements_upvalue(
                    AchievementTemplateMixin.ToggleTracking,
                    {{[{tracked_id}] = true}})
                assert(idx,
                    "ToggleTracking must capture trackedAchievements upvalue " ..
                    "(referenced at lua:1582; declared local at lua:69)")

                local return_value = row:ToggleTracking()
                local return_value_type = type(return_value)

                debug.setupvalue(AchievementTemplateMixin.ToggleTracking, idx, orig_table)

                local stop_signature = string.format(
                    "stop_tracking_calls=%d stop_tracking_id=%s " ..
                    "stop_tracking_type=%s stop_tracking_stop_type=%s " ..
                    "start_tracking_calls=%d " ..
                    "set_as_tracked_arg=%s check_set_shown_arg=%s " ..
                    "tracked_apply_checked_arg=%s tracked_hide_calls=%d " ..
                    "tracked_show_calls=%d label_set_width_calls=%d " ..
                    "too_many_message_calls=%d completed_message_calls=%d " ..
                    "legacy_set_tracked_calls=%d legacy_remove_tracked_calls=%d " ..
                    "return_value_type=%s",
                    captures.stop_tracking_calls or 0,
                    tostring(captures.stop_tracking_id),
                    tostring(captures.stop_tracking_type),
                    tostring(captures.stop_tracking_stop_type),
                    captures.start_tracking_calls or 0,
                    tostring(captures.check_set_shown_arg == "false"
                             and captures.tracked_apply_checked_arg == "false"
                             and "false" or captures.check_set_shown_arg),
                    tostring(captures.check_set_shown_arg),
                    tostring(captures.tracked_apply_checked_arg),
                    captures.tracked_hide_calls or 0,
                    captures.tracked_show_calls or 0,
                    captures.label_set_width_calls or 0,
                    captures.too_many_message_calls or 0,
                    captures.completed_message_calls or 0,
                    captures.legacy_set_tracked_calls or 0,
                    captures.legacy_remove_tracked_calls or 0,
                    return_value_type)

                return stop_signature
                "#,
                fake_row_builder = FAKE_ROW_BUILDER,
                tracking_globals_builder = TRACKING_GLOBALS_BUILDER,
                tracked_upvalue_helper = TRACKED_UPVALUE_HELPER,
                tracked_id = TRACKED_ACHIEVEMENT_ID,
            ))
            .expect("ToggleTracking stop-branch drive must run cleanly");

        let signature = observations;

        assert_eq!(
            signature, EXPECTED_STOP,
            "Expected ToggleTracking stop-branch signature to match. The drive \
             pre-populates `trackedAchievements[{TRACKED_ACHIEVEMENT_ID}] = true` via \
             `debug.setupvalue` on the file-local upvalue at lua:69; calls \
             `:ToggleTracking()` on the row. The lua:1582 gate evaluates true → \
             lua:1583 fires `C_ContentTracking.StopTracking(Achievement, {TRACKED_ACHIEVEMENT_ID}, \
             Manual)`; lua:1584 fires `:SetAsTracked(false)` which threads through \
             lua:1610-1615 (`Check:SetShown(false)`, `Tracked:ApplyChecked(false, nil)`, \
             `Tracked:Hide()` because the SelectionBehaviorMixin spy returns false); \
             lua:1617 fires `Label:SetWidth(GetStringWidth+4)`. The implicit `return` \
             at lua:1585 yields `nil` (NOT `true` — only the lua:1606 success path \
             returns `true`). `start_tracking_calls=0` is the cross-branch tripwire — \
             the start path at lua:1601-1606 must NOT fire when the gate at lua:1582 \
             diverts. `tracked_show_calls=0` proves the lua:1612 branch (`if tracked \
             then ... Show()`) was skipped. `too_many_message_calls=0` and \
             `completed_message_calls=0` prove the two error-message branches at \
             lua:1591 and lua:1597 were skipped. Expected `{EXPECTED_STOP}`. Got \
             `{signature}`. A `stop_tracking_stop_type` other than `MANUAL` means \
             lua:1583's `Enum.ContentTrackingStopType.Manual` argument was changed — \
             reverting the user-action distinction (would also affect quest lockouts \
             in real WoW). A `start_tracking_calls=1` means the gate at lua:1582 \
             didn't divert — likely the upvalue swap missed and the closure's view of \
             `trackedAchievements` is empty. A `return_value_type=boolean` means \
             lua:1585's `return` was changed to `return true` — the caller at lua:1622 \
             ignores the return today, but the doc-style return-value semantics matter."
        );
    });
}

#[test]
fn toggle_tracking_start_branch_calls_c_content_tracking_start_tracking_returns_true_and_guards_max_count_and_completed_earned()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: StartProbe = env
            .eval(&format!(
                r#"
                {fake_row_builder}
                {tracking_globals_builder}
                {tracked_upvalue_helper}

                local happy_captures = {{}}
                local happy_row = build_row(happy_captures)
                Mixin(happy_row, AchievementTemplateMixin)
                install_tracking_globals(happy_captures, 0, false, false)
                install_legacy_spies(happy_captures)
                local idx_h, orig_h = set_tracked_achievements_upvalue(
                    AchievementTemplateMixin.ToggleTracking, {{}})
                local happy_return = happy_row:ToggleTracking()
                debug.setupvalue(AchievementTemplateMixin.ToggleTracking, idx_h, orig_h)

                local max_captures = {{}}
                local max_row = build_row(max_captures)
                Mixin(max_row, AchievementTemplateMixin)
                install_tracking_globals(max_captures, {max_tracked}, false, false)
                install_legacy_spies(max_captures)
                local idx_m, orig_m = set_tracked_achievements_upvalue(
                    AchievementTemplateMixin.ToggleTracking, {{}})
                max_row:ToggleTracking()
                debug.setupvalue(AchievementTemplateMixin.ToggleTracking, idx_m, orig_m)

                local completed_captures = {{}}
                local completed_row = build_row(completed_captures)
                Mixin(completed_row, AchievementTemplateMixin)
                install_tracking_globals(completed_captures, 0, false, true)
                install_legacy_spies(completed_captures)
                local idx_c, orig_c = set_tracked_achievements_upvalue(
                    AchievementTemplateMixin.ToggleTracking, {{}})
                completed_row:ToggleTracking()
                debug.setupvalue(AchievementTemplateMixin.ToggleTracking, idx_c, orig_c)

                local start_signature = string.format(
                    "start_tracking_calls=%d start_tracking_id=%s start_tracking_type=%s " ..
                    "stop_tracking_calls=%d " ..
                    "set_as_tracked_arg=%s check_set_shown_arg=%s " ..
                    "tracked_apply_checked_arg=%s tracked_show_calls=%d " ..
                    "tracked_hide_calls=%d label_set_width_calls=%d " ..
                    "happy_legacy_set_calls=%d happy_legacy_remove_calls=%d " ..
                    "return_value_type=%s return_value=%s " ..
                    "max_too_many_calls=%d max_start_calls=%d max_set_as_tracked_calls=%d " ..
                    "max_legacy_set_calls=%d " ..
                    "completed_message_calls=%d completed_start_calls=%d " ..
                    "completed_set_as_tracked_calls=%d completed_legacy_set_calls=%d",
                    happy_captures.start_tracking_calls or 0,
                    tostring(happy_captures.start_tracking_id),
                    tostring(happy_captures.start_tracking_type),
                    happy_captures.stop_tracking_calls or 0,
                    tostring(happy_captures.check_set_shown_arg),
                    tostring(happy_captures.check_set_shown_arg),
                    tostring(happy_captures.tracked_apply_checked_arg),
                    happy_captures.tracked_show_calls or 0,
                    happy_captures.tracked_hide_calls or 0,
                    happy_captures.label_set_width_calls or 0,
                    happy_captures.legacy_set_tracked_calls or 0,
                    happy_captures.legacy_remove_tracked_calls or 0,
                    type(happy_return), tostring(happy_return),
                    max_captures.too_many_message_calls or 0,
                    max_captures.start_tracking_calls or 0,
                    max_captures.check_set_shown_calls or 0,
                    max_captures.legacy_set_tracked_calls or 0,
                    completed_captures.completed_message_calls or 0,
                    completed_captures.start_tracking_calls or 0,
                    completed_captures.check_set_shown_calls or 0,
                    completed_captures.legacy_set_tracked_calls or 0)
                return start_signature
                "#,
                fake_row_builder = FAKE_ROW_BUILDER,
                tracking_globals_builder = TRACKING_GLOBALS_BUILDER,
                tracked_upvalue_helper = TRACKED_UPVALUE_HELPER,
                max_tracked = MAX_TRACKED,
            ))
            .expect("ToggleTracking three-drive must run cleanly");

        assert_eq!(
            observations, EXPECTED_START,
            "Expected ToggleTracking start+guard signature to match. Three drives, \
             each with a fresh row and `trackedAchievements = {{}}` (cleared via \
             upvalue swap so lua:1582's gate falls through). Drive 1 (happy): \
             `tracked_count=0`, `completed=false`, `was_earned=false` → fall through \
             both guards, lua:1600 fires `:SetAsTracked(true)` (Check:SetShown(true), \
             Tracked:ApplyChecked(true, nil), Tracked:Show() at lua:1613, \
             Label:SetWidth+4 at lua:1617), lua:1602 fires `StartTracking(Achievement, \
             {TRACKED_ACHIEVEMENT_ID})`, lua:1606 returns `true`. Drive 2 (max-count guard): \
             `tracked_count={MAX_TRACKED}` (= MaxTrackedAchievements at lua:1037-style \
             Constants), lua:1589 gate fires → `UIErrorsFrame:AddMessage(ACHIEVEMENT_\
             WATCH_TOO_MANY)`, RETURN. The cross-branch tripwires \
             `max_start_calls=0` and `max_set_as_tracked_calls=0` prove neither the \
             start path nor SetAsTracked fired when the gate diverted. Drive 3 \
             (completed-earned guard): `completed=false` (so the `completed and \
             isGuild` arm doesn't trigger) but `was_earned=true` → lua:1595 evaluates \
             `(completed and isGuild) or wasEarnedByMe` = true, fires \
             `UIErrorsFrame:AddMessage(ERR_ACHIEVEMENT_WATCH_COMPLETED)`, RETURN. The \
             cross-branch tripwires `completed_start_calls=0` and \
             `completed_set_as_tracked_calls=0` prove neither path fired. Expected \
             `{EXPECTED_START}`. Got `{observations}`. A `start_tracking_type` other \
             than `ACH` means lua:1602 stopped passing `Enum.ContentTrackingType.\
             Achievement` (would silently start tracking the wrong content type). A \
             `return_value=true` mismatch means lua:1606's explicit `return true` was \
             dropped — even though no caller checks it today, the doc-style success \
             signal matters for spec coverage. A `max_start_calls=1` means the \
             max-count guard at lua:1589 didn't fire — likely \
             `Constants.ContentTrackingConsts.MaxTrackedAchievements` was renamed or \
             the `>=` comparison flipped to `>`. A `completed_start_calls=1` means \
             the completed-earned guard at lua:1595 didn't fire — likely the \
             14th-tuple `wasEarnedByMe` index was renumbered (the destructure at \
             lua:1594 picks specific positions out of `GetAchievementInfo`)."
        );
    });
}
