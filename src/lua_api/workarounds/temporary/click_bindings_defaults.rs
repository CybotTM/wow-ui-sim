//! Temporary `C_ClickBindings` profile defaults.
//!
//! Click-binding profiles are client/account state we do not model yet. Keep
//! the no-profile fallback here so secure unit-button clicks still target units
//! until profile storage and macro/spell execution are modeled.

const CLICK_BINDINGS_DEFAULTS_LUA: &str = r#"
C_ClickBindings = C_ClickBindings or __wow_namespace()

if rawget(C_ClickBindings, "CanSpellBeClickBound") == nil then
    function C_ClickBindings.CanSpellBeClickBound(_spellID)
        return true
    end
end

if rawget(C_ClickBindings, "ExecuteBinding") == nil then
    function C_ClickBindings.ExecuteBinding(targetToken, _button, _modifiers)
        if type(targetToken) == "string" and type(TargetUnit) == "function" then
            TargetUnit(targetToken)
        end
    end
end

if rawget(C_ClickBindings, "GetBindingType") == nil then
    function C_ClickBindings.GetBindingType(button, _modifiers)
        return button == "LeftButton" and 2 or 3
    end
end

if rawget(C_ClickBindings, "GetEffectiveInteractionButton") == nil then
    function C_ClickBindings.GetEffectiveInteractionButton(button)
        return button
    end
end

if rawget(C_ClickBindings, "GetProfileInfo") == nil then
    function C_ClickBindings.GetProfileInfo()
        return {}
    end
end

-- Real WoW ownership: the GLOBAL is the live API and C_ClickBindings.* is the
-- deprecated forwarder (Blizzard_Deprecated/Deprecated_12_0_7.lua rewrites the
-- C_ClickBindings entries to call the globals). Define the globals as the real
-- impls so those forwarders resolve instead of recursing / calling nil.
if rawget(_G, "GetStringFromModifiers") == nil then
    function GetStringFromModifiers(_modifiers)
        return ""
    end
end

if rawget(C_ClickBindings, "GetStringFromModifiers") == nil then
    function C_ClickBindings.GetStringFromModifiers(modifiers)
        return GetStringFromModifiers(modifiers)
    end
end

if rawget(C_ClickBindings, "GetTutorialShown") == nil then
    function C_ClickBindings.GetTutorialShown()
        return true
    end
end

if rawget(_G, "MakeModifiers") == nil then
    function MakeModifiers()
        local modifiers = 0
        if type(IsShiftKeyDown) == "function" and IsShiftKeyDown() then
            modifiers = modifiers + 1
        end
        if type(IsControlKeyDown) == "function" and IsControlKeyDown() then
            modifiers = modifiers + 2
        end
        if type(IsAltKeyDown) == "function" and IsAltKeyDown() then
            modifiers = modifiers + 4
        end
        return modifiers
    end
end

if rawget(C_ClickBindings, "MakeModifiers") == nil then
    function C_ClickBindings.MakeModifiers()
        return MakeModifiers()
    end
end

if rawget(C_ClickBindings, "ResetCurrentProfile") == nil then
    function C_ClickBindings.ResetCurrentProfile()
    end
end

if rawget(C_ClickBindings, "SetProfileByInfo") == nil then
    function C_ClickBindings.SetProfileByInfo(_profileInfo)
    end
end

if rawget(C_ClickBindings, "SetTutorialShown") == nil then
    function C_ClickBindings.SetTutorialShown(_shown)
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(CLICK_BINDINGS_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_left_click_targeting_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("ClearTarget()").expect("target should clear");

        let binding_type: i32 = env
            .eval("return C_ClickBindings.GetBindingType('LeftButton', MakeModifiers())")
            .expect("binding type should be queryable");
        assert_eq!(binding_type, 2);

        env.exec("C_ClickBindings.ExecuteBinding('party1', 'LeftButton', 0)")
            .expect("click binding should execute");

        let target_name: String = env
            .eval("return UnitName('target')")
            .expect("target should be readable");
        assert_eq!(target_name, "Thrynn");
    }

    #[test]
    fn make_modifiers_reads_modeled_modifier_keys() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let initial: i32 = env
            .eval("return MakeModifiers()")
            .expect("initial modifiers");
        assert_eq!(initial, 0);

        env.exec("A_Admin.SetShiftKeyDown(true)")
            .expect("shift should set");
        env.exec("A_Admin.SetAltKeyDown(true)")
            .expect("alt should set");

        let shifted_alt: i32 = env.eval("return MakeModifiers()").expect("held modifiers");
        assert_eq!(shifted_alt, 5);
    }

    #[test]
    fn preserves_existing_click_bindings_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_ClickBindings = C_ClickBindings or __wow_namespace()

            function C_ClickBindings.GetBindingType(_button, _modifiers)
                return 99
            end

            function C_ClickBindings.ExecuteBinding(targetToken, _button, _modifiers)
                _G.existingClickBindingTarget = targetToken
            end
            "#,
        )
        .expect("fixture should install existing click-bindings provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: (i32, String) = env
            .eval(
                r#"
                C_ClickBindings.ExecuteBinding("party2", "LeftButton", 0)
                return C_ClickBindings.GetBindingType("LeftButton", 0), _G.existingClickBindingTarget
                "#,
            )
            .expect("existing click-bindings provider should remain callable");

        assert_eq!(result, (99, "party2".to_string()));
    }
}
