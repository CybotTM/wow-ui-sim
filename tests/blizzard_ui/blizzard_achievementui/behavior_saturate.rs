//! Behavior pin: PLAN-named "Saturate / Desaturate toggle icon and
//! shield desaturation when the achievement completion state flips"
//! collapses FIVE structural facts about
//! `AchievementTemplateMixin:Saturate` /
//! `AchievementTemplateMixin:Desaturate` at
//! `Mainline/Blizzard_AchievementUI.lua:1359-1418`.
//!
//! 1. **`saturatedStyle` is a 3-way enum, not a boolean.** Saturate
//!    sets `self.saturatedStyle` to one of `"guild"` (lua:1366), \
//!    `"account"` (lua:1373), or `"normal"` (lua:1378), driven by
//!    `InGuildView()` (lua:103, captured as upvalue) and
//!    `self.accountWide`. Desaturate clears it to `nil` (lua:1393).
//!    PLAN's binary "saturated/desaturated" framing elides the three
//!    distinct styles, each with its own Background/TitleBar texture
//!    pair and BackdropBorderColor.
//! 2. **Side-effect surface is far larger than "icon and shield".**
//!    Saturate fires Background:SetTexture, TitleBar:SetTexture +
//!    SetTexCoord, SetBackdropBorderColor, Glow:SetVertexColor,
//!    Icon:Saturate, Shield:Saturate, Reward:SetVertexColor,
//!    Label:SetVertexColor, Description:SetTextColor +
//!    SetShadowOffset, plus the helper call to
//!    `:UpdatePlusMinusTexture()` (lua:1391). Desaturate fires the
//!    same list (with desaturated values) plus a final
//!    `:SetBackdropBorderColor(.5, .5, .5)` (lua:1418). PLAN's
//!    "icon and shield" wording elides ~9 other widgets.
//! 3. **`Shield.Points:SetVertexColor` placement is asymmetric.** In
//!    Saturate it lives INSIDE the InGuildView/non-guild branches
//!    (lua:1365 sets green `(0,1,0)` for guild, lua:1380 sets white
//!    `(1,1,1)` for non-guild). In Desaturate it lives in the COMMON
//!    TAIL at lua:1411 (`(.65, .65, .65)` always). A regression that
//!    moved either call would silently mis-color the Shield points.
//! 4. **No idempotence guard.** Unlike `:Expand` / `:Collapse` (which
//!    short-circuit at lua:1309 / lua:1333), Saturate and Desaturate
//!    fire all side effects on every call. The CALLER at lua:1252
//!    gates with `if self.saturatedStyle ~= saturatedStyle then
//!    self:Saturate() end` to avoid redundant calls. A regression
//!    that added an internal idempotence guard inside
//!    Saturate/Desaturate would skip side effects when the style
//!    happened to match — but the callers expect every call to fully
//!    re-apply.
//! 5. **`:UpdatePlusMinusTexture` is called as a side effect.** Both
//!    methods call it (Saturate at lua:1391, Desaturate at lua:1417).
//!    This couples saturation-state changes to +/- texture updates,
//!    because `:UpdatePlusMinusTexture`'s four-way SetTexCoord
//!    dispatch (lua:1126-1135) reads `self.saturatedStyle` to choose
//!    among the colored vs gray regions. A regression that dropped
//!    either call would leave the +/- icon stuck in its previous
//!    saturated/desaturated region until the next Expand/Collapse.
//!
//! Source map of the contract:
//!
//! ```lua
//! -- lua:103  local function InGuildView()  -- closure upvalue
//! -- lua:5    ACHIEVEMENT_RED_BORDER_COLOR  = CreateColor(0.7, 0.15, 0.05)
//! -- lua:6    ACHIEVEMENT_BLUE_BORDER_COLOR = CreateColor(0.129, 0.671, 0.875)
//!
//! function AchievementTemplateMixin:Saturate()                    -- lua:1359
//!     if (InGuildView()) then
//!         self.Background:SetTexture(...)
//!         self.TitleBar:SetTexture(...)
//!         self.TitleBar:SetTexCoord(0, 1, 0.83203125, 0.91015625)
//!         self:SetBackdropBorderColor(RED_BORDER:GetRGB())
//!         self.Shield.Points:SetVertexColor(0, 1, 0)              -- guild GREEN
//!         self.saturatedStyle = "guild"                            -- lua:1366
//!     else
//!         self.Background:SetTexture(...)
//!         if (self.accountWide) then
//!             self.TitleBar:SetTexture("AccountLevel-AchievementHeader")
//!             self.TitleBar:SetTexCoord(0, 1, 0, 0.375)
//!             self:SetBackdropBorderColor(BLUE_BORDER:GetRGB())
//!             self.saturatedStyle = "account"                      -- lua:1373
//!         else
//!             self.TitleBar:SetTexture("UI-Achievement-Borders")
//!             self.TitleBar:SetTexCoord(0, 1, 0.66015625, 0.73828125)
//!             self:SetBackdropBorderColor(RED_BORDER:GetRGB())
//!             self.saturatedStyle = "normal"                       -- lua:1378
//!         end
//!         self.Shield.Points:SetVertexColor(1, 1, 1)               -- non-guild WHITE
//!     end
//!     -- common tail (always fires)
//!     self.Glow:SetVertexColor(1.0, 1.0, 1.0)
//!     self.Icon:Saturate()
//!     self.Shield:Saturate()
//!     self.Reward:SetVertexColor(1, .82, 0)
//!     self.Label:SetVertexColor(1, 1, 1)
//!     self.Description:SetTextColor(0, 0, 0, 1)
//!     self.Description:SetShadowOffset(0, 0)
//!     self:UpdatePlusMinusTexture()                                -- lua:1391
//! end
//!
//! function AchievementTemplateMixin:Desaturate()                  -- lua:1392
//!     self.saturatedStyle = nil                                    -- lua:1393 UNCONDITIONAL
//!     if (InGuildView()) then
//!         self.Background:SetTexture("...Desaturated")
//!         self.TitleBar:SetTexture("UI-Achievement-Borders")
//!         self.TitleBar:SetTexCoord(0, 1, 0.74609375, 0.82421875)
//!     else
//!         self.Background:SetTexture("...Desaturated")
//!         if (self.accountWide) then
//!             self.TitleBar:SetTexture("AccountLevel-AchievementHeader")
//!             self.TitleBar:SetTexCoord(0, 1, 0.40625, 0.78125)
//!         else
//!             self.TitleBar:SetTexture("UI-Achievement-Borders")
//!             self.TitleBar:SetTexCoord(0, 1, 0.91796875, 0.99609375)
//!         end
//!     end
//!     -- common tail (always fires) — note Shield.Points lives HERE in Desaturate
//!     self.Glow:SetVertexColor(.22, .17, .13)
//!     self.Icon:Desaturate()
//!     self.Shield:Desaturate()
//!     self.Shield.Points:SetVertexColor(.65, .65, .65)             -- lua:1411 COMMON tail
//!     self.Reward:SetVertexColor(.8, .8, .8)
//!     self.Label:SetVertexColor(.65, .65, .65)
//!     self.Description:SetTextColor(1, 1, 1, 1)
//!     self.Description:SetShadowOffset(1, -1)
//!     self:UpdatePlusMinusTexture()                                -- lua:1417
//!     self:SetBackdropBorderColor(.5, .5, .5)                      -- lua:1418
//! end
//! ```
//!
//! `InGuildView` is `local function` at lua:103; the closure capture
//! means a `_G.InGuildView` override does NOT reach the methods. The
//! tests install the spy via `debug.setupvalue` on the function's
//! upvalue index after iterating `debug.getupvalue` to find it.
//! `ACHIEVEMENT_RED_BORDER_COLOR` and `ACHIEVEMENT_BLUE_BORDER_COLOR`
//! are file-scope globals at lua:5-6 (assigned via `CreateColor`); we
//! rely on the addon load to install them.
//!
//! Two tests split the Saturate (3-way style + common tail) and
//! Desaturate (unconditional clear + common tail + no-idempotence)
//! paths so each body stays under the readability budget.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_NAMED_BUT_ABSENT_COMPLETION_HOOK: &str = "AchievementTemplate_OnCompletionStateChanged";
const PLAN_NAMED_BUT_ABSENT_TOGGLE: &str = "AchievementTemplate_ToggleSaturated";
const EXPECTED_SATURATE: &str = "normal_saturated_style=normal \
    account_saturated_style=account guild_saturated_style=guild \
    normal_shield_points_white=1 normal_shield_points_green=0 \
    guild_shield_points_white=0 guild_shield_points_green=1 \
    normal_backdrop_border_calls=1 \
    common_tail_glow=3 common_tail_icon_saturate=3 common_tail_shield_saturate=3 \
    common_tail_reward=3 common_tail_label=3 \
    common_tail_description_text=3 common_tail_description_shadow=3 \
    common_tail_update_plus_minus=3";
