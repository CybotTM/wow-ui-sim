use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn test_insecure_combat_blocks_protected_parent_and_anchored_frame_mutations() {
    let env = env();
    let (protected_width, protected_height, anchored_width, parent_height, parent_kept, blocked): (
        f32,
        f32,
        f32,
        f32,
        bool,
        String,
    ) = env
        .eval(
            r#"
            local blocked = {}
            local listener = CreateFrame("Frame")
            listener:RegisterEvent("ADDON_ACTION_BLOCKED")
            listener:SetScript("OnEvent", function(_, _, _, func)
                blocked[#blocked + 1] = func
            end)

            local parent = CreateFrame("Frame", "ProtectedParentFrame", UIParent)
            local protected = CreateFrame("Frame", "ProtectedActionFrame", parent)
            local anchored = CreateFrame("Frame", "AnchoredToProtectedFrame", UIParent)
            local otherParent = CreateFrame("Frame", "OtherParentFrame", UIParent)

            parent:SetSize(200, 100)
            protected:SetSize(100, 50)
            anchored:SetSize(80, 20)
            anchored:SetPoint("TOPLEFT", protected, "BOTTOMLEFT", 0, -4)

            A_Admin.SetFrameProtected("ProtectedActionFrame", true)
            A_Admin.SetInCombat(true)
            forceinsecure()

            protected:SetSize(120, 60)
            protected:SetParent(otherParent)
            anchored:SetWidth(123)
            parent:SetHeight(222)

            return protected:GetWidth(true),
                   protected:GetHeight(true),
                   anchored:GetWidth(true),
                   parent:GetHeight(true),
                   protected:GetParent() == parent,
                   table.concat(blocked, "|")
            "#,
        )
        .unwrap();

    assert_eq!(protected_width, 100.0);
    assert_eq!(protected_height, 50.0);
    assert_eq!(anchored_width, 80.0);
    assert_eq!(parent_height, 100.0);
    assert!(
        parent_kept,
        "SetParent should stay blocked on the protected frame"
    );
    assert_eq!(
        blocked,
        "ProtectedActionFrame:SetSize()|ProtectedActionFrame:SetParent()|AnchoredToProtectedFrame:SetWidth()|ProtectedParentFrame:SetHeight()"
    );
}

#[test]
fn test_secure_caller_can_mutate_protected_frame_during_combat() {
    let env = env();
    let (width, height, parent_changed, blocked_count): (f32, f32, bool, i32) = env
        .eval(
            r#"
            local blocked = 0
            local listener = CreateFrame("Frame")
            listener:RegisterEvent("ADDON_ACTION_BLOCKED")
            listener:SetScript("OnEvent", function()
                blocked = blocked + 1
            end)

            local originalParent = CreateFrame("Frame", "SecureProtectedOriginalParent", UIParent)
            local newParent = CreateFrame("Frame", "SecureProtectedNewParent", UIParent)
            local protected = CreateFrame("Frame", "SecureProtectedFrame", originalParent)

            protected:SetSize(40, 20)
            A_Admin.SetFrameProtected("SecureProtectedFrame", true)
            A_Admin.SetInCombat(true)

            protected:SetSize(90, 45)
            protected:SetParent(newParent)

            return protected:GetWidth(true),
                   protected:GetHeight(true),
                   protected:GetParent() == newParent,
                   blocked
            "#,
        )
        .unwrap();

    assert_eq!(width, 90.0);
    assert_eq!(height, 45.0);
    assert!(parent_changed);
    assert_eq!(blocked_count, 0);
}

#[test]
fn test_insecure_out_of_combat_can_mutate_protected_frame() {
    let env = env();
    let (width, height, parent_changed, blocked_count): (f32, f32, bool, i32) = env
        .eval(
            r#"
            local blocked = 0
            local listener = CreateFrame("Frame")
            listener:RegisterEvent("ADDON_ACTION_BLOCKED")
            listener:SetScript("OnEvent", function()
                blocked = blocked + 1
            end)

            local originalParent = CreateFrame("Frame", "OutOfCombatProtectedOriginalParent", UIParent)
            local newParent = CreateFrame("Frame", "OutOfCombatProtectedNewParent", UIParent)
            local protected = CreateFrame("Frame", "OutOfCombatProtectedFrame", originalParent)

            protected:SetSize(55, 25)
            A_Admin.SetFrameProtected("OutOfCombatProtectedFrame", true)
            A_Admin.SetInCombat(false)
            forceinsecure()

            protected:SetSize(95, 35)
            protected:SetParent(newParent)

            return protected:GetWidth(true),
                   protected:GetHeight(true),
                   protected:GetParent() == newParent,
                   blocked
            "#,
        )
        .unwrap();

    assert_eq!(width, 95.0);
    assert_eq!(height, 35.0);
    assert!(parent_changed);
    assert_eq!(blocked_count, 0);
}
