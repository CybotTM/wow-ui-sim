//! Achievement, tracking, and SimulatePing stubs split from c_stubs_api_extra.rs.

use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

struct AchievementCategory {
    id: i32,
    name: &'static str,
    parent_id: i32,
}

struct AchievementCriteria {
    name: &'static str,
    criteria_type: i32,
    required_quantity: i32,
}

struct AchievementData {
    id: i32,
    name: &'static str,
    description: &'static str,
    points: i32,
    icon: u32,
    category_id: i32,
    criteria: &'static [AchievementCriteria],
}

static ACHIEVEMENTS: &[AchievementData] = &[
    // General (level achievements)
    AchievementData {
        id: 6,
        name: "Level 10",
        description: "Reach level 10.",
        points: 10,
        icon: 236562,
        category_id: 92,
        criteria: &[AchievementCriteria {
            name: "Reach level 10",
            criteria_type: 5,
            required_quantity: 10,
        }],
    },
    AchievementData {
        id: 7,
        name: "Level 20",
        description: "Reach level 20.",
        points: 10,
        icon: 236563,
        category_id: 92,
        criteria: &[AchievementCriteria {
            name: "Reach level 20",
            criteria_type: 5,
            required_quantity: 20,
        }],
    },
    AchievementData {
        id: 8,
        name: "Level 40",
        description: "Reach level 40.",
        points: 10,
        icon: 236565,
        category_id: 92,
        criteria: &[AchievementCriteria {
            name: "Reach level 40",
            criteria_type: 5,
            required_quantity: 40,
        }],
    },
    AchievementData {
        id: 9,
        name: "Level 60",
        description: "Reach level 60.",
        points: 10,
        icon: 236567,
        category_id: 92,
        criteria: &[AchievementCriteria {
            name: "Reach level 60",
            criteria_type: 5,
            required_quantity: 60,
        }],
    },
    AchievementData {
        id: 10,
        name: "Level 70",
        description: "Reach level 70.",
        points: 10,
        icon: 236568,
        category_id: 92,
        criteria: &[AchievementCriteria {
            name: "Reach level 70",
            criteria_type: 5,
            required_quantity: 70,
        }],
    },
    AchievementData {
        id: 11,
        name: "Level 80",
        description: "Reach level 80.",
        points: 10,
        icon: 236569,
        category_id: 92,
        criteria: &[AchievementCriteria {
            name: "Reach level 80",
            criteria_type: 5,
            required_quantity: 80,
        }],
    },
    // Quests
    AchievementData {
        id: 503,
        name: "50 Quests Completed",
        description: "Complete 50 quests.",
        points: 10,
        icon: 236664,
        category_id: 96,
        criteria: &[AchievementCriteria {
            name: "Complete 50 quests",
            criteria_type: 0,
            required_quantity: 50,
        }],
    },
    AchievementData {
        id: 504,
        name: "100 Quests Completed",
        description: "Complete 100 quests.",
        points: 10,
        icon: 236665,
        category_id: 96,
        criteria: &[AchievementCriteria {
            name: "Complete 100 quests",
            criteria_type: 0,
            required_quantity: 100,
        }],
    },
    AchievementData {
        id: 505,
        name: "250 Quests Completed",
        description: "Complete 250 quests.",
        points: 10,
        icon: 236666,
        category_id: 96,
        criteria: &[AchievementCriteria {
            name: "Complete 250 quests",
            criteria_type: 0,
            required_quantity: 250,
        }],
    },
    // Exploration
    AchievementData {
        id: 776,
        name: "Explore Elwynn Forest",
        description: "Explore Elwynn Forest, revealing the covered areas of the world map.",
        points: 10,
        icon: 236809,
        category_id: 97,
        criteria: &[],
    },
    AchievementData {
        id: 627,
        name: "Explore Durotar",
        description: "Explore Durotar, revealing the covered areas of the world map.",
        points: 10,
        icon: 236809,
        category_id: 97,
        criteria: &[],
    },
    // PvP
    AchievementData {
        id: 238,
        name: "An Honorable Kill",
        description: "Achieve an honorable kill.",
        points: 10,
        icon: 236363,
        category_id: 95,
        criteria: &[AchievementCriteria {
            name: "Honorable kills",
            criteria_type: 0,
            required_quantity: 1,
        }],
    },
    AchievementData {
        id: 513,
        name: "100 Honorable Kills",
        description: "Get 100 honorable kills.",
        points: 10,
        icon: 236363,
        category_id: 95,
        criteria: &[AchievementCriteria {
            name: "Honorable kills",
            criteria_type: 0,
            required_quantity: 100,
        }],
    },
    // Dungeons & Raids
    AchievementData {
        id: 632,
        name: "Deadmines",
        description: "Defeat Edwin VanCleef.",
        points: 10,
        icon: 135274,
        category_id: 168,
        criteria: &[AchievementCriteria {
            name: "Edwin VanCleef",
            criteria_type: 0,
            required_quantity: 1,
        }],
    },
    AchievementData {
        id: 633,
        name: "Shadowfang Keep",
        description: "Defeat Lord Godfrey.",
        points: 10,
        icon: 136243,
        category_id: 168,
        criteria: &[AchievementCriteria {
            name: "Lord Godfrey",
            criteria_type: 0,
            required_quantity: 1,
        }],
    },
    // Professions
    AchievementData {
        id: 116,
        name: "Professional Journeyman",
        description: "Become a Journeyman in a profession.",
        points: 10,
        icon: 136243,
        category_id: 169,
        criteria: &[],
    },
    AchievementData {
        id: 731,
        name: "Professional Expert",
        description: "Become an Expert in a profession.",
        points: 10,
        icon: 136243,
        category_id: 169,
        criteria: &[],
    },
    // Reputation
    AchievementData {
        id: 948,
        name: "Ambassador of the Alliance",
        description: "Earn exalted status with the five Alliance capital cities.",
        points: 10,
        icon: 236685,
        category_id: 201,
        criteria: &[
            AchievementCriteria {
                name: "Exalted with Stormwind",
                criteria_type: 46,
                required_quantity: 1,
            },
            AchievementCriteria {
                name: "Exalted with Ironforge",
                criteria_type: 46,
                required_quantity: 1,
            },
            AchievementCriteria {
                name: "Exalted with Gnomeregan",
                criteria_type: 46,
                required_quantity: 1,
            },
            AchievementCriteria {
                name: "Exalted with Darnassus",
                criteria_type: 46,
                required_quantity: 1,
            },
            AchievementCriteria {
                name: "Exalted with Exodar",
                criteria_type: 46,
                required_quantity: 1,
            },
        ],
    },
    // World Events
    AchievementData {
        id: 913,
        name: "To Honor One's Elders",
        description: "Complete the Lunar Festival achievements.",
        points: 10,
        icon: 236704,
        category_id: 155,
        criteria: &[],
    },
    // Feats of Strength
    AchievementData {
        id: 879,
        name: "Old School Ride",
        description: "Owner of a classic epic mount.",
        points: 0,
        icon: 136243,
        category_id: 81,
        criteria: &[],
    },
];

