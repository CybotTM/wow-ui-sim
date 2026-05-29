pub(super) fn register_all() {
    register_mists_templates();
}

#[cfg(feature = "client-mists")]
fn register_mists_templates() {
    register_mists_legacy_item_button_template();
}

#[cfg(not(feature = "client-mists"))]
fn register_mists_templates() {}

#[cfg(feature = "client-mists")]
fn register_mists_legacy_item_button_template() {
    let template = parse_mists_legacy_item_button_template();
    super::template::register_template("ItemButtonTemplate", "Button", template);
}

#[cfg(feature = "client-mists")]
fn parse_mists_legacy_item_button_template() -> super::FrameXml {
    let ui = super::parse_xml(MISTS_LEGACY_ITEM_BUTTON_TEMPLATE_XML)
        .expect("Mists legacy ItemButtonTemplate should parse");
    ui.elements
        .into_iter()
        .find_map(button_frame)
        .expect("Mists legacy ItemButtonTemplate XML should contain a Button template")
}

#[cfg(feature = "client-mists")]
fn button_frame(element: super::XmlElement) -> Option<super::FrameXml> {
    match element {
        super::XmlElement::Button(frame) => Some(frame),
        _ => None,
    }
}

#[cfg(feature = "client-mists")]
const MISTS_LEGACY_ITEM_BUTTON_TEMPLATE_XML: &str = r#"
<Ui>
  <Button name="ItemButtonTemplate" virtual="true">
    <Size x="37" y="37"/>
    <Layers>
      <Layer level="BORDER">
        <Texture name="$parentIconTexture" parentKey="icon"/>
      </Layer>
      <Layer level="ARTWORK">
        <FontString name="$parentCount" inherits="NumberFontNormal" justifyH="RIGHT" hidden="true" parentKey="Count"/>
      </Layer>
      <Layer level="OVERLAY" textureSubLevel="4">
        <Texture name="$parentSearchOverlay" parentKey="searchOverlay" setAllPoints="true" hidden="true"/>
      </Layer>
    </Layers>
    <NormalTexture name="$parentNormalTexture" parentKey="NormalTexture" file="Interface\Buttons\UI-Quickslot2">
      <Size x="64" y="64"/>
      <Anchors>
        <Anchor point="CENTER" x="0" y="-1"/>
      </Anchors>
    </NormalTexture>
    <PushedTexture parentKey="PushedTexture" file="Interface\Buttons\UI-Quickslot-Depress"/>
    <HighlightTexture parentKey="HighlightTexture" file="Interface\Buttons\ButtonHilight-Square" alphaMode="ADD"/>
  </Button>
</Ui>
"#;
