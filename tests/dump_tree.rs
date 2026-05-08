use wow_ui_sim::dump::{build_tree, build_warning_dump, strip_wow_escapes};
use wow_ui_sim::widget::{Anchor, AnchorPoint, Frame, WidgetRegistry, WidgetType};

fn make_frame(id: u64, parent: Option<u64>, w: f32, h: f32) -> Frame {
    Frame {
        id,
        parent_id: parent,
        width: w,
        height: h,
        ..Frame::default()
    }
}

fn build_basic_registry() -> WidgetRegistry {
    let mut reg = WidgetRegistry::new();
    reg.register(ui_parent_frame());
    reg.register(button_frame());
    reg.register(texture_frame());
    reg.register(hidden_frame());
    reg
}

fn ui_parent_frame() -> Frame {
    let mut uip = make_frame(1, None, 1024.0, 768.0);
    uip.name = Some("UIParent".to_string());
    uip.children = vec![10, 11];
    uip
}

fn button_frame() -> Frame {
    let mut btn = make_frame(10, Some(1), 200.0, 36.0);
    btn.name = Some("MyButton".to_string());
    btn.visible = true;
    btn.anchors = vec![Anchor::from_relative_id(
        AnchorPoint::Center,
        None,
        AnchorPoint::Center,
    )];
    btn.children = vec![20];
    btn.children_keys.insert("Icon".to_string(), 20);
    btn
}

fn texture_frame() -> Frame {
    let mut tex = make_frame(20, Some(10), 32.0, 32.0);
    tex.widget_type = WidgetType::Texture;
    tex.name = Some("__tex_123".to_string());
    tex.visible = true;
    tex.texture = Some("Interface/Icons/foo".to_string());
    tex.tex_coords = Some((0.1, 0.9, 0.2, 0.8));
    tex.atlas_tex_coords = Some((0.0, 1.0, 0.0, 1.0));
    tex.anchors = vec![Anchor::from_relative_id(
        AnchorPoint::Center,
        None,
        AnchorPoint::Center,
    )];
    tex
}

fn hidden_frame() -> Frame {
    let mut hidden = make_frame(11, Some(1), 100.0, 50.0);
    hidden.name = Some("HiddenFrame".to_string());
    hidden.visible = false;
    hidden
}

// ── strip_wow_escapes ───────────────────────────────────────

#[test]
fn test_strip_plain_text() {
    assert_eq!(strip_wow_escapes("Hello World"), "Hello World");
}

#[test]
fn test_strip_color_codes() {
    assert_eq!(strip_wow_escapes("|cff00ff00Green|r Text"), "Green Text");
}

#[test]
fn test_strip_texture_escape() {
    assert_eq!(
        strip_wow_escapes("Before |TInterface/Icons/foo:16|t After"),
        "Before  After"
    );
}

#[test]
fn test_strip_hyperlink() {
    assert_eq!(strip_wow_escapes("|Hitem:12345|h[Sword]|h"), "[Sword]");
}

#[test]
fn test_strip_nested_escapes() {
    assert_eq!(
        strip_wow_escapes("|cffff0000|Hspell:1234|hFireball|h|r"),
        "Fireball"
    );
}

// ── build_tree integration ──────────────────────────────────

#[test]
fn test_build_tree_includes_children() {
    let reg = build_basic_registry();
    let names: Vec<String> = vec![];
    let lines = build_tree(&reg, &names, None, None, false, false, 1024.0, 768.0);
    let has_button = lines.iter().any(|l| l.contains("MyButton"));
    let has_icon = lines.iter().any(|l| l.contains(".Icon"));
    assert!(has_button, "Should contain MyButton");
    assert!(has_icon, "Should contain .Icon (parentKey)");
}

#[test]
fn test_build_tree_filter() {
    let reg = build_basic_registry();
    let names: Vec<String> = vec![];
    let lines = build_tree(
        &reg,
        &names,
        Some("MyButton"),
        None,
        false,
        false,
        1024.0,
        768.0,
    );
    assert!(lines.iter().any(|l| l.contains("MyButton")));
    assert!(!lines.iter().any(|l| l.contains("HiddenFrame")));
}

#[test]
fn test_build_tree_visible_only() {
    let reg = build_basic_registry();
    let names: Vec<String> = vec![];
    let lines = build_tree(&reg, &names, None, None, true, false, 1024.0, 768.0);
    assert!(!lines.iter().any(|l| l.contains("HiddenFrame")));
}

#[test]
fn test_build_tree_shows_texture_path() {
    let reg = build_basic_registry();
    let names: Vec<String> = vec![];
    let lines = build_tree(&reg, &names, None, None, false, false, 1024.0, 768.0);
    assert!(
        lines
            .iter()
            .any(|l| l.contains("[texture] Interface/Icons/foo"))
    );
}

#[test]
fn test_build_tree_shows_anchor_lines() {
    let reg = build_basic_registry();
    let names: Vec<String> = vec![];
    let lines = build_tree(&reg, &names, None, None, false, false, 1024.0, 768.0);
    assert!(lines.iter().any(|l| l.contains("[anchor]")));
}

#[test]
fn test_build_tree_verbose_shows_texture_rect_and_uv_coords() {
    let reg = build_basic_registry();
    let names: Vec<String> = vec![];
    let lines = build_tree(&reg, &names, None, None, false, true, 1024.0, 768.0);
    let texture_line = lines
        .iter()
        .find(|line| line.contains("[texture] Interface/Icons/foo"))
        .expect("texture line should exist");

    assert!(texture_line.contains("rect=("));
    assert!(texture_line.contains("tex_coords=(0.100,0.900,0.200,0.800)"));
    assert!(texture_line.contains("atlas_tex_coords=(0.000,1.000,0.000,1.000)"));
}

#[test]
fn test_build_tree_default_hides_verbose_texture_coords() {
    let reg = build_basic_registry();
    let names: Vec<String> = vec![];
    let lines = build_tree(&reg, &names, None, None, false, false, 1024.0, 768.0);
    let texture_line = lines
        .iter()
        .find(|line| line.contains("[texture] Interface/Icons/foo"))
        .expect("texture line should exist");

    assert!(!texture_line.contains("rect=("));
    assert!(!texture_line.contains("tex_coords=("));
    assert!(!texture_line.contains("atlas_tex_coords=("));
}

#[test]
fn test_build_warning_dump_includes_header() {
    let reg = build_basic_registry();
    let names: Vec<String> = vec![];
    let lines = build_warning_dump(&reg, &names, 1024.0, 768.0);
    assert!(lines[0].contains("Frame Dump"));
    assert!(lines[1].contains("1024x768"));
}
