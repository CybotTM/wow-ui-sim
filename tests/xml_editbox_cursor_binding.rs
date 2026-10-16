//! XML cursor handler identity and named arguments at real layout updates.

use std::cell::RefCell;
use std::rc::Rc;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::font::{DEFAULT_WOW_FONT, WowFontSystem};
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

const NAMED_CURSOR_XML: &str = r#"
<Ui>
    <EditBox name="NamedCursorFixture" parent="UIParent" virtual="__VIRTUAL__">
        <Scripts><OnCursorChanged>
            CursorArgs[#CursorArgs + 1] = {x, y, w, h, select('#', ...),
                self:GetCursorPosition(), self:GetUTF8CursorPosition()}
            self.cursorOffset = -y
        </OnCursorChanged></Scripts>
    </EditBox>
</Ui>
"#;

fn named_cursor_env(virtual_frame: bool) -> WowLuaEnv {
    let fonts = WowFontSystem::new_without_casc();
    assert!(
        fonts.font_system.db().faces().next().is_some(),
        "installed font face required"
    );
    assert!(
        fonts.family_name(Some(DEFAULT_WOW_FONT)).is_some(),
        "default font alias required"
    );
    let env = WowLuaEnv::new().unwrap();
    env.set_font_system(Rc::new(RefCell::new(fonts)));
    env.exec("CursorArgs = {}; x, y, w, h = 901, 902, 903, 904")
        .unwrap();
    let root = tempfile::tempdir().unwrap();
    let toc = root.path().join("CursorArgs.toc");
    std::fs::write(&toc, "## Title: Cursor arguments\nArgs.xml\n").unwrap();
    let xml = NAMED_CURSOR_XML.replace("__VIRTUAL__", &virtual_frame.to_string());
    std::fs::write(root.path().join("Args.xml"), xml).unwrap();
    let loaded = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    assert!(loaded.warnings.is_empty(), "{:?}", loaded.warnings);
    env
}

fn assert_named_cursor_layout(env: &WowLuaEnv, frame: &str) {
    env.exec(&format!(
        r#"
        NamedCursorBox = {frame}
        local box = NamedCursorBox
        assert(type(box:GetScript('OnCursorChanged')) == 'function')
        box:SetAutoFocus(false)
        box:SetMultiLine(true)
        box:SetSize(320, 240)
        box:SetPoint('TOPLEFT', UIParent, 'TOPLEFT', 31, -47)
        assert(box:SetFont('Fonts\\FRIZQT__.TTF', 20))
        box:SetJustifyH('LEFT'); box:SetJustifyV('TOP')
        box:SetTextInsets(5, 7, 3, 4)
        box:SetText('é\n猫z'); box:SetCursorPosition(0)
        assert(#CursorArgs == 0, 'XML cursor handler must wait for actual layout')
    "#
    ))
    .unwrap();
    env.fire_on_update(0.016).unwrap();
    assert_named_cursor_record(env, 1, -3, 0, 0);
    env.exec("NamedCursorBox:SetCursorPosition(3); assert(#CursorArgs == 1)")
        .unwrap();
    env.fire_on_update(0.016).unwrap();
    assert_named_cursor_record(env, 2, -27, 3, 2);
}

fn assert_named_cursor_record(env: &WowLuaEnv, count: i32, y: i32, byte: i32, scalar: i32) {
    let state = env.state().borrow();
    assert!(state.lua_errors.is_empty(), "{:?}", state.lua_errors);
    drop(state);
    env.exec(&format!(
        r#"
        assert(#CursorArgs == {count}, 'one XML callback per changed layout')
        local record = CursorArgs[{count}]
        assert(record[1] == 5 and record[2] == {y}, 'named x/y must shadow globals')
        assert(record[3] == 1.5 and record[4] == 24, 'named w/h must use actual geometry')
        assert(record[5] == 4, 'XML locals must preserve four varargs')
        assert(record[6] == {byte} and record[7] == {scalar}, 'public UTF-8 cursor state')
        assert(NamedCursorBox.cursorOffset == -({y}), 'inline cursor arithmetic uses local y')
        assert(x == 901 and y == 902 and w == 903 and h == 904, 'globals remain unchanged')
    "#
    ))
    .unwrap();
}

#[test]
fn ordinary_xml_cursor_names_shadow_globals_during_real_layout() {
    let env = named_cursor_env(false);
    assert_named_cursor_layout(&env, "NamedCursorFixture");
}

#[test]
fn runtime_xml_cursor_names_shadow_globals_during_real_layout() {
    let env = named_cursor_env(true);
    assert_named_cursor_layout(
        &env,
        "CreateFrame('EditBox', nil, UIParent, 'NamedCursorFixture')",
    );
}
