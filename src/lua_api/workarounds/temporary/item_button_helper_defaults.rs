//! Temporary item-button helper defaults for minimal startup modes.
//!
//! Blizzard_ItemButton owns the real global helpers in full UI loads. These
//! defaults keep no-addon/minimal surfaces callable without leaving the
//! fallback in the generic runtime bootstrap.

const ITEM_BUTTON_HELPER_DEFAULTS_LUA: &str = r#"
if SetItemButtonTexture == nil then
    function SetItemButtonTexture(button, texture)
        if type(button) ~= "table" then
            return
        end
        local icon = button.icon or button.Icon
        if icon ~= nil and type(icon.SetTexture) == "function" then
            icon:SetTexture(texture)
            if texture ~= nil and type(icon.Show) == "function" then
                icon:Show()
            end
        end
    end
end

if SetItemButtonCount == nil then
    function SetItemButtonCount(button, count)
        if type(button) ~= "table" then
            return
        end
        local countText = button.Count
        if countText ~= nil and type(countText.SetText) == "function" then
            if count == nil or count == 0 then
                countText:SetText("")
            else
                countText:SetText(tostring(count))
            end
        end
    end
end

if SetItemButtonTextureVertexColor == nil then
    function SetItemButtonTextureVertexColor(button, r, g, b)
        if type(button) ~= "table" then
            return
        end
        local icon = button.icon or button.Icon
        if icon ~= nil and type(icon.SetVertexColor) == "function" then
            icon:SetVertexColor(r or 1, g or 1, b or 1)
        end
    end
end

if SetItemButtonNormalTextureVertexColor == nil then
    function SetItemButtonNormalTextureVertexColor(button, r, g, b)
        if type(button) ~= "table" then
            return
        end
        local normalTexture = button.NormalTexture or button.normalTexture
        if normalTexture ~= nil and type(normalTexture.SetVertexColor) == "function" then
            normalTexture:SetVertexColor(r or 1, g or 1, b or 1)
            return
        end
        SetItemButtonTextureVertexColor(button, r, g, b)
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(ITEM_BUTTON_HELPER_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    fn apply_again(env: &WowLuaEnv) {
        let mut lua = env.lua.borrow_mut();
        super::apply_bootstrap(&mut lua).expect("item-button helper defaults should apply");
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 0.0001,
            "expected {actual} to be close to {expected}"
        );
    }

    #[test]
    fn installs_item_button_helper_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (String, String, String, String) = env
            .eval(
                r#"
                return type(SetItemButtonTexture),
                       type(SetItemButtonCount),
                       type(SetItemButtonTextureVertexColor),
                       type(SetItemButtonNormalTextureVertexColor)
                "#,
            )
            .expect("item-button helper type probe should run");

        assert_eq!(
            result,
            (
                "function".to_string(),
                "function".to_string(),
                "function".to_string(),
                "function".to_string()
            )
        );
    }

    #[test]
    fn updates_plain_item_button_texture_count_and_colors() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (String, String, f64, f64, f64, f64, f64, f64) = env
            .eval(
                r#"
                local button = CreateFrame("Button", "TemporaryItemButtonHelperProbe", UIParent)
                button.icon = button:CreateTexture(nil, "BORDER")
                button.Count = button:CreateFontString(nil, "OVERLAY")
                button.normalTexture = button:CreateTexture(nil, "ARTWORK")

                SetItemButtonTexture(button, "Interface\\Icons\\INV_Misc_QuestionMark")
                SetItemButtonCount(button, 7)
                SetItemButtonTextureVertexColor(button, 0.1, 0.2, 0.3)
                SetItemButtonNormalTextureVertexColor(button, 0.4, 0.5, 0.6)

                local ir, ig, ib = button.icon:GetVertexColor()
                local nr, ng, nb = button.normalTexture:GetVertexColor()
                return tostring(button.icon:GetTexture()),
                       button.Count:GetText(),
                       ir, ig, ib,
                       nr, ng, nb
                "#,
            )
            .expect("item-button helper behavior probe should run");

        assert!(result.0.contains("INV_Misc_QuestionMark"));
        assert_eq!(result.1, "7");
        assert_close(result.2, 0.1);
        assert_close(result.3, 0.2);
        assert_close(result.4, 0.3);
        assert_close(result.5, 0.4);
        assert_close(result.6, 0.5);
        assert_close(result.7, 0.6);
    }

    #[test]
    fn preserves_existing_item_button_helpers() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function SetItemButtonTexture() return "texture" end
            function SetItemButtonCount() return "count" end
            function SetItemButtonTextureVertexColor() return "texture-color" end
            function SetItemButtonNormalTextureVertexColor() return "normal-color" end
            "#,
        )
        .expect("fixture should install existing item-button helpers");

        apply_again(&env);

        let result: (String, String, String, String) = env
            .eval(
                r#"
                return SetItemButtonTexture(),
                       SetItemButtonCount(),
                       SetItemButtonTextureVertexColor(),
                       SetItemButtonNormalTextureVertexColor()
                "#,
            )
            .expect("item-button helper preservation probe should run");

        assert_eq!(
            result,
            (
                "texture".to_string(),
                "count".to_string(),
                "texture-color".to_string(),
                "normal-color".to_string()
            )
        );
    }
}
