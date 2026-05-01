//! Mixin application surface for the `Blizzard_ActionBar` lane —
//! pins that `StanceBar`, `PetActionBar`, `PossessActionBar`,
//! `ExtraActionButton1`, and `MainMenuBarVehicleLeaveButton` each
//! received their expected mixin's methods via the XML `mixin=`
//! attribute (or via a template's `mixin=` for `ExtraActionButton1`).
//! `ExtraActionBarFrame` is split off into its own absence test —
//! it has NO `mixin=` and the PLAN line is technically wrong for that
//! one frame.
//!
//! PLAN.md task: `StanceBar`, `PetActionBar`, `PossessActionBar`,
//! `ExtraActionBarFrame`, `ExtraActionButton1`,
//! `MainMenuBarVehicleLeaveButton` exist with expected mixins applied.
//!
//! Pulled out of `surface_frames.rs` because that file passed the
//! 750-line readability budget. Split is along the frames-vs-mixin-
//! shape aspect boundary.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBar";

/// PLAN-named frames that have a `mixin=` attribute in their XML
/// declaration. Each entry pairs the frame's own (xml_site, frame_name)
/// with the (mixin_name, mixin_lua_site, methods) that XML mixin
/// codegen at `src/loader/xml_frame_codegen.rs:155-173` expands into a
/// `Mixin(frame, MixinName)` call at frame-OnLoad time. The shared
/// `Mixin(object, ...)` impl at
/// `src/lua_api/env_init/shared_bootstrap.lua` walks `pairs(mixin)` and
/// does `object[k] = v` — so after addon load, `frame.method` IS the
/// same function reference as `MixinName.method` and `type(frame.method)
/// == "function"`. A nil reading on either side proves the codegen path
/// regressed (frame missed the Mixin call) or the mixin's source file
/// failed to execute past the `function MixinName:method(...)` line
/// (the mixin global ended up partial). Both are catastrophic — every
/// XML script handler that reads `self:Update()` etc. would nil-call.
struct MixinPin {
    frame_name: &'static str,
    frame_xml_site: &'static str,
    mixin_name: &'static str,
    mixin_lua_site: &'static str,
    methods: &'static [&'static str],
}

const PLAN_NAMED_MIXIN_FRAMES: &[MixinPin] = &[
    MixinPin {
        frame_name: "StanceBar",
        frame_xml_site: "Mainline/StanceBar.xml:12",
        mixin_name: "StanceBarMixin",
        mixin_lua_site: "Shared/StanceBar.lua",
        methods: &[
            "OnLoad",
            "OnEvent",
            "OnShow",
            "ShouldShow",
            "Update",
            "ShouldShowBackgroundArt",
            "SetBackgroundArtShown",
            "UpdateBackgroundArt",
            "UpdateState",
            "Select",
        ],
    },
    MixinPin {
        frame_name: "PetActionBar",
        frame_xml_site: "Mainline/PetActionBar.xml:33",
        mixin_name: "PetActionBarMixin",
        mixin_lua_site: "Shared/PetActionBar.lua",
        methods: &[
            "ClearPetActionHighlightMarks",
            "UpdatePetActionHighlightMarks",
            "OnHide",
            "OnLoad",
            "OnEvent",
            "OnUpdate",
            "Update",
            "UpdateCooldowns",
            "PetActionButtonDown",
            "PetActionButtonUp",
            "LockPetActionBar",
            "UnlockPetActionBar",
            "ShouldShowBackgroundArt",
            "SetBackgroundArtShown",
        ],
    },
    MixinPin {
        frame_name: "PossessActionBar",
        frame_xml_site: "Mainline/PossessActionBar.xml:13",
        mixin_name: "PossessActionBarMixin",
        mixin_lua_site: "Shared/PossessActionBar.lua",
        methods: &[
            "PossessActionBar_OnLoad",
            "Update",
            "UpdateState",
            "ShouldShowBackgroundArt",
            "SetBackgroundArtShown",
        ],
    },
    MixinPin {
        frame_name: "ExtraActionButton1",
        frame_xml_site: "Shared/ExtraActionBar.xml:116",
        mixin_name: "ExtraActionButtonMixin",
        mixin_lua_site: "Shared/ExtraActionBar.lua",
        methods: &["ExtraActionButton_OnLoad"],
    },
    MixinPin {
        frame_name: "MainMenuBarVehicleLeaveButton",
        frame_xml_site: "Shared/VehicleLeaveButton.xml:4",
        mixin_name: "MainMenuBarVehicleLeaveButtonMixin",
        mixin_lua_site: "Shared/VehicleLeaveButton.lua",
        methods: &[
            "OnLoad",
            "OnEnter",
            "OnEvent",
            "CanExitVehicle",
            "UpdateShownState",
            "Update",
            "OnClicked",
        ],
    },
];

