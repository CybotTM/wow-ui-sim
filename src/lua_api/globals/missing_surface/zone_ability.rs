//! `C_ZoneAbility` probe surface backed by a small mutable state table.
//!
//! Blizzard's ZoneAbility UI only needs two pieces of data in the sim:
//! the current active ability list and an icon lookup for a spell ID.
//! Tests seed the active list directly through `C_ZoneAbility._state`.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{create_string, create_table, table_get, table_set};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const ZONE_ABILITY_ICON: &str = "Interface\\ICONS\\INV_Misc_QuestionMark";

pub(super) fn register_zone_ability_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns_ref = ensure_namespace(state, "C_ZoneAbility")?;
    ensure_zone_ability_state(state);
    table_set_rust_fn_static(state, ns_ref, "GetActiveAbilities", get_active_abilities)?;
    table_set_rust_fn_static(state, ns_ref, "GetZoneAbilityIcon", get_zone_ability_icon)?;
    Ok(())
}

fn ensure_zone_ability_state(state: &mut LuaState) -> Val {
    let ns = Val::Table(ensure_namespace(state, "C_ZoneAbility").expect("C_ZoneAbility namespace"));
    match table_get(state, ns, "_state") {
        Val::Table(_) => table_get(state, ns, "_state"),
        _ => {
            let state_table = create_table(state);
            let active_abilities = create_table(state);
            table_set(state, state_table, "activeAbilities", active_abilities);
            table_set(state, ns, "_state", state_table);
            state_table
        }
    }
}

fn copy_zone_ability_info(state: &mut LuaState, ability: Val) -> Val {
    let copy = create_table(state);
    for key in [
        "zoneAbilityID",
        "uiPriority",
        "spellID",
        "textureKit",
        "tutorialText",
    ] {
        let value = table_get(state, ability, key);
        if !matches!(value, Val::Nil) {
            table_set(state, copy, key, value);
        }
    }
    copy
}

fn clone_active_abilities(state: &mut LuaState, abilities: Val) -> Val {
    let clone = create_table(state);
    let mut index = 1_i64;
    loop {
        let ability = table_get_int(state, abilities, index as i32);
        if matches!(ability, Val::Nil) {
            break;
        }
        let ability_copy = copy_zone_ability_info(state, ability);
        set_table_array(state, clone, index, ability_copy);
        index += 1;
    }
    clone
}

fn get_active_abilities(state: &mut LuaState) -> LuaResult<u32> {
    let state_table = ensure_zone_ability_state(state);
    let abilities = table_get(state, state_table, "activeAbilities");
    let clone = clone_active_abilities(state, abilities);
    state.push(clone);
    Ok(1)
}

fn get_zone_ability_icon(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    let texture = create_string(state, ZONE_ABILITY_ICON);
    state.push(texture);
    Ok(1)
}

fn table_get_int(state: &LuaState, table: Val, index: i32) -> Val {
    let Val::Table(table_ref) = table else {
        return Val::Nil;
    };
    state
        .gc
        .tables
        .get(table_ref)
        .map(|t| t.get_int(index as i64))
        .unwrap_or(Val::Nil)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn active_abilities_default_to_empty_list() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let count: i32 = env
            .eval("local abilities = C_ZoneAbility.GetActiveAbilities(); return #abilities")
            .expect("active abilities should be queryable");

        assert_eq!(count, 0);
    }

    #[test]
    fn active_abilities_return_copied_seed_data() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let (count, spell_id, tutorial_text): (i32, i32, String) = env
            .eval(
                r#"
                C_ZoneAbility._state.activeAbilities = {
                    {
                        zoneAbilityID = 1,
                        uiPriority = 2,
                        spellID = 372610,
                        tutorialText = "Skyward Ascent",
                    },
                }

                local abilities = C_ZoneAbility.GetActiveAbilities()
                return #abilities, abilities[1].spellID, abilities[1].tutorialText
                "#,
            )
            .expect("seeded active abilities should be returned");

        assert_eq!(count, 1);
        assert_eq!(spell_id, 372610);
        assert_eq!(tutorial_text, "Skyward Ascent");
    }
}