const EXPECTED_DESATURATE: &str = "first_saturated_style_after=nil \
    second_saturated_style_after=nil \
    background_set_texture_count=2 title_bar_set_texture_count=2 \
    title_bar_set_tex_coord_count=2 \
    common_tail_icon_desaturate=2 common_tail_shield_desaturate=2 \
    common_tail_shield_points_gray=2 \
    common_tail_glow=2 common_tail_reward=2 common_tail_label=2 \
    common_tail_description_text=2 common_tail_description_shadow=2 \
    common_tail_update_plus_minus=2 \
    final_backdrop_border_gray=2";

type SaturateProbe = (String, String, String);
type DesaturateProbe = (String, String);

const FAKE_ROW_BUILDER: &str = r#"
    local function counter(captures, key)
        return function(self, ...) captures[key] = (captures[key] or 0) + 1 end
    end
    local function build_row(captures)
        local row = {
            id = 4242, completed = false, accountWide = false,
            saturatedStyle = nil,
        }
        row.Background = {
            SetTexture = counter(captures, "background_set_texture_count"),
        }
        row.TitleBar = {
            SetTexture = counter(captures, "title_bar_set_texture_count"),
            SetTexCoord = counter(captures, "title_bar_set_tex_coord_count"),
        }
        row.Glow = {SetVertexColor = counter(captures, "glow_set_vertex_color")}
        row.Icon = {
            Saturate = counter(captures, "icon_saturate"),
            Desaturate = counter(captures, "icon_desaturate"),
        }
        row.Shield = {
            Saturate = counter(captures, "shield_saturate"),
            Desaturate = counter(captures, "shield_desaturate"),
            Points = {SetVertexColor = function(self, r, g, b)
                captures.shield_points_calls = (captures.shield_points_calls or 0) + 1
                if r == 1 and g == 1 and b == 1 then
                    captures.shield_points_white = (captures.shield_points_white or 0) + 1
                elseif r == 0 and g == 1 and b == 0 then
                    captures.shield_points_green = (captures.shield_points_green or 0) + 1
                elseif math.abs(r - 0.65) < 0.001 and math.abs(g - 0.65) < 0.001 then
                    captures.shield_points_gray = (captures.shield_points_gray or 0) + 1
                end
            end},
        }
        row.Reward = {SetVertexColor = counter(captures, "reward_set_vertex_color")}
        row.Label = {SetVertexColor = counter(captures, "label_set_vertex_color")}
        row.Description = {
            SetTextColor = counter(captures, "description_set_text_color"),
            SetShadowOffset = counter(captures, "description_set_shadow_offset"),
        }
        row.PlusMinus = {
            Show = function() end, Hide = function() end,
            SetTexCoord = function() end,
        }
        row.SetBackdropBorderColor = function(self, r, g, b)
            captures.backdrop_border_calls = (captures.backdrop_border_calls or 0) + 1
            if math.abs(r - 0.5) < 0.001 and math.abs(g - 0.5) < 0.001
               and math.abs(b - 0.5) < 0.001 then
                captures.backdrop_border_gray =
                    (captures.backdrop_border_gray or 0) + 1
            end
        end
        return row
    end
    local function install_update_plus_minus_counter(row, captures)
        row.UpdatePlusMinusTexture = function(self)
            captures.update_plus_minus = (captures.update_plus_minus or 0) + 1
        end
    end
