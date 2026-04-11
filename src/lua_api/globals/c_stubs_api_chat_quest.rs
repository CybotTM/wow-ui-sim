//! Chat, quest, and macro stubs split from c_stubs_api.rs.

use mlua::{Lua, MultiValue, Result, Value};

#[derive(Clone, Copy)]
pub(super) struct SeededMacro {
    pub(super) id: i32,
    pub(super) name: &'static str,
    pub(super) icon: &'static str,
    pub(super) body: &'static str,
}

pub(super) const SEEDED_ACCOUNT_MACROS: &[SeededMacro] = &[
    SeededMacro {
        id: 1,
        name: "Raid Beacon",
        icon: "Interface\\Icons\\INV_Misc_QuestionMark",
        body: "/rw Stack on star",
    },
    SeededMacro {
        id: 2,
        name: "Pull Timer",
        icon: "Interface\\Icons\\INV_Misc_PocketWatch_01",
        body: "/pull 10",
    },
];

pub(super) const SEEDED_CHARACTER_MACROS: &[SeededMacro] = &[SeededMacro {
    id: 121,
    name: "Crusader",
    icon: "Interface\\Icons\\Spell_Holy_CrusaderAura",
    body: "/cast Crusader Aura",
}];

/// Quest-related global functions used by ObjectiveTracker.
pub(super) fn register_quest_global_functions(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let g = lua.globals();
    g.set(
        "IsInInstance",
        lua.create_function(move |_, ()| {
            let s = state.borrow();
            Ok((s.world.in_instance, s.world.instance_type.clone()))
        })?,
    )?;
    register_quest_query_stubs(lua, &g)?;
    register_quest_leaderboard_functions(lua, &g)?;
    Ok(())
}

/// Stateless quest query/popup stubs.
fn register_quest_query_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "IsQuestSequenced",
        lua.create_function(|_, _quest_id: i32| Ok(false))?,
    )?;
    g.set(
        "GetQuestLogCompletionText",
        lua.create_function(|_, _log_idx: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetQuestProgressBarPercent",
        lua.create_function(|_, _quest_id: i32| Ok(0.0f64))?,
    )?;
    g.set(
        "QuestMapFrame_GetFocusedQuestID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "IsModifiedClick",
        lua.create_function(|_, _action: String| Ok(false))?,
    )?;
    g.set(
        "GetQuestLink",
        lua.create_function(|_, _quest_id: i32| Ok(Value::Nil))?,
    )?;
    g.set("IsInJailersTower", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "IsOnGroundFloorInJailersTower",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetNumAutoQuestPopUps",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetAutoQuestPopUp",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetQuestLogSpecialItemInfo",
        lua.create_function(|_, _log_idx: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetTasksTable",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "ExpandQuestHeader",
        lua.create_function(|_, (_idx, _no_update): (i32, Option<bool>)| Ok(()))?,
    )?;
    g.set(
        "CollapseQuestHeader",
        lua.create_function(|_, (_idx, _no_update): (i32, Option<bool>)| Ok(()))?,
    )?;
    Ok(())
}

/// GetNumQuestLeaderBoards / GetQuestLogLeaderBoard - quest objective data.
/// Delegates to c_quest_api which owns the single source of truth for quest data.
fn register_quest_leaderboard_functions(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetNumQuestLeaderBoards",
        lua.create_function(|_, log_idx: Option<i32>| {
            Ok(super::c_quest_api::num_quest_leaderboards(log_idx.unwrap_or(0)))
        })?,
    )?;
    g.set(
        "GetQuestLogLeaderBoard",
        lua.create_function(
            |_, (obj_idx, log_idx, _suppress): (i32, i32, Option<bool>)| {
                Ok(super::c_quest_api::quest_leaderboard_entry(
                    log_idx, obj_idx,
                ))
            },
        )?,
    )?;
    Ok(())
}

/// Chat window management stubs needed by FloatingChatFrame.
pub(super) fn register_chat_window_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "SetChatWindowLocked",
        lua.create_function(|_, (_id, _locked): (i32, bool)| Ok(()))?,
    )?;
    g.set(
        "SetChatWindowUninteractable",
        lua.create_function(|_, (_id, _flag): (i32, bool)| Ok(()))?,
    )?;
    g.set(
        "GetChatWindowSavedDimensions",
        lua.create_function(|_, _id: i32| Ok((430.0f64, 120.0f64)))?,
    )?;
    g.set(
        "SetChatWindowColor",
        lua.create_function(|_, (_id, _r, _g, _b): (i32, f64, f64, f64)| Ok(()))?,
    )?;
    g.set(
        "SetChatWindowAlpha",
        lua.create_function(|_, (_id, _a): (i32, f64)| Ok(()))?,
    )?;
    g.set(
        "GetChatWindowSavedPosition",
        lua.create_function(|_, _id: i32| {
            // Returns: point, yOffset, xOffset, relativePoint
            Ok(("BOTTOMLEFT", 0.0f64, 0.0f64, "BOTTOMLEFT"))
        })?,
    )?;
    // ChangeChatColor: sets r,g,b on ChatTypeInfo[type]
    g.set(
        "ChangeChatColor",
        lua.create_function(|lua, (ct, r, g, b): (String, f64, f64, f64)| {
            let cti: mlua::Table = lua
                .globals()
                .get::<mlua::Table>("ChatTypeInfo")?
                .get(&*ct)?;
            cti.set("r", r)?;
            cti.set("g", g)?;
            cti.set("b", b)?;
            Ok(())
        })?,
    )?;
    Ok(())
}

