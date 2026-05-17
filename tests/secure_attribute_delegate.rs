//! Secure attribute delegate behavior used by Blizzard callback registries.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn forbidden_attribute_delegate_dispatches_securely_from_tainted_stack() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let secure_inside: bool = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            frame:SetForbidden()
            frame:SetScript("OnAttributeChanged", function()
                SECURE_INSIDE_ATTRIBUTE_DELEGATE = issecure()
            end)

            forceinsecure()
            frame:SetAttribute("insert-secure-event", true)
            return SECURE_INSIDE_ATTRIBUTE_DELEGATE
            "#,
        )
        .unwrap();

    assert!(
        secure_inside,
        "forbidden attribute delegates should execute OnAttributeChanged securely"
    );
}