/// Pin that each PLAN-named frame's expected mixin was applied: the
/// frame exists as a table global, the mixin global is a table, every
/// expected mixin method is present on BOTH the mixin table AND the
/// frame, and the frame's method IS the same function reference as the
/// mixin's. The double-pin (frame.method is function AND frame.method
/// == mixin.method) catches two distinct regressions: a missing
/// `Mixin(frame, MixinName)` codegen call (frame.method nil while
/// mixin.method present) and a mixin source-load failure (mixin.method
/// nil while frame.method might be a stale leftover).
///
/// Note: `ExtraActionButton1` uses an INHERITED mixin — its XML at
/// `Shared/ExtraActionBar.xml:116` says `inherits="ExtraActionButtonTemplate"`,
/// and the mixin attribute is on the template at
/// `Shared/ExtraActionBar.xml:3` (`mixin="ExtraActionButtonMixin"`).
/// The codegen comment at `src/loader/xml_frame_codegen.rs:157-159`
/// notes that template-inherited mixins are applied inside CreateFrame
/// via `apply_templates_from_registry → apply_single_template →
/// apply_mixin`, NOT by the per-frame `append_mixins_code` path that
/// handles direct `mixin=` attributes — so this entry exercises the
/// inherited-mixin code path while the four bar entries exercise the
/// direct-mixin path.
#[test]
fn plan_named_frames_have_their_mixins_applied() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for pin in PLAN_NAMED_MIXIN_FRAMES {
            let frame_type: String = env
                .eval(&format!("return type(_G[{:?}])", pin.frame_name))
                .expect("frame existence probe must run cleanly");

            assert_eq!(
                frame_type, "table",
                "Expected `_G[{:?}]` to be a table after `{ROOT}` loads, got `{frame_type}`. \
                 The XML at `{}` declares this frame and the named-frame registration runs \
                 at XML-load time. A nil reading means the XML did not execute or the frame \
                 failed to register.",
                pin.frame_name, pin.frame_xml_site
            );

            for method in pin.methods {
                let probe = format!(
                    "return type({}.{method}) == \"function\" and \
                            type(_G[{:?}].{method}) == \"function\" and \
                            (_G[{:?}].{method} == {}.{method})",
                    pin.mixin_name, pin.frame_name, pin.frame_name, pin.mixin_name
                );
                let ok: bool = env.eval(&probe).unwrap_or_else(|err| {
                    panic!(
                        "Mixin pin probe failed to evaluate for `{}.{method}`: {err}",
                        pin.frame_name
                    )
                });
                assert!(
                    ok,
                    "Expected `{frame_name}.{method}` to be the same function as \
                     `{mixin_name}.{method}` after `{ROOT}` loads. The XML at \
                     `{frame_xml_site}` declares this frame's mixin (directly via \
                     `mixin=\"{mixin_name}\"` for the bars and \
                     `MainMenuBarVehicleLeaveButton`, OR via the inherited \
                     `ExtraActionButtonTemplate` for `ExtraActionButton1` — see \
                     `Shared/ExtraActionBar.xml:3`). The XML codegen at \
                     `src/loader/xml_frame_codegen.rs:155-173` expands the attribute into \
                     `Mixin(frame, {mixin_name})`, and the shared `Mixin(object, ...)` \
                     impl in `src/lua_api/env_init/shared_bootstrap.lua` does \
                     `object[k] = v` for every key. The mixin source at `{mixin_lua_site}` \
                     declares `function {mixin_name}:{method}(...) end`. A failure on the \
                     `{mixin_name}.{method}` half means the source file failed to execute \
                     past that line. A failure on the `{frame_name}.{method}` half means \
                     either the codegen Mixin call did not run (registered handlers like \
                     XML script bindings would call `self:{method}()` and nil-call) or the \
                     frame's per-instance fenv table was rebuilt post-Mixin and dropped \
                     the copied keys.",
                    frame_name = pin.frame_name,
                    mixin_name = pin.mixin_name,
                    frame_xml_site = pin.frame_xml_site,
                    mixin_lua_site = pin.mixin_lua_site,
                );
            }
        }
    });
}

