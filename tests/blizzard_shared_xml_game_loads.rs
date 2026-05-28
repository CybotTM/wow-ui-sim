use std::path::PathBuf;

use wow_ui_sim::loader::load_addon;
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn shared_xml_game_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_SharedXMLGame")
}

fn shared_xml_game_toc() -> PathBuf {
    shared_xml_game_dir().join("Blizzard_SharedXMLGame.toc")
}

const PUBLISHED_NAMESPACES: &[&str] = &[
    "StaticModelInfo",
    "TooltipUtil",
    "TooltipDataRules",
    "TooltipDataProcessor",
    "TooltipComparisonManager",
    "CompactRaidGroupTypeEnum",
];

const PUBLISHED_MIXINS: &[&str] = &[
    "TooltipDataHandlerMixin",
    "DressUpModelFrameBaseMixin",
    "DressUpModelFrameResetButtonMixin",
    "DressUpModelFrameLinkButtonMixin",
    "DressUpModelFrameCloseButtonMixin",
    "DressUpModelFrameCancelButtonMixin",
    "DressUpModelFrameMaximizeMinimizeMixin",
    "DressUpFrameSetSelectionLabelMixin",
    "DressUpFrameTransmogSetMixin",
    "DressUpFrameTransmogSetButtonMixin",
    "CollapseButtonMixin",
    "ListHeaderVisualMixin",
    "ListHeaderMixin",
];

const VIRTUAL_LIST_TEMPLATES: &[&str] = &["CollapseButtonTemplate", "ListHeaderVisualTemplate"];

fn fresh_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }
    wow_ui_sim::xml::register_intrinsic_templates();
    env
}

fn load_full_game_ui() -> WowLuaEnv {
    let env = fresh_env();

    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    env
}

#[test]
fn find_toc_file_resolves_bare_toc() {
    let resolved = find_toc_file(&shared_xml_game_dir()).expect("SharedXMLGame TOC resolves");
    assert_eq!(
        resolved,
        shared_xml_game_toc(),
        "Blizzard_SharedXMLGame ships exactly one bare \
         `Blizzard_SharedXMLGame.toc` (NO `_Mainline` / `_Mists` flavor \
         variants). Per-flavor differences are expressed inline using \
         `[Family]` / `[Game]` placeholders + `[AllowLoadGameType ...]` / \
         `[ExcludeLoadGameType ...]` postfix annotations on body lines, \
         not via separate TOC files. The simulator's `push_file_entry` at \
         src/toc.rs:144-146 substitutes [Family]→Mainline and [Game]→\
         Standard at parse time"
    );
}

#[test]
fn dependencies_accessor_returns_empty_for_singular_dep_form() {
    let toc = TocFile::from_file(&shared_xml_game_toc()).expect("TOC parses");

    let deps = toc.dependencies();
    assert!(
        deps.is_empty(),
        "OBSERVATIONAL QUIRK: SharedXMLGame uses the singular `## Dep:` \
         form (one line per dep) — `## Dep: Blizzard_SharedXML` and \
         `## Dep: Blizzard_Colors`. The simulator's `dependencies()` \
         accessor at src/toc.rs:210-217 only honors `RequiredDep`, \
         `Dependencies`, `RequiredDeps` keys and falls through silently on \
         the singular `Dep` form. In real WoW the singular form is \
         honored, but here it surfaces as empty. Both targets still load \
         because Blizzard_SharedXML and Blizzard_Colors are themselves \
         eager (AllowLoad: Both / Game with no LoadOnDemand) — the missing \
         dep edge is invisible at load time but matters for any audit \
         that walks the dep graph via `dependencies()`. Got: {deps:?}"
    );
}

#[test]
fn singular_dep_directives_present_in_raw_metadata() {
    let raw = std::fs::read_to_string(shared_xml_game_toc()).expect("TOC reads utf-8");

    assert!(
        raw.contains("## Dep: Blizzard_SharedXML"),
        "Raw bytes MUST pin the singular `## Dep: Blizzard_SharedXML` \
         line — SharedXMLGame layers TooltipDataHandler / \
         DressUpModelFrameMixin / Mainline ListTemplates on top of the \
         primitive surface that SharedXML provides (FrameUtil, \
         CallbackRegistryMixin, etc.)"
    );
    assert!(
        raw.contains("## Dep: Blizzard_Colors"),
        "Raw bytes MUST pin the singular `## Dep: Blizzard_Colors` \
         line — DressUpModelFrameMixin / TooltipComparisonManager / \
         List header mixins reference CLASS_COLOR_OVERRIDES / quality \
         color tables that Blizzard_Colors publishes"
    );
}

