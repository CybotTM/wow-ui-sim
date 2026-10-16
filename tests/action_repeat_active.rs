//! G2 supplied repeat state: original cached consumers, not a native-client oracle.
//! Cache root: ~/.cache/wow-ui-sim/blizzard-ui/<profile>/.
//! R/P: AddOns/Blizzard_ActionBar/Shared/ActionButton.lua:673-677,982-991,
//!      1088-1133,1304-1324. M: same path:673-677,1018-1027,1119-1171,
//!      1335-1355. F: same path:715-719,1026-1035,1132-1177,1353-1373.
//! W: FrameXML/ActionButton.lua:302-312,408-415,437-461,505-524.
//! Wrath's original cache is FrameXML, a sibling of AddOns (not a substitute tree).
#![cfg(not(any(feature = "client-era", feature = "client-anniversary")))]

use std::path::Path;
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

fn exec_cached_source(env: &WowLuaEnv, path: &Path) {
    let source = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read original {}: {error}", path.display()));
    env.exec_named(&source, &format!("@{}", path.display()))
        .unwrap_or_else(|error| panic!("execute original {}: {error}", path.display()));
}

fn load_action_button_consumers() -> WowLuaEnv {
    let addons = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    if ACTIVE == ClientProfile::Wrath {
        let env = WowLuaEnv::new().unwrap();
        exec_cached_source(
            &env,
            &addons.parent().unwrap().join("FrameXML/ActionButton.lua"),
        );
        return env;
    }
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &addons,
        &["Blizzard_SharedXMLBase"],
        &[],
    );
    exec_cached_source(
        &env,
        &addons.join("Blizzard_ActionBar/Shared/ActionButton.lua"),
    );
    env
}

fn create_consumer_buttons(env: &WowLuaEnv) {
    let setup = if ACTIVE == ClientProfile::Wrath {
        r#"
        function G2CreateButton(name, slot)
            local button = CreateFrame("CheckButton", name, UIParent)
            button.action = slot
            local flash = button:CreateTexture(name.."Flash", "OVERLAY")
            flash:Hide()
            button:SetScript("OnEvent", ActionButton_OnEvent)
            button:RegisterEvent("START_AUTOREPEAT_SPELL")
            button:RegisterEvent("STOP_AUTOREPEAT_SPELL")
            button:RegisterEvent("ACTIONBAR_UPDATE_STATE")
            ActionButton_UpdateState(button)
            return button, flash
        end
        function G2Tick(button, elapsed) ActionButton_OnUpdate(button, elapsed) end
        "#
    } else {
        r#"
        ActionBarButtonUpdateFrame = CreateFrame("Frame", nil, UIParent)
        Mixin(ActionBarButtonUpdateFrame, ActionBarButtonUpdateFrameMixin)
        ActionBarButtonUpdateFrame:OnLoad()
        function G2CreateButton(name, slot)
            local button = CreateFrame("CheckButton", name, UIParent)
            Mixin(button, ActionBarActionButtonMixin)
            button.action = slot
            button.Flash = button:CreateTexture(name.."Flash", "OVERLAY")
            button.Flash:Hide()
            button:SetScript("OnEvent", ActionBarActionButtonMixin.OnEvent)
            button:RegisterEvent("START_AUTOREPEAT_SPELL")
            button:RegisterEvent("STOP_AUTOREPEAT_SPELL")
            button:RegisterEvent("ACTIONBAR_UPDATE_STATE")
            button:UpdateState()
            return button, button.Flash
        end
        function G2Tick(button, elapsed) button:OnUpdate(elapsed) end
        "#
    };
    env.exec(setup).unwrap();
    env.exec(
        r#"
        A_Admin.SetActionSlot(201, 853)
        A_Admin.SetActionSlot(202, 853)
        A_Admin.StopCasting()
        G2Button, G2Flash = G2CreateButton("G2Action", 201)
        G2Other, G2OtherFlash = G2CreateButton("G2OtherAction", 202)
        "#,
    )
    .unwrap();
}