/// Pin that `ExtraActionBarFrame` exists with NO `mixin=` attribute —
/// the PLAN line says "with expected mixins applied" for all six frames
/// but `ExtraActionBarFrame` at `Shared/ExtraActionBar.xml:93` has no
/// `mixin=` attribute and no `ExtraActionBarFrameMixin` global is
/// defined anywhere in the addon. Its only behavioral wiring is an
/// `OnLoad` script handler that points at the FREE function
/// `ExtraActionBar_OnLoad` at `Shared/ExtraActionBar.lua:5` (NOT a
/// mixin method — there's no colon syntax, no method-receiver
/// argument). This is the inverse of the mixin-applied test for the
/// other five frames: the "expected mixin set" for this frame is empty,
/// and pinning that empty set guards against either (a) someone
/// "fixing" the inconsistency by sprinkling a mixin onto the XML
/// declaration (which would shadow the free-function OnLoad handler if
/// the mixin defined `OnLoad` too), OR (b) the simulator's codegen
/// silently injecting a mixin-style apply when none was declared.
#[test]
fn extra_action_bar_frame_publishes_no_mixin_only_a_script_handler() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let frame_type: String = env
            .eval("return type(_G.ExtraActionBarFrame)")
            .expect("ExtraActionBarFrame probe must run cleanly");

        assert_eq!(
            frame_type, "table",
            "Expected `_G.ExtraActionBarFrame` to be a table after `{ROOT}` loads, got \
             `{frame_type}`. The XML at `Shared/ExtraActionBar.xml:93` declares \
             `<Frame name=\"ExtraActionBarFrame\" frameStrata=\"LOW\" enableMouse=\"true\" \
             hidden=\"true\">` (no `parent=\"...\"` — a parented-by-Lua frame, attached to \
             `ExtraAbilityContainer` at runtime). A nil reading means the XML did not \
             execute or the frame failed to register its name."
        );

        let convention_named_mixin_type: String = env
            .eval("return type(_G.ExtraActionBarFrameMixin)")
            .expect("ExtraActionBarFrameMixin nil-probe must run cleanly");

        assert_eq!(
            convention_named_mixin_type, "nil",
            "Expected `_G.ExtraActionBarFrameMixin` to be nil — `ExtraActionBarFrame` at \
             `Shared/ExtraActionBar.xml:93` has NO `mixin=` attribute, and no \
             `ExtraActionBarFrameMixin` global is declared anywhere in the addon's \
             Mainline or Shared Lua sources. A non-nil reading proves either someone \
             added a convention-named mixin upstream (and the PLAN line for this task \
             needs revisiting to verify the new mixin's expected method set), or a \
             sibling addon re-published the name (a cross-addon collision worth \
             investigating). Got `{convention_named_mixin_type}`."
        );

        let on_load_handler_type: String = env
            .eval("return type(_G.ExtraActionBar_OnLoad)")
            .expect("ExtraActionBar_OnLoad probe must run cleanly");

        assert_eq!(
            on_load_handler_type, "function",
            "Expected `_G.ExtraActionBar_OnLoad` to be a function — declared at \
             `Shared/ExtraActionBar.lua:5` as a FREE function (no colon syntax, no \
             implicit `self`-receiver), wired to the frame via \
             `<OnLoad function=\"ExtraActionBar_OnLoad\"/>` at \
             `Shared/ExtraActionBar.xml:139`. This is the actual behavioral entry point \
             for the frame, taking the place of the mixin-method `OnLoad` pattern used \
             by the other five frames in this PLAN line. A nil or non-function reading \
             means the source file failed to execute past line 5 (every other \
             `ExtraActionBar_*` global at lua:10/32/38/51 would be missing too, since \
             they're declared sequentially in the same chunk). Got \
             `{on_load_handler_type}`."
        );
    });
}

const ACTION_BAR_MIXIN_LUA_SITE: &str = "Shared/ActionBar.lua:1";

/// PLAN-named methods that DO exist on `ActionBarMixin`. Source order
/// (lua:3/57/144/93/198): `ActionBar_OnLoad` and `ActionBar_OnEvent` are
/// the prefixed entry points the bar XML wires via
/// `<OnLoad function="ActionBar_OnLoad"/>`-style chains; `SetShowGrid`,
/// `UpdateGridLayout`, `UpdateShownButtons` are the three grid/visibility
/// helpers each bar's mixin calls into.
const ACTION_BAR_MIXIN_PLAN_NAMED_METHODS: &[&str] = &[
    "ActionBar_OnLoad",
    "ActionBar_OnEvent",
    "SetShowGrid",
    "UpdateGridLayout",
    "UpdateShownButtons",
];