#[test]
fn toc_is_game_only_with_no_load_on_demand() {
    let toc = TocFile::from_file(&shared_xml_game_toc()).expect("TOC parses");

    assert!(
        !toc.is_load_on_demand(),
        "SharedXMLGame must NOT be LoadOnDemand — server-driven \
         tooltip data callbacks (registered via \
         TooltipDataProcessor.AddTooltipPreCall) must be live before the \
         first GAME_TOOLTIP_SHOW or item-link inspection arrives, and \
         server events fire any time after PLAYER_ENTERING_WORLD"
    );
    assert!(!toc.is_load_first());
    assert!(!toc.is_secure_env());

    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(
        !toc.allows_screen(ScreenKind::Login),
        "`## AllowLoad: Game` must EXCLUDE Login — DressUpModelFrameMixin \
         and tooltip data layer require an in-game character context \
         (transmog state, equipped items, hovering live world units) \
         that doesn't exist on the glue screens"
    );
    assert!(!toc.allows_screen(ScreenKind::CharacterSelect));
    assert!(!toc.allows_screen(ScreenKind::CharacterCreate));
}

#[test]
fn toc_raw_bytes_pin_minimal_metadata_surface() {
    let raw = std::fs::read_to_string(shared_xml_game_toc()).expect("TOC reads utf-8");

    assert!(raw.contains("## Title: Blizzard_SharedXMLGame"));
    assert!(raw.contains("## AllowLoad: Game"));

    assert!(!raw.contains("## Author"));
    assert!(!raw.contains("## Version"));
    assert!(!raw.contains("## DefaultState"));
    assert!(!raw.contains("## AllowLoadGameType"));
    assert!(!raw.contains("## LoadOnDemand"));
    assert!(!raw.contains("## LoadFirst"));
    assert!(!raw.contains("## SavedVariables"));
    assert!(!raw.contains("## SavedVariablesPerCharacter"));
    assert!(!raw.contains("## OptionalDep"));
    assert!(!raw.contains("## RequiredDep"));
    assert!(!raw.contains("## Dependencies"));
    assert!(!raw.contains("## UseSecureEnvironment"));
}

#[test]
fn body_substitutes_family_and_drops_vanilla_only_entries() {
    let toc = TocFile::from_file(&shared_xml_game_toc()).expect("TOC parses");

    let body: Vec<String> = toc
        .files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    let expected = [
        "Shared/Localization.lua",
        "Mainline/Localization.lua",
        "Tooltip/TooltipUtil.lua",
        "Tooltip/TooltipDataHandler.lua",
        "Tooltip/TooltipDataRules.lua",
        "Tooltip/TooltipComparisonManager.lua",
        "StaticModelInfo.lua",
        "DressUpModelFrameMixin.lua",
        "Mainline/ListTemplates.lua",
        "Mainline/ListTemplates.xml",
        "CompactUnitFramesConstants.lua",
    ];

    assert_eq!(
        body, expected,
        "TOC body MUST reduce to exactly 11 entries on Mainline. The \
         loader (src/toc.rs:138-152) does three things per body line: \
         (1) drops lines tagged `[AllowLoadGameType vanilla]` (the \
         `[Game]\\Localization.lua` line is vanilla-only and thus \
         stripped on Mainline); (2) keeps lines tagged \
         `[ExcludeLoadGameType vanilla]` (TooltipDataHandler.lua); (3) \
         substitutes `[Family]` → `Mainline` and `[Game]` → `Standard` \
         on surviving lines; (4) normalizes backslashes to forward \
         slashes (Shared\\Localization.lua → Shared/Localization.lua). \
         If this test fails, either the placeholder substitution table \
         drifted or the ExcludeLoadGameType/AllowLoadGameType filter \
         logic regressed. Got: {body:?}"
    );
}