"#;

const IN_GUILD_VIEW_UPVALUE_HELPER: &str = r#"
    local function swap_in_guild_view_upvalue(target_func, replacement)
        for i = 1, 60 do
            local name, val = debug.getupvalue(target_func, i)
            if name == nil then break end
            if name == "InGuildView" then
                debug.setupvalue(target_func, i, replacement)
                return i, val
            end
        end
        return nil, nil
    end
"#;

#[test]
fn saturate_dispatches_three_way_style_via_in_guild_view_and_account_wide_and_fires_full_common_tail()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: SaturateProbe = env
            .eval(&format!(
                r#"
                assert(_G.AchievementTemplateMixin,
                    "AchievementTemplateMixin must exist (lua:1039)")
                assert(_G.AchievementTemplateMixin.Saturate,
                    "AchievementTemplateMixin:Saturate must exist (lua:1359)")
                assert(_G.AchievementTemplateMixin.Desaturate,
                    "AchievementTemplateMixin:Desaturate must exist (lua:1392)")

                {fake_row_builder}
                {upvalue_helper}

                local normal_captures = {{}}
                local normal_row = build_row(normal_captures)
                Mixin(normal_row, AchievementTemplateMixin)
                install_update_plus_minus_counter(normal_row, normal_captures)
                normal_row.accountWide = false
                local idx_n, orig_n = swap_in_guild_view_upvalue(
                    AchievementTemplateMixin.Saturate, function() return false end)
                assert(idx_n,
                    "Saturate must capture InGuildView upvalue (lua:1360, local at lua:103)")
                normal_row:Saturate()
                debug.setupvalue(AchievementTemplateMixin.Saturate, idx_n, orig_n)

                local account_captures = {{}}
                local account_row = build_row(account_captures)
                Mixin(account_row, AchievementTemplateMixin)
                install_update_plus_minus_counter(account_row, account_captures)
                account_row.accountWide = true
                local idx_a, orig_a = swap_in_guild_view_upvalue(
                    AchievementTemplateMixin.Saturate, function() return false end)
                account_row:Saturate()
                debug.setupvalue(AchievementTemplateMixin.Saturate, idx_a, orig_a)

                local guild_captures = {{}}
                local guild_row = build_row(guild_captures)
                Mixin(guild_row, AchievementTemplateMixin)
                install_update_plus_minus_counter(guild_row, guild_captures)
                guild_row.accountWide = false
                local idx_g, orig_g = swap_in_guild_view_upvalue(
                    AchievementTemplateMixin.Saturate, function() return true end)
                guild_row:Saturate()
                debug.setupvalue(AchievementTemplateMixin.Saturate, idx_g, orig_g)

                local saturate_signature = string.format(
                    "normal_saturated_style=%s account_saturated_style=%s " ..
                    "guild_saturated_style=%s " ..
                    "normal_shield_points_white=%d normal_shield_points_green=%d " ..
                    "guild_shield_points_white=%d guild_shield_points_green=%d " ..
                    "normal_backdrop_border_calls=%d " ..
                    "common_tail_glow=%d common_tail_icon_saturate=%d " ..
                    "common_tail_shield_saturate=%d " ..
                    "common_tail_reward=%d common_tail_label=%d " ..
                    "common_tail_description_text=%d common_tail_description_shadow=%d " ..
                    "common_tail_update_plus_minus=%d",
                    tostring(normal_row.saturatedStyle),
                    tostring(account_row.saturatedStyle),
                    tostring(guild_row.saturatedStyle),
                    normal_captures.shield_points_white or 0,
                    normal_captures.shield_points_green or 0,
                    guild_captures.shield_points_white or 0,
                    guild_captures.shield_points_green or 0,
                    normal_captures.backdrop_border_calls or 0,
                    (normal_captures.glow_set_vertex_color or 0)
                        + (account_captures.glow_set_vertex_color or 0)
                        + (guild_captures.glow_set_vertex_color or 0),
                    (normal_captures.icon_saturate or 0)
                        + (account_captures.icon_saturate or 0)
                        + (guild_captures.icon_saturate or 0),
                    (normal_captures.shield_saturate or 0)
                        + (account_captures.shield_saturate or 0)
                        + (guild_captures.shield_saturate or 0),
                    (normal_captures.reward_set_vertex_color or 0)
                        + (account_captures.reward_set_vertex_color or 0)
                        + (guild_captures.reward_set_vertex_color or 0),
                    (normal_captures.label_set_vertex_color or 0)
                        + (account_captures.label_set_vertex_color or 0)
                        + (guild_captures.label_set_vertex_color or 0),
                    (normal_captures.description_set_text_color or 0)
                        + (account_captures.description_set_text_color or 0)
                        + (guild_captures.description_set_text_color or 0),
                    (normal_captures.description_set_shadow_offset or 0)
                        + (account_captures.description_set_shadow_offset or 0)
                        + (guild_captures.description_set_shadow_offset or 0),
                    (normal_captures.update_plus_minus or 0)
                        + (account_captures.update_plus_minus or 0)
                        + (guild_captures.update_plus_minus or 0))

                return type(_G[ "{plan_named_completion_hook}" ]),
                       type(_G[ "{plan_named_toggle}" ]),
                       saturate_signature
                "#,
                fake_row_builder = FAKE_ROW_BUILDER,
                upvalue_helper = IN_GUILD_VIEW_UPVALUE_HELPER,
                plan_named_completion_hook = PLAN_NAMED_BUT_ABSENT_COMPLETION_HOOK,
                plan_named_toggle = PLAN_NAMED_BUT_ABSENT_TOGGLE,
            ))
            .expect("Saturate three-way drive must run cleanly");

        let (completion_hook_type, toggle_type, signature) = observations;

        assert_eq!(
            completion_hook_type, "nil",
            "Expected `_G.{PLAN_NAMED_BUT_ABSENT_COMPLETION_HOOK}` to be nil — PLAN says \
             `Saturate/Desaturate ... when the achievement completion state flips`, but \
             neither method reads `self.completed`. The completion-state binding lives in \
             the CALLER at lua:1252 (`if self.saturatedStyle ~= saturatedStyle then \
             self:Saturate() end`), not in a `*OnCompletionStateChanged` global. Got \
             `{completion_hook_type}`. A non-nil reading means a future refactor \
             introduced a completion-state hook; flag the rename and update PLAN's \
             wording rather than silently dropping this tripwire."
        );

        assert_eq!(
            toggle_type, "nil",
            "Expected `_G.{PLAN_NAMED_BUT_ABSENT_TOGGLE}` to be nil — PLAN says \
             `toggle ... saturation`, implying a single toggle entry point. The actual \
             API exposes TWO independent methods (`:Saturate` and `:Desaturate`) with no \
             toggle global. Got `{toggle_type}`. A non-nil reading means a future \
             refactor introduced a toggle wrapper; flag and revisit the test."
        );

        assert_eq!(
            signature, EXPECTED_SATURATE,
            "Expected Saturate three-way signature to match. Three drives: (1) \
             accountWide=false + InGuildView=false → saturatedStyle=\"normal\", \
             Shield.Points white at lua:1380; (2) accountWide=true + InGuildView=false → \
             saturatedStyle=\"account\", Shield.Points white at lua:1380 (same common \
             non-guild branch); (3) accountWide=any + InGuildView=true → \
             saturatedStyle=\"guild\", Shield.Points GREEN at lua:1365 (inside the \
             InGuildView branch — placement is asymmetric vs Desaturate which puts \
             Shield.Points coloring in the common tail). Common tail must fire 3 times \
             (once per drive) for Glow:SetVertexColor (lua:1382), Icon:Saturate \
             (lua:1383), Shield:Saturate (lua:1384), Reward:SetVertexColor (lua:1385), \
             Label:SetVertexColor (lua:1386), Description:SetTextColor (lua:1387), \
             Description:SetShadowOffset (lua:1388), UpdatePlusMinusTexture (lua:1391, \
             couples saturation state to +/- texture). \
             `normal_backdrop_border_calls=1` proves one SetBackdropBorderColor call \
             per non-guild drive (lua:1377 RED for normal). Expected \
             `{EXPECTED_SATURATE}`. Got `{signature}`. A `normal_saturated_style` other \
             than `normal` means lua:1378 wrote a different string. A `guild_saturated_style` \
             other than `guild` means the InGuildView upvalue swap missed (the closure \
             still references the original local). A `guild_shield_points_green=0` means \
             the asymmetric placement at lua:1365 was moved to the common tail \
             (silently breaking the guild-view green color). A \
             `common_tail_update_plus_minus` other than 3 means the lua:1391 helper \
             call was dropped from one of the drives — saturation-state changes would \
             leave the +/- texture stale until the next Expand/Collapse."
        );
    });
}