/// Source-additional methods on `ActionBarMixin` that PLAN omits. Each
/// is a direct `function ActionBarMixin:METHOD(...)` declaration in
/// `Shared/ActionBar.lua`. PLAN-only would silently drop these from the
/// surface contract: `CacheGridSettings` (lua:65) and `ShouldUpdateGrid`
/// (lua:76) gate the grid-layout dirty path; `GetShowAllButtons`
/// (lua:173), `ShouldRaise` (lua:183), `UpdateFrameStrata` (lua:194)
/// drive bar strata raise/lower on hold-key drag; `UpdateSpellFlyoutDirection`
/// (lua:221) and `GetSpellFlyoutDirection` (lua:246) are the actual
/// spell-flyout direction methods (PLAN's `SetSpellFlyoutDirection` is
/// not the contract — see absent slice below).
const ACTION_BAR_MIXIN_SOURCE_ADDITIONAL_METHODS: &[&str] = &[
    "CacheGridSettings",
    "ShouldUpdateGrid",
    "GetShowAllButtons",
    "ShouldRaise",
    "UpdateFrameStrata",
    "UpdateSpellFlyoutDirection",
    "GetSpellFlyoutDirection",
];

/// PLAN-named methods that DO NOT exist on `ActionBarMixin` — negative
/// tripwires for spec drift. `SetSpellFlyoutDirection`: source has only
/// `Update`/`GetSpellFlyoutDirection` (lua:221/246) — there is no
/// setter. `Layout`: `ActionBarMixin` does not declare `Layout`; the
/// method exists on `ResizeLayoutMixin` (`Blizzard_SharedXML/LayoutFrame.lua:486`)
/// and reaches bar frames only through the `ResizeLayoutFrame` template
/// inheritance chain at `Shared/ActionBarTemplate.xml:7`. Pinning these
/// as nil ensures (a) the PLAN line is recognised as drifted from
/// source, and (b) a future "fix" that adds either method to
/// `ActionBarMixin` directly forces a spec/test review rather than
/// shadowing the inherited Layout silently.
const ACTION_BAR_MIXIN_PLAN_NAMED_ABSENT_METHODS: &[&str] = &["SetSpellFlyoutDirection", "Layout"];

/// Pin `ActionBarMixin`'s method-surface contract. **Spec/source
/// mismatch in both directions.** PLAN names 7 methods; source declares
/// 12 on the mixin (5 PLAN-named match + 7 source-additional), and 2
/// PLAN-named methods don't exist on the mixin at all. Single test
/// drives 15 assertions: existence (1), PLAN-named functions (5),
/// source-additional functions (7), PLAN-named-but-absent nil
/// tripwires (2). Mixin global is plain Lua — no Mixin() call needed
/// for these assertions; `_G.ActionBarMixin.method` is the source-level
/// declaration directly. The `frame.method == mixin.method` reference
/// pin in `plan_named_frames_have_their_mixins_applied` covers
/// per-frame Mixin-codegen drift; this test covers source-load drift.
#[test]
fn action_bar_mixin_publishes_plan_named_and_source_additional_methods() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let mixin_type: String = env
            .eval("return type(_G.ActionBarMixin)")
            .expect("ActionBarMixin global probe must run cleanly");

        assert_eq!(
            mixin_type, "table",
            "Expected `_G.ActionBarMixin` to be a table after `{ROOT}` loads, got \
             `{mixin_type}`. Source declares it at `{ACTION_BAR_MIXIN_LUA_SITE}` \
             (`ActionBarMixin = {{}}`). Nil reading: source file failed to load before \
             line 1, or the global was overwritten by a later addon."
        );

        for method in ACTION_BAR_MIXIN_PLAN_NAMED_METHODS {
            let method_type: String = env
                .eval(&format!("return type(_G.ActionBarMixin.{method})"))
                .expect("ActionBarMixin method probe must run cleanly");

            assert_eq!(
                method_type, "function",
                "Expected `ActionBarMixin.{method}` to be a function after `{ROOT}` \
                 loads, got `{method_type}`. PLAN names this method; source declares \
                 `function ActionBarMixin:{method}(...)` in `Shared/ActionBar.lua`. \
                 False reading: source file failed to execute past the declaration, or \
                 the method was renamed/removed. Each per-bar mixin's OnLoad calls into \
                 these via `self:ActionBar_OnLoad()`-style invocations, so a nil \
                 reading would nil-call at frame-OnLoad time."
            );
        }

        for method in ACTION_BAR_MIXIN_SOURCE_ADDITIONAL_METHODS {
            let method_type: String = env
                .eval(&format!("return type(_G.ActionBarMixin.{method})"))
                .expect("ActionBarMixin method probe must run cleanly");

            assert_eq!(
                method_type, "function",
                "Expected `ActionBarMixin.{method}` to be a function after `{ROOT}` \
                 loads, got `{method_type}`. PLAN omits this method, but source \
                 declares it as a direct `function ActionBarMixin:{method}(...)` in \
                 `Shared/ActionBar.lua`. Pinned as a tripwire so the spec recognises \
                 source drift if the method is removed: the grid-cache path \
                 (`CacheGridSettings`/`ShouldUpdateGrid`), strata raise/lower path \
                 (`GetShowAllButtons`/`ShouldRaise`/`UpdateFrameStrata`), and \
                 spell-flyout direction path (`UpdateSpellFlyoutDirection`/\
                 `GetSpellFlyoutDirection`) all depend on these declarations."
            );
        }

        for method in ACTION_BAR_MIXIN_PLAN_NAMED_ABSENT_METHODS {
            let method_type: String = env
                .eval(&format!("return type(_G.ActionBarMixin.{method})"))
                .expect("ActionBarMixin absent-method probe must run cleanly");

            assert_eq!(
                method_type, "nil",
                "Expected `ActionBarMixin.{method}` to be nil after `{ROOT}` loads, \
                 got `{method_type}`. PLAN names this method but source does NOT \
                 declare it on `ActionBarMixin` (`Shared/ActionBar.lua` has \
                 `UpdateSpellFlyoutDirection` lua:221 and `GetSpellFlyoutDirection` \
                 lua:246 — no `SetSpellFlyoutDirection`; and `Layout` lives on \
                 `ResizeLayoutMixin` at `Blizzard_SharedXML/LayoutFrame.lua:486`, \
                 reaching bar frames only via the `ResizeLayoutFrame` template \
                 inheritance at `Shared/ActionBarTemplate.xml:7`). Non-nil reading: \
                 source added the method on `ActionBarMixin` directly — the spec \
                 needs review (a directly-declared `Layout` on `ActionBarMixin` \
                 would shadow the inherited `ResizeLayoutMixin:Layout` and silently \
                 change layout-pass behavior across every bar)."
            );
        }
    });
}