#[test]
fn body_count_matches_filtered_filesystem_layout() {
    let toc = TocFile::from_file(&shared_xml_game_toc()).expect("TOC parses");

    let lua_count = toc
        .files
        .iter()
        .filter(|f| f.extension().is_some_and(|ext| ext == "lua"))
        .count();
    let xml_count = toc
        .files
        .iter()
        .filter(|f| f.extension().is_some_and(|ext| ext == "xml"))
        .count();

    assert_eq!(toc.files.len(), 11, "11 entries after Mainline filter");
    assert_eq!(
        lua_count, 10,
        "Exactly 10 .lua entries survive Mainline filter: 2 Localization \
         (Shared + Mainline; vanilla [Game]\\Localization stripped), 4 \
         Tooltip (Util / DataHandler / DataRules / ComparisonManager — \
         all gated to Mainline except DataHandler which is \
         ExcludeLoadGameType vanilla), 2 root (StaticModelInfo / \
         DressUpModelFrameMixin), 1 Mainline ListTemplates.lua, 1 \
         CompactUnitFramesConstants.lua. Got {lua_count}"
    );
    assert_eq!(
        xml_count, 1,
        "Exactly 1 .xml entry: Mainline/ListTemplates.xml — the only \
         XML in the body. CollapseButtonTemplate / ListHeaderVisualTemplate \
         / ListHeaderCodeTemplate / ListHeaderThreeSliceTemplate all \
         live in this single XML file. Got {xml_count}"
    );
}

#[test]
fn tooltip_lua_companions_load_before_dressup_consumer() {
    let toc = TocFile::from_file(&shared_xml_game_toc()).expect("TOC parses");

    let body: Vec<String> = toc
        .files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    let dress_up = body
        .iter()
        .position(|p| p == "DressUpModelFrameMixin.lua")
        .expect("DressUpModelFrameMixin.lua present");
    let tooltip_data = body
        .iter()
        .position(|p| p == "Tooltip/TooltipDataHandler.lua")
        .expect("TooltipDataHandler.lua present");

    assert!(
        tooltip_data < dress_up,
        "TooltipDataHandler.lua (publishes TooltipDataProcessor + \
         TooltipDataHandlerMixin) MUST load before \
         DressUpModelFrameMixin.lua. The dress-up frame's tooltip-on-hover \
         path uses TooltipDataProcessor.AddTooltipPreCall to register \
         transmog-aware comparison logic; without TooltipDataProcessor \
         live, the pre-call registration would crash on nil indexing. \
         Got tooltip_data at {} dress_up at {}",
        tooltip_data,
        dress_up
    );

    let list_lua = body
        .iter()
        .position(|p| p == "Mainline/ListTemplates.lua")
        .expect("ListTemplates.lua present");
    let list_xml = body
        .iter()
        .position(|p| p == "Mainline/ListTemplates.xml")
        .expect("ListTemplates.xml present");
    assert!(
        list_lua < list_xml,
        "ListTemplates.lua MUST load before ListTemplates.xml. The XML \
         declares CollapseButtonTemplate with `mixin=\"CollapseButtonMixin\"` \
         and ListHeader{{Visual,Code,ThreeSlice}}Template with \
         corresponding mixin attrs — those mixin attrs resolve against \
         `_G` at parse time, so the .lua must publish them first. \
         Got lua at {} xml at {}",
        list_lua,
        list_xml
    );
}

#[test]
fn eager_discovery_includes_addon_only_on_game_screen() {
    let game_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let game_names: Vec<&str> = game_addons.iter().map(|(n, _)| n.as_str()).collect();
    assert!(
        game_names.contains(&"Blizzard_SharedXMLGame"),
        "Game-screen discovery MUST include Blizzard_SharedXMLGame. \
         AllowLoad: Game + no LoadOnDemand = eager on the in-game screen \
         only. Game-screen consumers (every tooltip frame, dress-up \
         preview, raid list display) hard-rely on TooltipDataProcessor \
         and DressUpModelFrameBaseMixin being live"
    );

    for screen in [
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        let glue_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), screen);
        let glue_names: Vec<&str> = glue_addons.iter().map(|(n, _)| n.as_str()).collect();
        assert!(
            !glue_names.contains(&"Blizzard_SharedXMLGame"),
            "{screen:?} discovery MUST NOT include Blizzard_SharedXMLGame. \
             AllowLoad: Game restricts it to in-game only — glue screens \
             have no transmog state / equipped items / live world units \
             for the dress-up + tooltip layer to operate against"
        );
    }
}

