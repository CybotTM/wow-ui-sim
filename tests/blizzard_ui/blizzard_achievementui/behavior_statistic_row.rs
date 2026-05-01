//! Behavior pin: PLAN-named "quantity column" is misleading —
//! `AchievementStatTemplateMixin:Init(elementData)` at
//! `Mainline/Blizzard_AchievementUI.lua:2152-2214` writes the
//! statistic value into a single `FontString` (`self.Value`), not a
//! column widget. The dispatch is split across TWO branches: the
//! header branch (lua:2155-2173) blanks `Value:SetText("")` and shows
//! the `Title` text; the non-header branch (lua:2175-2213) is the
//! ONLY branch that calls `GetStatistic(id)` (lua:2198). Even within
//! the non-header branch, the call is gated by `if (not isSummary)`
//! at lua:2197 — but `isSummary` is a FREE GLOBAL (never declared
//! local in this function or its enclosing scope; only `button.isSummary`
//! is ever written, at lua:2369 — a frame property, NOT an upvalue).
//! `_G.isSummary` is nil at runtime, so `not nil == true` and the
//! `GetStatistic(id)` branch ALWAYS wins for non-header rows. The
//! `GetAchievementCriteriaInfo` branch at lua:2200 is dead code.
//!
//! Source map of the contract:
//!
//! ```lua
//! -- lua:2125 AchievementStatTemplateMixin = {};
//! -- lua:2152-2214
//! function AchievementStatTemplateMixin:Init(elementData)
//!     local category = elementData.id                 -- lua:2153
//!     local colorIndex = elementData.colorIndex       -- lua:2154
//!     if elementData.header then                      -- lua:2155
//!         self.Left:Show(); self.Middle:Show(); self.Right:Show()
//!         local text
//!         if (category == ACHIEVEMENT_COMPARISON_STATS_SUMMARY_ID) then
//!             text = ACHIEVEMENT_SUMMARY_CATEGORY
//!         else
//!             text = GetCategoryInfo(category)
//!         end
//!         self.Title:SetText(text)                    -- lua:2166
//!         self.Title:Show()
//!         self.Value:SetText("")                      -- lua:2168 (blank!)
//!         self:SetText("")
//!         self:SetHeight(24)
//!         self.Background:Hide()
//!         self.isHeader = true
//!         self.id = category
//!     else
//!         local id, name = GetAchievementInfo(category)  -- lua:2175
//!         self.id = id
//!         self:SetText(name)
//!         self.Background:Show()
//!         -- ... colorIndex-driven texcoord/blend/alpha at lua:2182-2192
//!         local criteriaString, ..., quantity
//!         if (not isSummary) then                     -- lua:2197 (free global!)
//!             quantity = GetStatistic(id)             -- lua:2198 (the call)
//!         else
//!             ... = GetAchievementCriteriaInfo(category)  -- lua:2200 (dead)
//!         end
//!         if (not quantity) then quantity = "--" end  -- lua:2202-2204
//!         self.Value:SetText(quantity)                -- lua:2205
//!         self.Title:Hide(); self.Left:Hide(); self.Middle:Hide(); self.Right:Hide()
//!         self.isHeader = false
//!     end
//! end
//! ```
//!
//! Registration: `_G.GetStatistic` is wired at
//! `register_summary_globals` in
//! `src/lua_api/globals/missing_surface/achievement_info.rs:387`. The
//! depends-on tag `(depends-on: GetStatistic gap)` is stale.
//!
//! **Spec/source mismatch on FOUR axes:**
//!
//! 1. **No "quantity column" widget.** PLAN says "populates the quantity
//!    column". The widget written at lua:2205 is `self.Value`, a
//!    `FontString` declared in the StatTemplate XML — not a column.
//!    There is no `Column`, `QuantityColumn`, or similar widget anywhere
//!    in the StatTemplate hierarchy. PLAN's wording would lead a reader
//!    to look for a column widget that doesn't exist.
//! 2. **`GetStatistic(id)` only fires on non-header non-summary rows.**
//!    PLAN's wording "populates the quantity ... using `GetStatistic(id)`"
//!    elides the header branch (which blanks `Value:SetText("")` at
//!    lua:2168 — explicitly NOT using `GetStatistic`) AND the dead
//!    `isSummary` branch at lua:2200 (`GetAchievementCriteriaInfo` —
//!    never reached because `_G.isSummary == nil`).
//! 3. **`isSummary` is dead-code free global.** lua:2197 references
//!    `isSummary` without declaring it; the only `isSummary` writes in
//!    the file are `button.isSummary = true` at lua:2369 (a frame
//!    property). Since the function has no upvalue named `isSummary`,
//!    Lua resolves it as `_G.isSummary`, which is `nil`. The condition
//!    `not nil == true` always wins, so `GetAchievementCriteriaInfo` at
//!    lua:2200 is dead code in production. (Same pattern at lua:2955.)
//!    The test asserts `_G.isSummary == nil` to pin this — a non-nil
//!    reading would mean some other addon set the global and the dead
//!    branch suddenly came alive.
//! 4. **`nil` quantity falls back to `"--"`.** lua:2202-2204 routes
//!    `GetStatistic` returning nil to the literal string `"--"`. PLAN's
//!    wording implies the call result is written verbatim; in reality
//!    the dash fallback is part of the contract, and a regression that
//!    omits it would write `nil` to the FontString (Lua-side error or
//!    silent stringification depending on the widget impl).
//!
//! Two tests split the non-header and header branches. Each builds a
//! fake row with the `Value`/`Title`/`Left`/`Middle`/`Right`/`Background`
//! sub-tables the body touches. Spies installed for `GetAchievementInfo`,
//! `GetStatistic`, `GetCategoryInfo`, and `GetAchievementCriteriaInfo`
//! (the dead-branch tripwire) capture call counts and args.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_NAMED_BUT_ABSENT_COLUMN_SETTER: &str = "AchievementStatColumn_SetQuantity";
const RESOLVED_ACHIEVEMENT_ID: i64 = 4242;
const RESOLVED_ACHIEVEMENT_NAME: &str = "StubAchievement";
const STATISTIC_TOKEN: &str = "STATISTIC_42_TOKEN";
const HEADER_CATEGORY_TEXT: &str = "HEADER_CATEGORY_TOKEN";
const EXPECTED_NON_HEADER: &str = "achievement_info_calls=1 achievement_info_arg=99 \
    statistic_calls=1 statistic_arg=4242 criteria_info_calls=0 \
    value_set_calls=1 value_text=STATISTIC_42_TOKEN self_set_text_arg=StubAchievement \
    is_header=false self_id=4242 background_show=1 background_hide=0";
