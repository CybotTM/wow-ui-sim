//! Bounded supplied-state contracts, not native capability or cooldown policy.
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::SpellCooldownState;

#[test]
fn item_cooldown_projects_supplied_records_only_on_ptr() {
    let env = WowLuaEnv::new().unwrap();
    if ACTIVE != ClientProfile::Ptr {
        assert!(
            env.eval::<bool>("return rawget(C_Spell, 'GetItemCooldown') == nil")
                .unwrap()
        );
        return;
    }
    env.exec("assert(C_Spell.GetItemCooldown(999999) == nil)")
        .unwrap();
    env.state().borrow_mut().item_cooldowns.insert(
        6948,
        SpellCooldownState {
            start: 11.0,
            duration: 31.0,
        },
    );
    env.exec(
        r#"
        assert(select('#', C_Spell.GetItemCooldown(6948)) == 1)
        local info = C_Spell.GetItemCooldown(6948)
        assert(info.startTime == 11 and info.duration == 31)
        assert(info.isEnabled == true and info.isActive == true)
        assert(info.modRate == 1)
        assert(info.activeCategory == nil and info.isOnGCD == nil)
        info.duration = 99
        assert(C_Spell.GetItemCooldown(6948).duration == 31)
    "#,
    )
    .unwrap();
    env.state().borrow_mut().item_cooldowns.insert(
        6948,
        SpellCooldownState {
            start: 0.0,
            duration: 0.0,
        },
    );
    env.exec(
        r#"
        local info = C_Spell.GetItemCooldown(6948)
        assert(info.startTime == 0 and info.duration == 0)
        assert(info.isActive == false)
    "#,
    )
    .unwrap();
    env.state().borrow_mut().item_cooldowns.remove(&6948);
    env.exec("assert(C_Spell.GetItemCooldown(6948) == nil)")
        .unwrap();
    let other = WowLuaEnv::new().unwrap();
    other
        .exec("assert(C_Spell.GetItemCooldown(6948) == nil)")
        .unwrap();
}

#[test]
fn graveyard_query_projects_supplied_availability_without_cross_environment_leaks() {
    let env = WowLuaEnv::new().unwrap();
    if !matches!(ACTIVE, ClientProfile::Ptr | ClientProfile::WowForever) {
        assert!(env.eval::<bool>("return CanPortGraveyard == nil").unwrap());
        return;
    }
    env.exec("assert(CanPortGraveyard() == false and select('#', CanPortGraveyard()) == 1)")
        .unwrap();
    env.state().borrow_mut().player.can_port_graveyard = true;
    env.exec("assert(CanPortGraveyard() == true)").unwrap();
    let other = WowLuaEnv::new().unwrap();
    other.exec("assert(CanPortGraveyard() == false)").unwrap();
    env.state().borrow_mut().player.can_port_graveyard = false;
    env.exec("assert(CanPortGraveyard() == false)").unwrap();
}
