use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Synthetic spell ID used for spec-activation casts.
pub const SPEC_ACTIVATION_SPELL_ID: u32 = 200_000;

pub(super) fn register_all(lua: &Lua, state: Rc<RefCell<crate::lua_api::SimState>>) -> Result<()> {
    register_c_covenant_sanctum_ui(lua)?;
    register_c_ui_color(lua)?;
    register_c_class_color(lua)?;
    register_c_spec_info(lua, state)?;
    register_c_super_track(lua)?;
    register_c_player_interaction_manager(lua)?;
    register_c_paper_doll_info(lua)?;
    register_c_perks_program(lua)?;
    Ok(())
}

fn register_c_ui_color(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetColors",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    lua.globals().set("C_UIColor", t)?;
    Ok(())
}

fn class_color(name: &str) -> (f32, f32, f32) {
    match name {
        "WARRIOR" => (0.78, 0.61, 0.43),
        "PALADIN" => (0.96, 0.55, 0.73),
        "HUNTER" => (0.67, 0.83, 0.45),
        "ROGUE" => (1.00, 0.96, 0.41),
        "PRIEST" => (1.00, 1.00, 1.00),
        "DEATHKNIGHT" => (0.77, 0.12, 0.23),
        "SHAMAN" => (0.00, 0.44, 0.87),
        "MAGE" => (0.25, 0.78, 0.92),
        "WARLOCK" => (0.53, 0.53, 0.93),
        "MONK" => (0.00, 1.00, 0.60),
        "DRUID" => (1.00, 0.49, 0.04),
        "DEMONHUNTER" => (0.64, 0.19, 0.79),
        "EVOKER" => (0.20, 0.58, 0.50),
        _ => (1.0, 1.0, 1.0),
    }
}

fn register_c_class_color(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetClassColor",
        lua.create_function(|lua, class: String| {
            let (r, g, b) = class_color(&class);
            let a = 1.0f32;
            let color = lua.create_table()?;
            color.set("r", r)?;
            color.set("g", g)?;
            color.set("b", b)?;
            color.set("a", a)?;
            color.set("GetRGB", lua.create_function(move |_, ()| Ok((r, g, b)))?)?;
            color.set(
                "GetRGBA",
                lua.create_function(move |_, ()| Ok((r, g, b, a)))?,
            )?;
            color.set(
                "GenerateHexColor",
                lua.create_function(move |lua, ()| {
                    let hex = format!(
                        "{:02x}{:02x}{:02x}",
                        (r * 255.0) as u8,
                        (g * 255.0) as u8,
                        (b * 255.0) as u8
                    );
                    Ok(Value::String(lua.create_string(&hex)?))
                })?,
            )?;
            color.set(
                "WrapTextInColorCode",
                lua.create_function(move |lua, (_s, text): (Value, String)| {
                    let hex = format!(
                        "{:02x}{:02x}{:02x}",
                        (r * 255.0) as u8,
                        (g * 255.0) as u8,
                        (b * 255.0) as u8
                    );
                    let wrapped = format!("|cff{}{}|r", hex, text);
                    Ok(Value::String(lua.create_string(&wrapped)?))
                })?,
            )?;
            Ok(color)
        })?,
    )?;
    lua.globals().set("C_ClassColor", t)?;
    Ok(())
}

fn register_c_spec_info(lua: &Lua, state: Rc<RefCell<crate::lua_api::SimState>>) -> Result<()> {
    let t = build_c_spec_info_table(lua, Rc::clone(&state))?;
    lua.globals().set("C_SpecializationInfo", t)?;
    register_is_spec_activate_spell(lua)?;
    Ok(())
}

fn build_c_spec_info_table(
    lua: &Lua,
    state: Rc<RefCell<crate::lua_api::SimState>>,
) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    set_spec_info_static_methods(&t, lua)?;
    set_spec_info_get_specialization(&t, lua, Rc::clone(&state))?;
    set_spec_info_set_specialization(&t, lua, state)?;
    Ok(t)
}

