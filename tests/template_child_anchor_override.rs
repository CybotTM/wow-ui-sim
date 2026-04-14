//! Regressions for inherited child-frame anchor behavior.
//!
//! WoW XML applies inherited anchors first and then the child's inline anchors.
//! Inline anchors should override conflicting inherited points, but they should
//! not wipe inherited anchors that still contribute constraints.
//!
//! Two Blizzard patterns matter here:
//! - `WorldMapFrameTemplate.ScrollContainer` inherits `TOPLEFT + BOTTOMRIGHT`
//!   and then adds `TOPLEFT + BOTTOMLEFT + RIGHT`
//! - `HeroTalentsContainer.ExpandedContainer.NodesContainer` inherits `TOP`
//!   and then adds `LEFT + BOTTOMRIGHT`
//!
//! The simulator regressed by clearing all inherited anchors before reapplying
//! inline child anchors, which drops the hero-tree `TOP` anchor and shifts the
//! hero talent node container too low.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::{XmlElement, clear_templates, parse_xml, register_template};

/// A template whose anchors mirror `MapCanvasFrameScrollContainerTemplate`.
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

/// Parent template containing a map-style child with inherited and inline
/// anchors on the same frame.
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

/// Base template mirroring `HeroTalentsTreeNodesContainerTemplate`.
const HERO_NODES_CONTAINER_TEMPLATE_XML: &str = r#"
    <Ui>
        <Frame name="HeroNodesContainerTemplate" virtual="true">
            <Anchors>
                <Anchor point="TOP" y="-90"/>
            </Anchors>
        </Frame>
    </Ui>
"#;

/// Host template mirroring `ExpandedContainer.NodesContainer`.
const HERO_NODES_HOST_TEMPLATE_XML: &str = r#"
    <Ui>
        <Frame name="HeroNodesHostTemplate" virtual="true">
            <Size x="284" y="362"/>
            <Frames>
                <Frame parentKey="NodesContainer" inherits="HeroNodesContainerTemplate">
                    <Anchors>
                        <Anchor point="LEFT" x="60"/>
                        <Anchor point="BOTTOMRIGHT" x="-60" y="60"/>
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
        (
            "HeroNodesContainerTemplate",
            HERO_NODES_CONTAINER_TEMPLATE_XML,
        ),
        ("HeroNodesHostTemplate", HERO_NODES_HOST_TEMPLATE_XML),
    ] {
        let ui = parse_xml(xml).unwrap();
        if let XmlElement::Frame(frame) = &ui.elements[0] {
            register_template(name, "Frame", frame.clone());
        }
    }

    env
}

/// World-map style child anchors should still produce the intended stretched
/// layout after template application. This covers the case where the child
/// reasserts `TOPLEFT` and adds `BOTTOMLEFT + RIGHT`.
#[test]
fn child_inline_anchors_keep_expected_world_map_scroll_layout() {
    let env = setup_env();

    env.exec(r#"CreateFrame("Frame", "TestParentFrame", UIParent, "ParentTemplate")"#)
        .unwrap();

    let result: (f64, f64, f64, f64, i32) = env
        .eval(
            r#"
            local container = TestParentFrame.ScrollContainer
            if not container then
                error("ScrollContainer is nil")
            end

            return container:GetLeft(), container:GetTop(), container:GetWidth(), container:GetHeight(), container:GetNumPoints()
            "#,
        )
        .unwrap();

    let (left, top, width, height, num_points) = result;
    assert_eq!(
        left, 0.0,
        "world-map style container should stay flush to parent left"
    );
    assert_eq!(
        top, 1200.0,
        "GetTop() uses WoW bottom-left coordinates, so a frame flush to the screen top should report the screen height"
    );
    assert_eq!(
        width, 800.0,
        "world-map style container should stretch to parent right"
    );
    assert_eq!(
        height, 600.0,
        "world-map style container should stretch to parent bottom"
    );
    assert!(
        num_points >= 3,
        "world-map style container should retain enough anchors to resolve all edges"
    );
}

/// Hero-tree style child anchors must preserve the inherited `TOP` anchor while
/// adding `LEFT + BOTTOMRIGHT`; otherwise the node container collapses downward.
#[test]
fn child_inline_anchors_preserve_inherited_hero_nodes_top_anchor() {
    let env = setup_env();

    env.exec(r#"CreateFrame("Frame", "TestHeroNodesHost", UIParent, "HeroNodesHostTemplate")"#)
        .unwrap();

    let result: String = env
        .eval(
            r#"
            local container = TestHeroNodesHost.NodesContainer
            if not container then
                return "NodesContainer is nil"
            end

            local found = {}
            local list = {}
            for i = 1, container:GetNumPoints() do
                local point = select(1, container:GetPoint(i))
                found[point] = true
                table.insert(list, point)
            end
            table.sort(list)

            if not found["TOP"] then
                return "missing inherited TOP anchor: " .. table.concat(list, ", ")
            end
            if not found["LEFT"] then
                return "missing inline LEFT anchor: " .. table.concat(list, ", ")
            end
            if not found["BOTTOMRIGHT"] then
                return "missing inline BOTTOMRIGHT anchor: " .. table.concat(list, ", ")
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "hero nodes container should keep inherited TOP and inline anchors: {result}"
    );
}
