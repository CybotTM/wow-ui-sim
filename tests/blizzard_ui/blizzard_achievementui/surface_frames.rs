//! Frame-shape surface pins for the `Blizzard_AchievementUI` lane.
//!
//! PLAN.md task: pin that `AchievementFrame` exists, has `frameStrata` of
//! `MEDIUM`, has parent `UIParent`, and is hidden by default. All four
//! facts come from a single XML declaration at
//! `Mainline/Blizzard_AchievementUI.xml:1505`:
//!
//! ```xml
//! <Frame name="AchievementFrame" toplevel="true" parent="UIParent"
//!        frameStrata="MEDIUM" hidden="true" enableMouse="true"
//!        inherits="BackdropTemplate">
//! ```
//!
//! Each fact has its own assertion so a regression touches the smallest
//! possible test surface. The four together pin the panel's identity in
//! the WoW window manager:
//!
//! - **Existence as a global table.** XML `name="AchievementFrame"`
//!   registers the frame in `_G` at XML-load time. Without this the
//!   `TOGGLEACHIEVEMENT` keybind handler at `Blizzard_AchievementUI.lua:195`
//!   (`AchievementFrame_ToggleAchievementFrame` — pinned in
//!   `surface_globals.rs`) would surface a nil-table-method error on
//!   `ShowUIPanel(AchievementFrame)`.
//!
//! - **`frameStrata == "MEDIUM"`.** This is the default UIPanel stratum.
//!   The achievement panel deliberately renders at the same level as
//!   character / spellbook / inventory frames so the standard UIPanel
//!   layout system (`UIPanelWindows["AchievementFrame"]`, registered at
//!   `Blizzard_AchievementUI.lua:151`) can manage its anchor and the
//!   panel-stack push/pop without crossing strata boundaries. A regression
//!   to `LOW` would push it below world chrome; a regression to `HIGH`
//!   would let it cover dialogs.
//!
//! - **`parent == "UIParent"`.** The standard UI root. `parent="UIParent"`
//!   on the XML keeps the frame inside the user-scaled UI (UIParent is
//!   what `SetUIScale` and the resolution-aware reparenting drive against),
//!   not the world frame which renders 3D content. A regression that
//!   reparents this onto `WorldFrame` or some intermediate would break
//!   user-set UI scaling and detach the panel from `UIParent.Hide()`-style
//!   global toggles.
//!
//! - **Hidden by default.** XML `hidden="true"` makes the frame hidden
//!   on creation; `ToggleAchievementFrame` flips it visible via
//!   `ShowUIPanel`. A regression dropping `hidden="true"` would put the
//!   panel on screen at game start, blocking the player's view.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const FRAME_NAME: &str = "AchievementFrame";
const XML_SITE: &str = "Mainline/Blizzard_AchievementUI.xml:1505";

/// PLAN-named children of `AchievementFrame`. Each tuple is
/// (key, xml_site, routing_kind) where `routing_kind` is "parentKey" when the
/// XML attaches the child via `parentKey="<Key>"` and "name-prefix" when the
/// XML attaches via `name="$parent<Key>"` and the simulator's
/// `infer_parent_key_from_child_name` repair derives the key by stripping the
/// `AchievementFrame` prefix from the child's resolved name.
///
/// Both routes land in the same place — `AchievementFrame.<Key>` is the child
/// frame — so callers don't see a difference, but a regression in either path
/// would surface here:
///
/// - `parentKey="..."` route: handled by `append_parent_key_code` in
///   `src/loader/xml_frame_codegen.rs:90`. A regression dropping the
///   `parentKey=` attribute or a loader bug skipping the assignment would null
///   one of the four explicit-key entries below.
///
/// - `name="$parent<Key>"` route: handled by
///   `infer_parent_key_from_child_name` in
///   `src/lua_api/globals/template/mod.rs:72`. A regression that removed the
///   prefix-inference repair, or that changed the `name="$parentX"` token,
///   would null one of the four name-inferred entries below.
const PLAN_NAMED_CHILDREN: &[(&str, &str, &str)] = &[
    (
        "Header",
        "Mainline/Blizzard_AchievementUI.xml:1660",
        "parentKey",
    ),
    (
        "Categories",
        "Mainline/Blizzard_AchievementUI.xml:1729",
        "parentKey",
    ),
    (
        "FilterDropdown",
        "Mainline/Blizzard_AchievementUI.xml:2348",
        "parentKey",
    ),
    (
        "SearchBox",
        "Mainline/Blizzard_AchievementUI.xml:2356",
        "parentKey",
    ),
    (
        "Achievements",
        "Mainline/Blizzard_AchievementUI.xml:1755",
        "name-prefix",
    ),
    (
        "Stats",
        "Mainline/Blizzard_AchievementUI.xml:1816",
        "name-prefix",
    ),
    (
        "Summary",
        "Mainline/Blizzard_AchievementUI.xml:1866",
        "name-prefix",
    ),
    (
        "Comparison",
        "Mainline/Blizzard_AchievementUI.xml:2080",
        "name-prefix",
    ),
];

