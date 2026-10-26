//! Supplied ammo requirement through the selected original XML-bound slot.
#![cfg(any(feature = "client-ptr", feature = "client-wowforever"))]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

const UNSUPPLIED: &str = "C_PaperDollInfo.AmmoNeeded: ammo requirement has not been supplied";

fn supply(env: &WowLuaEnv, required: Option<bool>) {
    env.state().borrow_mut().forever_startup.ammo_required = required;
}

fn load_original_slot() -> WowLuaEnv {
    let ui = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui);
    assert_eq!(env.state().borrow().forever_startup.ammo_required, None);
    supply(&env, Some(false));
    load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_UIPanels_Game"], &[]);
    eprintln!("unfiltered slot load diagnostics: {:?}", env.state().borrow().lua_errors);
    let game = match ACTIVE {
        ClientProfile::Ptr => "Mainline",
        ClientProfile::WowForever => "Camelot",
        _ => unreachable!("only declaring profiles register these tests"),
    };
    env.exec(&format!(
        r#"
        local info = debug.getinfo(PaperDollItemSlotButton_OnLoad, 'S')
        local expected = 'Blizzard_UIPanels_Game/{game}/PaperDollFrame.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#expected) == expected, source)
        print('selected original slot consumer', source, info.linedefined)
        assert(CharacterAmmoSlot, 'original XML must create CharacterAmmoSlot')
        AmmoProbe = CharacterAmmoSlot
        AmmoOnLoad = assert(AmmoProbe:GetScript('OnLoad'), 'original XML OnLoad')
        local script = debug.getinfo(AmmoOnLoad, 'S')
        print('original XML slot binding', script.source, script.linedefined)
        assert(AmmoProbe.icon, 'real ItemButton icon')
        function InvokeOriginalAmmoSlot()
            AmmoProbe:Show()
            AmmoProbe:SetID(1777)
            AmmoOnLoad(AmmoProbe)
        end
        "#,
    ))
    .expect("selected original Lua and XML slot binding must load");
    env
}

fn observe_slot(env: &WowLuaEnv, required: bool) {
    supply(env, Some(required));
    env.exec("InvokeOriginalAmmoSlot()")
        .expect("original XML-bound slot must accept supplied requirement");
    let shown: bool = env.eval("return AmmoProbe:IsShown()").unwrap();
    assert_eq!(shown, required, "original slot visibility reflects supply");
    if required {
        env.exec(
            r#"
            local id, texture, relic = C_PaperDollInfo.GetInventorySlotInfo('AmmoSlot')
            assert(AmmoProbe:GetID() == id and id ~= 1777, 'original initialization')
            assert(AmmoProbe.icon:GetTexture() == texture, 'original icon')
            assert(AmmoProbe.backgroundTextureName == texture)
            assert(AmmoProbe.checkRelic == relic)
            assert(type(AmmoProbe.GetItemLocationCallback) == 'function')
            assert(select('#', C_PaperDollInfo.AmmoNeeded()) == 1)
            assert(C_PaperDollInfo.AmmoNeeded() == true)
            "#,
        )
        .unwrap();
    } else {
        assert_eq!(env.eval::<i32>("return AmmoProbe:GetID()").unwrap(), 1777);
        assert!(!env.eval::<bool>("return C_PaperDollInfo.AmmoNeeded()").unwrap());
    }
}

#[test]
fn original_xml_slot_supplied_transitions_isolation_and_unset() {
    let first = load_original_slot();
    let second = load_original_slot();
    for required in [false, true, false] {
        observe_slot(&first, required);
        observe_slot(&second, !required);
        assert_eq!(
            first.eval::<bool>("return C_PaperDollInfo.AmmoNeeded()").unwrap(),
            required,
            "second environment must not change first observation"
        );
    }
    supply(&first, None);
    let error = first
        .exec(
            "local ok, reason = xpcall(InvokeOriginalAmmoSlot, debug.traceback); if not ok then error(reason, 0) end",
        )
        .expect_err("unset input must fail through the original XML-bound slot")
        .to_string();
    assert!(error.contains(UNSUPPLIED), "{error}");
    let line = match ACTIVE {
        ClientProfile::Ptr => "PaperDollFrame.lua:1702",
        ClientProfile::WowForever => "PaperDollFrame.lua:1952",
        _ => unreachable!(),
    };
    assert!(error.contains(line), "original consumer boundary: {error}");
    observe_slot(&second, true);
    observe_slot(&first, true);
    observe_slot(&first, false);
}