#[test]
fn full_game_load_emits_no_addon_specific_lua_errors() {
    let env = load_full_game_ui();
    let errors: Vec<String> = env.state().borrow().lua_errors.clone();

    let addon_specific: Vec<&String> = errors
        .iter()
        .filter(|err| {
            err.contains("Blizzard_SharedXMLGame")
                || err.contains("TooltipDataHandler")
                || err.contains("TooltipDataRules")
                || err.contains("TooltipComparisonManager")
                || err.contains("DressUpModelFrameMixin")
                || err.contains("StaticModelInfo")
        })
        .collect();

    assert!(
        addon_specific.is_empty(),
        "Full Game-screen UI load must produce zero \
         SharedXMLGame-attributed Lua errors. Errors: {addon_specific:#?}"
    );
}

#[test]
fn is_addon_loaded_reports_true_after_eager_sweep() {
    let env = load_full_game_ui();

    let result: bool = env
        .eval("return C_AddOns.IsAddOnLoaded(\"Blizzard_SharedXMLGame\")")
        .expect("IsAddOnLoaded query succeeds");
    assert!(
        result,
        "C_AddOns.IsAddOnLoaded(\"Blizzard_SharedXMLGame\") MUST return \
         true after the Game-screen eager sweep. The TOC has no \
         LoadOnDemand and AllowLoad: Game, so discovery includes it"
    );
}

#[test]
fn publishes_six_top_level_namespace_tables() {
    let env = load_full_game_ui();

    for namespace in PUBLISHED_NAMESPACES {
        let result: bool = env
            .eval(&format!("return type(_G[{namespace:?}]) == \"table\""))
            .unwrap_or_else(|err| panic!("eval for {namespace}: {err}"));
        assert!(
            result,
            "Top-level namespace {namespace} must be a `_G` table after \
             SharedXMLGame loads. These are the static-helper namespaces \
             every game-screen consumer imports — TooltipDataProcessor for \
             tooltip-data callback registration, StaticModelInfo for \
             effect-model lookup, TooltipComparisonManager for the \
             multi-tooltip comparison logic"
        );
    }
}

#[test]
fn publishes_thirteen_dress_up_and_list_mixin_tables() {
    let env = load_full_game_ui();

    for mixin in PUBLISHED_MIXINS {
        let result: bool = env
            .eval(&format!("return type(_G[{mixin:?}]) == \"table\""))
            .unwrap_or_else(|err| panic!("eval for {mixin}: {err}"));
        assert!(
            result,
            "Mixin {mixin} must be a `_G` table after SharedXMLGame loads. \
             DressUp* mixins drive the dress-up preview frame's button \
             hierarchy (Reset/Link/Close/Cancel/MaxMin); CollapseButton + \
             ListHeader* mixins drive collapsible list section headers \
             used pervasively in Mainline addon dialogs (TalentTree spec \
             list, AchievementUI category list, etc.)"
        );
    }
}

#[test]
fn tooltip_data_processor_provides_callback_registration_api() {
    let env = load_full_game_ui();

    let result: bool = env
        .eval(
            "return type(TooltipDataProcessor) == \"table\" and \
                    type(TooltipDataProcessor.AddTooltipPreCall) == \"function\" and \
                    type(TooltipDataProcessor.AddTooltipPostCall) == \"function\" and \
                    type(TooltipDataProcessor.AddLinePreCall) == \"function\" and \
                    type(TooltipDataProcessor.AddLinePostCall) == \"function\"",
        )
        .expect("TooltipDataProcessor shape check");
    assert!(
        result,
        "TooltipDataProcessor must publish 4 callback registration \
         entry points: AddTooltipPreCall / AddTooltipPostCall (per-tooltip \
         hooks keyed by tooltipType) and AddLinePreCall / AddLinePostCall \
         (per-line hooks keyed by lineType). Every Blizzard tooltip enrich \
         site (transmog comparison, item-comparison, quest-objective \
         tooltips) registers via these"
    );
}

