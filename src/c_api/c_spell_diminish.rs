//! `C_SpellDiminish` surface for spell-diminish status trays.

use crate::c_api::helpers::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{create_string, create_table, table_set};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

type SpellDiminishFn = fn(&mut LuaState) -> LuaResult<u32>;

pub(crate) fn register_c_spell_diminish_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_SpellDiminish")?;
    for &(name, func) in SPELL_DIMINISH_METHODS {
        table_set_rust_fn_static(state, ns, name, func)?;
    }
    Ok(())
}

const SPELL_DIMINISH_METHODS: &[(&str, SpellDiminishFn)] = &[
    (
        "GetAllSpellDiminishCategories",
        get_all_spell_diminish_categories,
    ),
    (
        "GetSpellDiminishCategoryInfo",
        get_spell_diminish_category_info,
    ),
    ("IsSystemSupported", is_system_supported),
    (
        "ShouldTrackSpellDiminishCategory",
        should_track_spell_diminish_category,
    ),
];

#[derive(Clone, Copy)]
struct SpellDiminishCategory {
    id: i32,
    name: &'static str,
    icon: &'static str,
}

const SPELL_DIMINISH_CATEGORIES: &[SpellDiminishCategory] = &[
    SpellDiminishCategory {
        id: 0,
        name: "Root",
        icon: "Interface\\Icons\\Ability_EntanglingRoots",
    },
    SpellDiminishCategory {
        id: 1,
        name: "Taunt",
        icon: "Interface\\Icons\\Spell_Nature_Reincarnation",
    },
    SpellDiminishCategory {
        id: 2,
        name: "Stun",
        icon: "Interface\\Icons\\Ability_CheapShot",
    },
    SpellDiminishCategory {
        id: 3,
        name: "Knockback",
        icon: "Interface\\Icons\\Ability_Druid_Typhoon",
    },
    SpellDiminishCategory {
        id: 4,
        name: "Incapacitate",
        icon: "Interface\\Icons\\Spell_Nature_Polymorph",
    },
    SpellDiminishCategory {
        id: 5,
        name: "Disorient",
        icon: "Interface\\Icons\\Spell_Shadow_MindSteal",
    },
    SpellDiminishCategory {
        id: 6,
        name: "Silence",
        icon: "Interface\\Icons\\Spell_Shadow_ImpPhaseShift",
    },
    SpellDiminishCategory {
        id: 7,
        name: "Disarm",
        icon: "Interface\\Icons\\Ability_Warrior_Disarm",
    },
];

fn get_all_spell_diminish_categories(state: &mut LuaState) -> LuaResult<u32> {
    let _ruleset = Option::<i32>::from_stack(state, 1).ok().flatten();
    let result = create_table(state);
    for (index, category) in SPELL_DIMINISH_CATEGORIES.iter().enumerate() {
        let entry = category_info_table(state, *category);
        set_table_array(state, result, index as i64 + 1, entry);
    }
    state.push(result);
    Ok(1)
}

fn get_spell_diminish_category_info(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = Option::<i32>::from_stack(state, 1).ok().flatten();
    let Some(category) = category_id.and_then(find_category) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = category_info_table(state, category);
    state.push(info);
    Ok(1)
}

fn is_system_supported(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn should_track_spell_diminish_category(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = Option::<i32>::from_stack(state, 1).ok().flatten();
    state.push(Val::Bool(category_id.and_then(find_category).is_some()));
    Ok(1)
}

fn find_category(category_id: i32) -> Option<SpellDiminishCategory> {
    SPELL_DIMINISH_CATEGORIES
        .iter()
        .copied()
        .find(|category| category.id == category_id)
}

fn category_info_table(state: &mut LuaState, category: SpellDiminishCategory) -> Val {
    let info = create_table(state);
    let name = create_string(state, category.name);
    let icon = create_string(state, category.icon);
    table_set(state, info, "category", Val::Num(category.id as f64));
    table_set(state, info, "name", name);
    table_set(state, info, "icon", icon);
    info
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn spell_diminish_categories_are_ipairs_safe() {
        let env = WowLuaEnv::new().expect("env");
        let (count, first_category, first_name, supported): (i32, i32, String, bool) = env
            .eval(
                r#"
                local categories = C_SpellDiminish.GetAllSpellDiminishCategories(Enum.SpellDiminishRuleset.PvP)
                return #categories,
                    categories[1].category,
                    categories[1].name,
                    C_SpellDiminish.IsSystemSupported()
                "#,
            )
            .expect("query spell diminish categories");

        assert_eq!(count, 8);
        assert_eq!(first_category, 0);
        assert_eq!(first_name, "Root");
        assert!(supported);
    }
}