const ACTION_BUTTON_LUA_SITE: &str = "Shared/ActionButton.lua";

/// PLAN names 10 methods as living on `ActionBarButtonMixin /
/// BaseActionButtonMixin`. Source disagrees on 9: only `UpdateButtonArt`
/// is on `BaseActionButtonMixin` (lua:1546 stub + `Mainline/ActionButtonOverrides.lua:2`
/// real impl). The remaining 9 (`OnLoad`, `OnEvent`, `OnEnter`,
/// `OnLeave`, `UpdateUsable`, `UpdateState`, `UpdateAction`,
/// `SetTooltip`, `MatchesActiveButtonSpellID`) all live on the sibling
/// `ActionBarActionButtonMixin` (declared lua:442) — a third mixin PLAN
/// does not name. The plain script handlers (`OnLoad`, `OnEnter`,
/// `OnLeave`) on the named mixins use prefixed variants
/// (`BaseActionButtonMixin_OnLoad` lua:1502, `ActionBarButtonMixin_OnLoad`
/// lua:1605, etc.) so the chain `ActionBarButtonTemplate ->
/// ActionButtonTemplate` (`Mainline/ActionButtonTemplate.xml:189` ->
/// `xml:4`) can compose Mixin OnLoads without name collision.
const PLAN_NAMED_BUTTON_METHODS: &[&str] = &[
    "OnLoad",
    "OnEvent",
    "OnEnter",
    "OnLeave",
    "UpdateUsable",
    "UpdateState",
    "UpdateAction",
    "SetTooltip",
    "MatchesActiveButtonSpellID",
    "UpdateButtonArt",
];

/// The single PLAN-named method that IS on `BaseActionButtonMixin`.
/// Stub at lua:1546, overridden at `Mainline/ActionButtonOverrides.lua:2`
/// — the real Mainline body shows/hides `SlotArt`/`SlotBackground` and
/// switches the normal+pushed atlases between `UI-HUD-ActionBar-IconFrame*`
/// and `UI-HUD-ActionBar-IconFrame-AddRow*` based on `self.bar.hideBarArt`.
const BASE_ACTION_BUTTON_PRESENT_PLAN_METHOD: &str = "UpdateButtonArt";

/// Sample of source-additional methods on `BaseActionButtonMixin` that
/// PLAN omits — pinned so a regression that drops the chain entry
/// points (`BaseActionButtonMixin_OnLoad` etc.) or the grid-attribute
/// helpers (`GetShowGrid`/`SetShowGrid`/`UpdateFlyout`) surfaces with a
/// clear cause.
const BASE_ACTION_BUTTON_SOURCE_ADDITIONAL_METHODS: &[&str] = &[
    "BaseActionButtonMixin_OnLoad",
    "BaseActionButtonMixin_OnEnter",
    "BaseActionButtonMixin_OnLeave",
    "BaseActionButtonMixin_OnDragStart",
    "BaseActionButtonMixin_OnAttributeChanged",
    "GetShowGrid",
    "SetShowGrid",
    "UpdateFlyout",
];