#[test]
fn achievement_frame_publishes_expected_panel_identity() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let frame_type: String = env
            .eval(&format!("return type(_G[{FRAME_NAME:?}])"))
            .expect("AchievementFrame global probe must run cleanly");

        assert_eq!(
            frame_type, "table",
            "Expected `_G[{FRAME_NAME:?}]` to be a table after `{ROOT}` loads, got \
             `{frame_type}`. The frame is declared at `{XML_SITE}` with \
             `name=\"AchievementFrame\"` and `parent=\"UIParent\"`, so the named-frame \
             registration runs at XML load time. A nil reading means either the XML did not \
             execute (a regression in the load pipeline) or the frame failed to register its \
             name (a regression in the named-frame routing inside `CreateFrame`). Either way, \
             every downstream consumer that reaches `AchievementFrame.X` would surface a \
             nil-table-index error — including the keybind handler \
             `AchievementFrame_ToggleAchievementFrame` (`Blizzard_AchievementUI.lua:195`) \
             which calls `ShowUIPanel(AchievementFrame)` / `HideUIPanel(AchievementFrame)`."
        );

        let frame_strata: String = env
            .eval(&format!("return _G[{FRAME_NAME:?}]:GetFrameStrata()"))
            .expect("`GetFrameStrata` must run cleanly on AchievementFrame");

        assert_eq!(
            frame_strata, "MEDIUM",
            "Expected `AchievementFrame:GetFrameStrata()` to return `MEDIUM` after `{ROOT}` \
             loads, got `{frame_strata}`. The XML at `{XML_SITE}` declares \
             `frameStrata=\"MEDIUM\"` literally. MEDIUM is the default UIPanel stratum so the \
             achievement panel renders at the same level as character / spellbook / inventory \
             frames — the standard UIPanel layout system manages its anchor and the \
             panel-stack push/pop without crossing strata boundaries. A regression to `LOW` \
             would push it below world chrome; a regression to `HIGH` would let it cover \
             dialogs / tooltip text from other panels."
        );

        let parent_name: String = env
            .eval(&format!("return _G[{FRAME_NAME:?}]:GetParent():GetName()"))
            .expect("`GetParent():GetName()` must run cleanly on AchievementFrame");

        assert_eq!(
            parent_name, "UIParent",
            "Expected `AchievementFrame:GetParent():GetName()` to return `UIParent` after \
             `{ROOT}` loads, got `{parent_name}`. The XML at `{XML_SITE}` declares \
             `parent=\"UIParent\"` literally. `UIParent` is the standard scaled-UI root — \
             `SetUIScale` and the resolution-aware reparenting drive against it, and \
             `UIParent.Hide()`-style global toggles cascade to it. A regression that \
             reparents this onto `WorldFrame` (the 3D world root) or some intermediate would \
             break user-set UI scaling and detach the panel from the global UI toggle."
        );

        let is_shown: bool = env
            .eval(&format!("return _G[{FRAME_NAME:?}]:IsShown()"))
            .expect("`IsShown` must run cleanly on AchievementFrame");

        assert!(
            !is_shown,
            "Expected `AchievementFrame:IsShown()` to return false after `{ROOT}` loads. The \
             XML at `{XML_SITE}` declares `hidden=\"true\"` literally — the frame is hidden on \
             creation, and `ToggleAchievementFrame` flips it visible via `ShowUIPanel` only \
             when the player presses the achievement keybind or clicks the micro menu button. \
             A true reading here means a regression dropped `hidden=\"true\"` from the XML or \
             the loader failed to honour the attribute, putting the panel on screen at game \
             start and blocking the player's view."
        );
    });
}

