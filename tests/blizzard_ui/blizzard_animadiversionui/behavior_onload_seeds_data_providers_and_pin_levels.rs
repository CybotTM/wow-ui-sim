//! `AnimaDiversionFrameMixin:OnLoad` behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const ONLOAD_STATE_PROBE: &str = r#"
local providerCount = 0
local hasAnimaProvider = false
local hasWorldQuestProvider = false
for provider in pairs(AnimaDiversionFrame.dataProviders) do
    providerCount = providerCount + 1
    hasAnimaProvider = hasAnimaProvider or provider.OnShow == AnimaDiversionDataProviderMixin.OnShow
    hasWorldQuestProvider = hasWorldQuestProvider or provider.OnAdded == AnimaDiversion_WorldQuestDataProviderMixin.OnAdded
end

local levels = AnimaDiversionFrame.pinFrameLevelsManager.definitions
local worldQuestLevel = levels.PIN_FRAME_LEVEL_WORLD_QUEST
local modelSceneLevel = levels.PIN_FRAME_LEVEL_ANIMA_DIVERSION_MODELSCENE_PIN
local animaPinLevel = levels.PIN_FRAME_LEVEL_ANIMA_DIVERSION_PIN

return type(AnimaDiversionFrame.dataProviders),
       type(AnimaDiversionFrame.pinFrameLevelsManager),
       AnimaDiversionFrame:ShouldZoomInOnClick(),
       AnimaDiversionFrame.ScrollContainer.mouseWheelZoomMode,
       MAP_CANVAS_MOUSE_WHEEL_ZOOM_BEHAVIOR_NONE,
       AnimaDiversionFrame:ShouldPanOnClick(),
       providerCount,
       hasAnimaProvider,
       hasWorldQuestProvider,
       worldQuestLevel and worldQuestLevel.range,
       modelSceneLevel and modelSceneLevel.range,
       animaPinLevel and animaPinLevel.range,
       type(AnimaDiversionFrame.bolsterProgressGemPool)
"#;

#[test]
fn onload_seeds_data_providers_pin_levels_and_interaction_options() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: OnLoadState = env
            .eval(ONLOAD_STATE_PROBE)
            .expect("AnimaDiversionFrame OnLoad state probe must run cleanly");

        assert_onload_state(state);
    });
}

type OnLoadState = (
    String,
    String,
    bool,
    i64,
    i64,
    bool,
    i64,
    bool,
    bool,
    Option<i64>,
    Option<i64>,
    Option<i64>,
    String,
);

fn assert_onload_state(state: OnLoadState) {
    let (
        data_providers_type,
        pin_frame_levels_manager_type,
        should_zoom_in_on_click,
        mouse_wheel_zoom_mode,
        no_mouse_wheel_zoom_mode,
        should_pan_on_click,
        provider_count,
        has_anima_provider,
        has_world_quest_provider,
        world_quest_range,
        model_scene_range,
        anima_pin_range,
        gem_pool_type,
    ) = state;

    assert_map_canvas_seeded(data_providers_type, pin_frame_levels_manager_type);
    assert_interaction_options(
        should_zoom_in_on_click,
        mouse_wheel_zoom_mode,
        no_mouse_wheel_zoom_mode,
        should_pan_on_click,
    );
    assert_data_providers(provider_count, has_anima_provider, has_world_quest_provider);
    assert_pin_frame_levels(world_quest_range, model_scene_range, anima_pin_range);
    assert_gem_pool(gem_pool_type);
}

fn assert_map_canvas_seeded(data_providers_type: String, pin_frame_levels_manager_type: String) {
    assert_eq!(
        data_providers_type, "table",
        "`MapCanvasMixin.OnLoad` must initialize dataProviders"
    );
    assert_eq!(
        pin_frame_levels_manager_type, "table",
        "`MapCanvasMixin.OnLoad` must initialize pinFrameLevelsManager"
    );
}

fn assert_interaction_options(
    should_zoom_in_on_click: bool,
    mouse_wheel_zoom_mode: i64,
    no_mouse_wheel_zoom_mode: i64,
    should_pan_on_click: bool,
) {
    assert!(
        !should_zoom_in_on_click,
        "`AnimaDiversionFrameMixin:OnLoad` must disable zoom-on-click"
    );
    assert_eq!(
        mouse_wheel_zoom_mode, no_mouse_wheel_zoom_mode,
        "`AnimaDiversionFrameMixin:OnLoad` must disable mouse-wheel zoom"
    );
    assert!(
        !should_pan_on_click,
        "`AnimaDiversionFrameMixin:OnLoad` must disable pan-on-click"
    );
}

fn assert_data_providers(
    provider_count: i64,
    has_anima_provider: bool,
    has_world_quest_provider: bool,
) {
    assert_eq!(
        provider_count, 2,
        "`AddStandardDataProviders` must add exactly two data providers"
    );
    assert!(
        has_anima_provider,
        "`AddStandardDataProviders` must add `AnimaDiversionDataProviderMixin`"
    );
    assert!(
        has_world_quest_provider,
        "`AddStandardDataProviders` must add `AnimaDiversion_WorldQuestDataProviderMixin`"
    );
}

fn assert_pin_frame_levels(
    world_quest_range: Option<i64>,
    model_scene_range: Option<i64>,
    anima_pin_range: Option<i64>,
) {
    assert_eq!(
        world_quest_range,
        Some(500),
        "`PIN_FRAME_LEVEL_WORLD_QUEST` must reserve a 500-level range"
    );
    assert_eq!(
        model_scene_range,
        Some(1),
        "`PIN_FRAME_LEVEL_ANIMA_DIVERSION_MODELSCENE_PIN` must be registered"
    );
    assert_eq!(
        anima_pin_range,
        Some(1),
        "`PIN_FRAME_LEVEL_ANIMA_DIVERSION_PIN` must be registered"
    );
}

fn assert_gem_pool(gem_pool_type: String) {
    assert_eq!(
        gem_pool_type, "table",
        "`AnimaDiversionFrameMixin:OnLoad` must seed bolsterProgressGemPool"
    );
}