/// Source-additional methods on `ActionBarButtonMixin` (declared
/// lua:1603). All four are prefixed-name forwarders that delegate to
/// both `BaseActionButtonMixin` and `ActionBarActionButtonDerivedMixin`
/// (lua:1606-1607 etc.) — they exist precisely so the
/// `ActionBarButtonTemplate` XML can compose two parent mixins without
/// name collision on `OnLoad`/`OnEnter`/`OnLeave`/`OnDragStart`.
const ACTION_BAR_BUTTON_SOURCE_ADDITIONAL_METHODS: &[&str] = &[
    "ActionBarButtonMixin_OnLoad",
    "ActionBarButtonMixin_OnEnter",
    "ActionBarButtonMixin_OnLeave",
    "ActionBarButtonMixin_OnDragStart",
];

/// Pin `ActionBarButtonMixin` and `BaseActionButtonMixin` method
/// surfaces. **Spec/source mismatch — PLAN names 10 methods, but only
/// 1 (`UpdateButtonArt`) actually lives on the named mixins.** The
/// other 9 live on the sibling `ActionBarActionButtonMixin` (declared
/// `Shared/ActionButton.lua:442`) — a mixin PLAN does NOT name and
/// reaches frames via `ActionBarActionButtonDerivedMixin = CreateFromMixins(...)`
/// at lua:1444 + a function-call apply at lua:1607. Test pins 30
/// assertions: 2 mixin-table existence + 1 PLAN-named PRESENT on
/// `BaseActionButtonMixin` (`UpdateButtonArt`) + 9 PLAN-named ABSENT on
/// `BaseActionButtonMixin` + 10 PLAN-named ABSENT on
/// `ActionBarButtonMixin` + 8 source-additional functions on
/// `BaseActionButtonMixin` (`BaseActionButtonMixin_*` chain entries +
/// `GetShowGrid`/`SetShowGrid`/`UpdateFlyout`).
#[test]
fn action_bar_button_and_base_action_button_mixins_pin_plan_named_and_source_additional_methods() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for mixin in ["BaseActionButtonMixin", "ActionBarButtonMixin"] {
            let mixin_type: String = env
                .eval(&format!("return type(_G.{mixin})"))
                .expect("mixin global probe must run cleanly");

            assert_eq!(
                mixin_type, "table",
                "Expected `_G.{mixin}` to be a table after `{ROOT}` loads, got \
                 `{mixin_type}`. Source declares both at `{ACTION_BUTTON_LUA_SITE}` \
                 (`BaseActionButtonMixin = {{}}` lua:1500, `ActionBarButtonMixin = {{}};` \
                 lua:1603). Nil reading: source file failed to load, or one global was \
                 overwritten by a later addon."
            );
        }

        let present_method_type: String = env
            .eval(&format!(
                "return type(_G.BaseActionButtonMixin.{BASE_ACTION_BUTTON_PRESENT_PLAN_METHOD})"
            ))
            .expect("BaseActionButtonMixin.UpdateButtonArt probe must run cleanly");

        assert_eq!(
            present_method_type, "function",
            "Expected `BaseActionButtonMixin.{BASE_ACTION_BUTTON_PRESENT_PLAN_METHOD}` to be \
             a function — declared as a stub at `{ACTION_BUTTON_LUA_SITE}:1546` and \
             overridden by `Mainline/ActionButtonOverrides.lua:2` (the real Mainline body \
             switches `UI-HUD-ActionBar-IconFrame*` vs \
             `UI-HUD-ActionBar-IconFrame-AddRow*` atlases on `SlotArt`/`SlotBackground`). \
             False reading: stub failed to load (every action button's `:UpdateButtonArt()` \
             call at `BaseActionButtonMixin_OnLoad` lua:1505 nil-calls), or the override \
             failed and removed the stub."
        );

        for method in PLAN_NAMED_BUTTON_METHODS {
            if *method == BASE_ACTION_BUTTON_PRESENT_PLAN_METHOD {
                continue;
            }

            let method_type: String = env
                .eval(&format!("return type(_G.BaseActionButtonMixin.{method})"))
                .expect("BaseActionButtonMixin absent-method probe must run cleanly");

            assert_eq!(
                method_type, "nil",
                "Expected `BaseActionButtonMixin.{method}` to be nil after `{ROOT}` \
                 loads, got `{method_type}`. PLAN names this method, but source declares \
                 it on the sibling `ActionBarActionButtonMixin` (lua:442) — NOT on \
                 `BaseActionButtonMixin`: OnLoad lua:444, UpdateAction lua:529, \
                 UpdateState lua:673, UpdateUsable lua:679, MatchesActiveButtonSpellID \
                 lua:944, OnEvent lua:966, SetTooltip lua:1101, OnEnter lua:1419, OnLeave \
                 lua:1432. Plain `OnLoad`/`OnEnter`/`OnLeave` on the named mixins use \
                 prefixed variants (`BaseActionButtonMixin_OnLoad` lua:1502 etc.) so the \
                 `ActionBarButtonTemplate -> ActionButtonTemplate` chain at \
                 `Mainline/ActionButtonTemplate.xml:189`->xml:4 composes Mixins without \
                 name collision. Non-nil reading: source moved the method onto \
                 `BaseActionButtonMixin`, which would shadow the sibling's contract — \
                 spec needs review."
            );
        }

        for method in PLAN_NAMED_BUTTON_METHODS {
            let method_type: String = env
                .eval(&format!("return type(_G.ActionBarButtonMixin.{method})"))
                .expect("ActionBarButtonMixin absent-method probe must run cleanly");

            assert_eq!(
                method_type, "nil",
                "Expected `ActionBarButtonMixin.{method}` to be nil after `{ROOT}` loads, \
                 got `{method_type}`. PLAN names this method, but `ActionBarButtonMixin` \
                 (lua:1603) only declares 4 prefixed forwarders \
                 (`ActionBarButtonMixin_OnLoad`/`OnEnter`/`OnLeave`/`OnDragStart` at \
                 lua:1605/1610/1615/1620), each delegating to BOTH `BaseActionButtonMixin` \
                 (the corresponding `BaseActionButtonMixin_*` entry) and \
                 `ActionBarActionButtonDerivedMixin` (the derived sibling at lua:1444 \
                 created from `ActionBarActionButtonMixin`). The 10 PLAN-named methods \
                 reach frames through `ActionBarActionButtonDerivedMixin`, NOT \
                 `ActionBarButtonMixin` directly. Non-nil reading: source added the \
                 method on `ActionBarButtonMixin` directly — spec needs review."
            );
        }

        for method in BASE_ACTION_BUTTON_SOURCE_ADDITIONAL_METHODS {
            let method_type: String = env
                .eval(&format!("return type(_G.BaseActionButtonMixin.{method})"))
                .expect("BaseActionButtonMixin source-additional probe must run cleanly");

            assert_eq!(
                method_type, "function",
                "Expected `BaseActionButtonMixin.{method}` to be a function after \
                 `{ROOT}` loads, got `{method_type}`. PLAN omits this method, but source \
                 declares it as a direct `function BaseActionButtonMixin:{method}(...)` \
                 in `{ACTION_BUTTON_LUA_SITE}` between lua:1502 and 1551. Pinned as a \
                 tripwire: the prefixed chain entries (`BaseActionButtonMixin_OnLoad` \
                 etc.) are the actual XML-script targets via the prefixed forwarders on \
                 the sibling `ActionBarButtonMixin`; dropping any breaks Mixin chain \
                 composition. The grid-attribute helpers \
                 (`GetShowGrid`/`SetShowGrid`/`UpdateFlyout`) are referenced by \
                 `ActionBarMixin:ActionBar_OnLoad` (`Shared/ActionBar.lua:46-54`) and \
                 the `BaseActionButtonMixin_OnLoad` body (lua:1505-1506)."
            );
        }

        for method in ACTION_BAR_BUTTON_SOURCE_ADDITIONAL_METHODS {
            let method_type: String = env
                .eval(&format!("return type(_G.ActionBarButtonMixin.{method})"))
                .expect("ActionBarButtonMixin source-additional probe must run cleanly");

            assert_eq!(
                method_type, "function",
                "Expected `ActionBarButtonMixin.{method}` to be a function after \
                 `{ROOT}` loads, got `{method_type}`. Source declares the prefixed \
                 forwarder in `{ACTION_BUTTON_LUA_SITE}` between lua:1605 and 1620; \
                 each delegates to BOTH `BaseActionButtonMixin.BaseActionButtonMixin_*` \
                 and `ActionBarActionButtonDerivedMixin.ActionBarActionButtonDerivedMixin_*` \
                 (lua:1606-1607 etc.). Without these forwarders, the \
                 `ActionBarButtonTemplate` script handlers fail to compose the two \
                 parent Mixin chains."
            );
        }
    });
}

