//! C_ProfSpecs and C_SettingsUtil namespace stubs.

use crate::lua_api::SimState;
use crate::traits::{
    TRAIT_DEFINITION_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB, TRAIT_TREE_DB, TraitTreeInfo,
};
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

const DEFAULT_SPEC_SKILL_LINE_ID: i32 = 164;
const DEFAULT_SPEC_TREE_ID: i32 = 790;
const SPEC_TAB_DESCRIPTION: &str =
    "Develop your Blacksmithing specialization through a seeded knowledge tree.";
const SPEC_SOURCE_TEXT: &str = "Earn Blacksmithing knowledge from profession activities.";
const SPEC_CURRENCY_NAME: &str = "Blacksmithing Knowledge";

/// Register profession and settings namespace stubs.
pub fn register_profession_stubs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_c_prof_specs(lua, Rc::clone(&state))?;
    register_c_settings_util(lua)?;
    Ok(())
}

/// C_ProfSpecs namespace - profession specialization data.
fn register_c_prof_specs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let t = prof_specs_namespace(lua)?;
    register_prof_specs_static_queries(lua, &t, Rc::clone(&state))?;
    register_prof_specs_stateful_queries(lua, &t, state)?;
    lua.globals().set("C_ProfSpecs", t)?;
    Ok(())
}

fn prof_specs_namespace(lua: &Lua) -> Result<mlua::Table> {
    let globals = lua.globals();
    match globals.get::<Value>("C_ProfSpecs")? {
        Value::Table(existing) => Ok(existing),
        _ => {
            let created = lua.create_table()?;
            globals.set("C_ProfSpecs", created.clone())?;
            Ok(created)
        }
    }
}

fn register_prof_specs_static_queries(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("ShouldShowSpecTab", lua.create_function(|_, ()| Ok(true))?)?;
    t.set(
        "GetDefaultSpecSkillLine",
        lua.create_function(|_, ()| Ok(Some(DEFAULT_SPEC_SKILL_LINE_ID)))?,
    )?;
    t.set(
        "SkillLineHasSpecialization",
        lua.create_function(|_, skill_line_id: i32| {
            Ok(skill_line_id == DEFAULT_SPEC_SKILL_LINE_ID)
        })?,
    )?;
    t.set(
        "GetConfigIDForSkillLine",
        lua.create_function(|_, skill_line_id: i32| {
            Ok(if skill_line_id == DEFAULT_SPEC_SKILL_LINE_ID {
                1
            } else {
                0
            })
        })?,
    )?;
    t.set(
        "GetSpecTabIDsForSkillLine",
        lua.create_function(|lua, skill_line_id: i32| {
            let ids = lua.create_table()?;
            if skill_line_id == DEFAULT_SPEC_SKILL_LINE_ID {
                ids.set(1, DEFAULT_SPEC_TREE_ID)?;
            }
            Ok(ids)
        })?,
    )?;
    t.set(
        "GetSpecTabInfo",
        lua.create_function(|lua, ()| build_spec_tab_info(lua))?,
    )?;
    t.set(
        "GetTabInfo",
        lua.create_function(|lua, tree_id: i32| build_prof_spec_tab(lua, tree_id))?,
    )?;
    t.set(
        "GetRootPathForTab",
        lua.create_function(|_, tree_id: i32| {
            Ok(tab_matches_default_tree(tree_id)
                .then(default_prof_spec_root_node_id)
                .flatten())
        })?,
    )?;
    t.set(
        "GetStateForTab",
        lua.create_function(|_, (tree_id, config_id): (i32, i32)| {
            if tab_matches_default_tree(tree_id) && config_id == 1 {
                Ok(Some(EnumProfessionsSpecTabState::Unlocked as i32))
            } else {
                Ok(Option::<i32>::None)
            }
        })?,
    )?;
    t.set(
        "GetSpendCurrencyForPath",
        lua.create_function(|_, path_id: i32| {
            Ok(path_exists(path_id)
                .then(default_prof_spec_currency_id)
                .flatten())
        })?,
    )?;
    t.set(
        "GetSpendEntryForPath",
        lua.create_function(|_, path_id: i32| {
            Ok(node_first_entry_id(path_id as u32).unwrap_or(0) as i32)
        })?,
    )?;
    t.set(
        "GetUnlockEntryForPath",
        lua.create_function(|_, _path_id: i32| Ok(0i32))?,
    )?;
    t.set(
        "GetChildrenForPath",
        lua.create_function(|lua, path_id: i32| build_children_table(lua, path_id))?,
    )?;
    t.set(
        "GetDescriptionForPath",
        lua.create_function(|_, path_id: i32| Ok(node_display_description(path_id as u32)))?,
    )?;
    t.set(
        "GetSourceTextForPath",
        lua.create_function(|_, (path_id, _config_id): (i32, i32)| {
            Ok(if path_exists(path_id) {
                SPEC_SOURCE_TEXT.to_string()
            } else {
                String::new()
            })
        })?,
    )?;
    t.set(
        "GetPerksForPath",
        lua.create_function(|lua, _path_id: i32| lua.create_table())?,
    )?;
    t.set(
        "GetStateForPerk",
        lua.create_function(|_, (_perk_id, _config_id): (i32, i32)| {
            Ok(EnumProfessionsSpecPerkState::Unearned as i32)
        })?,
    )?;
    t.set(
        "GetUnlockRankForPerk",
        lua.create_function(|_, _perk_id: i32| Ok(0i32))?,
    )?;
    t.set(
        "GetEntryIDForPerk",
        lua.create_function(|_, _perk_id: i32| Ok(0i32))?,
    )?;
    t.set(
        "GetDescriptionForPerk",
        lua.create_function(|_, _perk_id: i32| Ok(String::new()))?,
    )?;
    t.set(
        "GetNewSpecReminderProfName",
        lua.create_function(|_, ()| Ok(Option::<String>::None))?,
    )?;
    t.set(
        "ShouldShowPointsReminder",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "ShouldShowPointsReminderForSkillLine",
        lua.create_function(|_, _skill_line_id: i32| Ok(false))?,
    )?;
    t.set(
        "CanRefundPath",
        lua.create_function(|_, (_path_id, _config_id): (i32, i32)| Ok(false))?,
    )?;
    t.set(
        "CanUnlockTab",
        lua.create_function(|_, (_tree_id, _config_id): (i32, i32)| Ok(false))?,
    )?;
    register_currency_info_query(lua, t, state)?;
    Ok(())
}

