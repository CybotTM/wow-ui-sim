use wow_ui_sim::loader::{LoadTiming, create_frame_from_xml};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::{XmlElement, parse_xml_file};

fn focused_editbox() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        arrowBox = CreateFrame("EditBox", "ArrowInputBox", UIParent)
        arrowBox:SetText("abcd")
        arrowBox:SetCursorPosition(2)
        arrowBox:SetFocus()
        arrowCalls = {}
        "#,
    )
    .unwrap();
    env
}

#[test]
fn editbox_arrow_pressed_accepts_set_hook_get_has_and_clearing() {
    let env = focused_editbox();
    env.exec(
        r#"
        assert(arrowBox:HasScript("OnArrowPressed"))
        assert(arrowBox:GetScript("OnArrowPressed") == nil)
        local handler = function(self, key)
            assert(self == arrowBox)
            table.insert(arrowCalls, "set:" .. key)
        end
        arrowBox:SetScript("OnArrowPressed", handler)
        assert(arrowBox:GetScript("OnArrowPressed") == handler)
        -- EllesmereUI's deferred options initialization uses this HookScript pattern.
        C_Timer.After(0, function()
            arrowBox:HookScript("OnArrowPressed", function(self, key)
                assert(self == arrowBox)
                table.insert(arrowCalls, "hook:" .. key)
            end)
        end)
        "#,
    )
    .unwrap();
    env.process_timers().unwrap();
    env.send_key_press("UP", None).unwrap();
    assert_eq!(
        env.eval::<String>("return table.concat(arrowCalls, ',')")
            .unwrap(),
        "set:UP,hook:UP"
    );
    env.exec("arrowBox:SetScript('OnArrowPressed', nil); assert(arrowBox:GetScript('OnArrowPressed') == nil)").unwrap();
    assert!(
        !env.eval::<bool>("return CreateFrame('Frame'):HasScript('OnArrowPressed')")
            .unwrap()
    );
}

#[test]
fn editbox_arrow_pressed_dispatches_four_keys_and_preserves_cursor_editing() {
    let env = focused_editbox();
    env.exec(
        r#"
        assert(arrowBox:GetAltArrowKeyMode() == false)
        arrowBox:SetScript("OnArrowPressed", function(self, key, ...)
            assert(self == arrowBox and select('#', ...) == 0)
            table.insert(arrowCalls, key)
        end)
        "#,
    )
    .unwrap();
    for (key, cursor) in [("LEFT", 1), ("RIGHT", 2), ("UP", 2), ("DOWN", 2)] {
        env.send_key_press(key, None).unwrap();
        assert_eq!(
            env.eval::<i32>("return arrowBox:GetCursorPosition()")
                .unwrap(),
            cursor
        );
    }
    env.send_key_press("HOME", None).unwrap();
    env.send_key_press("END", None).unwrap();
    assert_eq!(
        env.eval::<String>("return table.concat(arrowCalls, ',')")
            .unwrap(),
        "LEFT,RIGHT,UP,DOWN"
    );
    assert_eq!(
        env.eval::<String>("return arrowBox:GetText()").unwrap(),
        "abcd"
    );
}

#[test]
fn editbox_arrow_pressed_alt_mode_requires_alt_and_releases_plain_arrows_to_bindings() {
    let env = focused_editbox();
    env.exec(
        r#"
        arrowBox:SetAltArrowKeyMode(true)
        arrowBox:SetScript("OnArrowPressed", function(_, key)
            table.insert(arrowCalls, key)
        end)
        bindingCalls = 0
        SetBinding("LEFT", "TOGGLEBACKPACK")
        function ToggleBackpack() bindingCalls = bindingCalls + 1 end
        "#,
    )
    .unwrap();
    env.send_key_press("LEFT", None).unwrap();
    assert_eq!(
        env.eval::<(i32, i32, i32)>(
            "return #arrowCalls, arrowBox:GetCursorPosition(), bindingCalls"
        )
        .unwrap(),
        (0, 2, 1)
    );
    env.send_key_press("ALT-LEFT", None).unwrap();
    assert_eq!(
        env.eval::<(String, i32)>("return arrowCalls[1], arrowBox:GetCursorPosition()")
            .unwrap(),
        ("LEFT".into(), 1)
    );
    env.exec("A_Admin.SetAltKeyDown(true)").unwrap();
    env.send_key_press("RIGHT", None).unwrap();
    env.exec("A_Admin.SetAltKeyDown(false); arrowBox:SetAltArrowKeyMode(false)")
        .unwrap();
    env.send_key_press("LEFT", None).unwrap();
    assert_eq!(
        env.eval::<String>("return table.concat(arrowCalls, ',')")
            .unwrap(),
        "LEFT,RIGHT,LEFT"
    );
    env.exec("arrowBox:ClearFocus()").unwrap();
    env.send_key_press("DOWN", None).unwrap();
    assert_eq!(env.eval::<i32>("return #arrowCalls").unwrap(), 3);
}

#[test]
fn editbox_arrow_pressed_original_blizzard_xml_and_autocomplete_consumer() {
    let env = WowLuaEnv::new().unwrap();
    let root = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    for path in [
        "Blizzard_SharedXMLBase/Compat.lua",
        "Blizzard_AutoComplete/AutoComplete.lua",
    ] {
        let lua = std::fs::read_to_string(root.join(path)).unwrap();
        env.exec(&lua).unwrap();
    }
    let ui = parse_xml_file(&root.join("Blizzard_AutoComplete/AutoComplete.xml")).unwrap();
    let template = ui
        .elements
        .iter()
        .find_map(|element| match element {
            XmlElement::EditBox(frame)
                if frame.name.as_deref() == Some("AutoCompleteEditBoxTemplate") =>
            {
                Some(frame)
            }
            _ => None,
        })
        .expect("original Blizzard EditBox template");
    create_frame_from_xml(
        &env.loader_env(),
        template,
        "EditBox",
        None,
        None,
        None,
        &mut LoadTiming::default(),
    )
    .unwrap();
    env.exec(
        r#"
        arrowBox = CreateFrame("EditBox", "OriginalArrowConsumer", UIParent, "AutoCompleteEditBoxTemplate")
        assert(arrowBox:GetScript("OnArrowPressed") == AutoCompleteEditBox_OnArrowPressed)
        AutoCompleteBox = CreateFrame("Frame", nil, UIParent)
        AutoCompleteBox.parent = arrowBox
        AutoCompleteBox.numResults = 3
        for i = 1, AUTOCOMPLETE_MAX_BUTTONS do
            _G["AutoCompleteButton" .. i] = CreateFrame("Button", nil, AutoCompleteBox)
        end
        AutoComplete_SetSelectedIndex(AutoCompleteBox, 1)
        AutoCompleteBox:Show()
        arrowBox:SetFocus()
        "#,
    )
    .unwrap();
    for (key, selected) in [("DOWN", 2), ("UP", 1), ("UP", 3)] {
        env.send_key_press(key, None).unwrap();
        assert_eq!(
            env.eval::<i32>("return AutoComplete_GetSelectedIndex(AutoCompleteBox)")
                .unwrap(),
            selected
        );
    }
}