#[test]
fn static_model_info_helpers_callable() {
    let env = load_full_game_ui();

    let result: bool = env
        .eval(
            "return type(StaticModelInfo) == \"table\" and \
                    type(StaticModelInfo.CreateModelSceneEntry) == \"function\" and \
                    type(StaticModelInfo.SetupModelScene) == \"function\"",
        )
        .expect("StaticModelInfo shape check");
    assert!(
        result,
        "StaticModelInfo must publish CreateModelSceneEntry (factory for \
         model-scene effect entries) and SetupModelScene (applies a \
         pre-built scene config to a ModelScene frame). Used by the \
         dress-up preview, character creation backdrop, and login portrait \
         scenes — without these helpers each consumer would re-implement \
         model-effect plumbing"
    );
}

#[test]
fn dress_up_model_frame_base_mixin_lifecycle_methods_present() {
    let env = load_full_game_ui();

    let result: bool = env
        .eval(
            "return type(DressUpModelFrameBaseMixin) == \"table\" and \
                    type(DressUpModelFrameBaseMixin.OnLoad) == \"function\" and \
                    type(DressUpModelFrameBaseMixin.SetMode) == \"function\" and \
                    type(DressUpModelFrameBaseMixin.GetMode) == \"function\" and \
                    type(DressUpModelFrameBaseMixin.OnModelSceneReset) == \"function\"",
        )
        .expect("DressUpModelFrameBaseMixin shape check");
    assert!(
        result,
        "DressUpModelFrameBaseMixin must publish OnLoad / SetMode / \
         GetMode / OnModelSceneReset — the lifecycle entry points for the \
         dress-up preview frame. Mode controls whether the frame is in \
         transmog-preview mode vs model-inspection mode"
    );
}

#[test]
fn tooltip_comparison_manager_initialize_and_clear_present() {
    let env = load_full_game_ui();

    let result: bool = env
        .eval(
            "return type(TooltipComparisonManager) == \"table\" and \
                    type(TooltipComparisonManager.Initialize) == \"function\" and \
                    type(TooltipComparisonManager.Clear) == \"function\" and \
                    type(TooltipComparisonManager.CreateComparisonItem) == \"function\"",
        )
        .expect("TooltipComparisonManager shape check");
    assert!(
        result,
        "TooltipComparisonManager must publish Initialize / Clear / \
         CreateComparisonItem. Drives the side-by-side equipped-item \
         comparison shown when the Compare modifier (default Shift) is \
         held while hovering an item — Initialize binds the manager to a \
         primary tooltip + anchor frame, CreateComparisonItem builds the \
         compared-item TooltipData, Clear tears down secondary tooltips"
    );
}

#[test]
fn list_xml_publishes_four_virtual_templates() {
    let env = load_full_game_ui();

    for template in VIRTUAL_LIST_TEMPLATES {
        let probe = format!(
            "local ok = pcall(function() \
                local f = CreateFrame(\"Button\", nil, nil, {template:?}) \
                return type(f) == \"table\" \
             end) return ok"
        );
        let result: bool = env
            .eval(&probe)
            .unwrap_or_else(|err| panic!("eval for {template}: {err}"));
        assert!(
            result,
            "Virtual template {template} (registered by \
             Mainline/ListTemplates.xml) must materialize via \
             CreateFrame. CollapseButtonTemplate is the simple base; \
             ListHeaderVisualTemplate composes it via parentKey \
             inheritance, validating the cross-template wiring. The \
             script-bearing siblings ListHeaderCodeTemplate / \
             ListHeaderThreeSliceTemplate dispatch OnLoad to a method \
             on the empty ListHeaderMixin (set up later by consumer \
             addons), so they are intentionally excluded from this \
             materialization probe — their mixin tables are still \
             checked by publishes_thirteen_mixin_tables"
        );
    }
}

#[test]
fn compact_raid_group_type_enum_published_with_party_and_raid_keys() {
    let env = load_full_game_ui();

    let result: bool = env
        .eval(
            "return type(CompactRaidGroupTypeEnum) == \"table\" and \
                    CompactRaidGroupTypeEnum.Party ~= nil and \
                    CompactRaidGroupTypeEnum.Raid ~= nil",
        )
        .expect("CompactRaidGroupTypeEnum shape check");
    assert!(
        result,
        "CompactRaidGroupTypeEnum must publish Party + Raid keys. \
         CompactUnitFramesConstants.lua (always-loaded, no flavor gate) \
         seeds this enum so the compact raid frame manager can branch \
         layout strategy by group composition. Both keys must be present \
         and non-nil"
    );
}
