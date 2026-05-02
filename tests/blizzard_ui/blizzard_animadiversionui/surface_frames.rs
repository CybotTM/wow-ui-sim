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

#[test]
fn anima_diversion_frame_loads_as_hidden_map_canvas_panel() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let surface: AnimaDiversionFrameSurface = env
            .eval(ANIMA_DIVERSION_FRAME_SURFACE_PROBE)
            .expect("AnimaDiversionFrame surface probe must run cleanly");

        assert_anima_diversion_frame_surface(surface);
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
