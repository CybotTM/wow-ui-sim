use super::*;

#[test]
fn test_create_texture_inherits_template_size() {
    use wow_ui_sim::xml::{SizeXml, TextureXml, register_texture_template};

    let env = env();
    // Register the template after env creation because WowLuaEnv resets the
    // global XML registries to isolate tests from prior env state.
    register_texture_template(
        "TestTileTemplate",
        TextureXml {
            size: Some(SizeXml {
                x: Some(256.0),
                y: Some(256.0),
                abs_dimension: None,
            }),
            ..Default::default()
        },
    );

    // CreateTexture(name, layer, inherits, subLevel) should apply template size
    let (w, h): (f64, f64) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", nil, UIParent)
        local tex = f:CreateTexture(nil, "BACKGROUND", "TestTileTemplate")
        return tex:GetSize()
    "#,
        )
        .unwrap();
    assert_eq!(
        w, 256.0,
        "CreateTexture with inherits should apply template width"
    );
    assert_eq!(
        h, 256.0,
        "CreateTexture with inherits should apply template height"
    );
}

#[test]
fn test_create_texture_applies_sublevel_argument() {
    let env = env();
    let (layer, sublevel): (String, i32) = env
        .eval(
            r#"
        local f = CreateFrame("Frame", nil, UIParent)
        local tex = f:CreateTexture(nil, "OVERLAY", nil, 7)
        return tex:GetDrawLayer()
    "#,
        )
        .unwrap();

    assert_eq!(
        layer, "OVERLAY",
        "CreateTexture should keep the requested draw layer"
    );
    assert_eq!(
        sublevel, 7,
        "CreateTexture should apply the requested draw sublevel"
    );
}