/// One per-mixin row. PLAN says "are tables with the mixin methods
/// documented in the analyzer inventory" — the contract is the FULL
/// method set per mixin, not a subset.
struct MixinInventory {
    name: &'static str,
    lua_site: &'static str,
    methods: &'static [&'static str],
}

const MIXIN_INVENTORY: &[MixinInventory] = &[
    MixinInventory {
        name: "StanceBarMixin",
        lua_site: "Shared/StanceBar.lua:4",
        methods: &[
            "OnLoad",
            "OnEvent",
            "OnShow",
            "ShouldShow",
            "Update",
            "ShouldShowBackgroundArt",
            "SetBackgroundArtShown",
            "UpdateBackgroundArt",
            "UpdateState",
            "Select",
        ],
    },
    MixinInventory {
        name: "StanceButtonMixin",
        lua_site: "Shared/StanceBar.lua:107",
        methods: &[
            "StanceButtonMixin_OnLoad",
            "StanceButtonMixin_OnClick",
            "StanceButtonMixin_OnEnter",
            "StanceButtonMixin_OnLeave",
            "HasAction",
        ],
    },
    MixinInventory {
        name: "PetActionBarMixin",
        lua_site: "Shared/PetActionBar.lua:16",
        methods: &[
            "ClearPetActionHighlightMarks",
            "UpdatePetActionHighlightMarks",
            "OnHide",
            "OnLoad",
            "OnEvent",
            "OnUpdate",
            "Update",
            "UpdateCooldowns",
            "PetActionButtonDown",
            "PetActionButtonUp",
            "LockPetActionBar",
            "UnlockPetActionBar",
            "ShouldShowBackgroundArt",
            "SetBackgroundArtShown",
        ],
    },
    MixinInventory {
        name: "PossessActionBarMixin",
        lua_site: "Shared/PossessActionBar.lua:4",
        methods: &[
            "PossessActionBar_OnLoad",
            "Update",
            "UpdateState",
            "ShouldShowBackgroundArt",
            "SetBackgroundArtShown",
        ],
    },
    MixinInventory {
        name: "PossessButtonMixin",
        lua_site: "Shared/PossessActionBar.lua:65",
        methods: &["OnLoad", "OnClick", "OnEnter", "OnLeave", "HasAction"],
    },
    MixinInventory {
        name: "ExtraActionButtonMixin",
        lua_site: "Shared/ExtraActionBar.lua:82",
        methods: &["ExtraActionButton_OnLoad"],
    },
    MixinInventory {
        name: "MainActionBarMixin",
        lua_site: "Shared/MainActionBar.lua:3",
        methods: &[
            "OnLoad",
            "OnShow",
            "OnHide",
            "SetYOffset",
            "GetYOffset",
            "OnEvent",
            "AttachToFrame",
            "DetachFromFrame",
            "IsInDefaultPosition",
            "SetQuickKeybindModeEffectsShown",
            "UpdateEndCaps",
            "EditModeSetScale",
            "UpdateDividers",
            "GetEndCapsFrameLevel",
        ],
    },
];