fn find_achievement(id: i32) -> Option<&'static AchievementData> {
    ACHIEVEMENTS.iter().find(|a| a.id == id)
}

static ACHIEVEMENT_CATEGORIES: &[AchievementCategory] = &[
    AchievementCategory {
        id: 92,
        name: "General",
        parent_id: -1,
    },
    AchievementCategory {
        id: 96,
        name: "Quests",
        parent_id: -1,
    },
    AchievementCategory {
        id: 97,
        name: "Exploration",
        parent_id: -1,
    },
    AchievementCategory {
        id: 95,
        name: "PvP",
        parent_id: -1,
    },
    AchievementCategory {
        id: 168,
        name: "Dungeons & Raids",
        parent_id: -1,
    },
    AchievementCategory {
        id: 169,
        name: "Professions",
        parent_id: -1,
    },
    AchievementCategory {
        id: 201,
        name: "Reputation",
        parent_id: -1,
    },
    AchievementCategory {
        id: 155,
        name: "World Events",
        parent_id: -1,
    },
    AchievementCategory {
        id: 81,
        name: "Feats of Strength",
        parent_id: -1,
    },
];

/// Achievement category API stubs needed by Blizzard_AchievementUI at parse time.
pub fn register_achievement_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    register_achievement_empty_table_stubs(lua, &g)?;
    g.set(
        "GetCategoryInfo",
        lua.create_function(|lua, id: Value| {
            let cat_id = match &id {
                Value::Integer(n) => *n as i32,
                Value::Number(n) => *n as i32,
                _ => return Ok((Value::Nil, -1i32, -1i32)),
            };
            let cat = ACHIEVEMENT_CATEGORIES.iter().find(|c| c.id == cat_id);
            match cat {
                Some(c) => Ok((Value::String(lua.create_string(c.name)?), c.parent_id, 0i32)),
                None => Ok((Value::Nil, -1i32, -1i32)),
            }
        })?,
    )?;
    g.set(
        "GetCategoryNumAchievements",
        lua.create_function(|lua, cat_id: Value| {
            let cid = match &cat_id {
                Value::Integer(n) => *n as i32,
                Value::Number(n) => *n as i32,
                _ => return Ok((0i32, 0i32, 0i32)),
            };
            let total = ACHIEVEMENTS.iter().filter(|a| a.category_id == cid).count() as i32;
            let completed = ACHIEVEMENTS
                .iter()
                .filter(|a| a.category_id == cid && is_achievement_earned(lua, a.id))
                .count() as i32;
            Ok((total, completed, total - completed))
        })?,
    )?;
    g.set(
        "GetTotalAchievementPoints",
        lua.create_function(|_, _: mlua::MultiValue| Ok(0i32))?,
    )?;
    g.set(
        "GetAchievementInfo",
        lua.create_function(stub_get_achievement_info)?,
    )?;
    g.set(
        "GetNumCompletedAchievements",
        lua.create_function(|_, _: Option<bool>| Ok((0i32, 0i32)))?,
    )?;
    g.set(
        "GetAchievementNumCriteria",
        lua.create_function(|_, aid: i32| {
            Ok(find_achievement(aid)
                .map(|a| a.criteria.len() as i32)
                .unwrap_or(0))
        })?,
    )?;
    g.set(
        "GetAchievementCriteriaInfo",
        lua.create_function(stub_get_achievement_criteria_info)?,
    )?;
    Ok(())
}