/// Chat-related global function stubs needed by Blizzard_ChatFrame.
pub(super) fn register_chat_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    // GetChatTypeIndex: deterministic integer from chat type name
    g.set(
        "GetChatTypeIndex",
        lua.create_function(|_, name: String| {
            let hash = name
                .bytes()
                .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
            Ok((hash % 50 + 1) as i32)
        })?,
    )?;
    // CreateSecureDelegate: no taint system, return the function as-is
    g.set(
        "CreateSecureDelegate",
        lua.create_function(|_, func: mlua::Function| Ok(func))?,
    )?;
    // GetChatWindowInfo: return defaults (only window 1 is shown)
    // Returns: name, fontSize, r, g, b, alpha, shown, locked, docked, uninteractable
    // Default color is black (0,0,0) at 25% alpha, matching DEFAULT_CHATFRAME_COLOR/ALPHA
    g.set(
        "GetChatWindowInfo",
        lua.create_function(|_, id: i32| {
            let name = format!("ChatFrame{id}");
            let shown = id == 1;
            Ok((
                name, 14.0f64, 0.0f64, 0.0f64, 0.0f64, 0.25f64, shown, false, false, false,
            ))
        })?,
    )?;
    // GetChatWindowMessages/GetChatWindowChannels: return no message types or channels
    g.set(
        "GetChatWindowMessages",
        lua.create_function(|_, _id: i32| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "GetChatWindowChannels",
        lua.create_function(|_, _id: i32| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "GetDefaultLanguage",
        lua.create_function(|_, ()| Ok("Common"))?,
    )?;
    g.set(
        "GetAlternativeDefaultLanguage",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    Ok(())
}

/// C_Macro namespace - macro management stubs.
pub(super) fn register_c_macro(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "SetMacroExecuteLineCallback",
        lua.create_function(|_, _cb: Value| Ok(()))?,
    )?;
    t.set("GetMacroName", lua.create_function(get_macro_name)?)?;
    t.set(
        "GetSelectedMacroIcon",
        lua.create_function(get_selected_macro_icon)?,
    )?;
    t.set("GetMacroInfo", lua.create_function(get_macro_info)?)?;
    t.set("GetNumMacros", lua.create_function(get_num_macros)?)?;
    lua.globals().set("C_Macro", t)?;
    register_macro_globals(lua)?;
    Ok(())
}

fn register_macro_globals(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set("GetMacroInfo", lua.create_function(get_macro_info)?)?;
    globals.set("GetNumMacros", lua.create_function(get_num_macros)?)?;
    globals.set(
        "GetMacroIndexByName",
        lua.create_function(get_macro_index_by_name)?,
    )?;
    Ok(())
}

fn get_num_macros(_: &Lua, (): ()) -> Result<(i32, i32)> {
    Ok((
        SEEDED_ACCOUNT_MACROS.len() as i32,
        SEEDED_CHARACTER_MACROS.len() as i32,
    ))
}

fn get_macro_info(lua: &Lua, macro_id: Value) -> Result<MultiValue> {
    let Some(macro_info) = lookup_macro(macro_id) else {
        return Ok(MultiValue::from_vec(vec![Value::Nil]));
    };
    Ok(MultiValue::from_vec(vec![
        Value::String(lua.create_string(macro_info.name)?),
        Value::String(lua.create_string(macro_info.icon)?),
        Value::String(lua.create_string(macro_info.body)?),
    ]))
}

fn get_macro_name(lua: &Lua, macro_id: Value) -> Result<Value> {
    match lookup_macro(macro_id) {
        Some(macro_info) => Ok(Value::String(lua.create_string(macro_info.name)?)),
        None => Ok(Value::Nil),
    }
}

fn get_selected_macro_icon(lua: &Lua, macro_id: Value) -> Result<Value> {
    match lookup_macro(macro_id) {
        Some(macro_info) => Ok(Value::String(lua.create_string(macro_info.icon)?)),
        None => Ok(Value::Integer(0)),
    }
}

fn get_macro_index_by_name(_: &Lua, name: String) -> Result<i32> {
    Ok(all_seeded_macros()
        .find(|macro_info| macro_info.name.eq_ignore_ascii_case(&name))
        .map(|macro_info| macro_info.id)
        .unwrap_or(0))
}

fn lookup_macro(macro_id: Value) -> Option<&'static SeededMacro> {
    let macro_id = match macro_id {
        Value::Integer(index) => i32::try_from(index).ok()?,
        Value::Number(index) => index as i32,
        Value::String(name) => return lookup_macro_by_name(name.to_string_lossy().as_ref()),
        _ => return None,
    };
    lookup_macro_by_id(macro_id)
}

fn lookup_macro_by_id(macro_id: i32) -> Option<&'static SeededMacro> {
    all_seeded_macros().find(|macro_info| macro_info.id == macro_id)
}

fn lookup_macro_by_name(name: &str) -> Option<&'static SeededMacro> {
    all_seeded_macros().find(|macro_info| macro_info.name.eq_ignore_ascii_case(name))
}

fn all_seeded_macros() -> impl Iterator<Item = &'static SeededMacro> {
    SEEDED_ACCOUNT_MACROS
        .iter()
        .chain(SEEDED_CHARACTER_MACROS.iter())
}