const EXPECTED_HEADER_PLUS_NIL_FALLBACK: &str = "header_title_calls=1 header_title_text=HEADER_CATEGORY_TOKEN \
    header_value_calls=1 header_value_text= header_statistic_calls=0 \
    header_achievement_info_calls=0 header_background_hide=1 header_is_header=true \
    nil_fallback_value_text=-- nil_fallback_statistic_calls=1 is_summary_global_type=nil";

type NonHeaderProbe = (String, String, String, String, String);
type HeaderProbe = (String, String, String);

const FAKE_ROW_BUILDER: &str = r#"
    local function noop_method() end
    local function build_row(captures)
        return {
            id = nil,
            isHeader = nil,
            Value = {
                SetText = function(self, text)
                    captures.value_set_calls = captures.value_set_calls + 1
                    captures.value_text = tostring(text or "")
                end,
                SetVertexColor = noop_method,
            },
            Title = {
                SetText = function(self, text)
                    captures.title_set_calls = captures.title_set_calls + 1
                    captures.title_text = tostring(text or "")
                end,
                Show = noop_method, Hide = noop_method,
                IsTruncated = function(self) return false end,
                GetText = function(self) return "" end,
            },
            Left = {Show = noop_method, Hide = noop_method},
            Middle = {Show = noop_method, Hide = noop_method},
            Right = {Show = noop_method, Hide = noop_method},
            Background = {
                Show = function(self) captures.background_show = captures.background_show + 1 end,
                Hide = function(self) captures.background_hide = captures.background_hide + 1 end,
                SetTexCoord = noop_method, SetBlendMode = noop_method, SetAlpha = noop_method,
            },
            SetText = function(self, text)
                captures.self_set_text_calls = captures.self_set_text_calls + 1
                captures.self_set_text_arg = tostring(text or "")
            end,
            SetHeight = noop_method,
            SetPushedTextOffset = noop_method,
        }
    end
"#;