/// Pin every PLAN-named child as a parent property on `AchievementFrame`.
///
/// PLAN names eight children: `Achievements`, `Stats`, `Summary`,
/// `Categories`, `Comparison`, `SearchBox`, `FilterDropdown`, `Header`. The
/// XML installs them via two distinct routes (see `PLAN_NAMED_CHILDREN`
/// docs above): four use `parentKey="<Key>"` directly (`Header`,
/// `Categories`, `FilterDropdown`, `SearchBox`), and four use
/// `name="$parent<Key>"` with the prefix-inference repair filling in the
/// parent property (`Achievements`, `Stats`, `Summary`, `Comparison`).
/// Both routes converge on the same observable contract: every
/// PLAN-named child is reachable as `AchievementFrame.<Key>`, is a Frame,
/// and has `AchievementFrame` as its parent.
///
/// The single test asserts that contract for all eight children
/// uniformly. Each assertion message names the routing kind and the XML
/// site so a regression points to the right code path:
///
/// - A nil reading on a `parentKey`-routed child means either the XML
///   dropped the `parentKey="<Key>"` attribute (the panel's Lua code
///   that uses `AchievementFrame.Header.Points:SetText(...)` or
///   `AchievementFrame.SearchBox:HasFocus()` would surface a
///   nil-table-index error) or the loader's `append_parent_key_code`
///   stopped routing `parentKey=` into the parent's per-instance table.
///
/// - A nil reading on a `name-prefix`-routed child means either the XML
///   changed the `name="$parent<Key>"` token (so the resolved child
///   name no longer has the parent's name as a prefix) or the
///   simulator's `infer_parent_key_from_child_name` repair stopped
///   running. Either way every consumer that walks
///   `AchievementFrame.Achievements`, `.Stats`, `.Summary`, or
///   `.Comparison` (notably the panel-stack push/pop code that toggles
///   their visibility) would silently miss the child.
///
/// - A child whose `:GetParent():GetName()` is anything other than
///   `AchievementFrame` means the child was reparented post-load — none
///   of the panel's Lua code does that, so it would be a regression
///   that breaks `AchievementFrame:Hide()` cascading to children and
///   `SetUIScale` propagating from UIParent.
#[test]
fn achievement_frame_plan_named_children_are_reachable_as_parent_properties() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for (key, xml_site, routing_kind) in PLAN_NAMED_CHILDREN {
            let value_type: String = env
                .eval(&format!(
                    "return type(_G[{FRAME_NAME:?}][{key:?}])",
                    key = key
                ))
                .expect("`type(AchievementFrame[<Key>])` must run cleanly");

            assert_eq!(
                value_type, "table",
                "Expected `AchievementFrame.{key}` to be a table after `{ROOT}` loads, got \
                 `{value_type}`. The XML at `{xml_site}` attaches this child via the \
                 `{routing_kind}` route. A nil reading on a `parentKey` child means the \
                 XML dropped the attribute or `append_parent_key_code` stopped routing it; \
                 a nil reading on a `name-prefix` child means the XML changed the \
                 `name=\"$parent{key}\"` token or `infer_parent_key_from_child_name` \
                 stopped running. Either way the panel's Lua code that reaches \
                 `AchievementFrame.{key}` would surface a nil-table-index error."
            );

            let parent_name: String = env
                .eval(&format!(
                    "return _G[{FRAME_NAME:?}][{key:?}]:GetParent():GetName()"
                ))
                .expect("`GetParent():GetName()` must run cleanly on the child frame");

            assert_eq!(
                parent_name, FRAME_NAME,
                "Expected `AchievementFrame.{key}:GetParent():GetName()` to return \
                 `{FRAME_NAME}`, got `{parent_name}`. The XML at `{xml_site}` nests this \
                 child inside `AchievementFrame`'s `<Frames>` block, so its parent must \
                 be `AchievementFrame` — `AchievementFrame:Hide()` cascading to children \
                 and `SetUIScale` propagating from `UIParent` both depend on the parent \
                 chain landing here."
            );
        }
    });
}

