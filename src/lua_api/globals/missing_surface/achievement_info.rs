//! `C_AchievementInfo` probe surface backed by `SimState.achievements`
//! and `WorldState.earned_achievements`, plus legacy global achievement
//! helpers still used by older Blizzard code and tests.
//!
//! Migrates `C_AchievementInfo.GetAchievementInfo`,
//! `C_AchievementInfo.GetRewardItemID`, and
//! `C_AchievementInfo.IsValidAchievement` off the namespace stubs, then
//! layers the legacy globals (`GetAchievementInfo`,
//! `GetCategoryList`, `GetCategoryInfo`,
//! `GetCategoryNumAchievements`, `GetAchievementNumCriteria`,
//! `GetAchievementCriteriaInfo`) on top of the same seeded data.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_api::state::AchievementInfo;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const COMPLETION_MONTH: f64 = 1.0;
const COMPLETION_DAY: f64 = 15.0;
const COMPLETION_YEAR: f64 = 2025.0;

const GENERAL_ACHIEVEMENT_IDS: &[i32] = &[6, 7, 8, 9, 10, 11];
const EXPLORATION_ACHIEVEMENT_IDS: &[i32] = &[42, 776];
const PVP_ACHIEVEMENT_IDS: &[i32] = &[513, 558];
const REPUTATION_ACHIEVEMENT_IDS: &[i32] = &[948];
const REPUTATION_EXALTED_ACHIEVEMENT_IDS: &[i32] = &[1017];

const ACHIEVEMENT_CATEGORIES: &[CategoryBucket] = &[
    CategoryBucket::new(92, "General", -1, 0, GENERAL_ACHIEVEMENT_IDS),
    CategoryBucket::new(96, "Quests", -1, 0, &[]),
    CategoryBucket::new(97, "Exploration", -1, 0, EXPLORATION_ACHIEVEMENT_IDS),
    CategoryBucket::new(15522, "Character", -1, 0, &[]),
    CategoryBucket::new(95, "Player vs. Player", -1, 0, PVP_ACHIEVEMENT_IDS),
    CategoryBucket::new(168, "Dungeons & Raids", -1, 0, &[]),
    CategoryBucket::new(169, "Professions", -1, 0, &[]),
    CategoryBucket::new(201, "Reputation", -1, 0, REPUTATION_ACHIEVEMENT_IDS),
    CategoryBucket::new(
        202,
        "Exalted Reputations",
        201,
        0,
        REPUTATION_EXALTED_ACHIEVEMENT_IDS,
    ),
    CategoryBucket::new(155, "World Events", -1, 0, &[]),
    CategoryBucket::new(15117, "Expansion Features", -1, 0, &[]),
    CategoryBucket::new(15246, "Collections", -1, 0, &[]),
    CategoryBucket::new(81, "Feats of Strength", -1, 0, &[]),
];

const GUILD_CATEGORY_ID: i32 = 15076;
const GUILD_CATEGORIES: &[CategoryBucket] = &[
    CategoryBucket::new(15076, "Guild", -1, 0, &[]),
    CategoryBucket::new(15088, "Guild Summary", GUILD_CATEGORY_ID, 0, &[]),
    CategoryBucket::new(15077, "General", GUILD_CATEGORY_ID, 0, &[]),
    CategoryBucket::new(15078, "Quests", GUILD_CATEGORY_ID, 0, &[]),
    CategoryBucket::new(15079, "Player vs. Player", GUILD_CATEGORY_ID, 0, &[]),
    CategoryBucket::new(15080, "Dungeons & Raids", GUILD_CATEGORY_ID, 0, &[]),
    CategoryBucket::new(15089, "Professions", GUILD_CATEGORY_ID, 0, &[]),
    CategoryBucket::new(15093, "Guild Feats of Strength", GUILD_CATEGORY_ID, 0, &[]),
];

const STATISTICS_CATEGORIES: &[CategoryBucket] = &[
    CategoryBucket::new(130, "Statistics", -1, 0, &[]),
    CategoryBucket::new(1, "General", 130, 0, &[]),
    CategoryBucket::new(122, "Deaths", 130, 0, &[]),
    CategoryBucket::new(124, "Player vs. Player", 130, 0, &[]),
    CategoryBucket::new(128, "Wealth", 130, 0, &[]),
];

const AMBASSADOR_CRITERIA: &[AchievementCriterion] = &[
    AchievementCriterion::new("Exalted with Stormwind", 1),
    AchievementCriterion::new("Exalted with Ironforge", 1),
    AchievementCriterion::new("Exalted with Darnassus", 1),
    AchievementCriterion::new("Exalted with Gnomeregan", 1),
    AchievementCriterion::new("Exalted with Exodar", 1),
];

