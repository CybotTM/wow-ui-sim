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

#[test]
fn runtime_set_parent_key_without_force_keeps_existing_parent_key_alias() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");

        let (old_alias_kept, new_alias_set, parent_key): (bool, bool, String) = env
            .eval(
                r#"
                local parent = CreateFrame("Frame", "RuntimeSetParentKeyForceParent", UIParent)
                local child = CreateFrame("Button", "RuntimeSetParentKeyForceChild", parent)
                child:SetParentKey("OldAlias")
                child:SetParentKey("NewAlias")
                return parent.OldAlias == child, parent.NewAlias == child, child:GetParentKey()
                "#,
            )
            .expect("eval non-force SetParentKey alias behavior");

        assert!(old_alias_kept, "non-force SetParentKey should keep the old alias");
        assert!(new_alias_set, "non-force SetParentKey should add the new alias");
        assert_eq!(
            parent_key, "OldAlias",
            "non-force SetParentKey should keep GetParentKey() on the existing alias"
        );
    }
}

#[test]
fn runtime_set_parent_key_with_false_clear_existing_preserves_existing_aliases() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");

        let (old_alias_kept, new_alias_set, parent_key): (bool, bool, String) = env
            .eval(
                r#"
                local parent = CreateFrame("Frame", "RuntimeSetParentKeyFalseParent", UIParent)
                local child = CreateFrame("Button", "RuntimeSetParentKeyFalseChild", parent)
                child:SetParentKey("OldAlias")
                child:SetParentKey("NewAlias", false)
                return parent.OldAlias == child, parent.NewAlias == child, child:GetParentKey()
                "#,
            )
            .expect("eval false clear-existing SetParentKey behavior");

        assert!(old_alias_kept, "false clear-existing should keep the old alias");
        assert!(new_alias_set, "false clear-existing should add the new alias");
        assert_eq!(
            parent_key, "OldAlias",
            "false clear-existing should preserve the canonical parent key"
        );
    }
}

#[test]
fn runtime_set_parent_key_with_force_clears_existing_aliases() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");

        let (old_alias_cleared, new_alias_set, parent_key): (bool, bool, String) = env
            .eval(
                r#"
                local parent = CreateFrame("Frame", "RuntimeSetParentKeyClearParent", UIParent)
                local child = CreateFrame("Button", "RuntimeSetParentKeyClearChild", parent)
                child:SetParentKey("OldAlias")
                child:SetParentKey("NewAlias", true)
                return parent.OldAlias == nil, parent.NewAlias == child, child:GetParentKey()
                "#,
            )
            .expect("eval force SetParentKey alias behavior");

        assert!(old_alias_cleared, "force SetParentKey should clear the old alias");
        assert!(new_alias_set, "force SetParentKey should add the new alias");
        assert_eq!(
            parent_key, "NewAlias",
            "force SetParentKey should update GetParentKey() to the new alias"
        );
    }
}
