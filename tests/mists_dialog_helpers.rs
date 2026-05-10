#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

const MONEY_FRAME_LUA: &str =
    include_str!("../Interface/BlizzardUI/Mists/AddOns/Blizzard_MoneyFrame/Classic/MoneyFrame.lua");

fn load_money_frame_lua(env: &WowLuaEnv) {
    let loader = format!(
        r#"
        local source = [==[
{}
        ]==]
        local chunk = assert(loadstring(source))
        chunk("Blizzard_MoneyFrame", {{ MoneyTypeInfo = {{}} }})
        "#,
        MONEY_FRAME_LUA
    );

    env.exec(&loader)
        .expect("Mists MoneyFrame Lua should define money helpers");
}

#[test]
fn money_frame_set_type_reproduces_missing_basic_message_dialog_helper() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");
    load_money_frame_lua(&env);

    let (ok, err): (bool, String) = env
        .eval(
            r#"
            rawset(_G, "SetBasicMessageDialogText", nil)
            local ok, err = pcall(MoneyFrame_SetType, {}, "INVALID")
            return ok, tostring(err)
            "#,
        )
        .expect("MoneyFrame_SetType pcall should return a status");

    assert!(!ok, "MoneyFrame_SetType should reproduce the nil global");
    assert!(
        err.contains("SetBasicMessageDialogText"),
        "expected SetBasicMessageDialogText nil failure, got: {err}"
    );
}

#[test]
fn basic_message_dialog_helper_updates_text() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (String, i32, String, i32, String, i32) = env
        .eval(
            r#"
            local textValue = nil
            local shown = false
            local showCount = 0
            BasicMessageDialog = {
                IsShown = function()
                    return shown
                end,
                Show = function()
                    shown = true
                    showCount = showCount + 1
                end,
                Text = {
                    SetText = function(_self, text)
                        textValue = text
                    end,
                },
            }

            SetBasicMessageDialogText("Invalid money type: TEST")
            local firstText = textValue
            local firstShowCount = showCount

            SetBasicMessageDialogText("Ignored while shown")
            local secondText = textValue
            local secondShowCount = showCount

            SetBasicMessageDialogText("Forced replacement", true)
            local thirdText = textValue
            local thirdShowCount = showCount

            return firstText, firstShowCount, secondText, secondShowCount, thirdText, thirdShowCount
            "#,
        )
        .expect("SetBasicMessageDialogText should mutate BasicMessageDialog.Text");

    assert_eq!(
        result,
        (
            "Invalid money type: TEST".to_string(),
            1,
            "Invalid money type: TEST".to_string(),
            1,
            "Forced replacement".to_string(),
            2
        ),
        "SetBasicMessageDialogText should update and show only when hidden or forced"
    );
}