#[test]
fn action_repeat_active_original_consumer_checked_start_stop_flash() {
    let env = load_action_button_consumers();
    create_consumer_buttons(&env);
    env.exec(
        r#"
        assert(not C_ActionBar.IsCurrentAction(201), "current=false control")
        assert(not C_ActionBar.IsAttackAction(201), "attack=false control")
        assert(not C_ActionBar.IsAutoCastPetAction(201), "non-pet control")
        assert(not G2Button:GetChecked() and not G2Other:GetChecked())
        assert(not G2Flash:IsShown() and not G2OtherFlash:IsShown())

        A_Admin.SetActionRepeatActive(201, true)
        -- Setter supplies state only. Original event handlers perform the UI work.
        assert(not G2Button:GetChecked(), "supply must not mutate checked UI")
        A_Admin.FireEvent("START_AUTOREPEAT_SPELL")
        assert(G2Button:GetChecked(), "G2 repeat-only START must check original consumer")
        assert(G2Button.flashing == 1, "START must enter original flashing state")
        assert(not C_ActionBar.IsCurrentAction(201), "repeat must not create a cast")
        assert(not C_ActionBar.IsAttackAction(201), "repeat must not classify attack")
        assert(not C_ActionBar.IsAutoRepeatAction(202), "same spell, separate slot")
        assert(not G2Other:GetChecked() and G2Other.flashing ~= 1)
        G2Tick(G2Button, 0.01)
        assert(G2Flash:IsShown(), "original OnUpdate must show actual flash texture")
        G2Tick(G2Button, 0.4)
        assert(not G2Flash:IsShown(), "original OnUpdate must toggle flash texture off")
        G2Tick(G2Button, 0.4)
        assert(G2Flash:IsShown(), "flash must be shown before STOP")

        A_Admin.SetActionRepeatActive(201, false)
        assert(G2Button:GetChecked(), "clear input alone must not drive UI")
        A_Admin.FireEvent("STOP_AUTOREPEAT_SPELL")
        assert(not G2Button:GetChecked(), "STOP must uncheck repeat-only consumer")
        assert(G2Button.flashing == 0 and not G2Flash:IsShown())
        assert(not G2Other:GetChecked() and not G2OtherFlash:IsShown())
        G2Tick(G2Button, 0.4)
        assert(not G2Flash:IsShown(), "stopped flash must remain hidden")

        -- Check the state-refresh branch separately from START's UpdateState call.
        A_Admin.SetActionRepeatActive(201, true)
        A_Admin.FireEvent("ACTIONBAR_UPDATE_STATE")
        assert(G2Button:GetChecked(), "state event must check supplied repeat-only slot")
        assert(G2Button.flashing == 0, "state event must not synthesize START")
        A_Admin.SetActionRepeatActive(201, false)
        A_Admin.FireEvent("ACTIONBAR_UPDATE_STATE")
        assert(not G2Button:GetChecked() and not G2Other:GetChecked())
        "#,
    )
    .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "consumer event errors"
    );
}

#[test]
fn action_repeat_active_supply_is_independent_and_environment_local() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        A_Admin.SetActionSlot(201, 853)
        A_Admin.SetActionSlot(202, 853)
        assert(C_ActionBar.IsAutoRepeatAction(201) == false)
        assert(select('#', A_Admin.SetActionRepeatActive(201, true)) == 0)
        assert(C_ActionBar.IsAutoRepeatAction(201) == true, "supplied repeat must be queryable")
        assert(C_ActionBar.IsAutoRepeatAction(202) == false, "repeat is slot-local")
        A_Admin.SetActionRepeatActive(201, true)
        assert(C_ActionBar.IsAutoRepeatAction(201) == true, "same input is idempotent")
        assert(not C_ActionBar.IsCurrentAction(201) and not C_ActionBar.IsAttackAction(201))
        A_Admin.SetActionRepeatActive(201, false)
        A_Admin.SetCasting(853, "G2 cast control", "", 20)
        assert(C_ActionBar.IsCurrentAction(201), "cast control must actually be current")
        assert(not C_ActionBar.IsAutoRepeatAction(201), "current must not imply repeat")
        A_Admin.StopCasting()
        A_Admin.SetActionRepeatActive(201, true)
        "#,
    )
    .unwrap();
    let other = WowLuaEnv::new().unwrap();
    assert!(
        !other
            .eval::<bool>("return C_ActionBar.IsAutoRepeatAction(201)")
            .unwrap()
    );
}

#[test]
fn action_repeat_active_legacy_projection_uses_same_supplied_state() {
    if !matches!(ACTIVE, ClientProfile::Wrath | ClientProfile::Mists) {
        return;
    }
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        A_Admin.SetActionRepeatActive(201, true)
        assert(IsAutoRepeatAction(201) == true, "legacy repeat must consume supplied slot")
        assert(C_ActionBar.IsAutoRepeatAction(201) == IsAutoRepeatAction(201))
        A_Admin.SetActionRepeatActive(201, false)
        assert(IsAutoRepeatAction(201) == false)
        "#,
    )
    .unwrap();
}

#[test]
fn action_repeat_active_input_failure_preserves_supplied_state() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        A_Admin.SetActionRepeatActive(201, true)
        assert(not pcall(A_Admin.SetActionRepeatActive, 0, true))
        assert(not pcall(A_Admin.SetActionRepeatActive, 201, "false"))
        assert(C_ActionBar.IsAutoRepeatAction(201) == true)
        assert(C_ActionBar.IsAutoRepeatAction(0) == false)
        "#,
    )
    .unwrap();
}
