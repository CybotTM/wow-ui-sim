//! Frame-surface probes for `Blizzard_AnimaDiversionUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const ANIMA_DIVERSION_FRAME_SURFACE_PROBE: &str = r#"
return type(AnimaDiversionFrame),
       AnimaDiversionFrame:GetObjectType(),
       AnimaDiversionFrame:GetParent() == UIParent,
       AnimaDiversionFrame:IsToplevel(),
       AnimaDiversionFrame:IsShown(),
       AnimaDiversionFrame.OnLoad == AnimaDiversionFrameMixin.OnLoad,
       AnimaDiversionFrame.OnShow == AnimaDiversionFrameMixin.OnShow,
       type(AnimaDiversionFrame.dataProviders),
       type(AnimaDiversionFrame.pinFrameLevelsManager),
       type(AnimaDiversionFrame.pinFrameLevelsManager and AnimaDiversionFrame.pinFrameLevelsManager.definitions)
"#;
const ANIMA_DIVERSION_FRAME_CHILDREN_PROBE: &str = r#"
local frame = AnimaDiversionFrame
local currencyFrame = frame.AnimaDiversionCurrencyFrame
local nestedCurrencyFrame = currencyFrame and currencyFrame.CurrencyFrame
local quantity = nestedCurrencyFrame and nestedCurrencyFrame.Quantity

return type(frame.NineSlice),
       type(frame.BorderFrame),
       type(frame.ScrollContainer),
       type(frame.CloseButton),
       type(currencyFrame),
       type(frame.ReinforceProgressFrame),
       type(frame.ReinforceInfoFrame),
       type(nestedCurrencyFrame),
       type(quantity),
       quantity and quantity:GetObjectType()
"#;

#[test]
fn anima_diversion_frame_loads_as_hidden_map_canvas_panel() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let surface: AnimaDiversionFrameSurface = env
            .eval(ANIMA_DIVERSION_FRAME_SURFACE_PROBE)
            .expect("AnimaDiversionFrame surface probe must run cleanly");

        assert_anima_diversion_frame_surface(surface);
    });
}

#[test]
fn anima_diversion_frame_exposes_expected_children() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let surface: AnimaDiversionFrameChildren = env
            .eval(ANIMA_DIVERSION_FRAME_CHILDREN_PROBE)
            .expect("AnimaDiversionFrame child probe must run cleanly");

        assert_anima_diversion_frame_children(surface);
    });
}

type AnimaDiversionFrameSurface = (
    String,
    String,
    bool,
    bool,
    bool,
    bool,
    bool,
    String,
    String,
    String,
);
type AnimaDiversionFrameChildren = (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    Option<String>,
);

fn assert_anima_diversion_frame_surface(surface: AnimaDiversionFrameSurface) {
    let (
        frame_type,
        object_type,
        parent_is_ui_parent,
        is_toplevel,
        is_shown,
        on_load_matches_mixin,
        on_show_matches_mixin,
        data_providers_type,
        pin_frame_levels_manager_type,
        pin_frame_level_definitions_type,
    ) = surface;

    assert_frame_identity(frame_type, object_type, parent_is_ui_parent);
    assert_frame_state(is_toplevel, is_shown);
    assert_frame_mixin(on_load_matches_mixin, on_show_matches_mixin);
    assert_map_canvas_state(
        data_providers_type,
        pin_frame_levels_manager_type,
        pin_frame_level_definitions_type,
    );
}

fn assert_frame_identity(frame_type: String, object_type: String, parent_is_ui_parent: bool) {
    assert_eq!(frame_type, "table", "`AnimaDiversionFrame` must exist");
    assert_eq!(
        object_type, "Frame",
        "`AnimaDiversionFrame` must be a Frame object"
    );
    assert!(
        parent_is_ui_parent,
        "`AnimaDiversionFrame` must be parented to UIParent"
    );
}

fn assert_frame_state(is_toplevel: bool, is_shown: bool) {
    assert!(
        is_toplevel,
        "`AnimaDiversionFrame` must honor XML `toplevel=true`"
    );
    assert!(
        !is_shown,
        "`AnimaDiversionFrame` must be hidden immediately after load"
    );
}

fn assert_frame_mixin(on_load_matches_mixin: bool, on_show_matches_mixin: bool) {
    assert!(
        on_load_matches_mixin,
        "`AnimaDiversionFrame` must mix in `AnimaDiversionFrameMixin.OnLoad`"
    );
    assert!(
        on_show_matches_mixin,
        "`AnimaDiversionFrame` must mix in `AnimaDiversionFrameMixin.OnShow`"
    );
}

fn assert_map_canvas_state(
    data_providers_type: String,
    pin_frame_levels_manager_type: String,
    pin_frame_level_definitions_type: String,
) {
    assert_eq!(
        data_providers_type, "table",
        "`MapCanvasMixin.OnLoad` must initialize dataProviders"
    );
    assert_eq!(
        pin_frame_levels_manager_type, "table",
        "`MapCanvasMixin.OnLoad` must initialize pinFrameLevelsManager"
    );
    assert_eq!(
        pin_frame_level_definitions_type, "table",
        "`MapCanvasPinFrameLevelsManagerMixin:Initialize` must seed definitions"
    );
}

fn assert_anima_diversion_frame_children(children: AnimaDiversionFrameChildren) {
    let (
        nine_slice_type,
        border_frame_type,
        scroll_container_type,
        close_button_type,
        currency_frame_type,
        reinforce_progress_frame_type,
        reinforce_info_frame_type,
        nested_currency_frame_type,
        quantity_type,
        quantity_object_type,
    ) = children;

    assert_child_frame("NineSlice", nine_slice_type);
    assert_child_frame("BorderFrame", border_frame_type);
    assert_child_frame("ScrollContainer", scroll_container_type);
    assert_child_frame("CloseButton", close_button_type);
    assert_child_frame("AnimaDiversionCurrencyFrame", currency_frame_type);
    assert_child_frame("ReinforceProgressFrame", reinforce_progress_frame_type);
    assert_child_frame("ReinforceInfoFrame", reinforce_info_frame_type);
    assert_child_frame(
        "AnimaDiversionCurrencyFrame.CurrencyFrame",
        nested_currency_frame_type,
    );
    assert_child_frame(
        "AnimaDiversionCurrencyFrame.CurrencyFrame.Quantity",
        quantity_type,
    );
    assert_eq!(
        quantity_object_type.as_deref(),
        Some("FontString"),
        "`AnimaDiversionCurrencyFrame.CurrencyFrame.Quantity` must be a FontString"
    );
}

fn assert_child_frame(parent_key: &str, child_type: String) {
    assert_eq!(
        child_type, "table",
        "`AnimaDiversionFrame.{parent_key}` must exist as a frame child"
    );
}