const VETERAN_CRITERIA: &[AchievementCriterion] =
    &[AchievementCriterion::new("Honorable kills", 100)];

#[derive(Clone, Copy)]
enum CategoryListKind {
    Achievement,
    Guild,
    Statistics,
}

#[derive(Clone, Copy)]
struct CategoryBucket {
    category_id: i32,
    name: &'static str,
    parent_id: i32,
    flags: i32,
    achievement_ids: &'static [i32],
}

impl CategoryBucket {
    const fn new(
        category_id: i32,
        name: &'static str,
        parent_id: i32,
        flags: i32,
        achievement_ids: &'static [i32],
    ) -> Self {
        Self {
            category_id,
            name,
            parent_id,
            flags,
            achievement_ids,
        }
    }
}

#[derive(Clone, Copy)]
struct AchievementCriterion {
    name: &'static str,
    required_quantity: i32,
}

impl AchievementCriterion {
    const fn new(name: &'static str, required_quantity: i32) -> Self {
        Self {
            name,
            required_quantity,
        }
    }
}

pub(super) fn register_achievement_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_AchievementInfo")?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAchievementInfo",
        c_achievement_info_get_achievement_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetRewardItemID",
        c_achievement_info_get_reward_item_id,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsValidAchievement",
        c_achievement_info_is_valid_achievement,
    )?;
    register_legacy_achievement_globals(state)?;
    Ok(())
}

fn c_achievement_info_get_achievement_info(state: &mut LuaState) -> LuaResult<u32> {
    push_achievement_info_for_id(state, i32::from_stack(state, 1)?)
}