/// Pin the analyzer-inventory method surface for the seven bar/button
/// mixins PLAN names. 61 assertions = 7 mixin-table existence + 54
/// methods. `MainActionBarMixin:UpdateEndCaps` is a Shared stub
/// (`Shared/MainActionBar.lua:72`) overridden by
/// `Mainline/MainActionBarOverrides.lua:2` — both are valid `function`
/// readings. `StanceButtonMixin` uses prefixed forwarders
/// (`StanceButtonMixin_OnLoad` etc.); `PossessButtonMixin` uses plain
/// handler names — both pinned verbatim.
#[test]
fn analyzer_inventory_mixins_publish_their_documented_methods() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for inventory in MIXIN_INVENTORY {
            let mixin_type: String = env
                .eval(&format!("return type(_G.{})", inventory.name))
                .expect("mixin global probe must run cleanly");

            assert_eq!(
                mixin_type, "table",
                "Expected `_G.{}` to be a table after `{ROOT}` loads (declared at `{}`), \
                 got `{mixin_type}`. Every method assertion below depends on it.",
                inventory.name, inventory.lua_site
            );

            for method in inventory.methods {
                let method_type: String = env
                    .eval(&format!("return type(_G.{}.{method})", inventory.name))
                    .expect("mixin method probe must run cleanly");

                assert_eq!(
                    method_type,
                    "function",
                    "Expected `{mixin}.{method}` to be a function after `{ROOT}` loads \
                     (declared at `{lua_site}`), got `{method_type}`. PLAN names the \
                     analyzer inventory as the contract — full per-mixin method set, \
                     not a PLAN subset.",
                    mixin = inventory.name,
                    lua_site = inventory.lua_site,
                );
            }
        }
    });
}
