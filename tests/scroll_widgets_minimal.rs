//! Tests for MinimalScrollBar and unnamed scroll widget template regressions.

mod common;

use common::env_with_shared_xml;

#[test]
fn test_minimal_scrollbar_child_structure() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local sb = CreateFrame("EventFrame", "TestMinSBStructure", UIParent, "MinimalScrollBar")
        sb:SetSize(8, 200)
    "#,
    )
    .unwrap();

    let has_track: bool = env.eval("return TestMinSBStructure.Track ~= nil").unwrap();
    assert!(has_track, "MinimalScrollBar should have Track child");

    let has_thumb: bool = env
        .eval("return TestMinSBStructure.Track.Thumb ~= nil")
        .unwrap();
    assert!(has_thumb, "Track should have Thumb child");

    let has_back: bool = env.eval("return TestMinSBStructure.Back ~= nil").unwrap();
    let has_forward: bool = env
        .eval("return TestMinSBStructure.Forward ~= nil")
        .unwrap();
    assert!(has_back, "MinimalScrollBar should have Back stepper");
    assert!(has_forward, "MinimalScrollBar should have Forward stepper");
}

#[test]
fn test_minimal_scrollbar_track_textures() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local sb = CreateFrame("EventFrame", "TestMinSBTrackTex", UIParent, "MinimalScrollBar")
        sb:SetSize(8, 200)
    "#,
    )
    .unwrap();

    let has_begin: bool = env
        .eval("return TestMinSBTrackTex.Track.Begin ~= nil")
        .unwrap();
    let has_middle: bool = env
        .eval("return TestMinSBTrackTex.Track.Middle ~= nil")
        .unwrap();
    let has_end: bool = env
        .eval("return TestMinSBTrackTex.Track.End ~= nil")
        .unwrap();

    assert!(has_begin, "Track should have Begin texture");
    assert!(has_middle, "Track should have Middle texture");
    assert!(has_end, "Track should have End texture");

    let begin_atlas: String = env
        .eval("return TestMinSBTrackTex.Track.Begin:GetAtlas() or ''")
        .unwrap();
    assert_eq!(
        begin_atlas, "minimal-scrollbar-track-top",
        "Track.Begin atlas"
    );

    let end_atlas: String = env
        .eval("return TestMinSBTrackTex.Track.End:GetAtlas() or ''")
        .unwrap();
    assert_eq!(
        end_atlas, "minimal-scrollbar-track-bottom",
        "Track.End atlas"
    );
}

#[test]
fn test_minimal_scrollbar_thumb_textures() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local sb = CreateFrame("EventFrame", "TestMinSBThumbTex", UIParent, "MinimalScrollBar")
        sb:SetSize(8, 200)
    "#,
    )
    .unwrap();

    let has_begin: bool = env
        .eval("return TestMinSBThumbTex.Track.Thumb.Begin ~= nil")
        .unwrap();
    let has_middle: bool = env
        .eval("return TestMinSBThumbTex.Track.Thumb.Middle ~= nil")
        .unwrap();
    let has_end: bool = env
        .eval("return TestMinSBThumbTex.Track.Thumb.End ~= nil")
        .unwrap();

    assert!(has_begin, "Thumb should have Begin texture");
    assert!(has_middle, "Thumb should have Middle texture");
    assert!(has_end, "Thumb should have End texture");

    let begin_atlas: String = env
        .eval("return TestMinSBThumbTex.Track.Thumb.Begin:GetAtlas() or ''")
        .unwrap();
    assert_eq!(
        begin_atlas, "minimal-scrollbar-small-thumb-top",
        "Thumb.Begin atlas"
    );
}

#[test]
fn test_minimal_scrollbar_keyvalues() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local sb = CreateFrame("EventFrame", "TestMinSBKeyValues", UIParent, "MinimalScrollBar")
        sb:SetSize(8, 200)
    "#,
    )
    .unwrap();

    let thumb_anchor: String = env
        .eval("return TestMinSBKeyValues.thumbAnchor or ''")
        .unwrap();
    assert_eq!(thumb_anchor, "TOP", "thumbAnchor should be TOP");

    let min_thumb: f64 = env
        .eval("return TestMinSBKeyValues.minThumbExtent or 0")
        .unwrap();
    assert_eq!(min_thumb, 23.0, "minThumbExtent should be 23");
}