const SEARCH_PROGRESS_BAR_KEY: &str = "searchProgressBar";
const SEARCH_PROGRESS_BAR_XML_SITE: &str = "Mainline/Blizzard_AchievementUI.xml:2501";
const SEARCH_PROGRESS_BAR_PLAN_NAME: &str = "AchievementFrameSearchProgressBar";
const SEARCH_PROGRESS_BAR_HANDLER: &str = "AchievementFrameSearchProgressBar_OnUpdate";

const COMPARISON_FRAME_NAME: &str = "AchievementFrameComparison";
const COMPARISON_XML_SITE: &str = "Mainline/Blizzard_AchievementUI.xml:2080";

/// PLAN-named comparison-subtree paths. Each tuple is `(lua_path, xml_site,
/// routing_kind)` where `lua_path` is the dotted access from
/// `AchievementFrameComparison`, `xml_site` is the source declaration, and
/// `routing_kind` is `"parentKey"` when the XML uses an explicit
/// `parentKey="<Key>"` attribute and `"name-prefix"` when the XML uses
/// `name="$parent<Key>"` and the simulator's
/// `infer_parent_key_from_child_name` repair (`src/lua_api/globals/template/mod.rs:72`)
/// derives the key by stripping the parent's resolved name.
///
/// `Header.Portrait` is a two-step path: `Header` is a name-prefix child
/// of `AchievementFrameComparison` (XML `name="$parentHeader"`, resolved
/// name `AchievementFrameComparisonHeader`, prefix-strip yields
/// `Header`), and `Portrait` is a Texture inside the Header's
/// `<Layer level="BACKGROUND">` block (XML `name="$parentPortrait"`,
/// resolved name `AchievementFrameComparisonHeaderPortrait`, prefix-strip
/// yields `Portrait`).
const PLAN_NAMED_COMPARISON_PATHS: &[(&str, &str, &str)] = &[
    (
        "Header",
        "Mainline/Blizzard_AchievementUI.xml:2087",
        "name-prefix",
    ),
    (
        "Header.Portrait",
        "Mainline/Blizzard_AchievementUI.xml:2103",
        "name-prefix",
    ),
    (
        "Summary",
        "Mainline/Blizzard_AchievementUI.xml:2149",
        "parentKey",
    ),
    (
        "AchievementContainer",
        "Mainline/Blizzard_AchievementUI.xml:2212",
        "parentKey",
    ),
];

