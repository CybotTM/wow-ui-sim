//! Acceptance-owned filter regression through unchanged Blizzard/AuraKit consumers.
#![cfg(feature = "client-wowforever")]

use wow_ui_sim::lua_api::WowLuaEnv;

const FRAME_SECONDS: f64 = 0.016;
const SETTLE_FRAMES: usize = 8;

fn load_blizzard_env() -> WowLuaEnv {
    let ui = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &ui,
        &["Blizzard_AuraContainer"],
        &[],
    );
    env
}

#[cfg(feature = "ellesmere-acceptance")]
fn load_aura_kit_env() -> WowLuaEnv {
    let env = load_blizzard_env();
    let path = std::env::var("ELLESMERE_AURA_KIT_PATH")
        .expect("ELLESMERE_AURA_KIT_PATH must name the unchanged installed AuraKit Lua file");
    let source = std::fs::read_to_string(&path).expect("read installed AuraKit");
    // AuraKit owns this namespace. No addon functions or Blizzard APIs are mocked.
    env.exec("EllesmereUI = {}").unwrap();
    env.exec(&source).expect("load unchanged installed AuraKit");
    env.exec("FixtureCreateContainer = EllesmereUI.AuraKit.CreateContainer")
        .unwrap();
    env
}

fn settle(env: &WowLuaEnv) {
    for _ in 0..SETTLE_FRAMES {
        env.fire_on_update(FRAME_SECONDS).unwrap();
    }
}

fn native_widget_ids(env: &WowLuaEnv, container: &str) -> Vec<[u64; 3]> {
    let rows = env
        .eval::<Vec<Vec<rilua::Val>>>(&format!("return FixtureWidgets({container})"))
        .expect("collect actual initializer widget identities without reading secret state");
    let lua = env.rilua();
    rows.into_iter()
        .map(|row| {
            let widgets: [rilua::Val; 3] =
                row.try_into().expect("button, icon and stack identities");
            widgets.map(|value| {
                wow_ui_sim::lua_api::methods::native_frame_id_from_val(lua.state(), value)
                    .expect("real native widget backing")
            })
        })
        .collect()
}

fn assert_native_container(env: &WowLuaEnv, container: &str, expected: usize) {
    let identities = native_widget_ids(env, container);
    assert!(!identities.is_empty(), "{container}: owned pool must exist");
    let state = env.state().borrow();
    let mut shown = 0;
    for [button_id, icon_id, stack_id] in identities {
        let button = state.widgets.get(button_id).expect("owned aura button");
        if !button.visible {
            continue;
        }
        shown += 1;
        let icon = state.widgets.get(icon_id).expect("actual initializer icon");
        let stack = state
            .widgets
            .get(stack_id)
            .expect("actual initializer stack");
        assert_eq!(icon.texture_file_data_id, Some(135907), "{container}: icon");
        assert_eq!(stack.text.as_deref(), Some("3"), "{container}: stack text");
        println!("{container}: button={button_id} shown icon=135907 stack=3");
    }
    assert_eq!(shown, expected, "{container}: shown native buttons");
    println!("{container}: shown={shown}, expected={expected}");
}

fn assert_containers(env: &WowLuaEnv, keyed_shown: usize) {
    // Native tree/render state stays observable without granting addon Lua access
    // to secret-origin widget getters or clearing any caller/widget taint.
    assert_native_container(env, "FixtureArray", 0);
    assert_native_container(env, "FixtureKeyed", keyed_shown);
}

fn assert_admin_aura_lifecycle(env: &WowLuaEnv) {
    env.exec(include_str!("fixtures/ellesmere_aura_fixture.lua"))
        .unwrap();
    settle(env);
    assert_containers(env, 0);
    env.exec(
        r#"
        A_Admin.AddBuff(19750, 'CURRENT-PACKAGE helpful aura', 135907, 30, 3)
        A_Admin.AddBuff(19751, 'Excluded helpful aura', 135908, 30, 2)
        local aura = C_UnitAuras.GetPlayerAuraBySpellID(19750)
        assert(aura and aura.spellId == 19750 and aura.icon == 135907)
        assert(aura.applications == 3)
        "#,
    )
    .unwrap();
    settle(env);
    assert_containers(env, 1);
    env.exec("A_Admin.RemoveBuff(19750)").unwrap();
    settle(env);
    assert_containers(env, 0);
    env.exec(
        r#"
        assert(C_UnitAuras.GetPlayerAuraBySpellID(19750) == nil)
        assert(C_UnitAuras.GetPlayerAuraBySpellID(19751) ~= nil)
        "#,
    )
    .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
}

#[test]
fn blizzard_keyed_filter_accepts_admin_aura_and_array_rejects_it() {
    let env = load_blizzard_env();
    env.exec(include_str!("fixtures/ellesmere_blizzard_container.lua"))
        .unwrap();
    assert_admin_aura_lifecycle(&env);
}

#[cfg(feature = "ellesmere-acceptance")]
#[test]
fn ellesmere_keyed_filter_accepts_admin_aura_and_array_rejects_it() {
    assert_admin_aura_lifecycle(&load_aura_kit_env());
}