fn register_currency_info_query(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetCurrencyInfoForSkillLine",
        lua.create_function(move |lua, skill_line_id: i32| {
            build_currency_info_for_skill_line(lua, &state, skill_line_id)
        })?,
    )?;
    Ok(())
}

fn register_prof_specs_stateful_queries(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetStateForPath",
        lua.create_function(move |_, (path_id, _config_id): (i32, i32)| {
            Ok(path_state(&state.borrow(), path_id))
        })?,
    )?;
    Ok(())
}

fn build_spec_tab_info(lua: &Lua) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("enabled", true)?;
    info.set("errorReason", "")?;
    Ok(info)
}

fn build_prof_spec_tab(lua: &Lua, tree_id: i32) -> Result<Value> {
    if !tab_matches_default_tree(tree_id) {
        return Ok(Value::Nil);
    }

    let Some(root_node_id) = default_prof_spec_root_node_id() else {
        return Ok(Value::Nil);
    };

    let info = lua.create_table()?;
    info.set("treeID", DEFAULT_SPEC_TREE_ID)?;
    info.set("rootNodeID", root_node_id as i32)?;
    info.set("name", "Blacksmithing Specialization")?;
    info.set("description", SPEC_TAB_DESCRIPTION)?;
    info.set("rootIconID", node_icon_file_id(root_node_id) as i32)?;
    info.set("highlights", lua.create_table()?)?;
    Ok(Value::Table(info))
}

fn build_currency_info_for_skill_line(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    skill_line_id: i32,
) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    let Some(currency_id) = (skill_line_id == DEFAULT_SPEC_SKILL_LINE_ID)
        .then(default_prof_spec_currency_id)
        .flatten()
    else {
        info.set("numAvailable", 0)?;
        info.set("numTotal", 0)?;
        info.set("spentPercentage", 0)?;
        info.set("currencyName", "")?;
        return Ok(info);
    };

    let spent = state.borrow().talents.spent_for_currency(currency_id);
    let total = super::traits_api::max_points_for_currency(currency_id);
    let available = total.saturating_sub(spent);
    let spent_percentage = if total == 0 {
        0
    } else {
        ((spent * 100) / total) as i32
    };
    info.set("numAvailable", available as i32)?;
    info.set("numTotal", total as i32)?;
    info.set("spentPercentage", spent_percentage)?;
    info.set("currencyName", SPEC_CURRENCY_NAME)?;
    Ok(info)
}

fn build_children_table(lua: &Lua, path_id: i32) -> Result<mlua::Table> {
    let child_ids = children_for_path(path_id as u32);
    let result = lua.create_table()?;
    for (index, child_id) in child_ids.iter().enumerate() {
        result.set(index as i64 + 1, *child_id as i64)?;
    }
    Ok(result)
}

fn default_prof_spec_tree() -> Option<&'static TraitTreeInfo> {
    TRAIT_TREE_DB.get(&(DEFAULT_SPEC_TREE_ID as u32))
}

fn default_prof_spec_currency_id() -> Option<u32> {
    default_prof_spec_tree()?
        .currency_ids
        .iter()
        .copied()
        .find(|currency_id| super::traits_api::max_points_for_currency(*currency_id) > 0)
        .or_else(|| default_prof_spec_tree()?.currency_ids.first().copied())
}

fn default_prof_spec_root_node_id() -> Option<u32> {
    let tree = default_prof_spec_tree()?;
    let child_map = tree_child_map(tree);
    tree.node_ids.iter().copied().find(|node_id| {
        let has_children = child_map
            .get(node_id)
            .is_some_and(|children| !children.is_empty());
        let has_parents = TRAIT_NODE_DB
            .get(node_id)
            .is_some_and(|node| !node.edges.is_empty());
        has_children && !has_parents
    })
}

