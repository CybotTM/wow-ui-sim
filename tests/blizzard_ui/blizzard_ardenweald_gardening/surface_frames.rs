//! Frame factory surface for `Blizzard_ArdenwealdGardening`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArdenwealdGardening";

#[test]
fn ardenweald_gardening_create_returns_panel_with_template_children() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let surface: FrameSurfaceProbe = env
            .eval(
                r#"
                local parent = CreateFrame("Frame", "ArdenwealdGardeningTestParent", UIParent)
                local panel = ArdenwealdGardening.Create(parent)

                return panel:GetObjectType(),
                       panel:GetParent() == parent,
                       panel:GetWidth(),
                       panel:GetHeight(),
                       panel.Background:GetObjectType(),
                       panel.Label:GetObjectType(),
                       type(panel.Button),
                       ArdenwealdGardeningButtonTemplate:GetObjectType(),
                       ArdenwealdGardeningButtonTemplate:GetParent() == panel,
                       panel.Label:GetText(),
                       GARDENWEALD_LANDING_HEADER
                "#,
            )
            .expect("Ardenweald Gardening frame surface probe must run cleanly");

        assert_frame_surface(surface);
    });
}

type FrameSurfaceProbe = (
    String,
    bool,
    f64,
    f64,
    String,
    String,
    String,
    String,
    bool,
    String,
    String,
);

fn assert_frame_surface(surface: FrameSurfaceProbe) {
    let (
        panel_type,
        parent_matches,
        panel_width,
        panel_height,
        background_type,
        label_type,
        panel_button_type,
        named_button_type,
        named_button_parent_matches,
        label_text,
        expected_label_text,
    ) = surface;

    assert_panel_shape(panel_type, parent_matches, panel_width, panel_height);
    assert_panel_children(
        background_type,
        label_type,
        panel_button_type,
        named_button_type,
        named_button_parent_matches,
    );
    assert_eq!(
        label_text, expected_label_text,
        "`panel.Label` text must resolve from `GARDENWEALD_LANDING_HEADER`"
    );
}

fn assert_panel_shape(
    panel_type: String,
    parent_matches: bool,
    panel_width: f64,
    panel_height: f64,
) {
    assert_eq!(
        panel_type, "Frame",
        "`ArdenwealdGardening.Create(parent)` must return a Frame"
    );
    assert!(
        parent_matches,
        "created Ardenweald gardening panel must be parented to the supplied parent"
    );
    assert_eq!(
        panel_width, 350.0,
        "`ArdenwealdGardeningPanelTemplate` width must be applied"
    );
    assert_eq!(
        panel_height, 200.0,
        "`ArdenwealdGardeningPanelTemplate` height must be applied"
    );
}

fn assert_panel_children(
    background_type: String,
    label_type: String,
    panel_button_type: String,
    named_button_type: String,
    named_button_parent_matches: bool,
) {
    assert_eq!(
        background_type, "Texture",
        "panel must expose `Background` texture by parentKey"
    );
    assert_eq!(
        label_type, "FontString",
        "panel must expose `Label` font string by parentKey"
    );
    assert_eq!(
        panel_button_type, "nil",
        "`ArdenwealdGardeningPanelTemplate` does not declare a `Button` parentKey"
    );
    assert_eq!(
        named_button_type, "Button",
        "`ArdenwealdGardeningButtonTemplate` named child must be instantiated"
    );
    assert!(
        named_button_parent_matches,
        "`ArdenwealdGardeningButtonTemplate` named child must be parented to the created panel"
    );
}
