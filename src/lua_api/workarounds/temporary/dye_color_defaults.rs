//! Temporary `C_DyeColor` defaults.
//!
//! Dye color data is not modeled yet. Return empty color lists for the 12.1
//! plural query surface until an item/dye backend owns these APIs.

const DYE_COLOR_DEFAULTS_LUA: &str = r#"
C_DyeColor = C_DyeColor or __wow_namespace()

if rawget(C_DyeColor, "GetDyeColorsForItem") == nil then
    function C_DyeColor.GetDyeColorsForItem()
        return {}
    end
end

if rawget(C_DyeColor, "GetDyeColorsForItemLocation") == nil then
    function C_DyeColor.GetDyeColorsForItemLocation()
        return {}
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(DYE_COLOR_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_empty_dye_color_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (i32, i32) = env
            .eval(
                r#"
                return #C_DyeColor.GetDyeColorsForItem(1),
                    #C_DyeColor.GetDyeColorsForItemLocation({})
                "#,
            )
            .expect("dye color defaults should be callable");

        assert_eq!(result, (0, 0));
    }

    #[test]
    fn preserves_existing_dye_color_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_DyeColor = {
                GetDyeColorsForItem = function()
                    return { "existing" }
                end,
            }
            "#,
        )
        .expect("fixture should install existing dye color provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: (String, i32) = env
            .eval(
                r#"
                return C_DyeColor.GetDyeColorsForItem()[1],
                    #C_DyeColor.GetDyeColorsForItemLocation({})
                "#,
            )
            .expect("dye color providers should remain callable");

        assert_eq!(result, ("existing".to_string(), 0));
    }
}