#[test]
fn test_minimal_scrollbar_mixin_accessors() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local sb = CreateFrame("EventFrame", "TestMinSBAccessors", UIParent, "MinimalScrollBar")
        sb:SetSize(8, 200)
    "#,
    )
    .unwrap();

    let track_matches: bool = env
        .eval("return TestMinSBAccessors:GetTrack() == TestMinSBAccessors.Track")
        .unwrap();
    assert!(track_matches, "GetTrack() should return Track child");

    let thumb_matches: bool = env
        .eval("return TestMinSBAccessors:GetThumb() == TestMinSBAccessors.Track.Thumb")
        .unwrap();
    assert!(thumb_matches, "GetThumb() should return Track.Thumb");

    let back_matches: bool = env
        .eval("return TestMinSBAccessors:GetBackStepper() == TestMinSBAccessors.Back")
        .unwrap();
    assert!(back_matches, "GetBackStepper() should return Back");

    let forward_matches: bool = env
        .eval("return TestMinSBAccessors:GetForwardStepper() == TestMinSBAccessors.Forward")
        .unwrap();
    assert!(forward_matches, "GetForwardStepper() should return Forward");
}

#[test]
fn test_minimal_scrollbar_stepper_sizes() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local sb = CreateFrame("EventFrame", "TestMinSBSizes", UIParent, "MinimalScrollBar")
        sb:SetSize(8, 200)
    "#,
    )
    .unwrap();

    let back_w: f64 = env.eval("return TestMinSBSizes.Back:GetWidth()").unwrap();
    let back_h: f64 = env.eval("return TestMinSBSizes.Back:GetHeight()").unwrap();
    assert_eq!(back_w, 17.0, "Back button width");
    assert_eq!(back_h, 11.0, "Back button height");

    let fwd_w: f64 = env
        .eval("return TestMinSBSizes.Forward:GetWidth()")
        .unwrap();
    let fwd_h: f64 = env
        .eval("return TestMinSBSizes.Forward:GetHeight()")
        .unwrap();
    assert_eq!(fwd_w, 17.0, "Forward button width");
    assert_eq!(fwd_h, 11.0, "Forward button height");
}

#[test]
fn test_minimal_scrollbar_rust_children_keys() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local sb = CreateFrame("EventFrame", "TestMinSBRust", UIParent, "MinimalScrollBar")
        sb:SetSize(8, 200)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let registry = &state.widgets;

    let sb_id = registry.get_id_by_name("TestMinSBRust");
    assert!(sb_id.is_some(), "TestMinSBRust should exist in registry");
    let sb_id = sb_id.unwrap();

    let sb = registry.get(sb_id).unwrap();
    assert!(
        sb.children_keys.contains_key("Track"),
        "Rust children_keys should have Track"
    );
    assert!(
        sb.children_keys.contains_key("Back"),
        "Rust children_keys should have Back"
    );
    assert!(
        sb.children_keys.contains_key("Forward"),
        "Rust children_keys should have Forward"
    );

    let track_id = *sb.children_keys.get("Track").unwrap();
    let track = registry.get(track_id).unwrap();
    assert!(
        track.children_keys.contains_key("Thumb"),
        "Track's Rust children_keys should have Thumb"
    );
}

#[test]
fn test_unnamed_scrollbox_creates_child_frames() {
    let env = env_with_shared_xml();

    let has_shadows: bool = env
        .eval(
            r#"
        local sb = CreateFrame("Frame", nil, UIParent, "WowScrollBoxList")
        sb:SetSize(300, 400)
        sb:SetPoint("CENTER")
        return sb.Shadows ~= nil
    "#,
        )
        .unwrap();
    assert!(
        has_shadows,
        "Unnamed WowScrollBoxList should have Shadows child from template"
    );
}

#[test]
fn test_minimal_scrollbar_track_has_thumb() {
    let env = env_with_shared_xml();

    let result: String = env
        .eval(
            r#"
        local sb = CreateFrame("EventFrame", nil, UIParent, "MinimalScrollBar")
        local track = sb.Track
        local thumb = track and track.Thumb or nil
        return tostring(track ~= nil) .. "," .. tostring(thumb ~= nil)
    "#,
        )
        .unwrap();
    assert_eq!(
        result, "true,true",
        "MinimalScrollBar Track should have Thumb child"
    );
}

#[test]
fn test_scrollframe_template_creates_scrollbar_with_thumb() {
    let env = env_with_shared_xml();

    let result: String = env
        .eval(
            r#"
        local sf = CreateFrame("ScrollFrame", nil, UIParent, "ScrollFrameTemplate")
        local bar = sf.ScrollBar
        if not bar then return "no_scrollbar" end
        local track = bar.Track
        if not track then return "no_track" end
        local thumb = track.Thumb
        return tostring(thumb ~= nil)
    "#,
        )
        .unwrap();
    assert_eq!(
        result, "true",
        "ScrollFrameTemplate's ScrollBar.Track.Thumb should exist"
    );
}
