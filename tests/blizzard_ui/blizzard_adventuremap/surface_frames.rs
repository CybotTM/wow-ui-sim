//! Frame-surface probes for `Blizzard_AdventureMap`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";

#[test]
fn adventure_map_frame_loads_as_map_canvas_panel() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface: AdventureMapFrameSurface = env
            .eval(
                r#"
                return type(AdventureMapFrame),
                       AdventureMapFrame and AdventureMapFrame:GetFrameStrata() or nil,
                       AdventureMapFrame and AdventureMapFrame:GetParent():GetName() or nil,
                       type(AdventureMapFrame and AdventureMapFrame.dataProviders),
                       type(AdventureMapFrame and AdventureMapFrame.pinFrameLevelsManager),
                       type(AdventureMapFrame and AdventureMapFrame.pinFrameLevelsManager and AdventureMapFrame.pinFrameLevelsManager.definitions)
                "#,
            )
            .expect("AdventureMapFrame surface probe must run cleanly");

        assert_adventure_map_frame_surface(surface);
    });
}

type AdventureMapFrameSurface = (String, String, String, String, String, String);

fn assert_adventure_map_frame_surface(surface: AdventureMapFrameSurface) {
    let (
        frame_type,
        frame_strata,
        parent_name,
        data_providers_type,
        pin_frame_levels_manager_type,
        pin_frame_level_definitions_type,
    ) = surface;

    assert_eq!(frame_type, "table", "`AdventureMapFrame` must exist");
    assert_eq!(
        frame_strata, "DIALOG",
        "`AdventureMapFrame` must load in the DIALOG strata"
    );
    assert_eq!(
        parent_name, "UIParent",
        "`AdventureMapFrame` must be parented to UIParent"
    );
    assert_eq!(
        data_providers_type, "table",
        "`AdventureMapFrame` must expose MapCanvas `dataProviders` state"
    );
    assert_eq!(
        pin_frame_levels_manager_type, "table",
        "`AdventureMapFrame` must expose MapCanvas `pinFrameLevelsManager` state"
    );
    assert_eq!(
        pin_frame_level_definitions_type, "table",
        "`MapCanvasPinFrameLevelsManagerMixin:Initialize` must seed frame-level definitions"
    );
}