fn c_achievement_info_get_reward_item_id(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = i32::from_stack(state, 1)?;
    let reward = borrow_state(state)?
        .achievements
        .get(&achievement_id)
        .and_then(|a| a.reward_item_id);
    match reward {
        Some(id) => state.push(Val::Num(id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_achievement_info_is_valid_achievement(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = i32::from_stack(state, 1)?;
    let valid = borrow_state(state)?
        .achievements
        .contains_key(&achievement_id);
    state.push(Val::Bool(valid));
    Ok(1)
}

fn register_legacy_achievement_globals(state: &mut LuaState) -> LuaResult<()> {
    let globals = state.global;
    table_set_rust_fn_static(
        state,
        globals,
        "GetAchievementInfo",
        get_achievement_info_global,
    )?;
    register_category_globals(state, globals)?;
    register_criteria_globals(state, globals)?;
    register_traversal_globals(state, globals)?;
    table_set_rust_fn_static(
        state,
        globals,
        "GetAchievementGuildRep",
        get_achievement_guild_rep,
    )?;
    Ok(())
}

fn register_category_globals(state: &mut LuaState, globals: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, globals, "GetCategoryList", get_category_list)?;
    table_set_rust_fn_static(
        state,
        globals,
        "GetGuildCategoryList",
        get_guild_category_list,
    )?;
    table_set_rust_fn_static(
        state,
        globals,
        "GetStatisticsCategoryList",
        get_statistics_category_list,
    )?;
    table_set_rust_fn_static(state, globals, "GetCategoryInfo", get_category_info)?;
    table_set_rust_fn_static(
        state,
        globals,
        "GetAchievementCategory",
        get_achievement_category,
    )?;
    table_set_rust_fn_static(
        state,
        globals,
        "GetCategoryNumAchievements",
        get_category_num_achievements,
    )?;
    Ok(())
}

fn register_criteria_globals(state: &mut LuaState, globals: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        globals,
        "GetAchievementNumCriteria",
        get_achievement_num_criteria,
    )?;
    table_set_rust_fn_static(
        state,
        globals,
        "GetAchievementCriteriaInfo",
        get_achievement_criteria_info,
    )?;
    Ok(())
}

fn register_traversal_globals(state: &mut LuaState, globals: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        globals,
        "GetPreviousAchievement",
        get_previous_achievement,
    )?;
    table_set_rust_fn_static(state, globals, "GetNextAchievement", get_next_achievement)?;
    table_set_rust_fn_static(
        state,
        globals,
        "GetLatestCompletedAchievements",
        get_latest_completed_achievements,
    )?;
    Ok(())
}

fn get_achievement_info_global(state: &mut LuaState) -> LuaResult<u32> {
    push_achievement_info_for_id(state, i32::from_stack(state, 1)?)
}

fn get_category_list(state: &mut LuaState) -> LuaResult<u32> {
    push_category_list(state, CategoryListKind::Achievement);
    Ok(1)
}

fn get_guild_category_list(state: &mut LuaState) -> LuaResult<u32> {
    push_category_list(state, CategoryListKind::Guild);
    Ok(1)
}

fn get_statistics_category_list(state: &mut LuaState) -> LuaResult<u32> {
    push_category_list(state, CategoryListKind::Statistics);
    Ok(1)
}

fn get_category_info(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = i32::from_stack(state, 1)?;
    let Some(category) = find_category(category_id) else {
        return Ok(0);
    };
    let name = create_string(state, category.name);
    state.push(name);
    state.push(Val::Num(category.parent_id as f64));
    state.push(Val::Num(category.flags as f64));
    Ok(3)
}

fn get_achievement_category(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = i32::from_stack(state, 1)?;
    match category_id_for_achievement(achievement_id) {
        Some(category_id) => state.push(Val::Num(category_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_category_num_achievements(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = i32::from_stack(state, 1)?;
    let Some(category) = find_category(category_id) else {
        push_category_counts(state, 0, 0);
        return Ok(3);
    };
    let achievement_ids = collect_category_achievement_ids(category.category_id);
    let completed = count_completed_achievements(state, &achievement_ids)?;
    push_category_counts(state, achievement_ids.len() as i32, completed);
    Ok(3)
}

fn get_achievement_num_criteria(state: &mut LuaState) -> LuaResult<u32> {
    let criteria_len = criteria_for_achievement(i32::from_stack(state, 1)?)
        .map(|criteria| criteria.len())
        .unwrap_or_default();
    state.push(Val::Num(criteria_len as f64));
    Ok(1)
}

fn get_achievement_criteria_info(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = i32::from_stack(state, 1)?;
    let criterion_index = i32::from_stack(state, 2)?;
    let Some(criterion) = criterion_at(achievement_id, criterion_index) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    push_criterion_multiret(state, achievement_id, criterion)?;
    Ok(5)
}

fn get_previous_achievement(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = i32::from_stack(state, 1)?;
    match previous_achievement_id(achievement_id) {
        Some(previous_id) => state.push(Val::Num(previous_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_next_achievement(state: &mut LuaState) -> LuaResult<u32> {
    let achievement_id = i32::from_stack(state, 1)?;
    match next_achievement_id(achievement_id) {
        Some(next_id) => {
            let completed = borrow_state(state)?
                .world
                .earned_achievements
                .contains(&next_id);
            state.push(Val::Num(next_id as f64));
            state.push(Val::Bool(completed));
        }
        None => {
            state.push(Val::Nil);
            state.push(Val::Bool(false));
        }
    }
    Ok(2)
}

fn get_latest_completed_achievements(state: &mut LuaState) -> LuaResult<u32> {
    let _guild_view = bool::from_stack(state, 1).unwrap_or(false);
    let mut earned_ids = {
        let sim = borrow_state(state)?;
        sim.world
            .earned_achievements
            .iter()
            .copied()
            .collect::<Vec<_>>()
    };
    earned_ids.sort_unstable();

    let table_ref = state.gc.alloc_table(Table::new());
    let table = Val::Table(table_ref);
    for (index, achievement_id) in earned_ids.into_iter().enumerate() {
        set_table_array(
            state,
            table,
            (index + 1) as i64,
            Val::Num(achievement_id as f64),
        );
    }
    state.push(table);
    Ok(1)
}

fn get_achievement_guild_rep(state: &mut LuaState) -> LuaResult<u32> {
    let _achievement_id = i32::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
}

fn push_achievement_info_for_id(state: &mut LuaState, achievement_id: i32) -> LuaResult<u32> {
    let row = {
        let sim = borrow_state(state)?;
        let Some(info) = sim.achievements.get(&achievement_id).cloned() else {
            return Ok(0);
        };
        let completed = sim.world.earned_achievements.contains(&achievement_id);
        (info, completed)
    };
    push_achievement_multiret(state, &row.0, row.1);
    Ok(15)
}

fn push_category_list(state: &mut LuaState, kind: CategoryListKind) {
    let table_ref = state.gc.alloc_table(Table::new());
    let table = Val::Table(table_ref);
    for (index, category) in categories_for(kind).iter().enumerate() {
        set_table_array(
            state,
            table,
            (index + 1) as i64,
            Val::Num(category.category_id as f64),
        );
    }
    state.push(table);
}

fn categories_for(kind: CategoryListKind) -> &'static [CategoryBucket] {
    match kind {
        CategoryListKind::Achievement => ACHIEVEMENT_CATEGORIES,
        CategoryListKind::Guild => GUILD_CATEGORIES,
        CategoryListKind::Statistics => STATISTICS_CATEGORIES,
    }
}

fn find_category(category_id: i32) -> Option<&'static CategoryBucket> {
    ACHIEVEMENT_CATEGORIES
        .iter()
        .chain(GUILD_CATEGORIES.iter())
        .chain(STATISTICS_CATEGORIES.iter())
        .find(|category| category.category_id == category_id)
}

fn category_id_for_achievement(achievement_id: i32) -> Option<i32> {
    category_for_achievement(achievement_id).map(|category| category.category_id)
}

fn category_for_achievement(achievement_id: i32) -> Option<&'static CategoryBucket> {
    ACHIEVEMENT_CATEGORIES
        .iter()
        .find(|category| category.achievement_ids.contains(&achievement_id))
}

fn category_achievement_position(achievement_id: i32) -> Option<(&'static [i32], usize)> {
    let category = category_for_achievement(achievement_id)?;
    let position = category
        .achievement_ids
        .iter()
        .position(|&id| id == achievement_id)?;
    Some((category.achievement_ids, position))
}

fn previous_achievement_id(achievement_id: i32) -> Option<i32> {
    let (achievement_ids, position) = category_achievement_position(achievement_id)?;
    position
        .checked_sub(1)
        .and_then(|index| achievement_ids.get(index))
        .copied()
}

fn next_achievement_id(achievement_id: i32) -> Option<i32> {
    let (achievement_ids, position) = category_achievement_position(achievement_id)?;
    achievement_ids.get(position + 1).copied()
}

fn count_completed_achievements(state: &mut LuaState, achievement_ids: &[i32]) -> LuaResult<i32> {
    let sim = borrow_state(state)?;
    Ok(achievement_ids
        .iter()
        .filter(|achievement_id| sim.world.earned_achievements.contains(achievement_id))
        .count() as i32)
}

fn collect_category_achievement_ids(category_id: i32) -> Vec<i32> {
    let mut achievement_ids = Vec::new();
    append_category_achievement_ids(category_id, &mut achievement_ids);
    achievement_ids
}

fn append_category_achievement_ids(category_id: i32, achievement_ids: &mut Vec<i32>) {
    let Some(category) = find_category(category_id) else {
        return;
    };
    achievement_ids.extend(category.achievement_ids.iter().copied());
    for child in ACHIEVEMENT_CATEGORIES
        .iter()
        .filter(|child| child.parent_id == category_id)
    {
        append_category_achievement_ids(child.category_id, achievement_ids);
    }
}

fn push_category_counts(state: &mut LuaState, total: i32, completed: i32) {
    state.push(Val::Num(total as f64));
    state.push(Val::Num(completed as f64));
    state.push(Val::Num((total - completed) as f64));
}

fn criteria_for_achievement(achievement_id: i32) -> Option<&'static [AchievementCriterion]> {
    match achievement_id {
        513 => Some(VETERAN_CRITERIA),
        948 => Some(AMBASSADOR_CRITERIA),
        _ => None,
    }
}

fn criterion_at(
    achievement_id: i32,
    criterion_index: i32,
) -> Option<&'static AchievementCriterion> {
    let index = usize::try_from(criterion_index.checked_sub(1)?).ok()?;
    criteria_for_achievement(achievement_id)?.get(index)
}

fn push_criterion_multiret(
    state: &mut LuaState,
    achievement_id: i32,
    criterion: &AchievementCriterion,
) -> LuaResult<()> {
    let completed = borrow_state(state)?
        .world
        .earned_achievements
        .contains(&achievement_id);
    let quantity = if completed {
        criterion.required_quantity
    } else {
        0
    };
    let name = create_string(state, criterion.name);
    state.push(name);
    state.push(Val::Num(0.0));
    state.push(Val::Bool(completed));
    state.push(Val::Num(quantity as f64));
    state.push(Val::Num(criterion.required_quantity as f64));
    Ok(())
}

fn push_achievement_multiret(state: &mut LuaState, info: &AchievementInfo, completed: bool) {
    let name = create_string(state, &info.name);
    let description = create_string(state, &info.description);
    let reward_text = create_string(state, &info.reward_text);
    let earned_by = create_string(state, if completed { "player" } else { "" });

    state.push(Val::Num(info.achievement_id as f64));
    state.push(name);
    state.push(Val::Num(info.points as f64));
    state.push(Val::Bool(completed));
    push_completion_date(state, completed);
    state.push(description);
    state.push(Val::Num(info.flags as f64));
    state.push(Val::Num(info.icon as f64));
    state.push(reward_text);
    state.push(Val::Bool(info.is_guild));
    state.push(Val::Bool(completed));
    state.push(earned_by);
    state.push(Val::Bool(info.is_statistic));
}

fn push_completion_date(state: &mut LuaState, completed: bool) {
    let (month, day, year) = if completed {
        (COMPLETION_MONTH, COMPLETION_DAY, COMPLETION_YEAR)
    } else {
        (0.0, 0.0, 0.0)
    };
    state.push(Val::Num(month));
    state.push(Val::Num(day));
    state.push(Val::Num(year));
}