#[test]
fn desaturate_clears_saturated_style_unconditionally_fires_common_tail_and_has_no_idempotence_guard()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: DesaturateProbe = env
            .eval(&format!(
                r#"
                {fake_row_builder}
                {upvalue_helper}

                local captures = {{}}
                local row = build_row(captures)
                Mixin(row, AchievementTemplateMixin)
                install_update_plus_minus_counter(row, captures)
                row.accountWide = false
                row.saturatedStyle = "normal"

                local idx, orig = swap_in_guild_view_upvalue(
                    AchievementTemplateMixin.Desaturate, function() return false end)
                assert(idx,
                    "Desaturate must capture InGuildView upvalue (lua:1394, local at lua:103)")

                row:Desaturate()
                local first_saturated_style_after = tostring(row.saturatedStyle)

                row.saturatedStyle = "account"
                row:Desaturate()
                local second_saturated_style_after = tostring(row.saturatedStyle)

                debug.setupvalue(AchievementTemplateMixin.Desaturate, idx, orig)

                local desaturate_signature = string.format(
                    "first_saturated_style_after=%s second_saturated_style_after=%s " ..
                    "background_set_texture_count=%d title_bar_set_texture_count=%d " ..
                    "title_bar_set_tex_coord_count=%d " ..
                    "common_tail_icon_desaturate=%d common_tail_shield_desaturate=%d " ..
                    "common_tail_shield_points_gray=%d " ..
                    "common_tail_glow=%d common_tail_reward=%d common_tail_label=%d " ..
                    "common_tail_description_text=%d common_tail_description_shadow=%d " ..
                    "common_tail_update_plus_minus=%d " ..
                    "final_backdrop_border_gray=%d",
                    first_saturated_style_after, second_saturated_style_after,
                    captures.background_set_texture_count or 0,
                    captures.title_bar_set_texture_count or 0,
                    captures.title_bar_set_tex_coord_count or 0,
                    captures.icon_desaturate or 0, captures.shield_desaturate or 0,
                    captures.shield_points_gray or 0,
                    captures.glow_set_vertex_color or 0,
                    captures.reward_set_vertex_color or 0,
                    captures.label_set_vertex_color or 0,
                    captures.description_set_text_color or 0,
                    captures.description_set_shadow_offset or 0,
                    captures.update_plus_minus or 0,
                    captures.backdrop_border_gray or 0)

                return tostring(_G.AchievementTemplateMixin.Saturate ~= nil
                                and _G.AchievementTemplateMixin.Desaturate ~= nil),
                       desaturate_signature
                "#,
                fake_row_builder = FAKE_ROW_BUILDER,
                upvalue_helper = IN_GUILD_VIEW_UPVALUE_HELPER,
            ))
            .expect("Desaturate twice-drive must run cleanly");

        let (both_methods_present, signature) = observations;

        assert_eq!(
            both_methods_present, "true",
            "Expected both `:Saturate` and `:Desaturate` on `AchievementTemplateMixin` \
             (lua:1359 and lua:1392). Got `{both_methods_present}`. A `false` reading \
             means one of the methods was renamed or removed; the OnEnter / OnLeave \
             hover styling at lua:1252-1259 would break."
        );

        assert_eq!(
            signature, EXPECTED_DESATURATE,
            "Expected Desaturate twice-drive signature to match. Drive 1 starts with \
             `saturatedStyle=\"normal\"`, calls Desaturate; lua:1393 \
             (`self.saturatedStyle = nil`) must clear it BEFORE the InGuildView branch \
             (this is the unconditional clear — placement matters: a regression that \
             moved the clear inside one branch would leak the previous style on the \
             other branch). Drive 2 starts with `saturatedStyle=\"account\"` (re-set \
             between drives to prove the clear fires from any state), calls Desaturate \
             again — `second_saturated_style_after=nil` proves there is NO idempotence \
             guard (unlike Expand at lua:1333 / Collapse at lua:1309); every call \
             re-clears and re-fires the side effects. Common tail counters at 2 prove \
             both drives fired the full surface: Icon:Desaturate (lua:1409), \
             Shield:Desaturate (lua:1410), Shield.Points:SetVertexColor(.65,.65,.65) \
             (lua:1411 — note this is in the COMMON TAIL in Desaturate, not branch- \
             conditional like Saturate's lua:1365/1380 placement), Glow:SetVertexColor \
             (lua:1408), Reward:SetVertexColor (lua:1412), Label:SetVertexColor \
             (lua:1413), Description:SetTextColor (lua:1414), \
             Description:SetShadowOffset (lua:1415), UpdatePlusMinusTexture (lua:1417). \
             `final_backdrop_border_gray=2` pins lua:1418 \
             (`self:SetBackdropBorderColor(.5, .5, .5)`) — Saturate sets red/blue \
             border colors per branch; Desaturate's tail unconditionally writes the \
             gray-50 border. Expected `{EXPECTED_DESATURATE}`. Got `{signature}`. A \
             `first_saturated_style_after` other than `nil` means lua:1393 didn't clear \
             the field. A `common_tail_shield_points_gray` other than 2 means the \
             asymmetric placement was broken — Shield.Points coloring should fire from \
             the COMMON TAIL in Desaturate (always 2 calls for 2 drives), distinct from \
             Saturate's branch-conditional placement. A `final_backdrop_border_gray` \
             other than 2 means lua:1418 didn't fire — the gray border would not get \
             applied, leaving the row with the previous Saturate red/blue border."
        );
    });
}