fn register_achievement_empty_table_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetCategoryList",
        lua.create_function(|lua, ()| {
            let t = lua.create_table()?;
            for (i, cat) in ACHIEVEMENT_CATEGORIES.iter().enumerate() {
                t.set(i + 1, cat.id)?;
            }
            Ok(t)
        })?,
    )?;
    let empty_table = lua.create_function(|lua, ()| lua.create_table())?;
    for name in ["GetGuildCategoryList", "GetStatisticsCategoryList"] {
        g.set(name, empty_table.clone())?;
    }
    let empty_multi = lua.create_function(|_, _: mlua::MultiValue| Ok(mlua::MultiValue::new()))?;
    for name in ["GetLatestCompletedAchievements", "GetTrackedAchievements"] {
        g.set(name, empty_multi.clone())?;
    }
    Ok(())
}

fn is_achievement_earned(lua: &Lua, aid: i32) -> bool {
    lua.app_data_ref::<Rc<RefCell<SimState>>>()
        .map(|s| s.borrow().world.earned_achievements.contains(&aid))
        .unwrap_or(false)
}

/// GetAchievementInfo — returns 14 values matching WoW's signature.
/// Looks up from ACHIEVEMENTS data, falls back to generic for unknown IDs.
fn stub_get_achievement_info(lua: &Lua, id: Value) -> Result<mlua::MultiValue> {
    let aid = match &id {
        Value::Integer(n) => *n as i32,
        Value::Number(n) => *n as i32,
        _ => return Ok(mlua::MultiValue::from_vec(vec![Value::Nil])),
    };
    let completed = is_achievement_earned(lua, aid);
    let data = find_achievement(aid);
    let name = data.map(|a| a.name).unwrap_or("Achievement");
    let desc = data
        .map(|a| a.description)
        .unwrap_or("Achievement description");
    let points = data.map(|a| a.points).unwrap_or(10) as i64;
    let icon = data.map(|a| a.icon).unwrap_or(136243) as i64;
    let (month, day, year) = if completed { (1, 15, 2025) } else { (0, 0, 0) };
    Ok(mlua::MultiValue::from_vec(vec![
        Value::Integer(aid as i64),              // id
        Value::String(lua.create_string(name)?), // name
        Value::Integer(points),                  // points
        Value::Boolean(completed),               // completed
        Value::Integer(month),                   // month
        Value::Integer(day),                     // day
        Value::Integer(year),                    // year
        Value::String(lua.create_string(desc)?), // description
        Value::Integer(0),                       // flags
        Value::Integer(icon),                    // icon
        Value::String(lua.create_string("")?),   // rewardText
        Value::Boolean(false),                   // isGuild
        Value::Boolean(completed),               // wasEarnedByMe
        Value::Nil,                              // earnedBy
    ]))
}