/// Pin `AchievementFrameComparison` and its PLAN-named subtree paths.
///
/// PLAN names three children of the comparison frame: `Header.Portrait`
/// (a two-step path through `Header`), `Summary`, and
/// `AchievementContainer`. The XML at
/// `Mainline/Blizzard_AchievementUI.xml:2080` declares the comparison
/// frame itself as `<Frame name="$parentComparison">`, reachable in the
/// simulator as both `_G.AchievementFrameComparison` and
/// `AchievementFrame.Comparison` — the latter via the same name-prefix
/// inference that handles `AchievementFrame.Achievements` /
/// `AchievementFrame.Stats` / `AchievementFrame.Summary` (pinned by the
/// sibling `achievement_frame_plan_named_children_are_reachable_as_parent_properties`
/// test).
///
/// Subtree routing kinds:
///
/// - `Header` (line 2087) and its `Portrait` texture (line 2103) are both
///   declared via `name="$parent<Key>"` without explicit `parentKey=`
///   attributes. They land on the parent's per-instance table only
///   because the simulator's `infer_parent_key_from_child_name` repair
///   strips the parent's resolved name from each child's resolved name
///   and installs the resulting suffix as the parentKey. A regression
///   that removed the prefix-inference would null both. The Lua source
///   touches `Portrait` via the OnShow handler at line 2144 —
///   `SetPortraitTexture(_G[self:GetName().."Portrait"], "player")` —
///   which uses the global path, but the `Header.Portrait`
///   parent-property path is what every test fixture and downstream
///   consumer expects.
///
/// - `Summary` (line 2149) and `AchievementContainer` (line 2212) are
///   declared with explicit `parentKey="<Key>"` attributes, routed
///   through `append_parent_key_code` in
///   `src/loader/xml_frame_codegen.rs:90`. They participate in the
///   comparison-mode show/hide swap driven by
///   `AchievementFrameComparison_OnEvent` and the panel Lua code at
///   `Blizzard_AchievementUI.xml:2229-2238` (`parent.Summary:Show()` and
///   `parent.Summary:Hide()` from the AchievementContainer OnShow/OnHide
///   scripts) — a nil reading would surface a nil-table-method error in
///   the show/hide cascade.
///
/// The test asserts a uniform contract for all four paths: each is
/// reachable as a chained access from `_G.AchievementFrameComparison`
/// and is itself a table (frame or texture). The first probe
/// additionally confirms the comparison frame itself exists as a global
/// before the chained accesses are attempted, so a missing comparison
/// frame surfaces with a precise message rather than a confusing
/// nil-index error inside the loop.
#[test]
fn achievement_frame_comparison_subtree_publishes_plan_named_paths() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let comparison_type: String = env
            .eval(&format!("return type(_G[{COMPARISON_FRAME_NAME:?}])"))
            .expect("AchievementFrameComparison global probe must run cleanly");

        assert_eq!(
            comparison_type, "table",
            "Expected `_G[{COMPARISON_FRAME_NAME:?}]` to be a table after `{ROOT}` \
             loads, got `{comparison_type}`. The XML at `{COMPARISON_XML_SITE}` declares \
             the comparison sub-frame as `<Frame name=\"$parentComparison\">` nested \
             inside `AchievementFrame`'s `<Frames>` block, which resolves the name token \
             to `AchievementFrameComparison` and registers it in `_G`. A nil reading \
             means either the XML changed the name token, the frame was removed, or the \
             file chunk failed before reaching the declaration. Every Lua call-site that \
             drives the comparison panel — `AchievementFrameComparison_OnEvent` at line \
             2814, `AchievementFrameComparison_ForceUpdate` at 2846, the comparison-mode \
             tab swap in `AchievementFrame_SetComparisonTabs` at 332 — would surface a \
             nil-table-method error."
        );

        for (lua_path, xml_site, routing_kind) in PLAN_NAMED_COMPARISON_PATHS {
            let value_type: String = env
                .eval(&format!(
                    "return type(_G[{COMPARISON_FRAME_NAME:?}].{lua_path})"
                ))
                .expect("comparison subtree path probe must run cleanly");

            assert_eq!(
                value_type, "table",
                "Expected `AchievementFrameComparison.{lua_path}` to be a table after \
                 `{ROOT}` loads, got `{value_type}`. The XML at `{xml_site}` attaches \
                 this path via the `{routing_kind}` route. A nil reading on a \
                 `parentKey` path means the XML dropped the attribute or \
                 `append_parent_key_code` stopped routing it; a nil reading on a \
                 `name-prefix` path means the XML changed the `name=\"$parent<Key>\"` \
                 token or `infer_parent_key_from_child_name` stopped running. The \
                 comparison panel's show/hide cascade \
                 (`Blizzard_AchievementUI.xml:2229-2238`, `parent.Summary:Show()` etc.) \
                 and its inline INSPECT_ACHIEVEMENT_READY handler at \
                 `Blizzard_AchievementUI.lua:2815-2819` both walk this subtree."
            );
        }
    });
}

