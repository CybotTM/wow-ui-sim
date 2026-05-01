//! Behavior pin: PLAN-named `AchievementFrameComparison_Update` does
//! NOT exist; the comparison frame has `_UpdateDataProvider`,
//! `_UpdateStatsDataProvider`, `_UpdateStatusBars`, and `_ForceUpdate`
//! but none of those call `C_AchievementInfo.SetPortraitTexture`. The
//! actual call sites for the portrait texture are FOUR distinct
//! handlers, each passing ONLY the portrait texture (NOT a unit
//! argument as PLAN's wording claims). The unit data lives on the
//! portrait's `unit`/`race`/`sex` properties (lua:2841-2843), set by
//! `_SetUnit`, and the texture impl reads from those.
//!
//! Source map (the actual call sites):
//!
//! ```lua
//! -- lua:2792-2802 (OnShow)
//! function AchievementFrameComparison_OnShow(self)
//!     ...
//!     C_AchievementInfo.SetPortraitTexture(AchievementFrameComparisonHeaderPortrait);  -- line 2799
//!     ...
//! end
//! ```
//!
//! ```lua
//! -- lua:2814-2832 (OnEvent — three call sites)
//! function AchievementFrameComparison_OnEvent (self, event, ...)
//!     ...
//!     elseif event == "DISPLAY_SIZE_CHANGED" then
//!         C_AchievementInfo.SetPortraitTexture(AchievementFrameComparisonHeaderPortrait);  -- line 2821
//!     elseif event == "PORTRAITS_UPDATED" then
//!         C_AchievementInfo.SetPortraitTexture(AchievementFrameComparisonHeaderPortrait);  -- line 2823
//!     elseif event == "UNIT_PORTRAIT_UPDATE" then
//!         local updateUnit = ...;
//!         if UnitName(updateUnit) == AchievementFrameComparisonHeaderName:GetText() then
//!             C_AchievementInfo.SetPortraitTexture(AchievementFrameComparisonHeaderPortrait);  -- line 2827
//!         end
//!     end
//!     ...
//! end
//! ```
//!
//! ```lua
//! -- lua:2834-2844 (SetUnit — the only call site that ALSO writes
//! -- unit/race/sex onto the portrait BEFORE calling the texture API)
//! function AchievementFrameComparison_SetUnit (unit)
//!     ClearAchievementComparisonUnit();
//!     SetAchievementComparisonUnit(unit);
//!     ...
//!     C_AchievementInfo.SetPortraitTexture(AchievementFrameComparisonHeaderPortrait);  -- line 2840
//!     AchievementFrameComparisonHeaderPortrait.unit = unit;                              -- line 2841
//!     AchievementFrameComparisonHeaderPortrait.race = UnitRace(unit);                    -- line 2842
//!     AchievementFrameComparisonHeaderPortrait.sex = UnitSex(unit);                      -- line 2843
//! end
//! ```
//!
//! XML chain (`AchievementFrameComparisonHeaderPortrait` resolves via
//! nested `$parent`):
//!
//! - `<Frame name="$parentComparison">` at xml:2080 → AchievementFrameComparison
//! - `<Frame name="$parentHeader">` at xml:2087 → AchievementFrameComparisonHeader
//! - `<Texture name="$parentPortrait">` at xml:2103 → AchievementFrameComparisonHeaderPortrait
//!   (a 36×36 Texture in `Layer level="BACKGROUND"`).
//!
//! Rust-side impl
//! (`src/lua_api/globals/missing_surface/achievement_info.rs:201-220`):
//! `c_achievement_info_set_portrait_texture` writes
//! `DEFAULT_PORTRAIT_PATH = "Interface\\Icons\\Achievement_Character_Default"`
//! (line 23) onto the supplied texture's `frame.texture` field, clears
//! the atlas/file-id/color-texture overrides, and returns `Val::Bool(true)`.
//!
//! **Spec/source mismatch on THREE axes:**
//!
//! 1. **`AchievementFrameComparison_Update` does not exist.** The
//!    comparison frame has `_UpdateDataProvider` (lua:2854),
//!    `_UpdateStatsDataProvider` (lua:2868), `_UpdateStatusBars`
//!    (lua:820), and `_ForceUpdate` (lua:2846), but PLAN's named
//!    function is fully imagined; none of the existing `_Update*`
//!    siblings call `SetPortraitTexture`.
//! 2. **`SetPortraitTexture(portrait, unit)` arity is wrong.** Every
//!    one of the FIVE actual call sites (lua:2799, 2821, 2823, 2827,
//!    2840) passes ONLY the portrait texture. The unit argument PLAN
//!    appends would be a stack-unused extra arg in the impl at
//!    `achievement_info.rs:201-220` — the impl reads `frame_id_from_stack(state, 1)`
//!    and ignores anything on stack slot 2.
//! 3. **The depends-on tag `C_AchievementInfo.SetPortraitTexture gap`
//!    is stale.** The C API is implemented at
//!    `src/lua_api/globals/missing_surface/achievement_info.rs:160`
//!    (registration on the `C_AchievementInfo` namespace stub) and
//!    `:201` (impl, writes `DEFAULT_PORTRAIT_PATH` to the texture).
//!
//! Eight assertions split absence/presence/behavior:
//!
//! - **Absence half** (1): `_G.AchievementFrameComparison_Update` is
//!   nil. A non-nil reading would prove Blizzard added a real
//!   `_Update` orchestrator to the comparison frame (in which case
//!   PLAN's wording becomes accurate retroactively and the absence
//!   half should flip to a behavior probe driving the new function).
//! - **Presence half** (5): `C_AchievementInfo.SetPortraitTexture` is
//!   a function (depends-on stale); the four ACTUAL call-site
//!   handlers (`_OnShow`, `_OnEvent`, `_SetUnit`) are all functions;
//!   `AchievementFrameComparisonHeaderPortrait:GetObjectType() == "Texture"`.
//! - **Behavior half** (2):
//!   `C_AchievementInfo.SetPortraitTexture(AchievementFrameComparisonHeaderPortrait)`
//!   returns `true` (the impl's success path); after the call,
//!   `AchievementFrameComparisonHeaderPortrait:GetTexture()` is a
//!   non-empty string containing `"Achievement_Character_Default"`
//!   (the `DEFAULT_PORTRAIT_PATH` written by the impl).

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_NAMED_BUT_ABSENT_FUNCTION: &str = "AchievementFrameComparison_Update";
const PLAN_REFERENCED_C_API: &str = "C_AchievementInfo.SetPortraitTexture";
const EXPECTED_TEXTURE_PATH_FRAGMENT: &str = "Achievement_Character_Default";