fn tree_child_map(tree: &TraitTreeInfo) -> std::collections::BTreeMap<u32, Vec<u32>> {
    let mut child_map: std::collections::BTreeMap<u32, Vec<u32>> =
        std::collections::BTreeMap::new();
    for &node_id in tree.node_ids {
        let Some(node) = TRAIT_NODE_DB.get(&node_id) else {
            continue;
        };
        for edge in node.edges {
            child_map
                .entry(edge.source_node_id)
                .or_default()
                .push(node_id);
        }
    }
    child_map
}

fn tree_depth_map(
    tree: &TraitTreeInfo,
    root_node_id: u32,
) -> std::collections::BTreeMap<u32, usize> {
    let child_map = tree_child_map(tree);
    let mut depths = std::collections::BTreeMap::new();
    let mut queue = std::collections::VecDeque::from([(root_node_id, 0usize)]);

    while let Some((node_id, depth)) = queue.pop_front() {
        if depths.insert(node_id, depth).is_some() {
            continue;
        }

        for child_id in child_map.get(&node_id).into_iter().flatten() {
            queue.push_back((*child_id, depth + 1));
        }
    }

    depths
}

fn children_for_path(path_id: u32) -> Vec<u32> {
    let Some(tree) = default_prof_spec_tree() else {
        return Vec::new();
    };
    let Some(root_node_id) = default_prof_spec_root_node_id() else {
        return Vec::new();
    };
    let depth_map = tree_depth_map(tree, root_node_id);
    if depth_map.get(&path_id).copied().unwrap_or(usize::MAX) >= 2 {
        return Vec::new();
    }

    tree_child_map(tree).remove(&path_id).unwrap_or_default()
}

fn tab_matches_default_tree(tree_id: i32) -> bool {
    tree_id == DEFAULT_SPEC_TREE_ID
}

fn path_exists(path_id: i32) -> bool {
    TRAIT_NODE_DB.contains_key(&(path_id as u32))
}

fn node_first_entry_id(node_id: u32) -> Option<u32> {
    TRAIT_NODE_DB.get(&node_id)?.entry_ids.first().copied()
}

fn node_display_name(node_id: u32) -> String {
    node_first_entry_id(node_id)
        .and_then(super::traits_api_node::trait_entry_name)
        .unwrap_or_else(|| "Blacksmithing Path".to_string())
}

fn node_display_description(node_id: u32) -> String {
    node_first_entry_id(node_id)
        .and_then(|entry_id| super::traits_api_node::trait_entry_description(entry_id, 1))
        .unwrap_or_else(|| format!("Advance {}.", node_display_name(node_id)))
}

fn node_icon_file_id(node_id: u32) -> u32 {
    let Some(entry_id) = node_first_entry_id(node_id) else {
        return 0;
    };
    let Some(entry) = TRAIT_ENTRY_DB.get(&entry_id) else {
        return 0;
    };
    let Some(def) = TRAIT_DEFINITION_DB.get(&entry.definition_id) else {
        return 0;
    };
    if def.override_icon != 0 {
        return def.override_icon;
    }
    let Some(spell_id) = super::traits_api_node::trait_entry_display_spell_id(def) else {
        return 0;
    };
    crate::spells::get_spell(spell_id)
        .map(|spell| spell.icon_file_data_id)
        .unwrap_or(0)
}

fn path_state(state: &SimState, path_id: i32) -> Option<i32> {
    let node_id = path_id as u32;
    let node = TRAIT_NODE_DB.get(&node_id)?;
    let current_rank = state.talents.node_ranks.get(&node_id).copied().unwrap_or(0);
    let max_ranks = super::traits_api_node::node_max_ranks(node).max(1) as u32;
    let state = if current_rank >= max_ranks {
        EnumProfessionsSpecPathState::Completed
    } else {
        EnumProfessionsSpecPathState::Progressing
    };
    Some(state as i32)
}

#[derive(Clone, Copy)]
enum EnumProfessionsSpecTabState {
    Unlocked = 1,
}

#[derive(Clone, Copy)]
enum EnumProfessionsSpecPathState {
    Progressing = 1,
    Completed = 2,
}

#[derive(Clone, Copy)]
enum EnumProfessionsSpecPerkState {
    Unearned = 0,
}

/// C_SettingsUtil namespace - settings loading and panel management.
fn register_c_settings_util(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "NotifySettingsLoaded",
        lua.create_function(|lua, ()| {
            let fire: mlua::Function = lua.globals().get("FireEvent")?;
            fire.call::<()>(lua.create_string("SETTINGS_LOADED")?)?;
            Ok(())
        })?,
    )?;
    t.set(
        "OpenSettingsPanel",
        lua.create_function(|_, _args: mlua::Variadic<Value>| Ok(()))?,
    )?;
    lua.globals().set("C_SettingsUtil", t)?;
    Ok(())
}
