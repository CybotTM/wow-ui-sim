use super::*;

#[test]
fn test_account_store_icon_card_sets_icon_size() {
    let env = WowLuaEnv::new().unwrap();
    let lua = extract_icon_card_update_lua();
    env.exec(&lua).unwrap();
    run_icon_card_update(&env);

    let texture: String = env.eval("return TEST_ICON_CALLS.texture").unwrap();
    let width: Option<f64> = env.eval("return TEST_ICON_CALLS.width").unwrap();
    let height: Option<f64> = env.eval("return TEST_ICON_CALLS.height").unwrap();

    assert_eq!(texture, r"Interface\Icons\INV_Misc_QuestionMark");
    assert_eq!(width, Some(64.0));
    assert_eq!(height, Some(64.0));
}

fn extract_icon_card_update_lua() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("Interface/BlizzardUI/Blizzard_AccountStore/Blizzard_AccountStoreCardTemplates.lua");
    let source = std::fs::read_to_string(path).unwrap();
    let start = source.find("AccountStoreIconCardMixin = {};").unwrap();
    let end = source.find("AccountStoreTransmogSetCardMixin = {};").unwrap();
    source[start..end].to_string()
}

fn run_icon_card_update(env: &WowLuaEnv) {
    env.exec(
        r#"
        TEST_ICON_CALLS = {}
        local icon = {
            SetTexture = function(_, texture) TEST_ICON_CALLS.texture = texture end,
            SetSize = function(_, w, h)
                TEST_ICON_CALLS.width = w
                TEST_ICON_CALLS.height = h
            end,
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
