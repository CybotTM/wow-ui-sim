//! XML parsing for WoW UI definition files.

mod parse;
mod template;
mod types;
mod types_elements;

// Re-export all public types and functions
pub use parse::{XmlLoadError, parse_xml, parse_xml_file};
pub use template::{
    TemplateEntry, TemplateInfo, anim_group_template_registry_read, clear_templates,
    collect_anim_group_mixins, collect_texture_mixins, get_template, get_template_chain,
    get_template_info, register_anim_group_template, register_intrinsic_templates,
    register_template, register_texture_template, resolve_texture_inheritance,
};
pub use types::{
    AbsDimensionXml, AnchorXml, AnchorsXml, AnimationsXml, AttributeXml, AttributesXml,
    BackdropXml, ColorXml, FontRefXml, FrameChildElement, FrameXml, GradientXml, InsetsXml,
    KeyValueXml, KeyValuesXml, OffsetXml, ResizeBoundsXml, ScopedModifierXml, ScriptBodyXml,
    ScriptsXml, ScrollChildXml, SizeXml, UiXml, XmlElement,
};
pub use types_elements::{
    ActorXml, ActorsXml, AnimationElement, AnimationGroupXml, AnimationXml, FontFamilyXml,
    FontStringXml, FontXml, FrameElement, FramesXml, IncludeXml, LayerElement, LayerXml, LayersXml,
    ScriptXml, TextureXml,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_frame() {
        let xml = r#"
            <Ui>
                <Frame name="TestFrame" parent="UIParent">
                    <Size x="200" y="100"/>
                    <Anchors>
                        <Anchor point="CENTER"/>
                    </Anchors>
                </Frame>
            </Ui>
        "#;

        let ui = parse_xml(xml).unwrap();
        assert_eq!(ui.elements.len(), 1);
    }
}
