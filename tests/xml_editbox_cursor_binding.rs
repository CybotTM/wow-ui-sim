//! XML-to-public-script binding only; no cursor geometry or producer assertions.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::{XmlElement, parse_xml, register_template};

#[test]
fn runtime_template_editbox_child_binds_cursor_changed_and_text_changed() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function XmlCursorBinding_OnCursorChanged(self, x, y, width, height) end
        function XmlCursorBinding_OnTextChanged(self, userInput) end
        "#,
    )
    .unwrap();

    let ui = parse_xml(
        r#"<Ui>
            <Frame name="XmlCursorBindingTemplate" virtual="true">
                <Frames>
                    <EditBox parentKey="Input">
                        <Scripts>
                            <OnCursorChanged function="XmlCursorBinding_OnCursorChanged"/>
                            <OnTextChanged function="XmlCursorBinding_OnTextChanged"/>
                        </Scripts>
                    </EditBox>
                </Frames>
            </Frame>
        </Ui>"#,
    )
    .unwrap();
    let XmlElement::Frame(template) = &ui.elements[0] else {
        panic!("expected virtual Frame fixture");
    };
    register_template("XmlCursorBindingTemplate", "Frame", template.clone());

    env.exec(
        r#"
        local parent = CreateFrame("Frame", nil, UIParent, "XmlCursorBindingTemplate")
        local child = parent.Input
        assert(child and child:IsObjectType("EditBox"), "XML EditBox child missing")
        local cursorHandler = child:GetScript("OnCursorChanged")
        local textHandler = child:GetScript("OnTextChanged")
        assert(type(cursorHandler) == "function", "XML OnCursorChanged not bound")
        assert(cursorHandler == XmlCursorBinding_OnCursorChanged, "wrong cursor handler")
        assert(type(textHandler) == "function", "XML OnTextChanged not bound")
        assert(textHandler == XmlCursorBinding_OnTextChanged, "wrong text handler")
        "#,
    )
    .unwrap();
}
