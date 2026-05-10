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

    let message_text: String = env
        .eval(
            r#"
            local textValue = nil
            BasicMessageDialog = {
                Text = {
                    SetText = function(_self, text)
                        textValue = text
                    end,
                },
            }

            SetBasicMessageDialogText("Invalid money type: TEST")
            return textValue
            "#,
        )
        .expect("SetBasicMessageDialogText should mutate BasicMessageDialog.Text");

    assert_eq!(
        message_text, "Invalid money type: TEST",
        "SetBasicMessageDialogText should write the supplied text to BasicMessageDialog.Text"
    );
}
