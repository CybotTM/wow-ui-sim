//! Original Forever 70532 Gamepad/Core.lua:4/6 temporary-CVar lifecycle.
//! Declarations: CVarDocumentation.lua:171 SetTempCVar, :114 RemoveTempCVar.
#![cfg(any(feature = "client-wowforever"))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[path = "common/cf2_cvar_storage.rs"]
mod storage;

fn new_cache_env() -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    eprintln!("CF-2 original consumer cache: {}", ui.display());
    crate::common::blizzard_addon_harness::new_blizzard_addon_env(&ui)
}

fn load_original_addon(env: &WowLuaEnv, addon: &str) {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let loaded = crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env(
        env,
        &ui,
        &[addon],
        &[],
    );
    assert!(loaded.iter().any(|name| name == addon));
}

fn assert_original_function(env: &WowLuaEnv, function: &str, suffix: &str) {
    env.exec(&format!(
        r#"
        local info = debug.getinfo({function}, 'S')
        local source = info.source:gsub('\\', '/')
        local suffix = '{suffix}'
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'unchanged cached consumer required')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)
        "#,
    ))
    .expect("original cached Lua function provenance");
}

fn drive_original_input_transition(env: &WowLuaEnv, new: &str, old: &str) {
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec(&format!(
        "local types = Enum.InputDeviceInterfaceType; \
         Cf2InputListener:GetScript('OnEvent')(Cf2InputListener, \
         'INPUT_DEVICE_INTERFACE_TRANSITION', types.{new}, types.{old})"
    ))
    .expect("original InputUtil listener must run original Gamepad/Core callbacks");
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

fn assert_live_cvars(env: &WowLuaEnv, enemy: &str) {
    assert_eq!(
        env.state().borrow().cvars.get("SoftTargetEnemy").as_deref(),
        Some(enemy)
    );
    assert_eq!(
        env.state()
            .borrow()
            .cvars
            .get("cameraDistanceMaxZoomFactor")
            .as_deref(),
        Some("1.9")
    );
    env.exec(&format!(
        "assert(GetCVar('SoftTargetEnemy') == '{enemy}'); \
         assert(C_CVar.GetCVar('SoftTargetEnemy') == '{enemy}'); \
         assert(GetCVar('cameraDistanceMaxZoomFactor') == '1.9')"
    ))
    .expect("Lua and Rust must observe the same effective CVar storage");
}

fn read_persisted_cvars() -> Vec<u8> {
    let data = std::env::var_os("XDG_DATA_HOME").expect("child-only CVar storage");
    std::fs::read(std::path::PathBuf::from(data).join("wow-sim/cvars.json"))
        .expect("read child-only saved CVar file")
}

#[test]
fn original_gamepad_temp_cvars_restore_persisted_values_without_touching_ordinary_cvar() {
    storage::run_isolated(
        "cf2_gamepad_temp_cvars::original_gamepad_temp_cvars_restore_persisted_values_without_touching_ordinary_cvar",
        || {
            let env = new_cache_env();
            assert!(env.state().borrow().cvars.set("SoftTargetEnemy", "2"));
            assert!(
                env.state()
                    .borrow()
                    .cvars
                    .set("cameraDistanceMaxZoomFactor", "1.9")
            );
            let persisted = read_persisted_cvars();
            load_original_addon(&env, "Blizzard_Gamepad");
            assert_original_function(
                &env,
                "InputUtil.RegisterMKBInit",
                "Blizzard_SharedXML/InputUtil.lua",
            );
            env.exec(
        r#"
        EventUtil.TriggerOnVariablesLoaded()
        -- InputUtil's original listener is forbidden, so public EnumerateFrames omits it.
        local frames = assert(debug.getregistry().__frame_refs, 'real frame cache required')
        local count = 0
        for _, frame in pairs(frames) do
            if frame:IsEventRegistered('INPUT_DEVICE_INTERFACE_TRANSITION') then
                local handler = frame:GetScript('OnEvent')
                local info = handler and debug.getinfo(handler, 'S')
                if info and info.source:gsub('\\', '/'):match('Blizzard_SharedXML/InputUtil.lua$') then
                    count = count + 1
                    Cf2InputListener = frame
                    print('CF-2 input listener provenance', info.source, info.linedefined, info.lastlinedefined)
                end
            end
        end
        assert(count == 1, 'exact original InputUtil transition listener required')
        "#,
    )
    .expect("original Gamepad registration and original event listener must exist");
            // Direct unchanged listener invocation exposes errors dispatch might record.
            drive_original_input_transition(&env, "Mkb", "Gamepad");
            assert_live_cvars(&env, "2");
            drive_original_input_transition(&env, "Gamepad", "Mkb");
            assert_live_cvars(&env, "1");
            assert_eq!(
                env.state()
                    .borrow()
                    .cvars
                    .get("SoftTargetFriend")
                    .as_deref(),
                Some("0")
            );
            assert_eq!(
                env.state()
                    .borrow()
                    .cvars
                    .get("SoftTargetInteractRange")
                    .as_deref(),
                Some("20")
            );
            assert_eq!(
                read_persisted_cvars(),
                persisted,
                "temp callbacks must not rewrite the saved file"
            );
            // An ordinary write during the override must still save only persisted values.
            assert!(
                env.state()
                    .borrow()
                    .cvars
                    .set("cameraDistanceMaxZoomFactor", "1.9")
            );
            let saved: serde_json::Value = serde_json::from_slice(&read_persisted_cvars()).unwrap();
            assert_eq!(saved["softtargetenemy"], "2");
            assert_eq!(saved["cameradistancemaxzoomfactor"], "1.9");
            // A live temporary override must not replace the underlying persisted value.
            let during_override = new_cache_env();
            assert_live_cvars(&during_override, "2");
            drop(during_override);
            drive_original_input_transition(&env, "Mkb", "Gamepad");
            assert_live_cvars(&env, "2");
            drive_original_input_transition(&env, "Mkb", "Mkb");
            assert_live_cvars(&env, "2");
            drop(env);
            assert_live_cvars(&new_cache_env(), "2");
        },
    );
}

#[test]
fn nil_temp_cvar_reports_unmodelled_case_without_changing_saved_values() {
    storage::run_isolated(
        "cf2_gamepad_temp_cvars::nil_temp_cvar_reports_unmodelled_case_without_changing_saved_values",
        || {
            let env = new_cache_env();
            assert!(env.state().borrow().cvars.set("SoftTargetEnemy", "2"));
            let persisted = read_persisted_cvars();
            env.exec(
                r#"
                local ok, err = pcall(C_CVar.SetTempCVar, 'SoftTargetEnemy', nil)
                assert(not ok)
                assert(tostring(err):find('unmodelled nil%-value case'), tostring(err))
                assert(GetCVar('SoftTargetEnemy') == '2')
                "#,
            )
            .expect("unknown native nil semantics must fail explicitly");
            assert_eq!(read_persisted_cvars(), persisted);
        },
    );
}

#[test]
fn original_target_logic_temp_cvar_tracks_supplied_combat_without_persisting() {
    storage::run_isolated(
        "cf2_gamepad_temp_cvars::original_target_logic_temp_cvar_tracks_supplied_combat_without_persisting",
        || {
            let env = new_cache_env();
            assert!(env.state().borrow().cvars.set("SoftTargetInteract", "2"));
            load_original_addon(&env, "Blizzard_GamepadTargeting");
            assert_original_function(
                &env,
                "GamepadTargetLogic.UpdateSoftTargetInteract",
                "Blizzard_GamepadTargeting/TargetLogic.lua",
            );
            let persisted = read_persisted_cvars();
            env.exec("TargetUnit('enemy1')")
                .expect("supply hostile target");
            env.state()
                .borrow_mut()
                .current_target
                .as_mut()
                .expect("supplied target")
                .is_enemy = true;
            for (in_combat, expected) in [(false, "1"), (true, "0"), (false, "1")] {
                env.state().borrow_mut().player.in_combat = in_combat;
                env.exec("GamepadTargetLogic:UpdateSoftTargetInteract()")
                    .expect("original target logic must update effective CVar");
                assert_eq!(
                    env.state()
                        .borrow()
                        .cvars
                        .get("SoftTargetInteract")
                        .as_deref(),
                    Some(expected)
                );
                assert_eq!(read_persisted_cvars(), persisted);
            }
        },
    );
}
