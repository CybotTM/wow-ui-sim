//! Forever capability input, not native hardware detection.

#[cfg(feature = "client-wowforever")]
#[test]
fn restricted_wrapper_observes_enabled_cvar_independently_of_mapped_sticks() {
    use crate::common::blizzard_addon_harness::{
        blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
    };
    use wow_ui_sim::c_api::c_game_pad::{MappedStick, MappedStickSnapshot};
    use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
    use wow_ui_sim::screen::ScreenKind;
    use wow_ui_sim::toc::TocFile;

    let ui = blizzard_ui_dir();
    let selected = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    let (_, toc_path) = selected
        .iter()
        .find(|(name, _)| name == "Blizzard_RestrictedAddOnEnvironment")
        .expect("original restricted addon must be selected on Forever Game");
    let toc = TocFile::from_file(toc_path).unwrap();
    let file_index = toc
        .files
        .iter()
        .position(|path| path.to_string_lossy() == "RestrictedEnvironment.lua")
        .expect("original wrapper file must be selected");
    assert_eq!(toc.file_use_secure_env(file_index), Some(true));

    let env = new_blizzard_addon_env(&ui);
    let loaded = load_blizzard_addon_closure_into_env(
        &env,
        &ui,
        &["Blizzard_RestrictedAddOnEnvironment"],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_RestrictedAddOnEnvironment"));
    env.exec(
        r#"
        local path = "Blizzard_RestrictedAddOnEnvironment/"
        local function assertSource(fn, file)
            local source = debug.getinfo(fn, "S").source:gsub("\\", "/")
            assert(source:find(path .. file, 1, true), source)
        end
        assertSource(CallRestrictedClosure, "RestrictedExecution.lua")
        GamepadCapabilityOwner = CreateFrame("Frame", nil, UIParent, "SecureHandlerBaseTemplate")
        GamepadCapabilityEnvironment = GetManagedEnvironment(GamepadCapabilityOwner, true)
        local wrapper = CallRestrictedClosure(GamepadCapabilityOwner, "", GamepadCapabilityEnvironment,
            nil, "return IsGamePadEnabled")
        assertSource(wrapper, "RestrictedEnvironment.lua")
        assert(getfenv(wrapper) == debug.getregistry().__secureenv)
        local function upvalue(fn, wanted)
            for index = 1, 20 do
                local name, value = debug.getupvalue(fn, index)
                if name == wanted then return value end
                if name == nil then break end
            end
            error("missing wrapper upvalue " .. wanted)
        end
        local producerForwarder = upvalue(wrapper, "func")
        assertSource(producerForwarder, "RestrictedEnvironment.lua")
        assert(debug.getinfo(producerForwarder, "S").linedefined == 176)
        assert(getfenv(producerForwarder) == debug.getregistry().__secureenv)
        assert(GetCVarDefault("GamePadEnable") == "0")
        "#,
    )
    .expect("authenticate selected original secure wrapper before querying capability");

    // Enabled without a snapshot, disabled with a nonzero snapshot, then
    // replacement/clearing under each CVar value reject stick-derived policies.
    for (value, mapped) in [
        ("0", false), ("1", false), ("1", true), ("0", true), ("0", false),
    ] {
        env.state().borrow_mut().gamepad_mapped_sticks = mapped.then(|| MappedStickSnapshot {
            sticks: vec![MappedStick { config_name: "Movement".into(), x: 3.0, y: 4.0 }],
        });
        env.exec(&format!(
            r#"
            SetCVar("GamePadEnable", "{value}")
            local expected = {value} == 1
            assert(GetCVarBool("GamePadEnable") == expected)
            local actual = CallRestrictedClosure(GamepadCapabilityOwner, "", GamepadCapabilityEnvironment,
                nil, "return IsGamePadEnabled()")
            assert(type(actual) == "boolean")
            assert(actual == expected, "original restricted wrapper ignored GamePadEnable={value}")
            assert(C_GamePad.IsEnabled() == expected)
            "#
        ))
        .expect("original restricted wrapper must follow the explicit CVar input");
    }
}

#[cfg(any(feature = "client-retail", feature = "client-ptr"))]
#[test]
fn retail_ptr_enabled_capability_is_not_published_by_forever_lane() {
    let env = wow_ui_sim::lua_api::WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local namespace = rawget(_G, "C_GamePad")
        assert(namespace == nil or rawget(namespace, "IsEnabled") == nil)
        for _, value in ipairs({"0", "1", "0"}) do
            SetCVar("GamePadEnable", value)
            assert(GetCVarBool("GamePadEnable") == (value == "1"))
            namespace = rawget(_G, "C_GamePad")
            assert(namespace == nil or rawget(namespace, "IsEnabled") == nil)
        end
        "#,
    )
    .unwrap();
}
