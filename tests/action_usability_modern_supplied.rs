//! G3 supplied spell usability through unchanged modern cached consumers.
//! Retail/PTR: Blizzard_ActionBar/Shared/ActionButton.lua:405-440,679-707,962-963.
//! Mists: same path:405-440,679-707,998-999; Forever:415-450,721-751,1006-1007.
//! Bounded native-widget fixture, not full ActionBar/template startup or native parity.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-mists",
    feature = "client-wowforever"
))]

use wow_ui_sim::c_api::action_usability::ActionUsability;
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_modern_consumers() -> WowLuaEnv {
    let addons = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &addons,
        &["Blizzard_SharedXMLBase"],
        &[],
    );
    let path = addons.join("Blizzard_ActionBar/Shared/ActionButton.lua");
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read original {}: {error}", path.display()));
    env.exec_named(&source, &format!("@{}", path.display()))
        .unwrap_or_else(|error| panic!("execute original {}: {error}", path.display()));
    env
}

fn create_modern_consumer_buttons(env: &WowLuaEnv) {
    env.exec(
        r#"
        A_Admin.SetActionSlot(201, 853)
        A_Admin.SetActionSlot(202, 853)
        A_Admin.SetActionSlot(203, 635)
        A_Admin.StopCasting()

        ActionBarButtonUpdateFrame = CreateFrame('Frame', nil, UIParent)
        Mixin(ActionBarButtonUpdateFrame, ActionBarButtonUpdateFrameMixin)
        ActionBarButtonUpdateFrame:OnLoad()

        G23Watcher = CreateFrame('Frame', nil, UIParent)
        Mixin(G23Watcher, ActionBarButtonUsableWatcherFrameMixin)
        G23Watcher:SetScript('OnEvent', ActionBarButtonUsableWatcherFrameMixin.OnEvent)
        G23Watcher:OnLoad()

        function G23CreateButton(name, slot)
            local b = CreateFrame('CheckButton', name, UIParent)
            Mixin(b, ActionBarActionButtonMixin)
            b.action = slot
            b.icon = b:CreateTexture(nil, 'ARTWORK')
            b.Flash = b:CreateTexture(nil, 'OVERLAY')
            b.Flash:Hide()
            b:SetScript('OnEvent', ActionBarActionButtonMixin.OnEvent)
            b:RegisterEvent('PLAYER_MOUNT_DISPLAY_CHANGED')
            b:RegisterEvent('START_AUTOREPEAT_SPELL')
            b:RegisterEvent('STOP_AUTOREPEAT_SPELL')
            b:RegisterEvent('ACTIONBAR_UPDATE_STATE')
            G23Watcher:RegisterFrame(slot, b)
            b:UpdateState()
            b:UpdateUsable()
            return b
        end
        G23A = G23CreateButton('G23ModernA', 201)
        G23B = G23CreateButton('G23ModernB', 202)
        G23C = G23CreateButton('G23ModernC', 203)

        function G23Color(b, r, g, blue)
            local ar, ag, ab = b.icon:GetVertexColor()
            assert(math.abs(ar-r) < 0.00001, 'red')
            assert(math.abs(ag-g) < 0.00001, 'green')
            assert(math.abs(ab-blue) < 0.00001, 'blue')
        end
        function G23Query(slot, expectedUsable, expectedPower)
            local usable, power = C_ActionBar.IsUsableAction(slot)
            assert(usable == expectedUsable and power == expectedPower, 'usability pair')
        end
        function G23Changed(slot)
            local usable, power = C_ActionBar.IsUsableAction(slot)
            A_Admin.FireEvent('ACTION_USABLE_CHANGED', {
                {slot=slot, usable=usable, noMana=power}
            })
        end
        "#,
    )
    .unwrap();
}

#[test]
fn modern_action_usability_query_refresh_white_blue_gray_white() {
    let env = load_modern_consumers();
    create_modern_consumer_buttons(&env);
    env.exec(
        r#"
        G23Query(201,true,false)
        G23Color(G23A,1,1,1)
        G23Color(G23B,1,1,1)
        G23Color(G23C,1,1,1)
        "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .spell_action_usability
        .insert(853, ActionUsability::InsufficientPower);
    env.exec(
        r#"
        G23Query(201,false,true)
        G23Color(G23A,1,1,1)
        A_Admin.FireEvent('PLAYER_MOUNT_DISPLAY_CHANGED')
        G23Color(G23A,.5,.5,1)
        G23Color(G23B,.5,.5,1)
        G23Color(G23C,1,1,1)
        G23Query(202,false,true)
        G23Query(203,true,false)
        "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .spell_action_usability
        .insert(853, ActionUsability::Unusable);
    env.exec(
        r#"
        G23Color(G23A,.5,.5,1)
        A_Admin.FireEvent('PLAYER_MOUNT_DISPLAY_CHANGED')
        G23Query(201,false,false)
        G23Color(G23A,.4,.4,.4)
        G23Color(G23B,.4,.4,.4)
        G23Color(G23C,1,1,1)
        "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .spell_action_usability
        .insert(853, ActionUsability::Usable);
    env.exec(
        r#"
        G23Color(G23A,.4,.4,.4)
        A_Admin.FireEvent('PLAYER_MOUNT_DISPLAY_CHANGED')
        G23Query(201,true,false)
        G23Color(G23A,1,1,1)
        G23Color(G23B,1,1,1)
        G23Color(G23C,1,1,1)
        "#,
    )
    .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "consumer event errors: {:?}",
        env.state().borrow().lua_errors
    );
}

#[test]
fn modern_action_usability_changed_records_follow_supplied_bindings() {
    let env = load_modern_consumers();
    create_modern_consumer_buttons(&env);
    env.state()
        .borrow_mut()
        .spell_action_usability
        .insert(853, ActionUsability::InsufficientPower);
    env.exec(
        r#"
        G23Query(201,false,true)
        G23Color(G23A,1,1,1)
        G23Changed(201)
        G23Color(G23A,.5,.5,1)
        G23Color(G23B,1,1,1)
        G23Color(G23C,1,1,1)
        G23Changed(202)
        G23Color(G23B,.5,.5,1)
        G23Changed(203)
        G23Color(G23C,1,1,1)
        A_Admin.SetActionSlot(201,635)
        G23Query(201,true,false)
        G23Color(G23A,.5,.5,1)
        G23Changed(201)
        G23Color(G23A,1,1,1)
        G23Color(G23B,.5,.5,1)
        "#,
    )
    .unwrap();
    env.state().borrow_mut().action_bars.remove(&201);
    env.exec(
        r#"
        G23Query(201,false,false)
        G23Changed(201)
        G23Color(G23A,.4,.4,.4)
        G23Color(G23B,.5,.5,1)
        "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .spell_action_usability
        .insert(853, ActionUsability::Unusable);
    env.exec(
        r#"
        G23Changed(202)
        G23Color(G23B,.4,.4,.4)
        G23Color(G23A,.4,.4,.4)
        G23Color(G23C,1,1,1)
        "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .spell_action_usability
        .insert(853, ActionUsability::Usable);
    env.exec(
        r#"
        G23Changed(202)
        G23Color(G23B,1,1,1)
        G23Color(G23A,.4,.4,.4)
        G23Color(G23C,1,1,1)
        "#,
    )
    .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "consumer event errors: {:?}",
        env.state().borrow().lua_errors
    );
}
