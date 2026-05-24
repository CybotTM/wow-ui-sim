//! Temporary tooltip data processor defaults for partial addon loads.
//!
//! Full game loads get the real `TooltipDataProcessor` from Blizzard SharedXML.
//! These defaults keep minimal or isolated addon loads callable without hiding
//! the fallback in the central runtime bootstrap.

const TOOLTIP_DATA_PROCESSOR_DEFAULTS_LUA: &str = r#"
local function tooltip_data_processor_noop()
end

if AddTooltipDataAccessor == nil then
    function AddTooltipDataAccessor()
    end
end

TooltipDataProcessor = TooltipDataProcessor or {
    AllTypes = 0,
    AddTooltipPostCall = tooltip_data_processor_noop,
    AddLinePostCall = tooltip_data_processor_noop,
}
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(TOOLTIP_DATA_PROCESSOR_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    fn apply_again(env: &WowLuaEnv) {
        let mut lua = env.lua.borrow_mut();
        super::apply_bootstrap(&mut lua).expect("tooltip data processor defaults should apply");
    }

    #[test]
    fn installs_minimal_tooltip_data_processor_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (String, i32, String, String, bool) = env
            .eval(
                r#"
                return type(TooltipDataProcessor),
                       TooltipDataProcessor.AllTypes,
                       type(TooltipDataProcessor.AddTooltipPostCall),
                       type(TooltipDataProcessor.AddLinePostCall),
                       AddTooltipDataAccessor() == nil
                "#,
            )
            .expect("tooltip data processor default probe should run");

        assert_eq!(
            result,
            (
                "table".to_string(),
                0,
                "function".to_string(),
                "function".to_string(),
                true
            )
        );
    }

    #[test]
    fn preserves_existing_tooltip_data_processor_and_accessor() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            TooltipDataProcessor = {
                AllTypes = 99,
                AddTooltipPostCall = function() return "post" end,
            }
            function AddTooltipDataAccessor()
                return "accessor"
            end
            "#,
        )
        .expect("fixture should install existing tooltip data processor");

        apply_again(&env);

        let result: (i32, String, String) = env
            .eval(
                r#"
                return TooltipDataProcessor.AllTypes,
                       TooltipDataProcessor.AddTooltipPostCall(),
                       AddTooltipDataAccessor()
                "#,
            )
            .expect("tooltip data processor preservation probe should run");

        assert_eq!(result, (99, "post".to_string(), "accessor".to_string()));
    }
}
