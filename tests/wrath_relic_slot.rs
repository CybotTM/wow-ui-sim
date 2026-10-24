#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common;

#[test]
fn toggle_character_relic_capability_controls_original_ammo_slot_visibility() {
    let env = common::panel_fixtures::setup_env();
    const PALADIN: i32 = 2;
    const WARRIOR: i32 = 1;

    // Explicit fixture inputs, not a native class-name mapping (unknown in Q1).
    env.state()
        .borrow_mut()
        .wrath_relic_slot
        .class_flags
        .extend([(PALADIN, 8), (WARRIOR, 0)]);

    // Reopen the original panel after state changes. Either constant boolean,
    // nil/no returns, and a cached initial class must fail this selector alone.
    for (class, ammo_visible) in [(PALADIN, false), (WARRIOR, true), (PALADIN, false)] {
        env.state().borrow_mut().player.class_index = class;
        reopen_paper_doll(&env);
        let (character, paper_doll, ranged, ammo_shown, ammo): (bool, bool, bool, bool, bool) = env
            .eval(
                "return CharacterFrame:IsVisible(), PaperDollFrame:IsVisible(), \
                 CharacterRangedSlot:IsVisible(), CharacterAmmoSlot:IsShown(), \
                 CharacterAmmoSlot:IsVisible()",
            )
            .expect("read original character equipment frames");
        assert!(
            character && paper_doll && ranged,
            "class {class}: panel and ranged slot visible"
        );
        assert_eq!(ammo_shown, ammo_visible, "class {class}: ammo shown state");
        assert_eq!(
            ammo, ammo_visible,
            "class {class}: effective ammo visibility"
        );
    }
}

fn reopen_paper_doll(env: &WowLuaEnv) {
    common::panel_fixtures::clear_recorded_lua_errors(env);
    env.exec("HideUIPanel(CharacterFrame); ToggleCharacter('PaperDollFrame')")
        .expect("open original PaperDollFrame through ToggleCharacter");
    let errors = common::panel_fixtures::recorded_lua_errors(env);
    assert!(errors.is_empty(), "ToggleCharacter Lua errors: {errors:?}");
}

#[test]
fn native_relic_result_uses_live_class_flags_not_equipment() {
    let env = WowLuaEnv::new().unwrap();
    let class = env.state().borrow().player.class_index;
    env.exec("assert(select('#', UnitHasRelicSlot('player')) == 1); assert(UnitHasRelicSlot('player') == nil)")
        .unwrap();
    env.state()
        .borrow_mut()
        .wrath_relic_slot
        .class_flags
        .insert(class, 8);
    env.state().borrow_mut().player.equipped_items.clear();
    env.exec("assert(type(UnitHasRelicSlot('PLAYER')) == 'number'); assert(UnitHasRelicSlot('player', false) == 1)")
        .unwrap();
    env.state()
        .borrow_mut()
        .wrath_relic_slot
        .class_flags
        .insert(class, 16);
    env.exec("assert(UnitHasRelicSlot('player') == nil)")
        .unwrap();
    env.state()
        .borrow_mut()
        .wrath_relic_slot
        .class_flags
        .insert(class, 24);
    env.exec("assert(UnitHasRelicSlot('player') == 1)").unwrap();
    env.state()
        .borrow_mut()
        .wrath_relic_slot
        .class_flags
        .remove(&class);
    env.exec("assert(UnitHasRelicSlot('player') == nil)")
        .unwrap();
}

#[test]
fn native_relic_arguments_return_one_nil_or_raise_usage() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        for _, arg in ipairs({'unknown', 'none', '', 1, 0, -1, 1.25}) do
            assert(select('#', UnitHasRelicSlot(arg)) == 1)
            assert(UnitHasRelicSlot(arg) == nil)
        end
        local function usage(...)
            local ok, err = pcall(UnitHasRelicSlot, ...)
            assert(not ok)
            assert(string.find(err, 'Usage: UnitHasRelicSlot("unit")', 1, true))
        end
        usage(); usage(nil); usage(true); usage(false); usage({}); usage(function() end)
        "#,
    )
    .unwrap();
}

#[test]
fn native_relic_tokens_and_guids_require_a_resolved_player() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("TargetUnit('enemy1')").unwrap();
    let mut unit = env.state().borrow().current_target.clone().unwrap();
    unit.is_player = true;
    unit.guid = "Player-1234-00001234".into();
    let tokens = [
        "vehicle",
        "pet",
        "target",
        "partypet1",
        "party1",
        "raidpet1",
        "raid1",
        "boss1",
        "arenapet1",
        "arena1",
        "commentator1",
        "mouseover",
        "focus",
        "npc",
        "questnpc",
        "player-target",
        "target-pet-target",
    ];
    {
        let mut sim = env.state().borrow_mut();
        sim.current_target = None;
        sim.wrath_relic_slot.class_flags.insert(unit.class_index, 8);
        for token in tokens {
            sim.wrath_relic_slot
                .units
                .insert(token.into(), unit.clone());
        }
        for alias in ["self", "enemy1", "softinteract", "unknown", "none"] {
            sim.wrath_relic_slot
                .units
                .insert(alias.into(), unit.clone());
        }
    }
    for token in tokens {
        let result: i32 = env
            .eval(&format!("return UnitHasRelicSlot('{token}')"))
            .unwrap();
        assert_eq!(result, 1, "resolved token {token}");
    }
    env.exec(
        r#"
        assert(UnitHasRelicSlot('MOUSEOVER') == 1)
        assert(UnitHasRelicSlot('Player-1234-00001234') == 1)
        for _, token in ipairs({'self', 'enemy1', 'softinteract', 'unknown', 'none'}) do
            assert(UnitHasRelicSlot(token) == nil, token)
        end
        "#,
    )
    .unwrap();
    {
        let mut sim = env.state().borrow_mut();
        for snapshot in sim.wrath_relic_slot.units.values_mut() {
            snapshot.is_player = false;
        }
    }
    env.exec("assert(UnitHasRelicSlot('target') == nil); assert(UnitHasRelicSlot('Player-1234-00001234') == nil)")
        .unwrap();
    env.state().borrow_mut().wrath_relic_slot.units.clear();
    env.exec("assert(UnitHasRelicSlot('mouseover') == nil)")
        .unwrap();
}
