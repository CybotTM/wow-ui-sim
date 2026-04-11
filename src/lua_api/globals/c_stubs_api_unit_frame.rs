//! Unit frame, combat, and powerbar color stubs split from c_stubs_api.rs.

use mlua::{Lua, Result, Value};

/// POWERBAR_PREDICTION_COLOR_* globals used by PowerBarColorUtil.lua at parse time.
const POWERBAR_COLORS: &[(&str, f64, f64, f64)] = &[
    ("POWERBAR_PREDICTION_COLOR_MANA", 0.0, 0.0, 1.0),
    ("POWERBAR_PREDICTION_COLOR_RAGE", 1.0, 0.0, 0.0),
    ("POWERBAR_PREDICTION_COLOR_FOCUS", 1.0, 0.5, 0.25),
    ("POWERBAR_PREDICTION_COLOR_ENERGY", 1.0, 1.0, 0.0),
    ("POWERBAR_PREDICTION_COLOR_RUNIC_POWER", 0.0, 0.82, 1.0),
    ("POWERBAR_PREDICTION_COLOR_LUNAR_POWER", 0.3, 0.52, 0.9),
    ("POWERBAR_PREDICTION_COLOR_MAELSTROM", 0.0, 0.5, 1.0),
    ("POWERBAR_PREDICTION_COLOR_INSANITY", 0.4, 0.0, 0.8),
    ("POWERBAR_PREDICTION_COLOR_FURY", 0.788, 0.259, 0.992),
    ("POWERBAR_PREDICTION_COLOR_PAIN", 1.0, 0.612, 0.0),
];

/// Resolve a texture path or file data ID to a WoW interface path.
/// Global function stubs needed by Blizzard_UnitFrame.
pub(super) fn register_unit_frame_global_stubs(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_combat_state_globals(lua, state)?;
    register_unit_frame_stateless_stubs(lua)?;
    register_unit_frame_global_stubs_2(lua)?;
    Ok(())
}

fn register_combat_state_globals(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let g = lua.globals();
    let s2 = std::rc::Rc::clone(&state);
    g.set(
        "InCombatLockdown",
        lua.create_function(move |_, ()| Ok(s2.borrow().player.in_combat))?,
    )?;
    g.set(
        "IsResting",
        lua.create_function(move |_, ()| Ok(state.borrow().player.is_resting))?,
    )?;
    Ok(())
}

fn register_unit_frame_stateless_stubs(lua: &Lua) -> Result<()> {
    register_pvp_and_lfg_stubs(lua)?;
    register_raid_and_billing_stubs(lua)?;
    install_set_portrait_to_texture(lua)
}

