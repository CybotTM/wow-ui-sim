//! Panel-toggle verbs.
//!
//! Migrates 10 entries off `GLOBAL_NIL_STUBS` (`ToggleDropDownMenu` is
//! already registered from `create_frame/dropdown_api.rs`):
//!
//! | Verb                | Panel token    |
//! |---------------------|----------------|
//! | ToggleCharacter     | Character      |
//! | ToggleSpellBook     | SpellBook      |
//! | ToggleTalentFrame   | Talent         |
//! | ToggleQuestLog      | QuestLog       |
//! | ToggleWorldMap      | WorldMap       |
//! | ToggleFriendsFrame  | Friends        |
//! | ToggleGuildFrame    | Guild          |
//! | ToggleHelpFrame     | Help           |
//! | ToggleSocialPanel   | Social         |
//! | ToggleMinimap       | Minimap        |
//!
//! Each verb flips membership in `SimState.open_panels`. If a matching
//! Rust frame exists by canonical name (e.g. `CharacterFrame`), its
//! visibility is toggled in sync; otherwise the set is authoritative.
//!
//! Registered from `register_tail_globals` after `missing_surface`.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, call_function_state, table_get};
use rilua::Val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult};

/// (panel_token, companion_frame_name)
const PANELS: &[(&'static str, &str)] = &[
    ("Character", "CharacterFrame"),
    ("SpellBook", "SpellBookFrame"),
    ("Talent", "PlayerTalentFrame"),
    ("QuestLog", "QuestLogFrame"),
    ("WorldMap", "WorldMapFrame"),
    ("Friends", "FriendsFrame"),
    ("Guild", "GuildFrame"),
    ("Help", "HelpFrame"),
    ("Social", "SocialFrame"),
    ("Minimap", "MinimapCluster"),
];

fn toggle_panel(state: &mut LuaState, panel: &'static str, frame: &'static str) -> LuaResult<()> {
    if try_toggle_panel_via_frame_method(state, frame)? {
        sync_open_panel_membership(state, panel, frame);
        return Ok(());
    }

    let is_now_open = {
        let mut st = borrow_state_mut(state)?;
        if st.open_panels.contains(panel) {
            st.open_panels.remove(panel);
            false
        } else {
            st.open_panels.insert(panel.to_string());
            true
        }
    };
    sync_frame_visibility(state, frame, is_now_open);
    Ok(())
}

fn try_toggle_panel_via_frame_method(state: &mut LuaState, frame_name: &str) -> LuaResult<bool> {
    let global = Val::Table(state.global);
    let frame = table_get(state, global, frame_name);
    let Val::Table(_) = frame else {
        return Ok(false);
    };

    let handler = table_get(state, frame, "HandleUserActionToggleSelf");
    let Val::Function(_) = handler else {
        return Ok(false);
    };

    let _ = call_function_state(state, handler, &[frame])?;
    Ok(true)
}

fn sync_open_panel_membership(state: &mut LuaState, panel: &str, frame_name: &str) {
    let is_open = borrow_state(state)
        .ok()
        .and_then(|st| {
            st.widgets
                .get_id_by_name(frame_name)
                .and_then(|frame_id| st.widgets.get(frame_id).map(|frame| frame.visible))
        })
        .unwrap_or(false);

    let Ok(mut st) = borrow_state_mut(state) else {
        return;
    };
    if is_open {
        st.open_panels.insert(panel.to_string());
    } else {
        st.open_panels.remove(panel);
    }
}

fn sync_frame_visibility(state: &mut LuaState, frame_name: &str, visible: bool) {
    let Ok(mut st) = borrow_state_mut(state) else {
        return;
    };
    let Some(frame_id) = st.widgets.get_id_by_name(frame_name) else {
        return;
    };
    st.set_frame_visible(frame_id, visible);
}

macro_rules! define_toggle {
    ($fn_name:ident, $panel:literal, $frame:literal) => {
        fn $fn_name(state: &mut LuaState) -> LuaResult<u32> {
            toggle_panel(state, $panel, $frame)?;
            Ok(0)
        }
    };
}

define_toggle!(toggle_character, "Character", "CharacterFrame");
define_toggle!(toggle_spell_book, "SpellBook", "SpellBookFrame");
define_toggle!(toggle_talent_frame, "Talent", "PlayerTalentFrame");
define_toggle!(toggle_quest_log, "QuestLog", "QuestLogFrame");
define_toggle!(toggle_world_map, "WorldMap", "WorldMapFrame");
define_toggle!(toggle_friends_frame, "Friends", "FriendsFrame");
define_toggle!(toggle_guild_frame, "Guild", "GuildFrame");
define_toggle!(toggle_help_frame, "Help", "HelpFrame");
define_toggle!(toggle_social_panel, "Social", "SocialFrame");
define_toggle!(toggle_minimap, "Minimap", "MinimapCluster");

/// Panel-token table for introspection (exposed to docs + tests).
pub fn panel_tokens() -> &'static [(&'static str, &'static str)] {
    PANELS
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "ToggleCharacter", toggle_character)?;
    LuaApiMut::register_function(lua, "ToggleSpellBook", toggle_spell_book)?;
    LuaApiMut::register_function(lua, "ToggleTalentFrame", toggle_talent_frame)?;
    LuaApiMut::register_function(lua, "ToggleQuestLog", toggle_quest_log)?;
    LuaApiMut::register_function(lua, "ToggleWorldMap", toggle_world_map)?;
    LuaApiMut::register_function(lua, "ToggleFriendsFrame", toggle_friends_frame)?;
    LuaApiMut::register_function(lua, "ToggleGuildFrame", toggle_guild_frame)?;
    LuaApiMut::register_function(lua, "ToggleHelpFrame", toggle_help_frame)?;
    LuaApiMut::register_function(lua, "ToggleSocialPanel", toggle_social_panel)?;
    LuaApiMut::register_function(lua, "ToggleMinimap", toggle_minimap)?;
    Ok(())
}