/// Spec/source split for the search progress bar.
///
/// PLAN names this child as `AchievementFrameSearchProgressBar` (PascalCase
/// global) with an `OnUpdate` script wired. Both claims are wrong against
/// `Mainline/Blizzard_AchievementUI.xml:2501-2566`:
///
/// 1. The XML declares the StatusBar with `parentKey="searchProgressBar"`
///    (camelCase, lowercase first letter) and **no `name=` attribute** at
///    all — there is no `_G.AchievementFrameSearchProgressBar` global. The
///    Lua source corroborates: every call-site uses
///    `AchievementFrame.searchProgressBar` (camelCase property access on
///    the parent) — see lines 297, 3262, 3295-3296, 3310, 3333.
///
/// 2. The XML wires three scripts at lines 2551-2564 — `OnShow`, `OnLoad`,
///    `OnHide` — but **does NOT wire `OnUpdate` at load time**. OnUpdate
///    is set dynamically by `AchievementFrameSearchBox_OnUpdate` at line
///    3296 (`AchievementFrame.searchProgressBar:SetScript("OnUpdate", AchievementFrameSearchProgressBar_OnUpdate)`)
///    only when search progress requires animated polling, and the OnHide
///    script at line 2562 explicitly unsets it (`self:SetScript("OnUpdate", nil)`).
///    At rest / immediately after addon load, `:GetScript("OnUpdate")`
///    returns nil.
///
/// The OnUpdate function itself — `AchievementFrameSearchProgressBar_OnUpdate`
/// at `Blizzard_AchievementUI.lua:3318` — IS a top-level global ready to
/// be wired (zero-arg signature `(self, elapsed)`, polls
/// `GetAchievementSearchProgress() / GetAchievementSearchSize()`,
/// self-unwires when progress reaches the max value).
///
/// Test split along the spec/source boundary:
///
/// **Presence half** asserts the StatusBar is reachable as
/// `AchievementFrame.searchProgressBar` (the actual route), is a
/// StatusBar, has `AchievementFrame` as its parent, and the OnUpdate
/// handler function `AchievementFrameSearchProgressBar_OnUpdate` exists
/// as a global ready to be wired.
///
/// **Absence half** asserts `_G.AchievementFrameSearchProgressBar` is
/// nil (the StatusBar has no name attribute, so no global registration
/// happens) and `AchievementFrame.searchProgressBar:GetScript("OnUpdate")`
/// is nil at load time (OnUpdate is dynamically attached only when
/// search runs). A non-nil reading on either side would surface a
/// meaningful regression: a global appearing means Blizzard added a
/// `name="..."` attribute, an OnUpdate at load time means either
/// the XML now wires it directly or some other init-time code attached
/// it.
#[test]
fn achievement_frame_search_progress_bar_split_presence_absence() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let property_type: String = env
            .eval(&format!(
                "return type(_G[{FRAME_NAME:?}][{SEARCH_PROGRESS_BAR_KEY:?}])"
            ))
            .expect("AchievementFrame.searchProgressBar probe must run cleanly");

        assert_eq!(
            property_type, "table",
            "Expected `AchievementFrame.{SEARCH_PROGRESS_BAR_KEY}` to be a table after \
             `{ROOT}` loads, got `{property_type}`. The XML at \
             `{SEARCH_PROGRESS_BAR_XML_SITE}` declares this StatusBar with \
             `parentKey=\"{SEARCH_PROGRESS_BAR_KEY}\"` (camelCase, lowercase first letter), \
             routed through `append_parent_key_code` in \
             `src/loader/xml_frame_codegen.rs:90`. A nil reading means either the XML \
             dropped the `parentKey=` attribute or the loader stopped routing it. The \
             Lua source touches this property at `Blizzard_AchievementUI.lua:297, 3262, \
             3295-3296, 3310, 3333` — every one of those call-sites would surface a \
             nil-table-method error."
        );

        let object_type: String = env
            .eval(&format!(
                "return _G[{FRAME_NAME:?}][{SEARCH_PROGRESS_BAR_KEY:?}]:GetObjectType()"
            ))
            .expect("`GetObjectType()` must run cleanly on the StatusBar");

        assert_eq!(
            object_type, "StatusBar",
            "Expected `AchievementFrame.{SEARCH_PROGRESS_BAR_KEY}:GetObjectType()` to \
             return `StatusBar`, got `{object_type}`. The XML at \
             `{SEARCH_PROGRESS_BAR_XML_SITE}` declares this child as `<StatusBar \
             parentKey=\"{SEARCH_PROGRESS_BAR_KEY}\" hidden=\"false\">` — a different \
             object type means the XML was changed to a different widget kind, which \
             would break `AchievementFrameSearchProgressBar_OnUpdate`'s \
             `GetMinMaxValues` / `GetValue` / `SetValue` calls at \
             `Blizzard_AchievementUI.lua:3319-3327`."
        );

        let parent_name: String = env
            .eval(&format!(
                "return _G[{FRAME_NAME:?}][{SEARCH_PROGRESS_BAR_KEY:?}]:GetParent():GetName()"
            ))
            .expect("`GetParent():GetName()` must run cleanly on the StatusBar");

        assert_eq!(
            parent_name, FRAME_NAME,
            "Expected `AchievementFrame.{SEARCH_PROGRESS_BAR_KEY}:GetParent():GetName()` \
             to return `{FRAME_NAME}`, got `{parent_name}`. The XML at \
             `{SEARCH_PROGRESS_BAR_XML_SITE}` nests this StatusBar inside \
             `AchievementFrame`'s `<Frames>` block, so its parent must be \
             `AchievementFrame`. A different parent means the StatusBar was reparented \
             post-load — the panel's Lua code addresses it as \
             `AchievementFrame.searchProgressBar`, which would null out under reparenting."
        );

        let handler_type: String = env
            .eval(&format!("return type(_G[{SEARCH_PROGRESS_BAR_HANDLER:?}])"))
            .expect("OnUpdate handler global probe must run cleanly");

        assert_eq!(
            handler_type, "function",
            "Expected `_G[{SEARCH_PROGRESS_BAR_HANDLER:?}]` to be a function after `{ROOT}` \
             loads, got `{handler_type}`. The Lua source at \
             `Blizzard_AchievementUI.lua:3318` declares `function {SEARCH_PROGRESS_BAR_HANDLER}(self, elapsed)` \
             at file scope. This is the OnUpdate handler that gets dynamically wired by \
             `AchievementFrameSearchBox_OnUpdate` at line 3296 when search progress \
             requires animated polling. A nil reading means the file chunk failed before \
             reaching the declaration or Blizzard refactored the handler onto a mixin \
             namespace."
        );

        let global_type: String = env
            .eval(&format!(
                "return type(_G[{SEARCH_PROGRESS_BAR_PLAN_NAME:?}])"
            ))
            .expect("PLAN-named global probe must run cleanly");

        assert_eq!(
            global_type, "nil",
            "Expected `_G[{SEARCH_PROGRESS_BAR_PLAN_NAME:?}]` to be nil after `{ROOT}` \
             loads, got `{global_type}`. The XML at `{SEARCH_PROGRESS_BAR_XML_SITE}` \
             declares this child with `parentKey=\"{SEARCH_PROGRESS_BAR_KEY}\"` and NO \
             `name=` attribute, so no global registration runs. The Lua source addresses \
             this StatusBar exclusively via the parent-property path \
             `AchievementFrame.{SEARCH_PROGRESS_BAR_KEY}` (camelCase). A non-nil reading \
             means Blizzard added a `name=\"{SEARCH_PROGRESS_BAR_PLAN_NAME}\"` attribute \
             to the XML, at which point the spec needs updating to reflect the new \
             access path."
        );

        let onupdate_script: String = env
            .eval(&format!(
                "return type(_G[{FRAME_NAME:?}][{SEARCH_PROGRESS_BAR_KEY:?}]:GetScript(\"OnUpdate\"))"
            ))
            .expect("`GetScript(\"OnUpdate\")` must run cleanly on the StatusBar");

        assert_eq!(
            onupdate_script, "nil",
            "Expected `AchievementFrame.{SEARCH_PROGRESS_BAR_KEY}:GetScript(\"OnUpdate\")` \
             to be nil after `{ROOT}` loads, got `{onupdate_script}`. The XML at \
             `{SEARCH_PROGRESS_BAR_XML_SITE}` wires three scripts at lines 2551-2564 — \
             `OnShow`, `OnLoad`, `OnHide` — but does NOT wire OnUpdate. The handler is \
             attached dynamically by `AchievementFrameSearchBox_OnUpdate` at \
             `Blizzard_AchievementUI.lua:3296` only when search progress requires \
             animated polling, and the OnHide script at line 2562 explicitly unsets it. \
             A non-nil reading at load time means either the XML now wires OnUpdate \
             directly or some init-time Lua attached it, both of which would burn \
             OnUpdate ticks every frame even when no search is running."
        );
    });
}