#[test]
fn stat_template_init_non_header_calls_get_statistic_and_writes_value_skipping_dead_criteria_branch()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: NonHeaderProbe = env
            .eval(&format!(
                r#"
                assert(_G.AchievementStatTemplateMixin,
                    "AchievementStatTemplateMixin must exist (lua:2125)")
                assert(_G.AchievementStatTemplateMixin.Init,
                    "AchievementStatTemplateMixin:Init must exist (lua:2152)")

                {fake_row_builder}

                local captures = {{
                    achievement_info_calls = 0, achievement_info_arg = -1,
                    statistic_calls = 0, statistic_arg = -1,
                    criteria_info_calls = 0,
                    value_set_calls = 0, value_text = "<unset>",
                    title_set_calls = 0, title_text = "<unset>",
                    self_set_text_calls = 0, self_set_text_arg = "<unset>",
                    background_show = 0, background_hide = 0,
                }}

                local original_achievement_info = _G.GetAchievementInfo
                _G.GetAchievementInfo = function(category)
                    captures.achievement_info_calls = captures.achievement_info_calls + 1
                    captures.achievement_info_arg = category
                    return {resolved_id}, "{resolved_name}", 10, false, 0, 0, 0, "stub_desc",
                        0, "Interface\\Icons\\stub", "", false, false, ""
                end
                local original_statistic = _G.GetStatistic
                _G.GetStatistic = function(id)
                    captures.statistic_calls = captures.statistic_calls + 1
                    captures.statistic_arg = id
                    return "{statistic_token}"
                end
                local original_criteria_info = _G.GetAchievementCriteriaInfo
                _G.GetAchievementCriteriaInfo = function(category)
                    captures.criteria_info_calls = captures.criteria_info_calls + 1
                    return "criteria_string", "criteria_type", false, 0, 100, "char", 0, 0, "criteria_qty"
                end

                local fake_row = build_row(captures)
                AchievementStatTemplateMixin.Init(
                    fake_row, {{id = 99, colorIndex = 1, header = false}})

                _G.GetAchievementInfo = original_achievement_info
                _G.GetStatistic = original_statistic
                _G.GetAchievementCriteriaInfo = original_criteria_info

                local non_header_signature = string.format(
                    "achievement_info_calls=%d achievement_info_arg=%d " ..
                    "statistic_calls=%d statistic_arg=%d criteria_info_calls=%d " ..
                    "value_set_calls=%d value_text=%s self_set_text_arg=%s " ..
                    "is_header=%s self_id=%s background_show=%d background_hide=%d",
                    captures.achievement_info_calls, captures.achievement_info_arg,
                    captures.statistic_calls, captures.statistic_arg,
                    captures.criteria_info_calls,
                    captures.value_set_calls, captures.value_text,
                    captures.self_set_text_arg,
                    tostring(fake_row.isHeader), tostring(fake_row.id),
                    captures.background_show, captures.background_hide)

                return type(_G.AchievementStatTemplateMixin),
                       type(_G.AchievementStatTemplateMixin.Init),
                       type(_G.GetStatistic),
                       type(_G[ "{plan_named_setter}" ]),
                       non_header_signature
                "#,
                fake_row_builder = FAKE_ROW_BUILDER,
                resolved_id = RESOLVED_ACHIEVEMENT_ID,
                resolved_name = RESOLVED_ACHIEVEMENT_NAME,
                statistic_token = STATISTIC_TOKEN,
                plan_named_setter = PLAN_NAMED_BUT_ABSENT_COLUMN_SETTER,
            ))
            .expect("stat-template non-header drive must run cleanly");

        let (mixin_type, init_type, get_statistic_type, plan_named_setter_type, signature) =
            observations;

        assert_eq!(
            mixin_type, "table",
            "Expected `_G.AchievementStatTemplateMixin` to be a table (declared at \
             `Mainline/Blizzard_AchievementUI.lua:2125`). Got `{mixin_type}`. A `nil` \
             reading means the stat-template mixin module assignment regressed; every \
             stats row would crash in the ScrollBox initializer at lua:2224-2226 \
             (`view:SetElementInitializer(\"AchievementStatTemplate\", function(button, \
             elementData) button:Init(elementData) end)`)."
        );

        assert_eq!(
            init_type, "function",
            "Expected `AchievementStatTemplateMixin.Init` to be a function (declared at \
             lua:2152). Got `{init_type}`. A `nil` reading means the per-row initializer \
             would crash with `attempt to call nil` on every stats-frame entry; the \
             quantity write at lua:2205 would never fire."
        );

        assert_eq!(
            get_statistic_type, "function",
            "Expected `_G.GetStatistic` to be a function (registered at \
             `register_summary_globals` in `achievement_info.rs:387`). Got \
             `{get_statistic_type}`. PLAN's `(depends-on: GetStatistic gap)` tag is stale \
             — a `nil` reading would mean the call at lua:2198 crashes, and the cell \
             would silently fall back to `--` via lua:2202-2204 ONLY because \
             `not (call-error)` propagates up — actually no, the call would be a hard \
             error before reaching the nil-fallback. So a `nil` reading means stats \
             rows never paint."
        );

        assert_eq!(
            plan_named_setter_type, "nil",
            "Expected `_G.{PLAN_NAMED_BUT_ABSENT_COLUMN_SETTER}` to be nil — PLAN says \
             `populates the quantity column`, but no `*Column*` widget or setter exists \
             in the stat-template hierarchy. The widget written at lua:2205 is \
             `self.Value`, a FontString. Got `{plan_named_setter_type}`. A non-nil \
             reading means a future refactor introduced an alias under the PLAN-named \
             symbol; flag the rename and update PLAN's wording rather than silently \
             dropping this tripwire."
        );

        assert_eq!(
            signature, EXPECTED_NON_HEADER,
            "Expected non-header signature to match. The drive stubs `GetAchievementInfo` \
             to return id={RESOLVED_ACHIEVEMENT_ID}/name={RESOLVED_ACHIEVEMENT_NAME:?}, \
             `GetStatistic` to return {STATISTIC_TOKEN:?}, and `GetAchievementCriteriaInfo` \
             to return a token tuple (the dead-branch spy at lua:2200), then calls \
             `AchievementStatTemplateMixin.Init(fake_row, {{id=99, colorIndex=1, \
             header=false}})`. Expected `{EXPECTED_NON_HEADER}`. Got `{signature}`. \
             An `achievement_info_calls=0` means the non-header branch at lua:2175 was \
             not entered (likely the `elementData.header` gate at lua:2155 routed \
             header=false into the header branch — bug). An `achievement_info_arg` other \
             than 99 means lua:2153 (`local category = elementData.id`) read the wrong \
             field. A `statistic_calls=0` means the dead-branch gate at lua:2197 \
             (`if not isSummary`) inverted — `_G.isSummary` is nil so the condition \
             must be true and `GetStatistic(id)` must fire. A `statistic_arg` other than \
             {RESOLVED_ACHIEVEMENT_ID} means lua:2198 passed the wrong id (likely the \
             `category` from elementData rather than the resolved `id` from \
             `GetAchievementInfo` at lua:2175). A `criteria_info_calls=1` proves the dead \
             `isSummary` branch at lua:2200 ALIVE — meaning some addon set \
             `_G.isSummary` and the production code path silently changed. A \
             `value_text` other than the statistic token means lua:2205 routed the \
             wrong value (or the fallback at lua:2202-2204 fired despite the spy \
             returning a non-nil token). A `self_set_text_arg` other than the resolved \
             name means lua:2179 (`self:SetText(name)`) read the wrong unpack slot. A \
             `background_show=0` means lua:2180 didn't fire. An `is_header=true` means \
             lua:2212 (`self.isHeader = false`) didn't fire."
        );
    });
}

