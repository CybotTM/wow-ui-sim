//! `AnimaDiversionFrameMixin:OnLoad` close-button styling probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const CLOSE_BUTTON_BORDER_ATLAS: &str = "UI-Frame-Oribos-ExitButtonBorder";
const CLOSE_BUTTON_BORDER_ATLAS_PROBE: &str = r#"
local border = AnimaDiversionFrame.CloseButton.Border
return type(border), border and border:GetAtlas()
"#;

#[test]
fn onload_styles_close_button_border_atlas() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let surface: CloseButtonBorderSurface = env
            .eval(CLOSE_BUTTON_BORDER_ATLAS_PROBE)
            .expect("AnimaDiversionFrame close-button border probe must run cleanly");

        assert_close_button_border(surface);
    });
}

type CloseButtonBorderSurface = (String, Option<String>);

fn assert_close_button_border(surface: CloseButtonBorderSurface) {
    let (border_type, border_atlas) = surface;

    assert_eq!(
        border_type, "table",
        "`AnimaDiversionFrame.CloseButton.Border` must exist after `OnLoad`"
    );
    assert_eq!(
        border_atlas.as_deref(),
        Some(CLOSE_BUTTON_BORDER_ATLAS),
        "`OnLoad` must style the close-button border via `UIPanelCloseButton_SetBorderAtlas`"
    );
}