/// GetAchievementCriteriaInfo(achievementID, criteriaIndex) → 9 values.
fn stub_get_achievement_criteria_info(
    lua: &Lua,
    (aid, idx): (i32, i32),
) -> Result<mlua::MultiValue> {
    let criteria = find_achievement(aid).and_then(|a| a.criteria.get((idx - 1) as usize));
    let Some(c) = criteria else {
        return Ok(mlua::MultiValue::from_vec(vec![Value::Nil]));
    };
    let completed = is_achievement_earned(lua, aid);
    let quantity = if completed { c.required_quantity } else { 0 };
    Ok(mlua::MultiValue::from_vec(vec![
        Value::String(lua.create_string(c.name)?), // criteriaString
        Value::Integer(c.criteria_type as i64),    // criteriaType
        Value::Boolean(completed),                 // criteriaCompleted
        Value::Integer(quantity as i64),           // quantity
        Value::Integer(c.required_quantity as i64), // reqQuantity
        Value::String(lua.create_string("")?),     // charName
        Value::Integer(0),                         // criteriaFlags
        Value::Integer(0),                         // assetID
        Value::String(lua.create_string(format!("{}/{}", quantity, c.required_quantity))?), // quantityString
    ]))
}

/// SimulatePing(textureKit) - fires stored PingManager callbacks to render a pin.
pub fn register_simulate_ping(lua: &Lua) -> Result<()> {
    lua.load(
        r#"
        function SimulatePing(textureKit)
            textureKit = textureKit or "Attack"
            local cbs = _G.__PingSecureCallbacks
            if not cbs or not cbs.PingPinFrameAdded then
                print("SimulatePing: PingManager not initialized (no PingPinFrameAdded callback)")
                return
            end
            local anchor = CreateFrame("Frame", nil, UIParent)
            anchor:SetSize(1, 1)
            anchor:SetPoint("CENTER", UIParent, "CENTER", 0, 0)
            anchor:Show()
            cbs.PingPinFrameAdded(anchor, textureKit, true)
            C_Timer.After(5, function()
                if cbs.PingPinFrameRemoved then cbs.PingPinFrameRemoved(anchor) end
            end)
        end
    "#,
    )
    .exec()
}

/// Loot, content-tracking, and achievement telemetry namespace stubs.
pub fn register_tracking_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("C_Loot", register_c_loot(lua)?)?;
    g.set("C_ContentTracking", register_c_content_tracking(lua)?)?;
    g.set(
        "C_AchievementTelemetry",
        register_c_achievement_telemetry(lua)?,
    )?;
    Ok(())
}

fn register_c_loot(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set(
        "GetLootRollDuration",
        lua.create_function(|_, _: Value| Ok(0i32))?,
    )?;
    Ok(t)
}

fn register_c_content_tracking(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set(
        "GetTrackedIDs",
        lua.create_function(|lua, _: Value| lua.create_table())?,
    )?;
    t.set(
        "IsTracking",
        lua.create_function(|_, _: (Value, Value)| Ok(false))?,
    )?;
    t.set(
        "GetCollectableSourceTrackingEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    Ok(t)
}

fn register_c_achievement_telemetry(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set("ShowAchievements", lua.create_function(|_, ()| Ok(()))?)?;
    let noop = lua.create_function(|_, _: Value| Ok(()))?;
    t.set("LinkAchievementInWhisper", noop.clone())?;
    t.set("LinkAchievementInClub", noop)?;
    Ok(t)
}