#[test]
fn stat_template_init_header_branch_blanks_value_skipping_get_statistic_and_dash_fallback_on_nil() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: HeaderProbe = env
            .eval(&format!(
                r#"
                {fake_row_builder}

                -- Drive A: header branch
                local captures = {{
                    achievement_info_calls = 0, statistic_calls = 0,
                    title_set_calls = 0, title_text = "<unset>",
                    value_set_calls = 0, value_text = "<unset>",
                    self_set_text_calls = 0, self_set_text_arg = "<unset>",
                    background_show = 0, background_hide = 0,
                }}
                local original_category_info = _G.GetCategoryInfo
                _G.GetCategoryInfo = function(category) return "{header_text}" end
                local original_achievement_info = _G.GetAchievementInfo
                _G.GetAchievementInfo = function(category)
                    captures.achievement_info_calls = captures.achievement_info_calls + 1
                    return 0, "should_not_appear", 0, false
                end
                local original_statistic = _G.GetStatistic
                _G.GetStatistic = function(id)
                    captures.statistic_calls = captures.statistic_calls + 1
                    return "should_not_appear"
                end

                local header_row = build_row(captures)
                AchievementStatTemplateMixin.Init(
                    header_row, {{id = 555, colorIndex = 1, header = true}})

                local header_signature = string.format(
                    "header_title_calls=%d header_title_text=%s " ..
                    "header_value_calls=%d header_value_text=%s " ..
                    "header_statistic_calls=%d header_achievement_info_calls=%d " ..
                    "header_background_hide=%d header_is_header=%s",
                    captures.title_set_calls, captures.title_text,
                    captures.value_set_calls, captures.value_text,
                    captures.statistic_calls, captures.achievement_info_calls,
                    captures.background_hide, tostring(header_row.isHeader))

                -- Drive B: non-header with GetStatistic returning nil
                captures.achievement_info_calls = 0
                captures.statistic_calls = 0
                captures.value_set_calls = 0
                captures.value_text = "<unset>"
                _G.GetStatistic = function(id)
                    captures.statistic_calls = captures.statistic_calls + 1
                    return nil
                end
                _G.GetAchievementInfo = function(category)
                    captures.achievement_info_calls = captures.achievement_info_calls + 1
                    return 4242, "stub_name", 10, false, 0, 0, 0, "stub_desc",
                        0, "Interface\\Icons\\stub", "", false, false, ""
                end

                local nil_row = build_row(captures)
                AchievementStatTemplateMixin.Init(
                    nil_row, {{id = 99, colorIndex = 1, header = false}})

                local nil_signature = string.format(
                    "nil_fallback_value_text=%s nil_fallback_statistic_calls=%d",
                    captures.value_text, captures.statistic_calls)

                _G.GetCategoryInfo = original_category_info
                _G.GetAchievementInfo = original_achievement_info
                _G.GetStatistic = original_statistic

                local is_summary_global_type = type(_G.isSummary)
                local combined_signature = header_signature .. " " .. nil_signature ..
                    " is_summary_global_type=" .. is_summary_global_type

                return is_summary_global_type,
                       tostring(header_row.id),
                       combined_signature
                "#,
                fake_row_builder = FAKE_ROW_BUILDER,
                header_text = HEADER_CATEGORY_TEXT,
            ))
            .expect("stat-template header + nil-fallback drive must run cleanly");

        let (is_summary_global_type, header_self_id, combined_signature) = observations;

        assert_eq!(
            is_summary_global_type, "nil",
            "Expected `_G.isSummary` to be nil — the gate at lua:2197 (`if not isSummary`) \
             references a free GLOBAL (never declared local in `Init` or its enclosing \
             scope; the only `isSummary` writes in the file are `button.isSummary = true` \
             at lua:2369, a frame property). Lua resolves `isSummary` as `_G.isSummary`, \
             which is nil → `not nil == true` → the `GetStatistic(id)` branch at \
             lua:2198 always wins, and the `GetAchievementCriteriaInfo(category)` branch \
             at lua:2200 is dead code in production. Got `{is_summary_global_type}`. A \
             non-nil reading means some addon (or a future Blizzard refactor) set \
             `_G.isSummary` and the dead branch came alive — every non-header stats row \
             would now route through `GetAchievementCriteriaInfo` instead of \
             `GetStatistic`, silently changing the displayed numbers. Same dead-code \
             pattern at lua:2955."
        );

        assert_eq!(
            header_self_id, "555",
            "Expected `header_row.id == 555` after the header-branch drive — lua:2173 \
             (`self.id = category;` inside the header branch) writes the elementData.id \
             onto the row. Got `{header_self_id}`. A different value means lua:2173 \
             didn't fire (likely the header gate at lua:2155 misfired)."
        );

        assert_eq!(
            combined_signature, EXPECTED_HEADER_PLUS_NIL_FALLBACK,
            "Expected combined signature to match. Drive A (header) calls Init with \
             `{{id=555, colorIndex=1, header=true}}` and stubs `GetCategoryInfo` to \
             return {HEADER_CATEGORY_TEXT:?}; the header branch at lua:2155-2173 must \
             route to `Title:SetText({HEADER_CATEGORY_TEXT:?})` (lua:2166), \
             `Value:SetText(\"\")` (lua:2168, blank), `Background:Hide()` (lua:2171), \
             and MUST NOT call `GetAchievementInfo` or `GetStatistic`. Drive B \
             (nil-fallback) calls Init with `header=false` and stubs `GetStatistic` to \
             return nil; the nil-fallback at lua:2202-2204 must coerce to literal \
             `\"--\"` and write to `Value:SetText`. Expected \
             `{EXPECTED_HEADER_PLUS_NIL_FALLBACK}`. Got `{combined_signature}`. A \
             `header_value_text` other than empty means lua:2168 wrote the wrong value. \
             A `header_statistic_calls=1` means the header branch leaked into the \
             non-header `GetStatistic` path at lua:2198 — probably the `elementData.header` \
             gate at lua:2155 inverted. A `header_achievement_info_calls=1` means the \
             non-header branch at lua:2175 ran despite header=true. A \
             `nil_fallback_value_text` other than `--` means the dash fallback at \
             lua:2202-2204 didn't fire (a `nil_fallback_value_text=nil` would mean \
             `Value:SetText(nil)` was called — Lua-side that's a typed error or silent \
             stringification; a token like `STATISTIC_42_TOKEN` would mean the spy \
             override didn't take effect)."
        );
    });
}