fn register_pvp_and_lfg_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("IsPVPTimerRunning", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("GetPVPTimer", lua.create_function(|_, ()| Ok(0.0f64))?)?;
    g.set(
        "GetReadyCheckStatus",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )?;
    g.set(
        "HasLFGRestrictions",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetPartyLFGID",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "RequestGuildPartyState",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetLFGCategoryForID",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "IsEveryoneAssistant",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "WorldLootObjectExists",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    Ok(())
}

fn register_raid_and_billing_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("IsInRaid", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetRaidRosterInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    g.set("PartialPlayTime", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("NoPlayTime", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetBillingTimeRested",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    Ok(())
}

fn install_set_portrait_to_texture(lua: &Lua) -> Result<()> {
    lua.load(SET_PORTRAIT_TO_TEXTURE_LUA).exec()
}

const SET_PORTRAIT_TO_TEXTURE_LUA: &str = r#"
    function SetPortraitToTexture(tex, path)
        if tex and tex.SetTexture then
            tex:SetTexture(path)
            if tex.GetNumMaskTextures and tex:GetNumMaskTextures() == 0 and tex.GetParent then
                local parent = tex:GetParent()
                if parent and parent.CreateMaskTexture then
                    local mask = parent:CreateMaskTexture(nil, "ARTWORK")
                    mask:SetTexture("Interface\\CharacterFrame\\TempPortraitAlphaMask")
                    mask:SetAllPoints(tex)
                    tex:AddMaskTexture(mask)
                end
            end
        end
    end
"#;

/// Continuation of unit-frame global stubs (combat, arena, UIParent handlers).
fn register_unit_frame_global_stubs_2(lua: &Lua) -> Result<()> {
    register_combat_and_arena_stubs(lua)?;
    register_uiparent_entering_world_stubs(lua)?;
    Ok(())
}

/// Combat, threat, arena, pet, and misc OnUpdate handler stubs.
fn register_combat_and_arena_stubs(lua: &Lua) -> Result<()> {
    register_threat_and_arena_stubs(lua)?;
    register_pet_and_misc_stubs(lua)
}

fn register_threat_and_arena_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "GetUnitTotalModifiedMaxHealthPercent",
        lua.create_function(|_, _unit: Option<String>| Ok(0.0f64))?,
    )?;
    g.set(
        "IsThreatWarningEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetThreatStatusColor",
        lua.create_function(|_, _status: i32| Ok((1.0f64, 1.0f64, 1.0f64)))?,
    )?;
    g.set("LE_REALM_RELATION_VIRTUAL", 3i32)?;
    g.set(
        "IsActiveBattlefieldArena",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetNumArenaOpponents",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetBattlefieldEstimatedWaitTime",
        lua.create_function(|_, _index: Value| Ok(0i32))?,
    )?;
    Ok(())
}

fn register_pet_and_misc_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("PetUsesPetFrame", lua.create_function(|_, ()| Ok(true))?)?;
    g.set(
        "UnitIsPossessed",
        lua.create_function(|_, _unit: Option<String>| Ok(false))?,
    )?;
    g.set(
        "GetReleaseTimeRemaining",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "FCF_OnUpdate",
        lua.create_function(|_, _elapsed: Option<f64>| Ok(()))?,
    )?;
    g.set(
        "HelpOpenWebTicketButton_OnUpdate",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    g.set(
        "GetLootSpecialization",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    Ok(())
}

/// UIParent PLAYER_ENTERING_WORLD handler stubs.
fn register_uiparent_entering_world_stubs(lua: &Lua) -> Result<()> {
    register_entering_world_queries(lua)?;
    register_entering_world_actions(lua)
}

fn register_entering_world_queries(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "GetSpellConfirmationPromptsInfo",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "ResurrectGetOfferer",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetActiveLootRollIDs",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "GetTutorialsEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(())
}

fn register_entering_world_actions(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "BoostTutorial_AttemptLoad",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "ExpansionTrial_CheckLoadUI",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "SubscriptionInterstitial_LoadUI",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "ShowResurrectRequest",
        lua.create_function(|_, _offerer: String| Ok(()))?,
    )?;
    g.set(
        "GroupLootContainer_AddRoll",
        lua.create_function(|_, (_id, _dur): (Value, Value)| Ok(()))?,
    )?;
    g.set(
        "RemixArtifactTutorialUI_LoadUI",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    Ok(())
}

fn build_color_entry(
    lua: &Lua,
    r: f64,
    green: f64,
    b: f64,
    get_rgba: &mlua::Function,
    get_rgb: &mlua::Function,
) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set("r", r)?;
    t.set("g", green)?;
    t.set("b", b)?;
    t.set("a", 0.5f64)?;
    t.set("GetRGBA", get_rgba.clone())?;
    t.set("GetRGB", get_rgb.clone())?;
    Ok(t)
}

pub(super) fn register_powerbar_prediction_colors(lua: &Lua) -> Result<()> {
    let get_rgba = lua.create_function(|_, this: mlua::Table| {
        Ok((
            this.get::<f64>("r")?,
            this.get::<f64>("g")?,
            this.get::<f64>("b")?,
            this.get::<f64>("a")?,
        ))
    })?;
    let get_rgb = lua.create_function(|_, this: mlua::Table| {
        Ok((
            this.get::<f64>("r")?,
            this.get::<f64>("g")?,
            this.get::<f64>("b")?,
        ))
    })?;
    let g = lua.globals();
    for &(name, r, green, b) in POWERBAR_COLORS {
        g.set(
            name,
            build_color_entry(lua, r, green, b, &get_rgba, &get_rgb)?,
        )?;
    }
    Ok(())
}