fn make_get_spells_display(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|lua, spec_id: i32| {
        use crate::spec_display_spells;
        let tbl = lua.create_table()?;
        for (i, entry) in spec_display_spells::spells_for_spec(spec_id as u32).enumerate() {
            tbl.set(i as i64 + 1, entry.spell_id)?;
        }
        Ok(tbl)
    })
}

fn make_get_specialization_info(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|lua, idx: i32| {
        use crate::specializations;
        let specs: Vec<_> = specializations::specs_for_class(2u32).collect();
        let i = (idx - 1).clamp(0, specs.len() as i32 - 1) as usize;
        let spec = specs[i];
        Ok(mlua::MultiValue::from_vec(vec![
            Value::Integer(spec.id as i64),
            Value::String(lua.create_string(spec.name)?),
            Value::String(lua.create_string(spec.description)?),
            Value::Integer(spec.icon_file_data_id as i64),
            Value::String(lua.create_string(spec.role)?),
            Value::Integer(spec.primary_stat as i64),
        ]))
    })
}

fn set_spec_info_static_methods(t: &mlua::Table, lua: &Lua) -> Result<()> {
    t.set("GetSpellsDisplay", make_get_spells_display(lua)?)?;
    t.set(
        "GetInspectSelectedSpecialization",
        lua.create_function(|_, _u: Option<String>| Ok(0))?,
    )?;
    t.set(
        "CanPlayerUseTalentSpecUI",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    t.set(
        "CanPlayerUseTalentUI",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    t.set("IsInitialized", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("GetSpecializationInfo", make_get_specialization_info(lua)?)?;
    t.set(
        "GetAllSelectedPvpTalentIDs",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetPvpTalentSlotInfo",
        lua.create_function(|_, _s: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetNumSpecializationsForClassID",
        lua.create_function(|_, (class_id, _sex): (Option<i32>, Option<i32>)| {
            use crate::specializations;
            Ok(class_id.map_or(0, |cid| {
                specializations::specs_for_class(cid as u32).count() as i32
            }))
        })?,
    )?;
    Ok(())
}

fn set_spec_info_get_specialization(
    t: &mlua::Table,
    lua: &Lua,
    state: Rc<RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    t.set(
        "GetSpecialization",
        lua.create_function(move |_, ()| Ok(state.borrow().player.active_spec_index))?,
    )
}

fn set_spec_info_set_specialization(
    t: &mlua::Table,
    lua: &Lua,
    state: Rc<RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    t.set(
        "SetSpecialization",
        lua.create_function(move |lua, spec_index: i32| {
            if state.borrow().casting.is_some() {
                return Ok(false);
            }
            if state.borrow().player.active_spec_index == spec_index {
                return Ok(false);
            }
            state.borrow_mut().player.pending_spec_change = Some(spec_index);
            crate::lua_api::globals::action_bar_api::start_cast(
                &state,
                lua,
                SPEC_ACTIVATION_SPELL_ID,
                4000,
            )?;
            Ok(true)
        })?,
    )
}

fn register_is_spec_activate_spell(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "IsSpecializationActivateSpell",
        lua.create_function(|_, spell_id: i32| Ok(spell_id as u32 == SPEC_ACTIVATION_SPELL_ID))?,
    )
}

fn register_c_super_track(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetSuperTrackedMapPin",
        lua.create_function(|_, ()| Ok((Value::Nil, Value::Nil)))?,
    )?;
    t.set(
        "SetSuperTrackedMapPin",
        lua.create_function(|_, (_m, _x, _y): (i32, f32, f32)| Ok(()))?,
    )?;
    t.set(
        "ClearSuperTrackedMapPin",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    t.set(
        "GetSuperTrackedQuestID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "SetSuperTrackedQuestID",
        lua.create_function(|_, _id: i32| Ok(()))?,
    )?;
    t.set(
        "IsSuperTrackingQuest",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "IsSuperTrackingMapPin",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetSuperTrackedVignette",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "IsSuperTrackingAnything",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetSuperTrackedContent",
        lua.create_function(|_, ()| Ok((Value::Nil, Value::Nil)))?,
    )?;
    lua.globals().set("C_SuperTrack", t)?;
    Ok(())
}

fn register_c_player_interaction_manager(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "IsInteractingWithNpcOfType",
        lua.create_function(|_, _n: i32| Ok(false))?,
    )?;
    t.set("IsReplacingUnit", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "ClearInteraction",
        lua.create_function(|_, _i: Option<i32>| Ok(()))?,
    )?;
    t.set(
        "GetCurrentInteraction",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_PlayerInteractionManager", t)?;
    Ok(())
}

fn register_c_paper_doll_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetStatsError",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetMinItemLevel",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set("OffhandHasShield", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("OffhandHasWeapon", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsRangedSlotShown", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetArmorEffectiveness",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(0.0_f64))?,
    )?;
    t.set(
        "GetArmorEffectivenessAgainstTarget",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetStaggerPercentage",
        lua.create_function(|_, _unit: Value| Ok((0.0_f64, Value::Nil)))?,
    )?;
    t.set(
        "CanCursorCanGoInSlot",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?,
    )?;
    lua.globals().set("C_PaperDollInfo", t)?;
    Ok(())
}

fn register_c_perks_program(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "IsTradingPostAvailable",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("C_PerksProgram", t)?;
    Ok(())
}

fn make_renown_level_entry(lua: &Lua, level: i32) -> Result<mlua::Table> {
    let entry = lua.create_table()?;
    entry.set("level", level)?;
    entry.set("isCapstone", level % 10 == 0)?;
    entry.set("isMilestone", level % 5 == 0)?;
    entry.set("locked", false)?;
    Ok(entry)
}

fn seeded_covenant_renown_reward(
    covenant_id: i32,
    renown_level: i32,
) -> Option<(i32, &'static str, &'static str, &'static str)> {
    match (covenant_id, renown_level) {
        (1, 5) => Some((
            4_089_529,
            "Path of Ascension",
            "Unlocks a new covenant activity.",
            "Path of Ascension unlocked",
        )),
        _ => None,
    }
}

fn make_covenant_renown_rewards_for_level(
    lua: &Lua,
    covenant_id: i32,
    renown_level: i32,
) -> Result<mlua::Table> {
    let rewards = lua.create_table()?;
    let Some((icon, name, description, toast_description)) =
        seeded_covenant_renown_reward(covenant_id, renown_level)
    else {
        return Ok(rewards);
    };

    let reward = lua.create_table()?;
    reward.set("icon", icon)?;
    reward.set("name", name)?;
    reward.set("description", description)?;
    reward.set("toastDescription", toast_description)?;
    rewards.set(1, reward)?;
    Ok(rewards)
}

fn register_c_covenant_sanctum_ui(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetRenownLevels",
        lua.create_function(|lua, covenant_id: i32| {
            let result = lua.create_table()?;
            if (1..=4).contains(&covenant_id) {
                for level in 1..=80 {
                    result.set(level, make_renown_level_entry(lua, level)?)?;
                }
            }
            Ok(result)
        })?,
    )?;
    t.set(
        "GetCurrentRenownLevel",
        lua.create_function(|_, _id: i32| Ok(0i32))?,
    )?;
    t.set(
        "HasMaximumRenown",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set(
        "GetRenownRewardsForLevel",
        lua.create_function(|lua, (covenant_id, renown_level): (i32, i32)| {
            make_covenant_renown_rewards_for_level(lua, covenant_id, renown_level)
        })?,
    )?;
    lua.globals().set("C_CovenantSanctumUI", t)?;
    Ok(())
}
