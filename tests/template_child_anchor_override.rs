//! Regression test: inline anchors on a child frame that also has `inherits=`
//! must REPLACE (not be overridden by) anchors from the inherited template.
//!
//! Mirrors the real-world case in WorldMapFrameTemplate:
//!   MapCanvasFrameScrollContainerTemplate → defines TOPLEFT + BOTTOMRIGHT
//!   WorldMapFrameTemplate defines a child:
//!     <ScrollFrame parentKey="ScrollContainer"
//!                  inherits="MapCanvasFrameScrollContainerTemplate">
//!       <Anchors>
//!         <Anchor point="TOPLEFT" .../>
//!         <Anchor point="BOTTOMLEFT" .../>
//!         <Anchor point="RIGHT" .../>
//!       </Anchors>
//!     </ScrollFrame>
//!
//! The inline anchors (TOPLEFT, BOTTOMLEFT, RIGHT) should win.
//! Bug: anchors from the inherited template (BOTTOMRIGHT) are applied AFTER
//! the inline anchors, leaving all 4 anchors on the child instead of 3.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::{XmlElement, clear_templates, parse_xml, register_template};

/// A template whose child will be inherited by the test child frame.
/// Sets TOPLEFT + BOTTOMRIGHT on the frame itself (not via child — this template
/// is used as the `inherits=` of the child, same as MapCanvasFrameScrollContainerTemplate).
const SCROLL_CONTAINER_TEMPLATE_XML: &str = r#"
    <Ui>
        <Frame name="BaseScrollContainerTemplate" virtual="true">
            <Anchors>
                <Anchor point="TOPLEFT"/>
                <Anchor point="BOTTOMRIGHT"/>
            </Anchors>
        </Frame>
    </Ui>
"#;

/// Parent template containing a child with:
///   - `inherits="BaseScrollContainerTemplate"` (brings TOPLEFT + BOTTOMRIGHT)
///   - Inline anchors: TOPLEFT + BOTTOMLEFT + RIGHT  (should replace the inherited ones)
const PARENT_TEMPLATE_XML: &str = r#"
    <Ui>
        <Frame name="ParentTemplate" virtual="true">
            <Size x="800" y="600"/>
            <Frames>
                <Frame parentKey="ScrollContainer" inherits="BaseScrollContainerTemplate">
                    <Anchors>
                        <Anchor point="TOPLEFT"/>
                        <Anchor point="BOTTOMLEFT"/>
                        <Anchor point="RIGHT"/>
                    </Anchors>
                </Frame>
            </Frames>
        </Frame>
    </Ui>
"#;

fn setup_env() -> WowLuaEnv {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    for (name, xml) in [
        ("BaseScrollContainerTemplate", SCROLL_CONTAINER_TEMPLATE_XML),
        ("ParentTemplate", PARENT_TEMPLATE_XML),
    ] {
        let ui = parse_xml(xml).unwrap();
        if let XmlElement::Frame(frame) = &ui.elements[0] {
            register_template(name, "Frame", frame.clone());
        }
    }

    env
}

/// A child frame defined with both `inherits=` (template with TOPLEFT+BOTTOMRIGHT)
/// and inline anchors (TOPLEFT+BOTTOMLEFT+RIGHT) should end up with exactly the
/// 3 inline anchors. The template's BOTTOMRIGHT must not survive.
#[test]
fn child_inline_anchors_replace_inherited_template_anchors() {
    let env = setup_env();

    env.exec(
        r#"CreateFrame("Frame", "TestParentFrame", UIParent, "ParentTemplate")"#,
    )
    .unwrap();

    let result: String = env
        .eval(
            r#"
            local container = TestParentFrame.ScrollContainer
            if not container then
                return "ScrollContainer is nil"
            end

            local n = container:GetNumPoints()

            -- Collect anchor names
            local found = {}
            local list = {}
            for i = 1, n do
                local point = container:GetPoint(i)
                found[point] = true
                table.insert(list, point)
            end

            -- BOTTOMRIGHT comes from the inherited template; inline anchors should replace it
            if found["BOTTOMRIGHT"] then
                return "BOTTOMRIGHT must not be present (template anchor must be replaced by inline anchors). Got " .. n .. " anchors: " .. table.concat(list, ", ")
            end
            if n ~= 3 then
                return "expected 3 anchors, got " .. n .. ": " .. table.concat(list, ", ")
            end
            if not found["TOPLEFT"] then
                return "TOPLEFT should be present"
            end
            if not found["BOTTOMLEFT"] then
                return "BOTTOMLEFT should be present"
            end
            if not found["RIGHT"] then
                return "RIGHT should be present"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "child inline anchors must replace inherited template anchors: {}",
        result
    );
}
