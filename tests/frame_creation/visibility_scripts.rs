use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_cross_frame_show_recursion_does_not_overflow() {
    let env = WowLuaEnv::new().unwrap();
    env.eval::<()>(
        r#"
        local a = CreateFrame("Frame", "RecurseA", UIParent)
        local b = CreateFrame("Frame", "RecurseB", UIParent)
        a:Hide()
        b:Hide()
        a:SetScript("OnShow", function() b:Show() end)
        b:SetScript("OnShow", function() a:Show() end)
        a:Show()
        "#,
    )
    .unwrap();
}

#[test]
fn test_onshow_onhide_mutual_recursion_terminates_with_reference_order() {
    let env = WowLuaEnv::new().unwrap();
    let log: String = env
        .eval(
            r#"
            local log = {}
            local f = CreateFrame("Frame", "MutualVisibilityFrame", UIParent)
            f:SetScript("OnShow", function(self)
                table.insert(log, self:IsVisible() and "A" or "a")
                self:Hide()
                table.insert(log, self:IsVisible() and "B" or "b")
            end)
            f:SetScript("OnHide", function(self)
                table.insert(log, self:IsVisible() and "C" or "c")
                self:Show()
                table.insert(log, self:IsVisible() and "D" or "d")
            end)

            f:Hide()
            return table.concat(log)
        "#,
        )
        .unwrap();

    assert_eq!(
        log,
        "cDAb".repeat(6),
        "OnShow/OnHide mutual recursion should unwind iteratively with wowless/master ordering"
    );
}

#[test]
fn test_child_onshow_fires_when_parent_becomes_visible() {
    let env = WowLuaEnv::new().unwrap();
    let fired: i32 = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "ChildOnShowParent", UIParent)
            local child = CreateFrame("Frame", "ChildOnShowChild", parent)
            parent:Hide()
            child:Hide()

            local fired = 0
            child:SetScript("OnShow", function()
                fired = fired + 1
            end)

            child:Show()
            parent:Show()
            return fired
        "#,
        )
        .unwrap();
    assert_eq!(
        fired, 1,
        "child OnShow should fire when a hidden parent becomes visible"
    );
}

#[test]
fn test_child_onhide_fires_when_parent_becomes_hidden() {
    let env = WowLuaEnv::new().unwrap();
    let fired: i32 = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "ChildOnHideParent", UIParent)
            local child = CreateFrame("Frame", "ChildOnHideChild", parent)

            local fired = 0
            child:SetScript("OnHide", function()
                fired = fired + 1
            end)

            parent:Hide()
            return fired
        "#,
        )
        .unwrap();
    assert_eq!(
        fired, 1,
        "child OnHide should fire when a visible parent becomes hidden"
    );
}
