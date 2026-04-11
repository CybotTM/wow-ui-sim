//! XML parsing for WoW UI definition files.

mod parse;
mod template;
mod types;
mod types_elements;
mod types_support;

// Re-export all public types and functions
pub use parse::{XmlLoadError, parse_xml, parse_xml_file};
pub use template::{
    TemplateEntry, TemplateInfo, anim_group_template_registry_read, clear_templates,
    collect_anim_group_mixins, collect_texture_mixins, get_template, get_template_chain,
    get_template_info, get_texture_template_size, register_anim_group_template,
    register_intrinsic_templates, register_template, register_texture_template,
    resolve_texture_inheritance,
};
pub use types::{FrameChildElement, FrameXml, ScopedModifierXml, UiXml, XmlElement};
pub use types_elements::{
    ActorXml, ActorsXml, AnimationElement, AnimationGroupXml, AnimationXml, FontFamilyMemberXml,
    FontFamilyXml, FontStringXml, FontXml, FrameElement, FramesXml, IncludeXml, LayerElement,
    LayerXml, LayersXml, ScriptXml, TextureXml, widget_type_for_tag,
};
pub use types_support::{
    AbsDimensionXml, AnchorXml, AnchorsXml, AnimationsXml, AttributeXml, AttributesXml,
    BackdropXml, BindingXml, ColorXml, FontRefXml, GradientXml, InsetsXml, KeyValueXml,
    KeyValuesXml, ModifiedClickXml, OffsetXml, ResizeBoundsXml, ScriptBodyXml, ScriptsXml,
    ScrollChildXml, SizeXml,
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

    #[test]
    fn test_parse_element_key_values() {
        let xml = r#"
            <Ui>
                <Frame name="TestFrame">
                    <Layers>
                        <Layer level="ARTWORK">
                            <FontString parentKey="Text">
                                <KeyValues>
                                    <KeyValue key="anchorSpacing" value="4" type="number"/>
                                </KeyValues>
                            </FontString>
                            <Texture parentKey="Icon">
                                <KeyValues>
                                    <KeyValue key="layoutIndex" value="2" type="number"/>
                                </KeyValues>
                            </Texture>
                        </Layer>
                    </Layers>
                </Frame>
            </Ui>
        "#;

        let ui = parse_xml(xml).unwrap();
        let frame = match &ui.elements[0] {
            XmlElement::Frame(frame) => frame,
            other => panic!("expected frame, got {:?}", other),
        };
        let layer = &frame.layers().next().unwrap().layers[0];
        let text = match &layer.elements[0] {
            LayerElement::FontString(text) => text,
            other => panic!("expected fontstring, got {:?}", other),
        };
        let texture = match &layer.elements[1] {
            LayerElement::Texture(texture) => texture,
            other => panic!("expected texture, got {:?}", other),
        };

        assert_eq!(
            text.key_values.as_ref().unwrap().values[0].key,
            "anchorSpacing"
        );
        assert_eq!(
            texture.key_values.as_ref().unwrap().values[0].key,
            "layoutIndex"
        );
    }
}
