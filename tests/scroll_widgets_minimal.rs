//! Tests for MinimalScrollBar and unnamed scroll widget template regressions.

use crate::common;

use common::env_with_shared_xml;
use wow_ui_sim::xml::{XmlElement, parse_xml, register_template};

const CHATFRAME_SCROLLBAR_HOST_XML: &str = r#"
    <Ui>
        <Frame name="ChatFrameScrollBarHostTemplate" virtual="true">
            <Frames>
                <EventFrame parentKey="ScrollBar" inherits="MinimalScrollBar">
                    <Anchors>
                        <Anchor point="TOPRIGHT"/>
                        <Anchor point="BOTTOMRIGHT"/>
                    </Anchors>
                </EventFrame>
            </Frames>
        </Frame>
    </Ui>
"#;

const INLINE_REAPPLY_HOST_XML: &str = r#"
    <Ui>
        <Button name="InlineReapplyBaseButtonTemplate" virtual="true">
            <Size x="16" y="16"/>
        </Button>
        <Frame name="InlineReapplyHostTemplate" virtual="true">
            <Layers>
                <Layer level="BACKGROUND">
                    <Texture name="$parentBackground" parentKey="Background"/>
                </Layer>
            </Layers>
            <Frames>
                <Button name="$parentResizeButton" parentKey="ResizeButton" inherits="InlineReapplyBaseButtonTemplate">
                    <Anchors>
                        <Anchor point="BOTTOMRIGHT" relativeTo="$parentBackground"/>
                    </Anchors>
                </Button>
            </Frames>
        </Frame>
    </Ui>
"#;

fn register_chatframe_scrollbar_host_template() {
    let ui = parse_xml(CHATFRAME_SCROLLBAR_HOST_XML).unwrap();
    let XmlElement::Frame(frame) = &ui.elements[0] else {
        panic!("expected frame template");
    };
    register_template("ChatFrameScrollBarHostTemplate", "Frame", frame.clone());
}

fn register_inline_reapply_host_template() {
    let ui = parse_xml(INLINE_REAPPLY_HOST_XML).unwrap();
    let XmlElement::Button(base) = &ui.elements[0] else {
        panic!("expected button template");
    };
    register_template("InlineReapplyBaseButtonTemplate", "Button", base.clone());

    let XmlElement::Frame(host) = &ui.elements[1] else {
        panic!("expected host template");
    };
    register_template("InlineReapplyHostTemplate", "Frame", host.clone());
}

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

    let (middle_atlas, middle_horiz_tile, middle_vert_tile): (String, bool, bool) = env
        .eval(
            r#"
            return TestMinSBTrackTex.Track.Middle:GetAtlas() or '',
                TestMinSBTrackTex.Track.Middle:GetHorizTile(),
                TestMinSBTrackTex.Track.Middle:GetVertTile()
            "#,
        )
        .unwrap();
    assert_eq!(
        middle_atlas, "!minimal-scrollbar-track-middle",
        "Track.Middle atlas"
    );
    assert!(
        !middle_horiz_tile,
        "Track.Middle should not be horizontally tiled"
    );
    assert!(
        middle_vert_tile,
        "Track.Middle atlas metadata should enable vertical tiling"
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

#[test]
fn test_inherited_minimal_scrollbar_child_keeps_inline_outer_anchors() {
    let env = env_with_shared_xml();
    register_chatframe_scrollbar_host_template();

    let result: String = env
        .eval(
            r#"
        local host = CreateFrame("Frame", "TestChatFrameScrollBarHost", UIParent, "ChatFrameScrollBarHostTemplate")
        local bar = host.ScrollBar
        if not bar then
            return "missing scrollbar"
        end
        local track = bar.Track
        if not track then
            return "missing track"
        end

        local function collect(frame)
            local out = {}
            for i = 1, frame:GetNumPoints() do
                local point, rel, relPoint, x, y = frame:GetPoint(i)
                local relName = "$parent"
                if rel then
                    if frame == bar and rel == host then
                        relName = "$parent"
                    elseif frame == track and rel == bar then
                        relName = "$parent"
                    else
                        relName = rel:GetName() or "<unnamed>"
                    end
                end
                table.insert(out, string.format("%s->%s:%s(%.0f,%.0f)", point, relName, relPoint, x, y))
            end
            table.sort(out)
            return table.concat(out, " | ")
        end

        return collect(bar) .. " || " .. collect(track)
    "#,
        )
        .unwrap();

    assert_eq!(
        result,
        "BOTTOMRIGHT->$parent:BOTTOMRIGHT(0,0) | TOPRIGHT->$parent:TOPRIGHT(0,0) || BOTTOM->$parent:BOTTOM(0,19) | TOP->$parent:TOP(0,-19)",
        "outer scrollbar anchors should stay TOPRIGHT/BOTTOMRIGHT while Track keeps TOP/BOTTOM: {result}"
    );
}

#[test]
fn test_reapplied_inline_child_anchor_uses_actual_parent_for_dollar_parent_lookup() {
    let env = env_with_shared_xml();
    register_inline_reapply_host_template();

    let relative_name: String = env
        .eval(
            r#"
        local host = CreateFrame("Frame", "TestInlineReapply", UIParent, "InlineReapplyHostTemplate")
        local _, rel = host.ResizeButton:GetPoint(1)
        return rel and rel:GetName() or "nil"
    "#,
        )
        .unwrap();

    assert_eq!(
        relative_name, "TestInlineReapplyBackground",
        "reapplied child anchors should resolve $parent against the actual parent frame"
    );
}
