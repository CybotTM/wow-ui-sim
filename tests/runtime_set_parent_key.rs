use crate::common;

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn runtime_set_parent_key_exposes_child_on_parent_and_child_getter() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");

        env.exec(
            r#"
            RuntimeSetParentKeyParent = CreateFrame("Frame", "RuntimeSetParentKeyParent", UIParent)
            RuntimeSetParentKeyChild = CreateFrame("Button", "RuntimeSetParentKeyChild", RuntimeSetParentKeyParent)
            RuntimeSetParentKeyChild:SetParentKey("Foo")
            "#,
        )
        .expect("Create runtime parent/child frames");

        let (parent_lookup_matches, child_parent_key): (bool, String) = env
            .eval(
                r#"
                return RuntimeSetParentKeyParent.Foo == RuntimeSetParentKeyChild,
                    RuntimeSetParentKeyChild:GetParentKey()
                "#,
            )
            .expect("eval runtime parent-key wiring");

        assert!(
            parent_lookup_matches,
            "SetParentKey should expose the child via parent.Foo"
        );
        assert_eq!(
            child_parent_key, "Foo",
            "SetParentKey should update GetParentKey() on the child"
        );
    }
}