type ComparisonPortraitProbe = (String, String, String, String, String, String, bool, String);

#[test]
fn comparison_update_is_absent_but_set_portrait_texture_writes_default_portrait_path() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: ComparisonPortraitProbe = env
            .eval(
                r#"
                local plan_named_function_type = type(_G.AchievementFrameComparison_Update)
                local set_portrait_api_type =
                    (type(_G.C_AchievementInfo) == "table"
                        and type(_G.C_AchievementInfo.SetPortraitTexture))
                    or "no-c-achievement-info-table"

                local on_show_handler_type = type(_G.AchievementFrameComparison_OnShow)
                local on_event_handler_type = type(_G.AchievementFrameComparison_OnEvent)
                local set_unit_handler_type = type(_G.AchievementFrameComparison_SetUnit)

                local portrait_object_type = "no-portrait-texture"
                if type(_G.AchievementFrameComparisonHeaderPortrait) == "table" then
                    portrait_object_type = AchievementFrameComparisonHeaderPortrait:GetObjectType()
                end

                local set_portrait_call_returned_true = false
                local portrait_texture_path = ""
                if type(_G.C_AchievementInfo) == "table"
                    and type(_G.C_AchievementInfo.SetPortraitTexture) == "function"
                    and type(_G.AchievementFrameComparisonHeaderPortrait) == "table"
                then
                    local ok, result = pcall(C_AchievementInfo.SetPortraitTexture,
                        AchievementFrameComparisonHeaderPortrait)
                    set_portrait_call_returned_true = (ok and result == true)
                    portrait_texture_path =
                        AchievementFrameComparisonHeaderPortrait:GetTexture() or ""
                end

                return plan_named_function_type,
                       set_portrait_api_type,
                       on_show_handler_type,
                       on_event_handler_type,
                       set_unit_handler_type,
                       portrait_object_type,
                       set_portrait_call_returned_true,
                       portrait_texture_path
                "#,
            )
            .expect("AchievementFrameComparison portrait probe must run cleanly");

        let (
            plan_named_function_type,
            set_portrait_api_type,
            on_show_handler_type,
            on_event_handler_type,
            set_unit_handler_type,
            portrait_object_type,
            set_portrait_call_returned_true,
            portrait_texture_path,
        ) = observations;

        assert_eq!(
            plan_named_function_type, "nil",
            "Expected `_G.{PLAN_NAMED_BUT_ABSENT_FUNCTION}` to be nil — the comparison \
             frame has `_UpdateDataProvider` (lua:2854), `_UpdateStatsDataProvider` \
             (lua:2868), `_UpdateStatusBars` (lua:820), and `_ForceUpdate` (lua:2846), \
             but no plain `_Update`. Got `{plan_named_function_type}`. A non-nil reading \
             would prove Blizzard added a real `_Update` orchestrator (in which case the \
             absence half should flip to a behavior probe driving the new function and \
             asserting it reaches `SetPortraitTexture` plus one of the existing \
             update-data-provider call sites). The actual portrait-update fan-out lives \
             on the four handlers asserted below."
        );

        assert_eq!(
            set_portrait_api_type, "function",
            "Expected `{PLAN_REFERENCED_C_API}` to be a function (PLAN tags this as a \
             gap, but it's implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:160` (registration \
             on the `C_AchievementInfo` namespace) and `:201` (impl: writes \
             `DEFAULT_PORTRAIT_PATH = \"Interface\\\\Icons\\\\Achievement_Character_Default\"` \
             at line 212 to the texture's `frame.texture` field, clears \
             atlas/file-id/color-texture overrides, returns `Val::Bool(true)`)). Got \
             `{set_portrait_api_type}`. The depends-on tag is stale; if this assertion \
             fails the FIVE call sites at lua:2799/2821/2823/2827/2840 would all crash \
             with `attempt to call a nil value (field 'SetPortraitTexture')`."
        );

        assert_eq!(
            on_show_handler_type, "function",
            "Expected `_G.AchievementFrameComparison_OnShow` to be a function — declared \
             at `Mainline/Blizzard_AchievementUI.lua:2792` and bound via xml:2305 \
             (`<OnShow function=\"AchievementFrameComparison_OnShow\"/>`). Calls \
             `C_AchievementInfo.SetPortraitTexture(AchievementFrameComparisonHeaderPortrait)` \
             at lua:2799 — the FIRST opportunity to paint the portrait when the \
             comparison pane becomes visible. Got `{on_show_handler_type}`. A `nil` \
             reading means OnShow dispatch is unwired and the portrait would stay blank \
             until an event triggers a refresh."
        );

        assert_eq!(
            on_event_handler_type, "function",
            "Expected `_G.AchievementFrameComparison_OnEvent` to be a function — declared \
             at `Mainline/Blizzard_AchievementUI.lua:2814` and bound via xml:2304 \
             (`<OnEvent function=\"AchievementFrameComparison_OnEvent\"/>`). Owns three \
             of the five `SetPortraitTexture` call sites: lua:2821 \
             (DISPLAY_SIZE_CHANGED), lua:2823 (PORTRAITS_UPDATED), lua:2827 \
             (UNIT_PORTRAIT_UPDATE — gated by \
             `UnitName(updateUnit) == AchievementFrameComparisonHeaderName:GetText()`). \
             Got `{on_event_handler_type}`. A `nil` reading means the portrait would \
             never refresh on resolution changes, GPU portrait rebuilds, or inspect \
             updates."
        );

        assert_eq!(
            set_unit_handler_type, "function",
            "Expected `_G.AchievementFrameComparison_SetUnit` to be a function — declared \
             at `Mainline/Blizzard_AchievementUI.lua:2834`. The fifth call site at \
             lua:2840 paints the portrait BEFORE writing the \
             `unit`/`race`/`sex` properties at lua:2841-2843. PLAN's wording \
             `SetPortraitTexture(..., unit)` collapses these two lines into one — but \
             the actual call passes only the portrait, and the per-unit metadata is \
             written separately as portrait properties for downstream readers. Got \
             `{set_unit_handler_type}`. A `nil` reading means the call chain \
             `_DisplayComparison(unit) → _SetUnit(unit) → SetPortraitTexture(portrait)` \
             at lua:233 → 2840 would crash."
        );

        assert_eq!(
            portrait_object_type, "Texture",
            "Expected `AchievementFrameComparisonHeaderPortrait:GetObjectType()` to \
             return `\"Texture\"` — declared at `Mainline/Blizzard_AchievementUI.xml:2103` \
             as `<Texture name=\"$parentPortrait\">` inside \
             `AchievementFrameComparisonHeader` (xml:2087, the `$parentHeader` whose \
             outer `$parent` is `AchievementFrameComparison` at xml:2080). 36×36 in \
             `Layer level=\"BACKGROUND\"` with custom `<TexCoords>`. Got \
             `{portrait_object_type}`. A `no-portrait-texture` reading means the nested \
             `$parent`-name resolution dropped (e.g. the Header frame failed to \
             instantiate, breaking the inner Texture's name binding)."
        );

        assert!(
            set_portrait_call_returned_true,
            "Expected `C_AchievementInfo.SetPortraitTexture(AchievementFrameComparisonHeaderPortrait)` \
             to return `true` — the impl at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:201-220` returns \
             `Val::Bool(false)` only when the first arg fails `frame_id_from_stack` \
             (line 202) or `widgets.get_mut_visual` (line 207); otherwise it writes the \
             default path and returns `Val::Bool(true)` at line 218. Got `false`. A \
             `false` reading means the portrait Texture's frame id couldn't be resolved \
             — likely a userdata/handle mismatch where the Lua-side global doesn't carry \
             a Rust-side widget id. Note: PLAN's wording adds a `unit` second argument; \
             the impl reads only stack slot 1, so passing or omitting the second arg \
             does not change the return value."
        );

        assert!(
            portrait_texture_path.contains(EXPECTED_TEXTURE_PATH_FRAGMENT),
            "Expected `AchievementFrameComparisonHeaderPortrait:GetTexture()` to contain \
             `{EXPECTED_TEXTURE_PATH_FRAGMENT:?}` after \
             `C_AchievementInfo.SetPortraitTexture(AchievementFrameComparisonHeaderPortrait)`. \
             The impl at `achievement_info.rs:212` writes \
             `DEFAULT_PORTRAIT_PATH = \"Interface\\\\Icons\\\\Achievement_Character_Default\"` \
             (defined at line 23) to `frame.texture`. Got `{portrait_texture_path:?}`. \
             An empty reading means the texture field never got written (impl dropped \
             the borrow before the assignment, or the widget lookup returned the wrong \
             frame); a string without the expected fragment means the default path was \
             changed in a way that breaks the comparison portrait's fallback rendering."
        );
    });
}
