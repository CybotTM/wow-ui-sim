use super::*;

#[test]
fn test_account_store_icon_card_sets_icon_size() {
    let env = WowLuaEnv::new().unwrap();
    let lua = extract_icon_card_update_lua();
    env.exec(&lua).unwrap();
    run_icon_card_update(&env);

    let texture: String = env.eval("return TEST_ICON_CALLS.texture").unwrap();

    assert_eq!(texture, r"Interface\Icons\INV_Misc_QuestionMark");
}

fn extract_icon_card_update_lua() -> String {
    let path = crate::paths::default_blizzard_ui_addons_path()
        .expect("Blizzard UI cache should be available")
        .join("Blizzard_AccountStore/Blizzard_AccountStoreCardTemplates.lua");
    let source = std::fs::read_to_string(path).unwrap();
    let start = source.find("AccountStoreIconCardMixin = {};").unwrap();
    let end = source
        .find("AccountStoreTransmogSetCardMixin = {};")
        .unwrap();
    source[start..end].to_string()
}

fn run_icon_card_update(env: &WowLuaEnv) {
    env.exec(
        r#"
        TEST_ICON_CALLS = {}
        local icon = {
            SetTexture = function(_, texture) TEST_ICON_CALLS.texture = texture end,
        }
        local self = {
            itemInfo = { displayIcon = "Interface\\Icons\\INV_Misc_QuestionMark" },
            Icon = icon,
        }
        AccountStoreIconCardMixin.UpdateCardDisplay(self)
        "#,
    )
    .unwrap();
}
